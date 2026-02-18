//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Vertex Matching - Sequential Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.3: Greedy Vertex Matching (sequential)
//! - Baseline sequential version of parallel matching algorithm

pub mod VertexMatchingStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    verus! {
        pub trait VertexMatchingStEphTrait {
            /// Greedy vertex matching algorithm
            /// APAS: Work Θ(|E|), Span Θ(|E|)
            fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>>;

            /// Sequential version of parallel matching
            /// APAS: Work Θ(|E|), Span Θ(|E|)
            fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> SetStEph<Edge<V>>;
        }
    } // verus!

    /// Algorithm 61.3: Greedy Vertex Matching
    ///
    /// Iterates over edges sequentially, adding each edge to the matching
    /// if neither endpoint is already matched.
    ///
    /// - APAS: Work Θ(|E|), Span Θ(|E|)
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(|E|) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - A set of edges forming a vertex matching (no two edges share an endpoint)
    #[cfg(not(verus_keep_ghost))]
    pub fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>> {
        let mut matching: SetStEph<Edge<V>> = SetLit![];
        let mut matched_vertices: SetStEph<V> = SetLit![];

        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;

            if !matched_vertices.mem(u) && !matched_vertices.mem(v) {
                let _ = matching.insert(edge.clone());
                let _ = matched_vertices.insert(u.clone());
                let _ = matched_vertices.insert(v.clone());
            }
        }

        matching
    }

    /// Baseline Sequential Version of Parallel Matching
    ///
    /// Simulates the parallel matching algorithm (Algorithm 61.4) sequentially
    /// by flipping a coin for each edge and selecting edges where:
    /// - The coin is heads (probability 1/2)
    /// - All adjacent edges are tails
    ///
    /// - APAS: (no cost stated — sequential baseline of Algorithm 61.4)
    /// - Claude-Opus-4.6: Work Θ(|E|²), Span Θ(|E|²) — for each edge, scans all edges for adjacency check
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for reproducibility
    ///
    /// Returns:
    /// - A set of edges forming a vertex matching
    #[cfg(not(verus_keep_ghost))]
    pub fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> SetStEph<Edge<V>> {
        use rand::rngs::StdRng;
        use rand::{Rng, RngExt, SeedableRng};
        pub type T<V> = UnDirGraphStEph<V>;

        let mut rng = StdRng::seed_from_u64(seed);
        let mut matching: SetStEph<Edge<V>> = SetLit![];

        let mut edge_coins = HashMap::<Edge<V>, bool>::new();

        for edge in graph.edges().iter() {
            edge_coins.insert(edge.clone(), rng.random());
        }

        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;

            if !edge_coins.get(edge).copied().unwrap_or(false) {
                continue;
            }

            let mut all_adjacent_tails = true;

            for adj_edge in graph.edges().iter() {
                if adj_edge == edge {
                    continue;
                }

                if (graph.incident(adj_edge, u) || graph.incident(adj_edge, v))
                    && edge_coins.get(adj_edge).copied().unwrap_or(false)
                {
                    all_adjacent_tails = false;
                    break;
                }
            }

            if all_adjacent_tails {
                let _ = matching.insert(edge.clone());
            }
        }

        matching
    }
}
