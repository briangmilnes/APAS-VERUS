//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighed Undirected Graph (ephemeral) with integer weights - Single-threaded version.

pub mod WeighedUnDirGraphStEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::{*, LabGraphView};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_WeighedEdge_axioms,
    };

    pub type WeighedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;

    pub trait WeighedUnDirGraphStEphIntTrait<V: StT + Hash + Ord>: 
        View<V = LabGraphView<<V as View>::V, i32>> + Sized + LabUnDirGraphStEphTrait<V, i32> {

        open spec fn spec_total_weight(&self) -> int {
            self@.A.fold(0int, |acc: int, t: (V::V, V::V, i32)| acc + t.2 as int)
        }

        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> (g: WeighedUnDirGraphStEphInt<V>)
            requires valid_key_type_WeighedEdge::<V, i32>();

        fn add_weighed_edge(&mut self, v1: V, v2: V, weight: i32)
            requires valid_key_type_WeighedEdge::<V, i32>();

        fn get_edge_weight(&self, v1: &V, v2: &V) -> (weight: Option<i32>)
            requires valid_key_type_WeighedEdge::<V, i32>()
            ensures 
                weight.is_some() == (exists |w: i32| self@.A.contains((v1@, v2@, w)) || self@.A.contains((v2@, v1@, w))),
                weight.is_some() ==> (self@.A.contains((v1@, v2@, weight.unwrap())) || self@.A.contains((v2@, v1@, weight.unwrap())));

        fn weighed_edges(&self) -> (weighed_edges: SetStEph<Triple<V, V, i32>>)
            requires valid_key_type_WeighedEdge::<V, i32>()
            ensures 
                forall |t: (V::V, V::V, i32)| weighed_edges@.contains(t) == self@.A.contains(t);

        fn neighbors_weighed(&self, v: &V) -> (neighbors: SetStEph<Pair<V, i32>>)
            requires valid_key_type_WeighedEdge::<V, i32>()
            ensures 
                forall |p: (V::V, i32)| neighbors@.contains(p) == 
                    ((exists |w: i32| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w) ||
                     (exists |w: i32| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w));

        fn total_weight(&self) -> (total_weight: i32)
            requires valid_key_type_WeighedEdge::<V, i32>()
            ensures total_weight as int == self.spec_total_weight();

        fn vertex_degree(&self, v: &V) -> (degree: usize)
            requires valid_key_type_WeighedEdge::<V, i32>()
            ensures degree == self.spec_neighbors(v@).len();

        fn is_connected(&self) -> (connected: bool)
            requires valid_key_type_WeighedEdge::<V, i32>();
    }

    impl<V: StT + Hash + Ord> WeighedUnDirGraphStEphIntTrait<V> for WeighedUnDirGraphStEphInt<V> {

        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> (g: WeighedUnDirGraphStEphInt<V>) {
            let mut edge_set: SetStEph<LabEdge<V, i32>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeighedEdge::<V, i32>(),
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

        fn add_weighed_edge(&mut self, v1: V, v2: V, weight: i32) { 
            self.add_labeled_edge(v1, v2, weight); 
        }

        fn get_edge_weight(&self, v1: &V, v2: &V) -> (weight: Option<i32>) { 
            match self.get_edge_label(v1, v2) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        fn weighed_edges(&self) -> (weighed_edges: SetStEph<Triple<V, V, i32>>) {
            let mut edges: SetStEph<Triple<V, V, i32>> = SetStEph::empty();
            let mut it = self.labeled_edges().iter();
            let ghost we_seq = it@.1;
            let ghost we_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeighedEdge::<V, i32>(),
                    it@.0 <= we_seq.len(),
                    it@.1 == we_seq,
                    we_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == we_view,
                    forall |t: (V::V, V::V, i32)| edges@.contains(t) == 
                        (exists |i: int| #![trigger we_seq[i]] 0 <= i < it@.0 && we_seq[i]@ == t),
                decreases we_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |t: (V::V, V::V, i32)| #[trigger] edges@.contains(t) implies we_view.contains(t) by {
                                if edges@.contains(t) {
                                    let i = choose |i: int| #![trigger we_seq[i]] 0 <= i < we_seq.len() && we_seq[i]@ == t;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(we_seq, i);
                                }
                            }
                            assert forall |t: (V::V, V::V, i32)| #[trigger] we_view.contains(t) implies edges@.contains(t) by {
                                if we_view.contains(t) {
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(we_seq, t);
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

        fn neighbors_weighed(&self, v: &V) -> (neighbors: SetStEph<Pair<V, i32>>) {
            let mut neighbors: SetStEph<Pair<V, i32>> = SetStEph::empty();
            let mut it = self.labeled_edges().iter();
            let ghost we_seq = it@.1;
            let ghost v_view = v@;
            let ghost we_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeighedEdge::<V, i32>(),
                    it@.0 <= we_seq.len(),
                    it@.1 == we_seq,
                    we_seq.map(|i: int, e: LabEdge<V, i32>| e@).to_set() == we_view,
                    forall |p: (V::V, i32)| neighbors@.contains(p) == 
                        (exists |i: int| #![trigger we_seq[i]] 0 <= i < it@.0 && 
                            ((we_seq[i]@.0 == v_view && we_seq[i]@.1 == p.0 && we_seq[i]@.2 == p.1) ||
                             (we_seq[i]@.1 == v_view && we_seq[i]@.0 == p.0 && we_seq[i]@.2 == p.1))),
                decreases we_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |p: (V::V, i32)| neighbors@.contains(p) implies 
                                ((exists |w: i32| #![trigger we_view.contains((v_view, p.0, w))] we_view.contains((v_view, p.0, w)) && p.1 == w) ||
                                 (exists |w: i32| #![trigger we_view.contains((p.0, v_view, w))] we_view.contains((p.0, v_view, w)) && p.1 == w)) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger we_seq[i]] 0 <= i < we_seq.len() && 
                                        ((we_seq[i]@.0 == v_view && we_seq[i]@.1 == p.0 && we_seq[i]@.2 == p.1) ||
                                         (we_seq[i]@.1 == v_view && we_seq[i]@.0 == p.0 && we_seq[i]@.2 == p.1));
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(we_seq, i);
                                }
                            }
                            assert forall |p: (V::V, i32)| 
                                ((exists |w: i32| #![trigger we_view.contains((v_view, p.0, w))] we_view.contains((v_view, p.0, w)) && p.1 == w) ||
                                 (exists |w: i32| #![trigger we_view.contains((p.0, v_view, w))] we_view.contains((p.0, v_view, w)) && p.1 == w)) implies 
                                neighbors@.contains(p) by {
                                if exists |w: i32| #![trigger we_view.contains((v_view, p.0, w))] we_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: i32| #![trigger we_view.contains((v_view, p.0, w))] we_view.contains((v_view, p.0, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(we_seq, (v_view, p.0, w));
                                } else if exists |w: i32| #![trigger we_view.contains((p.0, v_view, w))] we_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: i32| #![trigger we_view.contains((p.0, v_view, w))] we_view.contains((p.0, v_view, w)) && p.1 == w;
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(we_seq, (p.0, v_view, w));
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
        fn total_weight(&self) -> (total_weight: i32) { 
            self.labeled_edges().iter().map(|edge| edge.2).sum() 
        }

        fn vertex_degree(&self, v: &V) -> (degree: usize) { 
            self.neighbors(v).size() 
        }

        #[verifier::external_body]
        fn is_connected(&self) -> (connected: bool) {
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
    macro_rules! WeighedUnDirGraphStEphIntLit {
        () => {{
            $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeighedUnDirGraphStEphInt::WeighedUnDirGraphStEphInt::WeighedUnDirGraphStEphInt<_> as $crate::Chap06::WeighedUnDirGraphStEphInt::WeighedUnDirGraphStEphInt::WeighedUnDirGraphStEphIntTrait<_>>::from_weighed_edges(vertices, edges)
        }};
    }
}
