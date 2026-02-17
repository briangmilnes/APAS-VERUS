//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Edge Set Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where V is a set of vertices and A: ⊆ V × V is a set of directed arcs.
//!
//! Uses AVLTreeSetMtPer with Arc-based backing for PARALLEL operations.

pub mod EdgeSetGraphMtPer {

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct EdgeSetGraphMtPer<V: StTInMtT + Ord + 'static> {
        vertices: AVLTreeSetMtPer<V>,
        edges: AVLTreeSetMtPer<Pair<V, V>>,
    }

    pub trait EdgeSetGraphMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                                            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_edges(&self)                                                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                                                -> &AVLTreeSetMtPer<V>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self)                                                                   -> &AVLTreeSetMtPer<Pair<V, V>>;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                                                  -> B;
        /// claude-4-sonet: Work Θ(|E| log |V|), Span Θ(log |E| × log |V|), Parallelism Θ(|E|/log |E|)
        fn out_neighbors(&self, u: &V)                                                    -> AVLTreeSetMtPer<V>;
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|)
        fn out_degree(&self, u: &V)                                                       -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V)                                                     -> Self;
        /// claude-4-sonet: Work Θ(|E| log |V| + |E| log |E|), Span Θ(log |E| × log |V|), Parallelism Θ(|E|/log |E|)
        fn delete_vertex(&self, v: &V)                                                    -> Self;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V)                                                 -> Self;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V)                                               -> Self;
    }

    impl<V: StTInMtT + Ord + 'static> EdgeSetGraphMtPerTrait<V> for EdgeSetGraphMtPer<V> {
        fn empty() -> Self {
            EdgeSetGraphMtPer {
                vertices: AVLTreeSetMtPer::empty(),
                edges: AVLTreeSetMtPer::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> Self {
            EdgeSetGraphMtPer { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetMtPer<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetMtPer<Pair<V, V>> { &self.edges }

        // Work: Θ(log |E|), Span: Θ(log |E|)
        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        // PARALLEL: Work: Θ(|E|), Span: Θ(log |E|) - TRUE parallel filter
        fn out_neighbors(&self, u: &V) -> AVLTreeSetMtPer<V> {
            let u_clone = u.clone();
            let filtered = self.edges.filter(move |edge| edge.0 == u_clone);
            let mut neighbors = AVLTreeSetMtPer::empty();
            let seq = filtered.to_seq();
            for i in 0..seq.length() {
                let Pair(_, v) = seq.nth(i);
                neighbors = neighbors.insert(v.clone());
            }
            neighbors
        }

        // Work: Θ(|E|), Span: Θ(log |E|)
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        // Work: Θ(log |V|), Span: Θ(log |V|)
        fn insert_vertex(&self, v: V) -> Self {
            EdgeSetGraphMtPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        // PARALLEL: Work: Θ(|E| log |E|), Span: Θ(log² |E|) - TRUE parallel filter
        fn delete_vertex(&self, v: &V) -> Self {
            let v_clone = v.clone();
            let new_vertices = self.vertices.delete(&v_clone);
            let v_clone2 = v_clone.clone();
            let new_edges = self.edges.filter(move |edge| {
                let Pair(u, w) = edge;
                u != &v_clone2 && w != &v_clone2
            });
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        // Work: Θ(log |V| + log |E|), Span: Θ(log |V| + log |E|)
        fn insert_edge(&self, u: V, v: V) -> Self {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        // Work: Θ(log |E|), Span: Θ(log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> Self {
            EdgeSetGraphMtPer {
                vertices: self.vertices.clone(),
                edges: self.edges.delete(&Pair(u.clone(), v.clone())),
            }
        }
    }

    impl<V: StTInMtT + Ord + 'static> Default for EdgeSetGraphMtPer<V> {
        fn default() -> Self { Self::empty() }
    }
}
