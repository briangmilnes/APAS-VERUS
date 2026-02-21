//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent matrix chain multiplication implementation using Vec and Arc for thread safety.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<RwLock<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod MatrixChainMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Types::Types::*;

    verus! {
    // 4. type definitions
    #[verifier::reject_recursive_types]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    impl View for MatrixDim {
        type V = (nat, nat);
        open spec fn view(&self) -> (nat, nat) {
            (self.rows as nat, self.cols as nat)
        }
    }

        pub struct McPerMemoWf;
        impl RwLockPredicate<HashMap<(usize, usize), usize>> for McPerMemoWf {
            open spec fn inv(self, v: HashMap<(usize, usize), usize>) -> bool { true }
        }
        #[verifier::external_body]
        fn new_mcper_memo_lock(val: HashMap<(usize, usize), usize>) -> (lock: RwLock<HashMap<(usize, usize), usize>, McPerMemoWf>) {
            RwLock::new(val, Ghost(McPerMemoWf))
        }
    }

    /// Persistent multi-threaded matrix chain multiplication solver using parallel dynamic programming
    #[derive(Clone)]
    pub struct MatrixChainMtPerS {
        pub dimensions: Arc<Vec<MatrixDim>>,
        pub memo: Arc<RwLock<HashMap<(usize, usize), usize>, McPerMemoWf>>,
    }

    // 8. traits
    /// Trait for parallel matrix chain multiplication operations
    pub trait MatrixChainMtPerTrait: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc wrappers
        fn new()                                              -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n pairs then wrap in Arc
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — memoized DP with parallel min reduction
        fn optimal_cost(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Arc reference access
        fn dimensions(&self)                                  -> &Arc<Vec<MatrixDim>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len through Arc
        fn num_matrices(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len under read lock
        fn memo_size(&self)                                   -> usize;
    }

    // 9. impls

    fn multiply_cost_mt_per(s: &MatrixChainMtPerS, i: usize, k: usize, j: usize) -> usize {
        let left_rows = s.dimensions[i].rows;
        let split_cols = s.dimensions[k].cols;
        let right_cols = s.dimensions[j].cols;
        left_rows * split_cols * right_cols
    }

    fn parallel_min_reduction_mt_per(s: &MatrixChainMtPerS, costs: Vec<usize>) -> usize {
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

        let handle1 = thread::spawn(move || parallel_min_reduction_mt_per(&s1, left_costs));
        let handle2 = thread::spawn(move || parallel_min_reduction_mt_per(&s2, right_costs));

        let left_min = handle1.join().unwrap();
        let right_min = handle2.join().unwrap();

        left_min.min(right_min)
    }

    fn matrix_chain_rec_mt_per(s: &MatrixChainMtPerS, i: usize, j: usize) -> usize {
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
                    let left_cost = matrix_chain_rec_mt_per(s, i, k);
                    let right_cost = matrix_chain_rec_mt_per(s, k + 1, j);
                    let split_cost = multiply_cost_mt_per(s, i, k, j);
                    left_cost + right_cost + split_cost
                })
                .collect::<Vec<usize>>();

            parallel_min_reduction_mt_per(s, costs)
        };

        {
            let (mut memo, write_handle) = s.memo.acquire_write();
            memo.insert((i, j), result);
            write_handle.release_write(memo);
        }

        result
    }

    impl MatrixChainMtPerTrait for MatrixChainMtPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate Arc wrappers
        fn new() -> Self {
            Self {
                dimensions: Arc::new(Vec::new()),
                memo: Arc::new(new_mcper_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wrap Vec in Arc
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(new_mcper_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n Pair values then wrap in Arc
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                }).collect::<Vec<MatrixDim>>();

            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(new_mcper_memo_lock(HashMap::new())),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n² lg n)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n² lg n) — clears memo, invokes matrix_chain_rec
        fn optimal_cost(&self) -> usize {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            let n = self.dimensions.len();
            matrix_chain_rec_mt_per(self, 0, n - 1)
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Arc reference access
        fn dimensions(&self) -> &Arc<Vec<MatrixDim>> { &self.dimensions }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len through Arc
        fn num_matrices(&self) -> usize { self.dimensions.len() }

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
    impl PartialEq for MatrixChainMtPerS {
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — compare Arc<Vec> contents
        fn eq(&self, other: &Self) -> bool { self.dimensions == other.dimensions }
    }

    impl Eq for MatrixChainMtPerS {}

    // 13. derive impls outside verus!
    impl Debug for MatrixChainMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl Display for MatrixChainMtPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — unwrap or clone Vec from Arc
        fn into_iter(self) -> Self::IntoIter {
            match Arc::try_unwrap(self.dimensions) {
                | Ok(vec) => vec.into_iter(),
                | Err(arc) => (*arc).clone().into_iter(),
            }
        }
    }

    impl<'a> IntoIterator for &'a MatrixChainMtPerS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter over Arc<Vec>
        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    impl Display for MatrixDim {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    // 12. macros
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
