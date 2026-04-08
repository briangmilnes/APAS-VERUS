//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod LabDirGraphStEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::seq_set::*;

verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabDirGraphStEph<V: StT + Hash, L: StT + Hash> {
        pub vertices: SetStEph<V>,
        pub labeled_arcs: SetStEph<LabEdge<V, L>>,
    }

    //		Section 5. view impls


    impl<V: StT + Hash, L: StT + Hash> View for LabDirGraphStEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;

        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_arcs@ }
        }
    }

    //		Section 8. traits


    pub trait LabDirGraphStEphTrait<V: StT + Hash, L: StT + Hash>:
    View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized {

        spec fn spec_labdirgraphsteph_wf(&self) -> bool;

        open spec fn spec_finite(&self) -> bool {
            self@.V.finite() && self@.A.finite()
        }

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V> 
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| exists |l: L::V| #![trigger self@.A.contains((v, w, l))] self@.A.contains((v, w, l)))
        }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V> 
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        { 
            Set::new(|u: V::V| exists |l: L::V| #![trigger self@.A.contains((u, v, l))] self@.A.contains((u, v, l)))
        }

        open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
            Set::new(|e: (V::V, V::V)| exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_LabEdge::<V, L>()
            ensures
                g.spec_labdirgraphsteph_wf(),
                spec_labgraphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |A|), Span O(|V| + |A|) — DIFFERS: St sequential, APAS Span O(1)
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_arcs@.contains((u, w, l)) ==>
                        vertices@.contains(u) && vertices@.contains(w),
            ensures
                g.spec_labdirgraphsteph_wf(),
                spec_labgraphview_wf(g@),
                g@.V =~= vertices@,
                g@.A =~= labeled_arcs@;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>)
            ensures a@ =~= self@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — DIFFERS: St sequential, APAS Span O(1)
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>)
            requires valid_key_type_LabEdge::<V, L>(), valid_key_type_Edge::<V>()
            ensures arcs@.finite(), arcs@ == self.spec_arcs();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_vertex(&mut self, v: V)
            requires valid_key_type_LabEdge::<V, L>()
            ensures self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, label@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — DIFFERS: St sequential, APAS Span O(1)
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures 
                label.is_some() == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l))),
                label.is_some() ==> self@.A.contains((from@, to@, label.unwrap()@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — DIFFERS: St sequential, APAS Span O(1)
        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures b == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l)));

        /// out-neighbors
        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — DIFFERS: St sequential, APAS Span O(1)
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures n_plus@.finite(), n_plus@ == self.spec_n_plus(v@);

        /// in-neighbors
        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — DIFFERS: St sequential, APAS Span O(1)
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures n_minus@.finite(), n_minus@ == self.spec_n_minus(v@);
    }

    //		Section 9. impls


    impl<V: StT + Hash, L: StT + Hash> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L> {

        open spec fn spec_labdirgraphsteph_wf(&self) -> bool {
            spec_labgraphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: LabDirGraphStEph<V, L>)
            ensures g.spec_labdirgraphsteph_wf()
        {
            LabDirGraphStEph { vertices: SetStEph::empty(), labeled_arcs: SetStEph::empty() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: LabDirGraphStEph<V, L>)
            ensures g.spec_labdirgraphsteph_wf()
        {
            LabDirGraphStEph { vertices, labeled_arcs }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>) { &self.labeled_arcs }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential scan of labeled arcs
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
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |e: (V::V, V::V)| #[trigger] arcs@.contains(e) implies 
                                self.spec_arcs().contains(e) by {
                                if arcs@.contains(e) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_vertex(&mut self, v: V) { let _ = self.vertices.insert(v); }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            let _ = self.vertices.insert(from.clone_plus());
            let _ = self.vertices.insert(to.clone_plus());
            let _ = self.labeled_arcs.insert(LabEdge(from, to, label));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential scan of labeled arcs
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
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        return None;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            // Veracity: NEEDED proof block
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential scan of labeled arcs
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
                        // Veracity: NEEDED proof block
                        proof {
                            // No arc found - prove spec_arcs doesn't contain (from, to)
                            let pred = |e: (V::V, V::V)| exists |l: L::V| #![trigger la_view.contains((e.0, e.1, l))] la_view.contains((e.0, e.1, l));
                            vstd::set::axiom_set_new(pred, (from_view, to_view));
                            // Now we have: Set::new(pred).contains((from_view, to_view)) == pred((from_view, to_view))
                            // Need to show pred((from_view, to_view)) is false
                        }
                        return false;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            // Veracity: NEEDED proof block
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential scan of labeled arcs
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
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] neighbors@.contains(w) implies 
                                self.spec_n_plus(v_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential scan of labeled arcs
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
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |u: V::V| #[trigger] neighbors@.contains(u) implies 
                                self.spec_n_minus(v_view).contains(u) by {
                                if neighbors@.contains(u) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
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

    //		Section 10. iterators


    /// Iterator wrapper for LabDirGraphStEph vertex iteration.
    #[verifier::reject_recursive_types(V)]
    pub struct LabDirGraphStEphIter<'a, V: StT + Hash> {
        pub inner: SetStEphIter<'a, V>,
    }

    impl<'a, V: StT + Hash> View for LabDirGraphStEphIter<'a, V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, V: StT + Hash>(it: &LabDirGraphStEphIter<'a, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, V: StT + Hash> std::iter::Iterator for LabDirGraphStEphIter<'a, V> {
        type Item = &'a V;

        fn next(&mut self) -> (next: Option<&'a V>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(V)]
    pub struct LabDirGraphStEphGhostIterator<'a, V: StT + Hash> {
        pub pos: int,
        pub elements: Seq<V>,
        pub phantom: core::marker::PhantomData<&'a V>,
    }

    impl<'a, V: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for LabDirGraphStEphIter<'a, V> {
        type GhostIter = LabDirGraphStEphGhostIterator<'a, V>;

        open spec fn ghost_iter(&self) -> LabDirGraphStEphGhostIterator<'a, V> {
            LabDirGraphStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, V: StT + Hash> vstd::pervasive::ForLoopGhostIterator for LabDirGraphStEphGhostIterator<'a, V> {
        type ExecIter = LabDirGraphStEphIter<'a, V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &LabDirGraphStEphIter<'a, V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<V> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &LabDirGraphStEphIter<'a, V>) -> LabDirGraphStEphGhostIterator<'a, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, V: StT + Hash> View for LabDirGraphStEphGhostIterator<'a, V> {
        type V = Seq<V>;

        open spec fn view(&self) -> Seq<V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, V: StT + Hash, L: StT + Hash> std::iter::IntoIterator for &'a LabDirGraphStEph<V, L> {
        type Item = &'a V;
        type IntoIter = LabDirGraphStEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
                iter_invariant(&it),
        {
            LabDirGraphStEphIter { inner: self.vertices().iter() }
        }
    }

    //		Section 12. derive impls in verus!


    impl<V: StT + Hash, L: StT + Hash> Clone for LabDirGraphStEph<V, L> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            LabDirGraphStEph { vertices: self.vertices.clone(), labeled_arcs: self.labeled_arcs.clone() }
        }
    }

} // verus!

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

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

    impl<'a, V: StT + Hash> Debug for LabDirGraphStEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabDirGraphStEphIter") }
    }

    impl<'a, V: StT + Hash> Display for LabDirGraphStEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabDirGraphStEphIter") }
    }

    impl<'a, V: StT + Hash> Debug for LabDirGraphStEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabDirGraphStEphGhostIterator") }
    }

    impl<'a, V: StT + Hash> Display for LabDirGraphStEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabDirGraphStEphGhostIterator") }
    }
}
