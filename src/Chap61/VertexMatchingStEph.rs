//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 61: Vertex Matching - Sequential Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.3: Greedy Vertex Matching (sequential)
//! - Baseline sequential version of parallel matching algorithm

pub mod VertexMatchingStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::rand::rand::{seeded_rng, random_bool_seeded};

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct VertexMatchingStEph;

    // 8. traits

    pub trait VertexMatchingStEphTrait {
        /// Well-formedness for vertex matching algorithm input.
        open spec fn spec_vertexmatchingsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Greedy vertex matching algorithm.
        /// APAS: Work O(|E|), Span O(|E|)
        fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>>
            requires Self::spec_vertexmatchingsteph_wf(graph);

        /// Sequential version of parallel matching.
        /// APAS: Work O(|E|), Span O(|E|)
        fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> SetStEph<Edge<V>>
            requires Self::spec_vertexmatchingsteph_wf(graph);
    }

    /// Algorithm 61.3: Greedy Vertex Matching
    ///
    /// - APAS: Work Θ(|E|), Span Θ(|E|)
    /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(|E|) — agrees with APAS
    pub fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> (matching: SetStEph<Edge<V>>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures true,
    {
        let mut matching: SetStEph<Edge<V>> = SetLit![];
        let mut matched_vertices: SetStEph<V> = SetLit![];

        for edge in graph.edges().iter()
            invariant
                valid_key_type_Edge::<V>(),
                matching.spec_setsteph_wf(),
                matched_vertices.spec_setsteph_wf(),
        {
            let Edge(ref u, ref v) = edge;

            if !matched_vertices.mem(u) && !matched_vertices.mem(v) {
                let _ = matching.insert(edge.clone());
                let _ = matched_vertices.insert(u.clone());
                let _ = matched_vertices.insert(v.clone());
            }
        }

        matching
    }

    /// Baseline Sequential Version of Parallel Matching
    ///
    /// - APAS: (no cost stated — sequential baseline of Algorithm 61.4)
    /// - Claude-Opus-4.6: Work Θ(|E|²), Span Θ(|E|²)
    #[verifier::external_body]
    #[cfg(not(verus_keep_ghost))]
    pub fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> SetStEph<Edge<V>> {
        pub type T<V> = UnDirGraphStEph<V>;

        let mut matching: SetStEph<Edge<V>> = SetLit![];

        let mut rng = seeded_rng(seed);
        let mut edge_coins = HashMapWithViewPlus::<Edge<V>, bool>::new();

        for edge in graph.edges().iter() {
            edge_coins.insert(edge.clone(), random_bool_seeded(&mut rng));
        }

        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;

            if !edge_coins.get(edge).copied().unwrap_or(false) {
                continue;
            }

            let mut all_adjacent_tails = true;

            for adj_edge in graph.edges().iter() {
                if adj_edge == edge {
                    continue;
                }

                if (graph.incident(adj_edge, u) || graph.incident(adj_edge, v))
                    && edge_coins.get(adj_edge).copied().unwrap_or(false)
                {
                    all_adjacent_tails = false;
                    break;
                }
            }

            if all_adjacent_tails {
                let _ = matching.insert(edge.clone());
            }
        }

        matching
    }

    } // verus!
}
