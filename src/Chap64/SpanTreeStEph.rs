//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Sequential)
//!
//! Implements Exercise 64.2: Compute spanning tree using star contraction.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod SpanTreeStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;
    use crate::Chap62::StarContractionStEph::StarContractionStEph::star_contract;
    use crate::SetLit;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    pub type T<V> = UnDirGraphStEph<V>;

    verus! 
{

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct SpanTreeStEph;

    //		Section 8. traits


    pub trait SpanTreeStEphTrait {
        /// Well-formedness for sequential spanning tree algorithm input.
        open spec fn spec_spantreesteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential spanning tree via star contraction.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O((n+m) lg n) — delegates to star_contract; St sequential.
        fn spanning_tree_star_contraction<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>>
            requires Self::spec_spantreesteph_wf(graph);

        /// Verify spanning tree properties.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — connectivity check + edge count; St sequential.
        fn verify_spanning_tree<V: HashOrd>(graph: &UnDirGraphStEph<V>, tree: &SetStEph<Edge<V>>) -> bool
            requires Self::spec_spantreesteph_wf(graph);
    }

    //		Section 9. impls


    /// Exercise 64.2: Spanning Tree via Star Contraction.
    ///
    /// - Alg Analysis: APAS (Ch64 Ex 64.2): Work O((n+m) lg n), Span O((n+m) lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS.
    pub fn spanning_tree_star_contraction<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (tree_edges: SetStEph<Edge<V>>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures tree_edges.spec_setsteph_wf(),
    {
        // Base: no edges means no spanning tree edges (isolated vertices).
        let base = |_vertices: &SetStEph<V>| -> (empty_edges: SetStEph<Edge<V>>)
            requires valid_key_type_Edge::<V>()
            ensures empty_edges.spec_setsteph_wf()
        {
            SetLit![]
        };

        // Expand: add star partition edges and map quotient tree edges back.
        // Uses elements.iter() (HashSetWithViewPlus iter, no wf required) instead
        // of SetStEph::iter() so the closure has only type-level requires.
        let expand = |_v: &SetStEph<V>,
                      original_edges: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMapWithViewPlus<V, V>,
                      quotient_tree: SetStEph<Edge<V>>|
            -> (span_edges: SetStEph<Edge<V>>)
            requires
                valid_key_type_Edge::<V>(),
                obeys_key_model::<V>(),
            ensures span_edges.spec_setsteph_wf()
        {
            let mut spanning_edges: SetStEph<Edge<V>> = SetLit![];

            // Part 1: Collect edges from partition map (vertex -> center edges).
            let it_pm = partition_map.iter();
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for pair in iter: it_pm
                invariant
                    spanning_edges.spec_setsteph_wf(),
                    valid_key_type_Edge::<V>(),
            {
                let (vertex, center) = pair;
                if *vertex != *center {
                    let edge = if *vertex < *center {
                        Edge(vertex.clone(), center.clone())
                    } else {
                        Edge(center.clone(), vertex.clone())
                    };
                    let _ = spanning_edges.insert(edge);
                }
            }

            // Part 2: Map quotient tree edges back to original edges.
            // Use elements.iter() to avoid needing quotient_tree.spec_setsteph_wf().
            let it_qt = quotient_tree.elements.iter();
            let ghost qt_seq = it_qt@.1;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for qe in iter: it_qt
                invariant
                    iter.elements == qt_seq,
                    spanning_edges.spec_setsteph_wf(),
                    valid_key_type_Edge::<V>(),
                    obeys_key_model::<V>(),
            {
                let Edge(c1, c2) = qe;
                // Use elements.iter() to avoid needing original_edges.spec_setsteph_wf().
                let mut it_oe = original_edges.elements.iter();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                loop
                    invariant
                        spanning_edges.spec_setsteph_wf(),
                        valid_key_type_Edge::<V>(),
                        obeys_key_model::<V>(),
                        it_oe@.0 <= it_oe@.1.len(),
                    decreases it_oe@.1.len() - it_oe@.0,
                {
                    match it_oe.next() {
                        None => break,
                        Some(oe) => {
                            let Edge(u, v) = oe;
                            let u_center = partition_map.get(u).unwrap_or(u);
                            let v_center = partition_map.get(v).unwrap_or(v);
                            if (*u_center == *c1 && *v_center == *c2) || (*u_center == *c2 && *v_center == *c1) {
                                let owned_edge = Edge(u.clone(), v.clone());
                                let _ = spanning_edges.insert(owned_edge);
                                break;
                            }
                        }
                    }
                }
            }

            spanning_edges
        };

        star_contract(graph, &base, &expand, Ghost(|r: SetStEph<Edge<V>>| r.spec_setsteph_wf()))
    }

    /// Verify that result is a valid spanning tree.
    ///
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E_tree|), Span O(|V| + |E_tree|) — N/A, Verus-specific scaffolding
    pub fn verify_spanning_tree<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        tree_edges: &SetStEph<Edge<V>>,
    ) -> (valid: bool)
        requires
            spec_graphview_wf(graph@),
            tree_edges.spec_setsteph_wf(),
            valid_key_type_Edge::<V>(),
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
        let ghost edge_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for edge in iter: it
            invariant
                iter.elements == edge_seq,
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


    impl std::fmt::Debug for SpanTreeStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SpanTreeStEph")
        }
    }

    impl std::fmt::Display for SpanTreeStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SpanTreeStEph")
        }
    }
}
