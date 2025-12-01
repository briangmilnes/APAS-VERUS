// DISABLED: OrderedFloat only provides PartialEq, not Eq - interesting when we get there.
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Weighed Undirected Graph (ephemeral) with floating-point weights - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for weighed neighbor operations.
//! Weighed edge filtering (neighbors_weighed) is parallel.

pub mod WeighedUnDirGraphMtEphFloat {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    pub type WeighedUnDirGraphMtEphFloat<V> = LabUnDirGraphMtEph<V, OrderedF64>;

    /// Convenience functions for weighed undirected graphs with floating-point weights (multi-threaded)
    pub trait WeighedUnDirGraphMtEphFloatTrait<V: HashOrd + MtT + 'static> {
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<Triple<V, V, OrderedFloat<f64>>>) -> Self;
        fn add_weighed_edge(&mut self, v1: V, v2: V, weight: OrderedFloat<f64>);
        fn get_edge_weight(&self, v1: &V, v2: &V)                                                       -> Option<OrderedFloat<f64>>;
        fn weighed_edges(&self)                                                                        -> SetStEph<Triple<V, V, OrderedFloat<f64>>>;
        fn neighbors_weighed(&self, v: &V)                                                             -> SetStEph<Pair<V, OrderedFloat<f64>>>;
        fn total_weight(&self)                                                                          -> OrderedFloat<f64>;
        fn vertex_degree(&self, v: &V)                                                                  -> usize;
    }

    impl<V: HashOrd + MtT + 'static> WeighedUnDirGraphMtEphFloatTrait<V> for WeighedUnDirGraphMtEphFloat<V> {
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
                edges.insert(Triple(
                    labeled_edge.0.clone_mt(),
                    labeled_edge.1.clone_mt(),
                    labeled_edge.2,
                ));
            }
            edges
        }

        /// Get neighbors with weights
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|) - parallel divide-and-conquer filter
        fn neighbors_weighed(&self, v: &V) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
            // PARALLEL: filter weighed edges using divide-and-conquer
            let edges = self.labeled_edges().iter().cloned().collect::<Vec<LabEdge<V, OrderedF64>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_neighbors<V: HashOrd + MtT + 'static>(
                edges: Vec<LabEdge<V, OrderedF64>>,
                v: V,
            ) -> SetStEph<Pair<V, OrderedFloat<f64>>> {
                let n = edges.len();
                if n == 0 {
                    return SetStEph::empty();
                }
                if n == 1 {
                    if edges[0].0 == v {
                        let mut s = SetStEph::empty();
                        s.insert(Pair(edges[0].1.clone_mt(), edges[0].2));
                        return s;
                    } else if edges[0].1 == v {
                        let mut s = SetStEph::empty();
                        s.insert(Pair(edges[0].0.clone_mt(), edges[0].2));
                        return s;
                    }
                    return SetStEph::empty();
                }

                let mid = n / 2;
                let mut right_edges = edges;
                let left_edges = right_edges.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_neighbors(left_edges, v_left), move || {
                        parallel_neighbors(right_edges, v_right)
                    });

                left_result.union(&right_result)
            }

            parallel_neighbors(edges, v.clone_mt())
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
    }

    /// Macro requires explicit Triple wrappers: `E: [Triple(v1, v2, OrderedFloat(weight)), ...]`
    /// No automatic wrapping - enforces type safety at call site.
    #[macro_export]
    macro_rules! WeighedUnDirGraphMtEphFloatLit {
        () => {{
            $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            $crate::Chap06::WeighedUnDirGraphMtEphFloat::WeighedUnDirGraphMtEphFloat::WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges)
        }};
    }
}
