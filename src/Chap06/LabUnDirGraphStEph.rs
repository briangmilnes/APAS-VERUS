//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges.

pub mod LabUnDirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabUnDirGraphStEph<V: HashOrd, L: StT + Hash> {
        vertices: SetStEph<V>,
        labeled_edges: SetStEph<LabEdge<V, L>>,
    }

    pub trait LabUnDirGraphStEphTrait<V: HashOrd, L: StT + Hash> {
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
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1) - sequential filter
        fn neighbors(&self, v: &V)                                                                        -> SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn normalize_edge(v1: V, v2: V)                                                                   -> LabEdge<V, L>;
    }

    impl<V: HashOrd, L: StT + Hash> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L> {
        fn empty() -> Self {
            LabUnDirGraphStEph {
                vertices: SetStEph::empty(),
                labeled_edges: SetStEph::empty(),
            }
        }

        fn from_vertices_and_labeled_edges(vertices: SetStEph<V>, labeled_edges: SetStEph<LabEdge<V, L>>) -> Self {
            LabUnDirGraphStEph {
                vertices,
                labeled_edges,
            }
        }

        fn vertices(&self) -> &SetStEph<V> { &self.vertices }

        fn labeled_edges(&self) -> &SetStEph<LabEdge<V, L>> { &self.labeled_edges }

        fn edges(&self) -> SetStEph<Edge<V>> {
            let mut edges = SetStEph::empty();
            for labeled_edge in self.labeled_edges.iter() {
                edges.insert(Edge(labeled_edge.0.clone(), labeled_edge.1.clone()));
            }
            edges
        }

        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {
            self.vertices.insert(v1.clone());
            self.vertices.insert(v2.clone());
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
            // Check both orientations since this is undirected
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return true;
                }
            }
            false
        }

        fn neighbors(&self, v: &V) -> SetStEph<V> {
            let mut neighbors = SetStEph::empty();
            for labeled_edge in self.labeled_edges.iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert(labeled_edge.1.clone());
                } else if labeled_edge.1 == *v {
                    neighbors.insert(labeled_edge.0.clone());
                }
            }
            neighbors
        }

        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {
            // This method signature doesn't make sense for LabEdge without a label
            // This is a design issue - we need the label to create a LabEdge
            // For now, we'll panic to indicate this needs to be fixed
            panic!("normalize_edge cannot create LabEdge without a label - method signature needs revision")
        }
    }

    impl<V: HashOrd, L: StT + Hash> Display for LabUnDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph(V: {}, E: {})", self.vertices, self.labeled_edges)
        }
    }

    impl<V: HashOrd, L: StT + Hash> Debug for LabUnDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabUnDirGraph {{ vertices: {:?}, labeled_edges: {:?} }}",
                self.vertices, self.labeled_edges
            )
        }
    }

    #[macro_export]
    macro_rules! LabUnDirGraphStEphLit {
        () => {{
            < $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph<_, _> as $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait<_, _> >::empty()
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
            < $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph<_, _> as $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait<_, _> >::from_vertices_and_labeled_edges(vertices, labeled_edges)
        }};
    }
}
