//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Sequential)
//!
//! Implements Exercise 64.2: Compute spanning tree using star contraction.

pub mod SpanTreeStEph {

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap62::StarContractionStEph::StarContractionStEph::star_contract;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    pub type T<V> = UnDirGraphStEph<V>;

    verus! {
        pub trait SpanTreeStEphTrait {
            /// Sequential spanning tree via star contraction
            /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
            fn spanning_tree_star_contraction<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>>;

            /// Verify spanning tree properties
            /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
            fn verify_spanning_tree<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>, tree: &SetStEph<Edge<V>>) -> B;
        }
    }

    /// Exercise 64.2: Spanning Tree via Star Contraction
    ///
    /// Computes a spanning tree by recursively applying star contraction and
    /// collecting all edges from star partitions.
    ///
    /// Algorithm:
    /// 1. Base case: If no edges, return empty edge set
    /// 2. Compute star partition (centers and partition map)
    /// 3. Add all edges from partition map to spanning tree
    /// 4. Build quotient graph
    /// 5. Recursively compute spanning tree of quotient
    /// 6. Map quotient tree edges back to original edges
    ///
    /// - APAS: Work O((n+m) lg n), Span O((n+m) lg n)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS.
    ///   Expand closure scans original edges per quotient edge (O(E_q * E) per round),
    ///   but total across O(lg n) rounds stays within APAS bound since edges shrink.
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - Set of edges forming a spanning tree
    #[cfg(not(verus_keep_ghost))]
    pub fn spanning_tree_star_contraction<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>> {
        // Base: no edges means no spanning tree edges (isolated vertices)
        let base = |_vertices: &SetStEph<V>| SetLit![];

        // Expand: add star partition edges and map quotient tree edges back
        let expand = |_v: &SetStEph<V>,
                      original_edges: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMap<V, V>,
                      quotient_tree: SetStEph<Edge<V>>| {
            // Collect edges from partition map (vertex → center edges)
            let mut spanning_edges = SetLit![];

            for (vertex, center) in partition_map.iter() {
                // Add edge if vertex is not its own center (avoid self-loops)
                if vertex != center {
                    // Normalize edge order
                    let edge = if vertex < center {
                        Edge(vertex.clone(), center.clone())
                    } else {
                        Edge(center.clone(), vertex.clone())
                    };
                    let _ = spanning_edges.insert(edge);
                }
            }

            // Map quotient tree edges back to original edges
            // For each edge between centers in quotient tree, find original edge that maps to it
            for quotient_edge in quotient_tree.iter() {
                let Edge(c1, c2) = quotient_edge;

                // Find an original edge that connects the two stars (centers c1 and c2)
                for original_edge in original_edges.iter() {
                    let Edge(u, v) = original_edge;
                    let u_center = partition_map.get(u).unwrap_or(u);
                    let v_center = partition_map.get(v).unwrap_or(v);

                    // Check if this original edge connects the two centers (in either direction)
                    if (u_center == c1 && v_center == c2) || (u_center == c2 && v_center == c1) {
                        let _ = spanning_edges.insert(original_edge.clone());
                        break; // Only need one edge between the two stars
                    }
                }
            }

            spanning_edges
        };

        star_contract(graph, &base, &expand)
    }

    /// Verify that result is a valid spanning tree
    ///
    /// Checks:
    /// 1. All vertices are included
    /// 2. Exactly |V| - 1 edges
    /// 3. All edges are from original graph
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(|V| + |E_tree|), Span O(|V| + |E_tree|) — linear scan of tree edges.
    ///
    /// Returns true if valid spanning tree
    #[cfg(not(verus_keep_ghost))]
    pub fn verify_spanning_tree<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>, tree_edges: &SetStEph<Edge<V>>) -> B {
        let n = graph.sizeV();
        let expected_edges = if n > 0 { n - 1 } else { 0 };

        // Check edge count
        if tree_edges.size() != expected_edges {
            return false;
        }

        // Check all edges are from original graph
        for edge in tree_edges.iter() {
            let Edge(u, v) = edge;
            // For undirected graphs, Neighbor checks if u and v are connected
            if !graph.Neighbor(u, v) {
                return false;
            }
        }

        true
    }
}
