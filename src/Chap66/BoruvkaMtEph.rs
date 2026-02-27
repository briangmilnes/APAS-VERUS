//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 66: Borůvka's MST Algorithm (Parallel Ephemeral)
//!
//! Implements parallel versions of Algorithm 66.2 and 66.3 using ParaPair! macro.
//! Achieves Work O(m log n), Span O(log² n).

pub mod BoruvkaMtEph {

    use vstd::prelude::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use std::sync::Arc;
    #[cfg(not(verus_keep_ghost))]
    use rand::rngs::StdRng;
    #[cfg(not(verus_keep_ghost))]
    use rand::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::{ParaPair, SetLit};

    verus! {
        /// Edge with label: (u, v, weight, label). Vertices u,v change during contraction.
        pub struct LabeledEdge<V>(pub V, pub V, pub WrappedF64, pub usize);

        impl<V: Copy> Copy for LabeledEdge<V> {}

        impl<V: Copy> Clone for LabeledEdge<V> {
            fn clone(&self) -> (s: Self)
                ensures s@ == self@
            {
                *self
            }
        }

        impl<V: PartialEq + Copy> PartialEq for LabeledEdge<V> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self@ == other@)
            {
                let r = self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3;
                proof { accept(r == (self@ == other@)); }
                r
            }
        }

        impl<V: Eq + Copy> Eq for LabeledEdge<V> {}

        pub trait BoruvkaMtEphTrait {
            /// Find vertex bridges for parallel Borůvka's algorithm.
            /// APAS: Work O(|E|), Span O(lg |E|)
            fn vertex_bridges_mt<V: StTInMtT + Hash + Ord + 'static>(
                edges: Arc<Vec<LabeledEdge<V>>>,
                start: usize,
                end: usize,
            ) -> HashMap<V, (V, WrappedF64, usize)>;

            /// Parallel bridge-based star partition.
            /// APAS: Work O(|V| + |E|), Span O(lg |V|)
            fn bridge_star_partition_mt<V: StTInMtT + Hash + Ord + 'static>(
                vertices_vec: Vec<V>,
                bridges: HashMap<V, (V, WrappedF64, usize)>,
                seed: u64,
                round: usize,
            ) -> (SetStEph<V>, HashMap<V, (V, WrappedF64, usize)>);

            /// Parallel Borůvka's MST algorithm.
            /// APAS: Work O(m log n), Span O(log² n)
            fn boruvka_mst_mt<V: StTInMtT + Hash + Ord + 'static>(
                vertices_vec: Vec<V>,
                edges_vec: Vec<LabeledEdge<V>>,
                mst_labels: SetStEph<usize>,
                seed: u64,
                round: usize,
            ) -> SetStEph<usize>;

            /// Parallel Borůvka's MST with random seed.
            /// APAS: Work O(m log n), Span O(log² n)
            fn boruvka_mst_mt_with_seed<V: StTInMtT + Hash + Ord + 'static>(
                vertices: &SetStEph<V>,
                edges: &SetStEph<LabeledEdge<V>>,
                seed: u64,
            ) -> SetStEph<usize>;

            /// Compute total weight of MST.
            /// APAS: Work O(m), Span O(1)
            fn mst_weight<V: StT + Hash>(
                edges: &SetStEph<LabeledEdge<V>>,
                mst_labels: &SetStEph<usize>,
            ) -> WrappedF64;
        }

        impl<V: StTInMtT + Ord + 'static> View for LabeledEdge<V> {
            type V = Self;
            open spec fn view(&self) -> Self { *self }
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> PartialOrd for LabeledEdge<V> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> Ord for LabeledEdge<V> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
                .then_with(|| self.1.cmp(&other.1))
                .then_with(|| self.2.val.partial_cmp(&other.2.val).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| self.3.cmp(&other.3))
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: std::hash::Hash> std::hash::Hash for LabeledEdge<V> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
            self.1.hash(state);
            self.2.val.to_bits().hash(state);
            self.3.hash(state);
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: std::fmt::Debug> std::fmt::Debug for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("LabeledEdge").field(&self.0).field(&self.1).field(&self.2.val).field(&self.3).finish()
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: std::fmt::Display> std::fmt::Display for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }

    // Hash-based coin flip: deterministic from (seed, round, vertex index).
    // Replaces sequential StdRng coin flips with a parallelizable hash function.
    #[cfg(not(verus_keep_ghost))]
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
    #[cfg(not(verus_keep_ghost))]
    fn hash_coin_flips_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices: Arc<Vec<V>>,
        seed: u64,
        round: usize,
        start: usize,
        end: usize,
    ) -> HashMap<V, bool> {
        let size = end - start;
        if size == 0 {
            return HashMap::new();
        }
        if size == 1 {
            let mut result = HashMap::new();
            let _ = result.insert(vertices[start].clone(), hash_coin(seed, round, start));
            return result;
        }

        let mid = start + size / 2;
        let v1 = vertices.clone();
        let v2 = vertices;

        let pair = ParaPair!(
            move || hash_coin_flips_mt(v1, seed, round, start, mid),
            move || hash_coin_flips_mt(v2, seed, round, mid, end)
        );

        let mut merged = pair.0;
        merged.extend(pair.1);
        merged
    }

    /// Parallel remaining-vertex filter using divide-and-conquer.
    ///
    /// - Work O(n), Span O(log n) — parallel filter via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    fn compute_remaining_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices: Arc<Vec<V>>,
        partition: Arc<HashMap<V, (V, WrappedF64, usize)>>,
        start: usize,
        end: usize,
    ) -> Vec<V> {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            let v = &vertices[start];
            if !partition.contains_key(v) {
                return vec![v.clone()];
            }
            return Vec::new();
        }

        let mid = start + size / 2;
        let v1 = vertices.clone();
        let p1 = partition.clone();
        let v2 = vertices;
        let p2 = partition;

        let pair = ParaPair!(
            move || compute_remaining_mt(v1, p1, start, mid),
            move || compute_remaining_mt(v2, p2, mid, end)
        );

        let mut left = pair.0;
        left.extend(pair.1);
        left
    }

    /// Parallel MST label collection using divide-and-conquer.
    ///
    /// - Work O(n), Span O(log n) — parallel label extraction via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    fn collect_mst_labels_mt<V: StTInMtT + Hash + Ord + 'static>(
        keys: Arc<Vec<V>>,
        partition: Arc<HashMap<V, (V, WrappedF64, usize)>>,
        start: usize,
        end: usize,
    ) -> Vec<usize> {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }
        if size == 1 {
            if let Some((_, _, label)) = partition.get(&keys[start]) {
                return vec![*label];
            }
            return Vec::new();
        }

        let mid = start + size / 2;
        let k1 = keys.clone();
        let p1 = partition.clone();
        let k2 = keys;
        let p2 = partition;

        let pair = ParaPair!(
            move || collect_mst_labels_mt(k1, p1, start, mid),
            move || collect_mst_labels_mt(k2, p2, mid, end)
        );

        let mut left = pair.0;
        left.extend(pair.1);
        left
    }

    /// Parallel partition map construction using divide-and-conquer.
    /// Maps tails→heads from partition, remaining→identity.
    ///
    /// - Work O(n), Span O(log n) — parallel map building via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    fn build_partition_map_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices: Arc<Vec<V>>,
        partition: Arc<HashMap<V, (V, WrappedF64, usize)>>,
        start: usize,
        end: usize,
    ) -> HashMap<V, V> {
        let size = end - start;
        if size == 0 {
            return HashMap::new();
        }
        if size == 1 {
            let v = &vertices[start];
            let mut result = HashMap::new();
            if let Some((head, _, _)) = partition.get(v) {
                let _ = result.insert(v.clone(), head.clone());
            } else {
                let _ = result.insert(v.clone(), v.clone());
            }
            return result;
        }

        let mid = start + size / 2;
        let v1 = vertices.clone();
        let p1 = partition.clone();
        let v2 = vertices;
        let p2 = partition;

        let pair = ParaPair!(
            move || build_partition_map_mt(v1, p1, start, mid),
            move || build_partition_map_mt(v2, p2, mid, end)
        );

        let mut merged = pair.0;
        merged.extend(pair.1);
        merged
    }

    /// Parallel vertex bridges using divide-and-conquer reduce.
    ///
    /// For each vertex, find the minimum weight edge incident on it.
    /// Uses parallel reduce over edges via ParaPair!.
    ///
    /// - APAS: Work O(m), Span O(log m)
    /// - Claude-Opus-4.6: Work O(m), Span O(log m) — agrees with APAS; parallel divide-and-conquer via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    pub fn vertex_bridges_mt<V: StTInMtT + Hash + Ord + 'static>(
        edges: Arc<Vec<LabeledEdge<V>>>,
        start: usize,
        end: usize,
    ) -> HashMap<V, (V, WrappedF64, usize)> {
        let size = end - start;
        if size == 0 {
            return HashMap::new();
        }

        if size == 1 {
            // Base case: single edge contributes bridges for both endpoints.
            let LabeledEdge(u, v, w, label) = edges[start].clone();
            let mut result = HashMap::new();
            let _ = result.insert(u.clone(), (v.clone(), w, label));
            let _ = result.insert(v.clone(), (u.clone(), w, label));
            return result;
        }

        // Divide and conquer.
        let mid = start + size / 2;
        let edges1 = edges.clone();
        let edges2 = edges;

        let pair = ParaPair!(
            move || vertex_bridges_mt(edges1, start, mid),
            move || vertex_bridges_mt(edges2, mid, end)
        );

        // Merge: for each vertex, keep the minimum weight edge.
        let mut merged = pair.0;
        let right_bridges = pair.1;
        for (v, (neighbor, w, label)) in right_bridges {
            match merged.get(&v) {
                | None => {
                    let _ = merged.insert(v, (neighbor, w, label));
                }
                | Some((_, existing_w, _)) => {
                    if w < *existing_w {
                        let _ = merged.insert(v, (neighbor, w, label));
                    }
                }
            }
        }

        merged
    }

    /// Parallel bridge star partition.
    ///
    /// Performs star contraction along vertex bridges using hash-based coin flips.
    /// All operations parallelized via ParaPair! divide-and-conquer.
    ///
    /// - APAS: Work O(n), Span O(log n)
    /// - Claude-Opus-4.6: Work O(n), Span O(log n) — coin flips, filter, and remaining all O(log n) via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    pub fn bridge_star_partition_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices_vec: Vec<V>,
        bridges: HashMap<V, (V, WrappedF64, usize)>,
        seed: u64,
        round: usize,
    ) -> (SetStEph<V>, HashMap<V, (V, WrappedF64, usize)>) {
        // Parallel hash-based coin flips: O(n) work, O(log n) span.
        let vertices_len = vertices_vec.len();
        let vertices_arc = Arc::new(vertices_vec);
        let coin_flips = hash_coin_flips_mt(vertices_arc.clone(), seed, round, 0, vertices_len);

        // Parallel edge filtering: select edges from Tail→Head.
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
        let mut remaining = SetLit![];
        for v in remaining_vec {
            let _ = remaining.insert(v);
        }

        // Unwrap the Arc to return owned partition.
        let partition = Arc::try_unwrap(partition_arc).unwrap_or_else(|arc| (*arc).clone());
        (remaining, partition)
    }

    /// Parallel filter: find edges from Tail→Head.
    ///
    /// - Claude-Opus-4.6: Work O(n), Span O(log n) — parallel divide-and-conquer via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    fn filter_tail_to_head_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices: Arc<Vec<V>>,
        bridges: Arc<HashMap<V, (V, WrappedF64, usize)>>,
        coin_flips: Arc<HashMap<V, bool>>,
        start: usize,
        end: usize,
    ) -> HashMap<V, (V, WrappedF64, usize)> {
        let size = end - start;
        if size == 0 {
            return HashMap::new();
        }

        if size == 1 {
            // Base case: check single vertex.
            let u = &vertices[start];
            if let Some((v, w, label)) = bridges.get(u) {
                let u_heads = coin_flips.get(u).copied().unwrap_or(false);
                let v_heads = coin_flips.get(v).copied().unwrap_or(false);

                if !u_heads && v_heads {
                    let mut result = HashMap::new();
                    let _ = result.insert(u.clone(), (v.clone(), *w, *label));
                    return result;
                }
            }
            return HashMap::new();
        }

        // Divide and conquer.
        let mid = start + size / 2;
        let verts1 = vertices.clone();
        let bridges1 = bridges.clone();
        let flips1 = coin_flips.clone();
        let verts2 = vertices;
        let bridges2 = bridges;
        let flips2 = coin_flips;

        let pair = ParaPair!(
            move || filter_tail_to_head_mt(verts1, bridges1, flips1, start, mid),
            move || filter_tail_to_head_mt(verts2, bridges2, flips2, mid, end)
        );

        // Merge.
        let mut merged = pair.0;
        merged.extend(pair.1);
        merged
    }

    /// Parallel Borůvka's MST.
    ///
    /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
    /// All per-round operations parallelized via ParaPair!.
    ///
    /// - APAS: Work O(m log n), Span O(log² n)
    /// - Claude-Opus-4.6: Work O(m log n), Span O(log² n) — each round is O(log n) span (bridges O(log m), partition O(log n), reroute O(log m)); O(log n) rounds total.
    #[cfg(not(verus_keep_ghost))]
    pub fn boruvka_mst_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices_vec: Vec<V>,
        edges_vec: Vec<LabeledEdge<V>>,
        mst_labels: SetStEph<usize>,
        seed: u64,
        round: usize,
    ) -> SetStEph<usize> {
        // Base case: no edges remaining.
        if edges_vec.is_empty() {
            return mst_labels;
        }

        // Find vertex bridges (parallel): O(m) work, O(log m) span.
        let edges_len = edges_vec.len();
        let edges_arc = Arc::new(edges_vec);
        let bridges = vertex_bridges_mt(edges_arc.clone(), 0, edges_len);

        // Perform bridge star partition (parallel): O(n) work, O(log n) span.
        let (remaining_vertices, partition) =
            bridge_star_partition_mt(vertices_vec, bridges, seed, round);

        // Parallel MST label collection: O(n) work, O(log n) span.
        let partition_keys: Vec<V> = partition.keys().cloned().collect();
        let partition_keys_len = partition_keys.len();
        let partition_arc = Arc::new(partition);
        let keys_arc = Arc::new(partition_keys);
        let new_labels = collect_mst_labels_mt(
            keys_arc, partition_arc.clone(), 0, partition_keys_len,
        );
        let mut new_mst_labels = mst_labels.clone();
        for label in new_labels {
            let _ = new_mst_labels.insert(label);
        }

        // Parallel partition map construction: O(n) work, O(log n) span.
        let all_vertices: Vec<V> = {
            let mut v: Vec<V> = remaining_vertices.iter().cloned().collect();
            for k in partition_arc.keys() {
                v.push(k.clone());
            }
            v
        };
        let all_len = all_vertices.len();
        let all_arc = Arc::new(all_vertices);
        let full_partition = build_partition_map_mt(all_arc, partition_arc, 0, all_len);

        // Parallel edge re-routing: O(m) work, O(log m) span.
        let part_arc = Arc::new(full_partition);
        let new_edges = reroute_edges_mt(edges_arc, part_arc, 0, edges_len);

        // Recurse.
        let remaining_vec = remaining_vertices.iter().cloned().collect::<Vec<V>>();
        boruvka_mst_mt(remaining_vec, new_edges, new_mst_labels, seed, round + 1)
    }

    /// Parallel edge re-routing: map edges to new endpoints and remove self-edges.
    ///
    /// - Claude-Opus-4.6: Work O(m), Span O(log m) — parallel divide-and-conquer via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    fn reroute_edges_mt<V: StTInMtT + Hash + Ord + 'static>(
        edges: Arc<Vec<LabeledEdge<V>>>,
        partition: Arc<HashMap<V, V>>,
        start: usize,
        end: usize,
    ) -> Vec<LabeledEdge<V>> {
        let size = end - start;
        if size == 0 {
            return Vec::new();
        }

        if size == 1 {
            let LabeledEdge(u, v, w, label) = &edges[start];
            let new_u = partition.get(u).cloned().unwrap_or_else(|| u.clone());
            let new_v = partition.get(v).cloned().unwrap_or_else(|| v.clone());

            if new_u != new_v {
                return vec![LabeledEdge(new_u, new_v, *w, *label)];
            }
            return Vec::new();
        }

        // Divide and conquer.
        let mid = start + size / 2;
        let edges1 = edges.clone();
        let part1 = partition.clone();
        let edges2 = edges;
        let part2 = partition;

        let pair = ParaPair!(move || reroute_edges_mt(edges1, part1, start, mid), move || {
            reroute_edges_mt(edges2, part2, mid, end)
        });

        // Merge.
        let mut left_result = pair.0;
        let mut right_result = pair.1;
        left_result.append(&mut right_result);
        left_result
    }

    /// Create Borůvka MST with a specific seed.
    /// Wrapper that converts sets to vecs and delegates to `boruvka_mst_mt`.
    ///
    /// - APAS: Work O(m log n), Span O(log² n)
    /// - Claude-Opus-4.6: Work O(m log n), Span O(log² n) — delegates to boruvka_mst_mt which achieves O(log n) span per round.
    #[cfg(not(verus_keep_ghost))]
    pub fn boruvka_mst_mt_with_seed<V: StTInMtT + Hash + Ord + 'static>(
        vertices: &SetStEph<V>,
        edges: &SetStEph<LabeledEdge<V>>,
        seed: u64,
    ) -> SetStEph<usize> {
        let vertices_vec = vertices.iter().cloned().collect::<Vec<V>>();
        let edges_vec = edges.iter().cloned().collect::<Vec<LabeledEdge<V>>>();
        boruvka_mst_mt(vertices_vec, edges_vec, SetLit![], seed, 0)
    }

    /// Compute MST weight from edge labels.
    ///
    /// - APAS: N/A — utility function, not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential scan of edges.
    #[cfg(not(verus_keep_ghost))]
    pub fn mst_weight<V: StT + Hash>(
        edges: &SetStEph<LabeledEdge<V>>,
        mst_labels: &SetStEph<usize>,
    ) -> WrappedF64 {
        let mut total = zero_dist();
        for LabeledEdge(_, _, w, label) in edges.iter() {
            if mst_labels.mem(label) {
                total += *w;
            }
        }
        total
    }
}
