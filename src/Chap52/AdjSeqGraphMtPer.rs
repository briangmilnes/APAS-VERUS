//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded).
//! PARALLEL operations via ArraySeqMtPer.

pub mod AdjSeqGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjSeqGraphMtPer {
        pub adj: ArraySeqMtPerS<ArraySeqMtPerS<N>>,
    }

    // 5. view impls

    impl View for AdjSeqGraphMtPer {
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

    pub trait AdjSeqGraphMtPerTrait: Sized {
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
    }

    // 9. impls

    impl AdjSeqGraphMtPerTrait for AdjSeqGraphMtPer {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop creating n empty neighbor lists.
        #[verifier::external_body]
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
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.adj.length() }

        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential loop; span not parallel despite Mt type.
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

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> { self.adj.nth(u).clone() }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length() }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl AdjSeqGraphMtPer {
        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential double loop; span not parallel despite Mt type.
        pub fn map_vertices<F: Fn(N) -> N + Send + Sync + Clone + 'static>(&self, f: F) -> Self
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
