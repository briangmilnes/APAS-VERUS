//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral matrix chain multiplication implementation using Vec and Arc<Mutex<Vec>> for mutable thread safety.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<Mutex<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod MatrixChainMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Types::Types::*;

    // 4. type definitions
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    // Struct contains Arc<Mutex<HashMap>> for memoization — cannot be inside verus!.
    /// Ephemeral multi-threaded matrix chain multiplication solver using parallel dynamic programming
    #[derive(Clone, Debug)]
    pub struct MatrixChainMtEphS {
        pub dimensions: Arc<Mutex<Vec<MatrixDim>>>,
        pub memo: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    // 8. traits
    /// Trait for parallel matrix chain multiplication operations
    pub trait MatrixChainMtEphTrait: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<Mutex> wrappers
        fn new()                                              -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<Mutex>
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n pairs then wrap in Arc<Mutex>
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP with parallel min reduction
        fn optimal_cost(&mut self)                            -> usize;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn dimensions(&self)                                  -> Vec<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_dimension(&mut self, index: usize, dim: MatrixDim);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under lock
        fn num_matrices(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under lock
        fn clear_memo(&mut self);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under lock
        fn memo_size(&self)                                   -> usize;
    }

    // 9. impls

    fn multiply_cost_mt_eph(s: &MatrixChainMtEphS, i: usize, k: usize, j: usize) -> usize {
        let dimensions_guard = s.dimensions.lock().unwrap();
        let left_rows = dimensions_guard[i].rows;
        let split_cols = dimensions_guard[k].cols;
        let right_cols = dimensions_guard[j].cols;
        left_rows * split_cols * right_cols
    }

    fn parallel_min_reduction(s: &MatrixChainMtEphS, costs: Vec<usize>) -> usize {
        if costs.is_empty() {
            return usize::MAX;
        }
        if costs.len() == 1 {
            return costs[0];
        }

        let mid = costs.len() / 2;
        let left_costs = costs[..mid].to_vec();
        let right_costs = costs[mid..].to_vec();

        let s1 = s.clone();
        let s2 = s.clone();

        let handle1 = thread::spawn(move || parallel_min_reduction(&s1, left_costs));
        let handle2 = thread::spawn(move || parallel_min_reduction(&s2, right_costs));

        let left_min = handle1.join().unwrap();
        let right_min = handle2.join().unwrap();

        left_min.min(right_min)
    }

    fn matrix_chain_rec_mt_eph(s: &MatrixChainMtEphS, i: usize, j: usize) -> usize {
        {
            let memo_guard = s.memo.lock().unwrap();
            if let Some(&result) = memo_guard.get(&(i, j)) {
                return result;
            }
        }

        let result = if i == j {
            0
        } else {
            let costs = (i..j)
                .map(|k| {
                    let left_cost = matrix_chain_rec_mt_eph(s, i, k);
                    let right_cost = matrix_chain_rec_mt_eph(s, k + 1, j);
                    let split_cost = multiply_cost_mt_eph(s, i, k, j);
                    left_cost + right_cost + split_cost
                })
                .collect::<Vec<usize>>();

            parallel_min_reduction(s, costs)
        };

        {
            let mut memo_guard = s.memo.lock().unwrap();
            memo_guard.insert((i, j), result);
        }

        result
    }

    impl MatrixChainMtEphTrait for MatrixChainMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<Mutex> wrappers
        fn new() -> Self {
            Self {
                dimensions: Arc::new(Mutex::new(Vec::new())),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<Mutex>
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions: Arc::new(Mutex::new(dimensions)),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n Pair values then wrap in Arc<Mutex>
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

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — invokes matrix_chain_rec on full range
        fn optimal_cost(&mut self) -> usize {
            let dimensions_len = {
                let dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard.len()
            };

            if dimensions_len <= 1 {
                return 0;
            }

            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            matrix_chain_rec_mt_eph(self, 0, dimensions_len - 1)
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn dimensions(&self) -> Vec<MatrixDim> {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.clone()
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            {
                let mut dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard[index] = dim;
            }
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            {
                let mut dimensions_guard = self.dimensions.lock().unwrap();
                dimensions_guard[index] = dim;
            }
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under lock
        fn num_matrices(&self) -> usize {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.len()
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under lock
        fn clear_memo(&mut self) {
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under lock
        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    // 11. derive impls
    impl PartialEq for MatrixChainMtEphS {
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — compare Vec contents under lock
        fn eq(&self, other: &Self) -> bool {
            let self_dims = self.dimensions.lock().unwrap();
            let other_dims = other.dimensions.lock().unwrap();
            *self_dims == *other_dims
        }
    }

    impl Eq for MatrixChainMtEphS {}

    // 13. derive impls outside verus!
    impl Display for MatrixChainMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers under locks
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — unwrap or clone Vec from Arc<Mutex>
        fn into_iter(self) -> Self::IntoIter {
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn into_iter(self) -> Self::IntoIter {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.clone().into_iter()
        }
    }

    impl IntoIterator for &mut MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under lock
        fn into_iter(self) -> Self::IntoIter {
            let dimensions_guard = self.dimensions.lock().unwrap();
            dimensions_guard.clone().into_iter()
        }
    }

    impl Display for MatrixDim {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    // 12. macros
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
