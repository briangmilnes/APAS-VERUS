//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges.

pub mod LabUnDirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::seq_set::*;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
    };

    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabUnDirGraphStEph<V: HashOrd, L: StT + Hash> {
        pub vertices: SetStEph<V>,
        pub labeled_edges: SetStEph<LabEdge<V, L>>,
    }

    impl<V: HashOrd, L: StT + Hash> View for LabUnDirGraphStEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_edges@ }
        }
    }

    pub trait LabUnDirGraphStEphTrait<V: HashOrd, L: StT + Hash>:
    View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized {

        open spec fn spec_finite(&self) -> bool {
            self@.V.finite() && self@.A.finite()
        }

        open spec fn spec_neighbors(&self, v: V::V) -> Set<V::V> 
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| exists |l: L::V| 
                self@.A.contains((v, w, l)) || self@.A.contains((w, v, l)))
        }

        open spec fn spec_edges(&self) -> Set<(V::V, V::V)> {
            Set::new(|e: (V::V, V::V)| exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)))
        }

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: LabUnDirGraphStEph<V, L>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures
                g@.V.finite(),
                g@.A.finite(),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: LabUnDirGraphStEph<V, L>)
            ensures
                g@.V.finite(),
                g@.A.finite(),
                g@.V =~= vertices@,
                g@.A =~= labeled_edges@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn labeled_edges(&self) -> (e: &SetStEph<LabEdge<V, L>>)
            ensures e@ =~= self@.A;

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn edges(&self) -> (edges: SetStEph<Edge<V>>)
            requires valid_key_type_LabEdge::<V, L>(), valid_key_type_Edge::<V>()
            ensures 
                forall |e: (V::V, V::V)| edges@.contains(e) == (exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)));

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_vertex(&mut self, v: V)
            requires valid_key_type_LabEdge::<V, L>()
            ensures self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                self@.V == old(self)@.V.insert(v1@).insert(v2@),
                self@.A == old(self)@.A.insert((v1@, v2@, label@)) || 
                self@.A == old(self)@.A.insert((v2@, v1@, label@));

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn get_edge_label(&self, v1: &V, v2: &V) -> (label: Option<&L>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                label.is_some() == (exists |l: L::V| 
                    self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l))),
                label.is_some() ==> (self@.A.contains((v1@, v2@, label.unwrap()@)) || 
                                      self@.A.contains((v2@, v1@, label.unwrap()@)));

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn has_edge(&self, v1: &V, v2: &V) -> (b: bool)
            requires valid_key_type_LabEdge::<V, L>()
            ensures b == (exists |l: L::V| 
                self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l)));

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn neighbors(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures neighbors@ == self.spec_neighbors(v@);
    }

    impl<V: HashOrd, L: StT + Hash> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L> {

        fn empty() -> (g: LabUnDirGraphStEph<V, L>) {
            LabUnDirGraphStEph { vertices: SetStEph::empty(), labeled_edges: SetStEph::empty() }
        }

        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: LabUnDirGraphStEph<V, L>) { 
            LabUnDirGraphStEph { vertices, labeled_edges } 
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
                                let edge_view = le_seq[idx]@;
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
                                let arc_view = le_seq[idx]@;
                                let witness_l = arc_view.2;
                            }
                            return true;
                        }
                    },
                }
            }
        }

        fn neighbors(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = self.labeled_edges.iter();
            let ghost le_seq = it@.1;
            let ghost v_view = v@;
            let ghost le_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == le_view,
                    neighbors@ == Set::new(|w: V::V| 
                        exists |i: int| #![trigger le_seq[i]] 0 <= i < it@.0 && 
                            ((le_seq[i]@.0 == v_view && le_seq[i]@.1 == w) ||
                             (le_seq[i]@.1 == v_view && le_seq[i]@.0 == w))),
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #[trigger] neighbors@.contains(w) implies 
                                self.spec_neighbors(v_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger le_seq[i]] 0 <= i < le_seq.len() && 
                                        ((le_seq[i]@.0 == v_view && le_seq[i]@.1 == w) ||
                                         (le_seq[i]@.1 == v_view && le_seq[i]@.0 == w));
                                    lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
                            assert forall |w: V::V| #[trigger] self.spec_neighbors(v_view).contains(w) implies 
                                neighbors@.contains(w) by {
                                if self.spec_neighbors(v_view).contains(w) {
                                    if exists |l: L::V| #![trigger le_view.contains((v_view, w, l))] le_view.contains((v_view, w, l)) {
                                        let l = choose |l: L::V| #![trigger le_view.contains((v_view, w, l))] le_view.contains((v_view, w, l));
                                        lemma_map_to_set_contains_index(le_seq, (v_view, w, l));
                                    } else {
                                        let l = choose |l: L::V| #![trigger le_view.contains((w, v_view, l))] le_view.contains((w, v_view, l));
                                        lemma_map_to_set_contains_index(le_seq, (w, v_view, l));
                                    }
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.0, v) {
                            let _ = neighbors.insert(labeled_edge.1.clone_plus());
                        } else if feq(&labeled_edge.1, v) {
                            let _ = neighbors.insert(labeled_edge.0.clone_plus());
                        }
                    },
                }
            }
        }
    }

} // verus!

    impl<V: HashOrd, L: StT + Hash> Clone for LabUnDirGraphStEph<V, L> {
        fn clone(&self) -> Self { 
            LabUnDirGraphStEph { vertices: self.vertices.clone(), labeled_edges: self.labeled_edges.clone() } 
        }
    }

    impl<V: HashOrd, L: StT + Hash> Display for LabUnDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph(V: {}, E: {})", self.vertices, self.labeled_edges)
        }
    }

    impl<V: HashOrd, L: StT + Hash> Debug for LabUnDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph {{ vertices: {:?}, labeled_edges: {:?} }}", self.vertices, self.labeled_edges)
        }
    }

    #[macro_export]
    macro_rules! LabUnDirGraphStEphLit {
        () => {{
            < $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph<_, _> as $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait<_, _> >::empty()
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
                    let _ = edges.insert(normalized_edge);
                )*
                edges
            };
            < $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph<_, _> as $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait<_, _> >::from_vertices_and_labeled_edges(vertices, labeled_edges)
        }};
    }
}
