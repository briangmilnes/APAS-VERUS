//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Vertex Matching - Multi-threaded Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.4: Parallel Vertex Matching (randomized with fork/join)

pub mod VertexMatchingMtEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use std::sync::Arc;
    #[cfg(not(verus_keep_ghost))]
    use std::vec::Vec;
    #[cfg(not(verus_keep_ghost))]
    use crate::{ParaPair, SetLit};

    verus! {
        pub trait VertexMatchingMtEphTrait {
            /// Parallel vertex matching using randomized symmetry breaking
            /// APAS: Work O(|E|), Span O(lg |V|)
            fn parallel_matching_mt<V: StT + MtT + Hash + 'static>(graph: &UnDirGraphMtEph<V>) -> SetStEph<Edge<V>>;
        }
    } // verus!

    /// Algorithm 61.4: Parallel Vertex Matching
    ///
    /// Computes a vertex matching using randomized symmetry breaking.
    /// For each edge, flips a coin in parallel. Selects an edge if:
    /// - Its coin is heads (true)
    /// - All edges incident on its endpoints flipped tails (false)
    ///
    /// - APAS: Work O(|E|), Span O(lg |V|)
    /// - Claude-Opus-4.6: Work Θ(|E|²), Span Θ(|E|) — coin flip phase is sequential (RNG),
    ///   edge selection scans all edges per candidate via should_select_edge
    ///
    /// Phase 1: Flip coins for all edges — sequential (RNG is inherently sequential)
    /// Phase 2: Select edges based on local maxima — parallel divide-and-conquer
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for reproducibility
    ///
    /// Returns:
    /// - A set of edges forming a vertex matching
    #[cfg(not(verus_keep_ghost))]
    pub fn parallel_matching_mt<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> SetStEph<Edge<V>> {
        use rand::rngs::StdRng;
        use rand::{Rng, RngExt, SeedableRng};
        use std::sync::{Arc, Mutex};

        let mut rng = StdRng::seed_from_u64(seed);

        let edges_vec = graph.edges().iter().cloned().collect::<Vec<Edge<V>>>();
        let edges_seq = ArraySeqStEphS::<Edge<V>>::from_vec(edges_vec);
        let n_edges = edges_seq.length();

        if n_edges == 0 {
            return SetLit![];
        }

        let coins = flip_coins_parallel(&edges_seq, &mut rng);

        select_edges_parallel(graph, &edges_seq, &coins)
    }

    /// Phase 1: Flip coins for all edges
    ///
    /// - APAS: Work Θ(|E|), Span Θ(1) — each coin is independent
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(|E|) — RNG is sequential, no actual parallelism
    #[cfg(not(verus_keep_ghost))]
    fn flip_coins_parallel<V: StT + MtT + 'static>(
        edges: &ArraySeqStEphS<Edge<V>>,
        rng: &mut rand::rngs::StdRng,
    ) -> ArraySeqStEphS<B> {
        use rand::{Rng, RngExt};

        let n = edges.length();
        if n == 0 {
            return ArraySeqStEphS::empty();
        }

        let mut coins_vec = Vec::with_capacity(n);
        for _ in 0..n {
            coins_vec.push(rng.random());
        }

        ArraySeqStEphS::from_vec(coins_vec)
    }

    /// Phase 2: Select edges in parallel where coin is heads and adjacent edges are tails
    ///
    /// - APAS: Work O(|E|), Span O(lg |V|) — each edge checks only incident edges
    /// - Claude-Opus-4.6: Work Θ(|E|²), Span Θ(lg |E| + |E|) — should_select_edge scans all |E| edges
    #[cfg(not(verus_keep_ghost))]
    fn select_edges_parallel<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        edges: &ArraySeqStEphS<Edge<V>>,
        coins: &ArraySeqStEphS<B>,
    ) -> SetStEph<Edge<V>> {
        use std::sync::Arc;
        pub type T<V> = UnDirGraphMtEph<V>;

        let n = edges.length();
        if n == 0 {
            return SetLit![];
        }

        let edge_coin_map = edges.iter().zip(coins.iter()).map(|(e, c)| (e.clone(), *c)).collect::<HashMap<Edge<V>, bool>>();

        let graph_arc = Arc::new(graph.clone());
        let edges_arc = Arc::new(edges.clone());
        let map_arc = Arc::new(edge_coin_map);

        let selected = select_edges_recursive(graph_arc, edges_arc, map_arc, 0, n);

        let mut result: SetStEph<Edge<V>> = SetLit![];
        for edge in selected.iter() {
            let _ = result.insert(edge.clone());
        }
        result
    }

    /// - APAS: N/A — Verus-specific scaffolding (parallel recursion helper)
    /// - Claude-Opus-4.6: Work Θ(k × |E|), Span Θ(lg k + |E|) — each base case calls should_select_edge which is Θ(|E|)
    #[cfg(not(verus_keep_ghost))]
    fn select_edges_recursive<V: StT + MtT + Hash + 'static>(
        graph: Arc<UnDirGraphMtEph<V>>,
        edges: Arc<ArraySeqStEphS<Edge<V>>>,
        edge_coins: Arc<HashMap<Edge<V>, bool>>,
        start: usize,
        end: usize,
    ) -> ArraySeqStEphS<Edge<V>> {
        let size = end - start;

        if size == 0 {
            return ArraySeqStEphS::empty();
        }

        if size == 1 {
            let edge = edges.nth(start as N);
            if should_select_edge(&graph, edge, &edge_coins) {
                return ArraySeqStEphS::from_vec(std::vec![edge.clone()]);
            } else {
                return ArraySeqStEphS::empty();
            }
        }

        let mid = start + size / 2;

        let graph1 = graph.clone();
        let edges1 = edges.clone();
        let coins1 = edge_coins.clone();
        let graph2 = graph;
        let edges2 = edges;
        let coins2 = edge_coins;

        let pair = ParaPair!(
            move || select_edges_recursive(graph1, edges1, coins1, start, mid),
            move || select_edges_recursive(graph2, edges2, coins2, mid, end)
        );

        let mut left_vec = pair.0.iter().cloned().collect::<Vec<Edge<V>>>();
        let right_vec = pair.1.iter().cloned().collect::<Vec<Edge<V>>>();
        left_vec.extend(right_vec);
        ArraySeqStEphS::from_vec(left_vec)
    }

    /// Check if an edge should be selected
    ///
    /// Edge is selected if its coin is heads and all adjacent edges have tails
    ///
    /// - APAS: Work O(degree(u) + degree(v)), Span O(degree(u) + degree(v)) — checks only incident edges
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(|E|) — iterates all edges, not just incident ones
    #[cfg(not(verus_keep_ghost))]
    fn should_select_edge<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        edge: &Edge<V>,
        edge_coins: &HashMap<Edge<V>, bool>,
    ) -> bool {
        let Edge(u, v) = edge;

        if !edge_coins.get(edge).copied().unwrap_or(false) {
            return false;
        }

        for adj_edge in graph.edges().iter() {
            if adj_edge == edge {
                continue;
            }

            if (graph.incident(adj_edge, u) || graph.incident(adj_edge, v))
                && edge_coins.get(adj_edge).copied().unwrap_or(false)
            {
                return false;
            }
        }

        true
    }
}
