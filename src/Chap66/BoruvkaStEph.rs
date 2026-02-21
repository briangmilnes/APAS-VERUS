//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 66: Borůvka's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 66.2 and 66.3: Borůvka's algorithm for computing Minimum Spanning Trees
//! using vertex bridges and graph contraction with randomized star contraction.

pub mod BoruvkaStEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::{F64Dist, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use rand::rngs::StdRng;
    #[cfg(not(verus_keep_ghost))]
    use rand::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    /// Edge with label: (u, v, weight, label)
    /// Vertices u,v change during contraction, but weight and label are immutable
    /// Labeled edge for Borůvka's algorithm: (from, to, weight, label_id)
    /// Tuple struct wrapper to implement Display trait for StT compliance
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LabeledEdge<V>(pub V, pub V, pub F64Dist, pub usize);

    verus! {
        pub trait BoruvkaStEphTrait {
            /// Find vertex bridges for Borůvka's algorithm
            /// APAS: Work O(|E|), Span O(|E|)
            fn vertex_bridges<V: StT + Hash + Ord>(
                edges: &SetStEph<LabeledEdge<V>>,
            ) -> HashMap<V, (V, F64Dist, usize)>;

            /// Bridge-based star partition
            /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
            fn bridge_star_partition<V: StT + Hash + Ord>(
                vertices: &SetStEph<V>,
                bridges: &HashMap<V, (V, F64Dist, usize)>,
                rng: &mut StdRng,
            ) -> (SetStEph<V>, HashMap<V, (V, F64Dist, usize)>);

            /// Borůvka's MST algorithm
            /// APAS: Work O(m log n), Span O(m log n)
            fn boruvka_mst<V: StT + Hash + Ord>(
                vertices: &SetStEph<V>,
                edges: &SetStEph<LabeledEdge<V>>,
                mst_labels: SetStEph<usize>,
                rng: &mut StdRng,
            ) -> SetStEph<usize>;

            /// Borůvka's MST with random seed
            /// APAS: Work O(m log n), Span O(m log n)
            fn boruvka_mst_with_seed<V: StT + Hash + Ord>(
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

        impl<V: StT + Ord> View for LabeledEdge<V> {
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

    /// Algorithm 66.3: Find vertex bridges
    ///
    /// For each vertex, find the minimum weight edge incident on it.
    /// Returns a table mapping each vertex to (neighbor, weight, label).
    ///
    /// - APAS: Work O(m), Span O(log m)
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential iteration over edges.
    #[cfg(not(verus_keep_ghost))]
    pub fn vertex_bridges<V: StT + Hash + Ord>(
        edges: &SetStEph<LabeledEdge<V>>,
    ) -> HashMap<V, (V, F64Dist, usize)> {
        let mut bridges = HashMap::<V, (V, F64Dist, usize)>::new();

        for edge in edges.iter() {
            let LabeledEdge(u, v, w, label) = edge.clone();

            // Update bridge for u
            match bridges.get(&u) {
                | None => {
                    let _ = bridges.insert(u.clone(), (v.clone(), w, label));
                }
                | Some((_, existing_w, _)) => {
                    if w < *existing_w {
                        let _ = bridges.insert(u.clone(), (v.clone(), w, label));
                    }
                }
            }

            // Update bridge for v
            match bridges.get(&v) {
                | None => {
                    let _ = bridges.insert(v.clone(), (u.clone(), w, label));
                }
                | Some((_, existing_w, _)) => {
                    if w < *existing_w {
                        let _ = bridges.insert(v.clone(), (u.clone(), w, label));
                    }
                }
            }
        }

        bridges
    }

    /// Algorithm 66.2: Bridge star partition
    ///
    /// Performs star contraction along vertex bridges using randomized coin flips.
    /// Each vertex flips a coin (Heads/Tails). Edges from Tail→Head are contracted.
    ///
    /// - APAS: Work O(n), Span O(log n)
    /// - Claude-Opus-4.6: Work O(n), Span O(n) — sequential iteration over vertices.
    #[cfg(not(verus_keep_ghost))]
    pub fn bridge_star_partition<V: StT + Hash + Ord>(
        vertices: &SetStEph<V>,
        bridges: &HashMap<V, (V, F64Dist, usize)>,
        rng: &mut StdRng,
    ) -> (SetStEph<V>, HashMap<V, (V, F64Dist, usize)>) {
        // Coin flips for all vertices
        let mut flips = HashMap::<V, bool>::new();
        for v in vertices.iter() {
            let is_heads = rng.random::<bool>();
            let _ = flips.insert(v.clone(), is_heads);
        }

        // Select edges from Tail→Head (Tail=false, Head=true)
        let mut partition = HashMap::<V, (V, F64Dist, usize)>::new();
        for (u, (v, w, label)) in bridges.iter() {
            let u_heads = flips.get(u).copied().unwrap_or(false);
            let v_heads = flips.get(v).copied().unwrap_or(false);

            // Contract if u is Tail and v is Head
            if !u_heads && v_heads {
                let _ = partition.insert(u.clone(), (v.clone(), *w, *label));
            }
        }

        // Remaining vertices = all vertices minus contracted tails
        let mut remaining = SetLit![];
        for v in vertices.iter() {
            if !partition.contains_key(v) {
                let _ = remaining.insert(v.clone());
            }
        }

        (remaining, partition)
    }

    /// Algorithm 66.3: Borůvka's MST
    ///
    /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
    /// Returns the set of edge labels in the MST.
    ///
    /// - APAS: Work O(m log n), Span O(log² n)
    /// - Claude-Opus-4.6: Work O(m log n), Span O(m log n) — sequential; O(log n) rounds each doing O(m) work sequentially.
    #[cfg(not(verus_keep_ghost))]
    pub fn boruvka_mst<V: StT + Hash + Ord>(
        vertices: &SetStEph<V>,
        edges: &SetStEph<LabeledEdge<V>>,
        mst_labels: SetStEph<usize>,
        rng: &mut StdRng,
    ) -> SetStEph<usize> {
        // Base case: no edges remaining
        if edges.size() == 0 {
            return mst_labels;
        }

        // Find vertex bridges
        let bridges = vertex_bridges(edges);

        // Perform bridge star partition
        let (remaining_vertices, partition) = bridge_star_partition(vertices, &bridges, rng);

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

        // Re-route edges to new endpoints, removing self-edges
        let mut new_edges = SetLit![];
        for LabeledEdge(u, v, w, label) in edges.iter() {
            let new_u = full_partition.get(u).cloned().unwrap_or_else(|| u.clone());
            let new_v = full_partition.get(v).cloned().unwrap_or_else(|| v.clone());

            // Skip self-edges
            if new_u != new_v {
                let _ = new_edges.insert(LabeledEdge(new_u, new_v, *w, *label));
            }
        }

        // Recurse
        boruvka_mst(&remaining_vertices, &new_edges, new_mst_labels, rng)
    }

    /// Create Borůvka MST with a specific seed.
    /// Wrapper that initializes RNG and delegates to `boruvka_mst`.
    ///
    /// - APAS: Work O(m log n), Span O(log² n)
    /// - Claude-Opus-4.6: Work O(m log n), Span O(m log n) — delegates to sequential boruvka_mst.
    #[cfg(not(verus_keep_ghost))]
    pub fn boruvka_mst_with_seed<V: StT + Hash + Ord>(
        vertices: &SetStEph<V>,
        edges: &SetStEph<LabeledEdge<V>>,
        seed: u64,
    ) -> SetStEph<usize> {
        let mut rng = StdRng::seed_from_u64(seed);
        boruvka_mst(vertices, edges, SetLit![], &mut rng)
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
