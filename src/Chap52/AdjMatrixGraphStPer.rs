//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, single-threaded).
//! G = (bool seq) seq - for enumerable vertex sets V = {0, 1, ..., n-1}.

pub mod AdjMatrixGraphStPer {

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
    // 13. derive impls outside verus!

    // 4. type definitions

    #[derive(Clone, PartialEq, Eq)]
    pub struct AdjMatrixGraphStPer {
        pub matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>,
        pub n: N,
    }

    // 5. view impls

    impl View for AdjMatrixGraphStPer {
        type V = Seq<Seq<bool>>;
        open spec fn view(&self) -> Self::V {
            self.matrix@
        }
    }

    // 8. traits

    pub trait AdjMatrixGraphStPerTrait: Sized {
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn new(n: N)                                                 -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_matrix(matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                       -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn num_edges(&self)                                          -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn has_edge(&self, u: N, v: N)                               -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn out_neighbors(&self, u: N)                                -> ArraySeqStPerS<N>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn out_degree(&self, u: N)                                   -> N;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn set_edge(&self, u: N, v: N, exists: B)                    -> Self;
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn complement(&self)                                         -> Self;
    }

    // 9. impls

    impl AdjMatrixGraphStPerTrait for AdjMatrixGraphStPer {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential creation of n×n false matrix.
        #[verifier::external_body]
        fn new(n: N) -> Self {
            let false_row = ArraySeqStPerS::from_vec(vec![false; n]);
            let mut matrix_rows = Vec::with_capacity(n);
            for _ in 0..n {
                matrix_rows.push(false_row.clone());
            }
            AdjMatrixGraphStPer {
                matrix: ArraySeqStPerS::from_vec(matrix_rows),
                n,
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing matrix.
        #[verifier::external_body]
        fn from_matrix(matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>) -> Self {
            let n = matrix.length();
            AdjMatrixGraphStPer { matrix, n }
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
        fn out_neighbors(&self, u: N) -> ArraySeqStPerS<N> {
            if u >= self.n {
                return ArraySeqStPerS::empty();
            }
            let row = self.matrix.nth(u);
            let mut neighbors = Vec::new();
            for v in 0..self.n {
                if *row.nth(v) {
                    neighbors.push(v);
                }
            }
            ArraySeqStPerS::from_vec(neighbors)
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
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential copy of row + outer seq; span not parallel.
        #[verifier::external_body]
        fn set_edge(&self, u: N, v: N, exists: B) -> Self {
            if u >= self.n || v >= self.n {
                return self.clone();
            }
            let old_row = self.matrix.nth(u);
            let mut new_row_vec = Vec::with_capacity(self.n);
            for j in 0..self.n {
                if j == v {
                    new_row_vec.push(exists);
                } else {
                    new_row_vec.push(*old_row.nth(j));
                }
            }
            let new_row = ArraySeqStPerS::from_vec(new_row_vec);

            let mut new_matrix_vec = Vec::with_capacity(self.n);
            for i in 0..self.n {
                if i == u {
                    new_matrix_vec.push(new_row.clone());
                } else {
                    new_matrix_vec.push(self.matrix.nth(i).clone());
                }
            }
            AdjMatrixGraphStPer {
                matrix: ArraySeqStPerS::from_vec(new_matrix_vec),
                n: self.n,
            }
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
                new_matrix_vec.push(ArraySeqStPerS::from_vec(new_row_vec));
            }
            AdjMatrixGraphStPer {
                matrix: ArraySeqStPerS::from_vec(new_matrix_vec),
                n: self.n,
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for AdjMatrixGraphStPer {
        /// - APAS: N/A — Rust Debug trait, not in textbook.
        /// - Claude-Opus-4.6: Work depends on graph size — outside verus!, not verified.
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjMatrixGraphStPer")
                .field("matrix", &self.matrix)
                .field("n", &self.n)
                .finish()
        }
    }
}
