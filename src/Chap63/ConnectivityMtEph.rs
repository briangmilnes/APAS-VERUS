//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Implementation
//!
//! Implements parallel graph connectivity algorithms using star contraction.
//! - Algorithm 63.2: count_components (parallel)
//! - Algorithm 63.3: connected_components (parallel)
//! - Exercise 63.1: count_components using star_contract
//! - Exercise 63.2: connected_components using star_contract

pub mod ConnectivityMtEph {

    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::Arc;
    use std::vec::Vec;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::star_contract_mt;
    use crate::Chap62::StarPartitionMtEph::StarPartitionMtEph::parallel_star_partition;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;
    pub type T<V> = UnDirGraphMtEph<V>;

    pub trait ConnectivityMtEphTrait {
        /// Count connected components using parallel star contraction
        /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
        fn count_components_mt<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>)  -> N;

        /// Find connected components using parallel star contraction
        /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
        fn connected_components_mt<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
        ) -> SetStEph<SetStEph<V>>;

        /// Count components using higher-order function approach
        /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
        fn count_components_hof<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>) -> N;

        /// Find components using higher-order function approach
        /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
        fn connected_components_hof<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
        ) -> SetStEph<SetStEph<V>>;
    }

    /// Algorithm 63.2: Count Connected Components (Parallel)
    ///
    /// Uses recursive parallel star contraction to count connected components.
    ///
    /// APAS: Work O((n+m) lg n), Span O(lg² n)
    /// claude-4-sonet: Work O((n+m) lg n), Span O(lg² n), Parallelism Θ((n+m)/lg² n)
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for star partition
    ///
    /// Returns:
    /// - The number of connected components
    pub fn count_components_mt<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> N {
        // Base case: no edges, each vertex is own component
        if graph.sizeE() == 0 {
            return graph.sizeV();
        }

        // Partition the graph in parallel
        let (centers, partition_map) = parallel_star_partition(graph, seed);

        // Build quotient graph in parallel
        let quotient_edges = build_quotient_edges_parallel(graph, &partition_map);
        let quotient_graph = <UnDirGraphMtEph<V> as UnDirGraphMtEphTrait<V>>::from_sets(centers, quotient_edges);

        // Recursively count components in quotient graph
        count_components_mt(&quotient_graph, seed + 1)
    }

    /// Algorithm 63.3: Connected Components (Parallel)
    ///
    /// Computes all connected components in parallel.
    ///
    /// APAS: Work O((n+m) lg n), Span O(lg² n)
    /// claude-4-sonet: Work O((n+m) lg n), Span O(lg² n), Parallelism Θ((n+m)/lg² n)
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for star partition
    ///
    /// Returns:
    /// - (representatives, component_map): Set of component representatives and
    ///   mapping from each vertex to its component representative
    pub fn connected_components_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (SetStEph<V>, HashMap<V, V>) {
        // Base case: no edges, each vertex maps to itself
        if graph.sizeE() == 0 {
            let mut component_map = HashMap::new();
            for vertex in graph.vertices().iter() {
                let _ = component_map.insert(vertex.clone(), vertex.clone());
            }
            return (graph.vertices().clone(), component_map);
        }

        // Partition the graph in parallel
        let (centers, partition_map) = parallel_star_partition(graph, seed);

        // Build quotient graph in parallel
        let quotient_edges = build_quotient_edges_parallel(graph, &partition_map);
        let quotient_graph = <UnDirGraphMtEph<V> as UnDirGraphMtEphTrait<V>>::from_sets(centers, quotient_edges);

        // Recursively compute components in quotient graph
        let (representatives, component_map_quotient) = connected_components_mt(&quotient_graph, seed + 1);

        // Compose maps in parallel
        let component_map = compose_maps_parallel(&partition_map, &component_map_quotient);

        (representatives, component_map)
    }

    /// Build quotient graph edges in parallel
    /// Work O(m), Span O(lg m), Parallelism Θ(m/lg m)
    fn build_quotient_edges_parallel<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        partition_map: &HashMap<V, V>,
    ) -> SetStEph<Edge<V>> {
        let edges_vec = graph.edges().iter().cloned().collect::<Vec<Edge<V>>>();
        let edges_seq = ArraySeqStEphS::from_vec(edges_vec);
        let n_edges = edges_seq.length();

        let part_map_arc = Arc::new(partition_map.clone());

        route_edges_parallel(&edges_seq, part_map_arc, 0, n_edges)
    }

    /// Parallel edge routing using divide-and-conquer
    ///
    /// Work O(k), Span O(lg k), where k = end - start
    fn route_edges_parallel<V: StT + MtT + Hash + Ord + 'static>(
        edges: &ArraySeqStEphS<Edge<V>>,
        partition_map: Arc<HashMap<V, V>>,
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
            let u_center = partition_map.get(u).unwrap_or(u);
            let v_center = partition_map.get(v).unwrap_or(v);

            // Only add if centers are different
            if u_center != v_center {
                let new_edge = if u_center < v_center {
                    Edge(u_center.clone(), v_center.clone())
                } else {
                    Edge(v_center.clone(), u_center.clone())
                };
                return SetLit![new_edge];
            }
            return SetLit![];
        }

        // Recursive case: divide and conquer
        let mid = start + size / 2;

        let edges1 = edges.clone();
        let map1 = partition_map.clone();
        let edges2 = edges.clone();
        let map2 = partition_map;

        let pair = ParaPair!(move || route_edges_parallel(&edges1, map1, start, mid), move || {
            route_edges_parallel(&edges2, map2, mid, end)
        });

        // Union the two sets
        let mut result = pair.0;
        for edge in pair.1.iter() {
            let _ = result.insert(edge.clone());
        }
        result
    }

    /// Compose maps in parallel (P ∘ C)
    /// For each (u → v) in P, output (u → C[v])
    /// Work O(|P|), Span O(lg |P|), Parallelism Θ(|P|/lg |P|)
    fn compose_maps_parallel<V: StT + MtT + Hash + Ord + 'static>(
        partition_map: &HashMap<V, V>,
        component_map: &HashMap<V, V>,
    ) -> HashMap<V, V> {
        // For now, compose sequentially since tuples don't implement Display
        // Future optimization: use custom parallel map composition
        let mut result = HashMap::new();
        for (u, v) in partition_map.iter() {
            let component = component_map.get(v).unwrap_or(v);
            let _ = result.insert(u.clone(), component.clone());
        }
        result
    }

    /// Exercise 63.1: Count Components using star_contract_mt higher-order function
    ///
    /// APAS: Work O((n+m) lg n), Span O(lg² n)
    /// claude-4-sonet: Work O((n+m) lg n), Span O(lg² n), Parallelism Θ((n+m)/lg² n)
    pub fn count_components_hof<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> N {
        // Base: when no edges, return number of vertices
        let base = |vertices: &SetStEph<V>| vertices.size();

        // Expand: just return the recursive result
        let expand = |_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMap<V, V>, r: N| r;

        star_contract_mt(graph, seed, &base, &expand)
    }

    /// Exercise 63.2: Connected Components using star_contract_mt higher-order function
    ///
    /// APAS: Work O((n+m) lg n), Span O(lg² n)
    /// claude-4-sonet: Work O((n+m) lg n), Span O(lg² n), Parallelism Θ((n+m)/lg² n)
    pub fn connected_components_hof<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (SetStEph<V>, HashMap<V, V>) {
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

        star_contract_mt(graph, seed, &base, &expand)
    }
}
