//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - persistent, single-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization, which Verus does not support. Full verification would require
//! replacing HashMap with a verified equivalent.

pub mod SubsetSumStPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::ArraySeqStPerSLit;

    verus! {
        proof fn _subset_sum_st_per_verified() {}
    }

    // 4. type definitions
    // Struct contains HashMap for memoization — cannot be inside verus!.

    #[derive(Clone, PartialEq, Eq)]
    pub struct SubsetSumStPerS<T: StT> {
        pub multiset: ArraySeqStPerS<T>,
        pub memo: HashMap<(usize, i32), bool>,
    }

    // 8. traits

    /// Trait for subset sum operations
    pub trait SubsetSumStPerTrait<T: StT>: Sized {
        /// Create new subset sum solver
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn from_multiset(multiset: ArraySeqStPerS<T>) -> Self;

        /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
        /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(k×|S|) — sequential, span equals work; outside verus!, not verified
        fn subset_sum(&self, target: i32)             -> bool
        where
            T: Into<i32> + Copy;

        /// Get the multiset
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn multiset(&self)                            -> &ArraySeqStPerS<T>;

        /// Get memoization table size
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn memo_size(&self)                           -> usize;
    }

    // 9. impls

    /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
    /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(k×|S|) — sequential memoized recursion; outside verus!, not verified
    fn subset_sum_rec<T: StT + Into<i32> + Copy>(table: &mut SubsetSumStPerS<T>, i: usize, j: i32) -> bool {
        if let Some(&result) = table.memo.get(&(i, j)) {
            return result;
        }

        let result = match (i, j) {
            | (_, 0) => true,
            | (0, _) => false,
            | (i, j) => {
                let element_value: i32 = (*table.multiset.nth(i - 1)).into();
                if element_value > j {
                    subset_sum_rec(table, i - 1, j)
                } else {
                    subset_sum_rec(table, i - 1, j - element_value) || subset_sum_rec(table, i - 1, j)
                }
            }
        };

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

            let mut solver = self.clone();
            solver.memo.clear();

            let n = solver.multiset.length();
            subset_sum_rec(&mut solver, n, target)
        }

        fn multiset(&self) -> &ArraySeqStPerS<T> { &self.multiset }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    // 13. derive impls outside verus!

    impl<T: StT> Debug for SubsetSumStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumStPerS")
                .field("multiset", &self.multiset)
                .field("memo", &self.memo)
                .finish()
        }
    }

    impl<T: StT> Display for SubsetSumStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
