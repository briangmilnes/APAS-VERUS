// DISABLED: OrderedFloat only provides PartialEq, not Eq - interesting when we get there.
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighed Directed Graph (ephemeral) with floating-point weights - Single-threaded version.
//!
//! This module provides weighed directed graphs using `OrderedFloat<f64>` for edge weights,
//! enabling reliable hashing and ordering of floating-point values including NaN and Infinity.
//!
//! # Examples
//!
//! ```rust
//! use apas_ai::Chap06::WeighedDirGraphStEphFloat::WeighedDirGraphStEphFloat::*;
//! use apas_ai::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
//! use apas_ai::Chap05::SetStEph::SetStEph::*;
//! use apas_ai::WeighedDirGraphStEphFloatLit;
//! use ordered_float::OrderedFloat;
//!
//! // Create graph using API
//! let mut graph = WeighedDirGraphStEphFloat::empty();
//! graph.add_weighed_edge("A", "B", OrderedFloat(3.14));
//! graph.add_weighed_edge("B", "C", OrderedFloat(2.71));
//!
//! // Create graph using macro with APAS notation (A: for directed arcs)
//! use apas_ai::Types::Types::Triple;
//! let graph_macro = WeighedDirGraphStEphFloatLit!(
//!     V: ["A", "B", "C"],
//!     A: [Triple("A", "B", OrderedFloat(3.14)), Triple("B", "C", OrderedFloat(2.71))]
//! );
//!
//! // Query operations
//! let weight = graph.get_edge_weight(&"A", &"B"); // Returns Option<OrderedFloat<f64>>
//! let total = graph.total_weight(); // Returns OrderedFloat<f64>
//! let heavy_edges = graph.edges_above_weight(OrderedFloat(3.0));
//! ```

pub mod WeighedDirGraphStEphFloat {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Types::Types::*;

    use ordered_float::OrderedFloat;

    pub type WeighedDirGraphStEphFloat<V> = LabDirGraphStEph<V, OrderedF64>;

    /// Convenience functions for weighed directed graphs with floating-point weights
    pub trait WeighedDirGraphStEphFloatTrait<V: StT + Hash> {
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self;
        fn add_weighed_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>);
        fn get_edge_weight(&self, from: &V, to: &V)                                                     -> Option<OrderedFloat<f64>>;
        fn weighed_edges(&self)                                                                        -> SetStEph<Triple<V, V, OrderedFloat<f64>>>;
        fn out_neighbors_weighed(&self, v: &V)                                                         -> SetStEph<Pair<V, OrderedFloat<f64>>>;
        fn in_neighbors_weighed(&self, v: &V)                                                          -> SetStEph<Pair<V, OrderedFloat<f64>>>;
        fn total_weight(&self)                                                                          -> OrderedFloat<f64>;
        fn edges_above_weight(&self, threshold: OrderedFloat<f64>)                                      -> SetStEph<Triple<V, V, OrderedFloat<f64>>>;
        fn edges_below_weight(&self, threshold: OrderedFloat<f64>)                                      -> SetStEph<Triple<V, V, OrderedFloat<f64>>>;
        fn min_weight_edge(&self)                                                                       -> Option<Triple<V, V, OrderedFloat<f64>>>;
        fn max_weight_edge(&self)                                                                       -> Option<Triple<V, V, OrderedFloat<f64>>>;
        fn scale_weights(&mut self, factor: OrderedFloat<f64>);
    }

    impl<V: StT + Hash> WeighedDirGraphStEphFloatTrait<V> for WeighedDirGraphStEphFloat<V> {
        /// Create from vertices and weighed edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self {
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

        /// Add a weighed edge to the graph
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: OrderedFloat<f64>) {
            self.add_labeled_arc(from, to, weight);
        }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_edge_weight(&self, from: &V, to: &V) -> Option<OrderedFloat<f64>> {
            self.get_arc_label(from, to).copied()
        }

        /// Get all weighed edges as (from, to, weight) tuples
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn weighed_edges(&self) -> SetStEph<Triple<V, V, OrderedFloat<f64>>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
            }
            edges
        }

        /// Get outgoing neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        fn out_neighbors_weighed(&self, v: &V) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
            let mut neighbors = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert(Pair(labeled_edge.1.clone(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get incoming neighbors with weights
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        fn in_neighbors_weighed(&self, v: &V) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
            let mut neighbors = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.1 == *v {
                    neighbors.insert(Pair(labeled_edge.0.clone(), labeled_edge.2));
                }
            }
            neighbors
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

        /// Get edges with weight greater than threshold
        fn edges_above_weight(&self, threshold: OrderedFloat<f64>) -> SetStEph<Triple<V, V, OrderedFloat<f64>>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.2 > threshold {
                    edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
                }
            }
            edges
        }

        /// Get edges with weight less than threshold
        fn edges_below_weight(&self, threshold: OrderedFloat<f64>) -> SetStEph<Triple<V, V, OrderedFloat<f64>>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_arcs().iter() {
                if labeled_edge.2 < threshold {
                    edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
                }
            }
            edges
        }

        /// Get the minimum weight edge
        fn min_weight_edge(&self) -> Option<Triple<V, V, OrderedFloat<f64>>> {
            self.labeled_arcs()
                .iter()
                .min_by_key(|edge| edge.2)
                .map(|edge| Triple(edge.0.clone(), edge.1.clone(), edge.2))
        }

        /// Get the maximum weight edge
        fn max_weight_edge(&self) -> Option<Triple<V, V, OrderedFloat<f64>>> {
            self.labeled_arcs()
                .iter()
                .max_by_key(|edge| edge.2)
                .map(|edge| Triple(edge.0.clone(), edge.1.clone(), edge.2))
        }

        /// Scale all weights by a factor
        fn scale_weights(&mut self, factor: OrderedFloat<f64>) {
            let current_edges = self.labeled_arcs().iter().cloned().collect::<Vec<_>>();

            // Clear current edges and re-add with scaled weights
            *self = Self::empty();
            let vertices = current_edges.iter().map(|e| e.0.clone()).collect::<Vec<_>>();
            for v in vertices {
                self.add_vertex(v);
            }

            // Add scaled edges
            for edge in current_edges {
                self.add_labeled_arc(edge.0, edge.1, edge.2 * factor);
            }
        }
    }

    /// Macro requires explicit Triple wrappers: `A: [Triple(from, to, OrderedFloat(weight)), ...]`
    /// No automatic wrapping - enforces type safety at call site.
    #[macro_export]
    macro_rules! WeighedDirGraphStEphFloatLit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( $arc:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let arcs = $crate::SetLit![ $( $arc ),* ];
            $crate::Chap06::WeighedDirGraphStEphFloat::WeighedDirGraphStEphFloat::WeighedDirGraphStEphFloat::from_weighed_edges(vertices, arcs)
        }};
    }
}
