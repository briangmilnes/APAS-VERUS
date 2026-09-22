// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled arc filtering (n_plus, n_minus) are parallel.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 5a. view impls
//	Section 6a. spec fns
//	Section 8a. traits
//	Section 9a. impls
//	Section 10a. iterators
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 5c. view impls
//	Section 8c. traits
//	Section 9c. impls
//	Section 11b. top level coarse locking
//	Section 12a. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module


pub mod LabDirGraphMtEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::into_iter_hash_keys;
    use crate::Types::Types::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::vstdplus::accept::accept;

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::*;
    use vstd::rwlock::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::set::group_set_lemmas,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::set_lib::group_set_lib_default,
    };

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabDirGraphMtEph<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> {
        pub vertices: SetStEph<V>,
        pub labeled_arcs: SetStEph<LabEdge<V, L>>,
    }

    //		Section 5a. view impls


    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> View for LabDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_arcs@ }
        }
    }

    //		Section 6a. spec fns


    pub open spec fn valid_key_type_for_lab_graph<V: StTInMtT + Hash, L: StTInMtT + Hash>() -> bool {
        valid_key_type_LabEdge::<V, L>()
    }

    //		Section 8a. traits


    pub trait LabDirGraphMtEphTrait<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static>
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized
    {
        spec fn spec_labdirgraphmteph_wf(&self) -> bool;

        open spec fn spec_vertices(&self) -> Set<V::V> { self@.V }
        open spec fn spec_labeled_arcs(&self) -> Set<(V::V, V::V, L::V)> { self@.A }

        open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
            self@.A.map(|e: (V::V, V::V, L::V)| (e.0, e.1))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_lab_graph::<V, L>()
            ensures
                g.spec_labdirgraphmteph_wf(),
                spec_labgraphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |A|), Span O(1)
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                valid_key_type_for_lab_graph::<V, L>(),
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_arcs@.contains((u, w, l)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                g.spec_labdirgraphmteph_wf(),
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential map
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>)
            requires spec_labgraphview_wf(self@), valid_key_type_for_lab_graph::<V, L>(), valid_key_type_Edge::<V>()
            ensures arcs@ == self.spec_arcs();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_vertex(&mut self, v: V)
            requires spec_labgraphview_wf(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures spec_labgraphview_wf(self@), self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L)
            requires spec_labgraphview_wf(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures 
                spec_labgraphview_wf(self@),
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, label@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential search
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>)
            requires spec_labgraphview_wf(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures
                label.is_some() == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l))),
                label.is_some() ==> self@.A.contains((from@, to@, label.unwrap()@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential search
        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires spec_labgraphview_wf(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures b == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l)));

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        {
            self@.V.filter(|w: V::V| exists |l: L::V| #![trigger self@.A.contains((v, w, l))] self@.A.contains((v, w, l)))
        }

        open spec fn spec_n_plus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V, L::V)>) -> Set<V::V>
            recommends
                spec_labgraphview_wf(self@),
                subarcs <= self@.A,
        {
            self@.V.filter(|w: V::V| exists |l: L::V| #![trigger subarcs.contains((v, w, l))] subarcs.contains((v, w, l)))
        }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        {
            self@.V.filter(|u: V::V| exists |l: L::V| #![trigger self@.A.contains((u, v, l))] self@.A.contains((u, v, l)))
        }

        open spec fn spec_n_minus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V, L::V)>) -> Set<V::V>
            recommends
                spec_labgraphview_wf(self@),
                subarcs <= self@.A,
        {
            self@.V.filter(|u: V::V| exists |l: L::V| #![trigger subarcs.contains((u, v, l))] subarcs.contains((u, v, l)))
        }

        /// out-neighbors
        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — ParaPair! split arcs
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>)
            requires
                spec_labgraphview_wf(self@),
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures
                n_plus.spec_setsteph_wf(),
                n_plus@ == self.spec_n_plus(v@),
                n_plus@ <= self@.V;

        /// in-neighbors
        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — ParaPair! split arcs
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>)
            requires
                spec_labgraphview_wf(self@),
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures
                n_minus.spec_setsteph_wf(),
                n_minus@ == self.spec_n_minus(v@),
                n_minus@ <= self@.V;

        /// Parallel out-neighbor arc filtering using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on labeled arcs
        fn n_plus_par(&self, v: V, arcs: SetStEph<LabEdge<V, L>>) -> (n_plus: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type_LabEdge::<V, L>(),
                spec_labgraphview_wf(self@),
                arcs@ <= self@.A,
            ensures
                n_plus.spec_setsteph_wf(),
                n_plus@ == self.spec_n_plus_from_set(v@, arcs@),
                n_plus@ <= self.spec_n_plus(v@)
            decreases arcs@.len();

        /// Parallel in-neighbor arc filtering using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on labeled arcs
        fn n_minus_par(&self, v: V, arcs: SetStEph<LabEdge<V, L>>) -> (n_minus: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type_LabEdge::<V, L>(),
                spec_labgraphview_wf(self@),
                arcs@ <= self@.A,
            ensures
                n_minus.spec_setsteph_wf(),
                n_minus@ == self.spec_n_minus_from_set(v@, arcs@),
                n_minus@ <= self.spec_n_minus(v@)
            decreases arcs@.len();
    }

    //		Section 9a. impls


    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> LabDirGraphMtEphTrait<V, L>
        for LabDirGraphMtEph<V, L>
    {
        open spec fn spec_labdirgraphmteph_wf(&self) -> bool {
            spec_labgraphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self) {
            LabDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_arcs: SetStEph::empty(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self) {
            LabDirGraphMtEph { vertices, labeled_arcs }
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
                    valid_key_type_Edge::<V>(),
                    arcs.spec_setsteph_wf(),
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
                        // Veracity: NEEDED proof block (speed hint)
                        proof {
                            // Veracity: NEEDED assert
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
        fn add_vertex(&mut self, v: V) {
            let _ = self.vertices.insert(v);
            proof {
                assert forall |u: V::V, w: V::V, l: L::V| #[trigger] self@.A.contains((u, w, l))
                    implies self@.V.contains(u) && self@.V.contains(w) by {
                    assert(old(self)@.A.contains((u, w, l)));
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            let _ = self.vertices.insert(from.clone_plus());
            let _ = self.vertices.insert(to.clone_plus());
            let _ = self.labeled_arcs.insert(LabEdge(from, to, label));
            proof {
                assert forall |u: V::V, w: V::V, l: L::V| #[trigger] self@.A.contains((u, w, l))
                    implies self@.V.contains(u) && self@.V.contains(w) by {
                    if old(self)@.A.contains((u, w, l)) {
                        assert(old(self)@.V.contains(u) && old(self)@.V.contains(w));
                    }
                }
            }
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
                        // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED proof block
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
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
                // Veracity: NEEDED proof block
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            if exists |l: L::V| #![trigger la_view.contains((from_view, to_view, l))] la_view.contains((from_view, to_view, l)) {
                                let l = choose |l: L::V| #![trigger la_view.contains((from_view, to_view, l))] la_view.contains((from_view, to_view, l));
                                lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                            }
                        }
                        return false;
                    // Veracity: NEEDED proof block
                    },
                    Some(labeled_arc) => {
                        proof { pos = pos + 1; assert(la_seq[old_pos] == *labeled_arc); }
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_seq_index_in_map_to_set(la_seq, old_pos);
                            }
                            return true;
                        }
                    },
                }
            }
        }

        /// out-neighbors
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- delegates to n_plus_par
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>) {
            let arcs = self.labeled_arcs.clone();
            self.n_plus_par(v.clone_plus(), arcs)
        }

        /// in-neighbors
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- delegates to n_minus_par
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>) {
            let arcs = self.labeled_arcs.clone();
            self.n_minus_par(v.clone_plus(), arcs)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on labeled arcs
        fn n_plus_par(&self, v: V, arcs: SetStEph<LabEdge<V, L>>) -> (n_plus: SetStEph<V>)
            decreases arcs@.len()
        {
            let n = arcs.size();
            if n == 0 {
                SetStEph::empty()
            // Veracity: NEEDED proof block
            }
            else if n == 1 {
                let LabEdge(from, to, label) = arcs.choose();
                if feq(&from, &v) {
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall |w: V::V| #![trigger Set::empty().insert(to@).contains(w)] Set::empty().insert(to@).contains(w) implies
                            self.spec_n_plus_from_set(v@, arcs@).contains(w) by {
                        }
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall |w: V::V| #![trigger Set::empty().insert(to@).contains(w)] self.spec_n_plus_from_set(v@, arcs@).contains(w) implies
                            Set::empty().insert(to@).contains(w) by {
                            let l = choose |l: L::V| arcs@.contains((v@, w, l));
                            if (v@, w, l) != (from@, to@, label@) {
                            // Veracity: NEEDED proof block
                            }
                        }
                    }
                    SetStEph::singleton(to.clone_plus())
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                    }
                    SetStEph::empty()
                }
            }
            else {
                let mid = n / 2;
                let (left_arcs, right_arcs) = arcs.split(mid);
                let v_left  = v.clone_plus();
                let v_right = v.clone_plus();
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_n_plus_from_set(v_left@, left_arcs@)
                { g_left.n_plus_par(v_left, left_arcs) };

                // Veracity: NEEDED proof block
                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_n_plus_from_set(v_right@, right_arcs@)
                { g_right.n_plus_par(v_right, right_arcs) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |w: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(w)] left_neighbors@.union(right_neighbors@).contains(w) implies
                        self.spec_n_plus_from_set(v@, arcs@).contains(w) by {
                        if left_neighbors@.contains(w) {
                            let l = choose |l: L::V| left_arcs@.contains((v@, w, l));
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(arcs@.contains((v@, w, l)));
                        } else {
                            let l = choose |l: L::V| right_arcs@.contains((v@, w, l));
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(arcs@.contains((v@, w, l)));
                        }
                    }
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |w: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(w)] self.spec_n_plus_from_set(v@, arcs@).contains(w) implies
                        left_neighbors@.union(right_neighbors@).contains(w) by {
                        let l = choose |l: L::V| arcs@.contains((v@, w, l));
                        if left_arcs@.contains((v@, w, l)) {
                        } else {
                        }
                    }
                }

                left_neighbors.union(&right_neighbors)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on labeled arcs
        fn n_minus_par(&self, v: V, arcs: SetStEph<LabEdge<V, L>>) -> (n_minus: SetStEph<V>)
            decreases arcs@.len()
        {
            // Veracity: NEEDED proof block
            let n = arcs.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let LabEdge(from, to, label) = arcs.choose();
                if feq(&to, &v) {
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall |u: V::V| #![trigger Set::empty().insert(from@).contains(u)] Set::empty().insert(from@).contains(u) implies
                            self.spec_n_minus_from_set(v@, arcs@).contains(u) by {
                        }
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall |u: V::V| #![trigger Set::empty().insert(from@).contains(u)] self.spec_n_minus_from_set(v@, arcs@).contains(u) implies
                            // Veracity: NEEDED proof block
                            Set::empty().insert(from@).contains(u) by {
                            let l = choose |l: L::V| arcs@.contains((u, v@, l));
                            if (u, v@, l) != (from@, to@, label@) {
                            }
                        }
                    }
                    SetStEph::singleton(from.clone_plus())
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                    }
                    SetStEph::empty()
                }
            }
            else {
                let mid = n / 2;
                let (left_arcs, right_arcs) = arcs.split(mid);
                let v_left  = v.clone_plus();
                let v_right = v.clone_plus();
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    // Veracity: NEEDED proof block
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_n_minus_from_set(v_left@, left_arcs@)
                { g_left.n_minus_par(v_left, left_arcs) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_n_minus_from_set(v_right@, right_arcs@)
                { g_right.n_minus_par(v_right, right_arcs) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |u: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(u)] left_neighbors@.union(right_neighbors@).contains(u) implies
                        self.spec_n_minus_from_set(v@, arcs@).contains(u) by {
                        if left_neighbors@.contains(u) {
                            let l = choose |l: L::V| left_arcs@.contains((u, v@, l));
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(arcs@.contains((u, v@, l)));
                        } else {
                            let l = choose |l: L::V| right_arcs@.contains((u, v@, l));
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(arcs@.contains((u, v@, l)));
                        }
                    }
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |u: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(u)] self.spec_n_minus_from_set(v@, arcs@).contains(u) implies
                        left_neighbors@.union(right_neighbors@).contains(u) by {
                        let l = choose |l: L::V| arcs@.contains((u, v@, l));
                        if left_arcs@.contains((u, v@, l)) {
                        } else {
                        }
                    }
                }

                left_neighbors.union(&right_neighbors)
            }
        }
    }

    //		Section 10a. iterators


    // Delegated iteration over the vertices: the std hash-set iterator that vstd
    // specifies. An impl of an external trait method may not add `requires`, so
    // the contract is conditional on the vertex set's well-formedness.
    impl<'a, V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> std::iter::IntoIterator for &'a LabDirGraphMtEph<V, L> {
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

    //		Section 4b. type definitions


    #[derive(Debug)]
    pub struct LabDirGraphMtEphInv;

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LockedLabDirGraphMtEph<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> {
        pub(crate) locked_graph: RwLock<LabDirGraphMtEph<V, L>, LabDirGraphMtEphInv>,
        pub(crate) ghost_locked_graph: Ghost<LabGraphView<<V as View>::V, <L as View>::V>>,
    }

    //		Section 5c. view impls


    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> View for LockedLabDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V { self.spec_ghost_locked_graph() }
    }

    //		Section 8c. traits


    pub trait LockedLabDirGraphMtEphTrait<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static>
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized
    {
        spec fn spec_labdirgraphmteph_wf(&self) -> bool;

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        { self@.V.filter(|w: V::V| exists |l: L::V| #![trigger self@.A.contains((v, w, l))] self@.A.contains((v, w, l))) }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        { self@.V.filter(|u: V::V| exists |l: L::V| #![trigger self@.A.contains((u, v, l))] self@.A.contains((u, v, l))) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn new(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (s: Self)
            requires
                valid_key_type_for_lab_graph::<V, L>(),
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_arcs@.contains((u, w, l)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                s.spec_labdirgraphmteph_wf(),
                s@.V == vertices@,
                s@.A == labeled_arcs@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_vertex(&mut self, v: V) -> (added: std::result::Result<(), ()>)
            requires old(self).spec_labdirgraphmteph_wf()
            ensures
                self.spec_labdirgraphmteph_wf(),
                match added {
                    Ok(_) => self@.V == old(self)@.V.insert(v@) && self@.A == old(self)@.A,
                    Err(_) => self@ == old(self)@,
                };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) -> (added: std::result::Result<(), ()>)
            requires old(self).spec_labdirgraphmteph_wf()
            ensures
                self.spec_labdirgraphmteph_wf(),
                match added {
                    Ok(_) => self@.V == old(self)@.V.insert(from@).insert(to@)
                          && self@.A == old(self)@.A.insert((from@, to@, label@)),
                    Err(_) => self@ == old(self)@,
                };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>)
            requires
                self.spec_labdirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                n_plus.spec_setsteph_wf(),
                n_plus@ == self.spec_n_plus(v@),
                n_plus@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>)
            requires
                self.spec_labdirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                n_minus.spec_setsteph_wf(),
                n_minus@ == self.spec_n_minus(v@),
                n_minus@ <= self@.V;
    }

    //		Section 9c. impls


    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> LockedLabDirGraphMtEph<V, L> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            spec_labgraphview_wf(self.ghost_locked_graph@)
        }

        pub closed spec fn spec_ghost_locked_graph(self) -> LabGraphView<<V as View>::V, <L as View>::V> {
            self.ghost_locked_graph@
        }
    }

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> LockedLabDirGraphMtEphTrait<V, L> for LockedLabDirGraphMtEph<V, L> {
        open spec fn spec_labdirgraphmteph_wf(&self) -> bool {
            spec_labgraphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn new(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (s: Self) {
            let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(vertices, labeled_arcs);
            // Veracity: NEEDED proof block
            let ghost gv = g@;
            LockedLabDirGraphMtEph {
                locked_graph: RwLock::new(g, Ghost(LabDirGraphMtEphInv)),
                ghost_locked_graph: Ghost(gv),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_vertex(&mut self, v: V) -> (added: std::result::Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_graph.acquire_write();
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof { assume(self.ghost_locked_graph@ == locked_val@); }
            locked_val.add_vertex(v);
            let ghost new_val = locked_val@;
            self.ghost_locked_graph = Ghost(new_val);
            write_handle.release_write(locked_val);
            Ok(())
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_labeled_arc(&mut self, from: V, to: V, label: L) -> (added: std::result::Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_graph.acquire_write();
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof { assume(self.ghost_locked_graph@ == locked_val@); }
            locked_val.add_labeled_arc(from, to, label);
            let ghost new_val = locked_val@;
            self.ghost_locked_graph = Ghost(new_val);
            write_handle.release_write(locked_val);
            Ok(())
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>) {
            // Veracity: NEEDED proof block
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let n_plus = inner.n_plus(v);
            read_handle.release_read();
            n_plus
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let n_minus = inner.n_minus(v);
            read_handle.release_read();
            n_minus
        }
    }

    //		Section 11b. top level coarse locking


    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> RwLockPredicate<LabDirGraphMtEph<V, L>> for LabDirGraphMtEphInv {
        open spec fn inv(self, v: LabDirGraphMtEph<V, L>) -> bool {
            spec_labgraphview_wf(v@) && valid_key_type_for_lab_graph::<V, L>()
        }
    }

    //		Section 12a. derive impls in verus!


    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> Clone for LabDirGraphMtEph<V, L> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            LabDirGraphMtEph { vertices: self.vertices.clone(), labeled_arcs: self.labeled_arcs.clone() }
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14a. derive impls outside verus!

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

    //		Section 14b. derive impls outside verus!

    impl Display for LabDirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabDirGraphMtEphInv") }
    }

    //		Section 14c. derive impls outside verus!

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> Debug for LockedLabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedLabDirGraphMtEph") }
    }

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> Display for LockedLabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedLabDirGraphMtEph") }
    }
}
