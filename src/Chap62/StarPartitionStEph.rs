//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Sequential Ephemeral Implementation
//!
//! Implements sequential star partition using greedy vertex selection.
//! A star partition divides a graph into blocks where each block is a
//! vertex-induced subgraph with respect to a star graph.

pub mod StarPartitionStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;

    verus! {

    // 3. broadcast use

    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarPartitionStEph;

    // 8. traits

    pub trait StarPartitionStEphTrait {
        /// Well-formedness for star partition algorithm input.
        open spec fn spec_starpartitionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential star partition using greedy selection.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_starpartitionsteph_wf(graph);
    }

    pub type T<V> = UnDirGraphStEph<V>;

    /// Sequential Star Partition using greedy selection.
    ///
    /// - APAS: Work Θ(n + m), Span Θ(n + m)
    /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — agrees with APAS.
    pub fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures
            result.0.spec_setsteph_wf(),
            // Every graph vertex appears as a key in partition_map.
            forall |v_view: V::V| graph@.V.contains(v_view) ==>
                #[trigger] result.1@.contains_key(v_view),
            // Every partition_map value is a center.
            forall |v_view: V::V| result.1@.contains_key(v_view) ==>
                result.0@.contains(#[trigger] result.1@[v_view]@),
    {
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();
        let mut centers: SetStEph<V> = SetLit![];
        let mut processed: SetStEph<V> = SetLit![];

        let vert_vec = graph.V.to_seq();
        let edge_vec = graph.E.to_seq();
        let nv = vert_vec.len();
        let ne = edge_vec.len();

        let mut vi: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while vi < nv
            invariant
                valid_key_type_Edge::<V>(),
                centers.spec_setsteph_wf(),
                processed.spec_setsteph_wf(),
                vi <= nv,
                nv == vert_vec@.len(),
                ne == edge_vec@.len(),
                // All vertices at indices 0..vi have been processed.
                forall |k: int| 0 <= k < vi as int ==>
                    processed@.contains(#[trigger] vert_vec@[k]@),
                // All processed vertices are in partition_map.
                forall |v_view: V::V| #[trigger] processed@.contains(v_view) ==>
                    partition_map@.contains_key(v_view),
                // All partition_map values are in centers.
                forall |v_view: V::V| partition_map@.contains_key(v_view) ==>
                    centers@.contains(#[trigger] partition_map@[v_view]@),
            decreases nv - vi,
        {
            let vertex = &vert_vec[vi];
            let ghost curr_view = (*vertex)@;
            proof { assert(curr_view == vert_vec@[vi as int]@); }

            if !processed.mem(vertex) {
                // Capture clones as named variables so we can reason about their views.
                let ck = vertex.clone_view();
                let pk = vertex.clone_view();
                let pv = vertex.clone_view();
                let rk = vertex.clone_view();
                let _ = centers.insert(ck);
                partition_map.insert(pk, pv);
                let _ = processed.insert(rk);
                proof {
                    // clone_view ensures: each clone's view equals vertex@.
                    assert(ck@ == (*vertex)@);
                    assert(pk@ == (*vertex)@);
                    assert(pv@ == (*vertex)@);
                    // centers gained vertex@.
                    assert(centers@.contains((*vertex)@));
                    // processed gained vertex@.
                    assert(processed@.contains((*vertex)@));
                    // partition_map[vertex@] = pv, and pv@ = vertex@.
                    assert(partition_map@[(*vertex)@] == pv);
                    assert(partition_map@[(*vertex)@]@ == (*vertex)@);
                }

                let mut ei: usize = 0;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while ei < ne
                    invariant
                        valid_key_type_Edge::<V>(),
                        processed.spec_setsteph_wf(),
                        centers.spec_setsteph_wf(),
                        ei <= ne,
                        ne == edge_vec@.len(),
                        vi < nv,
                        nv == vert_vec@.len(),
                        // vertex remains in processed throughout the inner loop.
                        processed@.contains(curr_view),
                        // vertex is a center throughout the inner loop.
                        centers@.contains(curr_view),
                        // All previous outer-loop vertices remain processed.
                        forall |k: int| 0 <= k < vi as int ==>
                            processed@.contains(#[trigger] vert_vec@[k]@),
                        // All processed vertices are in partition_map.
                        forall |v_view: V::V| #[trigger] processed@.contains(v_view) ==>
                            partition_map@.contains_key(v_view),
                        // All partition_map values are in centers.
                        forall |v_view: V::V| partition_map@.contains_key(v_view) ==>
                            centers@.contains(#[trigger] partition_map@[v_view]@),
                    decreases ne - ei,
                {
                    let edge = &edge_vec[ei];
                    let Edge(a, b) = edge;
                    if *a == *vertex {
                        if !processed.mem(b) {
                            let bk = b.clone_view();
                            let vc = vertex.clone_view();
                            partition_map.insert(bk, vc);
                            let _ = processed.insert(b.clone_view());
                            proof {
                                assert(bk@ == (*b)@);
                                assert(vc@ == (*vertex)@);
                                assert(partition_map@[(*b)@] == vc);
                                assert(partition_map@[(*b)@]@ == (*vertex)@);
                            }
                        }
                    } else if *b == *vertex {
                        if !processed.mem(a) {
                            let ak = a.clone_view();
                            let vc = vertex.clone_view();
                            partition_map.insert(ak, vc);
                            let _ = processed.insert(a.clone_view());
                            proof {
                                assert(ak@ == (*a)@);
                                assert(vc@ == (*vertex)@);
                                assert(partition_map@[(*a)@] == vc);
                                assert(partition_map@[(*a)@]@ == (*vertex)@);
                            }
                        }
                    }
                    ei = ei + 1;
                }
            }

            proof {
                // curr_view was set to (*vertex)@ and proved equal to vert_vec@[vi]@.
                assert(processed@.contains(curr_view));
                assert(processed@.contains(vert_vec@[vi as int]@));
            }
            vi = vi + 1;
        }

        // Prove: every graph vertex is in partition_map.
        proof {
            assert forall |v_view: V::V| #[trigger] graph@.V.contains(v_view) implies
                partition_map@.contains_key(v_view) by {
                // to_seq postcondition: graph.V@.contains(v_view) iff map.contains(v_view).
                let vmap = vert_vec@.map(|_i: int, t: V| t@);
                assert(vmap.contains(v_view));
                // Find the index k where vert_vec@[k]@ = v_view.
                let k = choose|k: int| 0 <= k < vmap.len() && vmap[k] == v_view;
                assert(vmap[k] == vert_vec@[k]@);
                assert(vert_vec@[k]@ == v_view);
                // From outer loop invariant (vi == nv at exit): k < vi.
                assert(processed@.contains(vert_vec@[k]@));
                assert(processed@.contains(v_view));
                // From invariant: processed ⊆ partition_map.keys().
                assert(partition_map@.contains_key(v_view));
            };
        }

        (centers, partition_map)
    }

    } // verus!
}
