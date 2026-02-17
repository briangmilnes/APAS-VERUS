//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - persistent, multi-threaded.

pub mod SubsetSumMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct SubsetSumMtPerS<T: MtVal> {
        multiset: ArraySeqMtPerS<T>,
        memo: Arc<Mutex<HashMap<(usize, i32), bool>>>,
    }

    /// Trait for parallel subset sum operations
    pub trait SubsetSumMtPerTrait<T: MtVal> {
        /// Create new subset sum solver
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        fn from_multiset(multiset: ArraySeqMtPerS<T>) -> Self;

        /// claude-4-sonet: Work Θ(k×|S|), Span Θ(|S|), Parallelism Θ(k)
        /// Solve subset sum problem with parallel DP where k=target, |S|=multiset size
        fn subset_sum(&self, target: i32)             -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static;

        /// Get the multiset
        fn multiset(&self)                            -> &ArraySeqMtPerS<T>;

        /// Get memoization table size
        fn memo_size(&self)                           -> usize;
    }

    /// Internal parallel recursive subset sum with shared memoization
    /// Claude Work: O(k*|S|) - each subproblem computed once across all threads
    /// Claude Span: O(|S|) - maximum recursion depth, parallelism O(k)
    fn subset_sum_rec<T: MtVal + Into<i32> + Copy + Send + Sync + 'static>(
        table: &SubsetSumMtPerS<T>,
        i: usize,
        j: i32,
    ) -> bool {
        // Check memo first (thread-safe)
        {
            let memo_guard = table.memo.lock().unwrap();
            if let Some(&result) = memo_guard.get(&(i, j)) {
                return result;
            }
        }

        let result = match (i, j) {
            | (_, 0) => true,  // Base case: target sum is 0
            | (0, _) => false, // Base case: no elements left, target > 0
            | (i, j) => {
                let element_value: i32 = (*table.multiset.nth(i - 1)).into();
                if element_value > j {
                    // Element too large, skip it
                    subset_sum_rec(table, i - 1, j)
                } else {
                    // Parallel evaluation of both branches
                    let table_clone1 = table.clone();
                    let table_clone2 = table.clone();

                    let handle1 = thread::spawn(move || subset_sum_rec(&table_clone1, i - 1, j - element_value));

                    let handle2 = thread::spawn(move || subset_sum_rec(&table_clone2, i - 1, j));

                    let result1 = handle1.join().unwrap();
                    let result2 = handle2.join().unwrap();

                    result1 || result2
                }
            }
        };

        // Memoize result (thread-safe)
        {
            let mut memo_guard = table.memo.lock().unwrap();
            memo_guard.insert((i, j), result);
        }

        result
    }

    impl<T: MtVal> SubsetSumMtPerTrait<T> for SubsetSumMtPerS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                multiset: ArraySeqMtPerS::new(0, T::default()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_multiset(multiset: ArraySeqMtPerS<T>) -> Self {
            Self {
                multiset,
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn subset_sum(&self, target: i32) -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static,
        {
            if target < 0 {
                return false;
            }

            // Clear memo for fresh computation
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            let n = self.multiset.length();
            subset_sum_rec(self, n, target)
        }

        fn multiset(&self) -> &ArraySeqMtPerS<T> { &self.multiset }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo.lock().unwrap();
            memo_guard.len()
        }
    }

    impl<T: MtVal> PartialEq for SubsetSumMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.multiset == other.multiset }
    }

    impl<T: MtVal> Eq for SubsetSumMtPerS<T> {}

    impl<T: MtVal> Display for SubsetSumMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
            write!(
                f,
                "SubsetSumMtPer(multiset: {}, memo_entries: {})",
                self.multiset, memo_size
            )
        }
    }

    // Note: IntoIterator not implemented for ArraySeqMtPerS, so we don't provide it here
}

#[macro_export]
macro_rules! SubsetSumMtPerLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumMtPer::SubsetSumMtPer::SubsetSumMtPerS::from_multiset(
            <$crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<_> as $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerBaseTrait<_>>::from_vec(vec![$($x),*])
        )
    };
    () => {
        $crate::Chap49::SubsetSumMtPer::SubsetSumMtPer::SubsetSumMtPerS::new()
    };
}
