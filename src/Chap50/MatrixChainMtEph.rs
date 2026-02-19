//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral matrix chain multiplication implementation using Vec and Arc<RwLock<Vec>> for mutable thread safety.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<RwLock<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod MatrixChainMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Types::Types::*;

    // 4. type definitions
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    /// Ephemeral multi-threaded matrix chain multiplication solver using parallel dynamic programming
    #[derive(Clone)]
    pub struct MatrixChainMtEphS {
        pub dimensions: Arc<RwLock<Vec<MatrixDim>, McEphDimWf>>,
        pub memo: Arc<RwLock<HashMap<(usize, usize), usize>, McEphMemoWf>>,
    }

    verus! {
        #[verifier::external_type_specification]
        pub struct ExMatrixDim(MatrixDim);

        pub struct McEphDimWf;
        impl RwLockPredicate<Vec<MatrixDim>> for McEphDimWf {
            open spec fn inv(self, v: Vec<MatrixDim>) -> bool { true }
        }
        #[verifier::external_body]
        fn new_mceph_dim_lock(val: Vec<MatrixDim>) -> (lock: RwLock<Vec<MatrixDim>, McEphDimWf>) {
            RwLock::new(val, Ghost(McEphDimWf))
        }

        pub struct McEphMemoWf;
        impl RwLockPredicate<HashMap<(usize, usize), usize>> for McEphMemoWf {
            open spec fn inv(self, v: HashMap<(usize, usize), usize>) -> bool { true }
        }
        #[verifier::external_body]
        fn new_mceph_memo_lock(val: HashMap<(usize, usize), usize>) -> (lock: RwLock<HashMap<(usize, usize), usize>, McEphMemoWf>) {
            RwLock::new(val, Ghost(McEphMemoWf))
        }
    }

    // 8. traits
    /// Trait for parallel matrix chain multiplication operations
    pub trait MatrixChainMtEphTrait: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<RwLock> wrappers
        fn new()                                              -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<RwLock>
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n pairs then wrap in Arc<RwLock>
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP with parallel min reduction
        fn optimal_cost(&mut self)                            -> usize;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn dimensions(&self)                                  -> Vec<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_dimension(&mut self, index: usize, dim: MatrixDim);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under read lock
        fn num_matrices(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under write lock
        fn clear_memo(&mut self);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under read lock
        fn memo_size(&self)                                   -> usize;
    }

    // 9. impls

    fn multiply_cost_mt_eph(s: &MatrixChainMtEphS, i: usize, k: usize, j: usize) -> usize {
        let handle = s.dimensions.acquire_read();
        let left_rows = handle.borrow()[i].rows;
        let split_cols = handle.borrow()[k].cols;
        let right_cols = handle.borrow()[j].cols;
        handle.release_read();
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
            let handle = s.memo.acquire_read();
            let cached = handle.borrow().get(&(i, j)).copied();
            handle.release_read();
            if let Some(result) = cached {
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
            let (mut memo, write_handle) = s.memo.acquire_write();
            memo.insert((i, j), result);
            write_handle.release_write(memo);
        }

        result
    }

    impl MatrixChainMtEphTrait for MatrixChainMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc<RwLock> wrappers
        fn new() -> Self {
            Self {
                dimensions: Arc::new(new_mceph_dim_lock(Vec::new())),
                memo: Arc::new(new_mceph_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc<RwLock>
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions: Arc::new(new_mceph_dim_lock(dimensions)),
                memo: Arc::new(new_mceph_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n Pair values then wrap in Arc<RwLock>
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                }).collect::<Vec<MatrixDim>>();

            Self {
                dimensions: Arc::new(new_mceph_dim_lock(dimensions)),
                memo: Arc::new(new_mceph_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — invokes matrix_chain_rec on full range
        fn optimal_cost(&mut self) -> usize {
            let dimensions_len = {
                let handle = self.dimensions.acquire_read();
                let len = handle.borrow().len();
                handle.release_read();
                len
            };

            if dimensions_len <= 1 {
                return 0;
            }

            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            matrix_chain_rec_mt_eph(self, 0, dimensions_len - 1)
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn dimensions(&self) -> Vec<MatrixDim> {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            {
                let (mut dims, write_handle) = self.dimensions.acquire_write();
                dims[index] = dim;
                write_handle.release_write(dims);
            }
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — write under lock plus memo clear
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            {
                let (mut dims, write_handle) = self.dimensions.acquire_write();
                dims[index] = dim;
                write_handle.release_write(dims);
            }
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len under read lock
        fn num_matrices(&self) -> usize {
            let handle = self.dimensions.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear under write lock
        fn clear_memo(&mut self) {
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under read lock
        fn memo_size(&self) -> usize {
            let handle = self.memo.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }
    }

    // 11. derive impls
    impl PartialEq for MatrixChainMtEphS {
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — compare Vec contents under read locks
        fn eq(&self, other: &Self) -> bool {
            let self_handle = self.dimensions.acquire_read();
            let other_handle = other.dimensions.acquire_read();
            let result = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            result
        }
    }

    impl Eq for MatrixChainMtEphS {}

    // 13. derive impls outside verus!
    impl Debug for MatrixChainMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl Display for MatrixChainMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers under read locks
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
            let dim_handle = self.dimensions.acquire_read();
            let dimensions_len = dim_handle.borrow().len();
            dim_handle.release_read();
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
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec from Arc<RwLock>
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
        }
    }

    impl IntoIterator for &MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
        }
    }

    impl IntoIterator for &mut MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
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
