//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Prim's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.1: Prim's algorithm for computing Minimum Spanning Trees.
//! Uses priority-first search similar to Dijkstra's algorithm.

pub mod PrimStEph {

    use vstd::prelude::*;
    use ordered_float::OrderedFloat;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::cmp::Ordering;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::{HashSetWithViewPlus, HashSetWithViewPlusTrait};
    #[cfg(not(verus_keep_ghost))]
    use std::fmt::{Display, Formatter};
    #[cfg(not(verus_keep_ghost))]
    use std::fmt::Result as FmtResult;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    /// Priority queue entry for Prim's algorithm
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct PQEntry<V: StT + Ord> {
        priority: OrderedFloat<f64>,
        vertex: V,
        parent: Option<V>,
    }

    pub type T<V> = PQEntry<V>;

    verus! {
        pub trait PrimStEphTrait {
            /// Prim's MST algorithm
            /// APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
            fn prim_mst<V: StT + Hash + Ord>(
                graph: &LabUnDirGraphStEph<V, OrderedFloat<f64>>,
                start: V,
            ) -> SetStEph<LabEdge<V, OrderedFloat<f64>>>;

            /// Compute total weight of MST
            /// APAS: Work O(m), Span O(1)
            fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, OrderedFloat<f64>>>) -> OrderedFloat<f64>;
        }

        impl<V: StT + Ord> View for PQEntry<V> {
            type V = Self;
            open spec fn view(&self) -> Self { *self }
        }
    }

    /// Module-level function to create a new PQEntry.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    #[cfg(not(verus_keep_ghost))]
    fn pq_entry_new<V: StT + Hash + Ord>(priority: OrderedFloat<f64>, vertex: V, parent: Option<V>) -> PQEntry<V> {
        PQEntry {
            priority,
            vertex,
            parent,
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<V: StT + Hash + Ord> Ord for PQEntry<V> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn cmp(&self, other: &Self) -> Ordering { self.priority.cmp(&other.priority) }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<V: StT + Hash + Ord> PartialOrd for PQEntry<V> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    #[cfg(not(verus_keep_ghost))]
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
    /// - APAS: Work O(m lg n), Span O(m lg n)
    /// - Claude-Opus-4.6: Work O(m² lg n), Span O(m² lg n) — the APAS bound assumes
    ///   O(degree) adjacency-list lookups, but LabUnDirGraphStEph stores edges in a flat
    ///   set, so ng() and get_edge_label() each cost O(m) per call. Total neighbor/weight
    ///   work across all vertices is O(nm) = O(m²) in a dense graph. With an adjacency-list
    ///   graph representation this would be O(m lg n) as textbook states.
    #[cfg(not(verus_keep_ghost))]
    pub fn prim_mst<V: StT + Hash + Ord + Display>(
        graph: &LabUnDirGraphStEph<V, OrderedFloat<f64>>,
        start: &V,
    ) -> SetStEph<LabEdge<V, OrderedFloat<f64>>> {
        let mut mst_edges = SetLit![];
        let mut visited = HashSetWithViewPlus::<V>::new();

        let mut pq = BinaryHeapPQ::<PQEntry<V>>::singleton(pq_entry_new(OrderedFloat(0.0), start.clone(), None));

        while !pq.is_empty() {
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

            if let Some(parent_v) = parent_u {
                if let Some(weight) = graph.get_edge_label(&parent_v, &u) {
                    let edge = if parent_v < u {
                        LabEdge(parent_v, u.clone(), *weight)
                    } else {
                        LabEdge(u.clone(), parent_v, *weight)
                    };
                    let _ = mst_edges.insert(edge);
                }
            }

            let neighbors = graph.ng(&u);
            for v in neighbors.iter() {
                if !visited.contains(v) {
                    if let Some(weight) = graph.get_edge_label(&u, v) {
                        pq = pq.insert(pq_entry_new(*weight, v.clone(), Some(u.clone())));
                    }
                }
            }
        }

        mst_edges
    }

    /// Compute total MST weight.
    /// - APAS: (no cost stated) — utility function
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    #[cfg(not(verus_keep_ghost))]
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, OrderedFloat<f64>>>) -> OrderedFloat<f64> {
        let mut total = OrderedFloat(0.0);
        for edge in mst_edges.iter() {
            let LabEdge(_u, _v, w) = edge;
            total += *w;
        }
        total
    }
}
