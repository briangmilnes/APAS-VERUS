//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs.

pub mod LabDirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
    };

    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabDirGraphStEph<V: StT + Hash, L: StT + Hash> {
        pub vertices: SetStEph<V>,
        pub labeled_arcs: SetStEph<LabEdge<V, L>>,
    }

    impl<V: StT + Hash, L: StT + Hash> View for LabDirGraphStEph<V, L> {
        type V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V, <L as View>::V)>);
        
        open spec fn view(&self) -> Self::V {
            (self.vertices@, self.labeled_arcs@)
        }
    }

    pub trait LabDirGraphStEphTrait<V: StT + Hash, L: StT + Hash>:
    View<V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V, <L as View>::V)>)> + Sized {

        /// Out-neighbors: vertices w such that (v, w, _) is a labeled arc
        open spec fn spec_out_neighbors(&self, v: V::V) -> Set<V::V> { 
            Set::new(|w: V::V| exists |l: L::V| #![auto] self@.1.contains((v, w, l)))
        }

        /// In-neighbors: vertices u such that (u, v, _) is a labeled arc
        open spec fn spec_in_neighbors(&self, v: V::V) -> Set<V::V> { 
            Set::new(|u: V::V| exists |l: L::V| #![auto] self@.1.contains((u, v, l)))
        }

        /// Unlabeled arcs
        open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
            Set::new(|e: (V::V, V::V)| exists |l: L::V| #![auto] self@.1.contains((e.0, e.1, l)))
        }

        fn empty() -> (g: LabDirGraphStEph<V, L>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures
                g@.0 =~= Set::<<V as View>::V>::empty(),
                g@.1 =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: LabDirGraphStEph<V, L>)
            ensures
                g@.0 =~= vertices@,
                g@.1 =~= labeled_arcs@;

        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.0;

        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>)
            ensures a@ =~= self@.1;

        fn arcs(&self) -> (result: SetStEph<Edge<V>>)
            requires valid_key_type_LabEdge::<V, L>(), valid_key_type_Edge::<V>()
            ensures result@ == self.spec_arcs();

        fn add_vertex(&mut self, v: V)
            requires valid_key_type_LabEdge::<V, L>()
            ensures self@.0 == old(self)@.0.insert(v@), self@.1 == old(self)@.1;

        fn add_labeled_arc(&mut self, from: V, to: V, label: L)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                self@.0 == old(self)@.0.insert(from@).insert(to@),
                self@.1 == old(self)@.1.insert((from@, to@, label@));

        fn get_arc_label(&self, from: &V, to: &V) -> (result: Option<&L>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                result.is_some() == (exists |l: L::V| #![auto] self@.1.contains((from@, to@, l))),
                result.is_some() ==> self@.1.contains((from@, to@, result.unwrap()@));

        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires valid_key_type_LabEdge::<V, L>()
            ensures b == (exists |l: L::V| #![auto] self@.1.contains((from@, to@, l)));

        fn out_neighbors(&self, v: &V) -> (result: SetStEph<V>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures result@ == self.spec_out_neighbors(v@);

        fn in_neighbors(&self, v: &V) -> (result: SetStEph<V>)
            requires valid_key_type_LabEdge::<V, L>()
            ensures result@ == self.spec_in_neighbors(v@);
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

        fn arcs(&self) -> (result: SetStEph<Edge<V>>) {
            let mut arcs: SetStEph<Edge<V>> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    arcs@ == Set::new(|e: (V::V, V::V)| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |e: (V::V, V::V)| #![auto] arcs@.contains(e) implies 
                                self.spec_arcs().contains(e) by {
                                if arcs@.contains(e) {
                                    let i = choose |i: int| #![auto] 0 <= i < la_seq.len() && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |e: (V::V, V::V)| #![auto] self.spec_arcs().contains(e) implies 
                                arcs@.contains(e) by {
                                if self.spec_arcs().contains(e) {
                                    let l = choose |l: L::V| #![auto] la_view.contains((e.0, e.1, l));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (e.0, e.1, l));
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

        fn get_arc_label(&self, from: &V, to: &V) -> (result: Option<&L>) {
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.1;
            let ghost from_view = from@;
            let ghost to_view = to@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![auto] 0 <= i < it@.0 ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |l: L::V| !la_view.contains((from_view, to_view, l)) by {
                                if la_view.contains((from_view, to_view, l)) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                                }
                            }
                        }
                        return None;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            proof {
                                let idx = it@.0 - 1;
                                crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, idx);
                                assert(la_view.contains((from_view, to_view, labeled_arc.2@)));
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
            let ghost la_view = self@.1;
            let ghost from_view = from@;
            let ghost to_view = to@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![auto] 0 <= i < it@.0 ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            // No arc found - prove spec_arcs doesn't contain (from, to)
                            let pred = |e: (V::V, V::V)| exists |l: L::V| #![auto] la_view.contains((e.0, e.1, l));
                            vstd::set::axiom_set_new(pred, (from_view, to_view));
                            // Now we have: Set::new(pred).contains((from_view, to_view)) == pred((from_view, to_view))
                            // Need to show pred((from_view, to_view)) is false
                            assert forall |l: L::V| !la_view.contains((from_view, to_view, l)) by {
                                if la_view.contains((from_view, to_view, l)) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                                }
                            }
                        }
                        return false;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            proof {
                                let idx = it@.0 - 1;
                                crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, idx);
                                let arc_view = la_seq[idx]@;
                                let witness_l = arc_view.2;
                                assert(self@.1.contains((from_view, to_view, witness_l)));
                            }
                            return true;
                        }
                    },
                }
            }
        }

        fn out_neighbors(&self, v: &V) -> (result: SetStEph<V>) {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost v_view = v@;
            let ghost la_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    neighbors@ == Set::new(|w: V::V| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #![auto] neighbors@.contains(w) implies 
                                self.spec_out_neighbors(v_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![auto] 0 <= i < la_seq.len() && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |w: V::V| #![auto] self.spec_out_neighbors(v_view).contains(w) implies 
                                neighbors@.contains(w) by {
                                if self.spec_out_neighbors(v_view).contains(w) {
                                    let l = choose |l: L::V| #![auto] la_view.contains((v_view, w, l));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (v_view, w, l));
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

        fn in_neighbors(&self, v: &V) -> (result: SetStEph<V>) {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost v_view = v@;
            let ghost la_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    neighbors@ == Set::new(|u: V::V| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |u: V::V| #![auto] neighbors@.contains(u) implies 
                                self.spec_in_neighbors(v_view).contains(u) by {
                                if neighbors@.contains(u) {
                                    let i = choose |i: int| #![auto] 0 <= i < la_seq.len() && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |u: V::V| #![auto] self.spec_in_neighbors(v_view).contains(u) implies 
                                neighbors@.contains(u) by {
                                if self.spec_in_neighbors(v_view).contains(u) {
                                    let l = choose |l: L::V| #![auto] la_view.contains((u, v_view, l));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (u, v_view, l));
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
