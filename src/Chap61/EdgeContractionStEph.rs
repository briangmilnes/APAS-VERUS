//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Edge Contraction - Sequential Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.6: Parallel Edge Contraction (Sequential version)
//! - One round of contraction using greedy matching

pub mod EdgeContractionStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap61::VertexMatchingStEph::VertexMatchingStEph::greedy_matching;
    use crate::SetLit;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct EdgeContractionStEph;

    // 8. traits

    pub trait EdgeContractionStEphTrait {
        /// Well-formedness for edge contraction algorithm input.
        open spec fn spec_edgecontractionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential edge contraction algorithm.
        /// APAS: Work O(|E|), Span O(|E|)
        fn edge_contract<V: HashOrd>(
            graph: &UnDirGraphStEph<V>,
            matching: &SetStEph<Edge<V>>,
        ) -> UnDirGraphStEph<V>
            requires Self::spec_edgecontractionsteph_wf(graph);

        /// Single round of sequential edge contraction.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn contract_round<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> UnDirGraphStEph<V>
            requires Self::spec_edgecontractionsteph_wf(graph);
    }

    pub type T<V> = UnDirGraphStEph<V>;

    /// Algorithm 61.6: Sequential Edge Contraction
    ///
    /// Contracts edges in a matching by merging their endpoints.
    /// Each edge in the matching forms a block of two vertices.
    /// Unmatched vertices form singleton blocks.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work Θ(|V| + |E|), Span Θ(|V| + |E|) — agrees with APAS
    pub fn edge_contract<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        matching: &SetStEph<Edge<V>>,
    ) -> (contracted: UnDirGraphStEph<V>)
        requires
            valid_key_type_Edge::<V>(),
            matching.spec_setsteph_wf(),
            graph.E.spec_setsteph_wf(),
            graph.V.spec_setsteph_wf(),
        ensures true,
    {
        let mut vertex_to_block = HashMapWithViewPlus::<V, V>::new();

        let matching_it = matching.iter();
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for edge in iter: matching_it
            invariant
                valid_key_type_Edge::<V>(),
                matching.spec_setsteph_wf(),
        {
            let Edge(u, v) = edge;
            vertex_to_block.insert(u.clone(), u.clone());
            vertex_to_block.insert(v.clone(), u.clone());
        }

        let vertices_ref = graph.vertices();
        let vertices_it = vertices_ref.iter();
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for vertex in iter: vertices_it
            invariant
                valid_key_type_Edge::<V>(),
                graph.V.spec_setsteph_wf(),
        {
            if !vertex_to_block.contains_key(vertex) {
                vertex_to_block.insert(vertex.clone(), vertex.clone());
            }
        }

        let mut new_vertices: SetStEph<V> = SetLit![];
        let mut new_edges: SetStEph<Edge<V>> = SetLit![];

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for (_, representative) in vertex_to_block.iter()
            invariant
                valid_key_type_Edge::<V>(),
                new_vertices.spec_setsteph_wf(),
        {
            let _ = new_vertices.insert(representative.clone());
        }

        let edges_ref = graph.edges();
        let edges_it = edges_ref.iter();
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for edge in iter: edges_it
            invariant
                valid_key_type_Edge::<V>(),
                graph.E.spec_setsteph_wf(),
                new_edges.spec_setsteph_wf(),
        {
            let Edge(u, v) = edge;
            let block_u = match vertex_to_block.get(u) {
                Some(val) => val.clone(),
                None => u.clone(),
            };
            let block_v = match vertex_to_block.get(v) {
                Some(val) => val.clone(),
                None => v.clone(),
            };

            if block_u != block_v {
                let new_edge = if block_u < block_v {
                    Edge(block_u, block_v)
                } else {
                    Edge(block_v, block_u)
                };
                let _ = new_edges.insert(new_edge);
            }
        }

        UnDirGraphStEph { V: new_vertices, E: new_edges }
    }

    /// One round of sequential edge contraction
    ///
    /// Computes a greedy matching and contracts it.
    ///
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work Θ(|V| + |E|), Span Θ(|V| + |E|) — agrees with APAS
    pub fn contract_round<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (contracted: UnDirGraphStEph<V>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
            graph.E.spec_setsteph_wf(),
            graph.V.spec_setsteph_wf(),
        ensures true,
    {
        let matching = greedy_matching(graph);
        edge_contract(graph, &matching)
    }

    } // verus!
}
