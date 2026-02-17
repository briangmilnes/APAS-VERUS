//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (persistent, single-threaded).
//! G = (int seq) seq - for enumerable vertex sets V = {0, 1, ..., n-1}.

pub mod AdjSeqGraphStPer {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AdjSeqGraphStPer {
        adj: ArraySeqStPerS<ArraySeqStPerS<N>>,
    }

    pub trait AdjSeqGraphStPerTrait {
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn new(n: N)                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_seq(adj: ArraySeqStPerS<ArraySeqStPerS<N>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                              -> N;
        /// claude-4-sonet: Work Θ(Σ deg(v)), Span Θ(Σ deg(v)), Parallelism Θ(1)
        fn num_edges(&self)                                 -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)), Parallelism Θ(1)
        fn has_edge(&self, u: N, v: N)                      -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_neighbors(&self, u: N)                       -> &ArraySeqStPerS<N>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_degree(&self, u: N)                          -> N;
        /// claude-4-sonet: Work Θ(1 + deg(u)), Span Θ(1 + deg(u)), Parallelism Θ(1)
        fn insert_edge(&self, u: N, v: N)                   -> Self;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)), Parallelism Θ(1)
        fn delete_edge(&self, u: N, v: N)                   -> Self;
    }

    impl AdjSeqGraphStPerTrait for AdjSeqGraphStPer {
        // Work: Θ(n), Span: Θ(n) - create n empty neighbor lists
        fn new(n: N) -> Self {
            let empty_list = ArraySeqStPerS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphStPer {
                adj: ArraySeqStPerS::from_vec(adj_lists),
            }
        }

        fn from_seq(adj: ArraySeqStPerS<ArraySeqStPerS<N>>) -> Self { AdjSeqGraphStPer { adj } }

        // Work: Θ(1), Span: Θ(1)
        fn num_vertices(&self) -> N { self.adj.length() }

        // Work: Θ(n), Span: Θ(n) - sum all neighbor list lengths
        fn num_edges(&self) -> N {
            let n = self.adj.length();
            let mut count = 0;
            for i in 0..n {
                count += self.adj.nth(i).length();
            }
            count
        }

        // Work: Θ(d(u)), Span: Θ(d(u)) - linear search in neighbor list
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.adj.length() {
                return false;
            }
            let neighbors = self.adj.nth(u);
            for i in 0..neighbors.length() {
                if *neighbors.nth(i) == v {
                    return true;
                }
            }
            false
        }

        // Work: Θ(1), Span: Θ(1) - direct access
        fn out_neighbors(&self, u: N) -> &ArraySeqStPerS<N> { self.adj.nth(u) }

        // Work: Θ(1), Span: Θ(1)
        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length() }

        // Work: Θ(n), Span: Θ(n) - must copy entire sequence to update
        fn insert_edge(&self, u: N, v: N) -> Self {
            if u >= self.adj.length() || v >= self.adj.length() {
                return self.clone();
            }
            let old_neighbors = self.adj.nth(u);
            // Check if edge already exists
            for i in 0..old_neighbors.length() {
                if *old_neighbors.nth(i) == v {
                    return self.clone();
                }
            }
            // Add v to u's neighbor list
            let mut new_neighbors_vec = Vec::<N>::with_capacity(old_neighbors.length() + 1);
            for i in 0..old_neighbors.length() {
                new_neighbors_vec.push(*old_neighbors.nth(i));
            }
            new_neighbors_vec.push(v);
            let new_neighbors = ArraySeqStPerS::from_vec(new_neighbors_vec);

            // Update adjacency list
            let mut new_adj_vec = Vec::with_capacity(self.adj.length());
            for i in 0..self.adj.length() {
                if i == u {
                    new_adj_vec.push(new_neighbors.clone());
                } else {
                    new_adj_vec.push(self.adj.nth(i).clone());
                }
            }
            AdjSeqGraphStPer {
                adj: ArraySeqStPerS::from_vec(new_adj_vec),
            }
        }

        // Work: Θ(n), Span: Θ(n)
        fn delete_edge(&self, u: N, v: N) -> Self {
            if u >= self.adj.length() {
                return self.clone();
            }
            let old_neighbors = self.adj.nth(u);
            let mut new_neighbors_vec = Vec::<N>::new();
            for i in 0..old_neighbors.length() {
                let neighbor = *old_neighbors.nth(i);
                if neighbor != v {
                    new_neighbors_vec.push(neighbor);
                }
            }
            let new_neighbors = ArraySeqStPerS::from_vec(new_neighbors_vec);

            let mut new_adj_vec = Vec::with_capacity(self.adj.length());
            for i in 0..self.adj.length() {
                if i == u {
                    new_adj_vec.push(new_neighbors.clone());
                } else {
                    new_adj_vec.push(self.adj.nth(i).clone());
                }
            }
            AdjSeqGraphStPer {
                adj: ArraySeqStPerS::from_vec(new_adj_vec),
            }
        }
    }
}
