//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled edge filtering (ng) is parallel.

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


pub mod LabUnDirGraphMtEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
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
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::set_lib::group_set_lib_default,
    };

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabUnDirGraphMtEph<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> {
        pub vertices: SetStEph<V>,
        pub labeled_edges: SetStEph<LabEdge<V, L>>,
    }

    //		Section 5a. view impls


    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> View for LabUnDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_edges@ }
        }
    }

    //		Section 6a. spec fns


    pub open spec fn valid_key_type_for_lab_graph<V: StTInMtT + Hash + Ord, L: StTInMtT + Hash>() -> bool {
        valid_key_type_LabEdge::<V, L>()
    }

    //		Section 8a. traits


    pub trait LabUnDirGraphMtEphTrait<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static>
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized
    {
        spec fn spec_labundirgraphmteph_wf(&self) -> bool;

        open spec fn spec_finite(&self) -> bool {
            self@.V.finite() && self@.A.finite()
        }

        open spec fn spec_vertices(&self) -> Set<V::V> { self@.V }
        open spec fn spec_labeled_edges(&self) -> Set<(V::V, V::V, L::V)> { self@.A }

        open spec fn spec_edges(&self) -> Set<(V::V, V::V)> {
            Set::new(|e: (V::V, V::V)| exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)))
        }

        open spec fn spec_ng_from_set(&self, v: V::V, subedges: Set<(V::V, V::V, L::V)>) -> Set<V::V> 
            recommends 
                spec_labgraphview_wf(self@),
                subedges <= self@.A,
        {
            Set::new(|w: V::V| exists |l: L::V| subedges.contains((v, w, l)) || subedges.contains((w, v, l)))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_lab_graph::<V, L>()
            ensures
                g.spec_labundirgraphmteph_wf(),
                spec_labgraphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(1)
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                valid_key_type_for_lab_graph::<V, L>(),
                vertices@.finite(),
                labeled_edges@.finite(),
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_edges@.contains((u, w, l)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                g.spec_labundirgraphmteph_wf(),
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) — sequential map
        fn edges(&self) -> (edges: SetStEph<Edge<V>>)
            requires spec_labgraphview_wf(self@), valid_key_type_for_lab_graph::<V, L>(), valid_key_type_Edge::<V>()
            ensures forall |u: V::V, w: V::V| edges@.contains((u, w)) ==
                (exists |l: L::V| #![trigger self@.A.contains((u, w, l))] self@.A.contains((u, w, l)));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_vertex(&mut self, v: V)
            requires spec_labgraphview_wf(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures spec_labgraphview_wf(self@), self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L)
            requires spec_labgraphview_wf(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures
                spec_labgraphview_wf(self@),
                self@.V == old(self)@.V.insert(v1@).insert(v2@),
                self@.A == old(self)@.A.insert((v1@, v2@, label@)) ||
                self@.A == old(self)@.A.insert((v2@, v1@, label@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) — sequential search
        fn get_edge_label(&self, v1: &V, v2: &V) -> (label: Option<&L>)
            requires spec_labgraphview_wf(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures
                label.is_some() == (exists |l: L::V|
                    self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l))),
                label.is_some() ==> (self@.A.contains((v1@, v2@, label.unwrap()@)) ||
                                      self@.A.contains((v2@, v1@, label.unwrap()@)));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) — sequential search
        fn has_edge(&self, v1: &V, v2: &V) -> (b: bool)
            requires spec_labgraphview_wf(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures b == (exists |l: L::V| 
                #![trigger self@.A.contains((v1@, v2@, l))] 
                #![trigger self@.A.contains((v2@, v1@, l))]
                self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l)));

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        {
            Set::new(|w: V::V| exists |l: L::V| self@.A.contains((v, w, l)) || self@.A.contains((w, v, l)))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|E|), Span O(log |E|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) — ParaPair! split edges
        fn ng(&self, v: &V) -> (ng: SetStEph<V>)
            requires
                spec_labgraphview_wf(self@),
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures
                ng.spec_setsteph_wf(),
                ng@ == self.spec_ng(v@),
                ng@ <= self@.V;

        /// Parallel edge filtering for neighbors using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- parallel split on labeled edges
        fn ng_par(&self, v: V, edges: SetStEph<LabEdge<V, L>>) -> (neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type_LabEdge::<V, L>(),
                spec_labgraphview_wf(self@),
                edges.spec_setsteph_wf(),
                edges@ <= self@.A,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_from_set(v@, edges@),
                neighbors@ <= self.spec_ng(v@)
            decreases edges@.len();
    }

    //		Section 9a. impls


    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> LabUnDirGraphMtEphTrait<V, L>
        for LabUnDirGraphMtEph<V, L>
    {
        open spec fn spec_labundirgraphmteph_wf(&self) -> bool {
            spec_labgraphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self) {
            LabUnDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_edges: SetStEph::empty(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> (g: Self) {
            LabUnDirGraphMtEph {
                vertices,
                labeled_edges,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn labeled_edges(&self) -> (e: &SetStEph<LabEdge<V, L>>) { &self.labeled_edges }

        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
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
                    edges.spec_setsteph_wf(),
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
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |e: (V::V, V::V)| edges@.contains(e) implies
                                (exists |l: L::V| #![trigger le_view.contains((e.0, e.1, l))] le_view.contains((e.0, e.1, l))) by {
                                if edges@.contains(e) {
                                    let i = choose |i: int| #![trigger le_seq[i]] 0 <= i < le_seq.len() && le_seq[i]@.0 == e.0 && le_seq[i]@.1 == e.1;
                                    lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
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
                        proof {
                        }
                        return None;
                    },
                    Some(labeled_edge) => {
                        if (feq(&labeled_edge.0, v1) && feq(&labeled_edge.1, v2)) || 
                           (feq(&labeled_edge.0, v2) && feq(&labeled_edge.1, v1)) {
                            // Veracity: NEEDED proof block
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) -- sequential scan of labeled edges
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
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        return false;
                    },
                    Some(labeled_edge) => {
                        if (feq(&labeled_edge.0, v1) && feq(&labeled_edge.1, v2)) || 
                           (feq(&labeled_edge.0, v2) && feq(&labeled_edge.1, v1)) {
                            // Veracity: NEEDED proof block
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

        /// neighbors
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- delegates to ng_par
        fn ng(&self, v: &V) -> (ng: SetStEph<V>) {
            let edges = self.labeled_edges.clone();
            self.ng_par(v.clone_plus(), edges)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- parallel split on labeled edges
        fn ng_par(&self, v: V, edges: SetStEph<LabEdge<V, L>>) -> (neighbors: SetStEph<V>)
            decreases edges@.len()
        {
            let n = edges.size();
            if n == 0 {
                // Veracity: NEEDED proof block
                proof {
                }
                SetStEph::empty()
            }
            else if n == 1 {
                let LabEdge(a, b, label) = edges.choose();
                // edges@ contains (a@, b@, label@)
                if feq(&a, &v) {
                    // Veracity: NEEDED proof block
                    proof {
                        // a@ == v@ by feq correctness
                        // spec_ng_from_set = {w | exists l. edges@.contains((v@, w, l)) || edges@.contains((w, v@, l))}
                        // Since edges@ is singleton {(a@, b@, label@)} with a@ == v@:
                        //   (v@, w, l) in edges@ iff (v@, w, l) == (a@, b@, label@) iff w == b@
                        //   (w, v@, l) in edges@ iff (w, v@, l) == (a@, b@, label@) iff w == a@ == v@ and v@ == b@
                        // So the only neighbor is b@ (and possibly v@ if v@ == b@, but that's still just b@)

                        // Veracity: NEEDED assert
                        assert forall |w: V::V| #![trigger Set::empty().insert(b@).contains(w)] Set::empty().insert(b@).contains(w) implies
                            self.spec_ng_from_set(v@, edges@).contains(w) by {
                        }
                        // Veracity: NEEDED assert
                        assert forall |w: V::V| #![trigger Set::empty().insert(b@).contains(w)] self.spec_ng_from_set(v@, edges@).contains(w) implies
                            Set::empty().insert(b@).contains(w) by {
                            let l = choose |l: L::V| edges@.contains((v@, w, l)) || edges@.contains((w, v@, l));
                            if edges@.contains((v@, w, l)) {
                                if (v@, w, l) != (a@, b@, label@) {
                                }
                            } else {
                                if (w, v@, l) != (a@, b@, label@) {
                                }
                            }
                        }
                    }
                    SetStEph::singleton(b.clone_plus())
                } else if feq(&b, &v) {
                    // Veracity: NEEDED proof block
                    proof {
                        // a@ != v@ and b@ == v@
                        // (v@, w, l) in edges@ iff (v@, w, l) == (a@, b@, label@) iff v@ == a@ (false)
                        // (w, v@, l) in edges@ iff (w, v@, l) == (a@, b@, label@) iff w == a@ and v@ == b@ (true)
                        // So the only neighbor is a@

                        // Veracity: NEEDED assert
                        assert forall |w: V::V| #![trigger Set::empty().insert(a@).contains(w)] Set::empty().insert(a@).contains(w) implies
                            self.spec_ng_from_set(v@, edges@).contains(w) by {
                        }
                        // Veracity: NEEDED assert
                        assert forall |w: V::V| #![trigger Set::empty().insert(a@).contains(w)] self.spec_ng_from_set(v@, edges@).contains(w) implies
                            Set::empty().insert(a@).contains(w) by {
                            let l = choose |l: L::V| edges@.contains((v@, w, l)) || edges@.contains((w, v@, l));
                            if edges@.contains((v@, w, l)) {
                                if (v@, w, l) != (a@, b@, label@) {
                                }
                            }
                            if edges@.contains((w, v@, l)) {
                                if (w, v@, l) != (a@, b@, label@) {
                                }
                            }
                        }
                    }
                    SetStEph::singleton(a.clone_plus())
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        // a@ != v@ and b@ != v@
                        // (v@, w, l) in edges@ requires v@ == a@ (false)
                        // (w, v@, l) in edges@ requires v@ == b@ (false)
                        // So no w satisfies the condition

                    }
                    SetStEph::empty()
                }
            }
            else {
                let mid = n / 2;
                let (left_edges, right_edges) = edges.split(mid);
                let v_left  = v.clone_plus();
                let v_right = v.clone_plus();
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_ng_from_set(v_left@, left_edges@)
                { g_left.ng_par(v_left, left_edges) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_ng_from_set(v_right@, right_edges@)
                { g_right.ng_par(v_right, right_edges) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                // Veracity: NEEDED proof block
                proof {
                    // Prove subset in one direction
                    // Veracity: NEEDED assert
                    assert forall |w: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(w)] left_neighbors@.union(right_neighbors@).contains(w) implies
                        self.spec_ng_from_set(v@, edges@).contains(w) by {
                        if left_neighbors@.contains(w) {
                            let l = choose |l: L::V| left_edges@.contains((v@, w, l)) || left_edges@.contains((w, v@, l));
                            // Veracity: NEEDED assert
                            assert(edges@.contains((v@, w, l)) || edges@.contains((w, v@, l)));
                        } else {
                            let l = choose |l: L::V| right_edges@.contains((v@, w, l)) || right_edges@.contains((w, v@, l));
                            // Veracity: NEEDED assert
                            assert(edges@.contains((v@, w, l)) || edges@.contains((w, v@, l)));
                        }
                    }

                    // Prove subset in other direction
                    // Veracity: NEEDED assert
                    assert forall |w: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(w)] self.spec_ng_from_set(v@, edges@).contains(w) implies
                        left_neighbors@.union(right_neighbors@).contains(w) by {
                        let l = choose |l: L::V| edges@.contains((v@, w, l)) || edges@.contains((w, v@, l));
                        if edges@.contains((v@, w, l)) {
                            if left_edges@.contains((v@, w, l)) {
                            } else {
                            }
                        } else {
                            if left_edges@.contains((w, v@, l)) {
                            } else {
                            }
                        }
                    }

                }

                left_neighbors.union(&right_neighbors)
            }
        }
    }

    //		Section 10a. iterators


    /// Iterator wrapper for LabUnDirGraphMtEph vertex iteration.
    #[verifier::reject_recursive_types(V)]
    pub struct LabUnDirGraphMtEphIter<'a, V: StTInMtT + Hash + Ord + 'static> {
        pub inner: SetStEphIter<'a, V>,
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> View for LabUnDirGraphMtEphIter<'a, V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, V: StTInMtT + Hash + Ord + 'static>(it: &LabUnDirGraphMtEphIter<'a, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> std::iter::Iterator for LabUnDirGraphMtEphIter<'a, V> {
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
    pub struct LabUnDirGraphMtEphGhostIterator<'a, V: StTInMtT + Hash + Ord + 'static> {
        pub pos: int,
        pub elements: Seq<V>,
        pub phantom: core::marker::PhantomData<&'a V>,
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> vstd::pervasive::ForLoopGhostIteratorNew for LabUnDirGraphMtEphIter<'a, V> {
        type GhostIter = LabUnDirGraphMtEphGhostIterator<'a, V>;

        open spec fn ghost_iter(&self) -> LabUnDirGraphMtEphGhostIterator<'a, V> {
            LabUnDirGraphMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> vstd::pervasive::ForLoopGhostIterator for LabUnDirGraphMtEphGhostIterator<'a, V> {
        type ExecIter = LabUnDirGraphMtEphIter<'a, V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &LabUnDirGraphMtEphIter<'a, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &LabUnDirGraphMtEphIter<'a, V>) -> LabUnDirGraphMtEphGhostIterator<'a, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> View for LabUnDirGraphMtEphGhostIterator<'a, V> {
        type V = Seq<V>;

        open spec fn view(&self) -> Seq<V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> std::iter::IntoIterator for &'a LabUnDirGraphMtEph<V, L> {
        type Item = &'a V;
        type IntoIter = LabUnDirGraphMtEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>(), spec_labgraphview_wf(self@)
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
                iter_invariant(&it),
        {
            LabUnDirGraphMtEphIter { inner: self.vertices().iter() }
        }
    }

    //		Section 4b. type definitions


    pub struct LabUnDirGraphMtEphInv;

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LockedLabUnDirGraphMtEph<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> {
        pub(crate) locked_graph: RwLock<LabUnDirGraphMtEph<V, L>, LabUnDirGraphMtEphInv>,
        pub(crate) ghost_locked_graph: Ghost<LabGraphView<<V as View>::V, <L as View>::V>>,
    }

    //		Section 5c. view impls


    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> View for LockedLabUnDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V {
            self.spec_ghost_locked_graph()
        }
    }

    //		Section 8c. traits


    pub trait LockedLabUnDirGraphMtEphTrait<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static>
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized
    {
        spec fn spec_labundirgraphmteph_wf(&self) -> bool;

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends spec_labgraphview_wf(self@), self@.V.contains(v)
        { Set::new(|w: V::V| exists |l: L::V|
            self@.A.contains((v, w, l)) || self@.A.contains((w, v, l))) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn empty() -> (g: Self)
            requires valid_key_type_for_lab_graph::<V, L>()
            ensures
                spec_labgraphview_wf(g@),
                g@.V == Set::<<V as View>::V>::empty(),
                g@.A == Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) -- clones under lock
        fn vertices(&self) -> (v: SetStEph<V>)
            ensures v@ == self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- clones under lock
        fn labeled_edges(&self) -> (e: SetStEph<LabEdge<V, L>>)
            ensures e@ == self@.A;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- RwLock wrapper
        fn edges(&self) -> (edges: SetStEph<Edge<V>>)
            requires valid_key_type_Edge::<V>()
            ensures forall |u: V::V, w: V::V| edges@.contains((u, w)) ==
                (exists |l: L::V| #![trigger self@.A.contains((u, w, l))] self@.A.contains((u, w, l)));

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- RwLock wrapper
        fn has_edge(&self, v1: V, v2: V) -> (b: bool)
            ensures b == (exists |l: L::V|
                #![trigger self@.A.contains((v1@, v2@, l))]
                #![trigger self@.A.contains((v2@, v1@, l))]
                self@.A.contains((v1@, v2@, l)) || self@.A.contains((v2@, v1@, l)));

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- RwLock wrapper
        fn ng(&self, v: V) -> (ng: SetStEph<V>)
            requires self@.V.contains(v@)
            ensures
                ng.spec_setsteph_wf(),
                ng@ == self.spec_ng(v@),
                ng@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_vertex(&mut self, v: V) -> (added: std::result::Result<(), ()>)
            ensures
                spec_labgraphview_wf(self@),
                self@.V == old(self)@.V.insert(v@),
                self@.A == old(self)@.A;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) -> (added: std::result::Result<(), ()>)
            ensures
                spec_labgraphview_wf(self@),
                self@.V == old(self)@.V.insert(v1@).insert(v2@),
                self@.A == old(self)@.A.insert((v1@, v2@, label@)) ||
                self@.A == old(self)@.A.insert((v2@, v1@, label@));
    }

    //		Section 9c. impls


    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> LockedLabUnDirGraphMtEph<V, L> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            spec_labgraphview_wf(self.ghost_locked_graph@)
        }

        pub closed spec fn spec_ghost_locked_graph(self) -> LabGraphView<<V as View>::V, <L as View>::V> {
            self.ghost_locked_graph@
        }
    }

    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> LockedLabUnDirGraphMtEphTrait<V, L>
        for LockedLabUnDirGraphMtEph<V, L>
    {
        open spec fn spec_labundirgraphmteph_wf(&self) -> bool {
            spec_labgraphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn empty() -> (g: Self) {
            let inner = LabUnDirGraphMtEph::empty();
            let ghost view = inner@;
            LockedLabUnDirGraphMtEph {
                locked_graph: RwLock::new(inner, Ghost(LabUnDirGraphMtEphInv)),
                ghost_locked_graph: Ghost(view),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) -- clones under lock
        fn vertices(&self) -> (v: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let v = inner.vertices().clone();
            read_handle.release_read();
            v
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- clones under lock
        fn labeled_edges(&self) -> (e: SetStEph<LabEdge<V, L>>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let e = inner.labeled_edges().clone();
            read_handle.release_read();
            e
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- RwLock wrapper
        fn edges(&self) -> (edges: SetStEph<Edge<V>>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let edges = inner.edges();
            read_handle.release_read();
            edges
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- RwLock wrapper
        fn has_edge(&self, v1: V, v2: V) -> (b: bool) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let b = inner.has_edge(&v1, &v2);
            read_handle.release_read();
            b
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- RwLock wrapper
        fn ng(&self, v: V) -> (ng: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let ng = inner.ng(&v);
            read_handle.release_read();
            ng
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_vertex(&mut self, v: V) -> (added: std::result::Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_graph.acquire_write();
            // Veracity: NEEDED proof block
            proof { assume(self.ghost_locked_graph@ == locked_val@); }
            locked_val.add_vertex(v);
            let ghost new_val = locked_val@;
            self.ghost_locked_graph = Ghost(new_val);
            write_handle.release_write(locked_val);
            Ok(())
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) -> (added: std::result::Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_graph.acquire_write();
            // Veracity: NEEDED proof block
            proof { assume(self.ghost_locked_graph@ == locked_val@); }
            locked_val.add_labeled_edge(v1, v2, label);
            let ghost new_val = locked_val@;
            self.ghost_locked_graph = Ghost(new_val);
            write_handle.release_write(locked_val);
            Ok(())
        }
    }

    //		Section 11b. top level coarse locking


    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> RwLockPredicate<LabUnDirGraphMtEph<V, L>> for LabUnDirGraphMtEphInv {
        open spec fn inv(self, v: LabUnDirGraphMtEph<V, L>) -> bool {
            spec_labgraphview_wf(v@) && valid_key_type_for_lab_graph::<V, L>()
        }
    }

    //		Section 12a. derive impls in verus!


    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> Clone for LabUnDirGraphMtEph<V, L> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            LabUnDirGraphMtEph { vertices: self.vertices.clone(), labeled_edges: self.labeled_edges.clone() }
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14a. derive impls outside verus!

    impl<V: StTInMtT + Hash + Ord, L: StTInMtT + Hash> Display for LabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph(V: {}, E: {})", self.vertices, self.labeled_edges)
        }
    }

    impl<V: StTInMtT + Hash + Ord, L: StTInMtT + Hash> Debug for LabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabUnDirGraph {{ vertices: {:?}, labeled_edges: {:?} }}",
                self.vertices, self.labeled_edges
            )
        }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> Debug for LabUnDirGraphMtEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphMtEphIter") }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> Display for LabUnDirGraphMtEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphMtEphIter") }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> Debug for LabUnDirGraphMtEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphMtEphGhostIterator") }
    }

    impl<'a, V: StTInMtT + Hash + Ord + 'static> Display for LabUnDirGraphMtEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphMtEphGhostIterator") }
    }

    //		Section 14b. derive impls outside verus!

    impl Debug for LabUnDirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphMtEphInv") }
    }

    impl Display for LabUnDirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LabUnDirGraphMtEphInv") }
    }

    //		Section 14c. derive impls outside verus!

    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> Debug for LockedLabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedLabUnDirGraphMtEph") }
    }

    impl<V: StTInMtT + Hash + Ord + 'static, L: StTInMtT + Hash + 'static> Display for LockedLabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedLabUnDirGraphMtEph") }
    }
}
