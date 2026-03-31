//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Multi-threaded Ephemeral Implementation
//!
//! Implements Algorithm 62.3: Parallel Star Partition using randomized coin flips.
//! Uses Seq.inject for efficient parallel updates.

pub mod StarPartitionMtEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use std::vec::Vec;
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::rand::rand::{seeded_rng, random_bool_seeded};
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_view_injective, obeys_feq_full, lemma_reveal_view_injective};
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    verus! {

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarPartitionMtEph;

    // 8. traits

    pub trait StarPartitionMtEphTrait {
        /// Well-formedness for parallel star partition algorithm input.
        open spec fn spec_starpartitionmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel star partition using randomized coin flips.
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        /// - Alg Analysis: APAS (Ch62 Thm 62.1): Work O(n + m), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn parallel_star_partition<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_starpartitionmteph_wf(graph), valid_key_type_Edge::<V>();
    }

    pub type T<V> = UnDirGraphMtEph<V>;

    // 6. spec fns

    /// Partition map validity: every graph vertex is mapped and every value is a center.
    pub open spec fn spec_valid_partition_map<V: View>(
        graph_vertices: Set<V::V>,
        centers: Set<V::V>,
        partition_map: Map<V::V, V>,
    ) -> bool {
        &&& forall |v_view: V::V|
                #[trigger] graph_vertices.contains(v_view) ==>
                    partition_map.contains_key(v_view)
        &&& forall |v_view: V::V|
                #[trigger] partition_map.contains_key(v_view) ==>
                    centers.contains(partition_map[v_view]@)
    }

    /// Algorithm 62.3: Parallel Star Partition.
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.1): Work O(n + m), Span O(lg n)
    /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
    pub fn parallel_star_partition<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (partition: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
        ensures
            partition.0.spec_setsteph_wf(),
            spec_valid_partition_map::<V>(graph@.V, partition.0@, partition.1@),
    {
        proof {
            // Establish spec_setsteph_wf for graph.V and graph.E so we can call to_seq().
            // valid_key_type_Edge::<V>() ==> valid_key_type::<V>() and valid_key_type::<Edge<V>>().
            assert(obeys_key_model::<V>());
            assert(obeys_feq_full::<V>());
            assert(valid_key_type::<V>());
            assert(graph@.V.finite());
            assert(graph.V.spec_setsteph_wf());
            assert(obeys_key_model::<Edge<V>>());
            assert(obeys_feq_full::<Edge<V>>());
            assert(valid_key_type::<Edge<V>>());
            assert(graph@.A.finite());
            assert(graph.E.spec_setsteph_wf());
        }
        let vertices_vec = graph.V.to_seq();
        let nv = vertices_vec.len();

        // Loop 1: build vertex-to-index map.
        // Domain invariant: every key in vertex_to_index came from vertices_vec[0..i].
        let mut vertex_to_index = HashMapWithViewPlus::<V, usize>::new();
        let mut i: usize = 0;
        while i < nv
            invariant
                valid_key_type_Edge::<V>(),
                i <= nv,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                // vertex_to_index maps vertices_vec[0..i] to their indices.
                forall|j: int| 0 <= j < i as int ==>
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j]@) &&
                    vertex_to_index@[vertices_vec@[j]@] as usize == j,
                // Domain is exactly {vertices_vec@[j]@ | 0 <= j < i}.
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j: int| 0 <= j < i as int && #[trigger] vertices_vec@[j]@ == v_view,
            decreases nv - i,
        {
            let ghost iv = vertices_vec@[i as int]@;
            vertex_to_index.insert(vertices_vec[i].clone_view(), i as usize);
            proof {
                // After inserting vertices_vec[i] -> i, prove invariants for range [0..i+1].
                assert forall|j: int| 0 <= j < i as int + 1 implies
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j]@) &&
                    vertex_to_index@[vertices_vec@[j]@] as usize == j by {
                    if j < i as int {
                        let ghost jv = vertices_vec@[j]@;
                        if jv != iv {
                            assert(vertex_to_index@.contains_key(jv));
                            assert(vertex_to_index@[jv] as usize == j);
                        } else {
                            // jv == iv (view-equal). By obeys_feq_view_injective, value-equal.
                            lemma_reveal_view_injective::<V>();
                            assert(vertices_vec@[j as int] == vertices_vec@[i as int]);
                            assert(vertices_vec@.no_duplicates());
                            assert(false);
                        }
                    }
                };
                // Domain update: any new key must be vertices_vec@[i]@ or an old key.
                assert forall|v_view: V::V| vertex_to_index@.contains_key(v_view) implies
                    exists|j: int| 0 <= j < i as int + 1 && #[trigger] vertices_vec@[j]@ == v_view by {
                    if v_view != iv {
                        // Old key; old invariant provides witness.
                        let j2 = choose|j: int| 0 <= j < i as int && #[trigger] vertices_vec@[j]@ == v_view;
                        assert(0 <= j2 < i as int + 1);
                    }
                    // v_view == iv: witness j = i.
                };
            }
            i = i + 1;
        }

        // Ghost facts after loop 1:
        // (a) vertex_to_index covers all graph@.V.
        // (b) vertex_to_index domain is exactly {vertices_vec@[j]@ | j < nv}.
        proof {
            assert forall|v_view: V::V| #[trigger] graph@.V.contains(v_view) implies
                vertex_to_index@.contains_key(v_view) by {
                assert(vertices_vec@.map(|_i: int, t: V| t@).contains(v_view));
                let k = vertices_vec@.map(|_i: int, t: V| t@).index_of(v_view);
                assert(0 <= k < nv as int);
                assert(vertices_vec@[k]@ == v_view);
                assert(vertex_to_index@.contains_key(vertices_vec@[k]@));
            };
        }

        // Loop 2: flip coins for each vertex.
        let mut rng = seeded_rng(seed);
        let mut coin_flips = HashMapWithViewPlus::<V, bool>::new();
        let mut j: usize = 0;
        while j < nv
            invariant
                valid_key_type_Edge::<V>(),
                j <= nv,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                forall|jj: int| 0 <= jj < j as int ==>
                    #[trigger] coin_flips@.contains_key(vertices_vec@[jj]@),
            decreases nv - j,
        {
            coin_flips.insert(vertices_vec[j].clone_view(), random_bool_seeded(&mut rng));
            proof {
                assert forall|jj: int| 0 <= jj < j as int + 1 implies
                    #[trigger] coin_flips@.contains_key(vertices_vec@[jj]@) by {
                    if jj < j as int {
                        let ghost jjv = vertices_vec@[jj]@;
                        let ghost jv2 = vertices_vec@[j as int]@;
                        if jjv != jv2 {
                            assert(coin_flips@.contains_key(jjv));
                        } else {
                            lemma_reveal_view_injective::<V>();
                            assert(vertices_vec@[jj as int] == vertices_vec@[j as int]);
                            assert(vertices_vec@.no_duplicates());
                            assert(false);
                        }
                    }
                };
            }
            j = j + 1;
        }

        // Loop 3: build tail-heads edges.
        let edge_vec = graph.E.to_seq();
        let ne = edge_vec.len();
        let mut th_edges: Vec<(usize, V)> = Vec::new();
        let mut k: usize = 0;
        while k < ne
            invariant
                valid_key_type_Edge::<V>(),
                spec_graphview_wf(graph@),
                k <= ne,
                ne == edge_vec@.len(),
                nv == vertices_vec@.len(),
                // to_seq ensures for edges: edge membership ↔ graph@.A.
                forall|x: (V::V, V::V)| #[trigger] graph@.A.contains(x) <==>
                    edge_vec@.map(|_i: int, t: Edge<V>| t@).contains(x),
                // vertex_to_index invariants.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && #[trigger] vertices_vec@[j2]@ == v_view,
                forall|v_view: V::V| #[trigger] graph@.V.contains(v_view) ==>
                    vertex_to_index@.contains_key(v_view),
                // coin_flips covers all vertices.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] coin_flips@.contains_key(vertices_vec@[j2]@),
                // th_edges invariant.
                forall|s: int| 0 <= s < th_edges@.len() ==>
                    (th_edges@[s].0 as usize) < nv &&
                    #[trigger] coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                    !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                    coin_flips@.contains_key(th_edges@[s].1@) &&
                    coin_flips@[th_edges@[s].1@] &&
                    vertex_to_index@.contains_key(th_edges@[s].1@) &&
                    (vertex_to_index@[th_edges@[s].1@] as usize) < nv,
            decreases ne - k,
        {
            let edge = &edge_vec[k];
            let Edge(u, v) = edge;

            proof {
                // edge_vec@[k] is in graph.E@; its endpoints are in graph@.V.
                // Witness: map(f)[k] == edge_vec@[k]@ proves contains.
                let ghost mapped = edge_vec@.map(|_i: int, t: Edge<V>| t@);
                assert(mapped[k as int] == edge_vec@[k as int]@);
                assert(mapped.contains(edge_vec@[k as int]@));
                assert(graph@.A.contains(edge_vec@[k as int]@));
                assert(edge_vec@[k as int]@ == (u@, v@));
                assert(graph@.V.contains(u@) && graph@.V.contains(v@));
                assert(coin_flips@.contains_key(u@) && coin_flips@.contains_key(v@));
                assert(vertex_to_index@.contains_key(u@) && vertex_to_index@.contains_key(v@));
            }

            let u_heads = match coin_flips.get(u) { Some(val) => *val, None => false };
            let v_heads = match coin_flips.get(v) { Some(val) => *val, None => false };
            proof {
                assert(u_heads == coin_flips@[u@]);
                assert(v_heads == coin_flips@[v@]);
            }

            if !u_heads && v_heads {
                match vertex_to_index.get(u) {
                    Some(u_idx) => {
                        let ghost uid = *u_idx as usize;
                        let ghost pre_th = th_edges@;
                        let ghost pre_th_len = th_edges@.len() as int;
                        th_edges.push((*u_idx as usize, v.clone_view()));
                        proof {
                            assert(uid == vertex_to_index@[u@] as usize);
                            assert(uid < nv);
                            assert(vertices_vec@[uid as int]@ == u@);
                            assert(!coin_flips@[u@]);
                            assert(!coin_flips@[vertices_vec@[uid as int]@]);
                            assert(coin_flips@[v@]);
                            assert((vertex_to_index@[v@] as usize) < nv);
                            assert(th_edges@.len() == pre_th_len + 1);
                            assert forall|s: int| #![trigger th_edges@[s]] 0 <= s < th_edges@.len() implies
                                (th_edges@[s].0 as usize) < nv &&
                                coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                                !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                                coin_flips@.contains_key(th_edges@[s].1@) &&
                                coin_flips@[th_edges@[s].1@] &&
                                vertex_to_index@.contains_key(th_edges@[s].1@) &&
                                (vertex_to_index@[th_edges@[s].1@] as usize) < nv by {
                                if s < pre_th_len {
                                    assert(th_edges@[s] == pre_th[s]);
                                    assert(coin_flips@.contains_key(vertices_vec@[(pre_th[s].0 as usize) as int]@));
                                } else {
                                    assert(th_edges@[s].0 as usize == uid);
                                    assert(th_edges@[s].1@ == v@);
                                    assert(coin_flips@.contains_key(vertices_vec@[uid as int]@));
                                    assert(!coin_flips@[vertices_vec@[uid as int]@]);
                                    assert(coin_flips@.contains_key(v@));
                                    assert(coin_flips@[v@]);
                                    assert(vertex_to_index@.contains_key(v@));
                                    assert((vertex_to_index@[v@] as usize) < nv);
                                }
                            };
                        }
                    },
                    None => {},
                }
            } else if !v_heads && u_heads {
                match vertex_to_index.get(v) {
                    Some(v_idx) => {
                        let ghost vid = *v_idx as usize;
                        let ghost pre_th = th_edges@;
                        let ghost pre_th_len = th_edges@.len() as int;
                        th_edges.push((*v_idx as usize, u.clone_view()));
                        proof {
                            assert(vid == vertex_to_index@[v@] as usize);
                            assert(vid < nv);
                            assert(vertices_vec@[vid as int]@ == v@);
                            assert(!coin_flips@[v@]);
                            assert(!coin_flips@[vertices_vec@[vid as int]@]);
                            assert(coin_flips@[u@]);
                            assert((vertex_to_index@[u@] as usize) < nv);
                            assert(th_edges@.len() == pre_th_len + 1);
                            assert forall|s: int| #![trigger th_edges@[s]] 0 <= s < th_edges@.len() implies
                                (th_edges@[s].0 as usize) < nv &&
                                coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                                !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                                coin_flips@.contains_key(th_edges@[s].1@) &&
                                coin_flips@[th_edges@[s].1@] &&
                                vertex_to_index@.contains_key(th_edges@[s].1@) &&
                                (vertex_to_index@[th_edges@[s].1@] as usize) < nv by {
                                if s < pre_th_len {
                                    assert(th_edges@[s] == pre_th[s]);
                                    assert(coin_flips@.contains_key(vertices_vec@[(pre_th[s].0 as usize) as int]@));
                                } else {
                                    assert(th_edges@[s].0 as usize == vid);
                                    assert(th_edges@[s].1@ == u@);
                                    assert(coin_flips@.contains_key(vertices_vec@[vid as int]@));
                                    assert(!coin_flips@[vertices_vec@[vid as int]@]);
                                    assert(coin_flips@.contains_key(u@));
                                    assert(coin_flips@[u@]);
                                    assert(vertex_to_index@.contains_key(u@));
                                    assert((vertex_to_index@[u@] as usize) < nv);
                                }
                            };
                        }
                    },
                    None => {},
                }
            }
            // Re-establish th_edges invariant at merge point (after if/else if).
            proof {
                assert forall|s: int| #![trigger th_edges@[s]] 0 <= s < th_edges@.len() implies
                    (th_edges@[s].0 as usize) < nv &&
                    coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                    !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                    coin_flips@.contains_key(th_edges@[s].1@) &&
                    coin_flips@[th_edges@[s].1@] &&
                    vertex_to_index@.contains_key(th_edges@[s].1@) &&
                    (vertex_to_index@[th_edges@[s].1@] as usize) < nv by {
                    // On each path (push u→v, push v→u, no push), the invariant was proved.
                    assert(coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@));
                };
            }
            k = k + 1;
        }

        // Shadow th_edges as immutable so Verus knows it's unchanged in Loops 4-6.
        let th_edges = th_edges;

        // Bridge: th_edges invariant from Loop 3 for Loop 4 carry-through.
        proof {
            assert forall|s: int| #![trigger th_edges@[s]] 0 <= s < th_edges@.len() implies
                (th_edges@[s].0 as usize) < nv &&
                coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                coin_flips@.contains_key(th_edges@[s].1@) &&
                coin_flips@[th_edges@[s].1@] &&
                vertex_to_index@.contains_key(th_edges@[s].1@) &&
                (vertex_to_index@[th_edges@[s].1@] as usize) < nv by {
                assert(coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@));
            };
        }

        // Loop 4: initialize p_vec = vertices_vec.
        let mut p_vec: Vec<V> = Vec::new();
        let mut m: usize = 0;
        while m < nv
            invariant
                valid_key_type_Edge::<V>(),
                m <= nv,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                p_vec@.len() == m as int,
                forall|j2: int| 0 <= j2 < m as int ==> #[trigger] p_vec@[j2]@ == vertices_vec@[j2]@,
                // Carry-through: vertex_to_index invariants.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && #[trigger] vertices_vec@[j2]@ == v_view,
                // Carry-through: coin_flips covers all vertices.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] coin_flips@.contains_key(vertices_vec@[j2]@),
                // Carry-through: th_edges invariant.
                forall|s: int| 0 <= s < th_edges@.len() ==>
                    (th_edges@[s].0 as usize) < nv &&
                    #[trigger] coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                    !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                    coin_flips@.contains_key(th_edges@[s].1@) &&
                    coin_flips@[th_edges@[s].1@] &&
                    vertex_to_index@.contains_key(th_edges@[s].1@) &&
                    (vertex_to_index@[th_edges@[s].1@] as usize) < nv,
                // Carry-through: graph@.V <==> vertex_to_index.
                forall|v_view: V::V| #[trigger] graph@.V.contains(v_view) ==>
                    vertex_to_index@.contains_key(v_view),
                // Carry-through: to_seq ensures for vertices.
                forall|x: V::V| #[trigger] graph@.V.contains(x) <==>
                    vertices_vec@.map(|_i: int, t: V| t@).contains(x),
            decreases nv - m,
        {
            p_vec.push(vertices_vec[m].clone_view());
            m = m + 1;
        }

        // Loop 5: apply th_edges to p_vec.
        // Key invariant: heads vertices always keep p_vec[j] == vertices_vec[j].
        let nth = th_edges.len();
        let mut t: usize = 0;
        while t < nth
            invariant
                valid_key_type_Edge::<V>(),
                t <= nth,
                nth == th_edges@.len(),
                p_vec@.len() == nv as int,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                // Carry-through: to_seq ensures for vertices.
                forall|x: V::V| #[trigger] graph@.V.contains(x) <==>
                    vertices_vec@.map(|_i: int, t: V| t@).contains(x),
                // coin_flips covers all vertices (separate for trigger).
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] coin_flips@.contains_key(vertices_vec@[j2]@),
                // Heads preserve: coin_flips[vertices_vec[j]] == true => p_vec[j] == vertices_vec[j].
                forall|j2: int| 0 <= j2 < nv as int ==>
                    coin_flips@.contains_key(vertices_vec@[j2]@) &&
                    (coin_flips@[vertices_vec@[j2]@] ==>
                     #[trigger] p_vec@[j2]@ == vertices_vec@[j2]@),
                // Modified entries point to heads vertices (coin_flips == true).
                forall|j2: int| 0 <= j2 < nv as int ==>
                    p_vec@[j2]@ != vertices_vec@[j2]@ ==>
                    (coin_flips@.contains_key(p_vec@[j2]@) &&
                     #[trigger] coin_flips@[p_vec@[j2]@]),
                // All p_vec entries are in vertex_to_index (with valid indices).
                forall|j2: int| 0 <= j2 < nv as int ==>
                    vertex_to_index@.contains_key(#[trigger] p_vec@[j2]@) &&
                    (vertex_to_index@[p_vec@[j2]@] as usize) < nv,
                // vertex_to_index domain invariants.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && #[trigger] vertices_vec@[j2]@ == v_view,
                // th_edges invariant (immutable).
                forall|s: int| 0 <= s < th_edges@.len() ==>
                    (th_edges@[s].0 as usize) < nv &&
                    #[trigger] coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                    !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                    coin_flips@.contains_key(th_edges@[s].1@) &&
                    coin_flips@[th_edges@[s].1@] &&
                    vertex_to_index@.contains_key(th_edges@[s].1@) &&
                    (vertex_to_index@[th_edges@[s].1@] as usize) < nv,
            decreases nth - t,
        {
            let (idx, ref vertex) = th_edges[t];
            if (idx as usize) < nv {
                let ghost new_val = (*vertex)@;
                let ghost tail_view = vertices_vec@[idx as usize as int]@;
                let ghost pre_p = p_vec@;
                // Fire th_edges invariant trigger for s = t (before set).
                proof {
                    assert(coin_flips@.contains_key(vertices_vec@[(th_edges@[t as int].0 as usize) as int]@));
                    assert(!coin_flips@[tail_view]);
                    assert(coin_flips@.contains_key(th_edges@[t as int].1@));
                    assert(coin_flips@[new_val]);
                    assert(vertex_to_index@.contains_key(new_val));
                    assert((vertex_to_index@[new_val] as usize) < nv);
                }
                p_vec.set(idx as usize, vertex.clone_view());
                proof {
                    // After set: p_vec@[idx]@ == new_val (since clone_view preserves view).
                    assert(p_vec@[idx as usize as int]@ == new_val);
                    // new_val != tail_view since coin_flips differ.
                    assert(new_val != tail_view) by {
                        if new_val == tail_view { assert(false); }
                    };
                    // Heads preserve: for j == idx, coin_flips[vertices_vec[idx]] = false => vacuous.
                    assert forall|j2: int| 0 <= j2 < nv as int implies
                        coin_flips@.contains_key(#[trigger] vertices_vec@[j2]@) &&
                        (coin_flips@[vertices_vec@[j2]@] ==>
                         #[trigger] p_vec@[j2]@ == vertices_vec@[j2]@) by {
                        if j2 != idx as usize as int {
                            assert(p_vec@[j2] == pre_p[j2]);
                        }
                        // j2 == idx: coin_flips[tail_view] is false => implication vacuous.
                    };
                    // Modified entries point to heads.
                    assert forall|j2: int|
                        (0 <= j2 < nv as int && #[trigger] p_vec@[j2]@ != #[trigger] vertices_vec@[j2]@) implies
                        (coin_flips@.contains_key(p_vec@[j2]@) &&
                         coin_flips@[p_vec@[j2]@]) by {
                        if j2 == idx as usize as int {
                            assert(p_vec@[j2]@ == new_val);
                            assert(coin_flips@[new_val]);
                        } else {
                            assert(p_vec@[j2] == pre_p[j2]);
                        }
                    };
                    // vertex_to_index for p_vec entries.
                    assert forall|j2: int| 0 <= j2 < nv as int implies
                        vertex_to_index@.contains_key(#[trigger] p_vec@[j2]@) &&
                        (vertex_to_index@[p_vec@[j2]@] as usize) < nv by {
                        if j2 == idx as usize as int {
                            assert(p_vec@[j2]@ == new_val);
                            assert(vertex_to_index@.contains_key(new_val));
                            assert((vertex_to_index@[new_val] as usize) < nv);
                        } else {
                            assert(p_vec@[j2] == pre_p[j2]);
                        }
                    };
                }
            }
            // Re-assert th_edges invariant (th_edges is immutable, unchanged by p_vec.set).
            proof {
                assert forall|s: int| #![trigger th_edges@[s]] 0 <= s < th_edges@.len() implies
                    (th_edges@[s].0 as usize) < nv &&
                    coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                    !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                    coin_flips@.contains_key(th_edges@[s].1@) &&
                    coin_flips@[th_edges@[s].1@] &&
                    vertex_to_index@.contains_key(th_edges@[s].1@) &&
                    (vertex_to_index@[th_edges@[s].1@] as usize) < nv by {
                    assert(coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@));
                };
            }
            t = t + 1;
        }

        // Loop 6: build centers and partition_map.
        let mut centers: SetStEph<V> = SetLit![];
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();
        let mut q: usize = 0;
        while q < nv
            invariant
                valid_key_type_Edge::<V>(),
                centers.spec_setsteph_wf(),
                q <= nv,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                p_vec@.len() == nv as int,
                // to_seq ensures for vertices: vertex membership ↔ graph@.V.
                forall|x: V::V| #[trigger] graph@.V.contains(x) <==>
                    vertices_vec@.map(|_i: int, t: V| t@).contains(x),
                // Heads preserve and modified=heads (from loop 5, unchanged in loop 6).
                forall|j2: int| 0 <= j2 < nv as int ==>
                    coin_flips@.contains_key(vertices_vec@[j2]@) &&
                    (coin_flips@[vertices_vec@[j2]@] ==>
                     #[trigger] p_vec@[j2]@ == vertices_vec@[j2]@),
                forall|j2: int| 0 <= j2 < nv as int ==>
                    p_vec@[j2]@ != vertices_vec@[j2]@ ==>
                    (coin_flips@.contains_key(p_vec@[j2]@) &&
                     #[trigger] coin_flips@[p_vec@[j2]@]),
                // All p_vec entries in vertex_to_index.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    vertex_to_index@.contains_key(#[trigger] p_vec@[j2]@) &&
                    (vertex_to_index@[p_vec@[j2]@] as usize) < nv,
                // vertex_to_index domain.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && #[trigger] vertices_vec@[j2]@ == v_view,
                // Prefix: partition_map covers vertices_vec[0..q] with correct values.
                forall|j2: int| 0 <= j2 < q as int ==>
                    #[trigger] partition_map@.contains_key(vertices_vec@[j2]@) &&
                    partition_map@[vertices_vec@[j2]@]@ == p_vec@[j2]@,
                // Prefix: processed heads are in centers.
                forall|j2: int| 0 <= j2 < q as int ==>
                    p_vec@[j2]@ == vertices_vec@[j2]@ ==>
                    #[trigger] centers@.contains(p_vec@[j2]@),
                // All partition_map keys come from vertices_vec[0..q].
                forall|v_view: V::V| #[trigger] partition_map@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < q as int && #[trigger] vertices_vec@[j2]@ == v_view,
            decreases nv - q,
        {
            let vertex = &vertices_vec[q];
            let center = p_vec[q].clone_view();
            let ghost qv = (*vertex)@;
            let ghost cv = center@;
            let ghost pre_pm = partition_map@;
            let ghost pre_ctr = centers@;

            partition_map.insert(vertex.clone_view(), center.clone_view());

            proof {
                lemma_reveal_view_injective::<V>();
                assert(partition_map@ == pre_pm.insert(qv, center));
                assert(cv == p_vec@[q as int]@);
                assert(partition_map@[qv]@ == cv);
                // Prior entries are unchanged (qv can't equal earlier vertices by no_duplicates).
                assert forall|j2: int| 0 <= j2 < q as int implies
                    #[trigger] partition_map@.contains_key(vertices_vec@[j2]@) &&
                    partition_map@[vertices_vec@[j2]@]@ == p_vec@[j2]@ by {
                    let ghost jv = vertices_vec@[j2]@;
                    if jv != qv {
                        assert(pre_pm.contains_key(jv));
                        assert(partition_map@[jv] == pre_pm[jv]);
                    } else {
                        // jv == qv (view-equal). By obeys_feq_view_injective, value-equal.
                        assert(vertices_vec@[j2 as int] == vertices_vec@[q as int]);
                        assert(vertices_vec@.no_duplicates());
                        assert(false);
                    }
                };
                // All keys come from vertices_vec[0..q+1].
                assert forall|v_view: V::V| partition_map@.contains_key(v_view) implies
                    exists|j2: int| 0 <= j2 < q as int + 1 && #[trigger] vertices_vec@[j2]@ == v_view by {
                    if v_view != qv {
                        assert(pre_pm.contains_key(v_view));
                        let j2 = choose|j2: int| 0 <= j2 < q as int && #[trigger] vertices_vec@[j2]@ == v_view;
                        assert(0 <= j2 < q as int + 1);
                    }
                    // v_view == qv: witness j2 = q.
                };
            }

            if feq(vertex, &center) {
                let _ = centers.insert(vertex.clone_view());
                proof {
                    assert(centers@ == pre_ctr.insert(qv));
                    // feq ensures: feq(vertex, &center) == (vertex@ == center@), so cv == qv.
                    assert(cv == qv);
                    assert(p_vec@[q as int]@ == qv);
                    // q-th entry: p_vec@[q]@ == qv == vertices_vec@[q]@.
                    // centers@.contains(qv) from insert. ✓
                    assert(centers@.contains(qv));
                    // Prior heads-in-centers: pre_ctr had them; centers@ = pre_ctr.insert(qv) still does.
                    assert forall|j2: int|
                        (0 <= j2 < q as int && #[trigger] p_vec@[j2]@ == #[trigger] vertices_vec@[j2]@) implies
                        centers@.contains(p_vec@[j2]@) by {
                        // Both antecedents assumed via implies. Old invariant gives pre_ctr membership.
                        assert(pre_ctr.contains(p_vec@[j2]@));
                    };
                }
            } else {
                proof {
                    // feq returned false => vertex@ != center@.
                    assert((*vertex)@ != center@);
                    assert(qv != cv);
                    assert(p_vec@[q as int]@ != vertices_vec@[q as int]@);
                    // Prior entries unchanged.
                    assert forall|j2: int|
                        (0 <= j2 < q as int && #[trigger] p_vec@[j2]@ == #[trigger] vertices_vec@[j2]@) implies
                        centers@.contains(p_vec@[j2]@) by {
                        assert(pre_ctr.contains(p_vec@[j2]@));
                    };
                }
            }

            q = q + 1;
        }

        // Post-loop 6: prove spec_valid_partition_map.
        proof {
            // Part A: all graph@.V vertices are in partition_map.
            assert forall|v_view: V::V| #[trigger] graph@.V.contains(v_view) implies
                partition_map@.contains_key(v_view) by {
                assert(vertices_vec@.map(|_i: int, t: V| t@).contains(v_view));
                let k2 = vertices_vec@.map(|_i: int, t: V| t@).index_of(v_view);
                assert(0 <= k2 < nv as int);
                assert(vertices_vec@[k2]@ == v_view);
                // Loop 6 (q==nv): partition_map@.contains_key(vertices_vec@[k2]@).
                assert(partition_map@.contains_key(vertices_vec@[k2]@));
            };

            // Part B: all partition_map values are in centers.
            assert forall|v_view: V::V| #[trigger] partition_map@.contains_key(v_view) implies
                centers@.contains(partition_map@[v_view]@) by {
                // Any partition_map key came from vertices_vec (loop 6 domain invariant).
                let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices_vec@[j]@ == v_view;
                // partition_map@[v_view]@ == p_vec@[j]@ (loop 6 prefix).
                let ghost h = p_vec@[j]@;
                assert(partition_map@[vertices_vec@[j]@]@ == h);
                assert(partition_map@[v_view]@ == h);

                if h == vertices_vec@[j]@ {
                    // p_vec@[j]@ == vertices_vec@[j]@, so centers@.contains(h) from loop 6.
                    assert(centers@.contains(h));
                } else {
                    // h != vertices_vec@[j]@: from loop 5, h is a heads vertex.
                    assert(coin_flips@[h]);
                    // h is in vertex_to_index (loop 5 invariant).
                    assert(vertex_to_index@.contains_key(h));
                    let ghost q_h = vertex_to_index@[h] as usize;
                    assert(q_h < nv);
                    // vertex_to_index domain invariant: h ∈ domain => ∃j3 < nv: vertices_vec@[j3]@ == h.
                    let j3 = choose|j3: int| 0 <= j3 < nv as int && #[trigger] vertices_vec@[j3]@ == h;
                    // vertex_to_index@[h] == vertex_to_index@[vertices_vec@[j3]@] == j3 == q_h.
                    assert(vertex_to_index@[vertices_vec@[j3]@] as usize == j3);
                    assert(j3 as usize == q_h);
                    assert(vertices_vec@[q_h as int]@ == h);
                    // Heads preserve at q_h: coin_flips@[vertices_vec@[q_h]@] == coin_flips@[h] == true
                    // => p_vec@[q_h]@ == vertices_vec@[q_h]@ == h.
                    assert(coin_flips@[vertices_vec@[q_h as int]@]);
                    assert(p_vec@[q_h as int]@ == vertices_vec@[q_h as int]@);
                    assert(p_vec@[q_h as int]@ == h);
                    // Loop 6 prefix at q_h (q_h < nv = q after loop):
                    // p_vec@[q_h]@ == vertices_vec@[q_h]@ => centers@.contains(p_vec@[q_h]@).
                    assert(centers@.contains(p_vec@[q_h as int]@));
                    assert(centers@.contains(h));
                }
                assert(centers@.contains(partition_map@[v_view]@));
            };
        }

        (centers, partition_map)
    }

    } // verus!
}
