//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Contraction - Sequential Ephemeral Implementation
//!
//! Implements Algorithm 62.5: Star Contraction (sequential version)
//! A higher-order function that recursively contracts a graph using star partitions.

pub mod StarContractionStEph {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::sequential_star_partition;
    use crate::SetLit;
    use crate::Types::Types::*;
    pub type T<V> = UnDirGraphStEph<V>;

    pub trait StarContractionStEphTrait {
        /// Sequential star contraction higher-order function
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        fn star_contract<V, R, F, G>(graph: &UnDirGraphStEph<V>, base: F, expand: G) -> R
        where
            V: StT + Hash + Ord,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &R) -> R;

        /// Contract graph to just vertices (no edges)
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        fn contract_to_vertices<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>)     -> SetStEph<V>;
    }

    /// Algorithm 62.5: Star Contraction (Sequential)
    ///
    /// Higher-order recursive star contraction:
    /// - Base case: No edges, call base function on vertices
    /// - Recursive case: Partition graph, build quotient graph, recur, then expand
    ///
    /// - APAS: Work O((n + m) lg n), Span O((n + m) lg n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — agrees with APAS.
    ///
    /// Arguments:
    /// - graph: The undirected graph to contract
    /// - base: Function to call on the base case (isolated vertices)
    /// - expand: Function to expand result from quotient graph to original graph
    ///
    /// Returns:
    /// - Result of type R as computed by base and expand functions
    pub fn star_contract<V, R, F, G>(graph: &UnDirGraphStEph<V>, base: &F, expand: &G) -> R
    where
        V: StT + Hash + Ord,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMap<V, V>, R) -> R,
    {
        // Base case: no edges
        if graph.sizeE() == 0 {
            return base(graph.vertices());
        }

        // Recursive case:
        // 1. Compute star partition
        let (centers, partition_map) = sequential_star_partition(graph);

        // 2. Build quotient graph
        let quotient_graph = build_quotient_graph(graph, &centers, &partition_map);

        // 3. Recursively contract quotient graph
        let r = star_contract(&quotient_graph, base, expand);

        // 4. Expand result back to original graph
        expand(graph.vertices(), graph.edges(), &centers, &partition_map, r)
    }

    /// Build quotient graph from partition
    ///
    /// Routes edges through partition map, removing self-loops.
    ///
    /// - APAS: (no cost stated) — helper not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential loop over all edges.
    fn build_quotient_graph<V: StT + Hash + Ord>(
        graph: &UnDirGraphStEph<V>,
        centers: &SetStEph<V>,
        partition_map: &HashMap<V, V>,
    ) -> UnDirGraphStEph<V> {
        let mut quotient_edges: SetStEph<Edge<V>> = SetLit![];

        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;

            // Map endpoints to their centers
            let u_center = partition_map.get(u).unwrap_or(u);
            let v_center = partition_map.get(v).unwrap_or(v);

            // Add edge if centers are different (no self-loops)
            if u_center != v_center {
                // Normalize edge order for undirected graph
                let new_edge = if u_center < v_center {
                    Edge(u_center.clone(), v_center.clone())
                } else {
                    Edge(v_center.clone(), u_center.clone())
                };
                let _ = quotient_edges.insert(new_edge);
            }
        }

        <UnDirGraphStEph<V> as UnDirGraphStEphTrait<V>>::from_sets(centers.clone(), quotient_edges)
    }

    /// One round of sequential star contraction
    ///
    /// Convenience wrapper that performs contraction with identity base/expand.
    ///
    /// - APAS: Work O((n + m) lg n), Span O((n + m) lg n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — agrees with APAS.
    pub fn contract_to_vertices<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> SetStEph<V> {
        star_contract(
            graph,
            &|vertices| vertices.clone(),
            &|_v, _e, _centers, _part, result| result,
        )
    }
}
