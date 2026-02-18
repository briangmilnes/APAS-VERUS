//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - ephemeral, single-threaded.

pub mod SubsetSumStEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    use crate::ArraySeqStEphSLit;

    verus! {
    } // verus!

    // 4. type definitions

    #[derive(Clone, PartialEq, Eq)]
    pub struct SubsetSumStEphS<T: StT> {
        pub multiset: ArraySeqStEphS<T>,
        pub memo: HashMap<(usize, i32), bool>,
    }

    // 8. traits

    /// Trait for subset sum operations
    pub trait SubsetSumStEphTrait<T: StT>: Sized {
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

    // 9. impls

    /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
    /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(k×|S|) — sequential memoized recursion
    fn subset_sum_rec<T: StT + Into<i32> + Copy>(table: &mut SubsetSumStEphS<T>, i: usize, j: i32) -> bool {
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

            self.memo.clear();

            let n = self.multiset.length();
            subset_sum_rec(self, n, target)
        }

        fn multiset(&self) -> &ArraySeqStEphS<T> { &self.multiset }

        fn multiset_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.multiset }

        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            self.memo.clear();
        }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    // 13. derive impls outside verus!

    impl<T: StT> Debug for SubsetSumStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumStEphS")
                .field("multiset", &self.multiset)
                .field("memo", &self.memo)
                .finish()
        }
    }

    impl<T: StT> Display for SubsetSumStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
