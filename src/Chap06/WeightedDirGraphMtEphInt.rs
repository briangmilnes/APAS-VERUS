//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with integer weights - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for weighted neighbor operations.
//! Weighted arc filtering (out_neighbors_weighted, in_neighbors_weighted) is parallel.

pub mod WeightedDirGraphMtEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    pub type WeightedDirGraphMtEphInt<V> = LabDirGraphMtEph<V, i32>;

    /// Convenience functions for weighted directed graphs with integer weights (multi-threaded)
    pub trait WeightedDirGraphMtEphIntTrait<V: StT + MtT + Hash + 'static> {
        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> Self;
        fn add_weighted_edge(&mut self, from: V, to: V, weight: i32);
        fn get_edge_weight(&self, from: &V, to: &V)                                       -> Option<i32>;
        fn weighted_edges(&self)                                                          -> SetStEph<Triple<V, V, i32>>;
        fn out_neighbors_weighted(&self, v: &V)                                           -> SetStEph<Pair<V, i32>>;
        fn in_neighbors_weighted(&self, v: &V)                                            -> SetStEph<Pair<V, i32>>;
        fn total_weight(&self)                                                            -> i32;
    }

    impl<V: StT + MtT + Hash + 'static> WeightedDirGraphMtEphIntTrait<V> for WeightedDirGraphMtEphInt<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> Self {
            let labeled_edges = edges
                .iter()
                .map(|Triple(from, to, weight)| LabEdge(from.clone(), to.clone(), *weight))
                .collect::<Vec<_>>();

            let mut edge_set = SetStEph::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }

            Self::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// Add a weighted edge to the graph
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_weighted_edge(&mut self, from: V, to: V, weight: i32) { self.add_labeled_arc(from, to, weight); }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_edge_weight(&self, from: &V, to: &V) -> Option<i32> { self.get_arc_label(from, to).copied() }

        /// Get all weighted edges as (from, to, weight) tuples
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn weighted_edges(&self) -> SetStEph<Triple<V, V, i32>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                edges.insert(Triple(
                    labeled_edge.0.clone(),
                    labeled_edge.1.clone(),
                    labeled_edge.2,
                ));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn out_neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, i32>> {
            // PARALLEL: filter weighted arcs using divide-and-conquer
            let arcs = self.labeled_arcs().iter().cloned().collect::<Vec<LabEdge<V, i32>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_out<V: StT + MtT + Hash + 'static>(arcs: Vec<LabEdge<V, i32>>, v: V) -> SetStEph<Pair<V, i32>> {
                let n = arcs.len();
                if n == 0 {
                    return SetStEph::empty();
                }
                if n == 1 {
                    return if arcs[0].0 == v {
                        let mut s = SetStEph::empty();
                        s.insert(Pair(arcs[0].1.clone(), arcs[0].2));
                        s
                    } else {
                        SetStEph::empty()
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_out(left_arcs, v_left), move || parallel_out(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_out(arcs, v.clone())
        }

        /// Get incoming neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn in_neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, i32>> {
            // PARALLEL: filter weighted arcs using divide-and-conquer
            let arcs = self.labeled_arcs().iter().cloned().collect::<Vec<LabEdge<V, i32>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_in<V: StT + MtT + Hash + 'static>(arcs: Vec<LabEdge<V, i32>>, v: V) -> SetStEph<Pair<V, i32>> {
                let n = arcs.len();
                if n == 0 {
                    return SetStEph::empty();
                }
                if n == 1 {
                    return if arcs[0].1 == v {
                        let mut s = SetStEph::empty();
                        s.insert(Pair(arcs[0].0.clone(), arcs[0].2));
                        s
                    } else {
                        SetStEph::empty()
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_in(left_arcs, v_left), move || parallel_in(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_in(arcs, v.clone())
        }

        /// Get the total weight of all edges
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential sum
        fn total_weight(&self) -> i32 { self.labeled_arcs().iter().map(|edge| edge.2).sum() }
    }

    /// Macro requires explicit Triple wrappers: `E: [Triple(from, to, weight), ...]`
    /// No automatic wrapping - enforces type safety at call site.
    #[macro_export]
    macro_rules! WeightedDirGraphMtEphIntLit {
        () => {{
            $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            $crate::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges)
        }};
    }
}
