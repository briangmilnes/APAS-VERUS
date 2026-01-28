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
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::seq_set::*;
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

        /// Spec for neighbors computed from a subset of edges
        open spec fn spec_neighbors_from_set(&self, v: V::V, subedges: Set<(V::V, V::V, L::V)>) -> Set<V::V> 
            recommends 
                wf_lab_graph_view(self@),
                subedges <= self@.A,
        {
            Set::new(|w: V::V| exists |l: L::V| subedges.contains((v, w, l)) || subedges.contains((w, v, l)))
        }
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
    }

    /// Parallel edge filtering for neighbors using set split.
    fn neighbors_parallel<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static>(
        g: &LabUnDirGraphMtEph<V, L>, 
        v: V, 
        edges: SetStEph<LabEdge<V, L>>
    ) -> (neighbors: SetStEph<V>)
        requires
            valid_key_type::<V>(),
            valid_key_type_LabEdge::<V, L>(),
            wf_lab_graph_view(g@),
            edges@ <= g@.A,
        ensures 
            neighbors@ == g.spec_neighbors_from_set(v@, edges@),
            neighbors@ <= g.spec_neighbors(v@)
        decreases edges.size()
    {
        let n = edges.size();
        if n == 0 {
            SetStEph::empty()
        }
        else if n == 1 {
            let LabEdge(a, b, _label) = edges.choose();
            if feq(&a, &v) {
                SetStEph::singleton(b.clone_plus())
            } else if feq(&b, &v) {
                SetStEph::singleton(a.clone_plus())
            } else {
                SetStEph::empty()
            }
        }
        else {
            let mid = n / 2;
            let (left_edges, right_edges) = edges.split(mid);
            let v_left  = v.clone_plus();
            let v_right = v.clone_plus();
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_neighbors_from_set(v_left@, left_edges@)
            { neighbors_parallel(&g_left, v_left, left_edges) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_neighbors_from_set(v_right@, right_edges@)
            { neighbors_parallel(&g_right, v_right, right_edges) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            left_neighbors.union(&right_neighbors)
        }
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

        fn edges(&self) -> (edges: SetStEph<Edge<V>>) {
            let mut edges: SetStEph<Edge<V>> = SetStEph::empty();
            let mut it = self.labeled_edges.iter();
            let ghost le_seq = it@.1;
            let ghost le_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    valid_key_type_Edge::<V>(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == le_view,
                    forall |e: (V::V, V::V)| edges@.contains(e) == 
                        (exists |i: int| #![trigger le_seq[i]] 0 <= i < it@.0 && le_seq[i]@.0 == e.0 && le_seq[i]@.1 == e.1),
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |e: (V::V, V::V)| edges@.contains(e) implies 
                                (exists |l: L::V| #![trigger le_view.contains((e.0, e.1, l))] le_view.contains((e.0, e.1, l))) by {
                                if edges@.contains(e) {
                                    let i = choose |i: int| #![trigger le_seq[i]] 0 <= i < le_seq.len() && le_seq[i]@.0 == e.0 && le_seq[i]@.1 == e.1;
                                    lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
                            assert forall |e: (V::V, V::V)| (exists |l: L::V| #![trigger le_view.contains((e.0, e.1, l))] le_view.contains((e.0, e.1, l))) implies 
                                edges@.contains(e) by {
                                if exists |l: L::V| #![trigger le_view.contains((e.0, e.1, l))] le_view.contains((e.0, e.1, l)) {
                                    let l = choose |l: L::V| #![trigger le_view.contains((e.0, e.1, l))] le_view.contains((e.0, e.1, l));
                                    lemma_map_to_set_contains_index(le_seq, (e.0, e.1, l));
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        let _ = edges.insert(Edge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus()));
                    },
                }
            }
        }

        fn add_vertex(&mut self, v: V) { let _ = self.vertices.insert(v); }

        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {
            let _ = self.vertices.insert(v1.clone_plus());
            let _ = self.vertices.insert(v2.clone_plus());
            if v1 <= v2 {
                let _ = self.labeled_edges.insert(LabEdge(v1, v2, label));
            } else {
                let _ = self.labeled_edges.insert(LabEdge(v2, v1, label));
            }
        }

        fn get_edge_label(&self, v1: &V, v2: &V) -> (label: Option<&L>) {
            let mut it = self.labeled_edges.iter();
            let ghost le_seq = it@.1;
            let ghost le_view = self@.A;
            let ghost v1_view = v1@;
            let ghost v2_view = v2@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == le_view,
                    forall |i: int| #![trigger le_seq[i]] 0 <= i < it@.0 ==> 
                        !((le_seq[i]@.0 == v1_view && le_seq[i]@.1 == v2_view) ||
                          (le_seq[i]@.0 == v2_view && le_seq[i]@.1 == v1_view)),
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |l: L::V| 
                                !(le_view.contains((v1_view, v2_view, l)) || le_view.contains((v2_view, v1_view, l))) by {
                                if le_view.contains((v1_view, v2_view, l)) {
                                    lemma_map_to_set_contains_index(le_seq, (v1_view, v2_view, l));
                                }
                                if le_view.contains((v2_view, v1_view, l)) {
                                    lemma_map_to_set_contains_index(le_seq, (v2_view, v1_view, l));
                                }
                            }
                        }
                        return None;
                    },
                    Some(labeled_edge) => {
                        if (feq(&labeled_edge.0, v1) && feq(&labeled_edge.1, v2)) || 
                           (feq(&labeled_edge.0, v2) && feq(&labeled_edge.1, v1)) {
                            proof {
                                let idx = it@.0 - 1;
                                lemma_seq_index_in_map_to_set(le_seq, idx);
                            }
                            return Some(&labeled_edge.2);
                        }
                    },
                }
            }
        }

        fn has_edge(&self, v1: &V, v2: &V) -> (b: bool) {
            let mut it = self.labeled_edges.iter();
            let ghost le_seq = it@.1;
            let ghost le_view = self@.A;
            let ghost v1_view = v1@;
            let ghost v2_view = v2@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == le_view,
                    forall |i: int| #![trigger le_seq[i]] 0 <= i < it@.0 ==> 
                        !((le_seq[i]@.0 == v1_view && le_seq[i]@.1 == v2_view) ||
                          (le_seq[i]@.0 == v2_view && le_seq[i]@.1 == v1_view)),
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |l: L::V| 
                                !(le_view.contains((v1_view, v2_view, l)) || le_view.contains((v2_view, v1_view, l))) by {
                                if le_view.contains((v1_view, v2_view, l)) {
                                    lemma_map_to_set_contains_index(le_seq, (v1_view, v2_view, l));
                                }
                                if le_view.contains((v2_view, v1_view, l)) {
                                    lemma_map_to_set_contains_index(le_seq, (v2_view, v1_view, l));
                                }
                            }
                        }
                        return false;
                    },
                    Some(labeled_edge) => {
                        if (feq(&labeled_edge.0, v1) && feq(&labeled_edge.1, v2)) || 
                           (feq(&labeled_edge.0, v2) && feq(&labeled_edge.1, v1)) {
                            proof {
                                let idx = it@.0 - 1;
                                lemma_seq_index_in_map_to_set(le_seq, idx);
                            }
                            return true;
                        }
                    },
                }
            }
        }

        fn neighbors(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let edges = self.labeled_edges.clone();
            neighbors_parallel(self, v.clone_plus(), edges)
        }
    }

    } // verus!

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
