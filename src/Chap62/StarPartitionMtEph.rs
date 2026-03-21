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
            requires Self::spec_starpartitionmteph_wf(graph);
    }

    pub type T<V> = UnDirGraphMtEph<V>;

    /// Algorithm 62.3: Parallel Star Partition.
    ///
    /// - APAS: Work O(n + m), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — all loops sequential.
    pub fn parallel_star_partition<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
        ensures
            result.0.spec_setsteph_wf(),
            // Every graph vertex appears as a key in partition_map.
            forall |v_view: V::V| graph@.V.contains(v_view) ==>
                #[trigger] result.1@.contains_key(v_view),
            // Every partition_map value is a center.
            forall |v_view: V::V| result.1@.contains_key(v_view) ==>
                result.0@.contains(#[trigger] result.1@[v_view]@),
    {
        let vertices_vec = graph.V.to_seq();
        let edge_vec = graph.E.to_seq();
        let nv = vertices_vec.len();
        let ne = edge_vec.len();

        // Build vertex-to-index map: vertices_vec[k] -> k.
        let mut vertex_to_index = HashMapWithViewPlus::<V, N>::new();
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < nv
            invariant
                i <= nv,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                // Direct mapping: vertices_vec[k]@ -> k.
                forall |k: int| 0 <= k < i as int ==>
                    vertex_to_index@.contains_key(#[trigger] vertices_vec@[k]@),
                forall |k: int| 0 <= k < i as int ==>
                    vertex_to_index@[vertices_vec@[k]@] as usize == k,
                // All keys come from vertices_vec[0..i].
                forall |x: V::V| vertex_to_index@.contains_key(x) ==>
                    exists |k: int| 0 <= k < i as int && vertices_vec@[k]@ == x,
            decreases nv - i,
        {
            let vi = vertices_vec[i].clone_view();
            vertex_to_index.insert(vi, i as N);
            proof {
                assert(vi@ == vertices_vec@[i as int]@);
                assert(vertex_to_index@.contains_key(vertices_vec@[i as int]@));
                assert(vertex_to_index@[vertices_vec@[i as int]@] as usize == i);
            }
            i = i + 1;
        }

        // Flip coins for each vertex.
        let mut rng = seeded_rng(seed);
        let mut coin_flips = HashMapWithViewPlus::<V, bool>::new();
        let mut j: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while j < nv
            invariant
                j <= nv,
                nv == vertices_vec@.len(),
                // coin_flips covers vertices_vec[0..j].
                forall |k: int| 0 <= k < j as int ==>
                    coin_flips@.contains_key(#[trigger] vertices_vec@[k]@),
                // All coin_flips keys come from vertices_vec[0..j].
                forall |x: V::V| coin_flips@.contains_key(x) ==>
                    exists |k: int| 0 <= k < j as int && vertices_vec@[k]@ == x,
            decreases nv - j,
        {
            let vj = vertices_vec[j].clone_view();
            coin_flips.insert(vj, random_bool_seeded(&mut rng));
            proof {
                assert(vj@ == vertices_vec@[j as int]@);
            }
            j = j + 1;
        }

        // Build tail-heads edges: for each edge (u,v), if u is tails and v is heads,
        // record that u should map to v.
        let mut th_edges: Vec<(N, V)> = Vec::new();
        let mut k: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while k < ne
            invariant
                k <= ne,
                ne == edge_vec@.len(),
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                // vertex_to_index covers all vertices.
                forall |m: int| 0 <= m < nv as int ==>
                    vertex_to_index@.contains_key(#[trigger] vertices_vec@[m]@),
                forall |m: int| 0 <= m < nv as int ==>
                    vertex_to_index@[vertices_vec@[m]@] as usize == m,
                forall |x: V::V| vertex_to_index@.contains_key(x) ==>
                    exists |k2: int| 0 <= k2 < nv as int && vertices_vec@[k2]@ == x,
                // coin_flips covers all vertices.
                forall |m: int| 0 <= m < nv as int ==>
                    coin_flips@.contains_key(#[trigger] vertices_vec@[m]@),
                forall |x: V::V| coin_flips@.contains_key(x) ==>
                    exists |k2: int| 0 <= k2 < nv as int && vertices_vec@[k2]@ == x,
                // th_edges invariant: each (idx, v) has:
                //   idx < nv, v is a heads graph vertex, vertices_vec[idx]@ is tails.
                forall |t: int| 0 <= t < th_edges@.len() ==> {
                    let (idx, v) = th_edges@[t];
                    &&& idx as usize < nv
                    &&& coin_flips@.contains_key(v@)
                    &&& coin_flips@[v@] == true
                    &&& vertices_vec@.map(|_i: int, vt: V| vt@).contains(v@)
                    &&& coin_flips@.contains_key(vertices_vec@[idx as usize]@)
                    &&& coin_flips@[vertices_vec@[idx as usize]@] == false
                },
            decreases ne - k,
        {
            let edge = &edge_vec[k];
            let Edge(u, v) = edge;

            // Prove u@ and v@ are graph vertices (endpoints of graph edges).
            proof {
                assert(edge_vec@.map(|_i: int, t: Edge<V>| t@)[k as int] == edge_vec@[k as int]@);
                assert(edge_vec@.map(|_i: int, t: Edge<V>| t@).contains(edge_vec@[k as int]@));
                assert(graph.E@.contains(edge_vec@[k as int]@));
                assert(edge_vec@[k as int]@ == ((*u)@, (*v)@));
                assert(graph@.A.contains(((*u)@, (*v)@)));
                assert(graph@.V.contains((*u)@) && graph@.V.contains((*v)@));
                assert(vertices_vec@.map(|_i: int, t: V| t@).contains((*u)@));
                assert(vertices_vec@.map(|_i: int, t: V| t@).contains((*v)@));
                let ku = choose|ku: int| 0 <= ku < nv as int && vertices_vec@[ku]@ == (*u)@;
                assert(coin_flips@.contains_key(vertices_vec@[ku]@));
                assert(coin_flips@.contains_key((*u)@));
                let kv = choose|kv: int| 0 <= kv < nv as int && vertices_vec@[kv]@ == (*v)@;
                assert(coin_flips@.contains_key(vertices_vec@[kv]@));
                assert(coin_flips@.contains_key((*v)@));
            }

            let u_heads = match coin_flips.get(u) {
                Some(val) => *val,
                None => { proof { assert(false); } false }
            };
            let v_heads = match coin_flips.get(v) {
                Some(val) => *val,
                None => { proof { assert(false); } false }
            };

            if !u_heads && v_heads {
                match vertex_to_index.get(u) {
                    Some(u_idx) => {
                        let vc = v.clone_view();
                        proof {
                            assert(vc@ == (*v)@);
                            assert(coin_flips@[vc@] == true);
                            // u_idx = vertex_to_index@[(*u)@].
                            assert(*u_idx == vertex_to_index@[(*u)@]);
                            // Find the k with vertices_vec@[k]@ == (*u)@.
                            let ku = choose|ku: int| 0 <= ku < nv as int && vertices_vec@[ku]@ == (*u)@;
                            assert(vertex_to_index@[vertices_vec@[ku]@] as usize == ku);
                            assert(vertex_to_index@[(*u)@] as usize == ku);
                            assert(*u_idx as usize == ku);
                            assert(*u_idx as usize < nv);
                            // vertices_vec@[ku]@ == (*u)@ and coin_flips@[(*u)@] == false.
                            assert(coin_flips@[vertices_vec@[*u_idx as usize]@] == false);
                            assert(vertices_vec@.map(|_i: int, vt: V| vt@).contains(vc@));
                        }
                        th_edges.push((*u_idx, vc));
                    },
                    None => {},
                }
            }
            if !v_heads && u_heads {
                match vertex_to_index.get(v) {
                    Some(v_idx) => {
                        let uc = u.clone_view();
                        proof {
                            assert(uc@ == (*u)@);
                            assert(coin_flips@[uc@] == true);
                            assert(*v_idx == vertex_to_index@[(*v)@]);
                            let kv = choose|kv: int| 0 <= kv < nv as int && vertices_vec@[kv]@ == (*v)@;
                            assert(vertex_to_index@[vertices_vec@[kv]@] as usize == kv);
                            assert(vertex_to_index@[(*v)@] as usize == kv);
                            assert(*v_idx as usize == kv);
                            assert(*v_idx as usize < nv);
                            assert(coin_flips@[vertices_vec@[*v_idx as usize]@] == false);
                            assert(vertices_vec@.map(|_i: int, vt: V| vt@).contains(uc@));
                        }
                        th_edges.push((*v_idx, uc));
                    },
                    None => {},
                }
            }
            k = k + 1;
        }

        // Build p_vec: initially each vertex maps to itself.
        let mut p_vec: Vec<V> = Vec::new();
        let mut m: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while m < nv
            invariant
                m <= nv,
                nv == vertices_vec@.len(),
                p_vec@.len() == m,
                forall |k: int| 0 <= k < m as int ==>
                    p_vec@[k]@ == vertices_vec@[k]@,
            decreases nv - m,
        {
            let vm = vertices_vec[m].clone_view();
            p_vec.push(vm);
            proof {
                assert(vertices_vec[m].clone_view()@ == vertices_vec@[m as int]@);
                assert(p_vec@[m as int]@ == vertices_vec@[m as int]@);
            }
            m = m + 1;
        }

        // Apply tail-heads edges: update p_vec.
        let nth = th_edges.len();
        let mut t: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while t < nth
            invariant
                t <= nth,
                nth == th_edges@.len(),
                p_vec@.len() == nv,
                nv == vertices_vec@.len(),
                vertices_vec@.no_duplicates(),
                // vertex_to_index covers all vertices with exact inverse.
                forall |mk: int| 0 <= mk < nv as int ==>
                    vertex_to_index@.contains_key(#[trigger] vertices_vec@[mk]@),
                forall |mk: int| 0 <= mk < nv as int ==>
                    vertex_to_index@[vertices_vec@[mk]@] as usize == mk,
                forall |x: V::V| vertex_to_index@.contains_key(x) ==>
                    exists |k2: int| 0 <= k2 < nv as int && vertices_vec@[k2]@ == x,
                // coin_flips covers all vertices.
                forall |mk: int| 0 <= mk < nv as int ==>
                    coin_flips@.contains_key(#[trigger] vertices_vec@[mk]@),
                // th_edges invariant (carried from th_edges construction loop).
                forall |s: int| 0 <= s < th_edges@.len() ==> {
                    let (idx, v) = th_edges@[s];
                    &&& idx as usize < nv
                    &&& coin_flips@.contains_key(v@)
                    &&& coin_flips@[v@] == true
                    &&& vertices_vec@.map(|_i: int, vt: V| vt@).contains(v@)
                    &&& coin_flips@.contains_key(vertices_vec@[idx as usize]@)
                    &&& coin_flips@[vertices_vec@[idx as usize]@] == false
                },
                // Invariant A: p_vec[mk]@ is self or heads.
                forall |mk: int| 0 <= mk < nv as int ==>
                    p_vec@[mk]@ == vertices_vec@[mk]@ ||
                    (coin_flips@.contains_key(#[trigger] p_vec@[mk]@) &&
                     coin_flips@[p_vec@[mk]@] == true),
                // Invariant B: heads vertices always map to themselves in p_vec.
                forall |mk: int| 0 <= mk < nv as int ==>
                    (coin_flips@.contains_key(vertices_vec@[mk]@) &&
                     coin_flips@[vertices_vec@[mk]@] == true) ==>
                        p_vec@[mk]@ == vertices_vec@[mk]@,
                // p_vec values appear in vertices_vec.
                forall |mk: int| 0 <= mk < nv as int ==>
                    vertices_vec@.map(|_i: int, vt: V| vt@).contains(#[trigger] p_vec@[mk]@),
            decreases nth - t,
        {
            let (idx, ref center) = th_edges[t];
            if (idx as usize) < nv {
                let ghost old_p = p_vec@;
                let cc = center.clone_view();
                proof {
                    assert(cc@ == center@);
                    let (th_idx, th_v) = th_edges@[t as int];
                    assert(th_idx == idx);
                    assert(th_v@ == center@);
                    assert(coin_flips@[cc@] == true);
                    assert(vertices_vec@.map(|_i: int, vt: V| vt@).contains(cc@));
                }
                p_vec.set(idx as usize, cc);
                proof {
                    assert(p_vec@ == old_p.update(idx as usize as int, cc));
                    // Invariant A for all positions.
                    assert forall |mk: int| 0 <= mk < nv as int implies
                        p_vec@[mk]@ == vertices_vec@[mk]@ ||
                        (coin_flips@.contains_key(p_vec@[mk]@) && coin_flips@[p_vec@[mk]@] == true) by {
                        if mk == idx as usize as int {
                            assert(p_vec@[mk]@ == cc@);
                            assert(coin_flips@[p_vec@[mk]@] == true);
                        } else {
                            assert(p_vec@[mk] == old_p[mk]);
                            assert(p_vec@[mk]@ == old_p[mk]@);
                        }
                    };
                    // Invariant B: heads are fixpoints.
                    assert forall |mk: int| 0 <= mk < nv as int &&
                        coin_flips@.contains_key(vertices_vec@[mk]@) &&
                        coin_flips@[vertices_vec@[mk]@] == true
                        implies p_vec@[mk]@ == vertices_vec@[mk]@ by {
                        // The vertex at idx is tails (from th_edges invariant).
                        assert(coin_flips@[vertices_vec@[idx as usize]@] == false);
                        if mk == idx as usize as int {
                            // vertices_vec@[mk]@ is tails — contradicts it being heads.
                            assert(coin_flips@[vertices_vec@[mk]@] == false);
                            assert(false);
                        }
                        // mk != idx: p_vec@[mk] unchanged.
                        assert(p_vec@[mk] == old_p[mk]);
                        assert(p_vec@[mk]@ == old_p[mk]@);
                    };
                    // p_vec values remain in vertices_vec.
                    assert forall |mk: int| 0 <= mk < nv as int implies
                        vertices_vec@.map(|_i: int, vt: V| vt@).contains(p_vec@[mk]@) by {
                        if mk == idx as usize as int {
                            assert(p_vec@[mk]@ == cc@);
                        } else {
                            assert(p_vec@[mk] == old_p[mk]);
                            assert(p_vec@[mk]@ == old_p[mk]@);
                        }
                    };
                }
            }
            t = t + 1;
        }

        // Prove the fixpoint property: every p_vec value has a fixpoint.
        proof {
            assert forall |mk: int| 0 <= mk < nv as int implies
                exists |kf: int| 0 <= kf < nv as int &&
                    vertices_vec@[kf]@ == #[trigger] p_vec@[mk]@ &&
                    p_vec@[kf]@ == vertices_vec@[kf]@ by {
                if p_vec@[mk]@ == vertices_vec@[mk]@ {
                    // kf = mk: trivially a fixpoint.
                } else {
                    // By Invariant A: p_vec@[mk]@ is heads.
                    assert(coin_flips@[p_vec@[mk]@] == true);
                    // p_vec@[mk]@ appears in vertices_vec.
                    assert(vertices_vec@.map(|_i: int, vt: V| vt@).contains(p_vec@[mk]@));
                    let kf = choose|kf: int| 0 <= kf < nv as int && vertices_vec@[kf]@ == p_vec@[mk]@;
                    // vertices_vec@[kf]@ is heads: coin_flips@[vertices_vec@[kf]@] == true.
                    assert(coin_flips@[vertices_vec@[kf]@] == true);
                    // By Invariant B: p_vec@[kf]@ == vertices_vec@[kf]@.
                    assert(p_vec@[kf]@ == vertices_vec@[kf]@);
                }
            };
        }

        // First pass: build centers (fixpoint vertices — those that map to themselves in p_vec).
        // A vertex v = vertices_vec[q] is a fixpoint iff vertex_to_index.get(&p_vec[q]) == Some(q).
        let mut centers: SetStEph<V> = SetLit![];
        let mut q1: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while q1 < nv
            invariant
                valid_key_type_Edge::<V>(),
                centers.spec_setsteph_wf(),
                q1 <= nv,
                nv == vertices_vec@.len(),
                p_vec@.len() == nv,
                vertices_vec@.no_duplicates(),
                // vertex_to_index: exact inverse.
                forall |mk: int| 0 <= mk < nv as int ==>
                    vertex_to_index@.contains_key(#[trigger] vertices_vec@[mk]@),
                forall |mk: int| 0 <= mk < nv as int ==>
                    vertex_to_index@[vertices_vec@[mk]@] as usize == mk,
                forall |x: V::V| vertex_to_index@.contains_key(x) ==>
                    exists |k2: int| 0 <= k2 < nv as int && vertices_vec@[k2]@ == x,
                // coin_flips covers all vertices.
                forall |mk: int| 0 <= mk < nv as int ==>
                    coin_flips@.contains_key(#[trigger] vertices_vec@[mk]@),
                // Invariant B holds.
                forall |mk: int| 0 <= mk < nv as int ==>
                    (coin_flips@.contains_key(vertices_vec@[mk]@) &&
                     coin_flips@[vertices_vec@[mk]@] == true) ==>
                        p_vec@[mk]@ == vertices_vec@[mk]@,
                // p_vec values in vertices_vec.
                forall |mk: int| 0 <= mk < nv as int ==>
                    vertices_vec@.map(|_i: int, vt: V| vt@).contains(#[trigger] p_vec@[mk]@),
                // Fixpoint property.
                forall |mk: int| 0 <= mk < nv as int ==>
                    exists |kf: int| 0 <= kf < nv as int &&
                        vertices_vec@[kf]@ == p_vec@[mk]@ && p_vec@[kf]@ == vertices_vec@[kf]@,
                // Centers contains all fixpoints in 0..q1.
                forall |k: int| 0 <= k < q1 as int ==>
                    p_vec@[k]@ == vertices_vec@[k]@ ==>
                        centers@.contains(#[trigger] vertices_vec@[k]@),
            decreases nv - q1,
        {
            let center = &p_vec[q1];
            let vertex = &vertices_vec[q1];
            // Check fixpoint: is vertex_to_index[p_vec[q1]] == q1?
            if let Some(center_idx) = vertex_to_index.get(center) {
                if *center_idx as usize == q1 {
                    // Fixpoint: p_vec@[q1]@ == vertices_vec@[q1]@.
                    proof {
                        // center_idx = vertex_to_index@[(*center)@] as usize = q1.
                        assert(*center_idx == vertex_to_index@[(*center)@]);
                        assert(*center_idx as usize == q1);
                        // From "keys come from vertices_vec": exists k2 with vertices_vec@[k2]@ == (*center)@.
                        let k2 = choose|k2: int| 0 <= k2 < nv as int && vertices_vec@[k2]@ == (*center)@;
                        // vertex_to_index@[vertices_vec@[k2]@] as usize == k2.
                        assert(vertex_to_index@[vertices_vec@[k2]@] as usize == k2);
                        // vertex_to_index@[(*center)@] as usize == k2.
                        assert(vertex_to_index@[(*center)@] as usize == k2);
                        // *center_idx as usize = q1 = k2.
                        assert(k2 == q1 as int);
                        // Hence (*center)@ == vertices_vec@[q1]@.
                        assert((*center)@ == vertices_vec@[q1 as int]@);
                        assert(p_vec@[q1 as int]@ == vertices_vec@[q1 as int]@);
                    }
                    let vc = vertex.clone_view();
                    proof { assert(vc@ == (*vertex)@); }
                    let _ = centers.insert(vc);
                    proof {
                        assert(centers@.contains((*vertex)@));
                        assert((*vertex)@ == vertices_vec@[q1 as int]@);
                    }
                }
            }
            q1 = q1 + 1;
        }

        // After first pass: prove every p_vec value is in centers.
        proof {
            assert forall |mk: int| 0 <= mk < nv as int implies
                centers@.contains(#[trigger] p_vec@[mk]@) by {
                // By fixpoint property: exists kf s.t. vertices_vec@[kf]@ == p_vec@[mk]@ and p_vec@[kf]@ == vertices_vec@[kf]@.
                let kf = choose|kf: int| 0 <= kf < nv as int &&
                    vertices_vec@[kf]@ == p_vec@[mk]@ && p_vec@[kf]@ == vertices_vec@[kf]@;
                // kf < q1 = nv after loop: centers invariant applies.
                assert(p_vec@[kf]@ == vertices_vec@[kf]@);
                // By centers invariant (kf < q1): centers@.contains(vertices_vec@[kf]@).
                assert(centers@.contains(vertices_vec@[kf]@));
                // vertices_vec@[kf]@ == p_vec@[mk]@.
                assert(centers@.contains(p_vec@[mk]@));
            };
        }

        // Second pass: build partition_map (all vertices, each mapped to its p_vec center).
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();
        let mut q2: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while q2 < nv
            invariant
                valid_key_type_Edge::<V>(),
                q2 <= nv,
                nv == vertices_vec@.len(),
                p_vec@.len() == nv,
                centers.spec_setsteph_wf(),
                // All p_vec values are in centers.
                forall |mk: int| 0 <= mk < nv as int ==>
                    centers@.contains(#[trigger] p_vec@[mk]@),
                // partition_map covers 0..q2.
                forall |k: int| 0 <= k < q2 as int ==>
                    partition_map@.contains_key(#[trigger] vertices_vec@[k]@),
                // partition_map values are centers.
                forall |k: int| 0 <= k < q2 as int ==>
                    centers@.contains(#[trigger] partition_map@[vertices_vec@[k]@]@),
                // partition_map domain is exactly {vertices_vec@[k]@ : k in 0..q2}.
                forall |v_view: V::V| partition_map@.contains_key(v_view) ==>
                    exists |k: int| 0 <= k < q2 as int && vertices_vec@[k]@ == v_view,
            decreases nv - q2,
        {
            let vertex = &vertices_vec[q2];
            let center = p_vec[q2].clone_view();
            proof {
                assert(center@ == p_vec@[q2 as int]@);
                assert(centers@.contains(center@));
            }
            let vk = vertex.clone_view();
            partition_map.insert(vk, center);
            proof {
                assert(vk@ == (*vertex)@);
                assert(vk@ == vertices_vec@[q2 as int]@);
                assert(partition_map@.contains_key(vertices_vec@[q2 as int]@));
                assert(partition_map@[vertices_vec@[q2 as int]@]@ == center@);
                assert(centers@.contains(partition_map@[vertices_vec@[q2 as int]@]@));
            }
            q2 = q2 + 1;
        }

        // Post-loop proof: properties 1 and 2.
        proof {
            // Property 1: all graph vertices are in partition_map.
            assert forall |v_view: V::V| graph@.V.contains(v_view) implies
                partition_map@.contains_key(v_view) by {
                assert(vertices_vec@.map(|_i: int, vt: V| vt@).contains(v_view));
                let k = choose|k: int| 0 <= k < nv as int && vertices_vec@[k]@ == v_view;
                assert(partition_map@.contains_key(vertices_vec@[k]@));
                assert(vertices_vec@[k]@ == v_view);
            };
            // Property 2: all partition_map values are centers.
            assert forall |v_view: V::V| partition_map@.contains_key(v_view) implies
                centers@.contains(partition_map@[v_view]@) by {
                let k = choose|k: int| 0 <= k < nv as int && vertices_vec@[k]@ == v_view;
                assert(centers@.contains(partition_map@[vertices_vec@[k]@]@));
                assert(vertices_vec@[k]@ == v_view);
            };
        }

        (centers, partition_map)
    }

    } // verus!
}
