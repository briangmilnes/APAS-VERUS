//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Edge Set Graph representation (persistent, single-threaded).
//! G = (V, E) where V is a set of vertices and E ⊆ V × V is a set of edges.

pub mod EdgeSetGraphStPer {

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct EdgeSetGraphStPer<V: StT + Ord> {
        vertices: AVLTreeSetStPer<V>,
        edges: AVLTreeSetStPer<Pair<V, V>>,
    }

    pub trait EdgeSetGraphStPerTrait<V: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                                            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_edges(&self)                                                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                                                -> &AVLTreeSetStPer<V>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self)                                                                   -> &AVLTreeSetStPer<Pair<V, V>>;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                                                  -> B;
        /// claude-4-sonet: Work Θ(|E| log |V|), Span Θ(|E| log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V)                                                    -> AVLTreeSetStPer<V>;
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)                                                       -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V)                                                     -> Self;
        /// claude-4-sonet: Work Θ(|E| log |E|), Span Θ(|E| log |E|), Parallelism Θ(1)
        fn delete_vertex(&self, v: &V)                                                    -> Self;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V)                                                 -> Self;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V)                                               -> Self;
    }

    impl<V: StT + Ord> EdgeSetGraphStPerTrait<V> for EdgeSetGraphStPer<V> {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — creates two empty AVL sets.
        fn empty() -> Self {
            EdgeSetGraphStPer {
                vertices: AVLTreeSetStPer::empty(),
                edges: AVLTreeSetStPer::empty(),
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing sets.
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> Self {
            EdgeSetGraphStPer { vertices: v, edges: e }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to AVL tree size.
        fn num_vertices(&self) -> N { self.vertices.size() }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to AVL tree size.
        fn num_edges(&self) -> N { self.edges.size() }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — returns reference.
        fn vertices(&self) -> &AVLTreeSetStPer<V> { &self.vertices }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — returns reference.
        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>> { &self.edges }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1, since lg m = O(lg n)]
        /// - Claude-Opus-4.6: Work Θ(lg m), Span Θ(lg m) — agrees with APAS.
        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1, mapping over neighbors]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — sequential filter + insert loop; span not parallel.
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStPer<V> {
            let u_clone = u.clone();
            let filtered = self.edges.filter(|edge| edge.0 == u_clone);
            let mut neighbors = AVLTreeSetStPer::empty();
            let seq = filtered.to_seq();
            for i in 0..seq.length() {
                let Pair(_, v) = seq.nth(i);
                neighbors = neighbors.insert(v.clone());
            }
            neighbors
        }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1, degree of vertex]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors which is sequential.
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1, insert vertex]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn insert_vertex(&self, v: V) -> Self {
            EdgeSetGraphStPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1, isolated vertex deletion]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — filters all edges, sequential; APAS assumes isolated.
        fn delete_vertex(&self, v: &V) -> Self {
            let v_clone = v.clone();
            let new_vertices = self.vertices.delete(&v_clone);
            let new_edges = self.edges.filter(|edge| {
                let Pair(u, w) = edge;
                u != &v_clone && w != &v_clone
            });
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1, insert edge]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn insert_edge(&self, u: V, v: V) -> Self {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1, delete edge]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn delete_edge(&self, u: &V, v: &V) -> Self {
            let new_edges = self.edges.delete(&Pair(u.clone(), v.clone()));
            EdgeSetGraphStPer {
                vertices: self.vertices.clone(),
                edges: new_edges,
            }
        }
    }
}
