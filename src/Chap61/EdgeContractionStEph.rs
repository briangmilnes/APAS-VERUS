//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Edge Contraction - Sequential Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.6: Parallel Edge Contraction (Sequential version)
//! - One round of contraction using greedy matching

pub mod EdgeContractionStEph {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap61::VertexMatchingStEph::VertexMatchingStEph::greedy_matching;
    use crate::SetLit;
    use crate::Types::Types::*;
    pub type T<V> = UnDirGraphStEph<V>;

    pub trait EdgeContractionStEphTrait {
        /// Sequential edge contraction algorithm
        /// APAS: Work O(|E|), Span O(|E|)
        fn edge_contract<V: StT + Hash + Ord>(
            graph: &UnDirGraphStEph<V>,
            matching: &SetStEph<Edge<V>>,
        ) -> UnDirGraphStEph<SetStEph<V>>;

        /// Single round of sequential edge contraction
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn contract_round<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> UnDirGraphStEph<V>;
    }

    /// Algorithm 61.6: Sequential Edge Contraction
    ///
    /// Contracts edges in a matching by merging their endpoints.
    /// Each edge in the matching forms a block of two vertices.
    /// Unmatched vertices form singleton blocks.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work Θ(|V| + |E|), Span Θ(|V| + |E|) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - matching: A vertex matching (set of edges where no two share an endpoint)
    ///
    /// Returns:
    /// - Contracted graph where matched edges are merged into single vertices
    pub fn edge_contract<V: StT + Hash + Ord>(
        graph: &UnDirGraphStEph<V>,
        matching: &SetStEph<Edge<V>>,
    ) -> UnDirGraphStEph<V> {
        // Create a mapping from original vertices to their block representatives
        let mut vertex_to_block = HashMap::<V, V>::new();

        // For each edge in the matching, assign both endpoints to the same representative
        for edge in matching.iter() {
            let Edge(u, v) = edge;
            // Use the first vertex as the representative
            vertex_to_block.insert(u.clone(), u.clone());
            vertex_to_block.insert(v.clone(), u.clone());
        }

        // For unmatched vertices, they are their own representatives
        for vertex in graph.vertices().iter() {
            if !vertex_to_block.contains_key(vertex) {
                vertex_to_block.insert(vertex.clone(), vertex.clone());
            }
        }

        // Build the new contracted graph
        let mut new_vertices: SetStEph<V> = SetLit![];
        let mut new_edges: SetStEph<Edge<V>> = SetLit![];

        // Add all block representatives as vertices
        for representative in vertex_to_block.values() {
            let _ = new_vertices.insert(representative.clone());
        }

        // For each edge in the original graph, add a new edge between block representatives
        // (unless both endpoints are in the same block)
        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;
            let block_u = vertex_to_block.get(u).unwrap().clone();
            let block_v = vertex_to_block.get(v).unwrap().clone();

            // Only add edge if endpoints are in different blocks (no self-loops)
            if block_u != block_v {
                let new_edge = if block_u < block_v {
                    Edge(block_u, block_v)
                } else {
                    Edge(block_v, block_u)
                };
                let _ = new_edges.insert(new_edge);
            }
        }

        <UnDirGraphStEph<V> as UnDirGraphStEphTrait<V>>::from_sets(new_vertices, new_edges)
    }

    /// One round of sequential edge contraction
    ///
    /// Computes a greedy matching and contracts it.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work Θ(|V| + |E|), Span Θ(|V| + |E|) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - Contracted graph
    pub fn contract_round<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> UnDirGraphStEph<V> {
        let matching = greedy_matching(graph);
        edge_contract(graph, &matching)
    }
}
