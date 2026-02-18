//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - persistent, multi-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<Mutex<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod SubsetSumMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    // 4. type definitions
    // Struct contains Arc<Mutex<HashMap>> for memoization — cannot be inside verus!.

    #[derive(Clone)]
    pub struct SubsetSumMtPerS<T: MtVal> {
        pub multiset: ArraySeqMtPerS<T>,
        pub memo: Arc<Mutex<HashMap<(usize, i32), bool>>>,
    }

    // 8. traits

    /// Trait for parallel subset sum operations
    pub trait SubsetSumMtPerTrait<T: MtVal>: Sized {
        /// Create new subset sum solver
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn from_multiset(multiset: ArraySeqMtPerS<T>) -> Self;

        /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
        /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(|S|) — agrees with APAS; thread::spawn on both branches; outside verus!, not verified
        fn subset_sum(&self, target: i32)             -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static;

        /// Get the multiset
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn multiset(&self)                            -> &ArraySeqMtPerS<T>;

        /// Get memoization table size
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn memo_size(&self)                           -> usize;
    }

    // 9. impls

    /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
    /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(|S|) — parallel fork on include/exclude branches; outside verus!, not verified
    fn subset_sum_rec<T: MtVal + Into<i32> + Copy + Send + Sync + 'static>(
        table: &SubsetSumMtPerS<T>,
        i: usize,
        j: i32,
    ) -> bool {
        {
            let memo_guard = table.memo.lock().unwrap();
            if let Some(&result) = memo_guard.get(&(i, j)) {
                return result;
            }
        }

        let result = match (i, j) {
            | (_, 0) => true,
            | (0, _) => false,
            | (i, j) => {
                let element_value: i32 = (*table.multiset.nth(i - 1)).into();
                if element_value > j {
                    subset_sum_rec(table, i - 1, j)
                } else {
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

    // 11. derive impls

    impl<T: MtVal> PartialEq for SubsetSumMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.multiset == other.multiset }
    }

    impl<T: MtVal> Eq for SubsetSumMtPerS<T> {}

    // 13. derive impls outside verus!

    impl<T: MtVal> Debug for SubsetSumMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumMtPerS")
                .field("multiset", &self.multiset)
                .finish()
        }
    }

    impl<T: MtVal> Display for SubsetSumMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
