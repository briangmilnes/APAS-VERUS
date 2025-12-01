//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with integer weights - Single-threaded version.

pub mod WeightedDirGraphStEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Types::Types::{*, LabGraphView};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Types::Types::group_Triple_axioms,
    };

    pub type WeightedDirGraphStEphInt<V> = LabDirGraphStEph<V, i32>;

    pub trait WeightedDirGraphStEphIntTrait<V: StT + Hash>: 
        View<V = LabGraphView<<V as View>::V, i32>> + Sized {

        open spec fn spec_total_weight(&self) -> int {
            self@.A.fold(0int, |acc: int, t: (V::V, V::V, i32)| acc + t.2 as int)
        }

        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> (g: WeightedDirGraphStEphInt<V>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Triple::<V, V, i32>();

        fn add_weighted_edge(&mut self, from: V, to: V, weight: i32)
            requires valid_key_type_LabEdge::<V, i32>()
            ensures 
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight));

        fn get_edge_weight(&self, from: &V, to: &V) -> (result: Option<i32>)
            requires valid_key_type_LabEdge::<V, i32>()
            ensures 
                result.is_some() == (exists |w: i32| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                result.is_some() ==> self@.A.contains((from@, to@, result.unwrap()));

        fn weighted_edges(&self) -> (result: SetStEph<Triple<V, V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Triple::<V, V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| result@.contains(t) == self@.A.contains(t);

        fn out_neighbors_weighted(&self, v: &V) -> (result: SetStEph<Pair<V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Pair::<V, i32>()
            ensures 
                forall |p: (V::V, i32)| result@.contains(p) == 
                    (exists |w: i32| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        fn in_neighbors_weighted(&self, v: &V) -> (result: SetStEph<Pair<V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Pair::<V, i32>()
            ensures 
                forall |p: (V::V, i32)| result@.contains(p) == 
                    (exists |w: i32| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);

        fn total_weight(&self) -> (result: i32)
            requires valid_key_type_LabEdge::<V, i32>()
            ensures result as int == self.spec_total_weight();

        fn edges_above_weight(&self, threshold: i32) -> (result: SetStEph<Triple<V, V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Triple::<V, V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| #[trigger] result@.contains(t) == 
                    (self@.A.contains(t) && t.2 > threshold);

        fn edges_below_weight(&self, threshold: i32) -> (result: SetStEph<Triple<V, V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Triple::<V, V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| #[trigger] result@.contains(t) == 
                    (self@.A.contains(t) && t.2 < threshold);
    }

    impl<V: StT + Hash> WeightedDirGraphStEphIntTrait<V> for WeightedDirGraphStEphInt<V> {

        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> (g: WeightedDirGraphStEphInt<V>) {
            let mut edge_set: SetStEph<LabEdge<V, i32>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    valid_key_type_Triple::<V, V, i32>(),
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

        fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { 
            self.add_labeled_arc(from, to, weight); 
        }

        fn get_edge_weight(&self, from: &V, to: &V) -> (result: Option<i32>) { 
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        fn weighted_edges(&self) -> (result: SetStEph<Triple<V, V, i32>>) {
            let mut edges: SetStEph<Triple<V, V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    valid_key_type_Triple::<V, V, i32>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == la_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@ == t),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies la_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@ == t;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |t: (V::V, V::V, i32)| #[trigger] la_view.contains(t) implies edges@.contains(t) by {
                                if la_view.contains(t) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        let _ = edges.insert(Triple(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                    },
                }
            }
        }

        fn out_neighbors_weighted(&self, v: &V) -> (result: SetStEph<Pair<V, i32>>) {
            let mut neighbors: SetStEph<Pair<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost la_seq = it@.1;
            let ghost v_view = v@;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == la_view,
                    forall |p: (V::V, i32)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@.0 == v_view && la_seq[i]@.1 == p.0 && la_seq[i]@.2 == p.1),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |p: (V::V, i32)| neighbors@.contains(p) implies 
                                (exists |w: i32| #![trigger la_view.contains((v_view, p.0, w))] la_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == v_view && la_seq[i]@.1 == p.0 && la_seq[i]@.2 == p.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |p: (V::V, i32)| (exists |w: i32| #![trigger la_view.contains((v_view, p.0, w))] la_view.contains((v_view, p.0, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i32| #![trigger la_view.contains((v_view, p.0, w))] la_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: i32| #![trigger la_view.contains((v_view, p.0, w))] la_view.contains((v_view, p.0, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (v_view, p.0, w));
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

        fn in_neighbors_weighted(&self, v: &V) -> (result: SetStEph<Pair<V, i32>>) {
            let mut neighbors: SetStEph<Pair<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost la_seq = it@.1;
            let ghost v_view = v@;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == la_view,
                    forall |p: (V::V, i32)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@.1 == v_view && la_seq[i]@.0 == p.0 && la_seq[i]@.2 == p.1),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |p: (V::V, i32)| neighbors@.contains(p) implies 
                                (exists |w: i32| #![trigger la_view.contains((p.0, v_view, w))] la_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.1 == v_view && la_seq[i]@.0 == p.0 && la_seq[i]@.2 == p.1;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |p: (V::V, i32)| (exists |w: i32| #![trigger la_view.contains((p.0, v_view, w))] la_view.contains((p.0, v_view, w)) && p.1 == w) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i32| #![trigger la_view.contains((p.0, v_view, w))] la_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: i32| #![trigger la_view.contains((p.0, v_view, w))] la_view.contains((p.0, v_view, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, (p.0, v_view, w));
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

        #[verifier::external_body]
        fn total_weight(&self) -> (result: i32) { 
            self.labeled_arcs().iter().map(|edge| edge.2).sum() 
        }

        fn edges_above_weight(&self, threshold: i32) -> (result: SetStEph<Triple<V, V, i32>>) {
            let mut edges: SetStEph<Triple<V, V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    valid_key_type_Triple::<V, V, i32>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == la_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@ == t && t.2 > threshold),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies 
                                (la_view.contains(t) && t.2 > threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@ == t && t.2 > threshold;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |t: (V::V, V::V, i32)| #[trigger] la_view.contains(t) && t.2 > threshold implies 
                                edges@.contains(t) by {
                                if la_view.contains(t) && t.2 > threshold {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        if labeled_edge.2 > threshold {
                            let _ = edges.insert(Triple(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }

        fn edges_below_weight(&self, threshold: i32) -> (result: SetStEph<Triple<V, V, i32>>) {
            let mut edges: SetStEph<Triple<V, V, i32>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    valid_key_type_Triple::<V, V, i32>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == la_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@ == t && t.2 < threshold),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies 
                                (la_view.contains(t) && t.2 < threshold) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@ == t && t.2 < threshold;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |t: (V::V, V::V, i32)| #[trigger] la_view.contains(t) && t.2 < threshold implies 
                                edges@.contains(t) by {
                                if la_view.contains(t) && t.2 < threshold {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(la_seq, t);
                                }
                            }
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        if labeled_edge.2 < threshold {
                            let _ = edges.insert(Triple(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }
    }

} // verus!

    #[macro_export]
    macro_rules! WeightedDirGraphStEphIntLit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt<_> as $crate::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::WeightedDirGraphStEphIntTrait<_>>::from_weighted_edges(vertices, edges)
        }};
    }
}
