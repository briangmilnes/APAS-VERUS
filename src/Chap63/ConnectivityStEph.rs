//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Sequential Ephemeral Implementation
//!
//! Implements graph connectivity algorithms using star contraction.
//! - Algorithm 63.2: count_components
//! - Algorithm 63.3: connected_components
//! - Exercise 63.1: count_components using star_contract
//! - Exercise 63.2: connected_components using star_contract

pub mod ConnectivityStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_view_injective;
    use crate::Chap62::StarContractionStEph::StarContractionStEph::star_contract;
    use crate::SetLit;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct ConnectivityStEph;

    // 8. traits

    pub trait ConnectivityStEphTrait {
        /// Well-formedness for connectivity algorithm input.
        open spec fn spec_connectivitysteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Count connected components using star contraction.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn count_components<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> usize
            requires Self::spec_connectivitysteph_wf(graph), valid_key_type_Edge::<V>();

        /// Find connected components using star contraction.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn connected_components<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_connectivitysteph_wf(graph), valid_key_type_Edge::<V>();

        /// Count components using higher-order function approach.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn count_components_hof<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> usize
            requires Self::spec_connectivitysteph_wf(graph), valid_key_type_Edge::<V>();

        /// Find components using higher-order function approach.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn connected_components_hof<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_connectivitysteph_wf(graph), valid_key_type_Edge::<V>();
    }

    pub type T<V> = UnDirGraphStEph<V>;

    /// Algorithm 63.2: Count Connected Components
    ///
    /// Uses recursive star contraction to count the number of connected components.
    /// Delegates to count_components_hof which implements the same algorithm via star_contract.
    ///
    /// - Alg Analysis: APAS (Ch63 Ex 63.3): Work O((n+m) lg n), Span O((n+m) lg n) (edge-set representation)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) — matches APAS
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - The number of connected components
    pub fn count_components<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (count: usize)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> count as nat == graph@.V.len(),
    {
        count_components_hof(graph)
    }

    /// Algorithm 63.3: Connected Components
    ///
    /// Computes all connected components and returns a mapping from each vertex
    /// to a representative of its component.
    /// Delegates to connected_components_hof which implements the same algorithm via star_contract.
    ///
    /// - Alg Analysis: APAS (Ch63 Ex 63.4): Work O((n+m) lg n), Span O((n+m) lg n) (edge-set representation)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) — matches APAS
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS
    ///
    /// Arguments:
    /// - graph: The undirected graph
    ///
    /// Returns:
    /// - (representatives, component_map): Set of component representatives and
    ///   mapping from each vertex to its component representative
    pub fn connected_components<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (components: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> components.0@ == graph@.V,
    {
        connected_components_hof(graph)
    }

    /// Build quotient graph edges by routing through partition map.
    /// Filters out self-edges (where both endpoints map to same super-vertex).
    ///
    /// - Alg Analysis: APAS: N/A — helper function implicit in Algorithm 63.2/63.3 Line 7.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — single pass over edges
    fn build_quotient_edges<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        partition_map: &HashMapWithViewPlus<V, V>,
    ) -> (quotient_edges: SetStEph<Edge<V>>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures
            quotient_edges.spec_setsteph_wf(),
    {
        let mut quotient_edges: SetStEph<Edge<V>> = SetStEph::empty();
        let graph_edges = graph.edges();

        let it = graph_edges.iter();
        let ghost edge_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for edge in iter: it
            invariant
                iter.elements == edge_seq,
                edge_seq.map(|i: int, e: Edge<V>| e@).to_set() == graph_edges@,
                quotient_edges.spec_setsteph_wf(),
                valid_key_type_Edge::<V>(),
        {
            let u_center = match partition_map.get(&edge.0) {
                Some(c) => c.clone_plus(),
                None => edge.0.clone_plus(),
            };
            let v_center = match partition_map.get(&edge.1) {
                Some(c) => c.clone_plus(),
                None => edge.1.clone_plus(),
            };

            if u_center != v_center {
                let new_edge = if u_center < v_center {
                    Edge(u_center, v_center)
                } else {
                    Edge(v_center, u_center)
                };
                let _ = quotient_edges.insert(new_edge);
            }
        }

        quotient_edges
    }

    /// Exercise 63.1: Count Components using star_contract higher-order function
    ///
    /// Expresses countComponents in terms of starContract (Algorithm 62.5).
    ///
    /// - Alg Analysis: APAS (Ch63 Alg 63.2): Work O((n+m) lg n), Span O((n+m) lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) — matches APAS
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — delegates to star_contract
    pub fn count_components_hof<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (count: usize)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> count as nat == graph@.V.len(),
    {
        let base = |vertices: &SetStEph<V>| -> (n: usize)
            requires vertices.spec_setsteph_wf()
            ensures n as nat == vertices@.len()
        { vertices.size() };

        let expand = |_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, r: usize| -> (count: usize) { r };

        star_contract(graph, &base, &expand, Ghost(|r: usize| true))
    }

    /// Exercise 63.2: Connected Components using star_contract higher-order function
    ///
    /// Expresses connectedComponents in terms of starContract (Algorithm 62.5).
    ///
    /// - Alg Analysis: APAS (Ch63 Alg 63.3): Work O((n+m) lg n), Span O((n+m) lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) — matches APAS
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — delegates to star_contract
    pub fn connected_components_hof<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (components: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures graph@.A.is_empty() ==> components.0@ == graph@.V,
    {
        let base = |vertices: &SetStEph<V>| -> (r: (SetStEph<V>, HashMapWithViewPlus<V, V>))
            requires
                vertices.spec_setsteph_wf(),
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures r.0@ == vertices@,
        {
            let mut map = HashMapWithViewPlus::new();
            let it = vertices.iter();
            let ghost elem_seq = it@.1;
            for v in iter: it
                invariant
                    iter.elements == elem_seq,
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
                      partition_map: &HashMapWithViewPlus<V, V>,
                      reps_and_map: (SetStEph<V>, HashMapWithViewPlus<V, V>)|
            -> (expanded: (SetStEph<V>, HashMapWithViewPlus<V, V>))
            requires
                obeys_key_model::<V>(),
                obeys_feq_view_injective::<V>(),
        {
            let (reps, component_map) = reps_and_map;
            let mut result_map = HashMapWithViewPlus::new();
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

        star_contract(graph, &base, &expand, Ghost(|r: (SetStEph<V>, HashMapWithViewPlus<V, V>)| true))
    }

    } // verus!
}
