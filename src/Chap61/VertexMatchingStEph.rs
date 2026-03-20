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
            let Edge(u, v) = edge;

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
    pub fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> (matching: SetStEph<Edge<V>>)
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        let mut matching: SetStEph<Edge<V>> = SetLit![];

        let mut rng = seeded_rng(seed);
        let mut edge_coins = HashMapWithViewPlus::<Edge<V>, bool>::new();

        let edge_vec = graph.E.to_seq();
        let ne = edge_vec.len();

        // Phase 1: Flip coins for all edges.
        let mut i: usize = 0;
        while i < ne
            invariant
                valid_key_type_Edge::<V>(),
                i <= ne,
                ne == edge_vec@.len(),
            decreases ne - i,
        {
            let edge = &edge_vec[i];
            edge_coins.insert(edge.clone(), random_bool_seeded(&mut rng));
            i = i + 1;
        }

        // Phase 2: Select edges where coin is heads and all adjacent coins are tails.
        let mut j: usize = 0;
        while j < ne
            invariant
                valid_key_type_Edge::<V>(),
                matching.spec_setsteph_wf(),
                j <= ne,
                ne == edge_vec@.len(),
            decreases ne - j,
        {
            let edge = &edge_vec[j];
            let Edge(u, v) = edge;

            let this_coin = match edge_coins.get(edge) {
                Some(val) => *val,
                None => false,
            };

            if this_coin {
                let mut all_adjacent_tails = true;

                let mut k: usize = 0;
                while k < ne
                    invariant
                        valid_key_type_Edge::<V>(),
                        k <= ne,
                        ne == edge_vec@.len(),
                    decreases ne - k,
                {
                    let adj_edge = &edge_vec[k];
                    if adj_edge != edge {
                        if graph.incident(adj_edge, u) || graph.incident(adj_edge, v) {
                            let adj_coin = match edge_coins.get(adj_edge) {
                                Some(val) => *val,
                                None => false,
                            };
                            if adj_coin {
                                all_adjacent_tails = false;
                            }
                        }
                    }
                    k = k + 1;
                }

                if all_adjacent_tails {
                    let _ = matching.insert(edge.clone());
                }
            }
            j = j + 1;
        }

        matching
    }

    } // verus!
}
