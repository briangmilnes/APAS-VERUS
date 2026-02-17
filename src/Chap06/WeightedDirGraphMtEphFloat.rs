//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Directed Graph (ephemeral) with floating-point weights - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for weighted neighbor operations.
//! Weighted arc filtering (out_neighbors_weighted, in_neighbors_weighted) is parallel.

pub mod WeightedDirGraphMtEphFloat {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    pub type WeightedDirGraphMtEphFloat<V> = LabDirGraphMtEph<V, OrderedF64>;

    /// Trait for weighted directed graph operations with floating-point weights (multi-threaded)
    pub trait WeightedDirGraphMtEphFloatTrait<V: StT + MtT + Hash + 'static> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self;

        /// Add a weighted edge to the graph
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>);

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_edge_weight(&self, from: &V, to: &V)                                                     -> Option<OrderedFloat<f64>>;

        /// Get all weighted edges as (from, to, weight) tuples
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn weighted_edges(&self)                                                                        -> SetStEph<Triple<V, V, OrderedFloat<f64>>>;

        /// Get outgoing neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn out_neighbors_weighted(&self, v: &V)                                                         -> SetStEph<Pair<V, OrderedFloat<f64>>>;

        /// Get incoming neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn in_neighbors_weighted(&self, v: &V)                                                          -> SetStEph<Pair<V, OrderedFloat<f64>>>;

        /// Get the total weight of all edges
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential sum
        fn total_weight(&self)                                                                          -> OrderedFloat<f64>;
    }

    /// Trait implementation for WeightedDirGraphMtEphFloat
    impl<V: StT + MtT + Hash + 'static> WeightedDirGraphMtEphFloatTrait<V> for WeightedDirGraphMtEphFloat<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self {
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
        fn add_weighted_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {
            self.add_labeled_arc(from, to, weight);
        }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {
            self.get_arc_label(from, to).copied()
        }

        /// Get all weighted edges as (from, to, weight) tuples
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn weighted_edges(&self) -> SetStEph<Triple<V, V, OrderedFloat<f64>>> {
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
        fn out_neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
            // PARALLEL: filter weighted arcs using divide-and-conquer
            let arcs = self.labeled_arcs().iter().cloned().collect::<Vec<LabEdge<V, OrderedF64>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_out<V: StT + MtT + Hash + 'static>(
                arcs: Vec<LabEdge<V, OrderedF64>>,
                v: V,
            ) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
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
        fn in_neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
            // PARALLEL: filter weighted arcs using divide-and-conquer
            let arcs = self.labeled_arcs().iter().cloned().collect::<Vec<LabEdge<V, OrderedF64>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_in<V: StT + MtT + Hash + 'static>(
                arcs: Vec<LabEdge<V, OrderedF64>>,
                v: V,
            ) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
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
        fn total_weight(&self) -> OrderedFloat<f64> {
            self.labeled_arcs()
                .iter()
                .map(|edge| edge.2)
                .fold(OrderedFloat(0.0), |acc, w| acc + w)
        }
    }

    /// Macro requires explicit Triple wrappers: `A: [Triple(from, to, OrderedFloat(weight)), ...]`
    /// No automatic wrapping - enforces type safety at call site.
    #[macro_export]
    macro_rules! WeightedDirGraphMtEphFloatLit {
        () => {{
            $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( $arc:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let arcs = $crate::SetLit![ $( $arc ),* ];
            $crate::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, arcs)
        }};
    }
}
