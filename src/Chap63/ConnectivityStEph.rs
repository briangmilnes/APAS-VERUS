//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Sequential Ephemeral Implementation
//!
//! Implements graph connectivity algorithms using star contraction.
//! - Algorithm 63.2: count_components
//! - Algorithm 63.3: connected_components
//! - Exercise 63.1: count_components using star_contract
//! - Exercise 63.2: connected_components using star_contract

pub mod ConnectivityStEph {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Chap62::StarContractionStEph::StarContractionStEph::star_contract;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::sequential_star_partition;
    use crate::SetLit;
    use crate::Types::Types::*;
    pub type T<V> = UnDirGraphStEph<V>;

    pub trait ConnectivityStEphTrait {
        /// Count connected components using star contraction
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn count_components<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>)         -> N;

        /// Find connected components using star contraction
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn connected_components<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>)     -> SetStEph<SetStEph<V>>;

        /// Count components using higher-order function approach
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn count_components_hof<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>)     -> N;

        /// Find components using higher-order function approach
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn connected_components_hof<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> SetStEph<SetStEph<V>>;
    }

    /// Algorithm 63.2: Count Connected Components
    ///
    /// Uses recursive star contraction to count the number of connected components.
    /// Base case: No edges means each vertex is its own component.
    /// Inductive case: Partition, build quotient graph, recurse.
    ///
    /// - APAS: Work O((n+m) lg n), Span O((n+m) lg n) — Exercise 63.3 (edge-set representation)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - The number of connected components
    pub fn count_components<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> N {
        // Base case: no edges, each vertex is own component
        if graph.sizeE() == 0 {
            return graph.sizeV();
        }

        // Partition the graph
        let (centers, partition_map) = sequential_star_partition(graph);

        // Build quotient graph by routing edges through partition map
        let quotient_edges = build_quotient_edges(graph, &partition_map);
        let quotient_graph = <UnDirGraphStEph<V> as UnDirGraphStEphTrait<V>>::from_sets(centers, quotient_edges);

        // Recursively count components in quotient graph
        count_components(&quotient_graph)
    }

    /// Algorithm 63.3: Connected Components
    ///
    /// Computes all connected components and returns a mapping from each vertex
    /// to a representative of its component.
    ///
    /// - APAS: Work O((n+m) lg n), Span O((n+m) lg n) — Exercise 63.4 (edge-set representation)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - (representatives, component_map): Set of component representatives and
    ///   mapping from each vertex to its component representative
    pub fn connected_components<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMap<V, V>) {
        // Base case: no edges, each vertex maps to itself
        if graph.sizeE() == 0 {
            let mut component_map = HashMap::new();
            for vertex in graph.vertices().iter() {
                let _ = component_map.insert(vertex.clone(), vertex.clone());
            }
            return (graph.vertices().clone(), component_map);
        }

        // Partition the graph
        let (centers, partition_map) = sequential_star_partition(graph);

        // Build quotient graph
        let quotient_edges = build_quotient_edges(graph, &partition_map);
        let quotient_graph = <UnDirGraphStEph<V> as UnDirGraphStEphTrait<V>>::from_sets(centers, quotient_edges);

        // Recursively compute components in quotient graph
        let (representatives, component_map_quotient) = connected_components(&quotient_graph);

        // Compose maps: for each vertex u, map it to C[P[u]]
        let mut component_map = HashMap::new();
        for (u, v) in partition_map.iter() {
            let component = component_map_quotient.get(v).unwrap_or(v);
            let _ = component_map.insert(u.clone(), component.clone());
        }

        (representatives, component_map)
    }

    /// Build quotient graph edges by routing through partition map.
    /// Filters out self-edges (where both endpoints map to same super-vertex).
    ///
    /// - APAS: N/A — helper function implicit in Algorithm 63.2/63.3 Line 7.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — single pass over edges
    fn build_quotient_edges<V: StT + Hash + Ord>(
        graph: &UnDirGraphStEph<V>,
        partition_map: &HashMap<V, V>,
    ) -> SetStEph<Edge<V>> {
        let mut quotient_edges: SetStEph<Edge<V>> = SetLit![];

        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;
            let u_center = partition_map.get(u).unwrap_or(u);
            let v_center = partition_map.get(v).unwrap_or(v);

            // Only add if centers are different (no self-loops)
            if u_center != v_center {
                let new_edge = if u_center < v_center {
                    Edge(u_center.clone(), v_center.clone())
                } else {
                    Edge(v_center.clone(), u_center.clone())
                };
                let _ = quotient_edges.insert(new_edge);
            }
        }

        quotient_edges
    }

    /// Exercise 63.1: Count Components using star_contract higher-order function
    ///
    /// Expresses countComponents in terms of starContract (Algorithm 62.5).
    ///
    /// - APAS: Work O((n+m) lg n), Span O((n+m) lg n) — same as Algorithm 63.2
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — delegates to star_contract
    pub fn count_components_hof<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> N {
        // Base: when no edges, return number of vertices
        let base = |vertices: &SetStEph<V>| vertices.size();

        // Expand: just return the recursive result (no expansion needed for counting)
        let expand = |_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMap<V, V>, r: N| r;

        star_contract(graph, &base, &expand)
    }

    /// Exercise 63.2: Connected Components using star_contract higher-order function
    ///
    /// Expresses connectedComponents in terms of starContract (Algorithm 62.5).
    ///
    /// - APAS: Work O((n+m) lg n), Span O((n+m) lg n) — same as Algorithm 63.3
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — delegates to star_contract
    pub fn connected_components_hof<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMap<V, V>) {
        // Base: when no edges, each vertex maps to itself
        let base = |vertices: &SetStEph<V>| {
            let mut map = HashMap::new();
            for v in vertices.iter() {
                let _ = map.insert(v.clone(), v.clone());
            }
            (vertices.clone(), map)
        };

        // Expand: compose partition map P with component map C
        let expand = |_v: &SetStEph<V>,
                      _e: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMap<V, V>,
                      (reps, component_map): (SetStEph<V>, HashMap<V, V>)| {
            let mut result_map = HashMap::new();
            for (u, v) in partition_map.iter() {
                let component = component_map.get(v).unwrap_or(v);
                let _ = result_map.insert(u.clone(), component.clone());
            }
            (reps, result_map)
        };

        star_contract(graph, &base, &expand)
    }
}
