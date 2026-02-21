//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Kruskal's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.2: Kruskal's algorithm for computing Minimum Spanning Trees.
//! Uses Union-Find data structure for efficient cycle detection.

pub mod KruskalStEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap65::UnionFindStEph::UnionFindStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    pub type T<V> = LabUnDirGraphStEph<V, WrappedF64>;

    verus! {
        pub trait KruskalStEphTrait {
            /// Kruskal's MST algorithm
            /// APAS: Work O(m log m), Span O(m log m) where m = |E|
            fn kruskal_mst<V: StT + Hash + Ord>(
                graph: &LabUnDirGraphStEph<V, WrappedF64>,
            ) -> SetStEph<LabEdge<V, WrappedF64>>;

            /// Compute total weight of MST
            /// APAS: Work O(m), Span O(1)
            fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, WrappedF64>>) -> WrappedF64;

            /// Verify MST has correct size
            /// APAS: Work O(1), Span O(1)
            fn verify_mst_size<V: StT + Hash + Ord>(
                graph: &LabUnDirGraphStEph<V, WrappedF64>,
                mst: &SetStEph<LabEdge<V, WrappedF64>>,
            ) -> B;
        }
    }

    /// Algorithm 65.2: Kruskal's MST Algorithm
    ///
    /// Computes the Minimum Spanning Tree by sorting edges and greedily adding them.
    /// Uses Union-Find to detect cycles efficiently.
    ///
    /// Algorithm:
    /// 1. Sort edges by weight
    /// 2. For each edge (u,v) in sorted order:
    ///    - If find(u) != find(v): add edge to MST, union(u,v)
    ///    - Else: skip (would create cycle)
    ///
    /// - APAS: Work O(m lg n), Span O(m lg n)
    /// - Claude-Opus-4.6: Work O(m lg m), Span O(m lg m) — sorting dominates; sequential
    #[cfg(not(verus_keep_ghost))]
    pub fn kruskal_mst<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
    ) -> SetStEph<LabEdge<V, WrappedF64>> {
        let mut mst_edges = SetLit![];

        // Initialize Union-Find with all vertices
        let mut uf = UnionFindStEph::new();
        for vertex in graph.vertices().iter() {
            uf.insert(vertex.clone());
        }

        // Sort edges by weight
        let mut edges_vec = graph.labeled_edges().iter().cloned().collect::<Vec<LabEdge<V, WrappedF64>>>();
        edges_vec.sort_by(|e1, e2| {
            let LabEdge(_u1, _v1, w1) = e1;
            let LabEdge(_u2, _v2, w2) = e2;
            w1.cmp(w2)
        });

        // Greedily add edges that don't form cycles
        for edge in edges_vec.iter() {
            let LabEdge(u, v, _w) = edge;

            // Check if u and v are in different components
            if !uf.equals(u, v) {
                // Add edge to MST
                let _ = mst_edges.insert(edge.clone());
                // Union the components
                uf.union(u, v);
            }
            // Else: skip edge (would form cycle)
        }

        mst_edges
    }

    /// Compute total MST weight.
    /// - APAS: (no cost stated) — utility function
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    #[cfg(not(verus_keep_ghost))]
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, WrappedF64>>) -> WrappedF64 {
        let mut total = zero_dist();
        for edge in mst_edges.iter() {
            let LabEdge(_u, _v, w) = edge;
            total += *w;
        }
        total
    }

    /// Verify MST has correct number of edges.
    /// A valid MST of n vertices should have n-1 edges.
    /// - APAS: (no cost stated) — validation utility
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[cfg(not(verus_keep_ghost))]
    pub fn verify_mst_size<V: StT + Hash + Ord>(
        n_vertices: N,
        mst_edges: &SetStEph<LabEdge<V, WrappedF64>>,
    ) -> B {
        let expected_edges = if n_vertices > 0 { n_vertices - 1 } else { 0 };
        mst_edges.size() == expected_edges
    }
}
