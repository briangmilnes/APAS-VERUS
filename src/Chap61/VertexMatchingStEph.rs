//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 61: Vertex Matching - Sequential Ephemeral Implementation
//!
//! Implements:
//! - Algorithm 61.3: Greedy Vertex Matching (sequential)
//! - Baseline sequential version of parallel matching algorithm


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod VertexMatchingStEph {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::rand::rand::{seeded_rng, random_bool_seeded};

    verus! 
{

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct VertexMatchingStEph;

    //		Section 8. traits


    pub trait VertexMatchingStEphTrait {
        /// Well-formedness for vertex matching algorithm input.
        open spec fn spec_vertexmatchingsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Greedy vertex matching algorithm.
        /// APAS: Work O(|E|), Span O(|E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|), Span O(|E|) — single pass over edges; St sequential.
        fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>>
            requires Self::spec_vertexmatchingsteph_wf(graph);

        /// Sequential version of parallel matching.
        /// APAS: Work O(|E|), Span O(|E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|E|^2), Span O(|E|^2) — coin flip + per-edge scan of all edges; St sequential.
        fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> SetStEph<Edge<V>>
            requires Self::spec_vertexmatchingsteph_wf(graph);
    }

    //		Section 9. impls


    /// Algorithm 61.3: Greedy Vertex Matching
    ///
    /// - Alg Analysis: APAS (Ch61 Alg 61.4): Work O(|E|), Span O(|E|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(|E|), Span Θ(|E|) — agrees with APAS
    pub fn greedy_matching<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> (matching: SetStEph<Edge<V>>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
            graph.E.spec_setsteph_wf(),
        ensures
            matching.spec_setsteph_wf(),
        {
        let mut matching: SetStEph<Edge<V>> = SetLit![];
        let mut matched_vertices: SetStEph<V> = SetLit![];

        let edges_ref = graph.edges();
        let edges_it = edges_ref.iter();
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for edge in iter: edges_it
            invariant
                valid_key_type_Edge::<V>(),
                graph.E.spec_setsteph_wf(),
                matching.spec_setsteph_wf(),
                matched_vertices.spec_setsteph_wf(),
        {
            let Edge(u, v) = edge;

            if !matched_vertices.mem(u) && !matched_vertices.mem(v) {
                let _ = matching.insert(Edge(u.clone(), v.clone()));
                let _ = matched_vertices.insert(u.clone());
                let _ = matched_vertices.insert(v.clone());
            }
        }

        matching
    }

    /// Baseline Sequential Version of Parallel Matching
    ///
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(|E|²), Span Θ(|E|²)
    pub fn parallel_matching_st<V: StT + Hash>(graph: &UnDirGraphStEph<V>, seed: u64) -> (matching: SetStEph<Edge<V>>)
        requires
            valid_key_type_Edge::<V>(),
            graph.E.spec_setsteph_wf(),
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
            edge_coins.insert(Edge(edge.0.clone(), edge.1.clone()), random_bool_seeded(&mut rng));
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
                    if !(adj_edge.0 == edge.0 && adj_edge.1 == edge.1) {
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
                    let _ = matching.insert(Edge(edge.0.clone(), edge.1.clone()));
                }
            }
            j = j + 1;
        }

        matching
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for VertexMatchingStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VertexMatchingStEph")
        }
    }

    impl std::fmt::Display for VertexMatchingStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VertexMatchingStEph")
        }
    }
}
