//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded).
//! PARALLEL operations via ArraySeqMtPer.

pub mod AdjSeqGraphMtPer {

    use std::sync::Arc;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjSeqGraphMtPer {
        adj: ArraySeqMtPerS<ArraySeqMtPerS<N>>,
    }

    pub trait AdjSeqGraphMtPerTrait {
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn new(n: N)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)         -> N;
        /// claude-4-sonet: Work Θ(Σ deg(v)), Span Θ(log n), Parallelism Θ(|E|/log n)
        fn num_edges(&self)            -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(log(deg(u))), Parallelism Θ(deg(u)/log(deg(u)))
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_neighbors(&self, u: N)  -> ArraySeqMtPerS<N>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_degree(&self, u: N)     -> N;
        /// claude-4-sonet: Work Θ(n + |E|), Span Θ(log n), Parallelism Θ((n + |E|)/log n)
        fn map_vertices<F: Fn(N) -> N + Send + Sync + Clone + 'static>(&self, f: F) -> Self
        where
            N: 'static;
    }

    impl AdjSeqGraphMtPerTrait for AdjSeqGraphMtPer {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop creating n empty neighbor lists.
        fn new(n: N) -> Self {
            let empty_list = ArraySeqMtPerS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphMtPer {
                adj: ArraySeqMtPerS::from_vec(adj_lists),
            }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — sequence length.
        fn num_vertices(&self) -> N { self.adj.length() }

        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential loop; span not parallel despite Mt type.
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

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> { self.adj.nth(u).clone() }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length() }

        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential double loop; span not parallel despite Mt type.
        fn map_vertices<F: Fn(N) -> N + Send + Sync + Clone + 'static>(&self, f: F) -> Self
        where
            N: 'static,
        {
            let n = self.adj.length();
            let mut new_adj_vec = Vec::with_capacity(n);
            for i in 0..n {
                let neighbors = self.adj.nth(i);
                let len = neighbors.length();
                let mut new_neighbors_vec = Vec::with_capacity(len);
                for j in 0..len {
                    new_neighbors_vec.push(f(*neighbors.nth(j)));
                }
                new_adj_vec.push(ArraySeqMtPerS::from_vec(new_neighbors_vec));
            }
            AdjSeqGraphMtPer {
                adj: ArraySeqMtPerS::from_vec(new_adj_vec),
            }
        }
    }
}
