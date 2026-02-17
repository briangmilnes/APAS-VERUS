//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - persistent, single-threaded.

pub mod MatrixChainStPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    /// Persistent single-threaded matrix chain multiplication solver using dynamic programming
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixChainStPerS {
        dimensions: Vec<MatrixDim>,
        memo: HashMap<(usize, usize), usize>,
    }

    /// Trait for matrix chain multiplication operations
    pub trait MatrixChainStPerTrait {
        /// Create new matrix chain solver
        fn new()                                              -> Self;

        /// Create from matrix dimensions
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// Create from dimension pairs (rows, cols)
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// Compute optimal matrix chain multiplication cost using dynamic programming
        /// claude-4-sonet: Work Θ(n³), Span Θ(n²), Parallelism Θ(1)
        fn optimal_cost(&self)                                -> usize;

        /// Get the matrix dimensions
        fn dimensions(&self)                                  -> &Vec<MatrixDim>;

        /// Get number of matrices
        fn num_matrices(&self)                                -> usize;

        /// Get memoization table size
        fn memo_size(&self)                                   -> usize;
    }

    impl MatrixChainStPerS {
        /// Calculate cost of multiplying matrices from i to j with split at k
        /// Cost = rows[i] * cols[k] * cols[j] (scalar multiplications)
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> usize {
            let left_rows = self.dimensions[i].rows;
            let split_cols = self.dimensions[k].cols;
            let right_cols = self.dimensions[j].cols;
            left_rows * split_cols * right_cols
        }

        /// Internal recursive matrix chain with memoization
        /// Claude Work: O(n³) - O(n²) subproblems, each O(n) work
        /// Claude Span: O(n²) - maximum recursion depth O(n), each level O(n) work
        fn matrix_chain_rec(&mut self, i: usize, j: usize) -> usize {
            // Check memo first
            if let Some(&result) = self.memo.get(&(i, j)) {
                return result;
            }

            let result = if i == j {
                0 // Base case: single matrix, no multiplication needed
            } else {
                // Try each possible split point and find minimum cost
                (i..j)
                    .map(|k| {
                        let left_cost = self.matrix_chain_rec(i, k);
                        let right_cost = self.matrix_chain_rec(k + 1, j);
                        let split_cost = self.multiply_cost(i, k, j);
                        left_cost + right_cost + split_cost
                    })
                    .min()
                    .unwrap_or(0)
            };

            // Memoize result
            self.memo.insert((i, j), result);
            result
        }
    }

    impl MatrixChainStPerTrait for MatrixChainStPerS {
        fn new() -> Self {
            Self {
                dimensions: Vec::new(),
                memo: HashMap::new(),
            }
        }

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions,
                memo: HashMap::new(),
            }
        }

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                })
                .collect();

            Self {
                dimensions,
                memo: HashMap::new(),
            }
        }

        fn optimal_cost(&self) -> usize {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            // Create mutable copy for memoization
            let mut solver = self.clone();
            solver.memo.clear(); // Fresh memo for each query

            let n = solver.dimensions.len();
            solver.matrix_chain_rec(0, n - 1)
        }

        fn dimensions(&self) -> &Vec<MatrixDim> { &self.dimensions }

        fn num_matrices(&self) -> usize { self.dimensions.len() }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    impl Display for MatrixChainStPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "MatrixChainStPer(matrices: {}, memo_entries: {})",
                self.dimensions.len(),
                self.memo.len()
            )
        }
    }

    impl IntoIterator for MatrixChainStPerS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        fn into_iter(self) -> Self::IntoIter { self.dimensions.into_iter() }
    }

    impl<'a> IntoIterator for &'a MatrixChainStPerS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    impl Display for MatrixDim {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }
}

#[macro_export]
macro_rules! MatrixChainStPerLit {
    (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
        $crate::Chap50::MatrixChainStPer::MatrixChainStPer::MatrixChainStPerS::from_dim_pairs(
            vec![$($crate::Types::Types::Pair($r, $c)),*]
        )
    };
    () => {
        $crate::Chap50::MatrixChainStPer::MatrixChainStPer::MatrixChainStPerS::new()
    };
}
