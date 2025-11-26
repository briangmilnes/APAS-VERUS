//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled edge filtering (neighbors) is parallel.

pub mod LabUnDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabUnDirGraphMtEph<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> {
        vertices: SetStEph<V>,
        labeled_edges: SetStEph<LabEdge<V, L>>,
    }

    pub trait LabUnDirGraphMtEphTrait<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                                                                        -> Self;
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1) - sequential
        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn vertices(&self)                                                                                -> &SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn labeled_edges(&self)                                                                           -> &SetStEph<LabEdge<V, L>>;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential map
        fn edges(&self)                                                                                   -> SetStEph<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_vertex(&mut self, v: V);
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential search
        fn get_edge_label(&self, v1: &V, v2: &V)                                                          -> Option<&L>;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential search
        fn has_edge(&self, v1: &V, v2: &V)                                                                -> bool;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|) - parallel divide-and-conquer filter
        fn neighbors(&self, v: &V)                                                                        -> SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn normalize_edge(v1: V, v2: V)                                                                   -> LabEdge<V, L>;
    }

    impl<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static> LabUnDirGraphMtEphTrait<V, L>
        for LabUnDirGraphMtEph<V, L>
    {
        fn empty() -> Self {
            LabUnDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_edges: SetStEph::empty(),
            }
        }

        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> Self {
            LabUnDirGraphMtEph {
                vertices,
                labeled_edges,
            }
        }

        fn vertices(&self) -> &SetStEph<V> { &self.vertices }

        fn labeled_edges(&self) -> &SetStEph<LabEdge<V, L>> { &self.labeled_edges }

        fn edges(&self) -> SetStEph<Edge<V>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_edges.iter() {
                edges.insert(Edge(labeled_edge.0.clone_mt(), labeled_edge.1.clone_mt()));
            }
            edges
        }

        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {
            self.vertices.insert(v1.clone_mt());
            self.vertices.insert(v2.clone_mt());
            let normalized_edge = if v1 <= v2 {
                LabEdge(v1, v2, label)
            } else {
                LabEdge(v2, v1, label)
            };
            self.labeled_edges.insert(normalized_edge);
        }

        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {
            // Check both orientations since this is undirected
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return Some(&labeled_edge.2);
                }
            }
            None
        }

        fn has_edge(&self, v1: &V, v2: &V) -> bool {
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return true;
                }
            }
            false
        }

        fn neighbors(&self, v: &V) -> SetStEph<V> {
            // PARALLEL: filter labeled edges using divide-and-conquer
            let edges = self.labeled_edges.iter().cloned().collect::<Vec<LabEdge<V, L>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_neighbors<V: HashOrd + MtT + 'static, L: StTInMtT + Hash + 'static>(
                edges: Vec<LabEdge<V, L>>,
                v: V,
            ) -> SetStEph<V> {
                let n = edges.len();
                if n == 0 {
                    return SetStEph::empty();
                }
                if n == 1 {
                    if edges[0].0 == v {
                        let mut s = SetStEph::empty();
                        s.insert(edges[0].1.clone_mt());
                        return s;
                    } else if edges[0].1 == v {
                        let mut s = SetStEph::empty();
                        s.insert(edges[0].0.clone_mt());
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

        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {
            // This method signature doesn't make sense for LabEdge without a label
            // This is a design issue - we need the label to create a LabEdge
            // For now, we'll panic to indicate this needs to be fixed
            panic!("normalize_edge cannot create LabEdge without a label - method signature needs revision")
        }
    }

    impl<V: HashOrd + MtT, L: StTInMtT + Hash> Display for LabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph(V: {}, E: {})", self.vertices, self.labeled_edges)
        }
    }

    impl<V: HashOrd + MtT, L: StTInMtT + Hash> Debug for LabUnDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabUnDirGraph {{ vertices: {:?}, labeled_edges: {:?} }}",
                self.vertices, self.labeled_edges
            )
        }
    }

    #[macro_export]
    macro_rules! LabUnDirGraphMtEphLit {
        () => {{
            < $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEph<_, _> as $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($v1:expr, $v2:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_edges = {
                let mut edges = $crate::Chap05::SetStEph::SetStEph::SetStEph::empty();
                $(
                    let normalized_edge = if $v1 <= $v2 {
                        $crate::Types::Types::LabEdge($v1, $v2, $label)
                    } else {
                        $crate::Types::Types::LabEdge($v2, $v1, $label)
                    };
                    edges.insert(normalized_edge);
                )*
                edges
            };
            < $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEph<_, _> as $crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEphTrait<_, _> >::from_vertices_and_labeled_edges(vertices, labeled_edges)
        }};
    }
}
