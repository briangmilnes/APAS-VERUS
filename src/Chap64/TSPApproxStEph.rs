// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 64: TSP 2-Approximation via MST (Sequential)
//!
//! Implements Section 4: Approximating Metric TSP via MST
//! - Euler tour of spanning tree
//! - Shortcut to avoid revisiting vertices
//! - 2-approximation guarantee


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod TSPApproxStEph {


    //		Section 2. imports

    use vstd::prelude::*;

    use std::hash::Hash;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::{HashSetWithViewPlus, HashSetWithViewPlusTrait};
    #[cfg(not(verus_keep_ghost))]
    use std::vec::Vec;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    pub type T<V> = LabUnDirGraphStEph<V, WrappedF64>;

    verus! 
{

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct TSPApproxStEph;

    //		Section 8. traits


    pub trait TSPApproxStEphTrait {
        /// Well-formedness for sequential TSP approximation algorithm input.
        open spec fn spec_tspapproxsteph_wf<V: HashOrd>(graph: &LabUnDirGraphStEph<V, WrappedF64>) -> bool {
            spec_labgraphview_wf(graph@)
        }

        /// Compute Euler tour of a tree.
        /// APAS: Work O(|V|), Span O(|V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) — DFS traversal of tree; St sequential.
        fn euler_tour<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, WrappedF64>,
            start: &V,
            tree_edges: &SetStEph<LabEdge<V, WrappedF64>>,
        ) -> Vec<V>
            requires
                Self::spec_tspapproxsteph_wf(graph),
                valid_key_type_LabEdge::<V, WrappedF64>(),
                tree_edges.spec_setsteph_wf();

        /// Shortcut Euler tour to avoid revisiting vertices.
        /// APAS: Work O(|V|), Span O(|V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) — single pass removing duplicates; St sequential.
        fn shortcut_tour<V: HashOrd>(euler_tour: &[V]) -> Vec<V>;

        /// Compute total weight of a tour.
        /// APAS: Work O(|V|), Span O(|V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) — single pass summing edge weights; St sequential.
        fn tour_weight<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, WrappedF64>,
            tour: &[V],
        ) -> WrappedF64
            requires
                Self::spec_tspapproxsteph_wf(graph),
                valid_key_type_LabEdge::<V, WrappedF64>();

        /// 2-approximation algorithm for metric TSP.
        /// APAS: Work O(|V|² log |V|), Span O(|V|² log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) — euler tour + shortcut + weight; St sequential. MST cost dominates overall.
        fn approx_metric_tsp<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, WrappedF64>,
            spanning_tree: &SetStEph<LabEdge<V, WrappedF64>>,
            start: &V,
        ) -> (Vec<V>, WrappedF64)
            requires
                Self::spec_tspapproxsteph_wf(graph),
                valid_key_type::<V>(),
                valid_key_type_LabEdge::<V, WrappedF64>(),
                spanning_tree.spec_setsteph_wf();
    }

    //		Section 9. impls


    /// Linear scan for edge pair in visited-edges vector.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan over vec; St sequential.
    // veracity: no_requires
    fn vec_contains_pair<V: HashOrd>(v: &Vec<(V, V)>, key: &(V, V)) -> (found: bool)
        ensures true,
    {
        let mut i: usize = 0;
        while i < v.len()
            invariant 0 <= i <= v@.len(),
            decreases v.len() - i,
        {
            if v[i].0 == key.0 && v[i].1 == key.1 {
                return true;
            }
            i = i + 1;
        }
        false
    }

    /// Euler Tour of a Tree
    ///
    /// Performs DFS traversal that visits each edge twice (once in each direction).
    /// Returns a sequence of vertices visited.
    ///
    /// - Alg Analysis: APAS (Ch64 Sec 4): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DFS is inherently sequential.
    ///
    /// Arguments:
    /// - graph: Undirected graph (should be a tree)
    /// - start: Starting vertex
    /// - tree_edges: Set of edges forming the tree
    ///
    /// Returns:
    /// - Vector of vertices in Euler tour order
    pub fn euler_tour<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
        start: &V,
        tree_edges: &SetStEph<LabEdge<V, WrappedF64>>,
    ) -> (tour: Vec<V>)
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type_LabEdge::<V, WrappedF64>(),
            tree_edges.spec_setsteph_wf(),
        ensures true,
    {
        let mut tour = Vec::new();
        let mut visited_edges: Vec<(V, V)> = Vec::new();
        let fuel = tree_edges.size();

        euler_tour_dfs(graph, start, None, tree_edges, &mut tour, &mut visited_edges, fuel);

        tour
    }

    /// DFS helper for Euler tour with fuel-based termination.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m_tree), Span O(n * m_tree) — for each vertex,
    ///   scans neighbors (O(m)) and tree_edges (O(m_tree)) to find matching edges.
    fn euler_tour_dfs<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
        current: &V,
        parent: Option<&V>,
        tree_edges: &SetStEph<LabEdge<V, WrappedF64>>,
        tour: &mut Vec<V>,
        visited_edges: &mut Vec<(V, V)>,
        fuel: usize,
    )
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type_LabEdge::<V, WrappedF64>(),
            tree_edges.spec_setsteph_wf(),
        ensures true,
        decreases fuel,
    {
        tour.push(current.clone());

        if fuel == 0 {
            return;
        }

        // Collect neighbors into Vec via iterator.
        let neighbors = get_neighbors(graph, current);
        let mut ng_vec: Vec<V> = Vec::new();
        let mut ng_it = neighbors.iter();
        loop
            invariant
                ng_it@.0 <= ng_it@.1.len(),
            decreases ng_it@.1.len() - ng_it@.0,
        {
            match ng_it.next() {
                None => break,
                Some(n) => ng_vec.push(n.clone()),
            }
        }

        // Visit all neighbors connected by tree edges.
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < ng_vec.len()
            invariant
                0 <= i <= ng_vec@.len(),
                spec_labgraphview_wf(graph@),
                valid_key_type_LabEdge::<V, WrappedF64>(),
                tree_edges.spec_setsteph_wf(),
            decreases ng_vec.len() - i,
        {
            let neighbor = &ng_vec[i];

            // Skip parent to avoid immediate backtrack.
            let skip = match parent {
                Some(p) => *neighbor == *p,
                None => false,
            };

            if !skip {
                // Check if edge is in tree and not yet traversed.
                let edge_key = if *current < *neighbor {
                    (current.clone(), neighbor.clone())
                } else {
                    (neighbor.clone(), current.clone())
                };

                if !vec_contains_pair(visited_edges, &edge_key) {
                    // Check if edge exists in tree_edges.
                    let mut edge_found = false;
                    let mut te_it = tree_edges.iter();
                    loop
                        invariant te_it@.0 <= te_it@.1.len(),
                        decreases te_it@.1.len() - te_it@.0,
                    {
                        match te_it.next() {
                            None => break,
                            Some(te) => {
                                let u = &te.0;
                                let v = &te.1;
                                if (*u == *current && *v == *neighbor) || (*u == *neighbor && *v == *current) {
                                    edge_found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if edge_found {
                        visited_edges.push(edge_key);
                        euler_tour_dfs(graph, neighbor, Some(current), tree_edges, tour, visited_edges, fuel - 1);
                        tour.push(current.clone());
                    }
                }
            }

            i = i + 1;
        }
    }

    /// Shortcut Tour
    ///
    /// Removes duplicate vertex visits from Euler tour using triangle inequality.
    /// Result is a Hamiltonian cycle (visits each vertex exactly once).
    ///
    /// - Alg Analysis: APAS (Ch64 Sec 4): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — agrees with APAS.
    ///
    /// Arguments:
    /// - euler_tour: Euler tour with possible duplicate visits
    ///
    /// Returns:
    /// - Vector of vertices with each vertex appearing exactly once (except start/end)
    pub fn shortcut_tour<V: HashOrd>(euler_tour: &[V]) -> (tour: Vec<V>)
        requires valid_key_type::<V>(),
        ensures euler_tour@.len() == 0 ==> tour@.len() == 0,
    {
        if euler_tour.len() == 0 {
            return Vec::new();
        }

        let mut shortcut: Vec<V> = Vec::new();
        let mut visited = HashSetWithViewPlus::<V>::new();

        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < euler_tour.len()
            invariant
                0 <= i <= euler_tour@.len(),
                valid_key_type::<V>(),
            decreases euler_tour.len() - i,
        {
            let vertex = &euler_tour[i];
            if !visited.contains(vertex) {
                shortcut.push(vertex.clone_plus());
                let _ = visited.insert(vertex.clone_plus());
            }
            i = i + 1;
        }

        // Add starting vertex at end to complete cycle.
        if shortcut.len() > 0 {
            let start = shortcut[0].clone_plus();
            shortcut.push(start);
        }

        shortcut
    }

    /// Compute tour weight
    ///
    /// Sums the weights of edges in the tour.
    ///
    /// - Alg Analysis: APAS (Ch64 Sec 4): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — agrees with APAS.
    pub fn tour_weight<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
        tour: &[V],
    ) -> (total: WrappedF64)
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type_LabEdge::<V, WrappedF64>(),
        ensures tour@.len() <= 1 ==> total@ == 0.0f64,
    {
        let mut total = zero_dist();

        if tour.len() <= 1 {
            return total;
        }

        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < tour.len() - 1
            invariant
                0 <= i <= tour@.len() - 1,
                spec_labgraphview_wf(graph@),
            decreases tour.len() - 1 - i,
        {
            let u = &tour[i];
            let v = &tour[i + 1];

            match get_edge_weight(graph, u, v) {
                Some(weight) => {
                    total = total.dist_add(&weight);
                },
                None => {},
            }

            i = i + 1;
        }

        total
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — delegates to ng().
    fn get_neighbors<V: HashOrd>(graph: &LabUnDirGraphStEph<V, WrappedF64>, v: &V) -> (ng: SetStEph<V>)
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type_LabEdge::<V, WrappedF64>(),
        ensures ng@ == graph.spec_ng(v@), ng.spec_setsteph_wf(),
    {
        graph.ng(v)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — delegates to get_edge_label().
    fn get_edge_weight<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
        u: &V,
        v: &V,
    ) -> (weight: Option<WrappedF64>)
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type_LabEdge::<V, WrappedF64>(),
        ensures
            weight.is_some() == (exists |l: f64|
                graph@.A.contains((u@, v@, l)) || graph@.A.contains((v@, u@, l))),
            weight.is_some() ==> (graph@.A.contains((u@, v@, weight.unwrap()@)) ||
                                  graph@.A.contains((v@, u@, weight.unwrap()@))),
    {
        match graph.get_edge_label(u, v) {
            Some(w) => Some(*w),
            None => None,
        }
    }

    /// Approximate Metric TSP
    ///
    /// 2-approximation algorithm for metric TSP:
    /// 1. Given spanning tree T (should be MST for best results)
    /// 2. Compute Euler tour (visits each edge twice)
    /// 3. Apply shortcuts using triangle inequality
    /// 4. Result has weight <= 2 * w(T)
    ///
    /// - Alg Analysis: APAS (Ch64 Sec 4): Work O(n+m), Span O(n+m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — agrees with APAS.
    ///
    /// Arguments:
    /// - graph: Complete weighted undirected graph (metric: satisfies triangle inequality)
    /// - spanning_tree: Spanning tree edges (ideally MST)
    /// - start: Starting vertex for tour
    ///
    /// Returns:
    /// - (tour, weight): Hamiltonian cycle and its total weight
    pub fn approx_metric_tsp<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
        spanning_tree: &SetStEph<LabEdge<V, WrappedF64>>,
        start: &V,
    ) -> (tour_and_weight: (Vec<V>, WrappedF64))
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type::<V>(),
            valid_key_type_LabEdge::<V, WrappedF64>(),
            spanning_tree.spec_setsteph_wf(),
        ensures tour_and_weight.0@.len() <= 1 ==> tour_and_weight.1@ == 0.0f64,
    {
        let euler = euler_tour(graph, start, spanning_tree);
        let tour = shortcut_tour(&euler);
        let weight = tour_weight(graph, &tour);
        (tour, weight)
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for TSPApproxStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TSPApproxStEph")
        }
    }

    impl std::fmt::Display for TSPApproxStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TSPApproxStEph")
        }
    }
}
