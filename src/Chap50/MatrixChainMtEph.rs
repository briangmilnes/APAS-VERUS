//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral matrix chain multiplication implementation using Vec and Arc<Mutex<Vec>> for mutable thread safety.

pub mod MatrixChainMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    /// Ephemeral multi-threaded matrix chain multiplication solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct MatrixChainMtEphS {
        dimensions: Arc<Mutex<Vec<MatrixDim>>>,
        memo: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    /// Trait for parallel matrix chain multiplication operations
    pub trait MatrixChainMtEphTrait {
        /// Create new matrix chain solver
        fn new()                                              -> Self;

        /// Create from matrix dimensions
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// Create from dimension pairs (rows, cols)
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// APAS: Work Θ(n³), Span Θ(n log n)
        /// Claude-Opus-4.6: Work O(n³), Span O(n log n)
        fn optimal_cost(&mut self)                            -> usize;

        /// Get a copy of the matrix dimensions (thread-safe)
        fn dimensions(&self)                                  -> Vec<MatrixDim>;

        /// Set matrix dimension at index
        fn set_dimension(&mut self, index: usize, dim: MatrixDim);

        /// Update matrix dimensions
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize);

        /// Get number of matrices
        fn num_matrices(&self)                                -> usize;

        /// Clear memoization table
        fn clear_memo(&mut self);

        /// Get memoization table size
        fn memo_size(&self)                                   -> usize;
    }

    impl MatrixChainMtEphS {
        /// Calculate cost of multiplying matrices from i to j with split at k
        /// Cost = rows[i] * cols[k] * cols[j] (scalar multiplications)
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> usize {
            let dimensions_guard = self.dimensions.lock().unwrap();
            let left_rows = dimensions_guard[i].rows;
            let split_cols = dimensions_guard[k].cols;
            let right_cols = dimensions_guard[j].cols;
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

    impl MatrixChainMtEphTrait for MatrixChainMtEphS {
        fn new() -> Self {
            Self {
                dimensions: Arc::new(Mutex::new(Vec::new())),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions: Arc::new(Mutex::new(dimensions)),
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
                dimensions: Arc::new(Mutex::new(dimensions)),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn optimal_cost(&mut self) -> usize {
            let dimensions_len = {
                let dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard.len()
            };

            if dimensions_len <= 1 {
                return 0;
            }

            // Clear memo for fresh computation
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            self.matrix_chain_rec(0, dimensions_len - 1)
        }

        fn dimensions(&self) -> Vec<MatrixDim> {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.clone()
        }

        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            {
                let mut dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard[index] = dim;
            }
            // Clear memo since dimensions changed
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            {
                let mut dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard[index] = dim;
            }
            // Clear memo since dimensions changed
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn num_matrices(&self) -> usize {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.len()
        }

        fn clear_memo(&mut self) {
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    impl PartialEq for MatrixChainMtEphS {
        fn eq(&self, other: &Self) -> bool {
            // Compare the contents of the Arc<Mutex<Vec>>
            let self_dims = self.dimensions.lock().unwrap();
            let other_dims = other.dimensions.lock().unwrap();
            *self_dims == *other_dims
        }
    }

    impl Eq for MatrixChainMtEphS {}

    impl Display for MatrixChainMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
            let dimensions_len = {
                let dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard.len()
            };
            write!(
                f,
                "MatrixChainMtEph(matrices: {dimensions_len}, memo_entries: {memo_size})"
            )
        }
    }

    impl IntoIterator for MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        fn into_iter(self) -> Self::IntoIter {
            // Extract Vec from Arc<Mutex<Vec>> - this consumes the Arc
            match Arc::try_unwrap(self.dimensions) {
                | Ok(mutex) => mutex.into_inner().unwrap().into_iter(),
                | Err(arc) => {
                    let dimensions_guard = arc.lock().unwrap();
                    dimensions_guard.clone().into_iter()
                }
            }
        }
    }

    impl IntoIterator for &MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        fn into_iter(self) -> Self::IntoIter {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.clone().into_iter()
        }
    }

    impl IntoIterator for &mut MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        fn into_iter(self) -> Self::IntoIter {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.clone().into_iter()
        }
    }

    impl Display for MatrixDim {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    #[macro_export]
    macro_rules! MatrixChainMtEphLit {
        (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
            $crate::Chap50::MatrixChainMtEph::MatrixChainMtEph::MatrixChainMtEphS::from_dim_pairs(
                vec![$($crate::Types::Types::Pair($r, $c)),*]
            )
        };
        () => {
            $crate::Chap50::MatrixChainMtEph::MatrixChainMtEph::MatrixChainMtEphS::new()
        };
    }
}
