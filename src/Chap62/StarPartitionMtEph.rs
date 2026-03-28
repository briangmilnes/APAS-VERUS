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

    verus! {

    // 3. broadcast use

    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

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
    /// - APAS: Work O(n + m), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — all loops sequential.
    #[verifier::external_body]
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
                    exists|j: int| 0 <= j < i as int && vertices_vec@[j]@ == v_view,
            decreases nv - i,
        {
            let ghost iv = vertices_vec@[i as int]@;
            vertex_to_index.insert(vertices_vec[i].clone_view(), i as usize);
            proof {
                // After inserting vertices_vec[i] -> i, prove invariants for range [0..i+1].
                assert forall|j: int| 0 <= j < i as int + 1 implies
                    vertex_to_index@.contains_key(vertices_vec@[j]@) &&
                    vertex_to_index@[vertices_vec@[j]@] as usize == j by {
                    if j < i as int {
                        let ghost jv = vertices_vec@[j]@;
                        if jv != iv {
                            assert(vertex_to_index@.contains_key(jv));
                            assert(vertex_to_index@[jv] as usize == j);
                        } else {
                            // no_duplicates => j != i since jv == iv and j < i.
                            assert(vertices_vec@.no_duplicates());
                            assert(false);
                        }
                    }
                };
                // Domain update: any new key must be vertices_vec@[i]@ or an old key.
                assert forall|v_view: V::V| vertex_to_index@.contains_key(v_view) implies
                    exists|j: int| 0 <= j < i as int + 1 && vertices_vec@[j]@ == v_view by {
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
                    coin_flips@.contains_key(vertices_vec@[jj]@) by {
                    if jj < j as int {
                        let ghost jjv = vertices_vec@[jj]@;
                        let ghost jv2 = vertices_vec@[j as int]@;
                        if jjv != jv2 {
                            assert(coin_flips@.contains_key(jjv));
                        } else {
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
                // vertex_to_index invariants.
                forall|j2: int| 0 <= j2 < nv as int ==>
                    #[trigger] vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && vertices_vec@[j2]@ == v_view,
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
                assert(edge_vec@.map(|_i: int, t: Edge<V>| t@).contains(edge_vec@[k as int]@));
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
                        proof {
                            let ghost uid = *u_idx as usize;
                            assert(uid == vertex_to_index@[u@] as usize);
                            assert(uid < nv);
                            assert(vertices_vec@[uid as int]@ == u@);
                            assert(!coin_flips@[u@]);
                            assert(!coin_flips@[vertices_vec@[uid as int]@]);
                            assert(coin_flips@[v@]);
                            assert((vertex_to_index@[v@] as usize) < nv);
                            let ghost new_entry: (usize, V) = (*u_idx as usize, *v);
                            assert forall|s: int| 0 <= s < th_edges@.len() + 1 implies
                                (th_edges@.push(new_entry)[s].0 as usize) < nv &&
                                coin_flips@.contains_key(vertices_vec@[(th_edges@.push(new_entry)[s].0 as usize) as int]@) &&
                                !coin_flips@[vertices_vec@[(th_edges@.push(new_entry)[s].0 as usize) as int]@] &&
                                coin_flips@.contains_key(th_edges@.push(new_entry)[s].1@) &&
                                coin_flips@[th_edges@.push(new_entry)[s].1@] &&
                                vertex_to_index@.contains_key(th_edges@.push(new_entry)[s].1@) &&
                                (vertex_to_index@[th_edges@.push(new_entry)[s].1@] as usize) < nv by {
                                if s < th_edges@.len() {
                                    assert(th_edges@.push(new_entry)[s] == th_edges@[s]);
                                }
                            };
                        }
                        th_edges.push((*u_idx as usize, v.clone_view()));
                    },
                    None => {},
                }
            }
            if !v_heads && u_heads {
                match vertex_to_index.get(v) {
                    Some(v_idx) => {
                        proof {
                            let ghost vid = *v_idx as usize;
                            assert(vid == vertex_to_index@[v@] as usize);
                            assert(vid < nv);
                            assert(vertices_vec@[vid as int]@ == v@);
                            assert(!coin_flips@[v@]);
                            assert(!coin_flips@[vertices_vec@[vid as int]@]);
                            assert(coin_flips@[u@]);
                            assert((vertex_to_index@[u@] as usize) < nv);
                            let ghost new_entry: (usize, V) = (*v_idx as usize, *u);
                            assert forall|s: int| 0 <= s < th_edges@.len() + 1 implies
                                (th_edges@.push(new_entry)[s].0 as usize) < nv &&
                                coin_flips@.contains_key(vertices_vec@[(th_edges@.push(new_entry)[s].0 as usize) as int]@) &&
                                !coin_flips@[vertices_vec@[(th_edges@.push(new_entry)[s].0 as usize) as int]@] &&
                                coin_flips@.contains_key(th_edges@.push(new_entry)[s].1@) &&
                                coin_flips@[th_edges@.push(new_entry)[s].1@] &&
                                vertex_to_index@.contains_key(th_edges@.push(new_entry)[s].1@) &&
                                (vertex_to_index@[th_edges@.push(new_entry)[s].1@] as usize) < nv by {
                                if s < th_edges@.len() {
                                    assert(th_edges@.push(new_entry)[s] == th_edges@[s]);
                                }
                            };
                        }
                        th_edges.push((*v_idx as usize, u.clone_view()));
                    },
                    None => {},
                }
            }
            k = k + 1;
        }

        // Loop 4: initialize p_vec = vertices_vec.
        let mut p_vec: Vec<V> = Vec::new();
        let mut m: usize = 0;
        while m < nv
            invariant
                m <= nv,
                nv == vertices_vec@.len(),
                p_vec@.len() == m as int,
                forall|j2: int| 0 <= j2 < m as int ==> p_vec@[j2]@ == vertices_vec@[j2]@,
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
                    vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && vertices_vec@[j2]@ == v_view,
                // th_edges invariant (immutable).
                forall|s: int| 0 <= s < th_edges@.len() ==>
                    (th_edges@[s].0 as usize) < nv &&
                    coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                    !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                    coin_flips@.contains_key(th_edges@[s].1@) &&
                    #[trigger] coin_flips@[th_edges@[s].1@] &&
                    vertex_to_index@.contains_key(th_edges@[s].1@) &&
                    (vertex_to_index@[th_edges@[s].1@] as usize) < nv,
            decreases nth - t,
        {
            let (idx, ref vertex) = th_edges[t];
            if (idx as usize) < nv {
                proof {
                    let ghost new_val = (*vertex)@;
                    let ghost tail_view = vertices_vec@[idx as usize as int]@;
                    assert(!coin_flips@[tail_view]);
                    assert(coin_flips@[new_val]);
                    // new_val != tail_view since coin_flips differ.
                    assert(new_val != tail_view) by {
                        if new_val == tail_view {
                            assert(false);
                        }
                    };
                    // Heads preserve: for j == idx, coin_flips[vertices_vec[idx]] = false => implication vacuous.
                    assert forall|j2: int| 0 <= j2 < nv as int implies
                        coin_flips@.contains_key(vertices_vec@[j2]@) &&
                        (coin_flips@[vertices_vec@[j2]@] ==>
                         p_vec@.update(idx as usize as int, *vertex)[j2]@ == vertices_vec@[j2]@) by {
                        if j2 != idx as usize as int {
                            assert(p_vec@.update(idx as usize as int, *vertex)[j2] == p_vec@[j2]);
                        }
                        // j2 == idx: implication is false => true (coin_flips[tail] = false).
                    };
                    // Modified entries point to heads: for j == idx, new_val is heads.
                    assert forall|j2: int| 0 <= j2 < nv as int implies
                        p_vec@.update(idx as usize as int, *vertex)[j2]@ != vertices_vec@[j2]@ ==>
                        (coin_flips@.contains_key(p_vec@.update(idx as usize as int, *vertex)[j2]@) &&
                         coin_flips@[p_vec@.update(idx as usize as int, *vertex)[j2]@]) by {
                        if j2 == idx as usize as int {
                            assert(p_vec@.update(idx as usize as int, *vertex)[j2]@ == new_val);
                            assert(coin_flips@[new_val]);
                        } else {
                            assert(p_vec@.update(idx as usize as int, *vertex)[j2] == p_vec@[j2]);
                        }
                    };
                    // vertex_to_index: for j == idx, new_val = th_edges[t].1 has vertex_to_index entry.
                    assert forall|j2: int| 0 <= j2 < nv as int implies
                        #[trigger] vertex_to_index@.contains_key(p_vec@.update(idx as usize as int, *vertex)[j2]@) &&
                        (vertex_to_index@[p_vec@.update(idx as usize as int, *vertex)[j2]@] as usize) < nv by {
                        if j2 == idx as usize as int {
                            assert(p_vec@.update(idx as usize as int, *vertex)[j2]@ == new_val);
                            assert(vertex_to_index@.contains_key(new_val));
                            assert((vertex_to_index@[new_val] as usize) < nv);
                        } else {
                            assert(p_vec@.update(idx as usize as int, *vertex)[j2] == p_vec@[j2]);
                        }
                    };
                }
                p_vec.set(idx as usize, vertex.clone_view());
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
                p_vec@.len() == nv as int,
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
                    vertex_to_index@.contains_key(vertices_vec@[j2]@) &&
                    vertex_to_index@[vertices_vec@[j2]@] as usize == j2,
                forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                    exists|j2: int| 0 <= j2 < nv as int && vertices_vec@[j2]@ == v_view,
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
                    exists|j2: int| 0 <= j2 < q as int && vertices_vec@[j2]@ == v_view,
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
                assert(partition_map@ == pre_pm.insert(qv, center));
                assert(cv == p_vec@[q as int]@);
                assert(partition_map@[qv]@ == cv);
                // Prior entries are unchanged (qv can't equal earlier vertices by no_duplicates).
                assert forall|j2: int| 0 <= j2 < q as int implies
                    partition_map@.contains_key(vertices_vec@[j2]@) &&
                    partition_map@[vertices_vec@[j2]@]@ == p_vec@[j2]@ by {
                    let ghost jv = vertices_vec@[j2]@;
                    if jv != qv {
                        assert(pre_pm.contains_key(jv));
                        assert(partition_map@[jv] == pre_pm[jv]);
                    } else {
                        assert(vertices_vec@.no_duplicates());
                        assert(false);
                    }
                };
                // All keys come from vertices_vec[0..q+1].
                assert forall|v_view: V::V| partition_map@.contains_key(v_view) implies
                    exists|j2: int| 0 <= j2 < q as int + 1 && vertices_vec@[j2]@ == v_view by {
                    if v_view != qv {
                        assert(pre_pm.contains_key(v_view));
                        let j2 = choose|j2: int| 0 <= j2 < q as int && #[trigger] vertices_vec@[j2]@ == v_view;
                        assert(0 <= j2 < q as int + 1);
                    }
                    // v_view == qv: witness j2 = q.
                };
            }

            if *vertex == center {
                let _ = centers.insert(vertex.clone_view());
                proof {
                    assert(centers@ == pre_ctr.insert(qv));
                    // (*vertex)@ == qv and center@ == cv; since *vertex == center, qv == cv.
                    assert(cv == qv);
                    assert(p_vec@[q as int]@ == qv);
                    // q-th entry: p_vec@[q]@ == qv == vertices_vec@[q]@.
                    // centers@.contains(qv) from insert. ✓
                    assert(centers@.contains(qv));
                    // Prior heads-in-centers: pre_ctr had them; centers@ = pre_ctr.insert(qv) still does.
                    assert forall|j2: int| 0 <= j2 < q as int implies
                        p_vec@[j2]@ == vertices_vec@[j2]@ ==>
                        centers@.contains(p_vec@[j2]@) by {
                        assert(pre_ctr.contains(p_vec@[j2]@));
                    };
                }
            } else {
                proof {
                    // *vertex != center => qv != cv (from eq spec).
                    assert((*vertex)@ != center@);
                    assert(qv != cv);
                    assert(p_vec@[q as int]@ != vertices_vec@[q as int]@);
                    // Loop 6 invariant at q is vacuously true (p_vec@[q]@ != vertices_vec@[q]@).
                    // Prior entries unchanged.
                    assert forall|j2: int| 0 <= j2 < q as int implies
                        p_vec@[j2]@ == vertices_vec@[j2]@ ==>
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
                let j = choose|j: int| 0 <= j < nv as int && vertices_vec@[j]@ == v_view;
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
