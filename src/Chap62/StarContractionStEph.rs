//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Contraction - Sequential Ephemeral Implementation
//!
//! Implements Algorithm 62.5: Star Contraction (sequential version)
//! A higher-order function that recursively contracts a graph using star partitions.

pub mod StarContractionStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::sequential_star_partition;
    use crate::SetLit;

    verus! {

    // 3. broadcast use

    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarContractionStEph;

    // 8. traits

    pub trait StarContractionStEphTrait {
        /// Well-formedness for star contraction algorithm input.
        open spec fn spec_starcontractionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential star contraction higher-order function.
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        fn star_contract<V, R, F, G>(graph: &UnDirGraphStEph<V>, base: &F, expand: &G) -> R
        where
            V: HashOrd,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R
        requires Self::spec_starcontractionsteph_wf(graph);

        /// Contract graph to just vertices (no edges).
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<V>
            requires Self::spec_starcontractionsteph_wf(graph);
    }

    pub type T<V> = UnDirGraphStEph<V>;

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
    #[verifier::external_body]
    pub fn star_contract<V, R, F, G>(graph: &UnDirGraphStEph<V>, base: &F, expand: &G) -> R
    where
        V: HashOrd,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R,
    {
        if graph.sizeE() == 0 {
            return base(graph.vertices());
        }

        let (centers, partition_map) = sequential_star_partition(graph);

        let quotient_graph = build_quotient_graph(graph, &centers, &partition_map);

        let r = star_contract(&quotient_graph, base, expand);

        expand(graph.vertices(), graph.edges(), &centers, &partition_map, r)
    }

    /// Build quotient graph from partition
    ///
    /// Routes edges through partition map, removing self-loops.
    ///
    /// - APAS: (no cost stated) — helper not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential loop over all edges.
    fn build_quotient_graph<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        centers: &SetStEph<V>,
        partition_map: &HashMapWithViewPlus<V, V>,
    ) -> (quotient: UnDirGraphStEph<V>)
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        let mut quotient_edges: SetStEph<Edge<V>> = SetLit![];
        let edge_vec = graph.E.to_seq();
        let n = edge_vec.len();
        let mut i: usize = 0;
        while i < n
            invariant
                valid_key_type_Edge::<V>(),
                quotient_edges.spec_setsteph_wf(),
                i <= n,
                n == edge_vec@.len(),
            decreases n - i,
        {
            let edge = &edge_vec[i];
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
                let _ = quotient_edges.insert(new_edge);
            }
            i = i + 1;
        }

        UnDirGraphStEph { V: centers.clone(), E: quotient_edges }
    }

    /// One round of sequential star contraction
    ///
    /// Convenience wrapper that performs contraction with identity base/expand.
    ///
    /// - APAS: Work O((n + m) lg n), Span O((n + m) lg n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — agrees with APAS.
    pub fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (result: SetStEph<V>)
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        star_contract(
            graph,
            &|vertices: &SetStEph<V>| -> (r: SetStEph<V>) ensures true { vertices.clone() },
            &|_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, result: SetStEph<V>| -> (r: SetStEph<V>) ensures true { result },
        )
    }

    } // verus!
}
