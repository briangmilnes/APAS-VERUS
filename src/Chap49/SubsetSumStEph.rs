//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - ephemeral, single-threaded.

pub mod SubsetSumStEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::ArraySeqStEphSLit;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SubsetSumStEphS<T: StT> {
        multiset: ArraySeqStEphS<T>,
        memo: HashMap<(usize, i32), bool>,
    }

    /// Trait for subset sum operations
    pub trait SubsetSumStEphTrait<T: StT> {
        /// Create new subset sum solver
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        fn from_multiset(multiset: ArraySeqStEphS<T>) -> Self;

        /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
        /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(k×|S|) — sequential, span equals work
        fn subset_sum(&mut self, target: i32)         -> bool
        where
            T: Into<i32> + Copy;

        /// Get the multiset
        fn multiset(&self)                            -> &ArraySeqStEphS<T>;

        /// Get mutable multiset (ephemeral allows mutation)
        fn multiset_mut(&mut self)                    -> &mut ArraySeqStEphS<T>;

        /// Set element at index (ephemeral mutation)
        fn set(&mut self, index: usize, value: T);

        /// Clear memoization table
        fn clear_memo(&mut self);

        /// Get memoization table size
        fn memo_size(&self)                           -> usize;
    }

    /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
    /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(k×|S|) — sequential memoized recursion
    fn subset_sum_rec<T: StT + Into<i32> + Copy>(table: &mut SubsetSumStEphS<T>, i: usize, j: i32) -> bool {
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

    impl<T: StT> SubsetSumStEphTrait<T> for SubsetSumStEphS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                multiset: ArraySeqStEphS::new(0, T::default()),
                memo: HashMap::new(),
            }
        }

        fn from_multiset(multiset: ArraySeqStEphS<T>) -> Self {
            Self {
                multiset,
                memo: HashMap::new(),
            }
        }

        fn subset_sum(&mut self, target: i32) -> bool
        where
            T: Into<i32> + Copy,
        {
            if target < 0 {
                return false;
            }

            // Clear memo for fresh computation
            self.memo.clear();

            let n = self.multiset.length();
            subset_sum_rec(self, n, target)
        }

        fn multiset(&self) -> &ArraySeqStEphS<T> { &self.multiset }

        fn multiset_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.multiset }

        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            // Clear memo since multiset changed
            self.memo.clear();
        }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    impl<T: StT> Display for SubsetSumStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "SubsetSumStEph(multiset: {}, memo_entries: {})",
                self.multiset,
                self.memo.len()
            )
        }
    }

    impl<T: StT> IntoIterator for SubsetSumStEphS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStEphS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.into_iter() }
    }

    impl<T: StT> IntoIterator for &SubsetSumStEphS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStEphS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.clone().into_iter() }
    }

    impl<T: StT> IntoIterator for &mut SubsetSumStEphS<T> {
        type Item = T;
        type IntoIter = <ArraySeqStEphS<T> as IntoIterator>::IntoIter;

        fn into_iter(self) -> Self::IntoIter { self.multiset.clone().into_iter() }
    }
}

#[macro_export]
macro_rules! SubsetSumStEphLit {
    ($($x:expr),* $(,)?) => {
        $crate::Chap49::SubsetSumStEph::SubsetSumStEph::SubsetSumStEphS::from_multiset(
            $crate::ArraySeqStEphSLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumStEph::SubsetSumStEph::SubsetSumStEphS::new()
    };
}
