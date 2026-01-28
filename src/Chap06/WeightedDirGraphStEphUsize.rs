//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with usize weights - Single-threaded version.
//! Uses CheckedUsize for overflow-safe weight summation.

pub mod WeightedDirGraphStEphUsize {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::checked_nat::checked_nat::CheckedUsize;
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
    };


    pub type WeightedDirGraphStEphUsize<V> = LabDirGraphStEph<V, usize>;

    pub trait WeightedDirGraphStEphUsizeTrait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, usize>> + Sized {

        open spec fn spec_total_weight(&self) -> nat 
         { self@.A.fold(0nat, |acc: nat, t: (V::V, V::V, usize)| acc + t.2 as nat) }

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, usize>>) -> (g: WeightedDirGraphStEphUsize<V>)
            requires 
                valid_key_type_WeightedEdge::<V, usize>(),
                forall |u: V::V, w: V::V, weight: usize| 
                    #[trigger] edges@.contains((u, w, weight)) ==> 
                        vertices@.contains(u) && vertices@.contains(w),
            ensures wf_lab_graph_view(g@), g@.V =~= vertices@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: usize)
            requires valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<usize>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                weight.is_some() == (exists |w: usize| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, usize>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                forall |t: (V::V, V::V, usize)| #[trigger] weighed_edges@.contains(t) == self@.A.contains(t);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, usize>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                forall |p: (V::V, usize)| out_neighbors@.contains(p) == 
                    (exists |w: usize| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, usize>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                forall |p: (V::V, usize)| in_neighbors@.contains(p) == 
                    (exists |w: usize| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn total_weight(&self) -> (total_weight: CheckedUsize)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures total_weight@ == self.spec_total_weight() as int;

        fn edges_above_weight(&self, threshold: usize) -> (edges_above: SetStEph<WeightedEdge<V, usize>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                forall |t: (V::V, V::V, usize)| #[trigger] edges_above@.contains(t) == 
                    (self@.A.contains(t) && t.2 > threshold);

        fn edges_below_weight(&self, threshold: usize) -> (edges_below: SetStEph<WeightedEdge<V, usize>>)
            requires wf_lab_graph_view(self@), valid_key_type_WeightedEdge::<V, usize>()
            ensures 
                forall |t: (V::V, V::V, usize)| #[trigger] edges_below@.contains(t) == 
                    (self@.A.contains(t) && t.2 < threshold);
    }

    impl<V: StT + Hash> WeightedDirGraphStEphUsizeTrait<V> for WeightedDirGraphStEphUsize<V> {

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, usize>>) -> (g: WeightedDirGraphStEphUsize<V>) {
            let mut edge_set: SetStEph<LabEdge<V, usize>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
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
        fn add_weighed_edge(&mut self, from: V, to: V, weight: usize) { 
            self.add_labeled_arc(from, to, weight); 
        }

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<usize>) { 
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, usize>>) {
            let mut edges: SetStEph<WeightedEdge<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, usize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |t: (V::V, V::V, usize)| #[trigger] edges@.contains(t) implies wa_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
assert forall |t: (V::V, V::V, usize)| #[trigger] wa_view.contains(t) implies edges@.contains(t) by {
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
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |p: (V::V, usize)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |p: (V::V, usize)| neighbors@.contains(p) implies 
                                (exists |w: usize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
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

        /// APAS: Work Θ(|A|), Span Θ(1)
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
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |p: (V::V, usize)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |p: (V::V, usize)| neighbors@.contains(p) implies 
                                (exists |w: usize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
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

        /// APAS: Work Θ(|A|), Span Θ(1)
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
            {
                match it.next() {
                    None => {
                        proof {
                            lemma_seq_fold_left_plus_is_weighted_seq_sum_usize::<LabEdge<V, usize>, V::V, V::V>(wa_seq);
                            lemma_fold_left_int_equals_nat_as_int_usize::<LabEdge<V, usize>, V::V, V::V>(wa_seq);
                            lemma_weighted_seq_fold_equals_set_fold_usize(wa_seq.map(|_i: int, e: LabEdge<V, usize>| e@));
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

        fn edges_above_weight(&self, threshold: usize) -> (edges_above: SetStEph<WeightedEdge<V, usize>>) {
            let mut edges: SetStEph<WeightedEdge<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, usize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 > threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |t: (V::V, V::V, usize)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 > threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 > threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
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

        fn edges_below_weight(&self, threshold: usize) -> (edges_below: SetStEph<WeightedEdge<V, usize>>) {
            let mut edges: SetStEph<WeightedEdge<V, usize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, usize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, usize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, usize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 < threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
assert forall |t: (V::V, V::V, usize)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 < threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 < threshold;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
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
