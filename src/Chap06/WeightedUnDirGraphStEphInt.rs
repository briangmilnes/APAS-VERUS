//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighted Undirected Graph (ephemeral) with integer weights - Single-threaded version.

pub mod WeightedUnDirGraphStEphInt {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    pub type WeightedUnDirGraphStEphInt<V> = LabUnDirGraphStEph<V, i32>;

    /// Convenience functions for weighted undirected graphs with integer weights
    pub trait WeightedUnDirGraphStEphIntTrait<V: StT + Hash + Ord> {
        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> Self;
        fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32);
        fn get_edge_weight(&self, v1: &V, v2: &V)                                         -> Option<i32>;
        fn weighted_edges(&self)                                                          -> SetStEph<Triple<V, V, i32>>;
        fn neighbors_weighted(&self, v: &V)                                               -> SetStEph<Pair<V, i32>>;
        fn total_weight(&self)                                                            -> i32;
        fn vertex_degree(&self, v: &V)                                                    -> usize;
        fn is_connected(&self)                                                            -> bool;
    }

    impl<V: StT + Hash + Ord> WeightedUnDirGraphStEphIntTrait<V> for WeightedUnDirGraphStEphInt<V> {
        /// Create from vertices and weighted edges
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_weighted_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, i32>>) -> Self {
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

        /// Add a weighted edge to the graph (undirected)
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_weighted_edge(&mut self, v1: V, v2: V, weight: i32) { self.add_labeled_edge(v1, v2, weight); }

        /// Get the weight of an edge, if it exists
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential search
        fn get_edge_weight(&self, v1: &V, v2: &V) -> Option<i32> { self.get_edge_label(v1, v2).copied() }

        /// Get all weighted edges as (v1, v2, weight) tuples
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential map
        fn weighted_edges(&self) -> SetStEph<Triple<V, V, i32>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_edges().iter() {
                edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2));
            }
            edges
        }

        /// Get neighbors with weights
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential filter
        fn neighbors_weighted(&self, v: &V) -> SetStEph<Pair<V, i32>> {
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
        fn total_weight(&self) -> i32 { self.labeled_edges().iter().map(|edge| edge.2).sum() }

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
    }

    /// Macro requires explicit Triple wrappers: `E: [Triple(v1, v2, weight), ...]`
    /// No automatic wrapping - enforces type safety at call site.
    #[macro_export]
    macro_rules! WeightedUnDirGraphStEphIntLit {
        () => {{
            $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            $crate::Chap06::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::from_weighted_edges(vertices, edges)
        }};
    }
}
