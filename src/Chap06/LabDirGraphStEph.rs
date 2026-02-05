//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs.

pub mod LabDirGraphStEph {

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
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
    };

    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabDirGraphStEph<V: StT + Hash, L: StT + Hash> {
        pub vertices: SetStEph<V>,
        pub labeled_arcs: SetStEph<LabEdge<V, L>>,
    }

    impl<V: StT + Hash, L: StT + Hash> View for LabDirGraphStEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_arcs@ }
        }
    }

    pub trait LabDirGraphStEphTrait<V: StT + Hash, L: StT + Hash>:
    View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized {

        open spec fn spec_finite(&self) -> bool {
            self@.V.finite() && self@.A.finite()
        }

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V> 
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| exists |l: L::V| #![trigger self@.A.contains((v, w, l))] self@.A.contains((v, w, l)))
        }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V> 
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|u: V::V| exists |l: L::V| #![trigger self@.A.contains((u, v, l))] self@.A.contains((u, v, l)))
        }

        open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
            Set::new(|e: (V::V, V::V)| exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)))
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty() -> (g: LabDirGraphStEph<V, L>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures
                wf_lab_graph_view(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|V| + |A|), Span Θ(|V| + |A|), Parallelism Θ(1) - sequential
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: LabDirGraphStEph<V, L>)
            requires
                forall |u: V::V, w: V::V, l: L::V| 
                    #[trigger] labeled_arcs@.contains((u, w, l)) ==> 
                        vertices@.contains(u) && vertices@.contains(w),
            ensures
                wf_lab_graph_view(g@),
                g@.V =~= vertices@,
                g@.A =~= labeled_arcs@;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>)
            ensures a@ =~= self@.A;

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>)
            requires valid_key_type_LabEdge::<V, L>(), valid_key_type_Edge::<V>()
            ensures arcs@.finite(), arcs@ == self.spec_arcs();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_vertex(&mut self, v: V)
            requires valid_key_type_LabEdge::<V, L>()
            ensures self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, label@));

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>)
            requires wf_lab_graph_view(self@), valid_key_type_LabEdge::<V, L>()
            ensures 
                label.is_some() == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l))),
                label.is_some() ==> self@.A.contains((from@, to@, label.unwrap()@));

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires wf_lab_graph_view(self@), valid_key_type_LabEdge::<V, L>()
            ensures b == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l)));

        /// out-neighbors
        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>)
            requires wf_lab_graph_view(self@), valid_key_type_LabEdge::<V, L>()
            ensures n_plus@.finite(), n_plus@ == self.spec_n_plus(v@);

        /// in-neighbors
        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>)
            requires wf_lab_graph_view(self@), valid_key_type_LabEdge::<V, L>()
            ensures n_minus@.finite(), n_minus@ == self.spec_n_minus(v@);
    }

    impl<V: StT + Hash, L: StT + Hash> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L> {

        fn empty() -> (g: LabDirGraphStEph<V, L>) {
            LabDirGraphStEph { vertices: SetStEph::empty(), labeled_arcs: SetStEph::empty() }
        }

        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: LabDirGraphStEph<V, L>) { 
            LabDirGraphStEph { vertices, labeled_arcs } 
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>) { &self.labeled_arcs }

        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>) {
            let mut arcs: SetStEph<Edge<V>> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    arcs@ == Set::new(|e: (V::V, V::V)| 
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |e: (V::V, V::V)| #[trigger] arcs@.contains(e) implies 
                                self.spec_arcs().contains(e) by {
                                if arcs@.contains(e) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |e: (V::V, V::V)| #[trigger] self.spec_arcs().contains(e) implies 
                                arcs@.contains(e) by {
                                if self.spec_arcs().contains(e) {
                                    let l = choose |l: L::V| #![trigger la_view.contains((e.0, e.1, l))] la_view.contains((e.0, e.1, l));
                                    lemma_map_to_set_contains_index(la_seq, (e.0, e.1, l));
                                }
                            }
                        }
                        return arcs;
                    },
                    Some(labeled_arc) => {
                        let _ = arcs.insert(Edge(labeled_arc.0.clone_plus(), labeled_arc.1.clone_plus()));
                    },
                }
            }
        }

        fn add_vertex(&mut self, v: V) { let _ = self.vertices.insert(v); }

        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            let _ = self.vertices.insert(from.clone_plus());
            let _ = self.vertices.insert(to.clone_plus());
            let _ = self.labeled_arcs.insert(LabEdge(from, to, label));
        }

        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>) {
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;
            let ghost from_view = from@;
            let ghost to_view = to@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |l: L::V| !la_view.contains((from_view, to_view, l)) by {
                                if la_view.contains((from_view, to_view, l)) {
                                    lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                                }
                            }
                        }
                        return None;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            proof {
                                let idx = it@.0 - 1;
                                lemma_seq_index_in_map_to_set(la_seq, idx);
                            }
                            return Some(&labeled_arc.2);
                        }
                    },
                }
            }
        }

        fn has_arc(&self, from: &V, to: &V) -> (b: bool) {
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;
            let ghost from_view = from@;
            let ghost to_view = to@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            // No arc found - prove spec_arcs doesn't contain (from, to)
                            let pred = |e: (V::V, V::V)| exists |l: L::V| #![trigger la_view.contains((e.0, e.1, l))] la_view.contains((e.0, e.1, l));
                            vstd::set::axiom_set_new(pred, (from_view, to_view));
                            // Now we have: Set::new(pred).contains((from_view, to_view)) == pred((from_view, to_view))
                            // Need to show pred((from_view, to_view)) is false
                            assert forall |l: L::V| !la_view.contains((from_view, to_view, l)) by {
                                if la_view.contains((from_view, to_view, l)) {
                                    lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                                }
                            }
                        }
                        return false;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            proof {
                                let idx = it@.0 - 1;
                                lemma_seq_index_in_map_to_set(la_seq, idx);
                                let arc_view = la_seq[idx]@;
                                let witness_l = arc_view.2;
                            }
                            return true;
                        }
                    },
                }
            }
        }

        /// out-neighbors
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>) {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost v_view = v@;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    neighbors@ == Set::new(|w: V::V| 
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #[trigger] neighbors@.contains(w) implies 
                                self.spec_n_plus(v_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |w: V::V| #[trigger] self.spec_n_plus(v_view).contains(w) implies 
                                neighbors@.contains(w) by {
                                if self.spec_n_plus(v_view).contains(w) {
                                    let l = choose |l: L::V| #![trigger la_view.contains((v_view, w, l))] la_view.contains((v_view, w, l));
                                    lemma_map_to_set_contains_index(la_seq, (v_view, w, l));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, v) {
                            let _ = neighbors.insert(labeled_arc.1.clone_plus());
                        }
                    },
                }
            }
        }

        /// in-neighbors
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>) {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost v_view = v@;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    neighbors@ == Set::new(|u: V::V| 
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |u: V::V| #[trigger] neighbors@.contains(u) implies 
                                self.spec_n_minus(v_view).contains(u) by {
                                if neighbors@.contains(u) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |u: V::V| #[trigger] self.spec_n_minus(v_view).contains(u) implies 
                                neighbors@.contains(u) by {
                                if self.spec_n_minus(v_view).contains(u) {
                                    let l = choose |l: L::V| #![trigger la_view.contains((u, v_view, l))] la_view.contains((u, v_view, l));
                                    lemma_map_to_set_contains_index(la_seq, (u, v_view, l));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.1, v) {
                            let _ = neighbors.insert(labeled_arc.0.clone_plus());
                        }
                    },
                }
            }
        }
    }

} // verus!

    impl<V: StT + Hash, L: StT + Hash> Clone for LabDirGraphStEph<V, L> {
        fn clone(&self) -> Self { 
            LabDirGraphStEph { vertices: self.vertices.clone(), labeled_arcs: self.labeled_arcs.clone() } 
        }
    }

    impl<V: StT + Hash, L: StT + Hash> Display for LabDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V: StT + Hash, L: StT + Hash> Debug for LabDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph {{ vertices: {:?}, labeled_arcs: {:?} }}", self.vertices, self.labeled_arcs)
        }
    }

    #[macro_export]
    macro_rules! LabDirGraphStEphLit {
        () => {{
            < $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph<_, _> as $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ($from:expr, $to:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_arcs = $crate::SetLit![ $( $crate::Types::Types::LabEdge($from, $to, $label) ),* ];
            < $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph<_, _> as $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait<_, _> >::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }};
    }
}
