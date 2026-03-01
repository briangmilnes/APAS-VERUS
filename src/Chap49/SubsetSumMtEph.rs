//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Subset Sum - ephemeral, multi-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<RwLock<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod SubsetSumMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::sync::Arc;
    use std::thread;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::ArraySeqMtEphChap19SLit;

    verus! {
        pub struct SubsetSumMtEphInv;
        impl RwLockPredicate<HashMap<(usize, i32), bool>> for SubsetSumMtEphInv {
            open spec fn inv(self, v: HashMap<(usize, i32), bool>) -> bool { true }
        }

        #[verifier::external_body]
        fn new_subset_sum_eph_lock(val: HashMap<(usize, i32), bool>) -> (lock: RwLock<HashMap<(usize, i32), bool>, SubsetSumMtEphInv>) {
            RwLock::new(val, Ghost(SubsetSumMtEphInv))
        }
    }

    // 4. type definitions

    #[derive(Clone)]
    pub struct SubsetSumMtEphS<T: MtVal> {
        pub multiset: ArraySeqMtEphS<T>,
        pub memo: Arc<RwLock<HashMap<(usize, i32), bool>, SubsetSumMtEphInv>>,
    }

    // 8. traits

    /// Trait for parallel subset sum operations
    pub trait SubsetSumMtEphTrait<T: MtVal>: Sized {
        /// Create new subset sum solver
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn new()                                      -> Self
        where
            T: Default;

        /// Create from multiset
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> Self;

        /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
        /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(|S|) — agrees with APAS; thread::spawn on both branches; outside verus!, not verified
        fn subset_sum(&mut self, target: i32)         -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static;

        /// Get the multiset
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn multiset(&self)                            -> &ArraySeqMtEphS<T>;

        /// Get mutable multiset (ephemeral allows mutation)
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn multiset_mut(&mut self)                    -> &mut ArraySeqMtEphS<T>;

        /// Set element at index (ephemeral mutation)
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn set(&mut self, index: usize, value: T);

        /// Clear memoization table
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn clear_memo(&mut self);

        /// Get memoization table size
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn memo_size(&self)                           -> usize;
    }

    // 9. impls

    /// - APAS: Work Θ(k×|S|), Span Θ(|S|)
    /// - Claude-Opus-4.6: Work Θ(k×|S|), Span Θ(|S|) — parallel fork on include/exclude branches; outside verus!, not verified
    fn subset_sum_rec<T: MtVal + Into<i32> + Copy + Send + Sync + 'static>(
        table: &SubsetSumMtEphS<T>,
        i: usize,
        j: i32,
    ) -> bool {
        {
            let handle = table.memo.acquire_read();
            let found = handle.borrow().get(&(i, j)).copied();
            handle.release_read();
            if let Some(result) = found {
                return result;
            }
        }

        let result = match (i, j) {
            | (_, 0) => true,
            | (0, _) => false,
            | (i, j) => {
                let element_value: i32 = (*table.multiset.nth(i - 1)).clone().into();
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
            let (mut current, write_handle) = table.memo.acquire_write();
            current.insert((i, j), result);
            write_handle.release_write(current);
        }

        result
    }

    impl<T: MtVal> SubsetSumMtEphTrait<T> for SubsetSumMtEphS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                multiset: ArraySeqMtEphS::new(0, T::default()),
                memo: Arc::new(new_subset_sum_eph_lock(HashMap::new())),
            }
        }

        fn from_multiset(multiset: ArraySeqMtEphS<T>) -> Self {
            Self {
                multiset,
                memo: Arc::new(new_subset_sum_eph_lock(HashMap::new())),
            }
        }

        fn subset_sum(&mut self, target: i32) -> bool
        where
            T: Into<i32> + Copy + Send + Sync + 'static,
        {
            if target < 0 {
                return false;
            }

            {
                let (mut current, write_handle) = self.memo.acquire_write();
                current.clear();
                write_handle.release_write(current);
            }

            let n = self.multiset.length();
            subset_sum_rec(self, n, target)
        }

        fn multiset(&self) -> &ArraySeqMtEphS<T> { &self.multiset }

        fn multiset_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.multiset }

        fn set(&mut self, index: usize, value: T) {
            let _ = self.multiset.set(index, value);
            let (mut current, write_handle) = self.memo.acquire_write();
            current.clear();
            write_handle.release_write(current);
        }

        fn clear_memo(&mut self) {
            let (mut current, write_handle) = self.memo.acquire_write();
            current.clear();
            write_handle.release_write(current);
        }

        fn memo_size(&self) -> usize {
            let handle = self.memo.acquire_read();
            let size = handle.borrow().len();
            handle.release_read();
            size
        }
    }

    // 11. derive impls

    impl<T: MtVal> PartialEq for SubsetSumMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.multiset == other.multiset }
    }

    impl<T: MtVal> Eq for SubsetSumMtEphS<T> {}

    // 13. derive impls outside verus!

    impl<T: MtVal> Debug for SubsetSumMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("SubsetSumMtEphS")
                .field("multiset", &self.multiset)
                .finish()
        }
    }

    impl<T: MtVal> Display for SubsetSumMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
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
            $crate::ArraySeqMtEphChap19SLit![$($x),*]
        )
    };
    () => {
        $crate::Chap49::SubsetSumMtEph::SubsetSumMtEph::SubsetSumMtEphS::new()
    };
}
