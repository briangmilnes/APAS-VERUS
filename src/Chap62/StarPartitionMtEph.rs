//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 62: Star Partition - Multi-threaded Ephemeral Implementation
//!
//! Implements Algorithm 62.3: Parallel Star Partition using randomized coin flips.
//! Uses Seq.inject for efficient parallel updates.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod StarPartitionMtEph {


    //		Section 2. imports

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
    use crate::vstdplus::arc_rwlock::arc_rwlock::clone_arc;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_view_injective, obeys_feq_full, lemma_reveal_view_injective};
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct StarPartitionMtEph;

    pub type T<V> = UnDirGraphMtEph<V>;

    //		Section 6. spec fns


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

    //		Section 8. traits


    pub trait StarPartitionMtEphTrait {
        /// Well-formedness for parallel star partition algorithm input.
        open spec fn spec_starpartitionmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel star partition using randomized coin flips.
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        /// - Alg Analysis: APAS (Ch62 Thm 62.1): Work O(n + m), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg(n + m)), Span O(lg(n + m)) — all 6 loops parallel D&C
        fn parallel_star_partition<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_starpartitionmteph_wf(graph), valid_key_type_Edge::<V>();
    }

    //		Section 9. impls


    // Arc clone: uses generic clone_arc from vstdplus/arc_rwlock.rs.

    /// Deterministic hash-based coin flip from (seed, index).
    /// Replaces sequential RNG with a parallelizable hash function.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single hash computation.
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — D&C fork-join over n vertices; Mt parallel.
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
                // Veracity: NEEDED proof block
                proof {
                }
            } else {
                merge_done = true;
            }
        }

        // Post-merge: merged covers [start, end).
        // Veracity: NEEDED proof block
        proof {
            // merge_done is true; invariant gives it@.0 >= it_seq.len().
            // Veracity: NEEDED assert
            assert forall|j: int| start as int <= j < end as int implies
                #[trigger] merged@.contains_key(verts_view[j]@) by {
                if j < mid as int {
                    // Covered by loop invariant for [start, mid).
                } else {
                    // right covers [mid, end).
                    // Veracity: NEEDED assert
                    assert(right@.contains_key(verts_view[j]@));
                    // iter ensures: exists pair in it_seq with pair.0@ == verts_view[j]@.
                    let pair = choose|pair: (V, bool)| #[trigger] it_seq.contains(pair) && pair.0@ == verts_view[j]@;
                    let idx = it_seq.index_of(pair);
                }
            };
        }

        merged
    }

    /// Parallel edge classification using divide-and-conquer.
    ///
    /// For each edge in [start, end), classifies tail-to-head edges and collects them.
    /// Work O(m), Span O(lg m) — binary fork-join via ParaPair!.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(lg m) — D&C fork-join over m edges; Mt parallel.
    fn build_th_edges_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        edges: Arc<Vec<Edge<V>>>,
        coin_flips: Arc<HashMapWithViewPlus<V, bool>>,
        vertex_to_index: Arc<HashMapWithViewPlus<V, usize>>,
        vertices: Arc<Vec<V>>,
        nv: usize,
        start: usize,
        end: usize,
    ) -> (th_edges: Vec<(usize, V)>)
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
            forall|s: int| 0 <= s < th_edges@.len() ==>
                #[trigger] spec_valid_th_entry(th_edges@[s], nv as nat, coin_flips@, vertices@, vertex_to_index@),
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
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(edges@[start as int]@.0 == u@);
                // Veracity: NEEDED assert
                assert(coin_flips@.contains_key(u@));
            }

            let u_heads = match cf.get(u) { Some(val) => *val, None => false };
            let v_heads = match cf.get(v) { Some(val) => *val, None => false };

            let mut result: Vec<(usize, V)> = Vec::new();
            if !u_heads && v_heads {
                if let Some(u_idx) = vi.get(u) {
                    let uid = *u_idx as usize;
                    result.push((uid, v.clone_view()));
                    // Veracity: NEEDED proof block
                    proof {
                        // Use cf@/vi@ (which equal coin_flips@/vertex_to_index@ via arc_deref).
                        // Find u's position in vertices.
                        let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices@[j]@ == u@;
                        // u is tails.
                        // v is heads; find v's position for bound.
                        let j2 = choose|j2: int| 0 <= j2 < nv as int && #[trigger] vertices@[j2]@ == v@;
                    }
                }
            } else if !v_heads && u_heads {
                if let Some(v_idx) = vi.get(v) {
                    let vid = *v_idx as usize;
                    result.push((vid, u.clone_view()));
                    // Veracity: NEEDED proof block
                    proof {
                        let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices@[j]@ == v@;
                        let j2 = choose|j2: int| 0 <= j2 < nv as int && #[trigger] vertices@[j2]@ == u@;
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
            // Veracity: NEEDED proof block
            proof {
                let ghost new_s = result@.len() - 1;
                // Veracity: NEEDED assert
                assert(spec_valid_th_entry(right@[i as int], nv as nat, cf_view, vt_view, vi_view));
            }
            i = i + 1;
        }
        result
    }

    /// Parallel clone of a vertex slice into a Vec using D&C + join.
    ///
    /// Work O(n), Span O(lg n) — binary fork-join via ParaPair!.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — D&C fork-join over n vertices; Mt parallel.
    fn build_p_vec_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        vertices: Arc<Vec<V>>,
        start: usize,
        end: usize,
    ) -> (p_vec: Vec<V>)
        requires
            start <= end,
            end <= vertices@.len(),
        ensures
            p_vec@.len() == (end - start) as int,
            forall|j: int| 0 <= j < p_vec@.len() ==>
                #[trigger] p_vec@[j]@ == vertices@[(start as int + j)]@,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let mut result: Vec<V> = Vec::new();
            result.push(verts[start].clone_view());
            return result;
        }

        let mid = start + size / 2;
        let ghost verts_view = vertices@;
        let v1 = vertices.clone();
        let v2 = vertices;

        let f1 = move || -> (r: Vec<V>)
            requires
                start <= mid, mid <= v1@.len(),
            ensures
                r@.len() == (mid - start) as int,
                forall|j: int| 0 <= j < r@.len() ==>
                    #[trigger] r@[j]@ == v1@[(start as int + j)]@,
        {
            build_p_vec_mt(v1, start, mid)
        };

        let f2 = move || -> (r: Vec<V>)
            requires
                mid <= end, end <= v2@.len(),
            ensures
                r@.len() == (end - mid) as int,
                forall|j: int| 0 <= j < r@.len() ==>
                    #[trigger] r@[j]@ == v2@[(mid as int + j)]@,
        {
            build_p_vec_mt(v2, mid, end)
        };

        let Pair(mut result, right) = crate::ParaPair!(f1, f2);

        // Concatenate right onto result.
        let ghost left_len = result@.len();
        let mut i: usize = 0;
        while i < right.len()
            invariant
                i <= right@.len(),
                result@.len() == left_len + i as int,
                left_len == (mid - start) as int,
                // Left portion preserved.
                forall|j: int| 0 <= j < left_len ==>
                    #[trigger] result@[j]@ == verts_view[(start as int + j)]@,
                // Right portion appended.
                forall|j: int| 0 <= j < i as int ==>
                    #[trigger] result@[(left_len + j)]@ == verts_view[(mid as int + j)]@,
                // Source right ensures.
                forall|j: int| 0 <= j < right@.len() ==>
                    #[trigger] right@[j]@ == verts_view[(mid as int + j)]@,
            decreases right@.len() - i,
        {
            result.push(right[i].clone_view());
            i = i + 1;
        }

        // Post-merge: result covers [start, end).
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert forall|j: int| 0 <= j < result@.len() implies
                #[trigger] result@[j]@ == verts_view[(start as int + j)]@ by {
                if j < left_len {
                    // From left half — direct from invariant.
                } else {
                    // From right half: j >= left_len, so index into right portion.
                    let rj = j - left_len;
                    // Loop invariant: result@[(left_len + rj)]@ == verts_view[(mid + rj)]@.
                    // left_len + rj == j, and mid + rj == start + j.
                    // Veracity: NEEDED assert
                    assert(left_len + rj == j);
                }
            };
        }

        result
    }

    /// Parallel build of vertex-to-index map using D&C + join.
    ///
    /// Maps each vertex to its position in the vertex sequence.
    /// Work O(n lg n), Span O(lg n) — binary fork-join via ParaPair!, sequential merge.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg n) — D&C fork-join + sequential merge of hashmaps; Mt parallel.
    fn build_vertex_to_index_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        vertices: Arc<Vec<V>>,
        start: usize,
        end: usize,
    ) -> (vi: HashMapWithViewPlus<V, usize>)
        requires
            start <= end,
            end <= vertices@.len(),
            vertices@.no_duplicates(),
            valid_key_type_Edge::<V>(),
        ensures
            forall|j: int| start as int <= j < end as int ==>
                #[trigger] vi@.contains_key(vertices@[j]@) &&
                vi@[vertices@[j]@] as usize == j,
            forall|v_view: V::V| #[trigger] vi@.contains_key(v_view) ==>
                exists|j: int| start as int <= j < end as int && #[trigger] vertices@[j]@ == v_view,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let mut vi = HashMapWithViewPlus::new();
            vi.insert(verts[start].clone_view(), start);
            return vi;
        }

        let mid = start + size / 2;
        let ghost verts_view = vertices@;
        let v_merge = vertices.clone();
        let v1 = vertices.clone();
        let v2 = vertices;

        let f1 = move || -> (r: HashMapWithViewPlus<V, usize>)
            requires
                start <= mid, mid <= v1@.len(),
                v1@.no_duplicates(),
                valid_key_type_Edge::<V>(),
            ensures
                forall|j: int| start as int <= j < mid as int ==>
                    #[trigger] r@.contains_key(v1@[j]@) && r@[v1@[j]@] as usize == j,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|j: int| start as int <= j < mid as int && #[trigger] v1@[j]@ == v_view,
        {
            build_vertex_to_index_mt(v1, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, usize>)
            requires
                mid <= end, end <= v2@.len(),
                v2@.no_duplicates(),
                valid_key_type_Edge::<V>(),
            ensures
                forall|j: int| mid as int <= j < end as int ==>
                    #[trigger] r@.contains_key(v2@[j]@) && r@[v2@[j]@] as usize == j,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|j: int| mid as int <= j < end as int && #[trigger] v2@[j]@ == v_view,
        {
            build_vertex_to_index_mt(v2, mid, end)
        };

        let Pair(mut merged, _right) = crate::ParaPair!(f1, f2);

        // Merge: insert right half entries directly from vertex Arc.
        let verts = arc_deref(&v_merge);
        let ghost mut dom_witnesses: Map<V::V, int> = Map::empty();
        // Veracity: NEEDED proof block
        proof {
            dom_witnesses = Map::new(
                |v_view: V::V| merged@.contains_key(v_view),
                |v_view: V::V| {
                    let j2 = choose|j2: int| start as int <= j2 < mid as int && #[trigger] verts_view[j2]@ == v_view;
                    j2
                },
            );
        }
        let mut j: usize = mid;
        while j < end
            invariant
                valid_key_type_Edge::<V>(),
                start <= mid,
                mid <= j <= end,
                end <= verts_view.len(),
                verts_view == verts@,
                verts_view.no_duplicates(),
                // merged covers [start, mid) with correct values.
                forall|j2: int| start as int <= j2 < mid as int ==>
                    #[trigger] merged@.contains_key(verts_view[j2]@) &&
                    merged@[verts_view[j2]@] as usize == j2,
                // merged covers [mid, j) with correct values.
                forall|j2: int| mid as int <= j2 < j as int ==>
                    #[trigger] merged@.contains_key(verts_view[j2]@) &&
                    merged@[verts_view[j2]@] as usize == j2,
                // Ghost domain tracking: every key in merged@ has a witness in [start, j).
                forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) ==>
                    dom_witnesses.contains_key(v_view) &&
                    start as int <= dom_witnesses[v_view] &&
                    dom_witnesses[v_view] < j as int &&
                    verts_view[dom_witnesses[v_view]]@ == v_view,
            decreases end - j,
        {
            let ghost jv = verts_view[j as int]@;
            let ghost pre_merged = merged@;
            let ghost pre_dom = dom_witnesses;
            merged.insert(verts[j].clone_view(), j);
            // Veracity: NEEDED proof block
            proof {
                lemma_reveal_view_injective::<V>();
                dom_witnesses = pre_dom.insert(jv, j as int);
                // Prior left entries preserved.
                // Veracity: NEEDED assert
                assert forall|j2: int| start as int <= j2 < mid as int implies
                    #[trigger] merged@.contains_key(verts_view[j2]@) &&
                    merged@[verts_view[j2]@] as usize == j2 by {
                    let ghost j2v = verts_view[j2]@;
                    if j2v != jv {
                    } else {
                        // Veracity: NEEDED assert
                        assert(false);
                    }
                };
                // Prior right entries [mid, j) preserved + new entry j.
                // Veracity: NEEDED assert
                assert forall|j2: int| mid as int <= j2 < j as int + 1 implies
                    #[trigger] merged@.contains_key(verts_view[j2]@) &&
                    merged@[verts_view[j2]@] as usize == j2 by {
                    if j2 == j as int {
                    } else {
                        let ghost j2v = verts_view[j2]@;
                        if j2v != jv {
                            // Veracity: NEEDED assert
                            assert(pre_merged.contains_key(j2v));
                        } else {
                        }
                    }
                };
                // Domain witnesses updated.
                // Veracity: NEEDED assert
                assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                    dom_witnesses.contains_key(v_view) &&
                    start as int <= dom_witnesses[v_view] &&
                    dom_witnesses[v_view] < j as int + 1 &&
                    verts_view[dom_witnesses[v_view]]@ == v_view by {
                    if v_view == jv {
                    } else {
                    }
                };
            }
            j = j + 1;
        }

        // Post-merge: domain bound follows from ghost witnesses.
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                exists|j2: int| start as int <= j2 < end as int && #[trigger] verts_view[j2]@ == v_view by {
                let w = dom_witnesses[v_view];
            };
        }

        merged
    }

    /// Parallel build of satellite-to-center map from th_edges using D&C + join.
    ///
    /// For each (idx, head) in th_edges[start..end], inserts (vertices[idx], head) into map.
    /// Deduplicates naturally (HashMap last-write-wins; any center is acceptable per APAS).
    /// Work O(m lg m), Span O(lg m) — binary fork-join via ParaPair!, sequential merge.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg m), Span O(lg m) — D&C fork-join + sequential merge of hashmaps; Mt parallel.
    fn build_satellite_map_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        th_edges: Arc<Vec<(usize, V)>>,
        vertices: Arc<Vec<V>>,
        nv: usize,
        coin_flips: Arc<HashMapWithViewPlus<V, bool>>,
        vertex_to_index: Arc<HashMapWithViewPlus<V, usize>>,
        start: usize,
        end: usize,
    ) -> (sm: HashMapWithViewPlus<V, V>)
        requires
            start <= end,
            end <= th_edges@.len(),
            nv == vertices@.len(),
            vertices@.no_duplicates(),
            valid_key_type_Edge::<V>(),
            forall|s: int| 0 <= s < th_edges@.len() ==>
                #[trigger] spec_valid_th_entry(th_edges@[s], nv as nat, coin_flips@, vertices@, vertex_to_index@),
        ensures
            // Every key in sm came from a th_edge in [start, end).
            forall|v_view: V::V| #[trigger] sm@.contains_key(v_view) ==>
                exists|s: int| start as int <= s < end as int &&
                    #[trigger] vertices@[th_edges@[s].0 as int]@ == v_view,
            // Values are heads vertices with valid vertex_to_index entries.
            forall|v_view: V::V| #[trigger] sm@.contains_key(v_view) ==>
                coin_flips@.contains_key(sm@[v_view]@) &&
                coin_flips@[sm@[v_view]@] &&
                vertex_to_index@.contains_key(sm@[v_view]@) &&
                (vertex_to_index@[sm@[v_view]@] as usize) < nv,
            // Keys are tails vertices (not heads).
            forall|v_view: V::V| #[trigger] sm@.contains_key(v_view) ==>
                coin_flips@.contains_key(v_view) &&
                !coin_flips@[v_view],
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }
        if size == 1 {
            let the = arc_deref(&th_edges);
            let verts = arc_deref(&vertices);
            let (idx, ref head_v) = the[start];
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(spec_valid_th_entry(th_edges@[start as int], nv as nat, coin_flips@, vertices@, vertex_to_index@));
            }
            if (idx as usize) < nv {
                let mut sm = HashMapWithViewPlus::new();
                sm.insert(verts[idx].clone_view(), head_v.clone_view());
                // Veracity: NEEDED proof block
                proof {
                    let ghost tail_v = vertices@[idx as int]@;
                    let ghost head_view = head_v@;
                }
                return sm;
            } else {
                return HashMapWithViewPlus::new();
            }
        }

        let mid = start + size / 2;
        let ghost the_view = th_edges@;
        let ghost verts_view = vertices@;
        let the_merge = th_edges.clone();
        let verts_merge = vertices.clone();
        let the1 = th_edges.clone();
        let the2 = th_edges;
        let v1 = vertices.clone();
        let v2 = vertices;
        let cf1 = coin_flips.clone();
        let cf2 = coin_flips.clone();
        let vi1 = vertex_to_index.clone();
        let vi2 = vertex_to_index.clone();

        let f1 = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                start <= mid, mid <= the1@.len(),
                nv == v1@.len(), v1@.no_duplicates(),
                valid_key_type_Edge::<V>(),
                forall|s: int| 0 <= s < the1@.len() ==>
                    #[trigger] spec_valid_th_entry(the1@[s], nv as nat, cf1@, v1@, vi1@),
            ensures
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|s: int| start as int <= s < mid as int &&
                        #[trigger] v1@[the1@[s].0 as int]@ == v_view,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    cf1@.contains_key(r@[v_view]@) && cf1@[r@[v_view]@] &&
                    vi1@.contains_key(r@[v_view]@) && (vi1@[r@[v_view]@] as usize) < nv,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    cf1@.contains_key(v_view) && !cf1@[v_view],
        {
            build_satellite_map_mt(the1, v1, nv, cf1, vi1, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                mid <= end, end <= the2@.len(),
                nv == v2@.len(), v2@.no_duplicates(),
                valid_key_type_Edge::<V>(),
                forall|s: int| 0 <= s < the2@.len() ==>
                    #[trigger] spec_valid_th_entry(the2@[s], nv as nat, cf2@, v2@, vi2@),
            ensures
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|s: int| mid as int <= s < end as int &&
                        #[trigger] v2@[the2@[s].0 as int]@ == v_view,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    cf2@.contains_key(r@[v_view]@) && cf2@[r@[v_view]@] &&
                    vi2@.contains_key(r@[v_view]@) && (vi2@[r@[v_view]@] as usize) < nv,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    cf2@.contains_key(v_view) && !cf2@[v_view],
        {
            build_satellite_map_mt(the2, v2, nv, cf2, vi2, mid, end)
        };

        let Pair(mut merged, _right) = crate::ParaPair!(f1, f2);

        // Merge: re-insert right half entries from th_edges/vertices Arcs.
        let the = arc_deref(&the_merge);
        let verts = arc_deref(&verts_merge);
        let cf = arc_deref(&coin_flips);
        let vi = arc_deref(&vertex_to_index);
        let mut t: usize = mid;
        while t < end
            invariant
                valid_key_type_Edge::<V>(),
                start <= mid,
                mid <= t <= end,
                end <= the_view.len(),
                nv == verts_view.len(),
                verts_view == verts@,
                the_view == the@,
                verts_view.no_duplicates(),
                // Keys from left half and merged right entries.
                forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) ==>
                    exists|s: int| start as int <= s < t as int &&
                        #[trigger] verts_view[the_view[s].0 as int]@ == v_view,
                // Values are heads with valid vi entries.
                forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) ==>
                    cf@.contains_key(merged@[v_view]@) && cf@[merged@[v_view]@] &&
                    vi@.contains_key(merged@[v_view]@) && (vi@[merged@[v_view]@] as usize) < nv,
                // Keys are tails.
                forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) ==>
                    cf@.contains_key(v_view) && !cf@[v_view],
                // th_edges invariant.
                forall|s: int| 0 <= s < the_view.len() ==>
                    #[trigger] spec_valid_th_entry(the_view[s], nv as nat, cf@, verts_view, vi@),
            decreases end - t,
        {
            let (idx, ref head_v) = the[t];
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(spec_valid_th_entry(the_view[t as int], nv as nat, cf@, verts_view, vi@));
            }
            if (idx as usize) < nv {
                let ghost pre_merged = merged@;
                let ghost tail_v = verts_view[idx as int]@;
                let ghost head_view = head_v@;
                // Bridge: connect destructured idx to the_view[t].0.
                merged.insert(verts[idx].clone_view(), head_v.clone_view());
                // Veracity: NEEDED proof block
                proof {
                    // New entry properties.
                    // Bridge: witness for t.
                    // Key source witness for new entry.
                    // Veracity: NEEDED assert
                    assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                        exists|s: int| start as int <= s < t as int + 1 &&
                            #[trigger] verts_view[the_view[s].0 as int]@ == v_view by {
                        if v_view == tail_v {
                            // Witness: s = t.
                        } else {
                            // Old key; old invariant provides witness in [start, t).
                            let s2 = choose|s2: int| start as int <= s2 < t as int &&
                                #[trigger] verts_view[the_view[s2].0 as int]@ == v_view;
                        }
                    };
                    // Values are heads.
                    // Veracity: NEEDED assert
                    assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                        cf@.contains_key(merged@[v_view]@) && cf@[merged@[v_view]@] &&
                        vi@.contains_key(merged@[v_view]@) && (vi@[merged@[v_view]@] as usize) < nv by {
                        if v_view == tail_v {
                        } else {
                        }
                    };
                    // Keys are tails.
                    // Veracity: NEEDED assert
                    assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                        cf@.contains_key(v_view) && !cf@[v_view] by {
                        if v_view == tail_v {
                        } else {
                        }
                    };
                }
            }
            t = t + 1;
        }

        merged
    }

    /// Parallel build of p_vec with satellite injection using D&C + join.
    ///
    /// For each vertex j in [start, end): if satellite_map contains vertices[j], use
    /// satellite_map[vertices[j]] (the center); otherwise use vertices[j] (self-pointing).
    /// Replaces sequential Loop 4 (clone) + Loop 5 (inject) with a single parallel pass.
    /// Work O(n), Span O(lg n) — binary fork-join via ParaPair!.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — D&C fork-join over n vertices; Mt parallel.
    fn build_p_vec_with_inject_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        vertices: Arc<Vec<V>>,
        satellite_map: Arc<HashMapWithViewPlus<V, V>>,
        coin_flips: Arc<HashMapWithViewPlus<V, bool>>,
        vertex_to_index: Arc<HashMapWithViewPlus<V, usize>>,
        nv: usize,
        start: usize,
        end: usize,
    ) -> (p_vec: Vec<V>)
        requires
            start <= end,
            end <= vertices@.len(),
            nv == vertices@.len(),
            vertices@.no_duplicates(),
            valid_key_type_Edge::<V>(),
            // coin_flips covers all vertices.
            forall|j: int| 0 <= j < nv as int ==>
                #[trigger] coin_flips@.contains_key(vertices@[j]@),
            // satellite_map values are heads with valid vi entries.
            forall|v_view: V::V| #[trigger] satellite_map@.contains_key(v_view) ==>
                coin_flips@.contains_key(satellite_map@[v_view]@) &&
                coin_flips@[satellite_map@[v_view]@] &&
                vertex_to_index@.contains_key(satellite_map@[v_view]@) &&
                (vertex_to_index@[satellite_map@[v_view]@] as usize) < nv,
            // satellite_map keys are tails.
            forall|v_view: V::V| #[trigger] satellite_map@.contains_key(v_view) ==>
                coin_flips@.contains_key(v_view) && !coin_flips@[v_view],
            // vertex_to_index maps all vertices.
            forall|j: int| 0 <= j < nv as int ==>
                #[trigger] vertex_to_index@.contains_key(vertices@[j]@) &&
                vertex_to_index@[vertices@[j]@] as usize == j,
            forall|v_view: V::V| #[trigger] vertex_to_index@.contains_key(v_view) ==>
                exists|j: int| 0 <= j < nv as int && #[trigger] vertices@[j]@ == v_view,
        ensures
            p_vec@.len() == (end - start) as int,
            // Heads preserve: heads vertices keep themselves.
            forall|j: int| 0 <= j < p_vec@.len() ==>
                coin_flips@.contains_key(vertices@[(start as int + j)]@) &&
                (coin_flips@[vertices@[(start as int + j)]@] ==>
                 #[trigger] p_vec@[j]@ == vertices@[(start as int + j)]@),
            // Modified entries point to heads.
            forall|j: int| 0 <= j < p_vec@.len() ==>
                p_vec@[j]@ != vertices@[(start as int + j)]@ ==>
                (coin_flips@.contains_key(#[trigger] p_vec@[j]@) &&
                 coin_flips@[p_vec@[j]@]),
            // All entries in vertex_to_index.
            forall|j: int| 0 <= j < p_vec@.len() ==>
                vertex_to_index@.contains_key(#[trigger] p_vec@[j]@) &&
                (vertex_to_index@[p_vec@[j]@] as usize) < nv,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let sm = arc_deref(&satellite_map);
            let mut result: Vec<V> = Vec::new();
            match sm.get(&verts[start]) {
                Some(center) => {
                    result.push(center.clone_view());
                    // Veracity: NEEDED proof block
                    proof {
                        let ghost sv = vertices@[start as int]@;
                        // Heads preserve: satellite_map key is tails, so coin_flips[sv] is false => vacuous.
                        // Modified entry points to heads.
                        // In vertex_to_index.
                    }
                },
                None => {
                    result.push(verts[start].clone_view());
                    // Veracity: NEEDED proof block
                    proof {
                        let ghost sv = vertices@[start as int]@;
                        // Not modified, so modified-points-to-heads is vacuous.
                        // In vertex_to_index.
                        // Veracity: NEEDED assert
                        assert(vertex_to_index@.contains_key(sv));
                    }
                },
            }
            return result;
        }

        let mid = start + size / 2;
        let ghost verts_view = vertices@;
        let v1 = vertices.clone();
        let v2 = vertices;
        let sm1 = satellite_map.clone();
        let sm2 = satellite_map;
        let cf1 = coin_flips.clone();
        let cf2 = coin_flips;
        let vi1 = vertex_to_index.clone();
        let vi2 = vertex_to_index;

        let f1 = move || -> (r: Vec<V>)
            requires
                start <= mid, mid <= v1@.len(),
                nv == v1@.len(), v1@.no_duplicates(),
                valid_key_type_Edge::<V>(),
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] cf1@.contains_key(v1@[j]@),
                forall|v_view: V::V| #[trigger] sm1@.contains_key(v_view) ==>
                    cf1@.contains_key(sm1@[v_view]@) && cf1@[sm1@[v_view]@] &&
                    vi1@.contains_key(sm1@[v_view]@) && (vi1@[sm1@[v_view]@] as usize) < nv,
                forall|v_view: V::V| #[trigger] sm1@.contains_key(v_view) ==>
                    cf1@.contains_key(v_view) && !cf1@[v_view],
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] vi1@.contains_key(v1@[j]@) && vi1@[v1@[j]@] as usize == j,
                forall|v_view: V::V| #[trigger] vi1@.contains_key(v_view) ==>
                    exists|j: int| 0 <= j < nv as int && #[trigger] v1@[j]@ == v_view,
            ensures
                r@.len() == (mid - start) as int,
                forall|j: int| 0 <= j < r@.len() ==>
                    cf1@.contains_key(v1@[(start as int + j)]@) &&
                    (cf1@[v1@[(start as int + j)]@] ==>
                     #[trigger] r@[j]@ == v1@[(start as int + j)]@),
                forall|j: int| 0 <= j < r@.len() ==>
                    r@[j]@ != v1@[(start as int + j)]@ ==>
                    (cf1@.contains_key(#[trigger] r@[j]@) && cf1@[r@[j]@]),
                forall|j: int| 0 <= j < r@.len() ==>
                    vi1@.contains_key(#[trigger] r@[j]@) && (vi1@[r@[j]@] as usize) < nv,
        {
            build_p_vec_with_inject_mt(v1, sm1, cf1, vi1, nv, start, mid)
        };

        let f2 = move || -> (r: Vec<V>)
            requires
                mid <= end, end <= v2@.len(),
                nv == v2@.len(), v2@.no_duplicates(),
                valid_key_type_Edge::<V>(),
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] cf2@.contains_key(v2@[j]@),
                forall|v_view: V::V| #[trigger] sm2@.contains_key(v_view) ==>
                    cf2@.contains_key(sm2@[v_view]@) && cf2@[sm2@[v_view]@] &&
                    vi2@.contains_key(sm2@[v_view]@) && (vi2@[sm2@[v_view]@] as usize) < nv,
                forall|v_view: V::V| #[trigger] sm2@.contains_key(v_view) ==>
                    cf2@.contains_key(v_view) && !cf2@[v_view],
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] vi2@.contains_key(v2@[j]@) && vi2@[v2@[j]@] as usize == j,
                forall|v_view: V::V| #[trigger] vi2@.contains_key(v_view) ==>
                    exists|j: int| 0 <= j < nv as int && #[trigger] v2@[j]@ == v_view,
            ensures
                r@.len() == (end - mid) as int,
                forall|j: int| 0 <= j < r@.len() ==>
                    cf2@.contains_key(v2@[(mid as int + j)]@) &&
                    (cf2@[v2@[(mid as int + j)]@] ==>
                     #[trigger] r@[j]@ == v2@[(mid as int + j)]@),
                forall|j: int| 0 <= j < r@.len() ==>
                    r@[j]@ != v2@[(mid as int + j)]@ ==>
                    (cf2@.contains_key(#[trigger] r@[j]@) && cf2@[r@[j]@]),
                forall|j: int| 0 <= j < r@.len() ==>
                    vi2@.contains_key(#[trigger] r@[j]@) && (vi2@[r@[j]@] as usize) < nv,
        {
            build_p_vec_with_inject_mt(v2, sm2, cf2, vi2, nv, mid, end)
        };

        let Pair(mut result, right) = crate::ParaPair!(f1, f2);

        // Save left-half ghost snapshot before concatenation.
        let ghost left_snap = result@;
        let ghost left_len = result@.len();

        // Concatenate right onto result, tracking only element values.
        let mut i: usize = 0;
        while i < right.len()
            invariant
                i <= right@.len(),
                result@.len() == left_len + i as int,
                left_len == (mid - start) as int,
                // Left portion: elements unchanged from snapshot.
                forall|j: int| 0 <= j < left_len ==>
                    #[trigger] result@[j] == left_snap[j],
                // Appended right elements match right source.
                forall|j: int| 0 <= j < i as int ==>
                    #[trigger] result@[(left_len + j)]@ == right@[j]@,
                // Source right properties (immutable).
                forall|j: int| 0 <= j < right@.len() ==>
                    coin_flips@.contains_key(verts_view[(mid as int + j)]@) &&
                    (coin_flips@[verts_view[(mid as int + j)]@] ==>
                     #[trigger] right@[j]@ == verts_view[(mid as int + j)]@),
                forall|j: int| 0 <= j < right@.len() ==>
                    right@[j]@ != verts_view[(mid as int + j)]@ ==>
                    (coin_flips@.contains_key(#[trigger] right@[j]@) && coin_flips@[right@[j]@]),
                forall|j: int| 0 <= j < right@.len() ==>
                    vertex_to_index@.contains_key(#[trigger] right@[j]@) &&
                    (vertex_to_index@[right@[j]@] as usize) < nv,
                right@.len() == (end - mid) as int,
            decreases right@.len() - i,
        {
            result.push(right[i].clone_view());
            i = i + 1;
        }

        // Post-merge: derive complex properties from element values.
        // Veracity: NEEDED proof block
        proof {

            // Left half: elements match snapshot, which came from f1's result.
            // f1's ensures give us heads-preserve, modified-heads, vi properties for left_snap.
            // Since result@[j] == left_snap[j] for j < left_len, these transfer.

            // Heads preserve.
            // Veracity: NEEDED assert
            assert forall|j: int| 0 <= j < result@.len() implies
                coin_flips@.contains_key(verts_view[(start as int + j)]@) &&
                (coin_flips@[verts_view[(start as int + j)]@] ==>
                 #[trigger] result@[j]@ == verts_view[(start as int + j)]@) by {
                if j < left_len {
                    // Left half: result@[j] == left_snap[j].
                    // left_snap has the heads-preserve property from f1.
                } else {
                    // Right half.
                    let rj = j - left_len;
                    // Veracity: NEEDED assert
                    assert(result@[(left_len + rj)]@ == right@[rj]@);
                    // right has heads-preserve from f2.
                }
            };
            // Modified entries point to heads.
            // Veracity: NEEDED assert
            assert forall|j: int| 0 <= j < result@.len() &&
                result@[j]@ != verts_view[(start as int + j)]@ implies
                coin_flips@.contains_key(#[trigger] result@[j]@) && coin_flips@[result@[j]@] by {
                if j < left_len {
                } else {
                    let rj = j - left_len;
                    // Veracity: NEEDED assert
                    assert(result@[(left_len + rj)]@ == right@[rj]@);
                }
            };
            // All entries in vertex_to_index.
            // Veracity: NEEDED assert
            assert forall|j: int| 0 <= j < result@.len() implies
                vertex_to_index@.contains_key(#[trigger] result@[j]@) &&
                (vertex_to_index@[result@[j]@] as usize) < nv by {
                if j < left_len {
                } else {
                    let rj = j - left_len;
                    // Veracity: NEEDED assert
                    assert(result@[(left_len + rj)]@ == right@[rj]@);
                }
            };
        }

        result
    }

    /// Parallel build of partition_map (vertex → center) using D&C + join.
    ///
    /// Work O(n lg n), Span O(lg n) — binary fork-join via ParaPair!, sequential merge.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg n) — D&C fork-join + sequential merge of hashmaps; Mt parallel.
    fn build_partition_map_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        vertices: Arc<Vec<V>>,
        p_vec: Arc<Vec<V>>,
        start: usize,
        end: usize,
    ) -> (pm: HashMapWithViewPlus<V, V>)
        requires
            start <= end,
            end <= vertices@.len(),
            p_vec@.len() == vertices@.len(),
            vertices@.no_duplicates(),
            valid_key_type_Edge::<V>(),
        ensures
            forall|j: int| start as int <= j < end as int ==>
                #[trigger] pm@.contains_key(vertices@[j]@),
            forall|j: int| start as int <= j < end as int ==>
                pm@.contains_key(vertices@[j]@) ==>
                pm@[vertices@[j]@]@ == #[trigger] p_vec@[j]@,
            forall|v_view: V::V| #[trigger] pm@.contains_key(v_view) ==>
                exists|j: int| start as int <= j < end as int && #[trigger] vertices@[j]@ == v_view,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let pv = arc_deref(&p_vec);
            let mut pm = HashMapWithViewPlus::new();
            pm.insert(verts[start].clone_view(), pv[start].clone_view());
            return pm;
        }

        let mid = start + size / 2;
        let ghost verts_view = vertices@;
        let ghost pv_view = p_vec@;
        // Keep Arc clones for the merge step after ParaPair!.
        let v_merge = vertices.clone();
        let p_merge = p_vec.clone();
        let v1 = vertices.clone();
        let v2 = vertices;
        let p1 = p_vec.clone();
        let p2 = p_vec;

        let f1 = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                start <= mid, mid <= v1@.len(),
                p1@.len() == v1@.len(),
                v1@.no_duplicates(),
                valid_key_type_Edge::<V>(),
            ensures
                forall|j: int| start as int <= j < mid as int ==>
                    #[trigger] r@.contains_key(v1@[j]@),
                forall|j: int| start as int <= j < mid as int ==>
                    r@.contains_key(v1@[j]@) ==> r@[v1@[j]@]@ == #[trigger] p1@[j]@,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|j: int| start as int <= j < mid as int && #[trigger] v1@[j]@ == v_view,
        {
            build_partition_map_mt(v1, p1, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                mid <= end, end <= v2@.len(),
                p2@.len() == v2@.len(),
                v2@.no_duplicates(),
                valid_key_type_Edge::<V>(),
            ensures
                forall|j: int| mid as int <= j < end as int ==>
                    #[trigger] r@.contains_key(v2@[j]@),
                forall|j: int| mid as int <= j < end as int ==>
                    r@.contains_key(v2@[j]@) ==> r@[v2@[j]@]@ == #[trigger] p2@[j]@,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|j: int| mid as int <= j < end as int && #[trigger] v2@[j]@ == v_view,
        {
            build_partition_map_mt(v2, p2, mid, end)
        };

        let Pair(mut merged, _right) = crate::ParaPair!(f1, f2);

        // Merge: insert right half entries directly from vertex/p_vec Arcs.
        // This avoids HashMap iterator complexity while preserving value correspondence.
        let verts = arc_deref(&v_merge);
        let pv = arc_deref(&p_merge);
        // Ghost domain set: tracks exactly which V::V keys are in merged@.
        let ghost mut dom_witnesses: Map<V::V, int> = Map::empty();
        // Initialize with left D&C domain witnesses.
        // Veracity: NEEDED proof block
        proof {
            dom_witnesses = Map::new(
                |v_view: V::V| merged@.contains_key(v_view),
                |v_view: V::V| {
                    let j2 = choose|j2: int| start as int <= j2 < mid as int && #[trigger] verts_view[j2]@ == v_view;
                    j2
                },
            );
        }
        let mut j: usize = mid;
        while j < end
            invariant
                valid_key_type_Edge::<V>(),
                start <= mid,
                mid <= j <= end,
                end <= verts_view.len(),
                verts_view == verts@,
                pv_view == pv@,
                pv_view.len() == verts_view.len(),
                verts_view.no_duplicates(),
                // merged covers [start, mid) — contains_key.
                forall|j2: int| start as int <= j2 < mid as int ==>
                    #[trigger] merged@.contains_key(verts_view[j2]@),
                // merged covers [start, mid) — correct values.
                forall|j2: int| start as int <= j2 < mid as int ==>
                    merged@.contains_key(verts_view[j2]@) ==>
                    merged@[verts_view[j2]@]@ == #[trigger] pv_view[j2]@,
                // merged covers [mid, j) — contains_key.
                forall|j2: int| mid as int <= j2 < j as int ==>
                    #[trigger] merged@.contains_key(verts_view[j2]@),
                // merged covers [mid, j) — correct values.
                forall|j2: int| mid as int <= j2 < j as int ==>
                    merged@.contains_key(verts_view[j2]@) ==>
                    merged@[verts_view[j2]@]@ == #[trigger] pv_view[j2]@,
                // Ghost domain tracking: every key in merged@ has a witness in [start, j).
                forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) ==>
                    dom_witnesses.contains_key(v_view) &&
                    start as int <= dom_witnesses[v_view] &&
                    dom_witnesses[v_view] < j as int &&
                    verts_view[dom_witnesses[v_view]]@ == v_view,
            decreases end - j,
        {
            let ghost jv = verts_view[j as int]@;
            let ghost pre_merged = merged@;
            let ghost pre_dom = dom_witnesses;
            merged.insert(verts[j].clone_view(), pv[j].clone_view());
            // Veracity: NEEDED proof block
            proof {
                lemma_reveal_view_injective::<V>();
                // New entry at j.
                // Update domain witnesses: add jv -> j.
                dom_witnesses = pre_dom.insert(jv, j as int);
                // Prior left entries preserved — contains_key.
                // Prior left entries preserved — correct values.
                // Veracity: NEEDED assert
                assert forall|j2: int| start as int <= j2 < mid as int &&
                    merged@.contains_key(verts_view[j2]@) implies
                    merged@[verts_view[j2]@]@ == #[trigger] pv_view[j2]@ by {
                    let ghost j2v = verts_view[j2]@;
                    if j2v != jv {
                    } else {
                    }
                };
                // Prior right entries [mid, j) preserved + new entry j — contains_key.
                // Prior right entries [mid, j) preserved + new entry j — correct values.
                // Veracity: NEEDED assert
                assert forall|j2: int| mid as int <= j2 < j as int + 1 &&
                    merged@.contains_key(verts_view[j2]@) implies
                    merged@[verts_view[j2]@]@ == #[trigger] pv_view[j2]@ by {
                    if j2 == j as int {
                    } else {
                        let ghost j2v = verts_view[j2]@;
                        if j2v != jv {
                        } else {
                        }
                    }
                };
                // Domain witnesses updated.
                // Veracity: NEEDED assert
                assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                    dom_witnesses.contains_key(v_view) &&
                    start as int <= dom_witnesses[v_view] &&
                    dom_witnesses[v_view] < j as int + 1 &&
                    verts_view[dom_witnesses[v_view]]@ == v_view by {
                    if v_view == jv {
                        // New key: dom_witnesses[jv] == j.
                    } else {
                        // Old key: map insert preserves other entries.
                    }
                };
            }
            j = j + 1;
        }

        // Post-merge: domain bound follows from ghost witnesses.
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert forall|v_view: V::V| #[trigger] merged@.contains_key(v_view) implies
                exists|j2: int| start as int <= j2 < end as int && #[trigger] verts_view[j2]@ == v_view by {
                let w = dom_witnesses[v_view];
            };
        }

        merged
    }

    /// Parallel build of centers set using D&C + join.
    ///
    /// A vertex is a center if p_vec[j] == vertices[j] (self-pointing).
    /// Work O(n), Span O(lg n) — binary fork-join via ParaPair!.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — D&C fork-join filtering self-pointing vertices; Mt parallel.
    fn build_centers_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        vertices: Arc<Vec<V>>,
        p_vec: Arc<Vec<V>>,
        start: usize,
        end: usize,
    ) -> (centers: SetStEph<V>)
        requires
            start <= end,
            end <= vertices@.len(),
            p_vec@.len() == vertices@.len(),
            valid_key_type_Edge::<V>(),
        ensures
            centers.spec_setsteph_wf(),
            forall|j: int| start as int <= j < end as int ==>
                p_vec@[j]@ == vertices@[j]@ ==>
                #[trigger] centers@.contains(p_vec@[j]@),
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return SetLit![];
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let pv = arc_deref(&p_vec);
            if feq(&verts[start], &pv[start]) {
                let mut s: SetStEph<V> = SetLit![];
                let _ = s.insert(verts[start].clone_view());
                return s;
            } else {
                return SetLit![];
            }
        }

        let mid = start + size / 2;
        let ghost verts_view = vertices@;
        let ghost pv_view = p_vec@;
        let v1 = vertices.clone();
        let v2 = vertices;
        let p1 = p_vec.clone();
        let p2 = p_vec;

        let f1 = move || -> (r: SetStEph<V>)
            requires
                start <= mid, mid <= v1@.len(),
                p1@.len() == v1@.len(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
                forall|j: int| start as int <= j < mid as int ==>
                    p1@[j]@ == v1@[j]@ ==>
                    #[trigger] r@.contains(p1@[j]@),
        {
            build_centers_mt(v1, p1, start, mid)
        };

        let f2 = move || -> (r: SetStEph<V>)
            requires
                mid <= end, end <= v2@.len(),
                p2@.len() == v2@.len(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
                forall|j: int| mid as int <= j < end as int ==>
                    p2@[j]@ == v2@[j]@ ==>
                    #[trigger] r@.contains(p2@[j]@),
        {
            build_centers_mt(v2, p2, mid, end)
        };

        let Pair(left, right) = crate::ParaPair!(f1, f2);
        let result = left.union(&right);

        // Post-union: result covers [start, end).
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert forall|j: int| (start as int <= j < end as int && pv_view[j]@ == verts_view[j]@) implies
                 #[trigger] result@.contains(pv_view[j]@) by {
                if j < mid as int {
                    if pv_view[j]@ == verts_view[j]@ {
                    }
                } else {
                    if pv_view[j]@ == verts_view[j]@ {
                    }
                }
            };
        }

        result
    }

    /// Algorithm 62.3: Parallel Star Partition.
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.1): Work O(n + m), Span O(lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg(n + m)), Span O(lg(n + m)) — all 6 loops parallel D&C
    #[verifier::rlimit(20)]
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
        // Veracity: NEEDED proof block
        proof {
            // Establish spec_setsteph_wf for graph.V and graph.E so we can call to_seq().
            // valid_key_type_Edge::<V>() ==> valid_key_type::<V>() and valid_key_type::<Edge<V>>().
            // Veracity: NEEDED assert
            assert(graph.V.spec_setsteph_wf());
            // Veracity: NEEDED assert
            assert(graph.E.spec_setsteph_wf());
        }
        let vertices_vec = graph.V.to_seq();
        let nv = vertices_vec.len();

        // Wrap vertices_vec in Arc for parallel operations.
        let vertices_arc: Arc<Vec<V>> = Arc::new(vertices_vec);
        let vertices_vec = arc_deref(&vertices_arc);

        // Loop 1: parallel build of vertex-to-index map (Work O(n lg n), Span O(lg n)).
        let vertex_to_index = build_vertex_to_index_mt(clone_arc(&vertices_arc), 0, nv);

        // Ghost facts after loop 1: vertex_to_index covers all graph@.V.
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert forall|v_view: V::V| #[trigger] graph@.V.contains(v_view) implies
                vertex_to_index@.contains_key(v_view) by {
                let k = vertices_vec@.map(|_i: int, t: V| t@).index_of(v_view);
            };
        }

        // Wrap vertex_to_index in Arc for parallel operations.
        let vtx_to_idx_arc: Arc<HashMapWithViewPlus<V, usize>> = Arc::new(vertex_to_index);
        let vertex_to_index = arc_deref(&vtx_to_idx_arc);

        // Loop 2: parallel hash-based coin flips (Work O(n), Span O(lg n)).
        let coin_flips_owned = hash_coin_flips_mt(clone_arc(&vertices_arc), seed, 0, nv);
        let coin_flips_arc: Arc<HashMapWithViewPlus<V, bool>> = Arc::new(coin_flips_owned);
        let coin_flips = arc_deref(&coin_flips_arc);

        // Loop 3: parallel edge classification (Work O(m), Span O(lg m)).
        let edge_vec = graph.E.to_seq();
        let ne = edge_vec.len();
        // Prove edge endpoints are in coin_flips and vertex_to_index.
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert forall|k: int| 0 <= k < ne as int implies
                #[trigger] coin_flips@.contains_key(edge_vec@[k]@.0) &&
                coin_flips@.contains_key(edge_vec@[k]@.1) &&
                vertex_to_index@.contains_key(edge_vec@[k]@.0) &&
                vertex_to_index@.contains_key(edge_vec@[k]@.1) by {
                let ghost mapped = edge_vec@.map(|_i: int, t: Edge<V>| t@);
                // Veracity: NEEDED assert
                assert(mapped[k] == edge_vec@[k]@);
                // Veracity: NEEDED assert
                assert(graph@.A.contains(edge_vec@[k]@));
                // coin_flips covers all graph vertices.
                let ghost v0 = edge_vec@[k]@.0;
                let j0 = vertices_vec@.map(|_i: int, t: V| t@).index_of(v0);
                let ghost v1 = edge_vec@[k]@.1;
                let j1 = vertices_vec@.map(|_i: int, t: V| t@).index_of(v1);
            };
        }
        let edge_arc: Arc<Vec<Edge<V>>> = Arc::new(edge_vec);
        let th_edges = build_th_edges_mt(
            edge_arc, clone_arc(&coin_flips_arc), clone_arc(&vtx_to_idx_arc),
            clone_arc(&vertices_arc), nv, 0, ne,
        );
        let th_edges = th_edges;

        // Loops 4+5: parallel build of satellite map + injected p_vec.
        // Step 4a: build satellite-to-center map from th_edges (Work O(m lg m), Span O(lg m)).
        let th_edges_arc: Arc<Vec<(usize, V)>> = Arc::new(th_edges);
        let nth = arc_deref(&th_edges_arc).len();
        let satellite_map = build_satellite_map_mt(
            th_edges_arc, clone_arc(&vertices_arc), nv,
            clone_arc(&coin_flips_arc), clone_arc(&vtx_to_idx_arc), 0, nth,
        );
        let satellite_map_arc: Arc<HashMapWithViewPlus<V, V>> = Arc::new(satellite_map);

        // Step 4b: build p_vec with inject (Work O(n), Span O(lg n)).
        let p_vec = build_p_vec_with_inject_mt(
            clone_arc(&vertices_arc), satellite_map_arc,
            clone_arc(&coin_flips_arc), clone_arc(&vtx_to_idx_arc), nv, 0, nv,
        );

        // Bridge: establish p_vec properties before Arc wrapping.
        // The ensures of build_p_vec_with_inject_mt (with clone helpers) give us
        // properties in terms of coin_flips@, vertices_vec@, vertex_to_index@.
        // Veracity: NEEDED proof block
        proof {
            // Heads preserve.
            // Veracity: NEEDED assert
            assert forall|j2: int| 0 <= j2 < nv as int implies
                coin_flips@.contains_key(vertices_vec@[j2]@) &&
                (coin_flips@[vertices_vec@[j2]@] ==>
                 #[trigger] p_vec@[j2]@ == vertices_vec@[j2]@) by {};
            // Modified entries point to heads.
            // Veracity: NEEDED assert
            assert forall|j2: int| 0 <= j2 < nv as int &&
                p_vec@[j2]@ != vertices_vec@[j2]@ implies
                coin_flips@.contains_key(#[trigger] p_vec@[j2]@) &&
                 coin_flips@[p_vec@[j2]@] by {};
            // All entries in vertex_to_index.
            // Veracity: NEEDED assert
            assert forall|j2: int| 0 <= j2 < nv as int implies
                vertex_to_index@.contains_key(#[trigger] p_vec@[j2]@) &&
                (vertex_to_index@[p_vec@[j2]@] as usize) < nv by {};
        }

        // Loop 6: parallel build of centers and partition_map (Work O(n), Span O(lg n)).
        // Wrap p_vec in Arc for parallel access.
        let p_vec_arc: Arc<Vec<V>> = Arc::new(p_vec);
        let p_vec = arc_deref(&p_vec_arc);

        // Build centers and partition_map in parallel via join.
        let ghost verts_view6 = vertices_vec@;
        let ghost pv_view6 = p_vec@;
        let va1 = vertices_arc.clone();
        let va2 = vertices_arc.clone();
        let pa1 = p_vec_arc.clone();
        let pa2 = p_vec_arc.clone();

        let f_centers = move || -> (r: SetStEph<V>)
            requires
                va1@.len() == pa1@.len(),
                va1@.no_duplicates(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
                forall|j: int| 0 <= j < va1@.len() as int ==>
                    pa1@[j]@ == va1@[j]@ ==>
                    #[trigger] r@.contains(pa1@[j]@),
        {
            let n = arc_deref(&va1).len();
            build_centers_mt(va1, pa1, 0, n)
        };

        let f_pm = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                nv == va2@.len(),
                va2@.len() == pa2@.len(),
                va2@.no_duplicates(),
                valid_key_type_Edge::<V>(),
            ensures
                forall|j: int| 0 <= j < nv as int ==>
                    #[trigger] r@.contains_key(va2@[j]@),
                forall|j: int| 0 <= j < nv as int ==>
                    r@.contains_key(va2@[j]@) ==>
                    r@[va2@[j]@]@ == #[trigger] pa2@[j]@,
                forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
                    exists|j: int| 0 <= j < nv as int && #[trigger] va2@[j]@ == v_view,
        {
            build_partition_map_mt(va2, pa2, 0, nv)
        };

        let Pair(centers, partition_map) = crate::ParaPair!(f_centers, f_pm);

        // Post-loop 6: prove spec_valid_partition_map.
        // Veracity: NEEDED proof block
        proof {
            // Part A: all graph@.V vertices are in partition_map.
            // Veracity: NEEDED assert
            assert forall|v_view: V::V| #[trigger] graph@.V.contains(v_view) implies
                partition_map@.contains_key(v_view) by {
                let k2 = vertices_vec@.map(|_i: int, t: V| t@).index_of(v_view);
            };

            // Part B: all partition_map values are in centers.
            // Veracity: NEEDED assert
            assert forall|v_view: V::V| #[trigger] partition_map@.contains_key(v_view) implies
                centers@.contains(partition_map@[v_view]@) by {
                // Any partition_map key came from vertices_vec (D&C domain invariant).
                let j = choose|j: int| 0 <= j < nv as int && #[trigger] vertices_vec@[j]@ == v_view;
                // partition_map@[v_view]@ == p_vec@[j]@.
                let ghost h = p_vec@[j]@;

                if h == vertices_vec@[j]@ {
                    // p_vec@[j]@ == vertices_vec@[j]@, so centers@.contains(h) from D&C.
                } else {
                    // h != vertices_vec@[j]@: from inject, h is a heads vertex.
                    // h is in vertex_to_index (inject ensures).
                    let ghost q_h = vertex_to_index@[h] as usize;
                    let j3 = choose|j3: int| 0 <= j3 < nv as int && #[trigger] vertices_vec@[j3]@ == h;
                    // Heads preserve at q_h: coin_flips@[h] == true => p_vec@[q_h]@ == vertices_vec@[q_h]@.
                    // centers contains h from D&C.
                    // Veracity: NEEDED assert
                    assert(centers@.contains(p_vec@[q_h as int]@));
                }
            };
        }

        (centers, partition_map)
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for StarPartitionMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarPartitionMtEph")
        }
    }

    impl std::fmt::Display for StarPartitionMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarPartitionMtEph")
        }
    }
}
