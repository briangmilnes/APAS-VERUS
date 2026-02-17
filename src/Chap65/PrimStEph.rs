//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Prim's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.1: Prim's algorithm for computing Minimum Spanning Trees.
//! Uses priority-first search similar to Dijkstra's algorithm.

pub mod PrimStEph {

    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::hash::Hash;

    use ordered_float::OrderedFloat;
    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    pub type T<V> = PQEntry<V>;

    pub trait PrimStEphTrait {
        /// Prim's MST algorithm
        /// APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
        fn prim_mst<V: StT + Hash + Ord + Display>(
            graph: &LabUnDirGraphStEph<V, OrderedFloat<f64>>,
            start: V,
        ) -> SetStEph<LabEdge<V, OrderedFloat<f64>>>;

        /// Compute total weight of MST
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, OrderedFloat<f64>>>) -> OrderedFloat<f64>;
    }

    /// Priority queue entry for Prim's algorithm
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct PQEntry<V: StT + Hash + Ord> {
        priority: OrderedFloat<f64>,
        vertex: V,
        parent: Option<V>,
    }

    /// Module-level function to create a new PQEntry
    fn pq_entry_new<V: StT + Hash + Ord>(priority: OrderedFloat<f64>, vertex: V, parent: Option<V>) -> PQEntry<V> {
        PQEntry {
            priority,
            vertex,
            parent,
        }
    }

    impl<V: StT + Hash + Ord> Ord for PQEntry<V> {
        fn cmp(&self, other: &Self) -> Ordering { self.priority.cmp(&other.priority) }
    }

    impl<V: StT + Hash + Ord> PartialOrd for PQEntry<V> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl<V: StT + Hash + Ord + Display> Display for PQEntry<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "({}, {})", self.priority, self.vertex) }
    }

    /// Algorithm 65.1: Prim's MST Algorithm
    ///
    /// Computes the Minimum Spanning Tree using priority-first search.
    /// Similar to Dijkstra's, but priority is minimum edge weight to visited set X.
    ///
    /// Priority: p(v) = min_{x∈X} w(x,v)
    ///
    /// APAS: Work O(m lg n), Span O(m lg n)
    /// claude-4-sonet: Work O(m lg n), Span O(m lg n) [sequential]
    ///
    /// Arguments:
    /// - graph: Weighted undirected graph
    /// - start: Starting vertex (arbitrary choice)
    ///
    /// Returns:
    /// - Set of edges forming the MST
    pub fn prim_mst<V: StT + Hash + Ord + Display>(
        graph: &LabUnDirGraphStEph<V, OrderedFloat<f64>>,
        start: &V,
    ) -> SetStEph<LabEdge<V, OrderedFloat<f64>>> {
        let mut mst_edges = SetLit![];
        let mut visited = HashSet::<V>::new();

        // Priority queue
        let mut pq = BinaryHeapPQ::<PQEntry<V>>::singleton(pq_entry_new(OrderedFloat(0.0), start.clone(), None));

        while !pq.is_empty() {
            // Extract minimum priority vertex
            let (new_pq, entry_opt) = pq.delete_min();
            pq = new_pq;

            let entry = match entry_opt {
                | Some(e) => e,
                | None => break,
            };

            let u = entry.vertex;
            let parent_u = entry.parent;

            if visited.contains(&u) {
                continue;
            }

            let _ = visited.insert(u.clone());

            // Add edge to MST (except for start vertex)
            if let Some(parent_v) = parent_u {
                if let Some(weight) = get_edge_weight(graph, &parent_v, &u) {
                    let edge = if parent_v < u {
                        LabEdge(parent_v, u.clone(), weight)
                    } else {
                        LabEdge(u.clone(), parent_v, weight)
                    };
                    let _ = mst_edges.insert(edge);
                }
            }

            // Update priorities of neighbors
            let neighbors = get_neighbors(graph, &u);
            for v in neighbors.iter() {
                if !visited.contains(v) {
                    if let Some(weight) = get_edge_weight(graph, &u, v) {
                        pq = pq.insert(pq_entry_new(weight, v.clone(), Some(u.clone())));
                    }
                }
            }
        }

        mst_edges
    }

    fn get_neighbors<V: StT + Hash + Ord>(graph: &LabUnDirGraphStEph<V, OrderedFloat<f64>>, v: &V) -> SetStEph<V> {
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

    fn get_edge_weight<V: StT + Hash + Ord>(
        graph: &LabUnDirGraphStEph<V, OrderedFloat<f64>>,
        u: &V,
        v: &V,
    ) -> Option<OrderedFloat<f64>> {
        for edge in graph.labeled_edges().iter() {
            let LabEdge(a, b, w) = edge;
            if (a == u && b == v) || (a == v && b == u) {
                return Some(*w);
            }
        }
        None
    }

    /// Compute total MST weight
    ///
    /// APAS: Work O(|MST|), Span O(|MST|)
    /// claude-4-sonet: Work O(|MST|), Span O(|MST|)
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, OrderedFloat<f64>>>) -> OrderedFloat<f64> {
        let mut total = OrderedFloat(0.0);
        for edge in mst_edges.iter() {
            let LabEdge(_u, _v, w) = edge;
            total += *w;
        }
        total
    }

    verus! {
        impl<V: StT + Hash + Ord> View for PQEntry<V> {
            type V = Self;
            open spec fn view(&self) -> Self { *self }
        }
    }
}
