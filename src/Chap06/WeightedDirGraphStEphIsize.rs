//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with isize weights - Single-threaded version.
//! Uses CheckedIsize for overflow-safe weight summation.

pub mod WeightedDirGraphStEphIsize {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Types::Types::{*, LabGraphView};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::checked_int::checked_int::CheckedIsize;
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

    pub type WeightedDirGraphStEphIsize<V> = LabDirGraphStEph<V, isize>;

    pub trait WeightedDirGraphStEphIsizeTrait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, isize>> + Sized {

        open spec fn spec_total_weight(&self) -> int 
         { self@.A.fold(0int, |acc: int, t: (V::V, V::V, isize)| acc + t.2 as int) }

        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, isize>>) -> (g: WeightedDirGraphStEphIsize<V>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures g@.V.finite(), g@.A.finite();

        fn add_weighed_edge(&mut self, from: V, to: V, weight: isize)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight));

        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<isize>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                weight.is_some() == (exists |w: isize| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()));

        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, isize>>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                forall |t: (V::V, V::V, isize)| #[trigger] weighed_edges@.contains(t) == self@.A.contains(t);

        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, isize>>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                forall |p: (V::V, isize)| out_neighbors@.contains(p) == 
                    (exists |w: isize| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, isize>>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                forall |p: (V::V, isize)| in_neighbors@.contains(p) == 
                    (exists |w: isize| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        fn total_weight(&self) -> (total_weight: CheckedIsize)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures total_weight@ == self.spec_total_weight();

        fn edges_above_weight(&self, threshold: isize) -> (edges_above: SetStEph<WeightedEdge<V, isize>>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                forall |t: (V::V, V::V, isize)| #[trigger] edges_above@.contains(t) == 
                    (self@.A.contains(t) && t.2 > threshold);

        fn edges_below_weight(&self, threshold: isize) -> (edges_below: SetStEph<WeightedEdge<V, isize>>)
            requires valid_key_type_WeightedEdge::<V, isize>()
            ensures 
                forall |t: (V::V, V::V, isize)| #[trigger] edges_below@.contains(t) == 
                    (self@.A.contains(t) && t.2 < threshold);
    }

    impl<V: StT + Hash> WeightedDirGraphStEphIsizeTrait<V> for WeightedDirGraphStEphIsize<V> {

        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, isize>>) -> (g: WeightedDirGraphStEphIsize<V>) {
            let mut edge_set: SetStEph<LabEdge<V, isize>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= edge_seq.len(),
                    it@.1 == edge_seq,
                decreases edge_seq.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(triple) => {
                        let _ = edge_set.insert(LabEdge(triple.0.clone_plus(), triple.1.clone_plus(), triple.2));
                    },
                }
            }

            LabDirGraphStEph::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        fn add_weighed_edge(&mut self, from: V, to: V, weight: isize) { 
            self.add_labeled_arc(from, to, weight); 
        }

        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<isize>) { 
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, isize>>) {
            let mut edges: SetStEph<WeightedEdge<V, isize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, isize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, isize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
// Veracity: UNNEEDED assert                             assert forall |t: (V::V, V::V, isize)| #[trigger] edges@.contains(t) implies wa_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: UNNEEDED assert                             assert forall |t: (V::V, V::V, isize)| #[trigger] wa_view.contains(t) implies edges@.contains(t) by {
                                if wa_view.contains(t) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, t);
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

        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, isize>>) {
            let mut neighbors: SetStEph<Pair<V, isize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, isize>| e@).to_set() == wa_view,
                    forall |p: (V::V, isize)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
// Veracity: UNNEEDED assert                             assert forall |p: (V::V, isize)| neighbors@.contains(p) implies 
                                (exists |w: isize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: UNNEEDED assert                             assert forall |p: (V::V, isize)| (exists |w: isize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: isize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: isize| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, (v_view, p.0, w));
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

        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, isize>>) {
            let mut neighbors: SetStEph<Pair<V, isize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, isize>| e@).to_set() == wa_view,
                    forall |p: (V::V, isize)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
// Veracity: UNNEEDED assert                             assert forall |p: (V::V, isize)| neighbors@.contains(p) implies 
                                (exists |w: isize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: UNNEEDED assert                             assert forall |p: (V::V, isize)| (exists |w: isize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: isize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: isize| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, (p.0, v_view, w));
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

        fn total_weight(&self) -> (total_weight: CheckedIsize) { 
            let mut sum = CheckedIsize::new(0);
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.no_duplicates(),
                    wa_seq.map(|i: int, e: LabEdge<V, isize>| e@).to_set() == wa_view,
                    sum@ == wa_seq.take(it@.0 as int).fold_left(0int, |acc: int, e: LabEdge<V, isize>| acc + e@.2 as int),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_isize::<LabEdge<V, isize>, V::V, V::V>(wa_seq);
                            lemma_signed_weighted_seq_fold_equals_set_fold_isize(wa_seq.map(|_i: int, e: LabEdge<V, isize>| e@));
                        }
                        return sum;
                    },
                    Some(labeled_edge) => {
// Veracity: UNNEEDED assert                         proof { assert(wa_seq.take(it@.0 as int).drop_last() =~= wa_seq.take((it@.0 - 1) as int)); }
                        sum = sum.add_value(labeled_edge.2);
                    },
                }
            }
        }

        fn edges_above_weight(&self, threshold: isize) -> (edges_above: SetStEph<WeightedEdge<V, isize>>) {
            let mut edges: SetStEph<WeightedEdge<V, isize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, isize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, isize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 > threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
// Veracity: UNNEEDED assert                             assert forall |t: (V::V, V::V, isize)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 > threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 > threshold;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: UNNEEDED assert                             assert forall |t: (V::V, V::V, isize)| #[trigger] wa_view.contains(t) && t.2 > threshold implies 
                                edges@.contains(t) by {
                                if wa_view.contains(t) && t.2 > threshold {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, t);
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

        fn edges_below_weight(&self, threshold: isize) -> (edges_below: SetStEph<WeightedEdge<V, isize>>) {
            let mut edges: SetStEph<WeightedEdge<V, isize>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, isize>(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, isize>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, isize)| edges@.contains(t) == 
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t && t.2 < threshold),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
// Veracity: UNNEEDED assert                             assert forall |t: (V::V, V::V, isize)| #[trigger] edges@.contains(t) implies 
                                (wa_view.contains(t) && t.2 < threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@ == t && t.2 < threshold;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: UNNEEDED assert                             assert forall |t: (V::V, V::V, isize)| #[trigger] wa_view.contains(t) && t.2 < threshold implies 
                                edges@.contains(t) by {
                                if wa_view.contains(t) && t.2 < threshold {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(wa_seq, t);
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
    macro_rules! WeightedDirGraphStEphIsizeLit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphStEphIsize::WeightedDirGraphStEphIsize::WeightedDirGraphStEphIsize<_> as $crate::Chap06::WeightedDirGraphStEphIsize::WeightedDirGraphStEphIsize::WeightedDirGraphStEphIsizeTrait<_>>::from_weighed_edges(vertices, edges)
        }};
    }
}

