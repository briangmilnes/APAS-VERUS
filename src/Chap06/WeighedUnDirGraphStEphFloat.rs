// DISABLED: OrderedFloat only provides PartialEq, not Eq - interesting when we get there.
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighed Undirected Graph (ephemeral) with floating-point weights - Single-threaded version.
//!
//! This module provides weighed undirected graphs using `OrderedFloat<f64>` for edge weights,
//! enabling reliable hashing and ordering of floating-point values including NaN and Infinity.
//!
//! # Examples
//!
//! ```rust
//! use apas_ai::Chap06::WeighedUnDirGraphStEphFloat::WeighedUnDirGraphStEphFloat::*;
//! use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait;
//! use apas_ai::Chap05::SetStEph::SetStEph::*;
//! use apas_ai::WeighedUnDirGraphStEphFloatLit;
//! use ordered_float::OrderedFloat;
//!
//! // Create graph using API
//! let mut graph = WeighedUnDirGraphStEphFloat::empty();
//! graph.add_weighed_edge("A", "B", OrderedFloat(3.14));
//! graph.add_weighed_edge("B", "C", OrderedFloat(2.71));
//!
//! // Create graph using macro with APAS notation (E: for undirected edges)
//! use apas_ai::Types::Types::Triple;
//! let graph_macro = WeighedUnDirGraphStEphFloatLit!(
//!     V: ["A", "B", "C"],
//!     E: [Triple("A", "B", OrderedFloat(3.14)), Triple("B", "C", OrderedFloat(2.71))]
//! );
//!
//! // Query operations
//! let weight = graph.get_edge_weight(&"A", &"B"); // Returns Option<OrderedFloat<f64>>
//! let total = graph.total_weight(); // Returns OrderedFloat<f64>
//! let neighbors = graph.neighbors_weighed(&"A"); // Returns SetStEph<Pair<V, OrderedFloat<f64>>>
//! ```

pub mod WeighedUnDirGraphStEphFloat {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    pub type WeighedUnDirGraphStEphFloat<V> = LabUnDirGraphStEph<V, OrderedF64>;

    /// Convenience functions for weighed undirected graphs with floating-point weights
    pub trait WeighedUnDirGraphStEphFloatTrait<V: StT + Hash + Ord> {
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self;
        fn add_weighed_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>);
        fn get_edge_weight(&self, v1: &V, v2: &V)                                                       -> Option<OrderedFloat<f64>>;
        fn weighed_edges(&self)                                                                        -> SetStEph<Triple<V, V, OrderedFloat<f64>>>;
        fn neighbors_weighed(&self, v: &V)                                                             -> SetStEph<Pair<V, OrderedFloat<f64>>>;
        fn total_weight(&self)                                                                          -> OrderedFloat<f64>;
        fn vertex_degree(&self, v: &V)                                                                  -> usize;
        fn is_connected(&self)                                                                          -> bool;
        fn min_weight_edge(&self)                                                                       -> Option<(V, V, OrderedFloat<f64>)>;
        fn max_weight_edge(&self)                                                                       -> Option<(V, V, OrderedFloat<f64>)>;
    }

    impl<V: StT + Hash + Ord> WeighedUnDirGraphStEphFloatTrait<V> for WeighedUnDirGraphStEphFloat<V> {
        /// Create from vertices and weighed edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self {
            let labeled_edges = edges
                .iter()
                .map(|Triple(v1, v2, weight)| LabEdge(v1.clone(), v2.clone(), *weight))
                .collect::<Vec<_>>();

            let mut edge_set = SetStEph::empty();
            for edge in labeled_edges {
                edge_set.insert(edge);
            }

            Self::from_vertices_and_labeled_edges(vertices, edge_set)
        }

        /// Add a weighed edge to the graph (undirected)
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_weighed_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>) {
            self.add_labeled_edge(v1, v2, weight);
        }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential search
        fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<OrderedFloat<f64>> { self.get_edge_label(v1, v2).copied() }

        /// Get all weighed edges as (v1, v2, weight) tuples
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential map
        fn weighed_edges(&self) -> SetStEph<Triple<V, V, OrderedFloat<f64>>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_edges().iter() {
                edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
            }
            edges
        }

        /// Get neighbors with weights
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential filter
        fn neighbors_weighed(&self, v: &V) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
            let mut neighbors = SetStEph::empty();
            for labeled_edge in self.labeled_edges().iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert(Pair(labeled_edge.1.clone(), labeled_edge.2));
                } else if labeled_edge.1 == *v {
                    neighbors.insert(Pair(labeled_edge.0.clone(), labeled_edge.2));
                }
            }
            neighbors
        }

        /// Get the total weight of all edges
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential sum
        fn total_weight(&self) -> OrderedFloat<f64> {
            self.labeled_edges()
                .iter()
                .map(|edge| edge.2)
                .fold(OrderedFloat(0.0), |acc, w| acc + w)
        }

        /// Get the degree of a vertex (number of incident edges)
        fn vertex_degree(&self, v: &V) -> usize { self.neighbors(v).size() }

        /// Check if the graph is connected (all vertices reachable from any vertex)
        fn is_connected(&self) -> bool {
            if self.vertices().size() == 0 {
                return true; // Empty graph is considered connected
            }

            // Simple connectivity check using DFS from first vertex
            let mut visited = SetStEph::empty();
            let mut stack = Vec::new();

            if let Some(start) = self.vertices().iter().next() {
                stack.push(start.clone());

                while let Some(current) = stack.pop() {
                    if !visited.mem(&current) {
                        visited.insert(current.clone());
                        for neighbor in self.neighbors(&current).iter() {
                            if !visited.mem(neighbor) {
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }
            }

            visited.size() == self.vertices().size()
        }

        /// Get the minimum weight edge
        fn min_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {
            self.labeled_edges()
                .iter()
                .min_by_key(|edge| edge.2)
                .map(|edge| (edge.0.clone(), edge.1.clone(), edge.2))
        }

        /// Get the maximum weight edge
        fn max_weight_edge(&self) -> Option<(V, V, OrderedFloat<f64>)> {
            self.labeled_edges()
                .iter()
                .max_by_key(|edge| edge.2)
                .map(|edge| (edge.0.clone(), edge.1.clone(), edge.2))
        }
    }

    /// Macro requires explicit Triple wrappers: `E: [Triple(v1, v2, OrderedFloat(weight)), ...]`
    /// No automatic wrapping - enforces type safety at call site.
    #[macro_export]
    macro_rules! WeighedUnDirGraphStEphFloatLit {
        () => {{
            $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            $crate::Chap06::WeighedUnDirGraphStEphFloat::WeighedUnDirGraphStEphFloat::WeighedUnDirGraphStEphFloat::from_weighed_edges(vertices, edges)
        }};
    }
}
