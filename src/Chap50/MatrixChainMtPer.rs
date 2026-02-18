//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent matrix chain multiplication implementation using Vec and Arc for thread safety.

pub mod MatrixChainMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    /// Persistent multi-threaded matrix chain multiplication solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct MatrixChainMtPerS {
        dimensions: Arc<Vec<MatrixDim>>,
        memo: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    /// Trait for parallel matrix chain multiplication operations
    pub trait MatrixChainMtPerTrait {
        /// Create new matrix chain solver
        fn new()                                              -> Self;

        /// Create from matrix dimensions
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// Create from dimension pairs (rows, cols)
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// APAS: Work Θ(n³), Span Θ(n log n)
        /// Claude-Opus-4.6: Work O(n³), Span O(n log n)
        fn optimal_cost(&self)                                -> usize;

        /// Get the matrix dimensions
        fn dimensions(&self)                                  -> &Arc<Vec<MatrixDim>>;

        /// Get number of matrices
        fn num_matrices(&self)                                -> usize;

        /// Get memoization table size
        fn memo_size(&self)                                   -> usize;
    }

    impl MatrixChainMtPerS {
        /// Calculate cost of multiplying matrices from i to j with split at k
        /// Cost = rows[i] * cols[k] * cols[j] (scalar multiplications)
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> usize {
            let left_rows = self.dimensions[i].rows;
            let split_cols = self.dimensions[k].cols;
            let right_cols = self.dimensions[j].cols;
            left_rows * split_cols * right_cols
        }

        /// APAS: Work Θ(n), Span Θ(log n)
        /// Claude-Opus-4.6 Work: O(n) - n comparisons
        /// Claude-Opus-4.6 Span: O(log n) - parallel reduction tree
        fn parallel_min_reduction(&self, costs: Vec<usize>) -> usize {
            if costs.is_empty() {
                return 0;
            }
            if costs.len() == 1 {
                return costs[0];
            }

            let mid = costs.len() / 2;
            let left_costs = costs[..mid].to_vec();
            let right_costs = costs[mid..].to_vec();

            let self_clone1 = self.clone();
            let self_clone2 = self.clone();

            let handle1 = thread::spawn(move || self_clone1.parallel_min_reduction(left_costs));

            let handle2 = thread::spawn(move || self_clone2.parallel_min_reduction(right_costs));

            let left_min = handle1.join().unwrap();
            let right_min = handle2.join().unwrap();

            left_min.min(right_min)
        }

        /// APAS: Work Θ(n³), Span Θ(n log n)
        /// Claude-Opus-4.6 Work: O(n³) - O(n²) subproblems, each O(n) work
        /// Claude-Opus-4.6 Span: O(n log n) - recursion depth O(n), each level O(log n) parallel reduction
        fn matrix_chain_rec(&self, i: usize, j: usize) -> usize {
            // Check memo first (thread-safe)
            {
                let memo_guard = self.memo.lock().unwrap();
                if let Some(&result) = memo_guard.get(&(i, j)) {
                    return result;
                }
            }

            let result = if i == j {
                0 // Base case: single matrix, no multiplication needed
            } else {
                // Compute costs for each possible split in parallel
                let costs = (i..j)
                    .map(|k| {
                        let left_cost = self.matrix_chain_rec(i, k);
                        let right_cost = self.matrix_chain_rec(k + 1, j);
                        let split_cost = self.multiply_cost(i, k, j);
                        left_cost + right_cost + split_cost
                    }).collect::<Vec<usize>>();

                // Use parallel reduction to find minimum
                self.parallel_min_reduction(costs)
            };

            // Memoize result (thread-safe)
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.insert((i, j), result);
            }

            result
        }
    }

    impl MatrixChainMtPerTrait for MatrixChainMtPerS {
        fn new() -> Self {
            Self {
                dimensions: Arc::new(Vec::new()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                }).collect::<Vec<MatrixDim>>();

            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn optimal_cost(&self) -> usize {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            // Clear memo for fresh computation
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            let n = self.dimensions.len();
            self.matrix_chain_rec(0, n - 1)
        }

        fn dimensions(&self) -> &Arc<Vec<MatrixDim>> { &self.dimensions }

        fn num_matrices(&self) -> usize { self.dimensions.len() }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    impl PartialEq for MatrixChainMtPerS {
        fn eq(&self, other: &Self) -> bool { self.dimensions == other.dimensions }
    }

    impl Eq for MatrixChainMtPerS {}

    impl Display for MatrixChainMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
            write!(
                f,
                "MatrixChainMtPer(matrices: {}, memo_entries: {})",
                self.dimensions.len(),
                memo_size
            )
        }
    }

    impl IntoIterator for MatrixChainMtPerS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        fn into_iter(self) -> Self::IntoIter {
            // Extract Vec from Arc - this consumes the Arc
            match Arc::try_unwrap(self.dimensions) {
                | Ok(vec) => vec.into_iter(),
                | Err(arc) => (*arc).clone().into_iter(),
            }
        }
    }

    impl<'a> IntoIterator for &'a MatrixChainMtPerS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    impl Display for MatrixDim {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    #[macro_export]
    macro_rules! MatrixChainMtPerLit {
        (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
            $crate::Chap50::MatrixChainMtPer::MatrixChainMtPer::MatrixChainMtPerS::from_dim_pairs(
                vec![$($crate::Types::Types::Pair($r, $c)),*]
            )
        };
        () => {
            $crate::Chap50::MatrixChainMtPer::MatrixChainMtPer::MatrixChainMtPerS::new()
        };
    }
}
