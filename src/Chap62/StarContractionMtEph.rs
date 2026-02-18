//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Contraction - Multi-threaded Ephemeral Implementation
//!
//! Implements Algorithm 62.5: Star Contraction (parallel version)
//! Uses parallel star partition and parallel edge routing for quotient graph construction.

pub mod StarContractionMtEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use std::sync::Arc;
    #[cfg(not(verus_keep_ghost))]
    use std::vec::Vec;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap62::StarPartitionMtEph::StarPartitionMtEph::parallel_star_partition;
    #[cfg(not(verus_keep_ghost))]
    use crate::{ParaPair, SetLit};

    verus! {
        pub trait StarContractionMtEphTrait {
            /// Parallel star contraction higher-order function
            /// APAS: Work O((n + m) lg n), Span O(lg² n)
            fn star_contract_mt<V, R, F, G>(graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G) -> R
            where
                V: StT + MtT + Hash + Ord + 'static,
                F: Fn(&SetStEph<V>) -> R,
                G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMap<V, V>, R) -> R;

            /// Contract graph to just vertices (no edges)
            /// APAS: Work O((n + m) lg n), Span O(lg² n)
            fn contract_to_vertices_mt<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> SetStEph<V>;
        }
    } // verus!

    #[cfg(not(verus_keep_ghost))]
    pub type T<V> = UnDirGraphMtEph<V>;

    /// Algorithm 62.5: Star Contraction (Parallel)
    ///
    /// Higher-order recursive star contraction with parallelism:
    /// - Base case: No edges, call base function on vertices
    /// - Recursive case: Parallel partition, parallel quotient construction, recur, then expand
    ///
    /// - APAS: Work O((n + m) lg n), Span O(lg² n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — star_partition is sequential (all loops); quotient build uses ParaPair but partition dominates span.
    ///
    /// Arguments:
    /// - graph: The undirected graph to contract
    /// - seed: Random seed for partition
    /// - base: Function to call on the base case (isolated vertices)
    /// - expand: Function to expand result from quotient graph to original graph
    ///
    /// Returns:
    /// - Result of type R as computed by base and expand functions
    #[cfg(not(verus_keep_ghost))]
    pub fn star_contract_mt<V, R, F, G>(graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G) -> R
    where
        V: StT + MtT + Hash + Ord + 'static,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMap<V, V>, R) -> R,
    {
        if graph.sizeE() == 0 {
            return base(graph.vertices());
        }

        let (centers, partition_map) = parallel_star_partition(graph, seed);

        let quotient_graph = build_quotient_graph_parallel(graph, &centers, &partition_map);

        let r = star_contract_mt(&quotient_graph, seed + 1, base, expand);

        expand(graph.vertices(), graph.edges(), &centers, &partition_map, r)
    }

    /// Build quotient graph from partition (parallel version)
    ///
    /// Routes edges through partition map using divide-and-conquer parallelism.
    ///
    /// - APAS: (no cost stated) — helper not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(lg m) — delegates to route_edges_parallel which uses ParaPair fork-join.
    #[cfg(not(verus_keep_ghost))]
    fn build_quotient_graph_parallel<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        centers: &SetStEph<V>,
        partition_map: &HashMap<V, V>,
    ) -> UnDirGraphMtEph<V> {
        let edges_vec = graph.edges().iter().cloned().collect::<Vec<Edge<V>>>();
        let edges_seq = ArraySeqStEphS::from_vec(edges_vec);
        let n_edges = edges_seq.length();

        let part_map_arc = Arc::new(partition_map.clone());

        let quotient_edges = route_edges_parallel(&edges_seq, part_map_arc, 0, n_edges);

        <UnDirGraphMtEph<V> as UnDirGraphMtEphTrait<V>>::from_sets(centers.clone(), quotient_edges)
    }

    /// Parallel edge routing using divide-and-conquer
    ///
    /// - APAS: (no cost stated) — helper not in prose.
    /// - Claude-Opus-4.6: Work O(k), Span O(lg k) — binary fork-join via ParaPair; k = end - start.
    #[cfg(not(verus_keep_ghost))]
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
            let edge = edges.nth(start as N);
            let Edge(u, v) = edge;
            let u_center = partition_map.get(u).unwrap_or(u);
            let v_center = partition_map.get(v).unwrap_or(v);

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

        let mid = start + size / 2;

        let edges1 = edges.clone();
        let map1 = partition_map.clone();
        let edges2 = edges.clone();
        let map2 = partition_map;

        let pair = ParaPair!(move || route_edges_parallel(&edges1, map1, start, mid), move || {
            route_edges_parallel(&edges2, map2, mid, end)
        });

        let mut result = pair.0;
        for edge in pair.1.iter() {
            let _ = result.insert(edge.clone());
        }
        result
    }

    /// One round of parallel star contraction
    ///
    /// Convenience wrapper that performs contraction with identity base/expand.
    ///
    /// - APAS: Work O((n + m) lg n), Span O(lg² n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — delegates to star_contract_mt which has sequential partition.
    #[cfg(not(verus_keep_ghost))]
    pub fn contract_to_vertices_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> SetStEph<V> {
        star_contract_mt(
            graph,
            seed,
            &|vertices| vertices.clone(),
            &|_v, _e, _centers, _part, result| result,
        )
    }
}
