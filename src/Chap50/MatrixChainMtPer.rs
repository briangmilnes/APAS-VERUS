//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - persistent, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod MatrixChainMtPer {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

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
        impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for McPerMemoWf {
            open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
                v@.dom().finite()
            }
        }
        #[verifier::external_body]
        fn new_mcper_memo_lock(val: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> (lock: RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, McPerMemoWf>)
            requires val@.dom().finite()
        {
            RwLock::new(val, Ghost(McPerMemoWf))
        }

    /// Persistent multi-threaded matrix chain multiplication solver using parallel dynamic programming
    pub struct MatrixChainMtPerS {
        pub dimensions: Arc<Vec<MatrixDim>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, McPerMemoWf>>,
    }

    impl Clone for MatrixChainMtPerS {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            MatrixChainMtPerS {
                dimensions: self.dimensions.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    // 8. traits
    pub trait MatrixChainMtPerTrait: Sized {
        fn new() -> (result: Self);
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (result: Self);
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (result: Self);
        fn optimal_cost(&self) -> (result: usize);
        fn dimensions(&self) -> (result: &Arc<Vec<MatrixDim>>);
        fn num_matrices(&self) -> (result: usize);
        fn memo_size(&self) -> (result: usize);
    }

    // 9. impls

    #[verifier::external_body]
    fn multiply_cost_mt_per(s: &MatrixChainMtPerS, i: usize, k: usize, j: usize) -> (result: usize) {
        let left_rows = s.dimensions[i].rows;
        let split_cols = s.dimensions[k].cols;
        let right_cols = s.dimensions[j].cols;
        left_rows * split_cols * right_cols
    }

    #[verifier::external_body]
    fn parallel_min_reduction_mt_per(s: &MatrixChainMtPerS, costs: Vec<usize>) -> (result: usize) {
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

    #[verifier::external_body]
    fn matrix_chain_rec_mt_per(s: &MatrixChainMtPerS, i: usize, j: usize) -> (result: usize) {
        {
            let handle = s.memo.acquire_read();
            let cached = handle.borrow().get(&Pair(i, j)).copied();
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
            memo.insert(Pair(i, j), result);
            write_handle.release_write(memo);
        }

        result
    }

    impl MatrixChainMtPerTrait for MatrixChainMtPerS {
        #[verifier::external_body]
        fn new() -> (result: Self) {
            Self {
                dimensions: Arc::new(Vec::new()),
                memo: Arc::new(new_mcper_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (result: Self) {
            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(new_mcper_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (result: Self) {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                }).collect::<Vec<MatrixDim>>();

            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(new_mcper_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn optimal_cost(&self) -> (result: usize) {
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

        #[verifier::external_body]
        fn dimensions(&self) -> (result: &Arc<Vec<MatrixDim>>) { &self.dimensions }

        #[verifier::external_body]
        fn num_matrices(&self) -> (result: usize) { self.dimensions.len() }

        #[verifier::external_body]
        fn memo_size(&self) -> (result: usize) {
            let handle = self.memo.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }
    }

    // 11. derive impls in verus!
    impl PartialEq for MatrixChainMtPerS {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> bool { self.dimensions == other.dimensions }
    }

    impl Eq for MatrixChainMtPerS {}

    } // verus!

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
