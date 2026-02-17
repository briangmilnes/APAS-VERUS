//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - ephemeral, multi-threaded.

pub mod SubsetSumMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::ArraySeqMtEphSLit;
    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct SubsetSumMtEphS<T: MtVal> {
        multiset: ArraySeqMtEphS<T>,
        memo: Arc<Mutex<HashMap<(usize, i32), bool>>>,
    }

    /// Trait for parallel subset sum operations
    pub trait SubsetSumMtEphTrait<T: MtVal> {
        /// Create new subset sum solver
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> Self;

        /// claude-4-sonet: Work Θ(k×|S|), Span Θ(|S|), Parallelism Θ(k)
        /// Solve subset sum problem with parallel DP where k=target, |S|=multiset size
        fn subset_sum(&mut self, target: i32)         -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static;

        /// Get the multiset
        fn multiset(&self)                            -> &ArraySeqMtEphS<T>;

        /// Get mutable multiset (ephemeral allows mutation)
        fn multiset_mut(&mut self)                    -> &mut ArraySeqMtEphS<T>;

        /// Set element at index (ephemeral mutation)
        fn set(&mut self, index: usize, value: T);

        /// Clear memoization table
        fn clear_memo(&mut self);

        /// Get memoization table size
        fn memo_size(&self)                           -> usize;
    }

    impl<T: MtVal> SubsetSumMtEphS<T> {
        /// Internal parallel recursive subset sum with shared memoization
        /// Claude Work: O(k*|S|) - each subproblem computed once across all threads
        /// Claude Span: O(|S|) - maximum recursion depth, parallelism O(k)
        fn subset_sum_rec(&self, i: usize, j: i32) -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static,
        {
            // Check memo first (thread-safe)
            {
                let memo_guard = self.memo.lock().unwrap();
                if let Some(&result) = memo_guard.get(&(i, j)) {
                    return result;
                }
            }

            let result = match (i, j) {
                | (_, 0) => true,  // Base case: target sum is 0
                | (0, _) => false, // Base case: no elements left, target > 0
                | (i, j) => {
                    let element_value: i32 = self.multiset.nth_cloned(i - 1).into();
                    if element_value > j {
                        // Element too large, skip it
                        self.subset_sum_rec(i - 1, j)
                    } else {
                        // Parallel evaluation of both branches
                        let self_clone1 = self.clone();
                        let self_clone2 = self.clone();

                        let handle1 = thread::spawn(move || self_clone1.subset_sum_rec(i - 1, j - element_value));

                        let handle2 = thread::spawn(move || self_clone2.subset_sum_rec(i - 1, j));

                        let result1 = handle1.join().unwrap();
                        let result2 = handle2.join().unwrap();

                        result1 || result2
                    }
                }
            };

            // Memoize result (thread-safe)
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.insert((i, j), result);
            }

            result
        }
    }

    impl<T: MtVal> SubsetSumMtEphTrait<T> for SubsetSumMtEphS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                multiset: ArraySeqMtEphS::new(0, T::default()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> Self {
            Self {
                multiset,
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn subset_sum(&mut self, target: i32) -> bool
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
            self.subset_sum_rec(n, target)
        }

        fn multiset(&self) -> &ArraySeqMtEphS<T> { &self.multiset }

        fn multiset_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.multiset }

        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            // Clear memo since multiset changed
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
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

    impl<T: MtVal> PartialEq for SubsetSumMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.multiset == other.multiset }
    }

    impl<T: MtVal> Eq for SubsetSumMtEphS<T> {}

    impl<T: MtVal> Display for SubsetSumMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_size = {
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
            };
            write!(
                f,
                "SubsetSumMtEph(multiset: {}, memo_entries: {})",
                self.multiset, memo_size
            )
        }
    }

    // Note: IntoIterator not implemented for ArraySeqMtEphS, so we don't provide it here
}

#[macro_export]
macro_rules! SubsetSumMtEphLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumMtEph::SubsetSumMtEph::SubsetSumMtEphS::from_multiset(
            $crate::ArraySeqMtEphSLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumMtEph::SubsetSumMtEph::SubsetSumMtEphS::new()
    };
}
