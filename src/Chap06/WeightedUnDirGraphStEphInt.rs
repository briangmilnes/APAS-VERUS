//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Undirected Graph (ephemeral) with integer weights - Single-threaded version.

pub mod WeightedUnDirGraphStEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;
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

    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;

    pub trait WeightedUnDirGraphStEphIntTrait<V: StT + Hash + Ord>: 
        View<V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V, i32)>)> + Sized + LabUnDirGraphStEphTrait<V, i32> {

        open spec fn spec_total_weight(&self) -> int {
            self@.1.fold(0int, |acc: int, t: (V::V, V::V, i32)| acc + t.2 as int)
        }

        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> (g: WeightedUnDirGraphStEphInt<V>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Triple::<V, V, i32>();

        fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32)
            requires valid_key_type_LabEdge::<V, i32>();

        fn get_edge_weight(&self, v1: &V, v2: &V) -> (result: Option<i32>)
            requires valid_key_type_LabEdge::<V, i32>()
            ensures 
                result.is_some() == (exists |w: i32| #![auto] self@.1.contains((v1@, v2@, w)) || self@.1.contains((v2@, v1@, w))),
                result.is_some() ==> (self@.1.contains((v1@, v2@, result.unwrap())) || self@.1.contains((v2@, v1@, result.unwrap())));

        fn weighted_edges(&self) -> (result: SetStEph<Triple<V, V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Triple::<V, V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| result@.contains(t) == self@.1.contains(t);

        fn neighbors_weighted(&self, v: &V) -> (result: SetStEph<Pair<V, i32>>)
            requires 
                valid_key_type_LabEdge::<V, i32>(),
                valid_key_type_Pair::<V, i32>()
            ensures 
                forall |p: (V::V, i32)| result@.contains(p) == 
                    ((exists |w: i32| #![auto] self@.1.contains((v@, p.0, w)) && p.1 == w) ||
                     (exists |w: i32| #![auto] self@.1.contains((p.0, v@, w)) && p.1 == w));

        fn total_weight(&self) -> (result: i32)
            requires valid_key_type_LabEdge::<V, i32>()
            ensures result as int == self.spec_total_weight();

        fn vertex_degree(&self, v: &V) -> (result: usize)
            requires valid_key_type_LabEdge::<V, i32>()
            ensures result == self.spec_neighbors(v@).len();

        fn is_connected(&self) -> (result: bool)
            requires valid_key_type_LabEdge::<V, i32>();
    }

    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphIntTrait<V> for WeightedUnDirGraphStEphInt<V> {

        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> (g: WeightedUnDirGraphStEphInt<V>) {
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

            LabUnDirGraphStEph::from_vertices_and_labeled_edges(vertices, edge_set)
        }

        fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { 
            self.add_labeled_edge(v1, v2, weight); 
        }

        fn get_edge_weight(&self, v1: &V, v2: &V) -> (result: Option<i32>) { 
            match self.get_edge_label(v1, v2) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        fn weighted_edges(&self) -> (result: SetStEph<Triple<V, V, i32>>) {
            let mut edges: SetStEph<Triple<V, V, i32>> = SetStEph::empty();
            let mut it = self.labeled_edges().iter();
            let ghost le_seq = it@.1;
            let ghost le_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    valid_key_type_Triple::<V, V, i32>(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == le_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![auto] 0 <= i < it@.0 && le_seq[i]@ == t),
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies le_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![auto] 0 <= i < le_seq.len() && le_seq[i]@ == t;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
                            assert forall |t: (V::V, V::V, i32)| #[trigger] le_view.contains(t) implies edges@.contains(t) by {
                                if le_view.contains(t) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(le_seq, t);
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

        fn neighbors_weighted(&self, v: &V) -> (result: SetStEph<Pair<V, i32>>) {
            let mut neighbors: SetStEph<Pair<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_edges().iter();
            let ghost le_seq = it@.1;
            let ghost v_view = v@;
            let ghost le_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, i32>(),
                    valid_key_type_Pair::<V, i32>(),
                    it@.0 <= le_seq.len(),
                    it@.1 == le_seq,
                    le_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == le_view,
                    forall |p: (V::V, i32)| neighbors@.contains(p) == 
                        (exists |i: int| #![auto] 0 <= i < it@.0 && 
                            ((le_seq[i]@.0 == v_view && le_seq[i]@.1 == p.0 && le_seq[i]@.2 == p.1) ||
                             (le_seq[i]@.1 == v_view && le_seq[i]@.0 == p.0 && le_seq[i]@.2 == p.1))),
                decreases le_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |p: (V::V, i32)| neighbors@.contains(p) implies 
                                ((exists |w: i32| #![auto] le_view.contains((v_view, p.0, w)) && p.1 == w) ||
                                 (exists |w: i32| #![auto] le_view.contains((p.0, v_view, w)) && p.1 == w)) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![auto] 0 <= i < le_seq.len() && 
                                        ((le_seq[i]@.0 == v_view && le_seq[i]@.1 == p.0 && le_seq[i]@.2 == p.1) ||
                                         (le_seq[i]@.1 == v_view && le_seq[i]@.0 == p.0 && le_seq[i]@.2 == p.1));
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(le_seq, i);
                                }
                            }
                            assert forall |p: (V::V, i32)| 
                                ((exists |w: i32| #![auto] le_view.contains((v_view, p.0, w)) && p.1 == w) ||
                                 (exists |w: i32| #![auto] le_view.contains((p.0, v_view, w)) && p.1 == w)) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i32| #![auto] le_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: i32| #![auto] le_view.contains((v_view, p.0, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(le_seq, (v_view, p.0, w));
                                } else if exists |w: i32| #![auto] le_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: i32| #![auto] le_view.contains((p.0, v_view, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(le_seq, (p.0, v_view, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.0, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.1.clone_plus(), labeled_edge.2));
                        } else if feq(&labeled_edge.1, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.0.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }

        #[verifier::external_body]
        fn total_weight(&self) -> (result: i32) { 
            self.labeled_edges().iter().map(|edge| edge.2).sum() 
        }

        fn vertex_degree(&self, v: &V) -> (result: usize) { 
            self.neighbors(v).size() 
        }

        #[verifier::external_body]
        fn is_connected(&self) -> (result: bool) {
            if self.vertices().size() == 0 {
                return true;
            }

            let mut visited: SetStEph<V> = SetStEph::empty();
            let mut stack = Vec::new();

            if let Some(start) = self.vertices().iter().next() {
                stack.push(start.clone_plus());

                while let Some(current) = stack.pop() {
                    if !visited.mem(&current) {
                        let _ = visited.insert(current.clone_plus());
                        let neighbors = self.neighbors(&current);
                        let mut neighbor_it = neighbors.iter();
                        loop {
                            match neighbor_it.next() {
                                None => break,
                                Some(neighbor) => {
                                    if !visited.mem(neighbor) {
                                        stack.push(neighbor.clone_plus());
                                    }
                                },
                            }
                        }
                    }
                }
            }

            visited.size() == self.vertices().size()
        }
    }

} // verus!

    #[macro_export]
    macro_rules! WeightedUnDirGraphStEphIntLit {
        () => {{
            $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt<_> as $crate::Chap06::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphIntTrait<_>>::from_weighted_edges(vertices, edges)
        }};
    }
}
