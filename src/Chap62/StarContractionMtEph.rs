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

    use std::hash::Hash;
    use std::sync::Arc;
    use std::vec::Vec;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::Chap62::StarPartitionMtEph::StarPartitionMtEph::parallel_star_partition;
    use crate::{ParaPair, SetLit};

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarContractionMtEph;

    // 8. traits

    pub trait StarContractionMtEphTrait {
        /// Well-formedness for parallel star contraction algorithm input.
        open spec fn spec_starcontractionmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel star contraction higher-order function.
        /// APAS: Work O((n + m) lg n), Span O(lg^2 n)
        fn star_contract_mt<V, R, F, G>(graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G) -> R
        where
            V: StT + MtT + Hash + Ord + 'static,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R
        requires Self::spec_starcontractionmteph_wf(graph);

        /// Contract graph to just vertices (no edges).
        /// APAS: Work O((n + m) lg n), Span O(lg^2 n)
        fn contract_to_vertices_mt<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> SetStEph<V>
            requires Self::spec_starcontractionmteph_wf(graph);
    }

    pub type T<V> = UnDirGraphMtEph<V>;

    /// Algorithm 62.5: Star Contraction (Parallel)
    ///
    /// Higher-order recursive star contraction with parallelism:
    /// - Base case: No edges, call base function on vertices
    /// - Recursive case: Parallel partition, parallel quotient construction, recur, then expand
    ///
    /// - APAS: Work O((n + m) lg n), Span O(lg^2 n)
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
    #[verifier::external_body]
    pub fn star_contract_mt<V, R, F, G>(graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G) -> R
    where
        V: StT + MtT + Hash + Ord + 'static,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R,
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
    fn build_quotient_graph_parallel<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        centers: &SetStEph<V>,
        partition_map: &HashMapWithViewPlus<V, V>,
    ) -> (quotient: UnDirGraphMtEph<V>)
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        let edges_vec = graph.E.to_seq();
        let edges_seq = ArraySeqStEphS::from_vec(edges_vec);
        let n_edges = edges_seq.length();
        let edges_arc = Arc::new(edges_seq);

        let part_map_arc = Arc::new(partition_map.clone());

        let quotient_edges = route_edges_parallel(edges_arc, part_map_arc, 0, n_edges);

        UnDirGraphMtEph { V: centers.clone(), E: quotient_edges }
    }

    /// Parallel edge routing using divide-and-conquer
    ///
    /// - APAS: (no cost stated) — helper not in prose.
    /// - Claude-Opus-4.6: Work O(k), Span O(lg k) — binary fork-join via ParaPair; k = end - start.
    fn route_edges_parallel<V: StT + MtT + Hash + Ord + 'static>(
        edges: Arc<ArraySeqStEphS<Edge<V>>>,
        partition_map: Arc<HashMapWithViewPlus<V, V>>,
        start: usize,
        end: usize,
    ) -> (result: SetStEph<Edge<V>>)
        requires
            start <= end,
            end as nat <= (*edges)@.len(),
            valid_key_type_Edge::<V>(),
        ensures
            result.spec_setsteph_wf(),
        decreases end - start,
    {
        let size = end - start;

        if size == 0 {
            return SetLit![];
        }

        if size == 1 {
            let edge = edges.nth(start as N);
            let Edge(u, v) = edge;
            let u_center = match partition_map.get(u) {
                Some(val) => val.clone(),
                None => u.clone(),
            };
            let v_center = match partition_map.get(v) {
                Some(val) => val.clone(),
                None => v.clone(),
            };

            if u_center != v_center {
                let new_edge = if u_center < v_center {
                    Edge(u_center, v_center)
                } else {
                    Edge(v_center, u_center)
                };
                let mut new_edges: SetStEph<Edge<V>> = SetLit![];
                let _ = new_edges.insert(new_edge);
                return new_edges;
            }
            return SetLit![];
        }

        let mid = start + size / 2;

        let edges1 = edges.clone();
        let map1 = partition_map.clone();
        let edges2 = edges;
        let map2 = partition_map;

        let f1 = move || -> (r: SetStEph<Edge<V>>)
            requires
                start <= mid,
                (mid as nat) <= (*edges1)@.len(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
        {
            route_edges_parallel(edges1, map1, start, mid)
        };

        let f2 = move || -> (r: SetStEph<Edge<V>>)
            requires
                mid <= end,
                (end as nat) <= (*edges2)@.len(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
        {
            route_edges_parallel(edges2, map2, mid, end)
        };

        let Pair(left_edges, right_edges) = ParaPair!(f1, f2);

        left_edges.union(&right_edges)
    }

    /// One round of parallel star contraction
    ///
    /// Convenience wrapper that performs contraction with identity base/expand.
    ///
    /// - APAS: Work O((n + m) lg n), Span O(lg^2 n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — delegates to star_contract_mt which has sequential partition.
    pub fn contract_to_vertices_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (result: SetStEph<V>)
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        star_contract_mt(
            graph,
            seed,
            &|vertices: &SetStEph<V>| -> (r: SetStEph<V>) ensures true { vertices.clone() },
            &|_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, result: SetStEph<V>| -> (r: SetStEph<V>) ensures true { result },
        )
    }

    } // verus!
}
