//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Edge Contraction - Multi-threaded Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.6: Parallel Edge Contraction (with fork/join parallelism)

pub mod EdgeContractionMtEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use std::sync::Arc;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use std::vec::Vec;
    use crate::Chap61::VertexMatchingMtEph::VertexMatchingMtEph::parallel_matching_mt;
    use crate::{ParaPair, SetLit};

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct EdgeContractionMtEph;

    // 8. traits

    pub trait EdgeContractionMtEphTrait {
        /// Well-formedness for parallel edge contraction algorithm input.
        open spec fn spec_edgecontractionmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel edge contraction algorithm.
        /// APAS: Work O(|E|), Span O(lg |V|)
        fn edge_contract_mt<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
            matching: &SetStEph<Edge<V>>,
        ) -> UnDirGraphMtEph<V>
            requires Self::spec_edgecontractionmteph_wf(graph);

        /// Single round of parallel edge contraction.
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        fn contract_round_mt<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> UnDirGraphMtEph<V>
            requires Self::spec_edgecontractionmteph_wf(graph);
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
    #[verifier::external_body]
    pub fn edge_contract_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        matching: &SetStEph<Edge<V>>,
    ) -> UnDirGraphMtEph<V> {
        use std::sync::{Arc, Mutex};
        pub type T<V> = UnDirGraphMtEph<V>;

        let vertex_to_block = Arc::new(Mutex::new(HashMapWithViewPlus::new()));

        {
            let mut map = vertex_to_block.lock().unwrap();
            for edge in matching.iter() {
                let Edge(u, v) = edge;
                map.insert(u.clone(), u.clone());
                map.insert(v.clone(), u.clone());
            }

            for vertex in graph.vertices().iter() {
                if !map.contains_key(vertex) {
                    map.insert(vertex.clone(), vertex.clone());
                }
            }
        }

        let vertex_to_block = match Arc::try_unwrap(vertex_to_block) {
            Ok(mutex) => mutex.into_inner().unwrap(),
            Err(_) => unreachable!(),
        };

        let mut new_vertices: SetStEph<V> = SetLit![];
        for (_, representative) in vertex_to_block.iter() {
            let _ = new_vertices.insert(representative.clone());
        }

        let edges_vec = graph.edges().iter().cloned().collect::<Vec<Edge<V>>>();
        let edges_seq = ArraySeqStEphS::from_vec(edges_vec);
        let n_edges = edges_seq.length();
        let edges_arc = Arc::new(edges_seq);
        let vertex_map_arc: Arc<HashMapWithViewPlus<V, V>> = Arc::new(vertex_to_block);

        let new_edges_set = build_edges_parallel(edges_arc, vertex_map_arc, 0, n_edges);

        <UnDirGraphMtEph<V> as UnDirGraphMtEphTrait<V>>::from_sets(new_vertices, new_edges_set)
    }

    /// Build new edge set in parallel using divide-and-conquer
    ///
    /// - APAS: N/A — Verus-specific scaffolding (parallel edge routing helper)
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(lg |E|) — genuine divide-and-conquer parallelism
    fn build_edges_parallel<V: StT + MtT + Hash + Ord + 'static>(
        edges: Arc<ArraySeqStEphS<Edge<V>>>,
        vertex_map: Arc<HashMapWithViewPlus<V, V>>,
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
            let edge = edges.nth(start as usize);
            let Edge(u, v) = edge;
            let block_u = match vertex_map.get(u) {
                Some(val) => val.clone(),
                None => u.clone(),
            };
            let block_v = match vertex_map.get(v) {
                Some(val) => val.clone(),
                None => v.clone(),
            };

            if block_u != block_v {
                let new_edge = if block_u < block_v {
                    Edge(block_u, block_v)
                } else {
                    Edge(block_v, block_u)
                };
                let mut new_edges: SetStEph<Edge<V>> = SetLit![];
                let _ = new_edges.insert(new_edge);
                return new_edges;
            } else {
                return SetLit![];
            }
        }

        let mid = start + size / 2;

        let edges1 = edges.clone();
        let map1 = vertex_map.clone();
        let edges2 = edges;
        let map2 = vertex_map;

        let f1 = move || -> (r: SetStEph<Edge<V>>)
            requires
                start <= mid,
                (mid as nat) <= (*edges1)@.len(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
        {
            build_edges_parallel(edges1, map1, start, mid)
        };

        let f2 = move || -> (r: SetStEph<Edge<V>>)
            requires
                mid <= end,
                (end as nat) <= (*edges2)@.len(),
                valid_key_type_Edge::<V>(),
            ensures
                r.spec_setsteph_wf(),
        {
            build_edges_parallel(edges2, map2, mid, end)
        };

        let Pair(left_edges, right_edges) = ParaPair!(f1, f2);

        left_edges.union(&right_edges)
    }

    /// One round of parallel edge contraction
    ///
    /// Computes a parallel matching and contracts it.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(lg |V|)
    /// - Claude-Opus-4.6: Work Θ(|E|^2), Span Θ(|E|) — dominated by parallel_matching_mt's
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
    ) -> (contracted: UnDirGraphMtEph<V>)
        requires
            valid_key_type_Edge::<V>(),
            graph.E.spec_setsteph_wf(),
        ensures true,
    {
        let matching = parallel_matching_mt(graph, seed);
        edge_contract_mt(graph, &matching)
    }

    } // verus!
}
