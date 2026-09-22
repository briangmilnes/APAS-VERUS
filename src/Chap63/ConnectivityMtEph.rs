// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Implementation
//!
//! Implements parallel graph connectivity algorithms using star contraction.
//! - Algorithm 63.2: count_components (parallel)
//! - Algorithm 63.3: connected_components (parallel)
//! - Exercise 63.1: count_components using star_contract
//! - Exercise 63.2: connected_components using star_contract


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod ConnectivityMtEph {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Types::Types::*;

    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_view_injective;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::hash_specs_plus::hash_specs_plus::key_view;
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::star_contract_mt;

    verus!
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        crate::vstdplus::hash_specs_plus::hash_specs_plus::group_key_view_lemmas,
    };

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct ConnectivityMtEph;

    pub type T<V> = UnDirGraphMtEph<V>;

    //		Section 8. traits


    pub trait ConnectivityMtEphTrait {
        /// Well-formedness for parallel connectivity algorithm input.
        open spec fn spec_connectivitymteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Count connected components using parallel star contraction.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n) — delegates to star_contract_mt; Mt parallel.
        fn count_components_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> usize
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();

        /// Find connected components using parallel star contraction.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n) — delegates to star_contract_mt; Mt parallel.
        fn connected_components_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMap<V, V>)
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();

        /// Count components using higher-order function approach.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n) — star_contract_mt with base/expand closures; Mt parallel.
        fn count_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> usize
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();

        /// Find components using higher-order function approach.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n) — star_contract_mt with base/expand closures; Mt parallel.
        fn connected_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMap<V, V>)
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();
    }

    //		Section 9. impls


    /// Algorithm 63.2: Count Connected Components (Parallel)
    ///
    /// Uses parallel star contraction to count connected components.
    /// Delegates to count_components_hof which implements the same algorithm via star_contract_mt.
    ///
    /// - Alg Analysis: APAS (Ch63 Ex 63.3): Work O((n+m) lg n), Span O(lg² n) (edge-set, parallel)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n); parallel star contraction
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(m) — delegates to star_contract_mt
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for star partition
    ///
    /// Returns:
    /// - The number of connected components
    pub fn count_components_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> (count: usize)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> count as nat == graph@.V.len(),
    {
        count_components_hof(graph, seed)
    }

    /// Algorithm 63.3: Connected Components (Parallel)
    ///
    /// Computes all connected components in parallel.
    /// Delegates to connected_components_hof which implements the same algorithm via star_contract_mt.
    ///
    /// - Alg Analysis: APAS (Ch63 Ex 63.4): Work O((n+m) lg n), Span O(lg² n) (edge-set, parallel)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n); parallel star contraction
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(n lg n) — delegates to star_contract_mt
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for star partition
    ///
    /// Returns:
    /// - (representatives, component_map): Set of component representatives and
    ///   mapping from each vertex to its component representative
    pub fn connected_components_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (components: (SetStEph<V>, HashMap<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> components.0@ == graph@.V,
    {
        connected_components_hof(graph, seed)
    }

    /// Compose maps (P . C): for each (u -> v) in P, output (u -> C[v]).
    ///
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|P|), Span O(|P|) — currently sequential despite "parallel" name
    fn compose_maps_parallel<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        partition_map: &HashMap<V, V>,
        component_map: &HashMap<V, V>,
    ) -> (composed: HashMap<V, V>)
        requires
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            forall|k: V::V| #[trigger] key_view(composed@).contains_key(k) ==> key_view(partition_map@).contains_key(k),
    {
        let mut result: HashMap<V, V> = HashMap::new();

        let it = partition_map.iter();
        for pair in iter: it
            invariant
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
                forall|k: V::V| #[trigger] key_view(result@).contains_key(k) ==> key_view(partition_map@).contains_key(k),
        {
            let (u_ref, v_ref) = pair;
            let u_key = u_ref.clone_view();
            // contains_key ensures: is_in == partition_map@.contains_key(u_ref@).
            // Always true since u_ref came from iterating partition_map.
            let is_in = partition_map.contains_key(u_ref);
            if is_in {
                let component = match component_map.get(v_ref) {
                    Some(c) => c.clone_plus(),
                    None => v_ref.clone_plus(),
                };
                let _ = result.insert(u_key, component);
            }
        }

        result
    }

    /// Exercise 63.1: Count Components using star_contract_mt higher-order function
    ///
    /// - Alg Analysis: APAS (Ch63 Alg 63.2): Work O((n+m) lg n), Span O(lg^2 n) (parallel)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n); parallel star contraction
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(m) — delegates to star_contract_mt (inherits merge bottleneck)
    pub fn count_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> (count: usize)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> count as nat == graph@.V.len(),
    {
        let base = |vertices: &SetStEph<V>| -> (n: usize)
            requires vertices.spec_setsteph_wf()
            ensures n as nat == vertices@.len()
        { vertices.size() };

        let expand = |_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMap<V, V>, r: usize| -> (count: usize) { r };

        star_contract_mt(graph, seed, &base, &expand, Ghost(|_r: usize| true))
    }

    /// Exercise 63.2: Connected Components using star_contract_mt higher-order function
    ///
    /// - Alg Analysis: APAS (Ch63 Alg 63.3): Work O((n+m) lg n), Span O(lg^2 n) (parallel)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n); parallel star contraction
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(n lg n) — delegates to star_contract_mt (inherits compose bottleneck)
    pub fn connected_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (components: (SetStEph<V>, HashMap<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> components.0@ == graph@.V,
    {
        let base = |vertices: &SetStEph<V>| -> (r: (SetStEph<V>, HashMap<V, V>))
            requires
                vertices.spec_setsteph_wf(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures r.0@ == vertices@,
        {
            let mut map = HashMap::new();
            let it = vertices.iter();
            for v in iter: it
                invariant
                    obeys_key_model::<V>(),
                    obeys_feq_view_injective::<V>(),
            {
                let _ = map.insert(v.clone(), v.clone());
            }
            (vertices.clone_plus(), map)
        };

        let expand = |_v: &SetStEph<V>,
                      _e: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMap<V, V>,
                      reps_and_map: (SetStEph<V>, HashMap<V, V>)|
            -> (expanded: (SetStEph<V>, HashMap<V, V>))
            requires
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            let (reps, component_map) = reps_and_map;
            let mut result_map = HashMap::new();
            let it = partition_map.iter();
            for pair in iter: it
                invariant
                    obeys_key_model::<V>(),
                    obeys_feq_view_injective::<V>(),
            {
                let (u, v) = pair;
                let component = component_map.get(v).unwrap_or(v);
                let _ = result_map.insert(u.clone(), component.clone());
            }
            (reps, result_map)
        };

        star_contract_mt(graph, seed, &base, &expand, Ghost(|_r: (SetStEph<V>, HashMap<V, V>)| true))
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for ConnectivityMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConnectivityMtEph")
        }
    }

    impl std::fmt::Display for ConnectivityMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConnectivityMtEph")
        }
    }
}
