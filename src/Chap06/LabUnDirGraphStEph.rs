// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges.


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

pub mod LabUnDirGraphStEph {


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
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::set::group_set_axioms,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabUnDirGraphStEph<V: HashOrd, L: StT + Hash> {
        pub vertices: SetStEph<V>,
        pub labeled_edges: SetStEph<LabEdge<V, L>>,
    }

    //		Section 5. view impls


    impl<V: HashOrd, L: StT + Hash> View for LabUnDirGraphStEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;

        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_edges@ }
        }
    }

    //		Section 8. traits


    pub trait LabUnDirGraphStEphTrait<V: HashOrd, L: StT + Hash>:
    View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized {

        spec fn spec_labundirgraphsteph_wf(&self) -> bool;

        open spec fn spec_finite(&self) -> bool {
            self@.V.finite() && self@.A.finite()
        }

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> 
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| exists |l: L::V| 
                self@.A.contains((v, w, l)) || self@.A.contains((w, v, l)))
        }

        open spec fn spec_edges(&self) -> Set<(V::V, V::V)> {
            Set::new(|e: (V::V, V::V)| exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_LabEdge::<V, L>()
            ensures
                g.spec_labundirgraphsteph_wf(),
                spec_labgraphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — sequential
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_edges@.contains((u, w, l)) ==>
                        vertices@.contains(u) && vertices@.contains(w),
            ensures
                g.spec_labundirgraphsteph_wf(),
                spec_labgraphview_wf(g@),
                g@.V =~= vertices@,
                g@.A =~= labeled_edges@;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn labeled_edges(&self) -> (e: &SetStEph<LabEdge<V, L>>)
            ensures e@ =~= self@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) — sequential map
        fn edges(&self) -> (edges: SetStEph<Edge<V>>)
            requires valid_key_type_LabEdge::<V, L>(), valid_key_type_Edge::<V>()
            ensures 
                forall |e: (V::V, V::V)| edges@.contains(e) == (exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_vertex(&mut self, v: V)
            requires valid_key_type_LabEdge::<V, L>()
            ensures self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L)
            requires valid_key_type_LabEdge::<V, L>()
            ensures 
                self@.V == old(self)@.V.insert(v1@).insert(v2@),
                self@.A == old(self)@.A.insert((v1@, v2@, label@)) || 
                self@.A == old(self)@.A.insert((v2@, v1@, label@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) — sequential search
        fn get_edge_label(&self, v1: &V, v2: &V) -> (label: Option<&L>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures 
                label.is_some() == (exists |l: L::V| 
                    self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l))),
                label.is_some() ==> (self@.A.contains((v1@, v2@, label.unwrap()@)) || 
                                      self@.A.contains((v2@, v1@, label.unwrap()@)));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) — sequential search
        fn has_edge(&self, v1: &V, v2: &V) -> (b: bool)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures b == (exists |l: L::V| 
                self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l)));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) — sequential filter
        fn ng(&self, v: &V) -> (ng: SetStEph<V>)
            requires spec_labgraphview_wf(self@), valid_key_type_LabEdge::<V, L>()
            ensures ng@ == self.spec_ng(v@), ng.spec_setsteph_wf();
    }

    //		Section 9. impls


    impl<V: HashOrd, L: StT + Hash> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L> {

        open spec fn spec_labundirgraphsteph_wf(&self) -> bool {
            spec_labgraphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: LabUnDirGraphStEph<V, L>)
            ensures g.spec_labundirgraphsteph_wf()
        {
            LabUnDirGraphStEph { vertices: SetStEph::empty(), labeled_edges: SetStEph::empty() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: LabUnDirGraphStEph<V, L>)
            ensures g.spec_labundirgraphsteph_wf()
        {
            LabUnDirGraphStEph { vertices, labeled_edges }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn labeled_edges(&self) -> (e: &SetStEph<LabEdge<V, L>>) { &self.labeled_edges }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
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
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block (speed hint)
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |e: (V::V, V::V)| edges@.contains(e) implies 
                                (exists |l: L::V| #![trigger le_view.contains((e.0, e.1, l))] le_view.contains((e.0, e.1, l))) by {
                                if edges@.contains(e) {
                                    let i = choose |i: int| #![trigger le_seq[i]] 0 <= i < le_seq.len() && le_seq[i]@.0 == e.0 && le_seq[i]@.1 == e.1;
                                    lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_vertex(&mut self, v: V) { let _ = self.vertices.insert(v); }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {
            let _ = self.vertices.insert(v1.clone_plus());
            let _ = self.vertices.insert(v2.clone_plus());
            if v1 <= v2 {
                let _ = self.labeled_edges.insert(LabEdge(v1, v2, label));
            } else {
                let _ = self.labeled_edges.insert(LabEdge(v2, v1, label));
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
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
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        return None;
                    },
                    Some(labeled_edge) => {
                        if (feq(&labeled_edge.0, v1) && feq(&labeled_edge.1, v2)) || 
                           // Veracity: NEEDED proof block
                           (feq(&labeled_edge.0, v2) && feq(&labeled_edge.1, v1)) {
                            // Veracity: NEEDED proof block
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
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
                // Veracity: NEEDED proof block
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        return false;
                    },
                    // Veracity: NEEDED proof block
                    Some(labeled_edge) => {
                        if (feq(&labeled_edge.0, v1) && feq(&labeled_edge.1, v2)) || 
                           (feq(&labeled_edge.0, v2) && feq(&labeled_edge.1, v1)) {
                            // Veracity: NEEDED proof block
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
        fn ng(&self, v: &V) -> (ng: SetStEph<V>) {
            let mut ng: SetStEph<V> = SetStEph::empty();
            let mut it = self.labeled_edges.iter();
            let ghost le_seq = it@.1;
            let ghost v_view = v@;
            let ghost le_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    ng.spec_setsteph_wf(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == le_view,
                    ng@ == Set::new(|w: V::V|
                        exists |i: int| #![trigger le_seq[i]] 0 <= i < it@.0 &&
                            ((le_seq[i]@.0 == v_view && le_seq[i]@.1 == w) ||
                             (le_seq[i]@.1 == v_view && le_seq[i]@.0 == w))),
                // Veracity: NEEDED proof block
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] ng@.contains(w) implies 
                                self.spec_ng(v_view).contains(w) by {
                                if ng@.contains(w) {
                                    let i = choose |i: int| #![trigger le_seq[i]] 0 <= i < le_seq.len() && 
                                        ((le_seq[i]@.0 == v_view && le_seq[i]@.1 == w) ||
                                         (le_seq[i]@.1 == v_view && le_seq[i]@.0 == w));
                                    lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_ng(v_view).contains(w) implies 
                                ng@.contains(w) by {
                                if self.spec_ng(v_view).contains(w) {
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
                        return ng;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.0, v) {
                            let _ = ng.insert(labeled_edge.1.clone_plus());
                        } else if feq(&labeled_edge.1, v) {
                            let _ = ng.insert(labeled_edge.0.clone_plus());
                        }
                    },
                }
            }
        }
    }

    //		Section 10. iterators


    /// Iterator wrapper for LabUnDirGraphStEph vertex iteration.
    #[verifier::reject_recursive_types(V)]
    pub struct LabUnDirGraphStEphIter<'a, V: HashOrd> {
        pub inner: SetStEphIter<'a, V>,
    }

    impl<'a, V: HashOrd> View for LabUnDirGraphStEphIter<'a, V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, V: HashOrd>(it: &LabUnDirGraphStEphIter<'a, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, V: HashOrd> std::iter::Iterator for LabUnDirGraphStEphIter<'a, V> {
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
    pub struct LabUnDirGraphStEphGhostIterator<'a, V: HashOrd> {
        pub pos: int,
        pub elements: Seq<V>,
        pub phantom: core::marker::PhantomData<&'a V>,
    }

    impl<'a, V: HashOrd> vstd::pervasive::ForLoopGhostIteratorNew for LabUnDirGraphStEphIter<'a, V> {
        type GhostIter = LabUnDirGraphStEphGhostIterator<'a, V>;

        open spec fn ghost_iter(&self) -> LabUnDirGraphStEphGhostIterator<'a, V> {
            LabUnDirGraphStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, V: HashOrd> vstd::pervasive::ForLoopGhostIterator for LabUnDirGraphStEphGhostIterator<'a, V> {
        type ExecIter = LabUnDirGraphStEphIter<'a, V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &LabUnDirGraphStEphIter<'a, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &LabUnDirGraphStEphIter<'a, V>) -> LabUnDirGraphStEphGhostIterator<'a, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, V: HashOrd> View for LabUnDirGraphStEphGhostIterator<'a, V> {
        type V = Seq<V>;

        open spec fn view(&self) -> Seq<V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, V: HashOrd, L: StT + Hash> std::iter::IntoIterator for &'a LabUnDirGraphStEph<V, L> {
        type Item = &'a V;
        type IntoIter = LabUnDirGraphStEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
                iter_invariant(&it),
        {
            LabUnDirGraphStEphIter { inner: self.vertices().iter() }
        }
    }

    //		Section 12. derive impls in verus!


    impl<V: HashOrd, L: StT + Hash> Clone for LabUnDirGraphStEph<V, L> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            LabUnDirGraphStEph { vertices: self.vertices.clone(), labeled_edges: self.labeled_edges.clone() }
        }
    }

} // verus!

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

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

    impl<'a, V: HashOrd> Debug for LabUnDirGraphStEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphStEphIter") }
    }

    impl<'a, V: HashOrd> Display for LabUnDirGraphStEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphStEphIter") }
    }

    impl<'a, V: HashOrd> Debug for LabUnDirGraphStEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphStEphGhostIterator") }
    }

    impl<'a, V: HashOrd> Display for LabUnDirGraphStEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphStEphGhostIterator") }
    }
}
