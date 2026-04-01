//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 66: Borůvka's MST Algorithm (Parallel Ephemeral)
//!
//! Implements parallel versions of Algorithm 66.2 and 66.3 using ParaPair! macro.
//! Achieves Work O(m log n), Span O(log² n).

pub mod BoruvkaMtEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::iter_invariant;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use std::sync::Arc;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_view_injective;
    use crate::{ParaPair, SetLit};
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::float::float::group_float_finite_total_order,
    };

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct BoruvkaMtEph;

    /// Edge with label: (u, v, weight, label). Vertices u,v change during contraction.
    pub struct LabeledEdge<V>(pub V, pub V, pub WrappedF64, pub usize);

    impl<V: Copy> Copy for LabeledEdge<V> {}

    impl<V: Copy> Clone for LabeledEdge<V> {
        fn clone(&self) -> (s: Self) {
            *self
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<V: PartialEq + Copy> PartialEqSpecImpl for LabeledEdge<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<V: PartialEq + Copy> PartialEq for LabeledEdge<V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.0 == other.0 && self.1 == other.1 && self.2.eq(&other.2) && self.3 == other.3;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<V: Eq + Copy> Eq for LabeledEdge<V> {}

    // 5. view impls

    impl<V: Copy> View for LabeledEdge<V> {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    // 6. spec fns

    /// All edge weights are finite (Set version).
    pub open spec fn spec_all_weights_finite<V: Copy>(edges: Set<LabeledEdge<V>>) -> bool {
        forall|e: LabeledEdge<V>| #[trigger] edges.contains(e) ==> e.2.spec_is_finite()
    }

    /// All edge weights are finite (Seq version for Vec-based parallel functions).
    pub open spec fn spec_all_weights_finite_seq<V: Copy>(edges: Seq<LabeledEdge<V>>) -> bool {
        forall|i: int| 0 <= i < edges.len() ==> (#[trigger] edges[i]).2.spec_is_finite()
    }

    // 8. traits

    pub trait BoruvkaMtEphTrait {
        /// Well-formedness for parallel Borůvka MST algorithm input.
        open spec fn spec_boruvkamteph_wf<V: Copy>(
            edges: Set<LabeledEdge<V>>,
        ) -> bool {
            spec_all_weights_finite(edges)
        }

        /// Find vertex bridges for parallel Borůvka's algorithm.
        /// APAS: Work O(|E|), Span O(lg |E|)
        fn vertex_bridges_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
            edges: Arc<Vec<LabeledEdge<V>>>,
            start: usize,
            end: usize,
        ) -> (bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                start <= end, end <= edges@.len(),
                spec_all_weights_finite_seq(edges@),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures
                forall|k: V::V| #[trigger] bridges@.contains_key(k) ==> bridges@[k].1.spec_is_finite();

        /// Parallel bridge-based star partition.
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        fn bridge_star_partition_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
            vertices_vec: Vec<V>,
            bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>,
            seed: u64,
            round: usize,
        ) -> (partition: (SetStEph<V>, HashMapWithViewPlus<V, (V, WrappedF64, usize)>))
            requires
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
                SetStEph::<V>::spec_valid_key_type(),
            ensures partition.0.spec_setsteph_wf();

        /// Parallel Borůvka's MST algorithm.
        /// APAS: Work O(m log n), Span O(log² n)
        /// - Alg Analysis: APAS (Ch66 Alg 66.1): Work O(m lg n), Span O(lg^3 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(lg^2 n) — DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers
        fn boruvka_mst_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
            vertices_vec: Vec<V>,
            edges_vec: Vec<LabeledEdge<V>>,
            mst_labels: SetStEph<usize>,
            seed: u64,
            round: usize,
        ) -> (mst: SetStEph<usize>)
            requires
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
                mst_labels.spec_setsteph_wf(),
                spec_all_weights_finite_seq(edges_vec@),
                SetStEph::<V>::spec_valid_key_type(),
            ensures mst.spec_setsteph_wf();

        /// Parallel Borůvka's MST with random seed.
        /// APAS: Work O(m log n), Span O(log² n)
        /// - Alg Analysis: APAS (Ch66 Alg 66.1): Work O(m lg n), Span O(lg^3 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(lg^2 n) — DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers
        fn boruvka_mst_mt_with_seed<V: StTInMtT + Hash + Ord + Copy + 'static>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            seed: u64,
        ) -> (mst: SetStEph<usize>)
            requires
                Self::spec_boruvkamteph_wf(edges@),
                vertices.spec_setsteph_wf(),
                edges.spec_setsteph_wf(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
                SetStEph::<V>::spec_valid_key_type(),
                SetStEph::<usize>::spec_valid_key_type(),
            ensures mst.spec_setsteph_wf();

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StTInMtT + Hash + Ord + Copy + 'static>(
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: &SetStEph<usize>,
        ) -> WrappedF64
            requires
                edges.spec_setsteph_wf(),
                mst_labels.spec_setsteph_wf();
    }

    // 9. impls

    // Hash-based coin flip: deterministic from (seed, round, vertex index).
    // Replaces sequential StdRng coin flips with a parallelizable hash function.
    #[verifier::external_body]
    fn hash_coin(seed: u64, round: usize, index: usize) -> bool {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        round.hash(&mut hasher);
        index.hash(&mut hasher);
        hasher.finish() % 2 == 0
    }

    /// Parallel coin flip generation using divide-and-conquer.
    ///
    /// - Work O(n), Span O(log n) — parallel hash-based coin generation via ParaPair!.
    fn hash_coin_flips_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices: Arc<Vec<V>>,
        seed: u64,
        round: usize,
        start: usize,
        end: usize,
    ) -> (flips: HashMapWithViewPlus<V, bool>)
        requires
            start <= end, end <= vertices@.len(),
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            start == end ==> flips@.len() == 0,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let mut coins = HashMapWithViewPlus::new();
            coins.insert(verts[start].clone(), hash_coin(seed, round, start));
            return coins;
        }

        let mid = start + size / 2;
        let v1 = vertices.clone();
        let v2 = vertices;

        let f1 = move || -> (r: HashMapWithViewPlus<V, bool>)
            requires
                start <= mid, mid <= v1@.len(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            hash_coin_flips_mt(v1, seed, round, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, bool>)
            requires
                mid <= end, end <= v2@.len(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            hash_coin_flips_mt(v2, seed, round, mid, end)
        };

        let Pair(mut merged, right) = crate::ParaPair!(f1, f2);

        // Merge right into merged using Verus-compatible iterator.
        let mut it = right.iter();
        let ghost it_seq = it@.1;
        loop
            invariant
                it@.0 <= it@.1.len(),
                it_seq == it@.1,
            decreases it_seq.len() - it@.0,
        {
            if let Some((k, v)) = it.next() {
                merged.insert(k.clone(), *v);
            } else {
                break;
            }
        }
        merged
    }

    /// Parallel remaining-vertex filter using divide-and-conquer.
    ///
    /// - Work O(n), Span O(log n) — parallel filter via ParaPair!.
    fn compute_remaining_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices: Arc<Vec<V>>,
        partition: Arc<HashMapWithViewPlus<V, (V, WrappedF64, usize)>>,
        start: usize,
        end: usize,
    ) -> (remaining: Vec<V>)
        requires start <= end, end <= vertices@.len(),
        ensures remaining@.len() <= (end - start) as int,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let v = &verts[start];
            if !partition.contains_key(v) {
                let mut result = Vec::new();
                result.push(v.clone());
                return result;
            }
            return Vec::new();
        }

        let mid = start + size / 2;
        let v1 = vertices.clone();
        let p1 = partition.clone();
        let v2 = vertices;
        let p2 = partition;

        let f1 = move || -> (r: Vec<V>)
            requires start <= mid, mid <= v1@.len(),
            ensures r@.len() <= (mid - start) as int,
        {
            compute_remaining_mt(v1, p1, start, mid)
        };

        let f2 = move || -> (r: Vec<V>)
            requires mid <= end, end <= v2@.len(),
            ensures r@.len() <= (end - mid) as int,
        {
            compute_remaining_mt(v2, p2, mid, end)
        };

        let Pair(mut left, right) = crate::ParaPair!(f1, f2);

        // Merge right into left.
        let ghost left_init_len = left@.len() as int;
        let mut i: usize = 0;
        while i < right.len()
            invariant
                0 <= i <= right@.len(),
                left@.len() == left_init_len + i as int,
                left_init_len <= (mid - start) as int,
                right@.len() <= (end - mid) as int,
            decreases right@.len() - i,
        {
            left.push(right[i].clone());
            i = i + 1;
        }
        left
    }

    /// Parallel MST label collection using divide-and-conquer.
    ///
    /// - Work O(n), Span O(log n) — parallel label extraction via ParaPair!.
    fn collect_mst_labels_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        keys: Arc<Vec<V>>,
        partition: Arc<HashMapWithViewPlus<V, (V, WrappedF64, usize)>>,
        start: usize,
        end: usize,
    ) -> (labels: Vec<usize>)
        requires start <= end, end <= keys@.len(),
        ensures labels@.len() <= (end - start) as int,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            let ks = arc_deref(&keys);
            if let Some((_, _, label)) = partition.get(&ks[start]) {
                let mut result = Vec::new();
                result.push(*label);
                return result;
            }
            return Vec::new();
        }

        let mid = start + size / 2;
        let k1 = keys.clone();
        let p1 = partition.clone();
        let k2 = keys;
        let p2 = partition;

        let f1 = move || -> (r: Vec<usize>)
            requires start <= mid, mid <= k1@.len(),
            ensures r@.len() <= (mid - start) as int,
        {
            collect_mst_labels_mt(k1, p1, start, mid)
        };

        let f2 = move || -> (r: Vec<usize>)
            requires mid <= end, end <= k2@.len(),
            ensures r@.len() <= (end - mid) as int,
        {
            collect_mst_labels_mt(k2, p2, mid, end)
        };

        let Pair(mut left, right) = crate::ParaPair!(f1, f2);

        // Merge right into left.
        let ghost left_init_len = left@.len() as int;
        let mut i: usize = 0;
        while i < right.len()
            invariant
                0 <= i <= right@.len(),
                left@.len() == left_init_len + i as int,
                left_init_len <= (mid - start) as int,
                right@.len() <= (end - mid) as int,
            decreases right@.len() - i,
        {
            left.push(right[i]);
            i = i + 1;
        }
        left
    }

    /// Parallel partition map construction using divide-and-conquer.
    /// Maps tails->heads from partition, remaining->identity.
    ///
    /// - Work O(n), Span O(log n) — parallel map building via ParaPair!.
    fn build_partition_map_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices: Arc<Vec<V>>,
        partition: Arc<HashMapWithViewPlus<V, (V, WrappedF64, usize)>>,
        start: usize,
        end: usize,
    ) -> (part_map: HashMapWithViewPlus<V, V>)
        requires
            start <= end, end <= vertices@.len(),
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            start == end ==> part_map@.len() == 0,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }
        if size == 1 {
            let verts = arc_deref(&vertices);
            let v = &verts[start];
            let mut heads: HashMapWithViewPlus<V, V> = HashMapWithViewPlus::new();
            if let Some((head, _, _)) = partition.get(v) {
                heads.insert(v.clone(), head.clone());
            } else {
                heads.insert(v.clone(), v.clone());
            }
            return heads;
        }

        let mid = start + size / 2;
        let v1 = vertices.clone();
        let p1 = partition.clone();
        let v2 = vertices;
        let p2 = partition;

        let f1 = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                start <= mid, mid <= v1@.len(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            build_partition_map_mt(v1, p1, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, V>)
            requires
                mid <= end, end <= v2@.len(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            build_partition_map_mt(v2, p2, mid, end)
        };

        let Pair(mut merged, right) = crate::ParaPair!(f1, f2);

        // Merge right into merged.
        let mut it = right.iter();
        let ghost it_seq = it@.1;
        loop
            invariant
                it@.0 <= it@.1.len(),
                it_seq == it@.1,
            decreases it_seq.len() - it@.0,
        {
            if let Some((k, v)) = it.next() {
                merged.insert(k.clone(), v.clone());
            } else {
                break;
            }
        }
        merged
    }

    /// Parallel vertex bridges using divide-and-conquer reduce.
    ///
    /// For each vertex, find the minimum weight edge incident on it.
    /// Uses parallel reduce over edges via ParaPair!.
    ///
    /// - Alg Analysis: APAS (Ch66 Alg 66.3): Work O(m), Span O(log m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(log m) — matches APAS
    /// - Claude-Opus-4.6: Work O(m), Span O(log m) — agrees with APAS; parallel divide-and-conquer via ParaPair!.
    pub fn vertex_bridges_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        edges: Arc<Vec<LabeledEdge<V>>>,
        start: usize,
        end: usize,
    ) -> (bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
        requires
            start <= end, end <= edges@.len(),
            spec_all_weights_finite_seq(edges@),
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            forall|k: V::V| #[trigger] bridges@.contains_key(k) ==> bridges@[k].1.spec_is_finite(),
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }

        if size == 1 {
            let es = arc_deref(&edges);
            let LabeledEdge(u, v, w, label) = es[start];
            assert(w.spec_is_finite());
            let mut min_edges: HashMapWithViewPlus<V, (V, WrappedF64, usize)> = HashMapWithViewPlus::new();
            min_edges.insert(u.clone(), (v.clone(), w, label));
            min_edges.insert(v.clone(), (u.clone(), w, label));
            return min_edges;
        }

        // Divide and conquer.
        let mid = start + size / 2;
        let edges1 = edges.clone();
        let edges2 = edges;

        let f1 = move || -> (r: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                start <= mid, mid <= edges1@.len(),
                spec_all_weights_finite_seq(edges1@),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures
                forall|k: V::V| #[trigger] r@.contains_key(k) ==> r@[k].1.spec_is_finite(),
        {
            vertex_bridges_mt(edges1, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                mid <= end, end <= edges2@.len(),
                spec_all_weights_finite_seq(edges2@),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures
                forall|k: V::V| #[trigger] r@.contains_key(k) ==> r@[k].1.spec_is_finite(),
        {
            vertex_bridges_mt(edges2, mid, end)
        };

        let Pair(mut merged, right_bridges) = crate::ParaPair!(f1, f2);

        // Merge: for each vertex, keep the minimum weight edge.
        // Both merged and right_bridges have finite weights from their ensures.
        let mut it = right_bridges.iter();
        let ghost it_seq = it@.1;
        loop
            invariant
                it@.0 <= it@.1.len(),
                it_seq == it@.1,
                forall|k: V::V| #[trigger] merged@.contains_key(k) ==> merged@[k].1.spec_is_finite(),
                forall|kv: (V, (V, WrappedF64, usize))| #[trigger] it_seq.to_set().contains(kv)
                    ==> right_bridges@.contains_key(kv.0@) && right_bridges@[kv.0@] == kv.1,
                forall|k: V::V| #[trigger] right_bridges@.contains_key(k) ==> right_bridges@[k].1.spec_is_finite(),
                obeys_key_model::<V>(),
            decreases it_seq.len() - it@.0,
        {
            if let Some((v, entry)) = it.next() {
                let (neighbor, w, label) = entry;
                // From iterator: (*v, *entry) is in it_seq.to_set(), so right_bridges has the entry.
                assert(it_seq.to_set().contains((*v, *entry)));
                assert(right_bridges@.contains_key(v@));
                assert(right_bridges@[v@] == *entry);
                assert(w.spec_is_finite());
                match merged.get(v) {
                    None => {
                        merged.insert(v.clone(), (neighbor.clone(), *w, *label));
                    }
                    Some((_, existing_w, _)) => {
                        assert(existing_w.spec_is_finite());
                        if w.dist_lt(existing_w) {
                            merged.insert(v.clone(), (neighbor.clone(), *w, *label));
                        }
                    }
                }
            } else {
                break;
            }
        }

        merged
    }

    /// Parallel bridge star partition.
    ///
    /// Performs star contraction along vertex bridges using hash-based coin flips.
    /// All operations parallelized via ParaPair! divide-and-conquer.
    ///
    /// - Alg Analysis: APAS (Ch66 Alg 66.3): Work O(n), Span O(log n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n) — matches APAS
    /// - Claude-Opus-4.6: Work O(n), Span O(log n) — coin flips, filter, and remaining all O(log n) via ParaPair!.
    pub fn bridge_star_partition_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices_vec: Vec<V>,
        bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>,
        seed: u64,
        round: usize,
    ) -> (partition: (SetStEph<V>, HashMapWithViewPlus<V, (V, WrappedF64, usize)>))
        requires
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
            SetStEph::<V>::spec_valid_key_type(),
        ensures partition.0.spec_setsteph_wf(),
    {
        // Parallel hash-based coin flips: O(n) work, O(log n) span.
        let vertices_len = vertices_vec.len();
        let vertices_arc = Arc::new(vertices_vec);
        let coin_flips = hash_coin_flips_mt(vertices_arc.clone(), seed, round, 0, vertices_len);

        // Parallel edge filtering: select edges from Tail->Head.
        let bridges_arc = Arc::new(bridges);
        let flips_arc = Arc::new(coin_flips);
        let partition = filter_tail_to_head_mt(
            vertices_arc.clone(), bridges_arc, flips_arc, 0, vertices_len,
        );

        // Parallel remaining-vertex filter: O(n) work, O(log n) span.
        let partition_arc = Arc::new(partition);
        let remaining_vec = compute_remaining_mt(
            vertices_arc, partition_arc.clone(), 0, vertices_len,
        );

        // Convert remaining_vec to SetStEph.
        let mut remaining: SetStEph<V> = SetStEph::empty();
        let mut i: usize = 0;
        while i < remaining_vec.len()
            invariant
                0 <= i <= remaining_vec@.len(),
                remaining.spec_setsteph_wf(),
            decreases remaining_vec@.len() - i,
        {
            let _ = remaining.insert(remaining_vec[i].clone());
            i = i + 1;
        }

        // Reconstruct partition from Arc by iterating.
        let mut partition_out: HashMapWithViewPlus<V, (V, WrappedF64, usize)> = HashMapWithViewPlus::new();
        let mut pit = partition_arc.iter();
        let ghost pit_seq = pit@.1;
        loop
            invariant
                pit@.0 <= pit@.1.len(),
                pit_seq == pit@.1,
            decreases pit_seq.len() - pit@.0,
        {
            if let Some((k, entry)) = pit.next() {
                let (v, w, label) = entry;
                partition_out.insert(k.clone(), (v.clone(), *w, *label));
            } else {
                break;
            }
        }
        (remaining, partition_out)
    }

    /// Parallel filter: find edges from Tail->Head.
    ///
    /// - Claude-Opus-4.6: Work O(n), Span O(log n) — parallel divide-and-conquer via ParaPair!.
    fn filter_tail_to_head_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices: Arc<Vec<V>>,
        bridges: Arc<HashMapWithViewPlus<V, (V, WrappedF64, usize)>>,
        coin_flips: Arc<HashMapWithViewPlus<V, bool>>,
        start: usize,
        end: usize,
    ) -> (filtered: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
        requires
            start <= end, end <= vertices@.len(),
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            start == end ==> filtered@.len() == 0,
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return HashMapWithViewPlus::new();
        }

        if size == 1 {
            let verts = arc_deref(&vertices);
            let u = &verts[start];
            if let Some((v, w, label)) = bridges.get(u) {
                let u_heads = match coin_flips.get(u) {
                    Some(b) => *b,
                    None => false,
                };
                let v_heads = match coin_flips.get(v) {
                    Some(b) => *b,
                    None => false,
                };

                if !u_heads && v_heads {
                    let mut result: HashMapWithViewPlus<V, (V, WrappedF64, usize)> = HashMapWithViewPlus::new();
                    result.insert(u.clone(), (v.clone(), *w, *label));
                    return result;
                }
            }
            return HashMapWithViewPlus::new();
        }

        // Divide and conquer.
        let mid = start + size / 2;
        let verts1 = vertices.clone();
        let bridges1 = bridges.clone();
        let flips1 = coin_flips.clone();
        let verts2 = vertices;
        let bridges2 = bridges;
        let flips2 = coin_flips;

        let f1 = move || -> (r: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                start <= mid, mid <= verts1@.len(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            filter_tail_to_head_mt(verts1, bridges1, flips1, start, mid)
        };

        let f2 = move || -> (r: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                mid <= end, end <= verts2@.len(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            filter_tail_to_head_mt(verts2, bridges2, flips2, mid, end)
        };

        let Pair(mut merged, right) = crate::ParaPair!(f1, f2);

        // Merge right into merged.
        let mut it = right.iter();
        let ghost it_seq = it@.1;
        loop
            invariant
                it@.0 <= it@.1.len(),
                it_seq == it@.1,
            decreases it_seq.len() - it@.0,
        {
            if let Some((k, entry)) = it.next() {
                let (v, w, label) = entry;
                merged.insert(k.clone(), (v.clone(), *w, *label));
            } else {
                break;
            }
        }
        merged
    }

    /// Parallel Borůvka's MST.
    ///
    /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
    /// All per-round operations parallelized via ParaPair!.
    ///
    /// - Alg Analysis: APAS (Ch66 Alg 66.1): Work O(m lg n), Span O(lg^3 n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(lg^2 n) — DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers
    #[verifier::exec_allows_no_decreases_clause]
    pub fn boruvka_mst_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices_vec: Vec<V>,
        edges_vec: Vec<LabeledEdge<V>>,
        mst_labels: SetStEph<usize>,
        seed: u64,
        round: usize,
    ) -> (mst: SetStEph<usize>)
        requires
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
            mst_labels.spec_setsteph_wf(),
            spec_all_weights_finite_seq(edges_vec@),
            SetStEph::<V>::spec_valid_key_type(),
        ensures mst.spec_setsteph_wf(),
    {
        let mut vertices_vec = vertices_vec;
        let mut edges_vec = edges_vec;
        let mut mst_labels = mst_labels;
        let mut round = round;

        loop
            invariant
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
                mst_labels.spec_setsteph_wf(),
                spec_all_weights_finite_seq(edges_vec@),
                SetStEph::<V>::spec_valid_key_type(),
        {
            // Base case: no edges remaining.
            if edges_vec.len() == 0 {
                return mst_labels;
            }

            // Find vertex bridges (parallel): O(m) work, O(log m) span.
            let edges_len = edges_vec.len();
            let edges_arc = Arc::new(edges_vec);
            let bridges = vertex_bridges_mt(edges_arc.clone(), 0, edges_len);

            // Perform bridge star partition (parallel): O(n) work, O(log n) span.
            let (remaining_vertices, partition) =
                bridge_star_partition_mt(vertices_vec, bridges, seed, round);

            // Collect partition keys into Vec via iterator.
            let mut partition_keys: Vec<V> = Vec::new();
            {
                let mut pit = partition.iter();
                let ghost pit_seq = pit@.1;
                loop
                    invariant
                        pit@.0 <= pit@.1.len(),
                        pit_seq == pit@.1,
                    decreases pit_seq.len() - pit@.0,
                {
                    if let Some((k, _)) = pit.next() {
                        partition_keys.push(k.clone());
                    } else {
                        break;
                    }
                }
            }

            // Parallel MST label collection: O(n) work, O(log n) span.
            let partition_keys_len = partition_keys.len();
            let partition_arc = Arc::new(partition);
            let keys_arc = Arc::new(partition_keys);
            let new_labels = collect_mst_labels_mt(
                keys_arc, partition_arc.clone(), 0, partition_keys_len,
            );
            let mut li: usize = 0;
            while li < new_labels.len()
                invariant
                    0 <= li <= new_labels@.len(),
                    mst_labels.spec_setsteph_wf(),
                decreases new_labels@.len() - li,
            {
                let _ = mst_labels.insert(new_labels[li]);
                li = li + 1;
            }

            // Build all_vertices Vec: remaining + partition keys.
            let mut all_vertices: Vec<V> = Vec::new();
            let mut remaining_vec: Vec<V> = Vec::new();
            {
                let mut rit = remaining_vertices.iter();
                let ghost rit_seq = rit@.1;
                loop
                    invariant
                        iter_invariant(&rit),
                        rit_seq == rit@.1,
                    decreases rit_seq.len() - rit@.0,
                {
                    if let Some(v) = rit.next() {
                        all_vertices.push(v.clone());
                        remaining_vec.push(v.clone());
                    } else {
                        break;
                    }
                }
            }
            // Add partition keys to all_vertices via partition_arc iterator.
            {
                let mut pit2 = partition_arc.iter();
                let ghost pit2_seq = pit2@.1;
                loop
                    invariant
                        pit2@.0 <= pit2@.1.len(),
                        pit2_seq == pit2@.1,
                    decreases pit2_seq.len() - pit2@.0,
                {
                    if let Some((k, _)) = pit2.next() {
                        all_vertices.push(k.clone());
                    } else {
                        break;
                    }
                }
            }

            // Parallel partition map construction: O(n) work, O(log n) span.
            let all_len = all_vertices.len();
            let all_arc = Arc::new(all_vertices);
            let full_partition = build_partition_map_mt(all_arc, partition_arc, 0, all_len);

            // Parallel edge re-routing: O(m) work, O(log m) span.
            let part_arc = Arc::new(full_partition);
            let new_edges = reroute_edges_mt(edges_arc, part_arc, 0, edges_len);

            // Prepare for next iteration (tail-recursion to loop conversion).
            vertices_vec = remaining_vec;
            edges_vec = new_edges;
            if round < usize::MAX {
                round = round + 1;
            }
        }
    }

    /// Parallel edge re-routing: map edges to new endpoints and remove self-edges.
    ///
    /// - Claude-Opus-4.6: Work O(m), Span O(log m) — parallel divide-and-conquer via ParaPair!.
    fn reroute_edges_mt<V: StTInMtT + Hash + Ord + Copy + 'static>(
        edges: Arc<Vec<LabeledEdge<V>>>,
        partition: Arc<HashMapWithViewPlus<V, V>>,
        start: usize,
        end: usize,
    ) -> (rerouted: Vec<LabeledEdge<V>>)
        requires
            start <= end, end <= edges@.len(),
            spec_all_weights_finite_seq(edges@),
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            spec_all_weights_finite_seq(rerouted@),
        decreases end - start,
    {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }

        if size == 1 {
            let es = arc_deref(&edges);
            let LabeledEdge(u, v, w, label) = es[start];
            let new_u = match partition.get(&u) {
                Some(mapped) => mapped.clone(),
                None => u,
            };
            let new_v = match partition.get(&v) {
                Some(mapped) => mapped.clone(),
                None => v,
            };

            if new_u != new_v {
                let mut result = Vec::new();
                result.push(LabeledEdge(new_u, new_v, w, label));
                return result;
            }
            return Vec::new();
        }

        // Divide and conquer.
        let mid = start + size / 2;
        let edges1 = edges.clone();
        let part1 = partition.clone();
        let edges2 = edges;
        let part2 = partition;

        let f1 = move || -> (r: Vec<LabeledEdge<V>>)
            requires
                start <= mid, mid <= edges1@.len(),
                spec_all_weights_finite_seq(edges1@),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures
                spec_all_weights_finite_seq(r@),
        {
            reroute_edges_mt(edges1, part1, start, mid)
        };

        let f2 = move || -> (r: Vec<LabeledEdge<V>>)
            requires
                mid <= end, end <= edges2@.len(),
                spec_all_weights_finite_seq(edges2@),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures
                spec_all_weights_finite_seq(r@),
        {
            reroute_edges_mt(edges2, part2, mid, end)
        };

        let Pair(mut left_result, right_result) = crate::ParaPair!(f1, f2);

        // Merge right into left.
        let mut i: usize = 0;
        while i < right_result.len()
            invariant
                0 <= i <= right_result@.len(),
                spec_all_weights_finite_seq(left_result@),
                spec_all_weights_finite_seq(right_result@),
            decreases right_result@.len() - i,
        {
            left_result.push(right_result[i]);
            i = i + 1;
        }
        left_result
    }

    /// Create Borůvka MST with a specific seed.
    /// Wrapper that converts sets to vecs and delegates to `boruvka_mst_mt`.
    ///
    /// - Alg Analysis: APAS (Ch66 Alg 66.1): Work O(m lg n), Span O(lg^3 n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(lg^2 n) — DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers
    pub fn boruvka_mst_mt_with_seed<V: StTInMtT + Hash + Ord + Copy + 'static>(
        vertices: &SetStEph<V>,
        edges: &SetStEph<LabeledEdge<V>>,
        seed: u64,
    ) -> (mst: SetStEph<usize>)
        requires
            spec_all_weights_finite(edges@),
            vertices.spec_setsteph_wf(),
            edges.spec_setsteph_wf(),
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
            SetStEph::<V>::spec_valid_key_type(),
            SetStEph::<usize>::spec_valid_key_type(),
        ensures mst.spec_setsteph_wf(),
    {
        // Collect vertices into Vec.
        let mut vertices_vec: Vec<V> = Vec::new();
        let mut vit = vertices.iter();
        let ghost vseq = vit@.1;
        loop
            invariant
                iter_invariant(&vit),
                vseq == vit@.1,
            decreases vseq.len() - vit@.0,
        {
            if let Some(v) = vit.next() {
                vertices_vec.push(v.clone());
            } else {
                break;
            }
        }

        // Collect edges into Vec.
        let mut edges_vec: Vec<LabeledEdge<V>> = Vec::new();
        let mut eit = edges.iter();
        let ghost eseq = eit@.1;
        loop
            invariant
                iter_invariant(&eit),
                eseq == eit@.1,
                spec_all_weights_finite_seq(edges_vec@),
                forall|j: int| 0 <= j < eseq.len() ==> edges@.contains(#[trigger] eseq[j]@),
                spec_all_weights_finite(edges@),
            decreases eseq.len() - eit@.0,
        {
            if let Some(e) = eit.next() {
                // e is from edges@, so its weight is finite.
                assert(edges@.contains(eseq[eit@.0 - 1]@));
                assert((*e).2.spec_is_finite());
                edges_vec.push(*e);
            } else {
                break;
            }
        }

        boruvka_mst_mt(vertices_vec, edges_vec, SetStEph::empty(), seed, 0)
    }

    /// Compute MST weight from edge labels.
    ///
    /// - Alg Analysis: APAS: N/A — utility function, not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential scan of edges.
    pub fn mst_weight<V: StTInMtT + Hash + Ord + Copy + 'static>(
        edges: &SetStEph<LabeledEdge<V>>,
        mst_labels: &SetStEph<usize>,
    ) -> (total: WrappedF64)
        requires
            edges.spec_setsteph_wf(),
            mst_labels.spec_setsteph_wf(),
        ensures
            edges@.len() == 0 ==> total@ == 0.0f64,
    {
        if edges.size() == 0 {
            return zero_dist();
        }
        let mut total = zero_dist();
        let mut it = edges.iter();
        let ghost iter_seq = it@.1;
        loop
            invariant
                iter_invariant(&it),
                iter_seq == it@.1,
                mst_labels.spec_setsteph_wf(),
            decreases iter_seq.len() - it@.0,
        {
            if let Some(edge) = it.next() {
                let LabeledEdge(_, _, w, label) = edge;
                if mst_labels.mem(label) {
                    total = total.dist_add(w);
                }
            } else {
                break;
            }
        }
        total
    }

    } // verus!

    // 14. derive impls outside verus!

    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> PartialOrd for LabeledEdge<V> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> Ord for LabeledEdge<V> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
                .then_with(|| self.1.cmp(&other.1))
                .then_with(|| self.2.val.partial_cmp(&other.2.val).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| self.3.cmp(&other.3))
        }
    }
    impl<V: std::hash::Hash> std::hash::Hash for LabeledEdge<V> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
            self.1.hash(state);
            self.2.val.to_bits().hash(state);
            self.3.hash(state);
        }
    }
    impl<V: std::fmt::Debug> std::fmt::Debug for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("LabeledEdge").field(&self.0).field(&self.1).field(&self.2.val).field(&self.3).finish()
        }
    }
    impl<V: std::fmt::Display> std::fmt::Display for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }
}
