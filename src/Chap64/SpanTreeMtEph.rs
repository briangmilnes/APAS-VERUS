// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Parallel)
//!
//! Implements Exercise 64.2: Compute spanning tree using parallel star contraction.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod SpanTreeMtEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Types::Types::*;

    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::{into_iter_hash_keys, obeys_key_model};
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::star_contract_mt;
    #[cfg(verus_keep_ghost)]
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::spec_valid_partition_map;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::hash_specs_plus::hash_specs_plus::key_view;
    use crate::SetLit;

    pub type T<V> = UnDirGraphMtEph<V>;

    verus!
{

    //		Section 3. broadcast use


    broadcast use vstd::std_specs::hash::group_hash_axioms;

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    #[derive(Debug)]
    pub struct SpanTreeMtEph;

    //		Section 8. traits


    pub trait SpanTreeMtEphTrait {
        /// Well-formedness for parallel spanning tree algorithm input.
        open spec fn spec_spantreemteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel spanning tree via star contraction.
        /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O(lg^2 n) — delegates to star_contract_mt; Mt parallel.
        fn spanning_tree_star_contraction_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
            graph: &UnDirGraphMtEph<V>,
        ) -> (tree_edges: SetStEph<Edge<V>>)
            requires Self::spec_spantreemteph_wf(graph)
            ensures
                tree_edges.spec_setsteph_wf(),
                forall|u: V::V, w: V::V| #[trigger] tree_edges@.contains((u, w)) ==>
                    graph@.A.contains((u, w)) || graph@.A.contains((w, u));

        /// Verify spanning tree properties.
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — connectivity check + edge count; sequential despite Mt module.
        fn verify_spanning_tree<V: StT + MtT + Hash + Ord>(graph: &UnDirGraphMtEph<V>, tree: &SetStEph<Edge<V>>) -> bool
            requires Self::spec_spantreemteph_wf(graph);
    }

    //		Section 9. impls


    /// Exercise 64.2: Spanning Tree via Star Contraction (Parallel).
    ///
    /// - Alg Analysis: APAS (Ch64 Ex 64.2): Work O((n+m) lg n), Span O(lg² n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O((n+m) lg n) — expand closure
    ///   is sequential; parallelism comes from star_contract_mt framework.
    pub fn spanning_tree_star_contraction_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (tree_edges: SetStEph<Edge<V>>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures
            tree_edges.spec_setsteph_wf(),
            // Every tree edge is an edge of the graph, in one orientation or the other.
            forall|u: V::V, w: V::V| #[trigger] tree_edges@.contains((u, w)) ==>
                graph@.A.contains((u, w)) || graph@.A.contains((w, u)),
    {
        // Base: no edges means no spanning tree edges.
        let base = |_vertices: &SetStEph<V>| -> (empty_edges: SetStEph<Edge<V>>)
            requires valid_key_type_Edge::<V>()
            ensures
                empty_edges.spec_setsteph_wf(),
                empty_edges@ == Set::<(V::V, V::V)>::empty(),
        {
            SetLit![]
        };

        // Expand: add star partition edges and map quotient tree edges back.
        // Every edge it returns is an edge of original_edges, in one orientation or the other.
        let expand = |_v: &SetStEph<V>,
                      original_edges: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMap<V, V>,
                      quotient_tree: SetStEph<Edge<V>>|
            -> (span_edges: SetStEph<Edge<V>>)
            requires
                valid_key_type_Edge::<V>(),
                obeys_key_model::<V>(),
                original_edges.spec_setsteph_wf(),
            ensures
                span_edges.spec_setsteph_wf(),
                forall|x: (V::V, V::V)| #[trigger] span_edges@.contains(x) ==>
                    original_edges@.contains(x) || original_edges@.contains((x.1, x.0)),
        {
            let mut spanning_edges: SetStEph<Edge<V>> = SetLit![];

            // Part 1: Collect edges from partition map (vertex -> center edges).
            // A satellite and its center are adjacent; the membership test states that
            // fact here, where the partition's ensures do not.
            let it_pm = partition_map.iter();
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for pair in iter: it_pm
                invariant
                    spanning_edges.spec_setsteph_wf(),
                    valid_key_type_Edge::<V>(),
                    obeys_key_model::<V>(),
                    original_edges.spec_setsteph_wf(),
                    forall|x: (V::V, V::V)| #[trigger] spanning_edges@.contains(x) ==>
                        original_edges@.contains(x) || original_edges@.contains((x.1, x.0)),
            {
                let (vertex, center) = pair;
                if *vertex != *center {
                    let edge = if *vertex < *center {
                        Edge(vertex.clone_view(), center.clone_view())
                    } else {
                        Edge(center.clone_view(), vertex.clone_view())
                    };
                    let reversed = Edge(edge.1.clone_view(), edge.0.clone_view());
                    if original_edges.mem(&edge) || original_edges.mem(&reversed) {
                        let _ = spanning_edges.insert(edge);
                    }
                }
            }

            // Part 2: Map quotient tree edges back to original edges.
            // Use elements.iter() to avoid needing quotient_tree.spec_setsteph_wf().
            let oe_vec = original_edges.to_seq();
            let noe = oe_vec.len();
            let ghost mapped_oe = oe_vec@.map(|_i: int, t: Edge<V>| t@);
            let it_qt = quotient_tree.elements.iter();
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for qe in iter: it_qt
                invariant
                    spanning_edges.spec_setsteph_wf(),
                    valid_key_type_Edge::<V>(),
                    obeys_key_model::<V>(),
                    obeys_key_model::<Edge<V>>(),
                    noe == oe_vec@.len(),
                    mapped_oe == oe_vec@.map(|_i: int, t: Edge<V>| t@),
                    forall|x: (V::V, V::V)| original_edges@.contains(x) <==> #[trigger] mapped_oe.contains(x),
                    forall|x: (V::V, V::V)| #[trigger] spanning_edges@.contains(x) ==>
                        original_edges@.contains(x) || original_edges@.contains((x.1, x.0)),
            {
                let Edge(c1, c2) = qe;
                let mut k: usize = 0;
                let mut found = false;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while k < noe && !found
                    invariant
                        spanning_edges.spec_setsteph_wf(),
                        valid_key_type_Edge::<V>(),
                        obeys_key_model::<V>(),
                        k <= noe,
                        noe == oe_vec@.len(),
                        mapped_oe == oe_vec@.map(|_i: int, t: Edge<V>| t@),
                        forall|x: (V::V, V::V)| original_edges@.contains(x) <==> #[trigger] mapped_oe.contains(x),
                        forall|x: (V::V, V::V)| #[trigger] spanning_edges@.contains(x) ==>
                            original_edges@.contains(x) || original_edges@.contains((x.1, x.0)),
                    decreases noe - k,
                {
                    let Edge(u, v) = &oe_vec[k];
                    let u_center = partition_map.get(u).unwrap_or(u);
                    let v_center = partition_map.get(v).unwrap_or(v);
                    if (*u_center == *c1 && *v_center == *c2) || (*u_center == *c2 && *v_center == *c1) {
                        proof {
                            assert(mapped_oe[k as int] == oe_vec@[k as int]@);
                            assert(mapped_oe.contains(oe_vec@[k as int]@));
                        }
                        let _ = spanning_edges.insert(Edge(u.clone_view(), v.clone_view()));
                        found = true;
                    }
                    k = k + 1;
                }
            }

            spanning_edges
        };

        let tree_edges = star_contract_mt(graph, seed, &base, &expand, Ghost(|r: SetStEph<Edge<V>>| r.spec_setsteph_wf()));
        proof {
            if exists|s: &SetStEph<V>| #[trigger] s@ == graph@.V && s.spec_setsteph_wf() && base.ensures((s,), tree_edges) {
                let s = choose|s: &SetStEph<V>| #[trigger] s@ == graph@.V && s.spec_setsteph_wf() && base.ensures((s,), tree_edges);
                assert(tree_edges@ == Set::<(V::V, V::V)>::empty());
            } else {
                let (v, e, c, p, r) = choose|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: SetStEph<Edge<V>>|
                    #[trigger] expand.ensures((v, e, c, p, r), tree_edges)
                    && v@ == graph@.V && e@ == graph@.A
                    && v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf()
                    && spec_valid_partition_map::<V>(graph@.V, c@, key_view(p@));
                assert(e@ == graph@.A);
            }
        }
        tree_edges
    }

    /// Verify that result is a valid spanning tree.
    ///
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E_tree|), Span O(|E_tree|).
    pub fn verify_spanning_tree<V: StT + MtT + Hash + Ord>(
        graph: &UnDirGraphMtEph<V>,
        tree_edges: &SetStEph<Edge<V>>,
    ) -> (valid: bool)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_for_graph::<V>(),
            tree_edges.spec_setsteph_wf(),
        ensures
            valid ==> tree_edges@.len() == (
                if graph@.V.len() > 0 { (graph@.V.len() - 1) as nat } else { 0nat }),
    {
        let n = graph.sizeV();
        let expected_edges: usize = if n > 0 { (n - 1) as usize } else { 0 };

        if tree_edges.size() != expected_edges {
            return false;
        }

        let graph_edges = graph.edges();
        let it = tree_edges.iter();
        let ghost edge_seq = into_iter_hash_keys(it);

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for edge in iter: it
            invariant
                iter.seq().unref() == edge_seq,
                edge_seq.map(|i: int, e: Edge<V>| e@).to_set() == tree_edges@,
                graph_edges.spec_setsteph_wf(),
                valid_key_type_Edge::<V>(),
        {
            if !graph_edges.mem(edge) {
                let rev = Edge(edge.1.clone_plus(), edge.0.clone_plus());
                if !graph_edges.mem(&rev) {
                    return false;
                }
            }
        }

        true
    }

    } // verus!

    //		Section 14. derive impls outside verus!



    impl std::fmt::Display for SpanTreeMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SpanTreeMtEph")
        }
    }
}
