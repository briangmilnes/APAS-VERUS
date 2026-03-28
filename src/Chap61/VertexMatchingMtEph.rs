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

    use std::hash::Hash;
    use std::sync::Arc;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::rand::rand::{seeded_rng, random_bool_seeded};
    use std::vec::Vec;
    use crate::{ParaPair, SetLit};

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct VertexMatchingMtEph;

    // 8. traits

    pub trait VertexMatchingMtEphTrait {
        /// Well-formedness for parallel vertex matching algorithm input.
        open spec fn spec_vertexmatchingmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel vertex matching using randomized symmetry breaking.
        /// APAS: Work O(|E|), Span O(lg |V|)
        fn parallel_matching_mt<V: StT + MtT + Hash + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> SetStEph<Edge<V>>
            requires Self::spec_vertexmatchingmteph_wf(graph);
    }

    /// Algorithm 61.4: Parallel Vertex Matching
    ///
    /// Computes a vertex matching using randomized symmetry breaking.
    /// For each edge, flips a coin in parallel. Selects an edge if:
    /// - Its coin is heads (true)
    /// - All edges incident on its endpoints flipped tails (false)
    ///
    /// - APAS: Work O(|E|), Span O(lg |V|)
    /// - Claude-Opus-4.6: Work Θ(|E|^2), Span Θ(|E|) — coin flip phase is sequential (RNG),
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
    ) -> (matching: SetStEph<Edge<V>>)
        requires
            valid_key_type_Edge::<V>(),
            graph.E.spec_setsteph_wf(),
        ensures true,
    {
        let edges_vec = graph.E.to_seq();
        let edges_seq = ArraySeqStEphS::<Edge<V>>::from_vec(edges_vec);
        let n_edges = edges_seq.length();

        if n_edges == 0 {
            return SetLit![];
        }

        let coins = flip_coins_parallel(&edges_seq, seed);

        select_edges_parallel(graph, &edges_seq, &coins)
    }

    /// Phase 1: Flip coins for all edges
    ///
    /// - APAS: Work Θ(|E|), Span Θ(1) — each coin is independent
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(|E|) — RNG is sequential, no actual parallelism
    fn flip_coins_parallel<V: StT + MtT + Hash + 'static>(
        edges: &ArraySeqStEphS<Edge<V>>,
        seed: u64,
    ) -> (coins: ArraySeqStEphS<bool>)
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        let n = edges.length();
        if n == 0 {
            return ArraySeqStEphS::empty();
        }

        let mut rng = seeded_rng(seed);
        let mut coins_vec: Vec<bool> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                n == edges@.len(),
                i <= n,
            decreases n - i,
        {
            coins_vec.push(random_bool_seeded(&mut rng));
            i = i + 1;
        }

        ArraySeqStEphS::from_vec(coins_vec)
    }

    /// Phase 2: Select edges in parallel where coin is heads and adjacent edges are tails
    ///
    /// - APAS: Work O(|E|), Span O(lg |V|) — each edge checks only incident edges
    /// - Claude-Opus-4.6: Work Θ(|E|^2), Span Θ(lg |E| + |E|) — should_select_edge scans all |E| edges
    #[verifier::external_body]
    fn select_edges_parallel<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        edges: &ArraySeqStEphS<Edge<V>>,
        coins: &ArraySeqStEphS<bool>,
    ) -> SetStEph<Edge<V>> {
        use std::sync::Arc;
        pub type T<V> = UnDirGraphMtEph<V>;

        let n = edges.length();
        if n == 0 {
            return SetLit![];
        }

        let mut edge_coin_map = HashMapWithViewPlus::<Edge<V>, bool>::new();
        for (e, c) in edges.iter().zip(coins.iter()) {
            edge_coin_map.insert(e.clone(), *c);
        }

        let graph_arc = Arc::new(graph.clone());
        let edges_arc = Arc::new(edges.clone());
        let map_arc = Arc::new(edge_coin_map);

        let selected = select_edges_recursive(graph_arc, edges_arc, map_arc, 0, n);

        let mut edges: SetStEph<Edge<V>> = SetLit![];
        for edge in selected.iter() {
            let _ = edges.insert(edge.clone());
        }
        edges
    }

    /// - APAS: N/A — Verus-specific scaffolding (parallel recursion helper)
    /// - Claude-Opus-4.6: Work Θ(k * |E|), Span Θ(lg k + |E|) — each base case calls should_select_edge which is Θ(|E|)
    #[verifier::external_body]
    fn select_edges_recursive<V: StT + MtT + Hash + 'static>(
        graph: Arc<UnDirGraphMtEph<V>>,
        edges: Arc<ArraySeqStEphS<Edge<V>>>,
        edge_coins: Arc<HashMapWithViewPlus<Edge<V>, bool>>,
        start: usize,
        end: usize,
    ) -> ArraySeqStEphS<Edge<V>> {
        let size = end - start;

        if size == 0 {
            return ArraySeqStEphS::empty();
        }

        if size == 1 {
            let edge = edges.nth(start as usize);
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
    fn should_select_edge<V: StT + MtT + Hash + 'static>(
        graph: &UnDirGraphMtEph<V>,
        edge: &Edge<V>,
        edge_coins: &HashMapWithViewPlus<Edge<V>, bool>,
    ) -> (selected: bool)
        requires
            valid_key_type_Edge::<V>(),
            graph.E.spec_setsteph_wf(),
        ensures true,
    {
        let Edge(u, v) = edge;

        let this_coin = match edge_coins.get(edge) {
            Some(val) => *val,
            None => false,
        };
        if !this_coin {
            return false;
        }

        let edges_ref = graph.edges();
        let edges_it = edges_ref.iter();
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for adj_edge in iter: edges_it
            invariant
                valid_key_type_Edge::<V>(),
                graph.E.spec_setsteph_wf(),
        {
            if !(adj_edge.0 == edge.0 && adj_edge.1 == edge.1) {
                if graph.incident(adj_edge, u) || graph.incident(adj_edge, v) {
                    let adj_coin = match edge_coins.get(adj_edge) {
                        Some(val) => *val,
                        None => false,
                    };
                    if adj_coin {
                        return false;
                    }
                }
            }
        }

        true
    }

    } // verus!
}
