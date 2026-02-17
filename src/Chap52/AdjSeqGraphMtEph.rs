//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (ephemeral, multi-threaded).

pub mod AdjSeqGraphMtEph {

    use std::sync::Arc;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjSeqGraphMtEph {
        adj: ArraySeqMtEphS<ArraySeqMtEphS<N>>,
    }

    pub trait AdjSeqGraphMtEphTrait {
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn new(n: N)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)         -> N;
        /// claude-4-sonet: Work Θ(Σ deg(v)), Span Θ(n), Parallelism Θ(|E|/n)
        fn num_edges(&self)            -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)), Parallelism Θ(1)
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_neighbors(&self, u: N)  -> ArraySeqMtEphS<N>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_degree(&self, u: N)     -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)) with locking
        fn set_edge(&mut self, u: N, v: N, exists: B);
    }

    impl AdjSeqGraphMtEphTrait for AdjSeqGraphMtEph {
        fn new(n: N) -> Self {
            let empty_list = ArraySeqMtEphS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphMtEph {
                adj: ArraySeqMtEphS::from_vec(adj_lists),
            }
        }

        fn num_vertices(&self) -> N { self.adj.length() }

        fn num_edges(&self) -> N {
            let n = self.adj.length();
            let mut count = 0;
            for i in 0..n {
                let adj_list = self.adj.nth(i);
                count += adj_list.length();
            }
            count
        }

        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.adj.length() {
                return false;
            }
            let neighbors = self.adj.nth(u).clone();
            for i in 0..neighbors.length() {
                if neighbors.nth(i).clone() == v {
                    return true;
                }
            }
            false
        }

        fn out_neighbors(&self, u: N) -> ArraySeqMtEphS<N> { self.adj.nth(u).clone() }

        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length().clone() }

        fn set_edge(&mut self, u: N, v: N, exists: B) {
            if u >= self.adj.length() || v >= self.adj.length() {
                return;
            }
            let old_neighbors = self.adj.nth(u).clone();
            if exists {
                let mut found = false;
                for i in 0..old_neighbors.length() {
                    if old_neighbors.nth(i).clone() == v {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut new_neighbors_vec = Vec::<N>::with_capacity(old_neighbors.length() + 1);
                    for i in 0..old_neighbors.length() {
                        new_neighbors_vec.push(*old_neighbors.nth(i));
                    }
                    new_neighbors_vec.push(v);
                    let _ = self.adj.set(u, ArraySeqMtEphS::from_vec(new_neighbors_vec));
                }
            } else {
                let mut new_neighbors_vec = Vec::<N>::new();
                for i in 0..old_neighbors.length() {
                    let neighbor = old_neighbors.nth(i).clone();
                    if neighbor != v {
                        new_neighbors_vec.push(neighbor);
                    }
                }
                let _ = self.adj.set(u, ArraySeqMtEphS::from_vec(new_neighbors_vec));
            }
        }
    }
}
