//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Vertex Matching - Multi-threaded Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.4: Parallel Vertex Matching (randomized with fork/join)

pub mod VertexMatchingMtEph {

    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::Arc;
    use std::vec::Vec;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;

    pub trait VertexMatchingMtEphTrait {
        /// Parallel vertex matching using randomized symmetry breaking
        /// APAS: Work O(|E|), Span O(lg |V|)
        fn parallel_matching_mt<V: StT + MtT + Hash + 'static>(graph: &UnDirGraphMtEph<V>) -> SetStEph<Edge<V>>;
    }

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
    pub fn parallel_matching_mt<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> SetStEph<Edge<V>> {
    use rand::rngs::StdRng;
    use rand::{Rng, RngExt, SeedableRng};
        use std::sync::{Arc, Mutex};

        let mut rng = StdRng::seed_from_u64(seed);

        // Convert edges to a sequence for parallel processing
        let edges_vec = graph.edges().iter().cloned().collect::<Vec<Edge<V>>>();
        let edges_seq = ArraySeqStEphS::<Edge<V>>::from_vec(edges_vec);
        let n_edges = edges_seq.length();

        if n_edges == 0 {
            return SetLit![];
        }

        // Phase 1: Flip coins for all edges in parallel
        let coins = flip_coins_parallel(&edges_seq, &mut rng);

        // Phase 2: Select edges where coin is heads and all adjacent edges are tails

        select_edges_parallel(graph, &edges_seq, &coins)
    }

    /// Phase 1: Flip coins for all edges
    ///
    /// - APAS: Work Θ(|E|), Span Θ(1) — each coin is independent
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(|E|) — RNG is sequential, no actual parallelism
    fn flip_coins_parallel<V: StT + MtT + 'static>(
        edges: &ArraySeqStEphS<Edge<V>>,
        rng: &mut rand::rngs::StdRng,
    ) -> ArraySeqStEphS<B> {
        use rand::{Rng, RngExt};

        let n = edges.length();
        if n == 0 {
            return ArraySeqStEphS::empty();
        }

        // Generate all random values sequentially (RNG must be sequential)
        let mut coins_vec = Vec::with_capacity(n);
        for _ in 0..n {
            coins_vec.push(rng.random());
        }

        // Convert to sequence
        ArraySeqStEphS::from_vec(coins_vec)
    }

    /// Phase 2: Select edges in parallel where coin is heads and adjacent edges are tails
    ///
    /// - APAS: Work O(|E|), Span O(lg |V|) — each edge checks only incident edges
    /// - Claude-Opus-4.6: Work Θ(|E|²), Span Θ(lg |E| + |E|) — should_select_edge scans all |E| edges
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

        // Build edge index for O(1) coin lookups
        let edge_coin_map = edges.iter().zip(coins.iter()).map(|(e, c)| (e.clone(), *c)).collect::<HashMap<Edge<V>, bool>>();

        // Wrap in Arc for thread-safe sharing
        let graph_arc = Arc::new(graph.clone());
        let edges_arc = Arc::new(edges.clone());
        let map_arc = Arc::new(edge_coin_map);

        // Parallel edge selection using divide-and-conquer
        let selected = select_edges_recursive(graph_arc, edges_arc, map_arc, 0, n);

        // Convert sequence to set
        let mut result: SetStEph<Edge<V>> = SetLit![];
        for edge in selected.iter() {
            let _ = result.insert(edge.clone());
        }
        result
    }

    /// - APAS: N/A — Verus-specific scaffolding (parallel recursion helper)
    /// - Claude-Opus-4.6: Work Θ(k × |E|), Span Θ(lg k + |E|) — each base case calls should_select_edge which is Θ(|E|)
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
            // Base case: check single edge
            let edge = edges.nth(start as N);
            if should_select_edge(&graph, edge, &edge_coins) {
                return ArraySeqStEphS::from_vec(std::vec![edge.clone()]);
            } else {
                return ArraySeqStEphS::empty();
            }
        }

        // Recursive case: divide and conquer
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

        // Combine results
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
    fn should_select_edge<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        edge: &Edge<V>,
        edge_coins: &HashMap<Edge<V>, bool>,
    ) -> bool {
        let Edge(u, v) = edge;

        // Check if this edge flipped heads
        if !edge_coins.get(edge).copied().unwrap_or(false) {
            return false;
        }

        // Check if all edges incident on u and v flipped tails (except this one)
        for adj_edge in graph.edges().iter() {
            if adj_edge == edge {
                continue; // Skip the current edge
            }

            // Check if adjacent edge is incident on u or v
            if (graph.incident(adj_edge, u) || graph.incident(adj_edge, v))
                && edge_coins.get(adj_edge).copied().unwrap_or(false)
            {
                return false; // Adjacent edge flipped heads
            }
        }

        true
    }
}
