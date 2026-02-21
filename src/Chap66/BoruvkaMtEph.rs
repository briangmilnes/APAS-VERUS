//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 66: Borůvka's MST Algorithm (Parallel Ephemeral)
//!
//! Implements parallel versions of Algorithm 66.2 and 66.3 using ParaPair! macro.
//! Achieves Work O(m log n), Span O(log² n).

pub mod BoruvkaMtEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::*;
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

    /// Edge with label: (u, v, weight, label)
    /// Labeled edge for Borůvka's algorithm: (from, to, weight, label_id)
    /// Tuple struct wrapper to implement Display trait for StT compliance
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LabeledEdge<V>(pub V, pub V, pub F64Dist, pub usize);

    verus! {
        pub trait BoruvkaMtEphTrait {
            /// Find vertex bridges for parallel Borůvka's algorithm
            /// APAS: Work O(|E|), Span O(lg |E|)
            fn vertex_bridges_mt<V: StTInMtT + Hash + Ord + 'static>(
                edges: Arc<Vec<LabeledEdge<V>>>,
                start: usize,
                end: usize,
            ) -> HashMap<V, (V, F64Dist, usize)>;

            /// Parallel bridge-based star partition
            /// APAS: Work O(|V| + |E|), Span O(lg |V|)
            fn bridge_star_partition_mt<V: StTInMtT + Hash + Ord + 'static>(
                vertices_vec: Vec<V>,
                bridges: HashMap<V, (V, F64Dist, usize)>,
                rng: &mut StdRng,
            ) -> (SetStEph<V>, HashMap<V, (V, F64Dist, usize)>);

            /// Parallel Borůvka's MST algorithm
            /// APAS: Work O(m log n), Span O(log² n)
            fn boruvka_mst_mt<V: StTInMtT + Hash + Ord + 'static>(
                vertices_vec: Vec<V>,
                edges_vec: Vec<LabeledEdge<V>>,
                mst_labels: SetStEph<usize>,
                rng: &mut StdRng,
            ) -> SetStEph<usize>;

            /// Parallel Borůvka's MST with random seed
            /// APAS: Work O(m log n), Span O(log² n)
            fn boruvka_mst_mt_with_seed<V: StTInMtT + Hash + Ord + 'static>(
                vertices: &SetStEph<V>,
                edges: &SetStEph<LabeledEdge<V>>,
                seed: u64,
            ) -> SetStEph<usize>;

            /// Compute total weight of MST
            /// APAS: Work O(m), Span O(1)
            fn mst_weight<V: StT + Hash>(
                edges: &SetStEph<LabeledEdge<V>>,
                mst_labels: &SetStEph<usize>,
            ) -> F64Dist;
        }

        impl<V: StTInMtT + Ord + 'static> View for LabeledEdge<V> {
            type V = Self;
            open spec fn view(&self) -> Self { *self }
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<V: std::fmt::Display> std::fmt::Display for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
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
    ) -> HashMap<V, (V, F64Dist, usize)> {
        let size = end - start;
        if size == 0 {
            return HashMap::new();
        }

        if size == 1 {
            // Base case: single edge contributes bridges for both endpoints
            let LabeledEdge(u, v, w, label) = edges[start].clone();
            let mut result = HashMap::new();
            let _ = result.insert(u.clone(), (v.clone(), w, label));
            let _ = result.insert(v.clone(), (u.clone(), w, label));
            return result;
        }

        // Divide and conquer
        let mid = start + size / 2;
        let edges1 = edges.clone();
        let edges2 = edges;

        let pair = ParaPair!(
            move || vertex_bridges_mt(edges1, start, mid),
            move || vertex_bridges_mt(edges2, mid, end)
        );

        // Merge: for each vertex, keep the minimum weight edge
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
    /// Performs star contraction along vertex bridges using randomized coin flips.
    /// Parallelizes edge filtering via ParaPair!, but coin flips are sequential for RNG consistency.
    ///
    /// - APAS: Work O(n), Span O(log n)
    /// - Claude-Opus-4.6: Work O(n), Span O(n) — coin flips are sequential O(n); filter is parallel O(log n); remaining vertices computed sequentially O(n). Bottleneck is sequential loops.
    #[cfg(not(verus_keep_ghost))]
    pub fn bridge_star_partition_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices_vec: Vec<V>,
        bridges: HashMap<V, (V, F64Dist, usize)>,
        rng: &mut StdRng,
    ) -> (SetStEph<V>, HashMap<V, (V, F64Dist, usize)>) {
        // Coin flips (sequential for consistent seed)
        let mut coin_flips = HashMap::<V, bool>::new();
        for vertex in vertices_vec.iter() {
            let _ = coin_flips.insert(vertex.clone(), rng.random::<bool>());
        }

        // Parallel edge filtering: select edges from Tail→Head
        let vertices_len = vertices_vec.len();
        let vertices_arc = Arc::new(vertices_vec);
        let bridges_arc = Arc::new(bridges);
        let flips_arc = Arc::new(coin_flips);
        let partition = filter_tail_to_head_mt(vertices_arc.clone(), bridges_arc, flips_arc, 0, vertices_len);

        // Compute remaining vertices (not contracted)
        let mut remaining = SetLit![];
        for v in vertices_arc.iter() {
            if !partition.contains_key(v) {
                let _ = remaining.insert(v.clone());
            }
        }

        (remaining, partition)
    }

    /// Parallel filter: find edges from Tail→Head.
    ///
    /// - APAS: N/A — internal helper, not in prose.
    /// - Claude-Opus-4.6: Work O(n), Span O(log n) — parallel divide-and-conquer via ParaPair!.
    #[cfg(not(verus_keep_ghost))]
    fn filter_tail_to_head_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices: Arc<Vec<V>>,
        bridges: Arc<HashMap<V, (V, F64Dist, usize)>>,
        coin_flips: Arc<HashMap<V, bool>>,
        start: usize,
        end: usize,
    ) -> HashMap<V, (V, F64Dist, usize)> {
        let size = end - start;
        if size == 0 {
            return HashMap::new();
        }

        if size == 1 {
            // Base case: check single vertex
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

        // Divide and conquer
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

        // Merge
        let mut merged = pair.0;
        merged.extend(pair.1);
        merged
    }

    /// Parallel Borůvka's MST.
    ///
    /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
    /// Parallelizes vertex bridge computation and edge routing via ParaPair!.
    ///
    /// - APAS: Work O(m log n), Span O(log² n)
    /// - Claude-Opus-4.6: Work O(m log n), Span O(n log n) — vertex_bridges_mt is O(log m) span, but bridge_star_partition_mt has O(n) span due to sequential coin flips and remaining-vertex computation. Over O(log n) rounds, span is O(n log n), not O(log² n).
    #[cfg(not(verus_keep_ghost))]
    pub fn boruvka_mst_mt<V: StTInMtT + Hash + Ord + 'static>(
        vertices_vec: Vec<V>,
        edges_vec: Vec<LabeledEdge<V>>,
        mst_labels: SetStEph<usize>,
        rng: &mut StdRng,
    ) -> SetStEph<usize> {
        // Base case: no edges remaining
        if edges_vec.is_empty() {
            return mst_labels;
        }

        // Find vertex bridges (parallel)
        let edges_len = edges_vec.len();
        let edges_arc = Arc::new(edges_vec);
        let bridges = vertex_bridges_mt(edges_arc.clone(), 0, edges_len);

        // Perform bridge star partition
        let (remaining_vertices, partition) = bridge_star_partition_mt(vertices_vec, bridges, rng);

        // Collect new MST labels from partition
        let mut new_mst_labels = mst_labels.clone();
        for (_, (_, _, label)) in partition.iter() {
            let _ = new_mst_labels.insert(*label);
        }

        // Build full partition map (including identity for non-contracted vertices)
        let mut full_partition = HashMap::<V, V>::new();
        for (tail, (head, _, _)) in partition.iter() {
            let _ = full_partition.insert(tail.clone(), head.clone());
        }
        for v in remaining_vertices.iter() {
            let _ = full_partition.insert(v.clone(), v.clone());
        }

        // Parallel edge re-routing
        let part_arc = Arc::new(full_partition);
        let new_edges = reroute_edges_mt(edges_arc, part_arc, 0, edges_len);

        // Recurse
        let remaining_vec = remaining_vertices.iter().cloned().collect::<Vec<V>>();
        boruvka_mst_mt(remaining_vec, new_edges, new_mst_labels, rng)
    }

    /// Parallel edge re-routing: map edges to new endpoints and remove self-edges.
    ///
    /// - APAS: N/A — internal helper, not in prose.
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

        // Divide and conquer
        let mid = start + size / 2;
        let edges1 = edges.clone();
        let part1 = partition.clone();
        let edges2 = edges;
        let part2 = partition;

        let pair = ParaPair!(move || reroute_edges_mt(edges1, part1, start, mid), move || {
            reroute_edges_mt(edges2, part2, mid, end)
        });

        // Merge
        let mut left_result = pair.0;
        let mut right_result = pair.1;
        left_result.append(&mut right_result);
        left_result
    }

    /// Create Borůvka MST with a specific seed.
    /// Wrapper that initializes RNG, converts sets to vecs, and delegates to `boruvka_mst_mt`.
    ///
    /// - APAS: Work O(m log n), Span O(log² n)
    /// - Claude-Opus-4.6: Work O(m log n), Span O(n log n) — delegates to boruvka_mst_mt which has O(n) span per round.
    #[cfg(not(verus_keep_ghost))]
    pub fn boruvka_mst_mt_with_seed<V: StTInMtT + Hash + Ord + 'static>(
        vertices: &SetStEph<V>,
        edges: &SetStEph<LabeledEdge<V>>,
        seed: u64,
    ) -> SetStEph<usize> {
        let mut rng = StdRng::seed_from_u64(seed);
        let vertices_vec = vertices.iter().cloned().collect::<Vec<V>>();
        let edges_vec = edges.iter().cloned().collect::<Vec<LabeledEdge<V>>>();
        boruvka_mst_mt(vertices_vec, edges_vec, SetLit![], &mut rng)
    }

    /// Compute MST weight from edge labels.
    ///
    /// - APAS: N/A — utility function, not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential scan of edges.
    #[cfg(not(verus_keep_ghost))]
    pub fn mst_weight<V: StT + Hash>(
        edges: &SetStEph<LabeledEdge<V>>,
        mst_labels: &SetStEph<usize>,
    ) -> F64Dist {
        let mut total = zero_dist();
        for LabeledEdge(_, _, w, label) in edges.iter() {
            if mst_labels.mem(label) {
                total += *w;
            }
        }
        total
    }
}
