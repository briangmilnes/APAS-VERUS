// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 6 Weighted Directed Graph (ephemeral) with usize weights - Single-threaded version.
//! Uses CheckedUsize for overflow-safe weight summation.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 13. macros

//		Section 1. module

pub mod WeightedDirGraphStEphUsize {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::checked_nat::checked_nat::*;
    use crate::vstdplus::seq_set::*;

verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Types::Types::group_WeightedEdge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
    };

    //		Section 4. type definitions


    pub type WeightedDirGraphStEphUsize<V> = LabDirGraphStEph<V, usize>;

    //		Section 8. traits


    pub trait WeightedDirGraphStEphUsizeTrait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, usize>> + Sized {

        open spec fn spec_total_weight(&self) -> nat 
         { self@.A.fold(0nat, |acc: nat, t: (V::V, V::V, usize)| acc + t.2 as nat) }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) -- sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, usize>>) -> (g: WeightedDirGraphStEphUsize<V>)
            requires
                valid_key_type_WeightedEdge::<V, usize>(),
                edges@.finite(),
                forall |u: V::V, w: V::V, weight: usize|
                    #[trigger] edges@.contains((u, w, weight)) ==>
                        vertices@.contains(u) && vertices@.contains(w),
            ensures spec_labgraphview_wf(g@), g@.V =~= vertices@;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: usize)
            requires valid_key_type_WeightedEdge::<V, usize>()
            ensures
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential search
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<usize>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures
                weight.is_some() == (exists |w: usize| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential iteration
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, usize>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures
                forall |t: (V::V, V::V, usize)| #[trigger] weighed_edges@.contains(t) == self@.A.contains(t);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, usize>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures
                forall |p: (V::V, usize)| out_neighbors@.contains(p) ==
                    (exists |w: usize| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, usize>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures
                forall |p: (V::V, usize)| in_neighbors@.contains(p) ==
                    (exists |w: usize| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential fold
        fn total_weight(&self) -> (total_weight: CheckedUsize)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures total_weight@ == self.spec_total_weight() as int;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_above_weight(&self, threshold: usize) -> (edges_above: SetStEph<WeightedEdge<V, usize>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures
                forall |t: (V::V, V::V, usize)| #[trigger] edges_above@.contains(t) ==
                    (self@.A.contains(t) && t.2 > threshold);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_below_weight(&self, threshold: usize) -> (edges_below: SetStEph<WeightedEdge<V, usize>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures
                forall |t: (V::V, V::V, usize)| #[trigger] edges_below@.contains(t) ==
                    (self@.A.contains(t) && t.2 < threshold);
    }

    //		Section 9. impls


    impl<V: StT + Hash> WeightedDirGraphStEphUsizeTrait<V> for WeightedDirGraphStEphUsize<V> {

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, usize>>) -> (g: WeightedDirGraphStEphUsize<V>) {
            let mut edge_set: SetStEph<LabEdge<V, usize>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    edge_set.spec_setsteph_wf(),
                    it@.0 <= edge_seq.len(),
                    it@.1 == edge_seq,
                    edge_seq.map(|i: int, e: WeightedEdge<V, usize>| e@).to_set() == edges@,
                    forall |u: V::V, w: V::V, weight: usize| 
                        #[trigger] edge_set@.contains((u, w, weight)) ==> 
                            vertices@.contains(u) && vertices@.contains(w),
                decreases edge_seq.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(triple) => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_seq_index_in_map_to_set(edge_seq, it@.0 - 1);
                        }
                        let _ = edge_set.insert(LabEdge(triple.0.clone_plus(), triple.1.clone_plus(), triple.2));
                    },
                }
            }

            LabDirGraphStEph::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: usize) { 
            self.add_labeled_arc(from, to, weight); 
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<usize>) { 
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, usize>>) {
            let mut edges: SetStEph<WeightedEdge<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, usize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                    },
                }
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, usize>>) {
            let mut neighbors: SetStEph<Pair<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    neighbors.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |p: (V::V, usize)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    // Veracity: NEEDED proof block
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |p: (V::V, usize)| neighbors@.contains(p) implies 
                                (exists |w: usize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |p: (V::V, usize)| (exists |w: usize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: usize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: usize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w;
                                    lemma_map_to_set_contains_index(wa_seq, (v_view, p.0, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.0, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.1.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, usize>>) {
            let mut neighbors: SetStEph<Pair<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    neighbors.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |p: (V::V, usize)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                // Veracity: NEEDED proof block
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |p: (V::V, usize)| neighbors@.contains(p) implies 
                                (exists |w: usize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |p: (V::V, usize)| (exists |w: usize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: usize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: usize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w;
                                    lemma_map_to_set_contains_index(wa_seq, (p.0, v_view, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.1, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.0.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn total_weight(&self) -> (total_weight: CheckedUsize) { 
            let mut sum = CheckedUsize::new(0);
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.no_duplicates(),
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    sum@ == wa_seq.take(it@.0 as int).fold_left(0int, |acc: int, e: LabEdge<V, usize>| acc + e@.2 as nat),
                decreases wa_seq.len() - it@.0,
            // Veracity: NEEDED proof block
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_reveal_view_injective::<LabEdge<V, usize>>();
                            lemma_seq_fold_left_plus_is_weighted_seq_sum_usize::<LabEdge<V, usize>, V::V, V::V>(wa_seq);
                            lemma_fold_left_int_equals_nat_as_int_usize::<LabEdge<V, usize>, V::V, V::V>(wa_seq);
                            lemma_weighted_seq_fold_equals_set_fold_usize(wa_seq.map(|_i: int, e: LabEdge<V, usize>| e@));
                        }
                        return sum;
                    // Veracity: NEEDED proof block
                    },
                    Some(labeled_edge) => {
// Veracity: NEEDED proof block
// Veracity: NEEDED assert
// Veracity: NEEDED assert
proof { assert(wa_seq.take(it@.0 as int).drop_last() =~= wa_seq.take((it@.0 - 1) as int)); }
                        sum = sum.add_value(labeled_edge.2);
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_above_weight(&self, threshold: usize) -> (edges_above: SetStEph<WeightedEdge<V, usize>>) {
            let mut edges: SetStEph<WeightedEdge<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, usize)| edges@.contains(t) == 
                        // Veracity: NEEDED proof block
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 > threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, usize)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 > threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 > threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, usize)| #[trigger] wa_view.contains(t) && t.2 > threshold implies 
                                edges@.contains(t) by {
                                if wa_view.contains(t) && t.2 > threshold {
                                    lemma_map_to_set_contains_index(wa_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        if labeled_edge.2 > threshold {
                            let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_below_weight(&self, threshold: usize) -> (edges_below: SetStEph<WeightedEdge<V, usize>>) {
            let mut edges: SetStEph<WeightedEdge<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    // Veracity: NEEDED proof block
                    forall |t: (V::V, V::V, usize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 < threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, usize)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 < threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 < threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, usize)| #[trigger] wa_view.contains(t) && t.2 < threshold implies 
                                edges@.contains(t) by {
                                if wa_view.contains(t) && t.2 < threshold {
                                    lemma_map_to_set_contains_index(wa_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        if labeled_edge.2 < threshold {
                            let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }
    }

} // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! WeightedDirGraphStEphUsizeLit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize<_> as $crate::Chap06::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsizeTrait<_>>::from_weighed_edges(vertices, edges)
        }};
    }
}
