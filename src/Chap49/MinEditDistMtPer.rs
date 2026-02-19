//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - persistent, multi-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<RwLock<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod MinEditDistMtPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::sync::Arc;
    use std::thread;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    verus! {
        pub struct MinEditDistMtPerWf;
        impl RwLockPredicate<HashMap<(usize, usize), usize>> for MinEditDistMtPerWf {
            open spec fn inv(self, v: HashMap<(usize, usize), usize>) -> bool { true }
        }

        #[verifier::external_body]
        fn new_min_edit_dist_per_lock(val: HashMap<(usize, usize), usize>) -> (lock: RwLock<HashMap<(usize, usize), usize>, MinEditDistMtPerWf>) {
            RwLock::new(val, Ghost(MinEditDistMtPerWf))
        }
    }

    // 4. type definitions

    #[derive(Clone)]
    pub struct MinEditDistMtPerS<T: MtVal> {
        pub source: ArraySeqMtPerS<T>,
        pub target: ArraySeqMtPerS<T>,
        pub memo: Arc<RwLock<HashMap<(usize, usize), usize>, MinEditDistMtPerWf>>,
    }

    // 8. traits

    /// Trait for parallel minimum edit distance operations
    pub trait MinEditDistMtPerTrait<T: MtVal>: Sized {
        /// Create new minimum edit distance solver
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn new()                                                                -> Self
        where
            T: Default;

        /// Create from source and target sequences
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn from_sequences(source: ArraySeqMtPerS<T>, target: ArraySeqMtPerS<T>) -> Self;

        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — agrees with APAS; thread::spawn on delete/insert; outside verus!, not verified
        fn min_edit_distance(&self)                                             -> usize
        where
            T: Send + Sync + 'static;

        /// Get the source sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn source(&self)                                                        -> &ArraySeqMtPerS<T>;

        /// Get the target sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn target(&self)                                                        -> &ArraySeqMtPerS<T>;

        /// Get memoization table size
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn memo_size(&self)                                                     -> usize;
    }

    // 9. impls

    /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
    /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — parallel fork on delete/insert branches; outside verus!, not verified
    fn min_edit_distance_rec<T: MtVal + Send + Sync + 'static>(
        table: &MinEditDistMtPerS<T>,
        i: usize,
        j: usize,
    ) -> usize {
        {
            let handle = table.memo.acquire_read();
            let found = handle.borrow().get(&(i, j)).copied();
            handle.release_read();
            if let Some(result) = found {
                return result;
            }
        }

        let result = match (i, j) {
            | (i, 0) => i,
            | (0, j) => j,
            | (i, j) => {
                let source_char = table.source.nth(i - 1);
                let target_char = table.target.nth(j - 1);

                if source_char == target_char {
                    min_edit_distance_rec(table, i - 1, j - 1)
                } else {
                    let table_clone1 = table.clone();
                    let table_clone2 = table.clone();

                    let handle1 = thread::spawn(move || min_edit_distance_rec(&table_clone1, i - 1, j));
                    let handle2 = thread::spawn(move || min_edit_distance_rec(&table_clone2, i, j - 1));

                    let delete_cost = handle1.join().unwrap();
                    let insert_cost = handle2.join().unwrap();

                    1 + std::cmp::min(delete_cost, insert_cost)
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

    impl<T: MtVal> MinEditDistMtPerTrait<T> for MinEditDistMtPerS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                source: ArraySeqMtPerS::new(0, T::default()),
                target: ArraySeqMtPerS::new(0, T::default()),
                memo: Arc::new(new_min_edit_dist_per_lock(HashMap::new())),
            }
        }

        fn from_sequences(source: ArraySeqMtPerS<T>, target: ArraySeqMtPerS<T>) -> Self {
            Self {
                source,
                target,
                memo: Arc::new(new_min_edit_dist_per_lock(HashMap::new())),
            }
        }

        fn min_edit_distance(&self) -> usize
        where
            T: Send + Sync + 'static,
        {
            {
                let (mut current, write_handle) = self.memo.acquire_write();
                current.clear();
                write_handle.release_write(current);
            }

            let source_len = self.source.length();
            let target_len = self.target.length();

            min_edit_distance_rec(self, source_len, target_len)
        }

        fn source(&self) -> &ArraySeqMtPerS<T> { &self.source }

        fn target(&self) -> &ArraySeqMtPerS<T> { &self.target }

        fn memo_size(&self) -> usize {
            let handle = self.memo.acquire_read();
            let size = handle.borrow().len();
            handle.release_read();
            size
        }
    }

    // 11. derive impls

    impl<T: MtVal> PartialEq for MinEditDistMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.source == other.source && self.target == other.target }
    }

    impl<T: MtVal> Eq for MinEditDistMtPerS<T> {}

    // 13. derive impls outside verus!

    impl<T: MtVal> Debug for MinEditDistMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistMtPerS")
                .field("source", &self.source)
                .field("target", &self.target)
                .finish()
        }
    }

    impl<T: MtVal> Display for MinEditDistMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
            };
            write!(
                f,
                "MinEditDistMtPer(source: {}, target: {}, memo_entries: {})",
                self.source, self.target, memo_size
            )
        }
    }

    // Note: IntoIterator not implemented for ArraySeqMtPerS, so we don't provide it here
}

#[macro_export]
macro_rules! MinEditDistMtPerLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistMtPer::MinEditDistMtPer::MinEditDistMtPerS::from_sequences(
            <$crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<_> as $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerBaseTrait<_>>::from_vec(vec![$($s),*]),
            <$crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<_> as $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerBaseTrait<_>>::from_vec(vec![$($t),*])
        )
    };
    () => {
        $crate::Chap49::MinEditDistMtPer::MinEditDistMtPer::MinEditDistMtPerS::new()
    };
}
