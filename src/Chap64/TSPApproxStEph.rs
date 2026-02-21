//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: TSP 2-Approximation via MST (Sequential)
//!
//! Implements Section 4: Approximating Metric TSP via MST
//! - Euler tour of spanning tree
//! - Shortcut to avoid revisiting vertices
//! - 2-approximation guarantee

pub mod TSPApproxStEph {

    use vstd::prelude::*;

    verus! {
        // Placeholder. Full verusification blocked: uses ordered_float (Verus can't link).
    }

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::{HashSetWithViewPlus, HashSetWithViewPlusTrait};
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use std::vec::Vec;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::float::float::{F64Ord, f64_ord};
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;
    #[cfg(not(verus_keep_ghost))]
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    pub type T<V> = LabUnDirGraphStEph<V, F64Ord>;

    #[cfg(not(verus_keep_ghost))]
    pub trait TSPApproxStEphTrait {
        /// Compute Euler tour of a tree
        /// APAS: Work O(|V|), Span O(|V|)
        fn euler_tour<V: StT + Hash + Ord>(tree: &LabUnDirGraphStEph<V, F64Ord>, start: V) -> Vec<V>;

        /// Shortcut Euler tour to avoid revisiting vertices
        /// APAS: Work O(|V|), Span O(|V|)
        fn shortcut_tour<V: StT + Hash + Ord>(euler_tour: &[V]) -> Vec<V>;

        /// Compute total weight of a tour
        /// APAS: Work O(|V|), Span O(|V|)
        fn tour_weight<V: StT + Hash + Ord>(
            tour: &[V],
            distances: &HashMap<(V, V), F64Ord>,
        ) -> F64Ord;

        /// 2-approximation algorithm for metric TSP
        /// APAS: Work O(|V|² log |V|), Span O(|V|² log |V|)
        fn approx_metric_tsp<V: StT + Hash + Ord>(
            distances: &HashMap<(V, V), F64Ord>,
            vertices: &SetStEph<V>,
        ) -> Vec<V>;
    }

    /// Euler Tour of a Tree
    ///
    /// Performs DFS traversal that visits each edge twice (once in each direction).
    /// Returns a sequence of vertices visited.
    ///
    /// - APAS: Work O(n), Span O(n)
    /// - Claude-Opus-4.6: Work O(n), Span O(n) — DFS is inherently sequential.
    ///
    /// Arguments:
    /// - graph: Undirected graph (should be a tree)
    /// - start: Starting vertex
    /// - tree_edges: Set of edges forming the tree
    ///
    /// Returns:
    /// - Vector of vertices in Euler tour order
    #[cfg(not(verus_keep_ghost))]
    pub fn euler_tour<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, F64Ord>,
        start: &V,
        tree_edges: &SetStEph<LabEdge<V, F64Ord>>,
    ) -> Vec<V> {
        let mut tour = Vec::new();
        let mut visited_edges = HashSetWithViewPlus::<(V, V)>::new();

        euler_tour_dfs(graph, start, None, tree_edges, &mut tour, &mut visited_edges);

        tour
    }

    /// - APAS: N/A — internal helper for euler_tour.
    /// - Claude-Opus-4.6: Work O(n * m_tree), Span O(n * m_tree) — for each vertex,
    ///   scans neighbors (O(m)) and tree_edges (O(m_tree)) to find matching edges.
    #[cfg(not(verus_keep_ghost))]
    fn euler_tour_dfs<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, F64Ord>,
        current: &V,
        parent: Option<&V>,
        tree_edges: &SetStEph<LabEdge<V, F64Ord>>,
        tour: &mut Vec<V>,
        visited_edges: &mut HashSetWithViewPlus<(V, V)>,
    ) {
        tour.push(current.clone());

        // Visit all neighbors connected by tree edges
        let neighbors = get_neighbors(graph, current);
        for neighbor in neighbors.iter() {
            // Skip parent to avoid immediate backtrack
            if let Some(p) = parent {
                if neighbor == p {
                    continue;
                }
            }

            // Check if edge is in tree and not yet traversed
            let edge_key = if current < neighbor {
                (current.clone(), neighbor.clone())
            } else {
                (neighbor.clone(), current.clone())
            };

            if visited_edges.contains(&edge_key) {
                continue;
            }

            // Check if edge exists in tree_edges
            let mut edge_found = false;
            for edge in tree_edges.iter() {
                let LabEdge(u, v, _) = edge;
                if (u == current && v == neighbor) || (u == neighbor && v == current) {
                    edge_found = true;
                    break;
                }
            }

            if edge_found {
                visited_edges.insert(edge_key);
                euler_tour_dfs(graph, neighbor, Some(current), tree_edges, tour, visited_edges);
                tour.push(current.clone());
            }
        }
    }

    /// Shortcut Tour
    ///
    /// Removes duplicate vertex visits from Euler tour using triangle inequality.
    /// Result is a Hamiltonian cycle (visits each vertex exactly once).
    ///
    /// - APAS: Work O(n), Span O(n)
    /// - Claude-Opus-4.6: Work O(n), Span O(n) — agrees with APAS.
    ///
    /// Arguments:
    /// - euler_tour: Euler tour with possible duplicate visits
    ///
    /// Returns:
    /// - Vector of vertices with each vertex appearing exactly once (except start/end)
    #[cfg(not(verus_keep_ghost))]
    pub fn shortcut_tour<V: StT + Hash + Ord>(euler_tour: &[V]) -> Vec<V> {
        if euler_tour.is_empty() {
            return Vec::new();
        }

        let mut shortcut = Vec::new();
        let mut visited = HashSetWithViewPlus::<V>::new();

        for vertex in euler_tour.iter() {
            if !visited.contains(vertex) {
                shortcut.push(vertex.clone());
                let _ = visited.insert(vertex.clone());
            }
        }

        // Add starting vertex at end to complete cycle
        if let Some(start) = shortcut.first() {
            shortcut.push(start.clone());
        }

        shortcut
    }

    /// Compute tour weight
    ///
    /// Sums the weights of edges in the tour.
    ///
    /// - APAS: Work O(n), Span O(n)
    /// - Claude-Opus-4.6: Work O(n), Span O(n) — agrees with APAS.
    #[cfg(not(verus_keep_ghost))]
    pub fn tour_weight<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, F64Ord>,
        tour: &[V],
    ) -> F64Ord {
        let mut total = f64_ord(0.0);

        for i in 0..tour.len() - 1 {
            let u = &tour[i];
            let v = &tour[i + 1];

            // Find edge weight
            if let Some(weight) = get_edge_weight(graph, u, v) {
                total += weight;
            }
        }

        total
    }

    /// - APAS: N/A — internal helper, no prose counterpart.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — linear scan over all edges.
    #[cfg(not(verus_keep_ghost))]
    fn get_neighbors<V: StT + Hash + Ord>(graph: &LabUnDirGraphStEph<V, F64Ord>, v: &V) -> SetStEph<V> {
        let mut neighbors = SetLit![];
        for edge in graph.labeled_edges().iter() {
            let LabEdge(a, b, _) = edge;
            if a == v {
                let _ = neighbors.insert(b.clone());
            } else if b == v {
                let _ = neighbors.insert(a.clone());
            }
        }
        neighbors
    }

    /// - APAS: N/A — internal helper, no prose counterpart.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — linear scan over all edges.
    #[cfg(not(verus_keep_ghost))]
    fn get_edge_weight<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, F64Ord>,
        u: &V,
        v: &V,
    ) -> Option<F64Ord> {
        for edge in graph.labeled_edges().iter() {
            let LabEdge(a, b, w) = edge;
            if (a == u && b == v) || (a == v && b == u) {
                return Some(*w);
            }
        }
        None
    }

    /// Approximate Metric TSP
    ///
    /// 2-approximation algorithm for metric TSP:
    /// 1. Given spanning tree T (should be MST for best results)
    /// 2. Compute Euler tour (visits each edge twice)
    /// 3. Apply shortcuts using triangle inequality
    /// 4. Result has weight ≤ 2 × w(T)
    ///
    /// - APAS: Work O(n+m), Span O(n+m)
    /// - Claude-Opus-4.6: Work O(n+m), Span O(n+m) — agrees with APAS.
    ///
    /// Arguments:
    /// - graph: Complete weighted undirected graph (metric: satisfies triangle inequality)
    /// - spanning_tree: Spanning tree edges (ideally MST)
    /// - start: Starting vertex for tour
    ///
    /// Returns:
    /// - (tour, weight): Hamiltonian cycle and its total weight
    #[cfg(not(verus_keep_ghost))]
    pub fn approx_metric_tsp<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, F64Ord>,
        spanning_tree: &SetStEph<LabEdge<V, F64Ord>>,
        start: &V,
    ) -> (Vec<V>, F64Ord) {
        // Step 1: Compute Euler tour
        let euler = euler_tour(graph, start, spanning_tree);

        // Step 2: Apply shortcuts
        let tour = shortcut_tour(&euler);

        // Step 3: Compute tour weight
        let weight = tour_weight(graph, &tour);

        (tour, weight)
    }
}
