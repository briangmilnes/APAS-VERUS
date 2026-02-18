//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (persistent, single-threaded).
//! G = (int seq) seq - for enumerable vertex sets V = {0, 1, ..., n-1}.

pub mod AdjSeqGraphStPer {

    use std::fmt::{Debug, Formatter};

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone, PartialEq, Eq)]
    pub struct AdjSeqGraphStPer {
        pub adj: ArraySeqStPerS<ArraySeqStPerS<N>>,
    }

    // 5. view impls

    impl View for AdjSeqGraphStPer {
        type V = Seq<Seq<int>>;
        open spec fn view(&self) -> Self::V {
            Seq::new(self.adj.spec_len(), |i: int|
                Seq::new(self.adj.spec_index(i).spec_len(), |j: int|
                    self.adj.spec_index(i).spec_index(j) as int
                )
            )
        }
    }

    // 8. traits

    pub trait AdjSeqGraphStPerTrait: Sized {
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

    // 9. impls

    impl AdjSeqGraphStPerTrait for AdjSeqGraphStPer {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop creating n empty neighbor lists.
        #[verifier::external_body]
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

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing sequence.
        #[verifier::external_body]
        fn from_seq(adj: ArraySeqStPerS<ArraySeqStPerS<N>>) -> Self { AdjSeqGraphStPer { adj } }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — sequence length.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.adj.length() }

        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential loop; span not parallel.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            let n = self.adj.length();
            let mut count = 0;
            for i in 0..n {
                count += self.adj.nth(i).length();
            }
            count
        }

        /// - APAS: Work Θ(d(u)), Span Θ(lg d(u)) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(d(u)), Span Θ(d(u)) — sequential linear scan; span not logarithmic.
        #[verifier::external_body]
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

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5, out-neighbors]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_neighbors(&self, u: N) -> &ArraySeqStPerS<N> { self.adj.nth(u) }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length() }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.5, insert edge]
        /// - Claude-Opus-4.6: Work Θ(n + d(u)), Span Θ(n + d(u)) — sequential copy of outer seq + linear scan for duplicates; span not parallel.
        #[verifier::external_body]
        fn insert_edge(&self, u: N, v: N) -> Self {
            if u >= self.adj.length() || v >= self.adj.length() {
                return self.clone();
            }
            let old_neighbors = self.adj.nth(u);
            for i in 0..old_neighbors.length() {
                if *old_neighbors.nth(i) == v {
                    return self.clone();
                }
            }
            let mut new_neighbors_vec = Vec::<N>::with_capacity(old_neighbors.length() + 1);
            for i in 0..old_neighbors.length() {
                new_neighbors_vec.push(*old_neighbors.nth(i));
            }
            new_neighbors_vec.push(v);
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

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.5, delete edge]
        /// - Claude-Opus-4.6: Work Θ(n + d(u)), Span Θ(n + d(u)) — sequential copy of outer seq + linear scan; span not parallel.
        #[verifier::external_body]
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

    } // verus!

    // 13. derive impls outside verus!

    impl Debug for AdjSeqGraphStPer {
        /// - APAS: N/A — Rust Debug trait, not in textbook.
        /// - Claude-Opus-4.6: Work depends on graph size — outside verus!, not verified.
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjSeqGraphStPer").field("adj", &self.adj).finish()
        }
    }
}
