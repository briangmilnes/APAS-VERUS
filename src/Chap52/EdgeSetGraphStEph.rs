//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Edge Set Graph representation (ephemeral, single-threaded).

pub mod EdgeSetGraphStEph {

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct EdgeSetGraphStEph<V: StT + Ord> {
        vertices: AVLTreeSetStEph<V>,
        edges: AVLTreeSetStEph<Pair<V, V>>,
    }

    pub trait EdgeSetGraphStEphTrait<V: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_vertices_and_edges(v: AVLTreeSetStEph<V>, e: AVLTreeSetStEph<Pair<V, V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                                            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_edges(&self)                                                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                                                -> &AVLTreeSetStEph<V>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self)                                                                   -> &AVLTreeSetStEph<Pair<V, V>>;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                                                  -> B;
        /// claude-4-sonet: Work Θ(|E| log |V|), Span Θ(|E| log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V)                                                    -> AVLTreeSetStEph<V>;
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)                                                       -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&mut self, v: V);
        /// claude-4-sonet: Work Θ(|E| log |E|), Span Θ(|E| log |E|), Parallelism Θ(1)
        fn delete_vertex(&mut self, v: &V);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&mut self, u: V, v: V);
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn delete_edge(&mut self, u: &V, v: &V);
    }

    impl<V: StT + Ord> EdgeSetGraphStEphTrait<V> for EdgeSetGraphStEph<V> {
        fn empty() -> Self {
            EdgeSetGraphStEph {
                vertices: AVLTreeSetStEph::empty(),
                edges: AVLTreeSetStEph::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetStEph<V>, e: AVLTreeSetStEph<Pair<V, V>>) -> Self {
            EdgeSetGraphStEph { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStEph<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        fn out_neighbors(&self, u: &V) -> AVLTreeSetStEph<V> {
            let u_clone = u.clone();
            let filtered = self.edges.filter(|edge| edge.0 == u_clone);
            let mut neighbors = AVLTreeSetStEph::empty();
            let seq = filtered.to_seq();
            for i in 0..seq.length() {
                let Pair(_, v) = seq.nth(i);
                neighbors.insert(v.clone());
            }
            neighbors
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn delete_vertex(&mut self, v: &V) {
            let v_clone = v.clone();
            self.vertices.delete(&v_clone);
            let edges_to_remove = {
                let seq = self.edges.to_seq();
                let mut to_remove = Vec::<Pair<V, V>>::new();
                for i in 0..seq.length() {
                    let edge = seq.nth(i);
                    let Pair(u, w) = edge;
                    if u == &v_clone || w == &v_clone {
                        to_remove.push(edge.clone());
                    }
                }
                to_remove
            };
            for edge in edges_to_remove {
                self.edges.delete(&edge);
            }
        }

        fn insert_edge(&mut self, u: V, v: V) {
            self.vertices.insert(u.clone());
            self.vertices.insert(v.clone());
            self.edges.insert(Pair(u, v));
        }

        fn delete_edge(&mut self, u: &V, v: &V) { self.edges.delete(&Pair(u.clone(), v.clone())); }
    }
}
