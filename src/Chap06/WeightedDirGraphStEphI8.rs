//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 6 Weighted Directed Graph (ephemeral) with i8 weights - Single-threaded version.
//! Uses CheckedI8 for overflow-safe weight summation.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 13. macros

//		Section 1. module

pub mod WeightedDirGraphStEphI8 {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::checked_int::checked_int::*;
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


    pub type WeightedDirGraphStEphI8<V> = LabDirGraphStEph<V, i8>;

    //		Section 8. traits


    pub trait WeightedDirGraphStEphI8Trait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, i8>> + Sized {

        open spec fn spec_total_weight(&self) -> int 
         { self@.A.fold(0int, |acc: int, t: (V::V, V::V, i8)| acc + t.2 as int) }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) -- sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, i8>>) -> (g: WeightedDirGraphStEphI8<V>)
            requires
                valid_key_type_WeightedEdge::<V, i8>(),
                edges@.finite(),
                forall |u: V::V, w: V::V, weight: i8|
                    #[trigger] edges@.contains((u, w, weight)) ==>
                        vertices@.contains(u) && vertices@.contains(w),
            ensures spec_labgraphview_wf(g@), g@.V =~= vertices@;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: i8)
            requires valid_key_type_WeightedEdge::<V, i8>()
            ensures
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential search
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<i8>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures
                weight.is_some() == (exists |w: i8| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential iteration
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, i8>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures
                forall |t: (V::V, V::V, i8)| #[trigger] weighed_edges@.contains(t) == self@.A.contains(t);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, i8>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures
                forall |p: (V::V, i8)| out_neighbors@.contains(p) ==
                    (exists |w: i8| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, i8>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures
                forall |p: (V::V, i8)| in_neighbors@.contains(p) ==
                    (exists |w: i8| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential fold
        fn total_weight(&self) -> (total_weight: CheckedI8)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures total_weight@ == self.spec_total_weight();

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_above_weight(&self, threshold: i8) -> (edges_above: SetStEph<WeightedEdge<V, i8>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures
                forall |t: (V::V, V::V, i8)| #[trigger] edges_above@.contains(t) ==
                    (self@.A.contains(t) && t.2 > threshold);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_below_weight(&self, threshold: i8) -> (edges_below: SetStEph<WeightedEdge<V, i8>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, i8>()
            ensures
                forall |t: (V::V, V::V, i8)| #[trigger] edges_below@.contains(t) ==
                    (self@.A.contains(t) && t.2 < threshold);
    }

    //		Section 9. impls


    impl<V: StT + Hash> WeightedDirGraphStEphI8Trait<V> for WeightedDirGraphStEphI8<V> {

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, i8>>) -> (g: WeightedDirGraphStEphI8<V>) {
            let mut edge_set: SetStEph<LabEdge<V, i8>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    edge_set.spec_setsteph_wf(),
                    it@.0 <= edge_seq.len(),
                    it@.1 == edge_seq,
                    edge_seq.map(|i: int, e: WeightedEdge<V, i8>| e@).to_set() == edges@,
                    forall |u: V::V, w: V::V, weight: i8| 
                        #[trigger] edge_set@.contains((u, w, weight)) ==> 
                            vertices@.contains(u) && vertices@.contains(w),
                decreases edge_seq.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(triple) => {
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
        fn add_weighed_edge(&mut self, from: V, to: V, weight: i8) { 
            self.add_labeled_arc(from, to, weight); 
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<i8>) { 
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) — sequential
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, i8>>) {
            let mut edges: SetStEph<WeightedEdge<V, i8>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i8>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, i8)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
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
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, i8>>) {
            let mut neighbors: SetStEph<Pair<V, i8>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    neighbors.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i8>| e@).to_set() == wa_view,
                    forall |p: (V::V, i8)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
assert forall |p: (V::V, i8)| neighbors@.contains(p) implies 
                                (exists |w: i8| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
assert forall |p: (V::V, i8)| (exists |w: i8| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i8| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: i8| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w;
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
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, i8>>) {
            let mut neighbors: SetStEph<Pair<V, i8>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    neighbors.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i8>| e@).to_set() == wa_view,
                    forall |p: (V::V, i8)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
assert forall |p: (V::V, i8)| neighbors@.contains(p) implies 
                                (exists |w: i8| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
assert forall |p: (V::V, i8)| (exists |w: i8| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i8| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: i8| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w;
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
        fn total_weight(&self) -> (total_weight: CheckedI8) { 
            let mut sum = CheckedI8::new(0);
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.no_duplicates(),
                    wa_seq.map(|i: int, e: LabEdge<V, i8>| e@).to_set() == wa_view,
                    sum@ == wa_seq.take(it@.0 as int).fold_left(0int, |acc: int, e: LabEdge<V, i8>| acc + e@.2 as int),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_reveal_view_injective::<LabEdge<V, i8>>();
                            lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i8::<LabEdge<V, i8>, V::V, V::V>(wa_seq);
                            lemma_signed_weighted_seq_fold_equals_set_fold_i8(wa_seq.map(|_i: int, e: LabEdge<V, i8>| e@));
                        }
                        return sum;
                    },
                    Some(labeled_edge) => {
// Veracity: NEEDED proof block
// Veracity: NEEDED assert
proof { assert(wa_seq.take(it@.0 as int).drop_last() =~= wa_seq.take((it@.0 - 1) as int)); }
                        sum = sum.add_value(labeled_edge.2);
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- sequential filter
        fn edges_above_weight(&self, threshold: i8) -> (edges_above: SetStEph<WeightedEdge<V, i8>>) {
            let mut edges: SetStEph<WeightedEdge<V, i8>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i8>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, i8)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 > threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, i8)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 > threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 > threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, i8)| #[trigger] wa_view.contains(t) && t.2 > threshold implies 
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
        fn edges_below_weight(&self, threshold: i8) -> (edges_below: SetStEph<WeightedEdge<V, i8>>) {
            let mut edges: SetStEph<WeightedEdge<V, i8>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i8>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i8>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, i8)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 < threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, i8)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 < threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 < threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
assert forall |t: (V::V, V::V, i8)| #[trigger] wa_view.contains(t) && t.2 < threshold implies 
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
    macro_rules! WeightedDirGraphStEphI8Lit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphStEphI8::WeightedDirGraphStEphI8::WeightedDirGraphStEphI8<_> as $crate::Chap06::WeightedDirGraphStEphI8::WeightedDirGraphStEphI8::WeightedDirGraphStEphI8Trait<_>>::from_weighed_edges(vertices, edges)
        }};
    }
}
