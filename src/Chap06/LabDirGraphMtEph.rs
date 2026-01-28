//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled arc filtering (out_neighbors, in_neighbors) are parallel.

pub mod LabDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Concurrency::Concurrency::StTInMtT;
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

    pub open spec fn valid_key_type_for_lab_graph<V: StTInMtT + Hash, L: StTInMtT + Hash>() -> bool {
        valid_key_type_LabEdge::<V, L>()
    }

    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    #[derive(Clone)]
    pub struct LabDirGraphMtEph<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> {
        vertices: SetStEph<V>,
        labeled_arcs: SetStEph<LabEdge<V, L>>,
    }

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> View for LabDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_arcs@ }
        }
    }

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> LabDirGraphMtEph<V, L> {
        pub open spec fn spec_vertices(&self) -> Set<V::V> { self.vertices@ }
        pub open spec fn spec_labeled_arcs(&self) -> Set<(V::V, V::V, L::V)> { self.labeled_arcs@ }
    }

    pub trait LabDirGraphMtEphTrait<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> 
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized 
    {
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_lab_graph::<V, L>()
            ensures
                wf_lab_graph_view(g@),
                g@.V == Set::<<V as View>::V>::empty(),
                g@.A == Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                valid_key_type_for_lab_graph::<V, L>(),
                vertices@.finite(),
                labeled_arcs@.finite(),
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_arcs@.contains((u, w, l)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                wf_lab_graph_view(g@),
                g@.V == vertices@,
                g@.A == labeled_arcs@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>)
            ensures a@ == self@.A;

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures forall |u: V::V, w: V::V| arcs@.contains((u, w)) == 
                (exists |l: L::V| #![trigger self@.A.contains((u, w, l))] self@.A.contains((u, w, l)));

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_vertex(&mut self, v: V)
            requires wf_lab_graph_view(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures wf_lab_graph_view(self@), self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L)
            requires wf_lab_graph_view(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures 
                wf_lab_graph_view(self@),
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, label@));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures match label {
                Some(l) => self@.A.contains((from@, to@, l@)),
                None => forall |l: L::V| !self@.A.contains((from@, to@, l)),
            };

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures b == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l)));

        open spec fn spec_out_neighbors(&self, v: V::V) -> Set<V::V>
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        {
            Set::new(|w: V::V| exists |l: L::V| self@.A.contains((v, w, l)))
        }

        open spec fn spec_in_neighbors(&self, v: V::V) -> Set<V::V>
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        {
            Set::new(|u: V::V| exists |l: L::V| self@.A.contains((u, v, l)))
        }

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn out_neighbors(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires 
                wf_lab_graph_view(self@), 
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures 
                out_neighbors@ == self.spec_out_neighbors(v@),
                out_neighbors@ <= self@.V;

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn in_neighbors(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires 
                wf_lab_graph_view(self@), 
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures 
                in_neighbors@ == self.spec_in_neighbors(v@),
                in_neighbors@ <= self@.V;
    }

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> LabDirGraphMtEphTrait<V, L>
        for LabDirGraphMtEph<V, L>
    {
        fn empty() -> (g: Self) {
            LabDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_arcs: SetStEph::empty(),
            }
        }

        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self) {
            LabDirGraphMtEph { vertices, labeled_arcs }
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>) { &self.labeled_arcs }

        #[verifier::external_body]
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>) {
            let mut arcs = SetStEph::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                arcs.insert(Edge(labeled_arc.0.clone_plus(), labeled_arc.1.clone_plus()));
            }
            arcs
        }

        #[verifier::external_body]
        fn add_vertex(&mut self, v: V) { 
            self.vertices.insert(v); 
        }

        #[verifier::external_body]
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            self.vertices.insert(from.clone_plus());
            self.vertices.insert(to.clone_plus());
            self.labeled_arcs.insert(LabEdge(from, to, label));
        }

        #[verifier::external_body]
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>) {
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *from && labeled_arc.1 == *to {
                    return Some(&labeled_arc.2);
                }
            }
            None
        }

        #[verifier::external_body]
        fn has_arc(&self, from: &V, to: &V) -> (b: bool) {
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *from && labeled_arc.1 == *to {
                    return true;
                }
            }
            false
        }

        #[verifier::external_body]
        fn out_neighbors(&self, v: &V) -> (out_neighbors: SetStEph<V>) {
            let arcs = self.labeled_arcs.iter().cloned().collect::<Vec<LabEdge<V, L>>>();
            out_neighbors_parallel(arcs, v.clone_plus())
        }

        #[verifier::external_body]
        fn in_neighbors(&self, v: &V) -> (in_neighbors: SetStEph<V>) {
            let arcs = self.labeled_arcs.iter().cloned().collect::<Vec<LabEdge<V, L>>>();
            in_neighbors_parallel(arcs, v.clone_plus())
        }
    }

    } // verus!

    // Parallel helper functions (outside verus! block for closure support)
    fn out_neighbors_parallel<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
        arcs: Vec<LabEdge<V, L>>,
        v: V,
    ) -> SetStEph<V> {
        let n = arcs.len();
        if n == 0 {
            return SetStEph::empty();
        }
        if n == 1 {
            return if arcs[0].0 == v {
                let mut s = SetStEph::empty();
                s.insert(arcs[0].1.clone_plus());
                s
            } else {
                SetStEph::empty()
            };
        }

        let mid = n / 2;
        let mut right_arcs = arcs;
        let left_arcs = right_arcs.split_off(mid);

        let v_left = v.clone_plus();
        let v_right = v;

        let Pair(left_result, right_result) =
            ParaPair!(move || out_neighbors_parallel(left_arcs, v_left), move || out_neighbors_parallel(
                right_arcs, v_right
            ));

        left_result.union(&right_result)
    }

    fn in_neighbors_parallel<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
        arcs: Vec<LabEdge<V, L>>,
        v: V,
    ) -> SetStEph<V> {
        let n = arcs.len();
        if n == 0 {
            return SetStEph::empty();
        }
        if n == 1 {
            return if arcs[0].1 == v {
                let mut s = SetStEph::empty();
                s.insert(arcs[0].0.clone_plus());
                s
            } else {
                SetStEph::empty()
            };
        }

        let mid = n / 2;
        let mut right_arcs = arcs;
        let left_arcs = right_arcs.split_off(mid);

        let v_left = v.clone_plus();
        let v_right = v;

        let Pair(left_result, right_result) =
            ParaPair!(move || in_neighbors_parallel(left_arcs, v_left), move || in_neighbors_parallel(
                right_arcs, v_right
            ));

        left_result.union(&right_result)
    }

    impl<V: StTInMtT + Hash, L: StTInMtT + Hash> Display for LabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V: StTInMtT + Hash, L: StTInMtT + Hash> Debug for LabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabDirGraph {{ vertices: {:?}, labeled_arcs: {:?} }}",
                self.vertices, self.labeled_arcs
            )
        }
    }

    #[macro_export]
    macro_rules! LabDirGraphMtEphLit {
        () => {{
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ($from:expr, $to:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_arcs = $crate::SetLit![ $( $crate::Types::Types::LabEdge($from, $to, $label) ),* ];
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }};
    }
}
