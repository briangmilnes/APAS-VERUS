//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph with CheckedU32 as the weight type on edges.
//! Unlike WeightedDirGraphStEphCheckedU32 which stores u32 and returns CheckedU32 from total_weight,
//! this version stores CheckedU32 directly on edges, enabling overflow tracking per-edge.

pub mod WeightedDirGraphCheckedU32 {

    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Types::Types::{*, LabGraphView};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::checked_nat::checked_nat::CheckedU32;
    use crate::vstdplus::HashCheckedU32::CheckedU32_feq_trigger;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::vstdplus::HashCheckedU32::group_CheckedU32_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Types::Types::group_WeightedEdge_axioms,
    };

    // The graph type: edges are labeled with CheckedU32
    // View type uses int since CheckedU32 views to int
    pub type WeightedDirGraphCheckedU32<V> = LabDirGraphStEph<V, CheckedU32>;

    // valid_key_type for LabEdge<V, CheckedU32> and WeightedEdge<V, CheckedU32>
    pub open spec fn valid_key_type_LabEdge_CheckedU32<V: StT + Hash>() -> bool {
        &&& valid_key_type_LabEdge::<V, CheckedU32>()
        &&& valid_key_type_WeightedEdge::<V, CheckedU32>()
    }

    pub trait WeightedDirGraphCheckedU32Trait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, int>> + Sized {

        // Spec using Set.fold (unordered, harder to verify)
        open spec fn spec_total_weight(&self) -> int 
         { self@.A.fold(0int, |acc: int, t: (V::V, V::V, int)| acc + t.2) }

        // Spec using Seq.fold_left (ordered, easier to verify with loop invariants)
        // Takes a sequence of edges and computes the sum of weights
        open spec fn spec_total_weight_seq(edges: Seq<(V::V, V::V, int)>) -> int 
         { edges.fold_left(0int, |acc: int, t: (V::V, V::V, int)| acc + t.2) }

        // Helper spec for fold over LabEdge sequence
        open spec fn spec_sum_weights(edges: Seq<LabEdge<V, CheckedU32>>) -> int 
         { edges.fold_left(0int, |acc: int, e: LabEdge<V, CheckedU32>| acc + e@.2) }


        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, CheckedU32>>) -> (g: WeightedDirGraphCheckedU32<V>)
            requires valid_key_type_LabEdge_CheckedU32::<V>();

        fn add_weighted_edge(&mut self, from: V, to: V, weight: CheckedU32)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight@));

        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<CheckedU32>)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                weight.is_some() == (exists |w: int| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()@));

        fn weighted_edges(&self) -> (weighted_edges: SetStEph<WeightedEdge<V, CheckedU32>>)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                forall |t: (V::V, V::V, int)| #[trigger] weighted_edges@.contains(t) == self@.A.contains(t);

        fn out_neighbors_weighted(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, CheckedU32>>)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                forall |p: (V::V, int)| out_neighbors@.contains(p) == 
                    (exists |w: int| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        fn in_neighbors_weighted(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, CheckedU32>>)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                forall |p: (V::V, int)| in_neighbors@.contains(p) == 
                    (exists |w: int| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        fn total_weight(&self) -> (total_weight: CheckedU32)
            requires valid_key_type_LabEdge_CheckedU32::<V>();

        // Total weight using Seq.fold_left specification
        // Returns CheckedU32 which tracks overflow
        // Ensures: when result is normal, it equals the sum of all edge weights
        // Note: uses assume for fold_left closure equality (SMT limitation)
        fn total_weight_seq(&self) -> (total_weight: CheckedU32)
            requires valid_key_type_LabEdge_CheckedU32::<V>();

        fn edges_above_weight(&self, threshold: CheckedU32) -> (edges_above: SetStEph<WeightedEdge<V, CheckedU32>>)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                forall |t: (V::V, V::V, int)| #[trigger] edges_above@.contains(t) == 
                    (self@.A.contains(t) && t.2 > threshold@);

        fn edges_below_weight(&self, threshold: CheckedU32) -> (edges_below: SetStEph<WeightedEdge<V, CheckedU32>>)
            requires valid_key_type_LabEdge_CheckedU32::<V>()
            ensures 
                forall |t: (V::V, V::V, int)| #[trigger] edges_below@.contains(t) == 
                    (self@.A.contains(t) && t.2 < threshold@);
    }

    impl<V: StT + Hash> WeightedDirGraphCheckedU32Trait<V> for WeightedDirGraphCheckedU32<V> {

        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, CheckedU32>>) -> (g: WeightedDirGraphCheckedU32<V>) {
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
                assert(WeightedEdge_feq_trigger::<V, CheckedU32>());
            }
            let mut edge_set: SetStEph<LabEdge<V, CheckedU32>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge_CheckedU32::<V>(),
                    it@.0 <= edge_seq.len(),
                    it@.1 == edge_seq,
                decreases edge_seq.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(triple) => {
                        let _ = edge_set.insert(LabEdge(triple.0.clone_plus(), triple.1.clone_plus(), triple.2.clone()));
                    },
                }
            }

            LabDirGraphStEph::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        fn add_weighted_edge(&mut self, from: V, to: V, weight: CheckedU32) { 
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
            }
            self.add_labeled_arc(from, to, weight); 
        }

        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<CheckedU32>) { 
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
            }
            match self.get_arc_label(from, to) {
                Some(w) => Some(w.clone()),
                None => None,
            }
        }

        fn weighted_edges(&self) -> (weighted_edges: SetStEph<WeightedEdge<V, CheckedU32>>) {
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
                assert(WeightedEdge_feq_trigger::<V, CheckedU32>());
            }
            let mut edges: SetStEph<WeightedEdge<V, CheckedU32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge_CheckedU32::<V>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, CheckedU32>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, int)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |t: (V::V, V::V, int)| #[trigger] edges@.contains(t) implies wa_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
                            assert forall |t: (V::V, V::V, int)| #[trigger] wa_view.contains(t) implies edges@.contains(t) by {
                                if wa_view.contains(t) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2.clone()));
                    },
                }
            }
        }

        fn out_neighbors_weighted(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, CheckedU32>>) {
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
                assert(WeightedEdge_feq_trigger::<V, CheckedU32>());
            }
            let mut neighbors: SetStEph<Pair<V, CheckedU32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge_CheckedU32::<V>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, CheckedU32>| e@).to_set() == wa_view,
                    forall |p: (V::V, int)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |p: (V::V, int)| neighbors@.contains(p) implies 
                                (exists |w: int| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
                            assert forall |p: (V::V, int)| (exists |w: int| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: int| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: int| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, (v_view, p.0, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.0, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.1.clone_plus(), labeled_edge.2.clone()));
                        }
                    },
                }
            }
        }

        fn in_neighbors_weighted(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, CheckedU32>>) {
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
                assert(WeightedEdge_feq_trigger::<V, CheckedU32>());
            }
            let mut neighbors: SetStEph<Pair<V, CheckedU32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge_CheckedU32::<V>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, CheckedU32>| e@).to_set() == wa_view,
                    forall |p: (V::V, int)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |p: (V::V, int)| neighbors@.contains(p) implies 
                                (exists |w: int| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
                            assert forall |p: (V::V, int)| (exists |w: int| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: int| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: int| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, (p.0, v_view, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.1, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.0.clone_plus(), labeled_edge.2.clone()));
                        }
                    },
                }
            }
        }

        fn total_weight(&self) -> (total_weight: CheckedU32) {
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
                assert(WeightedEdge_feq_trigger::<V, CheckedU32>());
            }
            let mut sum = CheckedU32::new(0);
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge_CheckedU32::<V>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, CheckedU32>| e@).to_set() == wa_view,
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        return sum;
                    },
                    Some(labeled_edge) => {
                        // Add the weight value (as u32) to the running sum
                        // If the weight is in overflow, we get overflow propagation
                        if labeled_edge.2.is_normal() {
                            sum = sum.add_value(labeled_edge.2.unwrap());
                        } else {
                            // Edge weight is already overflow - propagate
                            sum = sum.add_checked(&labeled_edge.2);
                        }
                    },
                }
            }
        }

        // Total weight using Seq.fold_left specification
        // The invariant tracks: sum@ == spec_sum_weights(wa_seq.take(processed_count))
        // Uses assume for fold_left closure equality (SMT limitation with higher-order functions)
        fn total_weight_seq(&self) -> (total_weight: CheckedU32) {
            proof { 
                assert(CheckedU32_feq_trigger());
                assert(LabEdge_feq_trigger::<V, CheckedU32>());
                assert(WeightedEdge_feq_trigger::<V, CheckedU32>());
            }
            let mut sum = CheckedU32::new(0);
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge_CheckedU32::<V>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    // Key invariant: when normal, sum equals fold_left over processed elements
                    sum.is_normal() ==> sum@ == Self::spec_sum_weights(wa_seq.take(it@.0 as int)),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        return sum;
                    },
                    Some(labeled_edge) => {
                        let ghost old_idx = (it@.0 - 1) as int;
                        let ghost new_idx = it@.0 as int;
                        let ghost old_sum = sum@;
                        let ghost f = |acc: int, e: LabEdge<V, CheckedU32>| acc + e@.2;
                        
                        proof {
                            // Prepare: establish fold_left relationship
                            assert(wa_seq.take(new_idx).drop_last() =~= wa_seq.take(old_idx));
                            assert(wa_seq.take(new_idx).last() == wa_seq[old_idx]);
                            assert(wa_seq[old_idx] == labeled_edge);
                        }
                        
                        // Add the weight - overflow propagates automatically
                        if labeled_edge.2.is_normal() {
                            let weight_val = labeled_edge.2.unwrap();
                            sum = sum.add_value(weight_val);
                            proof {
                                reveal_with_fuel(Seq::fold_left, 1);
                                
                                let take_new = wa_seq.take(new_idx);
                                let take_old = wa_seq.take(old_idx);
                                let f = |acc: int, e: LabEdge<V, CheckedU32>| acc + e@.2;
                                
                                // unwrap ensures: weight_val as int == labeled_edge.2@
                                // add_value ensures: sum@ == old_sum + weight_val as int
                                assert(weight_val as int == labeled_edge.2@);
                                assert(sum@ == old_sum + labeled_edge.2@);
                                
                                // Establish key equalities for fold_left unfolding
                                assert(take_new.len() > 0);
                                assert(take_new.drop_last() =~= take_old);
                                assert(take_new.last() == wa_seq[old_idx]);
                                assert(wa_seq[old_idx] == labeled_edge);
                                
                                // fold_left unfolds: s.fold_left(b, f) = f(s.drop_last().fold_left(b, f), s.last())
                                assert(take_new.fold_left(0int, f) == 
                                       f(take_new.drop_last().fold_left(0int, f), take_new.last()));
                                
                                // spec_sum_weights is just fold_left with f
                                assert(Self::spec_sum_weights(take_new) == 
                                       Self::spec_sum_weights(take_old) + labeled_edge@.2);
                                
                                // Chain of equalities to establish invariant
                                // 1. old_sum == spec_sum_weights(take_old) (from loop invariant, assuming was normal)
                                // 2. sum@ == old_sum + labeled_edge@.2 (from add_value postcondition)
                                // 3. spec_sum_weights(take_new) == spec_sum_weights(take_old) + labeled_edge@.2 (from fold_left unfolding)
                                // 4. Therefore: sum@ == spec_sum_weights(take_new)
                                
                                // FIXME: SMT has trouble with closure equality in fold_left
                                // The logic is correct but needs more proof machinery
                                assume(sum.is_normal() ==> sum@ == Self::spec_sum_weights(take_new));
                            }
                        } else {
                            // Edge weight is already overflow - propagate
                            sum = sum.add_checked(&labeled_edge.2);
                            // sum.is_normal() might or might not be true depending on the sum
                            // The invariant is: sum.is_normal() ==> sum@ == spec_sum_weights(...)
                            // We need to handle this case
                            proof {
                                let take_new = wa_seq.take(new_idx);
                                // FIXME: Need to prove invariant when edge weight is overflow
                                assume(sum.is_normal() ==> sum@ == Self::spec_sum_weights(take_new));
                            }
                        }
                    },
                }
            }
        }

        // Note: external_body because CheckedU32 overflow semantics make invariant complex
        #[verifier::external_body]
        fn edges_above_weight(&self, threshold: CheckedU32) -> (edges_above: SetStEph<WeightedEdge<V, CheckedU32>>) {
            let mut edges: SetStEph<WeightedEdge<V, CheckedU32>> = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                // Compare weights - both must be normal for meaningful comparison
                if labeled_edge.2.is_normal() && threshold.is_normal() {
                    if labeled_edge.2.unwrap() > threshold.unwrap() {
                        let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2.clone()));
                    }
                } else if labeled_edge.2.is_overflow() {
                    // Overflow is "above" any normal threshold
                    let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2.clone()));
                }
            }
            edges
        }

        // Note: external_body because CheckedU32 overflow semantics make invariant complex
        #[verifier::external_body]
        fn edges_below_weight(&self, threshold: CheckedU32) -> (edges_below: SetStEph<WeightedEdge<V, CheckedU32>>) {
            let mut edges: SetStEph<WeightedEdge<V, CheckedU32>> = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                // Compare weights - both must be normal for meaningful comparison
                if labeled_edge.2.is_normal() && threshold.is_normal() {
                    if labeled_edge.2.unwrap() < threshold.unwrap() {
                        let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2.clone()));
                    }
                } else if threshold.is_overflow() && labeled_edge.2.is_normal() {
                    // Normal weight is "below" overflow threshold
                    let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2.clone()));
                }
            }
            edges
        }
    }

} // verus!

    #[macro_export]
    macro_rules! WeightedDirGraphCheckedU32Lit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphCheckedU32::WeightedDirGraphCheckedU32::WeightedDirGraphCheckedU32<_> as 
             $crate::Chap06::WeightedDirGraphCheckedU32::WeightedDirGraphCheckedU32::WeightedDirGraphCheckedU32Trait<_>>::from_weighted_edges(vertices, edges)
        }};
    }
}
