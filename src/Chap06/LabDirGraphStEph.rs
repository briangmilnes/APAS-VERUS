// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::into_iter_hash_keys;
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
        vstd::set::group_set_lemmas,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
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

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        {
            self@.V.filter(|w: V::V| exists |l: L::V| #![trigger self@.A.contains((v, w, l))] self@.A.contains((v, w, l)))
        }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        {
            self@.V.filter(|u: V::V| exists |l: L::V| #![trigger self@.A.contains((u, v, l))] self@.A.contains((u, v, l)))
        }

        open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
            self@.A.map(|e: (V::V, V::V, L::V)| (e.0, e.1))
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |A|), Span O(|V| + |A|) — ACCEPTED DIFFERENCE: St sequential, APAS Span O(1)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — ACCEPTED DIFFERENCE: St sequential, APAS Span O(1)
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>)
            requires valid_key_type_LabEdge::<V, L>(), valid_key_type_Edge::<V>()
            ensures arcs@ == self.spec_arcs();

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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — ACCEPTED DIFFERENCE: St sequential, APAS Span O(1)
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures
                label.is_some() == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l))),
                label.is_some() ==> self@.A.contains((from@, to@, label.unwrap()@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — ACCEPTED DIFFERENCE: St sequential, APAS Span O(1)
        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures b == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l)));

        /// out-neighbors
        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — ACCEPTED DIFFERENCE: St sequential, APAS Span O(1)
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures n_plus@ == self.spec_n_plus(v@);

        /// in-neighbors
        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — ACCEPTED DIFFERENCE: St sequential, APAS Span O(1)
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures n_minus@ == self.spec_n_minus(v@);
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
            let la_iter = self.labeled_arcs.iter();
            let ghost la_seq = into_iter_hash_keys(la_iter);
            let ghost la_view = self@.A;
            let mut it = la_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= la_seq.len(),
                    IteratorSpec::remaining(&it).len() == la_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == la_seq[pos + i],
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |e: (V::V, V::V)| #[trigger] arcs@.contains(e) <==>
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < pos && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1,
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
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
                                    assert(la_view.contains(la_seq[i]@));
                                }
                            }
                            // Veracity: NEEDED assert
                            assert forall |e: (V::V, V::V)| #[trigger] self.spec_arcs().contains(e) implies
                                arcs@.contains(e) by {
                                if self.spec_arcs().contains(e) {
                                    let a = choose |a: (V::V, V::V, L::V)| #[trigger] la_view.contains(a) && e == (a.0, a.1);
                                    lemma_map_to_set_contains_index(la_seq, a);
                                }
                            }
                            assert(arcs@ =~= self.spec_arcs());
                        }
                        return arcs;
                    },
                    Some(labeled_arc) => {
                        proof { pos = pos + 1; assert(la_seq[old_pos] == *labeled_arc); }
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
            let la_iter = self.labeled_arcs.iter();
            let ghost la_seq = into_iter_hash_keys(la_iter);
            let ghost la_view = self@.A;
            let ghost from_view = from@;
            let ghost to_view = to@;
            let mut it = la_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= la_seq.len(),
                    IteratorSpec::remaining(&it).len() == la_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == la_seq[pos + i],
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![trigger la_seq[i]] 0 <= i < pos ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block (speed hint)
                        proof {
                            if exists |l: L::V| #![trigger la_view.contains((from_view, to_view, l))] la_view.contains((from_view, to_view, l)) {
                                let l = choose |l: L::V| #![trigger la_view.contains((from_view, to_view, l))] la_view.contains((from_view, to_view, l));
                                lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                            }
                        }
                        return None;
                    },
                    Some(labeled_arc) => {
                        proof { pos = pos + 1; assert(la_seq[old_pos] == *labeled_arc); }
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            // Veracity: NEEDED proof block
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_seq_index_in_map_to_set(la_seq, old_pos);
                            }
                            return Some(&labeled_arc.2);
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential scan of labeled arcs
        fn has_arc(&self, from: &V, to: &V) -> (b: bool) {
            let la_iter = self.labeled_arcs.iter();
            let ghost la_seq = into_iter_hash_keys(la_iter);
            let ghost la_view = self@.A;
            let ghost from_view = from@;
            let ghost to_view = to@;
            let mut it = la_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= la_seq.len(),
                    IteratorSpec::remaining(&it).len() == la_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == la_seq[pos + i],
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![trigger la_seq[i]] 0 <= i < pos ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    // Veracity: NEEDED proof block
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // No arc found: any labeled arc (from, to, l) in the view would sit at some scanned index.
                            if exists |l: L::V| #![trigger la_view.contains((from_view, to_view, l))] la_view.contains((from_view, to_view, l)) {
                                let l = choose |l: L::V| #![trigger la_view.contains((from_view, to_view, l))] la_view.contains((from_view, to_view, l));
                                lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                            }
                        }
                        return false;
                    },
                    // Veracity: NEEDED proof block
                    Some(labeled_arc) => {
                        proof { pos = pos + 1; assert(la_seq[old_pos] == *labeled_arc); }
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_seq_index_in_map_to_set(la_seq, old_pos);
                                let arc_view = la_seq[old_pos]@;
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
            let la_iter = self.labeled_arcs.iter();
            let ghost la_seq = into_iter_hash_keys(la_iter);
            let ghost v_view = v@;
            let ghost la_view = self@.A;
            let mut it = la_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= la_seq.len(),
                    IteratorSpec::remaining(&it).len() == la_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == la_seq[pos + i],
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |w: V::V| #[trigger] neighbors@.contains(w) <==>
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < pos && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w,
                decreases IteratorSpec::decrease(&it)->0,
            // Veracity: NEEDED proof block
            {
                let ghost old_pos = pos;
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] neighbors@.contains(w) implies
                                self.spec_n_plus(v_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == v_view && la_seq[i]@.1 == w;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                    assert(la_view.contains((v_view, w, la_seq[i]@.2)));
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_n_plus(v_view).contains(w) implies
                                neighbors@.contains(w) by {
                                if self.spec_n_plus(v_view).contains(w) {
                                    let l = choose |l: L::V| #![trigger la_view.contains((v_view, w, l))] la_view.contains((v_view, w, l));
                                    lemma_map_to_set_contains_index(la_seq, (v_view, w, l));
                                }
                            }
                            assert(neighbors@ =~= self.spec_n_plus(v_view));
                        }
                        return neighbors;
                    },
                    Some(labeled_arc) => {
                        proof { pos = pos + 1; assert(la_seq[old_pos] == *labeled_arc); }
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
            let la_iter = self.labeled_arcs.iter();
            let ghost la_seq = into_iter_hash_keys(la_iter);
            let ghost v_view = v@;
            let ghost la_view = self@.A;
            let mut it = la_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= la_seq.len(),
                    IteratorSpec::remaining(&it).len() == la_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == la_seq[pos + i],
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |u: V::V| #[trigger] neighbors@.contains(u) <==>
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < pos && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u,
                // Veracity: NEEDED proof block
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |u: V::V| #[trigger] neighbors@.contains(u) implies
                                self.spec_n_minus(v_view).contains(u) by {
                                if neighbors@.contains(u) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.1 == v_view && la_seq[i]@.0 == u;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                    assert(la_view.contains((u, v_view, la_seq[i]@.2)));
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |u: V::V| #[trigger] self.spec_n_minus(v_view).contains(u) implies
                                neighbors@.contains(u) by {
                                if self.spec_n_minus(v_view).contains(u) {
                                    let l = choose |l: L::V| #![trigger la_view.contains((u, v_view, l))] la_view.contains((u, v_view, l));
                                    lemma_map_to_set_contains_index(la_seq, (u, v_view, l));
                                }
                            }
                            assert(neighbors@ =~= self.spec_n_minus(v_view));
                        }
                        return neighbors;
                    },
                    Some(labeled_arc) => {
                        proof { pos = pos + 1; assert(la_seq[old_pos] == *labeled_arc); }
                        if feq(&labeled_arc.1, v) {
                            let _ = neighbors.insert(labeled_arc.0.clone_plus());
                        }
                    },
                }
            }
        }
    }

    //		Section 10. iterators


    // Delegated iteration over the vertices: the std hash-set iterator that vstd
    // specifies. An impl of an external trait method may not add `requires`, so
    // the contract is conditional on the vertex set's well-formedness.
    impl<'a, V: StT + Hash, L: StT + Hash> std::iter::IntoIterator for &'a LabDirGraphStEph<V, L> {
        type Item = &'a V;
        type IntoIter = std::collections::hash_set::Iter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                self.vertices.spec_setsteph_wf() ==> {
                    &&& IteratorSpec::remaining(&it).unref().map(|i: int, k: V| k@).to_set() == self@.V
                    &&& IteratorSpec::remaining(&it).unref().no_duplicates()
                    &&& IteratorSpec::remaining(&it).len() == self@.V.len()
                    &&& into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref()
                    &&& IteratorSpec::decrease(&it) is Some
                },
        {
            (&self.vertices).into_iter()
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
}
