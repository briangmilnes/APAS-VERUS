//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with i32 weights - Single-threaded version.
//! Uses CheckedI32 for overflow-safe weight summation.

pub mod WeightedDirGraphStEphI32 {

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

verus! {

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


    pub type WeightedDirGraphStEphI32<V> = LabDirGraphStEph<V, i32>;

    pub trait WeightedDirGraphStEphI32Trait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, i32>> + Sized {

        open spec fn spec_total_weight(&self) -> int 
         { self@.A.fold(0int, |acc: int, t: (V::V, V::V, i32)| acc + t.2 as int) }

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, i32>>) -> (g: WeightedDirGraphStEphI32<V>)
            requires 
                valid_key_type_WeightedEdge::<V, i32>(),
                forall |u: V::V, w: V::V, weight: i32| 
                    #[trigger] edges@.contains((u, w, weight)) ==> 
                        vertices@.contains(u) && vertices@.contains(w),
            ensures wf_lab_graph_view(g@), g@.V =~= vertices@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: i32)
            requires valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<i32>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                weight.is_some() == (exists |w: i32| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, i32>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| #[trigger] weighed_edges@.contains(t) == self@.A.contains(t);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, i32>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                forall |p: (V::V, i32)| out_neighbors@.contains(p) == 
                    (exists |w: i32| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, i32>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                forall |p: (V::V, i32)| in_neighbors@.contains(p) == 
                    (exists |w: i32| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn total_weight(&self) -> (total_weight: CheckedI32)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures total_weight@ == self.spec_total_weight();

        fn edges_above_weight(&self, threshold: i32) -> (edges_above: SetStEph<WeightedEdge<V, i32>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| #[trigger] edges_above@.contains(t) == 
                    (self@.A.contains(t) && t.2 > threshold);

        fn edges_below_weight(&self, threshold: i32) -> (edges_below: SetStEph<WeightedEdge<V, i32>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| #[trigger] edges_below@.contains(t) == 
                    (self@.A.contains(t) && t.2 < threshold);
    }

    impl<V: StT + Hash> WeightedDirGraphStEphI32Trait<V> for WeightedDirGraphStEphI32<V> {

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, i32>>) -> (g: WeightedDirGraphStEphI32<V>) {
            let mut edge_set: SetStEph<LabEdge<V, i32>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= edge_seq.len(),
                    it@.1 == edge_seq,
                    edge_seq.map(|i: int, e: WeightedEdge<V, i32>| e@).to_set() == edges@,
                    forall |u: V::V, w: V::V, weight: i32| 
                        #[trigger] edge_set@.contains((u, w, weight)) ==> 
                            vertices@.contains(u) && vertices@.contains(w),
                decreases edge_seq.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(triple) => {
                        proof {
                            lemma_seq_index_in_map_to_set(edge_seq, it@.0 - 1);
                        }
                        let _ = edge_set.insert(LabEdge(triple.0.clone_plus(), triple.1.clone_plus(), triple.2));
                    },
                }
            }

            LabDirGraphStEph::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: i32) { 
            self.add_labeled_arc(from, to, weight); 
        }

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<i32>) { 
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, i32>>) {
            let mut edges: SetStEph<WeightedEdge<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies wa_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
assert forall |t: (V::V, V::V, i32)| #[trigger] wa_view.contains(t) implies edges@.contains(t) by {
                                if wa_view.contains(t) {
                                    lemma_map_to_set_contains_index(wa_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                    },
                }
            }
        }

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, i32>>) {
            let mut neighbors: SetStEph<Pair<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == wa_view,
                    forall |p: (V::V, i32)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |p: (V::V, i32)| neighbors@.contains(p) implies 
                                (exists |w: i32| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
assert forall |p: (V::V, i32)| (exists |w: i32| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i32| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: i32| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w;
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

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, i32>>) {
            let mut neighbors: SetStEph<Pair<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == wa_view,
                    forall |p: (V::V, i32)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |p: (V::V, i32)| neighbors@.contains(p) implies 
                                (exists |w: i32| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
assert forall |p: (V::V, i32)| (exists |w: i32| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i32| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: i32| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w;
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

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn total_weight(&self) -> (total_weight: CheckedI32) { 
            let mut sum = CheckedI32::new(0);
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.no_duplicates(),
                    wa_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == wa_view,
                    sum@ == wa_seq.take(it@.0 as int).fold_left(0int, |acc: int, e: LabEdge<V, i32>| acc + e@.2 as int),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            lemma_signed_seq_fold_left_plus_is_weighted_seq_sum::<LabEdge<V, i32>, V::V, V::V>(wa_seq);
                            lemma_signed_weighted_seq_fold_equals_set_fold(wa_seq.map(|_i: int, e: LabEdge<V, i32>| e@));
                        }
                        return sum;
                    },
                    Some(labeled_edge) => {
proof { assert(wa_seq.take(it@.0 as int).drop_last() =~= wa_seq.take((it@.0 - 1) as int)); }
                        sum = sum.add_value(labeled_edge.2);
                    },
                }
            }
        }

        fn edges_above_weight(&self, threshold: i32) -> (edges_above: SetStEph<WeightedEdge<V, i32>>) {
            let mut edges: SetStEph<WeightedEdge<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 > threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 > threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 > threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
assert forall |t: (V::V, V::V, i32)| #[trigger] wa_view.contains(t) && t.2 > threshold implies 
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

        fn edges_below_weight(&self, threshold: i32) -> (edges_below: SetStEph<WeightedEdge<V, i32>>) {
            let mut edges: SetStEph<WeightedEdge<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, i32>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 < threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 < threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 < threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
assert forall |t: (V::V, V::V, i32)| #[trigger] wa_view.contains(t) && t.2 < threshold implies 
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

    #[macro_export]
    macro_rules! WeightedDirGraphStEphI32Lit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphStEphI32::WeightedDirGraphStEphI32::WeightedDirGraphStEphI32<_> as $crate::Chap06::WeightedDirGraphStEphI32::WeightedDirGraphStEphI32::WeightedDirGraphStEphI32Trait<_>>::from_weighed_edges(vertices, edges)
        }};
    }
}
