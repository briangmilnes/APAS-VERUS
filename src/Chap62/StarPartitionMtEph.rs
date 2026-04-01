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
    use std::sync::Arc;
    use std::vec::Vec;
    use crate::{ParaPair, SetLit};
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + lg m) — DIFFERS: Loops 2, 3 parallel D&C; loops 1, 4, 5, 6 sequential
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

    /// Validity predicate for a tail-to-head edge entry.
    pub open spec fn spec_valid_th_entry<V: View>(
        entry: (usize, V),
        nv: nat,
        cf: Map<V::V, bool>,
        verts: Seq<V>,
        vi: Map<V::V, usize>,
    ) -> bool {
        let idx = entry.0;
        &&& (idx as usize) < nv
        &&& cf.contains_key(verts[idx as int]@)
        &&& !cf[verts[idx as int]@]
        &&& cf.contains_key(entry.1@)
        &&& cf[entry.1@]
        &&& vi.contains_key(entry.1@)
        &&& (vi[entry.1@] as usize) < nv
    }

    // 9. impls — parallel helpers

    /// Deterministic hash-based coin flip from (seed, index).
    /// Replaces sequential RNG with a parallelizable hash function.
    #[verifier::external_body]
    fn hash_coin(seed: u64, index: usize) -> bool {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        index.hash(&mut hasher);
        hasher.finish() % 2 == 0
    }

    /// Parallel coin flip generation using divide-and-conquer.
    ///
    /// Work O(n), Span O(lg n) — parallel hash-based coin generation via ParaPair!.
    fn hash_coin_flips_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        vertices: Arc<Vec<V>>,
        seed: u64,
        start: usize,
        end: usize,
    ) -> (flips: HashMapWithViewPlus<V, bool>)
        requires
            start <= end,
            end <= vertices@.len(),
            valid_key_type_Edge::<V>(),
            vertices@.no_duplicates(),
        ensures
            forall|j: int| start as int <= j < end as int ==>
                #[trigger] flips@.contains_key(vertices@[j]@),
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let mut coins = HashMapWithViewPlus::new();
            coins.insert(verts[start].clone_view(), hash_coin(seed, start));
            return coins;
        }

        let mid = start + size / 2;
        let ghost verts_view = vertices@;
        let v1 = vertices.clone();
        let v2 = vertices;

        let f1 = move || -> (r: HashMapWithViewPlus<V, bool>)
            requires
                start <= mid, mid <= v1@.len(),
                valid_key_type_Edge::<V>(),
                v1@.no_duplicates(),
            ensures
                forall|j: int| start as int <= j < mid as int ==>
                    #[trigger] r@.contains_key(v1@[j]@),
        {
            hash_coin_flips_mt(v1, seed, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, bool>)
            requires
                mid <= end, end <= v2@.len(),
                valid_key_type_Edge::<V>(),
                v2@.no_duplicates(),
            ensures
                forall|j: int| mid as int <= j < end as int ==>
                    #[trigger] r@.contains_key(v2@[j]@),
        {
            hash_coin_flips_mt(v2, seed, mid, end)
        };

        let Pair(mut merged, right) = crate::ParaPair!(f1, f2);

        // Merge right entries into merged using a while loop with merge_done flag
        // to capture the loop-exit fact (all entries visited).
        proof { assert(obeys_key_model::<V>()); }
        let mut it = right.iter();
        let ghost it_seq = it@.1;
        let mut merge_done = false;
        while !merge_done
            invariant
                it@.0 <= it@.1.len(),
                it_seq == it@.1,
                valid_key_type_Edge::<V>(),
                obeys_key_model::<V>(),
                merge_done ==> it@.0 >= it_seq.len(),
                // merged covers [start, mid).
                forall|j: int| start as int <= j < mid as int ==>
                    #[trigger] merged@.contains_key(verts_view[j]@),
                // merged contains all iterated entries from right.
                forall|idx: int| 0 <= idx < it@.0 ==>
                    #[trigger] merged@.contains_key(it_seq[idx].0@),
                // iter covers all right keys.
                forall|kv: V::V| #[trigger] right@.contains_key(kv) ==>
                    exists|pair: (V, bool)| #[trigger] it_seq.contains(pair) && pair.0@ == kv,
                // right covers [mid, end).
                forall|j: int| mid as int <= j < end as int ==>
                    #[trigger] right@.contains_key(verts_view[j]@),
            decreases (!merge_done) as int, it_seq.len() - it@.0,
        {
            if let Some((k, v)) = it.next() {
                let ghost pre_merged = merged@;
                merged.insert(k.clone_view(), *v);
                proof {
                    assert forall|j: int| start as int <= j < mid as int implies
                        #[trigger] merged@.contains_key(verts_view[j]@) by {
                        assert(pre_merged.contains_key(verts_view[j]@));
                    };
                    assert forall|idx: int| 0 <= idx < it@.0 implies
                        #[trigger] merged@.contains_key(it_seq[idx].0@) by {
                        if idx < it@.0 - 1 {
                            assert(pre_merged.contains_key(it_seq[idx].0@));
                        }
                    };
                }
            } else {
                merge_done = true;
            }
        }

        // Post-merge: merged covers [start, end).
        proof {
            // merge_done is true; invariant gives it@.0 >= it_seq.len().
            assert(it@.0 >= it_seq.len());
            assert forall|j: int| start as int <= j < end as int implies
                #[trigger] merged@.contains_key(verts_view[j]@) by {
                if j < mid as int {
                    // Covered by loop invariant for [start, mid).
                } else {
                    // right covers [mid, end).
                    assert(right@.contains_key(verts_view[j]@));
                    // iter ensures: exists pair in it_seq with pair.0@ == verts_view[j]@.
                    let pair = choose|pair: (V, bool)| #[trigger] it_seq.contains(pair) && pair.0@ == verts_view[j]@;
                    assert(it_seq.contains(pair));
                    let idx = it_seq.index_of(pair);
                    assert(0 <= idx < it_seq.len());
                    assert(it_seq.len() <= it@.0);
                    assert(idx < it@.0);
                    assert(merged@.contains_key(it_seq[idx].0@));
                    assert(it_seq[idx] == pair);
                    assert(pair.0@ == verts_view[j]@);
                }
            };
        }

        merged
    }

    /// Parallel edge classification using divide-and-conquer.
    ///
    /// For each edge in [start, end), classifies tail-to-head edges and collects them.
    /// Work O(m), Span O(lg m) — binary fork-join via ParaPair!.
    fn build_th_edges_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        edges: Arc<Vec<Edge<V>>>,
        coin_flips: Arc<HashMapWithViewPlus<V, bool>>,
        vertex_to_index: Arc<HashMapWithViewPlus<V, usize>>,
        vertices: Arc<Vec<V>>,
        nv: usize,
        start: usize,
        end: usize,
    ) -> (result: Vec<(usize, V)>)
        requires
            start <= end,
            end <= edges@.len(),
            valid_key_type_Edge::<V>(),
            nv == vertices@.len(),
            vertices@.no_duplicates(),
            forall|k: int| start as int <= k < end as int ==>
                #[trigger] coin_flips@.contains_key(edges@[k]@.0) &&
                coin_flips@.contains_key(edges@[k]@.1) &&
                vertex_to_index@.contains_key(edges@[k]@.0) &&
                vertex_to_index@.contains_key(edges@[k]@.1),
            forall|j: int| 0 <= j < nv as int ==>
                #[trigger] vertex_to_index@.contains_key(vertices@[j]@) &&
                vertex_to_index@[vertices@[j]@] as usize == j,
            forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                exists|j: int| 0 <= j < nv as int && #[trigger] vertices@[j]@ == v_view,
        ensures
            forall|s: int| 0 <= s < result@.len() ==>
                #[trigger] spec_valid_th_entry(result@[s], nv as nat, coin_flips@, vertices@, vertex_to_index@),
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }

        if size == 1 {
            let er = arc_deref(&edges);
            let edge = &er[start];
            let Edge(u, v) = edge;
            let cf = arc_deref(&coin_flips);
            let vi = arc_deref(&vertex_to_index);

            // Bridge: edge endpoints and Arc views.
            proof {
                assert(edges@[start as int]@.0 == u@);
                assert(edges@[start as int]@.1 == v@);
                assert(coin_flips@.contains_key(u@));
                assert(coin_flips@.contains_key(v@));
                assert(vertex_to_index@.contains_key(u@));
                assert(vertex_to_index@.contains_key(v@));
            }

            let u_heads = match cf.get(u) { Some(val) => *val, None => false };
            let v_heads = match cf.get(v) { Some(val) => *val, None => false };

            let mut result: Vec<(usize, V)> = Vec::new();
            if !u_heads && v_heads {
                if let Some(u_idx) = vi.get(u) {
                    let uid = *u_idx as usize;
                    result.push((uid, v.clone_view()));
                    proof {
                        // Use cf@/vi@ (which equal coin_flips@/vertex_to_index@ via arc_deref).
                        assert(uid == vi@[u@] as usize);
                        // Find u's position in vertices.
                        let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices@[j]@ == u@;
                        assert(vi@[vertices@[j]@] as usize == j);
                        assert(uid < nv);
                        assert(vertices@[uid as int]@ == u@);
                        // u is tails.
                        assert(!cf@[u@]);
                        // v is heads; find v's position for bound.
                        assert(vi@.contains_key(v@));
                        let j2 = choose|j2: int| 0 <= j2 < nv as int && #[trigger] vertices@[j2]@ == v@;
                        assert(vi@[vertices@[j2]@] as usize == j2);
                        assert((vi@[v@] as usize) < nv);
                        assert(spec_valid_th_entry(result@[0], nv as nat, cf@, vertices@, vi@));
                    }
                }
            } else if !v_heads && u_heads {
                if let Some(v_idx) = vi.get(v) {
                    let vid = *v_idx as usize;
                    result.push((vid, u.clone_view()));
                    proof {
                        assert(vid == vi@[v@] as usize);
                        let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices@[j]@ == v@;
                        assert(vi@[vertices@[j]@] as usize == j);
                        assert(vid < nv);
                        assert(vertices@[vid as int]@ == v@);
                        assert(!cf@[v@]);
                        assert(vi@.contains_key(u@));
                        let j2 = choose|j2: int| 0 <= j2 < nv as int && #[trigger] vertices@[j2]@ == u@;
                        assert(vi@[vertices@[j2]@] as usize == j2);
                        assert((vi@[u@] as usize) < nv);
                        assert(spec_valid_th_entry(result@[0], nv as nat, cf@, vertices@, vi@));
                    }
                }
            }
            return result;
        }

        // D&C: split edges in half, parallelize, concatenate.
        let mid = start + size / 2;
        let ghost cf_view = coin_flips@;
        let ghost vt_view = vertices@;
        let ghost vi_view = vertex_to_index@;
        let e1 = edges.clone();
        let cf1 = coin_flips.clone();
        let vi1 = vertex_to_index.clone();
        let vt1 = vertices.clone();
        let e2 = edges;
        let cf2 = coin_flips;
        let vi2 = vertex_to_index;
        let vt2 = vertices;

        let f1 = move || -> (r: Vec<(usize, V)>)
            requires
                start <= mid, mid <= e1@.len(),
                valid_key_type_Edge::<V>(),
                nv == vt1@.len(), vt1@.no_duplicates(),
                forall|k: int| start as int <= k < mid as int ==>
                    #[trigger] cf1@.contains_key(e1@[k]@.0) && cf1@.contains_key(e1@[k]@.1) &&
                    vi1@.contains_key(e1@[k]@.0) && vi1@.contains_key(e1@[k]@.1),
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] vi1@.contains_key(vt1@[j]@) && vi1@[vt1@[j]@] as usize == j,
                forall|v_view: V::V| #[trigger] vi1@.contains_key(v_view) ==>
                    exists|j: int| 0 <= j < nv as int && #[trigger] vt1@[j]@ == v_view,
            ensures
                forall|s: int| 0 <= s < r@.len() ==>
                    #[trigger] spec_valid_th_entry(r@[s], nv as nat, cf1@, vt1@, vi1@),
        {
            build_th_edges_mt(e1, cf1, vi1, vt1, nv, start, mid)
        };

        let f2 = move || -> (r: Vec<(usize, V)>)
            requires
                mid <= end, end <= e2@.len(),
                valid_key_type_Edge::<V>(),
                nv == vt2@.len(), vt2@.no_duplicates(),
                forall|k: int| mid as int <= k < end as int ==>
                    #[trigger] cf2@.contains_key(e2@[k]@.0) && cf2@.contains_key(e2@[k]@.1) &&
                    vi2@.contains_key(e2@[k]@.0) && vi2@.contains_key(e2@[k]@.1),
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] vi2@.contains_key(vt2@[j]@) && vi2@[vt2@[j]@] as usize == j,
                forall|v_view: V::V| #[trigger] vi2@.contains_key(v_view) ==>
                    exists|j: int| 0 <= j < nv as int && #[trigger] vt2@[j]@ == v_view,
            ensures
                forall|s: int| 0 <= s < r@.len() ==>
                    #[trigger] spec_valid_th_entry(r@[s], nv as nat, cf2@, vt2@, vi2@),
        {
            build_th_edges_mt(e2, cf2, vi2, vt2, nv, mid, end)
        };

        let Pair(mut result, right) = crate::ParaPair!(f1, f2);

        // Concatenate right onto result.
        let ghost left_len = result@.len();
        let mut i: usize = 0;
        while i < right.len()
            invariant
                i <= right@.len(),
                result@.len() == left_len + i as int,
                forall|s: int| 0 <= s < result@.len() ==>
                    #[trigger] spec_valid_th_entry(result@[s], nv as nat, cf_view, vt_view, vi_view),
                forall|s: int| 0 <= s < right@.len() ==>
                    #[trigger] spec_valid_th_entry(right@[s], nv as nat, cf_view, vt_view, vi_view),
            decreases right@.len() - i,
        {
            let (idx_val, ref head_v) = right[i];
            result.push((idx_val, head_v.clone_view()));
            proof {
                let ghost new_s = result@.len() - 1;
                assert(result@[new_s].0 == right@[i as int].0);
                assert(result@[new_s].1@ == right@[i as int].1@);
                assert(spec_valid_th_entry(right@[i as int], nv as nat, cf_view, vt_view, vi_view));
            }
            i = i + 1;
        }
        result
    }

    /// Algorithm 62.3: Parallel Star Partition.
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.1): Work O(n + m), Span O(lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + lg m) — DIFFERS: Loops 2, 3 use parallel D&C coin flips and edge classification; loops 1, 4, 5, 6 remain sequential (HashMap build, array copy, priority write, partition build)
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

        // Wrap vertices_vec and vertex_to_index in Arc for parallel operations.
        let vtx_to_idx_arc: Arc<HashMapWithViewPlus<V, usize>> = Arc::new(vertex_to_index);
        let vertex_to_index = arc_deref(&vtx_to_idx_arc);
        let vertices_arc: Arc<Vec<V>> = Arc::new(vertices_vec);
        let vertices_vec = arc_deref(&vertices_arc);

        // Loop 2: parallel hash-based coin flips (Work O(n), Span O(lg n)).
        let coin_flips_owned = hash_coin_flips_mt(vertices_arc.clone(), seed, 0, nv);
        let coin_flips_arc: Arc<HashMapWithViewPlus<V, bool>> = Arc::new(coin_flips_owned);
        let coin_flips = arc_deref(&coin_flips_arc);

        // Loop 3: parallel edge classification (Work O(m), Span O(lg m)).
        let edge_vec = graph.E.to_seq();
        let ne = edge_vec.len();
        // Prove edge endpoints are in coin_flips and vertex_to_index.
        proof {
            assert forall|k: int| 0 <= k < ne as int implies
                #[trigger] coin_flips@.contains_key(edge_vec@[k]@.0) &&
                coin_flips@.contains_key(edge_vec@[k]@.1) &&
                vertex_to_index@.contains_key(edge_vec@[k]@.0) &&
                vertex_to_index@.contains_key(edge_vec@[k]@.1) by {
                let ghost mapped = edge_vec@.map(|_i: int, t: Edge<V>| t@);
                assert(mapped[k] == edge_vec@[k]@);
                assert(mapped.contains(edge_vec@[k]@));
                assert(graph@.A.contains(edge_vec@[k]@));
                assert(graph@.V.contains(edge_vec@[k]@.0));
                assert(graph@.V.contains(edge_vec@[k]@.1));
                // coin_flips covers all graph vertices.
                let ghost v0 = edge_vec@[k]@.0;
                assert(vertices_vec@.map(|_i: int, t: V| t@).contains(v0));
                let j0 = vertices_vec@.map(|_i: int, t: V| t@).index_of(v0);
                assert(0 <= j0 < nv as int);
                assert(vertices_vec@[j0]@ == v0);
                let ghost v1 = edge_vec@[k]@.1;
                assert(vertices_vec@.map(|_i: int, t: V| t@).contains(v1));
                let j1 = vertices_vec@.map(|_i: int, t: V| t@).index_of(v1);
                assert(0 <= j1 < nv as int);
                assert(vertices_vec@[j1]@ == v1);
            };
        }
        let edge_arc: Arc<Vec<Edge<V>>> = Arc::new(edge_vec);
        let th_edges = build_th_edges_mt(
            edge_arc, coin_flips_arc.clone(), vtx_to_idx_arc.clone(),
            vertices_arc.clone(), nv, 0, ne,
        );
        let th_edges = th_edges;

        // Bridge: expand spec_valid_th_entry for Loop 4 carry-through.
        proof {
            assert forall|s: int| #![trigger th_edges@[s]] 0 <= s < th_edges@.len() implies
                (th_edges@[s].0 as usize) < nv &&
                coin_flips@.contains_key(vertices_vec@[(th_edges@[s].0 as usize) as int]@) &&
                !coin_flips@[vertices_vec@[(th_edges@[s].0 as usize) as int]@] &&
                coin_flips@.contains_key(th_edges@[s].1@) &&
                coin_flips@[th_edges@[s].1@] &&
                vertex_to_index@.contains_key(th_edges@[s].1@) &&
                (vertex_to_index@[th_edges@[s].1@] as usize) < nv by {
                assert(spec_valid_th_entry(th_edges@[s], nv as nat, coin_flips@, vertices_vec@, vertex_to_index@));
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
