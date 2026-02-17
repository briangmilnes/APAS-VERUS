//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, single-threaded).
//! G = (bool seq) seq - for enumerable vertex sets V = {0, 1, ..., n-1}.

pub mod AdjMatrixGraphStPer {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AdjMatrixGraphStPer {
        matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>,
        n: N,
    }

    pub trait AdjMatrixGraphStPerTrait {
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

    impl AdjMatrixGraphStPerTrait for AdjMatrixGraphStPer {
        // Work: Θ(n²), Span: Θ(n²) - create n×n matrix of false
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

        fn from_matrix(matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>) -> Self {
            let n = matrix.length();
            AdjMatrixGraphStPer { matrix, n }
        }

        // Work: Θ(1), Span: Θ(1)
        fn num_vertices(&self) -> N { self.n }

        // Work: Θ(n²), Span: Θ(n²) - count all true entries
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

        // Work: Θ(1), Span: Θ(1) - direct array access
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        // Work: Θ(n), Span: Θ(n) - scan row for true entries
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

        // Work: Θ(n), Span: Θ(n)
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

        // Work: Θ(n), Span: Θ(n) - copy row and update
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

        // Work: Θ(n²), Span: Θ(1) with parallel - Exercise 52.6
        fn complement(&self) -> Self {
            let mut new_matrix_vec = Vec::with_capacity(self.n);
            for i in 0..self.n {
                let row = self.matrix.nth(i);
                let mut new_row_vec = Vec::with_capacity(self.n);
                for j in 0..self.n {
                    // Complement: flip all entries except diagonal
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
}
