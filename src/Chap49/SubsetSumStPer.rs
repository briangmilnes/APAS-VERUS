//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - persistent, single-threaded.

pub mod SubsetSumStPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::ArraySeqStPerSLit;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SubsetSumStPerS<T: StT> {
        multiset: ArraySeqStPerS<T>,
        memo: HashMap<(usize, i32), bool>,
    }

    /// Trait for subset sum operations
    pub trait SubsetSumStPerTrait<T: StT> {
        /// Create new subset sum solver
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        fn from_multiset(multiset: ArraySeqStPerS<T>) -> Self;

        /// Solve subset sum problem
        /// claude-4-sonet: Work Θ(k×|S|), Span Θ(|S|), Parallelism Θ(1)
        fn subset_sum(&self, target: i32)             -> bool
        where
            T: Into<i32> + Copy;

        /// Get the multiset
        fn multiset(&self)                            -> &ArraySeqStPerS<T>;

        /// Get memoization table size
        fn memo_size(&self)                           -> usize;
    }

    /// Internal recursive subset sum with memoization
    /// Claude Work: O(k*|S|) - each subproblem computed once
    /// Claude Span: O(|S|) - maximum recursion depth
    fn subset_sum_rec<T: StT + Into<i32> + Copy>(table: &mut SubsetSumStPerS<T>, i: usize, j: i32) -> bool {
        // Check memo first
        if let Some(&result) = table.memo.get(&(i, j)) {
            return result;
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
                    // Try both including and excluding the element
                    subset_sum_rec(table, i - 1, j - element_value) || subset_sum_rec(table, i - 1, j)
                }
            }
        };

        // Memoize result
        table.memo.insert((i, j), result);
        result
    }

    impl<T: StT> SubsetSumStPerTrait<T> for SubsetSumStPerS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                multiset: ArraySeqStPerS::new(0, T::default()),
                memo: HashMap::new(),
            }
        }

        fn from_multiset(multiset: ArraySeqStPerS<T>) -> Self {
            Self {
                multiset,
                memo: HashMap::new(),
            }
        }

        fn subset_sum(&self, target: i32) -> bool
        where
            T: Into<i32> + Copy,
        {
            if target < 0 {
                return false;
            }

            // Create mutable copy for memoization
            let mut solver = self.clone();
            solver.memo.clear(); // Fresh memo for each query

            let n = solver.multiset.length();
            subset_sum_rec(&mut solver, n, target)
        }

        fn multiset(&self) -> &ArraySeqStPerS<T> { &self.multiset }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    impl<T: StT> Display for SubsetSumStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "SubsetSumStPer(multiset: {}, memo_entries: {})",
                self.multiset,
                self.memo.len()
            )
        }
    }

    impl<T: StT> IntoIterator for SubsetSumStPerS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStPerS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.into_iter() }
    }

    impl<T: StT> IntoIterator for &SubsetSumStPerS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStPerS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.clone().into_iter() }
    }
}

#[macro_export]
macro_rules! SubsetSumStPerLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumStPer::SubsetSumStPer::SubsetSumStPerS::from_multiset(
            $crate::ArraySeqStPerSLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumStPer::SubsetSumStPer::SubsetSumStPerS::new()
    };
}
