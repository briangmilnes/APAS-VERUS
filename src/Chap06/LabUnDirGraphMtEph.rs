//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled edge filtering (neighbors) is parallel.

pub mod LabUnDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};

    verus! {

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::valid_key_type;

    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::wf_lab_graph_view;

    broadcast use {
        vstd::set::group_set_axioms,
        crate::Types::Types::group_LabEdge_axioms,
    };

    pub open spec fn valid_key_type_for_lab_graph<V: HashOrd + MtT, L: StTInMtT + Hash>() -> bool {
        valid_key_type_LabEdge::<V, L>()
    }

    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    #[derive(Clone)]
    pub struct LabUnDirGraphMtEph<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> {
        vertices: SetStEph<V>,
        labeled_edges: SetStEph<LabEdge<V, L>>,
    }

    impl<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> View for LabUnDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_edges@ }
        }
    }

    impl<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> LabUnDirGraphMtEph<V, L> {
        pub open spec fn spec_vertices(&self) -> Set<V::V> { self.vertices@ }
        pub open spec fn spec_labeled_edges(&self) -> Set<(V::V, V::V, L::V)> { self.labeled_edges@ }
    }

    pub trait LabUnDirGraphMtEphTrait<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static>
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized
    {
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_lab_graph::<V, L>()
            ensures
                wf_lab_graph_view(g@),
                g@.V == Set::<<V as View>::V>::empty(),
                g@.A == Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                valid_key_type_for_lab_graph::<V, L>(),
                vertices@.finite(),
                labeled_edges@.finite(),
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_edges@.contains((u, w, l)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                wf_lab_graph_view(g@),
                g@.V == vertices@,
                g@.A == labeled_edges@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn labeled_edges(&self) -> (e: &SetStEph<LabEdge<V, L>>)
            ensures e@ == self@.A;

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn edges(&self) -> (edges: SetStEph<Edge<V>>)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures forall |u: V::V, w: V::V| edges@.contains((u, w)) ==
                (exists |l: L::V| #![trigger self@.A.contains((u, w, l))] self@.A.contains((u, w, l)));

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_vertex(&mut self, v: V)
            requires wf_lab_graph_view(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures wf_lab_graph_view(self@), self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L)
            requires wf_lab_graph_view(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures wf_lab_graph_view(self@);

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn get_edge_label(&self, v1: &V, v2: &V) -> (label: Option<&L>)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures match label {
                Some(l) => self@.A.contains((v1@, v2@, l@)) || self@.A.contains((v2@, v1@, l@)),
                None => forall |l: L::V| !self@.A.contains((v1@, v2@, l)) && !self@.A.contains((v2@, v1@, l)),
            };

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn has_edge(&self, v1: &V, v2: &V) -> (b: bool)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures b == (exists |l: L::V| 
                #![trigger self@.A.contains((v1@, v2@, l))] 
                #![trigger self@.A.contains((v2@, v1@, l))]
                self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l)));

        open spec fn spec_neighbors(&self, v: V::V) -> Set<V::V>
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        {
            Set::new(|w: V::V| exists |l: L::V| self@.A.contains((v, w, l)) || self@.A.contains((w, v, l)))
        }

        /// APAS: Work Θ(|E|), Span Θ(log |E|) - parallel
        fn neighbors(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires
                wf_lab_graph_view(self@),
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures
                neighbors@ == self.spec_neighbors(v@),
                neighbors@ <= self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn normalize_edge(v1: V, v2: V) -> (e: LabEdge<V, L>);
    }

    impl<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> LabUnDirGraphMtEphTrait<V, L>
        for LabUnDirGraphMtEph<V, L>
    {
        fn empty() -> (g: Self) {
            LabUnDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_edges: SetStEph::empty(),
            }
        }

        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: Self) {
            LabUnDirGraphMtEph {
                vertices,
                labeled_edges,
            }
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        fn labeled_edges(&self) -> (e: &SetStEph<LabEdge<V, L>>) { &self.labeled_edges }

        #[verifier::external_body]
        fn edges(&self) -> (edges: SetStEph<Edge<V>>) {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_edges.iter() {
                edges.insert(Edge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus()));
            }
            edges
        }

        #[verifier::external_body]
        fn add_vertex(&mut self, v: V) { 
            self.vertices.insert(v); 
        }

        #[verifier::external_body]
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {
            self.vertices.insert(v1.clone_plus());
            self.vertices.insert(v2.clone_plus());
            let normalized_edge = if v1 <= v2 {
                LabEdge(v1, v2, label)
            } else {
                LabEdge(v2, v1, label)
            };
            self.labeled_edges.insert(normalized_edge);
        }

        #[verifier::external_body]
        fn get_edge_label(&self, v1: &V, v2: &V) -> (label: Option<&L>) {
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return Some(&labeled_edge.2);
                }
            }
            None
        }

        #[verifier::external_body]
        fn has_edge(&self, v1: &V, v2: &V) -> (b: bool) {
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return true;
                }
            }
            false
        }

        #[verifier::external_body]
        fn neighbors(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let edges = self.labeled_edges.iter().cloned().collect::<Vec<LabEdge<V, L>>>();
            neighbors_parallel(edges, v.clone_plus())
        }

        #[verifier::external_body]
        fn normalize_edge(_v1: V, _v2: V) -> (e: LabEdge<V, L>) {
            panic!("normalize_edge cannot create LabEdge without a label - method signature needs revision")
        }
    }

    } // verus!

    // Parallel helper function (outside verus! block for closure support)
    fn neighbors_parallel<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static>(
        edges: Vec<LabEdge<V, L>>,
        v: V,
    ) -> SetStEph<V> {
        let n = edges.len();
        if n == 0 {
            return SetStEph::empty();
        }
        if n == 1 {
            if edges[0].0 == v {
                let mut s = SetStEph::empty();
                s.insert(edges[0].1.clone_plus());
                return s;
            } else if edges[0].1 == v {
                let mut s = SetStEph::empty();
                s.insert(edges[0].0.clone_plus());
                return s;
            }
            return SetStEph::empty();
        }

        let mid = n / 2;
        let mut right_edges = edges;
        let left_edges = right_edges.split_off(mid);

        let v_left = v.clone_plus();
        let v_right = v;

        let Pair(left_result, right_result) =
            ParaPair!(move || neighbors_parallel(left_edges, v_left), move || {
                neighbors_parallel(right_edges, v_right)
            });

        left_result.union(&right_result)
    }

    impl<V: HashOrd + MtT, L: StTInMtT + Hash> Display for LabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph(V: {}, E: {})", self.vertices, self.labeled_edges)
        }
    }

    impl<V: HashOrd + MtT, L: StTInMtT + Hash> Debug for LabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabUnDirGraph {{ vertices: {:?}, labeled_edges: {:?} }}",
                self.vertices, self.labeled_edges
            )
        }
    }

    #[macro_export]
    macro_rules! LabUnDirGraphMtEphLit {
        () => {{
            < $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEph<_, _> as $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($v1:expr, $v2:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_edges = {
                let mut edges = $crate::Chap05::SetStEph::SetStEph::SetStEph::empty();
                $(
                    let normalized_edge = if $v1 <= $v2 {
                        $crate::Types::Types::LabEdge($v1, $v2, $label)
                    } else {
                        $crate::Types::Types::LabEdge($v2, $v1, $label)
                    };
                    edges.insert(normalized_edge);
                )*
                edges
            };
            < $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEph<_, _> as $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEphTrait<_, _> >::from_vertices_and_labeled_edges(vertices, labeled_edges)
        }};
    }
}
