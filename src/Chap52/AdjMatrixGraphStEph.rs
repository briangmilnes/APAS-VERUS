//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (ephemeral, single-threaded).

pub mod AdjMatrixGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjMatrixGraphStEph {
        pub matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>,
        pub n: N,
    }

    // 5. view impls

    impl View for AdjMatrixGraphStEph {
        type V = Seq<Seq<bool>>;
        open spec fn view(&self) -> Self::V {
            self.matrix@
        }
    }

    // 8. traits

    pub trait AdjMatrixGraphStEphTrait: Sized {
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn new(n: N)                                                 -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_matrix(matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                       -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn num_edges(&self)                                          -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn has_edge(&self, u: N, v: N)                               -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn out_neighbors(&self, u: N)                                -> ArraySeqStEphS<N>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn out_degree(&self, u: N)                                   -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn set_edge(&mut self, u: N, v: N, exists: B);
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn complement(&self)                                         -> Self;
    }

    // 9. impls

    impl AdjMatrixGraphStEphTrait for AdjMatrixGraphStEph {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential creation of n×n false matrix.
        #[verifier::external_body]
        fn new(n: N) -> Self {
            let false_row = ArraySeqStEphS::from_vec(vec![false; n]);
            let mut matrix_rows = Vec::with_capacity(n);
            for _ in 0..n {
                matrix_rows.push(false_row.clone());
            }
            AdjMatrixGraphStEph {
                matrix: ArraySeqStEphS::from_vec(matrix_rows),
                n,
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing matrix.
        #[verifier::external_body]
        fn from_matrix(matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>) -> Self {
            let n = matrix.length();
            AdjMatrixGraphStEph { matrix, n }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — stored field.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.n }

        /// - APAS: Work Θ(n²), Span Θ(1) [Cost Spec 52.6, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential double loop; span not parallel.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            let mut count = 0;
            for i in 0..self.n {
                let row = self.matrix.nth(i);
                for j in 0..self.n {
                    if *row.nth(j) {
                        count += 1;
                    }
                }
            }
            count
        }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential row scan; span not parallel.
        #[verifier::external_body]
        fn out_neighbors(&self, u: N) -> ArraySeqStEphS<N> {
            if u >= self.n {
                return ArraySeqStEphS::empty();
            }
            let row = self.matrix.nth(u);
            let mut neighbors = Vec::new();
            for v in 0..self.n {
                if *row.nth(v) {
                    neighbors.push(v);
                }
            }
            ArraySeqStEphS::from_vec(neighbors)
        }

        /// - APAS: Work Θ(n), Span Θ(lg n) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential row count; span not logarithmic.
        #[verifier::external_body]
        fn out_degree(&self, u: N) -> N {
            if u >= self.n {
                return 0;
            }
            let row = self.matrix.nth(u);
            let mut count = 0;
            for v in 0..self.n {
                if *row.nth(v) {
                    count += 1;
                }
            }
            count
        }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.6, insert/delete edge]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — ephemeral in-place array set; better than APAS persistent bound.
        #[verifier::external_body]
        fn set_edge(&mut self, u: N, v: N, exists: B) {
            if u >= self.n || v >= self.n {
                return;
            }
            let mut row = self.matrix.nth(u).clone();
            let _ = row.set(v, exists);
            let _ = self.matrix.set(u, row);
        }

        /// - APAS: Work Θ(n²), Span Θ(1) [Exercise 52.6]
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential double loop; span not parallel.
        #[verifier::external_body]
        fn complement(&self) -> Self {
            let mut new_matrix_vec = Vec::with_capacity(self.n);
            for i in 0..self.n {
                let row = self.matrix.nth(i);
                let mut new_row_vec = Vec::with_capacity(self.n);
                for j in 0..self.n {
                    if i == j {
                        new_row_vec.push(false);
                    } else {
                        new_row_vec.push(!*row.nth(j));
                    }
                }
                new_matrix_vec.push(ArraySeqStEphS::from_vec(new_row_vec));
            }
            AdjMatrixGraphStEph {
                matrix: ArraySeqStEphS::from_vec(new_matrix_vec),
                n: self.n,
            }
        }
    }

    } // verus!
}
