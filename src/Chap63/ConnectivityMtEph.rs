//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Implementation
//!
//! Implements parallel graph connectivity algorithms using star contraction.
//! - Algorithm 63.2: count_components (parallel)
//! - Algorithm 63.3: connected_components (parallel)
//! - Exercise 63.1: count_components using star_contract
//! - Exercise 63.2: connected_components using star_contract

pub mod ConnectivityMtEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::star_contract_mt;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct ConnectivityMtEph;

    // 8. traits

    pub trait ConnectivityMtEphTrait {
        /// Well-formedness for parallel connectivity algorithm input.
        open spec fn spec_connectivitymteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Count connected components using parallel star contraction.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        fn count_components_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> usize
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();

        /// Find connected components using parallel star contraction.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        fn connected_components_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();

        /// Count components using higher-order function approach.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        fn count_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> usize
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();

        /// Find components using higher-order function approach.
        /// APAS: Work O(|V| + |E|), Span O(lg^2 |V|)
        fn connected_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_connectivitymteph_wf(graph), valid_key_type_Edge::<V>();
    }

    pub type T<V> = UnDirGraphMtEph<V>;

    /// Algorithm 63.2: Count Connected Components (Parallel)
    ///
    /// Uses parallel star contraction to count connected components.
    /// Delegates to count_components_hof which implements the same algorithm via star_contract_mt.
    ///
    /// - APAS: Work O((n+m) lg n), Span O(lg² n) — Exercise 63.3 (edge-set, parallel)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O(m) — delegates to star_contract_mt
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
    /// - APAS: Work O((n+m) lg n), Span O(lg² n) — Exercise 63.4 (edge-set, parallel)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O(n lg n) — delegates to star_contract_mt
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
    ) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> result.0@ == graph@.V,
    {
        connected_components_hof(graph, seed)
    }

    /// Compose maps (P . C): for each (u -> v) in P, output (u -> C[v]).
    ///
    /// - APAS: N/A — helper function, Line 10 of Algorithm 63.3.
    /// - Claude-Opus-4.6: Work O(|P|), Span O(|P|) — currently sequential despite "parallel" name
    fn compose_maps_parallel<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        partition_map: &HashMapWithViewPlus<V, V>,
        component_map: &HashMapWithViewPlus<V, V>,
    ) -> (result: HashMapWithViewPlus<V, V>)
        requires
            obeys_key_model::<V>(),
            forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        ensures
            forall|k: V::V| #[trigger] result@.contains_key(k) ==> partition_map@.contains_key(k),
    {
        let mut result: HashMapWithViewPlus<V, V> = HashMapWithViewPlus::new();

        let it = partition_map.iter();
        for pair in iter: it
            invariant
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                forall|k: V::V| #[trigger] result@.contains_key(k) ==> partition_map@.contains_key(k),
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
    /// - APAS: Work O((n+m) lg n), Span O(lg^2 n) — same as Algorithm 63.2 (parallel)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O(m) — delegates to star_contract_mt (inherits merge bottleneck)
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

        let expand = |_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, r: usize| -> (result: usize) { r };

        star_contract_mt(graph, seed, &base, &expand, Ghost(|_r: usize| true))
    }

    /// Exercise 63.2: Connected Components using star_contract_mt higher-order function
    ///
    /// - APAS: Work O((n+m) lg n), Span O(lg^2 n) — same as Algorithm 63.3 (parallel)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O(n lg n) — delegates to star_contract_mt (inherits compose bottleneck)
    pub fn connected_components_hof<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> result.0@ == graph@.V,
    {
        let base = |vertices: &SetStEph<V>| -> (r: (SetStEph<V>, HashMapWithViewPlus<V, V>))
            requires
                vertices.spec_setsteph_wf(),
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            ensures r.0@ == vertices@,
        {
            let mut map = HashMapWithViewPlus::new();
            let it = vertices.iter();
            let ghost elem_seq = it@.1;
            for v in iter: it
                invariant
                    iter.elements == elem_seq,
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            {
                let _ = map.insert(v.clone(), v.clone());
            }
            (vertices.clone_plus(), map)
        };

        let expand = |_v: &SetStEph<V>,
                      _e: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMapWithViewPlus<V, V>,
                      reps_and_map: (SetStEph<V>, HashMapWithViewPlus<V, V>)|
            -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
            requires
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        {
            let (reps, component_map) = reps_and_map;
            let mut result_map = HashMapWithViewPlus::new();
            let it = partition_map.iter();
            for pair in iter: it
                invariant
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            {
                let (u, v) = pair;
                let component = component_map.get(v).unwrap_or(v);
                let _ = result_map.insert(u.clone(), component.clone());
            }
            (reps, result_map)
        };

        star_contract_mt(graph, seed, &base, &expand, Ghost(|_r: (SetStEph<V>, HashMapWithViewPlus<V, V>)| true))
    }

    } // verus!
}
