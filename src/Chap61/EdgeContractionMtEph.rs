//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Edge Contraction - Multi-threaded Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.6: Parallel Edge Contraction (with fork/join parallelism)

pub mod EdgeContractionMtEph {

    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::Arc;
    use std::vec::Vec;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap61::VertexMatchingMtEph::VertexMatchingMtEph::parallel_matching_mt;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;

    pub trait EdgeContractionMtEphTrait {
        /// Parallel edge contraction algorithm
        /// APAS: Work O(|E|), Span O(lg |V|)
        fn edge_contract_mt<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
            matching: &SetStEph<Edge<V>>,
        ) -> UnDirGraphMtEph<V>;

        /// Single round of parallel edge contraction
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        fn contract_round_mt<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> UnDirGraphMtEph<V>;
    }

    /// Algorithm 61.6: Parallel Edge Contraction
    ///
    /// Contracts edges in a matching by merging their endpoints in parallel.
    /// Each edge in the matching forms a block of two vertices.
    /// Unmatched vertices form singleton blocks.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(lg |V|)
    /// - Claude-Opus-4.6: Work Θ(|V| + |E|), Span Θ(|V| + |E|) — Phases 1-2 are sequential loops;
    ///   only Phase 3 (build_edges_parallel) is parallel
    ///
    /// Phase 1: Build vertex-to-block mapping — sequential
    /// Phase 2: Build new vertex set — sequential
    /// Phase 3: Build new edge set — parallel via divide-and-conquer
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - matching: A vertex matching (set of edges where no two share an endpoint)
    ///
    /// Returns:
    /// - Contracted graph where matched edges are merged into single vertices
    pub fn edge_contract_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        matching: &SetStEph<Edge<V>>,
    ) -> UnDirGraphMtEph<V> {
        use std::sync::{Arc, Mutex};
        pub type T<V> = UnDirGraphMtEph<V>;

        // Create a mapping from original vertices to their block representatives
        let vertex_to_block = Arc::new(Mutex::new(HashMap::new()));

        // Phase 1: Assign block representatives for matched edges (sequential for now)
        {
            let mut map = vertex_to_block.lock().unwrap();
            for edge in matching.iter() {
                let Edge(u, v) = edge;
                map.insert(u.clone(), u.clone());
                map.insert(v.clone(), u.clone());
            }

            // For unmatched vertices, they are their own representatives
            for vertex in graph.vertices().iter() {
                if !map.contains_key(vertex) {
                    map.insert(vertex.clone(), vertex.clone());
                }
            }
        }

        let vertex_to_block = Arc::try_unwrap(vertex_to_block).unwrap().into_inner().unwrap();

        // Phase 2: Build new vertex set (representatives)
        let mut new_vertices: SetStEph<V> = SetLit![];
        for representative in vertex_to_block.values() {
            let _ = new_vertices.insert(representative.clone());
        }

        // Phase 3: Build new edge set in parallel
        let edges_vec = graph.edges().iter().cloned().collect::<Vec<Edge<V>>>();
        let edges_seq = ArraySeqStEphS::from_vec(edges_vec);
        let n_edges = edges_seq.length();
        let edges_arc = Arc::new(edges_seq);
        let vertex_map_arc = Arc::new(vertex_to_block);

        let new_edges_set = build_edges_parallel(edges_arc, vertex_map_arc, 0, n_edges);

        <UnDirGraphMtEph<V> as UnDirGraphMtEphTrait<V>>::from_sets(new_vertices, new_edges_set)
    }

    /// Build new edge set in parallel using divide-and-conquer
    ///
    /// - APAS: N/A — Verus-specific scaffolding (parallel edge routing helper)
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(lg |E|) — genuine divide-and-conquer parallelism
    fn build_edges_parallel<V: StT + MtT + Hash + Ord + 'static>(
        edges: Arc<ArraySeqStEphS<Edge<V>>>,
        vertex_map: Arc<HashMap<V, V>>,
        start: usize,
        end: usize,
    ) -> SetStEph<Edge<V>> {
        let size = end - start;

        if size == 0 {
            return SetLit![];
        }

        if size == 1 {
            // Base case: process single edge
            let edge = edges.nth(start as N);
            let Edge(u, v) = edge;
            let block_u = vertex_map.get(u).unwrap().clone();
            let block_v = vertex_map.get(v).unwrap().clone();

            if block_u != block_v {
                let new_edge = if block_u < block_v {
                    Edge(block_u, block_v)
                } else {
                    Edge(block_v, block_u)
                };
                let mut result: SetStEph<Edge<V>> = SetLit![];
                let _ = result.insert(new_edge);
                return result;
            } else {
                return SetLit![];
            }
        }

        // Recursive case: divide and conquer
        let mid = start + size / 2;

        let edges1 = edges.clone();
        let map1 = vertex_map.clone();
        let edges2 = edges;
        let map2 = vertex_map;

        let pair = ParaPair!(move || build_edges_parallel(edges1, map1, start, mid), move || {
            build_edges_parallel(edges2, map2, mid, end)
        });

        // Combine results (union of sets)
        let mut result = pair.0;
        for edge in pair.1.iter() {
            let _ = result.insert(edge.clone());
        }
        result
    }

    /// One round of parallel edge contraction
    ///
    /// Computes a parallel matching and contracts it.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(lg |V|)
    /// - Claude-Opus-4.6: Work Θ(|E|²), Span Θ(|E|) — dominated by parallel_matching_mt's
    ///   should_select_edge scanning all edges
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for matching
    ///
    /// Returns:
    /// - Contracted graph
    pub fn contract_round_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> UnDirGraphMtEph<V> {
        let matching = parallel_matching_mt(graph, seed);
        edge_contract_mt(graph, &matching)
    }
}
