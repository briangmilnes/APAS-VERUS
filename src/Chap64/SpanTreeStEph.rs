//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Sequential)
//!
//! Implements Exercise 64.2: Compute spanning tree using star contraction.

pub mod SpanTreeStEph {

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::Chap62::StarContractionStEph::StarContractionStEph::star_contract;
    use crate::SetLit;

    pub type T<V> = UnDirGraphStEph<V>;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct SpanTreeStEph;

    // 8. traits

    pub trait SpanTreeStEphTrait {
        /// Well-formedness for sequential spanning tree algorithm input.
        open spec fn spec_spantreesteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential spanning tree via star contraction.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn spanning_tree_star_contraction<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>>
            requires Self::spec_spantreesteph_wf(graph);

        /// Verify spanning tree properties.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn verify_spanning_tree<V: HashOrd>(graph: &UnDirGraphStEph<V>, tree: &SetStEph<Edge<V>>) -> B
            requires Self::spec_spantreesteph_wf(graph);
    }

    /// Exercise 64.2: Spanning Tree via Star Contraction.
    ///
    /// - APAS: Work O((n+m) lg n), Span O((n+m) lg n)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — agrees with APAS.
    #[verifier::external_body]
    pub fn spanning_tree_star_contraction<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<Edge<V>> {
        // Base: no edges means no spanning tree edges (isolated vertices)
        let base = |_vertices: &SetStEph<V>| SetLit![];

        // Expand: add star partition edges and map quotient tree edges back
        let expand = |_v: &SetStEph<V>,
                      original_edges: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMapWithViewPlus<V, V>,
                      quotient_tree: SetStEph<Edge<V>>| {
            // Collect edges from partition map (vertex → center edges)
            let mut spanning_edges = SetLit![];

            for (vertex, center) in partition_map.iter() {
                // Add edge if vertex is not its own center (avoid self-loops)
                if vertex != center {
                    // Normalize edge order
                    let edge = if vertex < center {
                        Edge(vertex.clone(), center.clone())
                    } else {
                        Edge(center.clone(), vertex.clone())
                    };
                    let _ = spanning_edges.insert(edge);
                }
            }

            // Map quotient tree edges back to original edges
            // For each edge between centers in quotient tree, find original edge that maps to it
            for quotient_edge in quotient_tree.iter() {
                let Edge(c1, c2) = quotient_edge;

                // Find an original edge that connects the two stars (centers c1 and c2)
                for original_edge in original_edges.iter() {
                    let Edge(u, v) = original_edge;
                    let u_center = partition_map.get(u).unwrap_or(u);
                    let v_center = partition_map.get(v).unwrap_or(v);

                    // Check if this original edge connects the two centers (in either direction)
                    if (u_center == c1 && v_center == c2) || (u_center == c2 && v_center == c1) {
                        let _ = spanning_edges.insert(original_edge.clone());
                        break; // Only need one edge between the two stars
                    }
                }
            }

            spanning_edges
        };

        star_contract(graph, &base, &expand)
    }

    /// Verify that result is a valid spanning tree.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(|V| + |E_tree|), Span O(|V| + |E_tree|).
    pub fn verify_spanning_tree<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        tree_edges: &SetStEph<Edge<V>>,
    ) -> (result: B)
        requires
            spec_graphview_wf(graph@),
            tree_edges.spec_setsteph_wf(),
        ensures
            result ==> tree_edges@.len() == (
                if graph@.V.len() > 0 { (graph@.V.len() - 1) as nat } else { 0nat }),
    {
        let n = graph.sizeV();
        let expected_edges: N = if n > 0 { (n - 1) as N } else { 0 };

        if tree_edges.size() != expected_edges {
            return false;
        }

        let graph_edges = graph.edges();
        let it = tree_edges.iter();
        let ghost edge_seq = it@.1;

        for edge in iter: it
            invariant
                iter.elements == edge_seq,
                edge_seq.map(|i: int, e: Edge<V>| e@).to_set() == tree_edges@,
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
}
