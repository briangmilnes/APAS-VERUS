//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - ephemeral, multi-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<RwLock<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod MinEditDistMtEph {

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
        pub struct MinEditDistMtEphInv;
        impl RwLockPredicate<HashMap<(usize, usize), usize>> for MinEditDistMtEphInv {
            open spec fn inv(self, v: HashMap<(usize, usize), usize>) -> bool { true }
        }

        #[verifier::external_body]
        fn new_min_edit_dist_eph_lock(val: HashMap<(usize, usize), usize>) -> (lock: RwLock<HashMap<(usize, usize), usize>, MinEditDistMtEphInv>) {
            RwLock::new(val, Ghost(MinEditDistMtEphInv))
        }
    }

    // 4. type definitions

    #[derive(Clone)]
    pub struct MinEditDistMtEphS<T: MtVal> {
        pub source: ArraySeqMtEphS<T>,
        pub target: ArraySeqMtEphS<T>,
        pub memo: Arc<RwLock<HashMap<(usize, usize), usize>, MinEditDistMtEphInv>>,
    }

    // 8. traits

    /// Trait for parallel minimum edit distance operations
    pub trait MinEditDistMtEphTrait<T: MtVal>: Sized {
        /// Create new minimum edit distance solver
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn new()                                                                -> Self
        where
            T: Default;

        /// Create from source and target sequences
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> Self;

        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — agrees with APAS; thread::spawn on delete/insert; outside verus!, not verified
        fn min_edit_distance(&mut self)                                         -> usize
        where
            T: Send + Sync + 'static;

        /// Get the source sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn source(&self)                                                        -> &ArraySeqMtEphS<T>;

        /// Get the target sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn target(&self)                                                        -> &ArraySeqMtEphS<T>;

        /// Get mutable source sequence (ephemeral allows mutation)
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn source_mut(&mut self)                                                -> &mut ArraySeqMtEphS<T>;

        /// Get mutable target sequence (ephemeral allows mutation)
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn target_mut(&mut self)                                                -> &mut ArraySeqMtEphS<T>;

        /// Set element in source sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn set_source(&mut self, index: usize, value: T);

        /// Set element in target sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn set_target(&mut self, index: usize, value: T);

        /// Clear memoization table
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn clear_memo(&mut self);

        /// Get memoization table size
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn memo_size(&self)                                                     -> usize;
    }

    // 9. impls

    fn min_edit_distance_rec<T: MtVal + Send + Sync + 'static>(
        source: &ArraySeqMtEphS<T>,
        target: &ArraySeqMtEphS<T>,
        memo: &Arc<RwLock<HashMap<(usize, usize), usize>, MinEditDistMtEphInv>>,
        i: usize,
        j: usize,
    ) -> usize {
        {
            let handle = memo.acquire_read();
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
                let source_char = source.nth(i - 1).clone();
                let target_char = target.nth(j - 1).clone();

                if source_char == target_char {
                    min_edit_distance_rec(source, target, memo, i - 1, j - 1)
                } else {
                    let source1 = source.clone();
                    let target1 = target.clone();
                    let memo1 = Arc::clone(memo);
                    let source2 = source.clone();
                    let target2 = target.clone();
                    let memo2 = Arc::clone(memo);

                    let handle1 = thread::spawn(move || min_edit_distance_rec(&source1, &target1, &memo1, i - 1, j));
                    let handle2 = thread::spawn(move || min_edit_distance_rec(&source2, &target2, &memo2, i, j - 1));

                    let delete_cost = handle1.join().unwrap();
                    let insert_cost = handle2.join().unwrap();

                    1 + std::cmp::min(delete_cost, insert_cost)
                }
            }
        };

        {
            let (mut current, write_handle) = memo.acquire_write();
            current.insert((i, j), result);
            write_handle.release_write(current);
        }

        result
    }

    impl<T: MtVal> MinEditDistMtEphTrait<T> for MinEditDistMtEphS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                source: ArraySeqMtEphS::new(0, T::default()),
                target: ArraySeqMtEphS::new(0, T::default()),
                memo: Arc::new(new_min_edit_dist_eph_lock(HashMap::new())),
            }
        }

        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> Self {
            Self {
                source,
                target,
                memo: Arc::new(new_min_edit_dist_eph_lock(HashMap::new())),
            }
        }

        fn min_edit_distance(&mut self) -> usize
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

            min_edit_distance_rec(&self.source, &self.target, &self.memo, source_len, target_len)
        }

        fn source(&self) -> &ArraySeqMtEphS<T> { &self.source }

        fn target(&self) -> &ArraySeqMtEphS<T> { &self.target }

        fn source_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.source }

        fn target_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.target }

        fn set_source(&mut self, index: usize, value: T) {
            let _ = self.source.set(index, value);
            let (mut current, write_handle) = self.memo.acquire_write();
            current.clear();
            write_handle.release_write(current);
        }

        fn set_target(&mut self, index: usize, value: T) {
            let _ = self.target.set(index, value);
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

    impl<T: MtVal> PartialEq for MinEditDistMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.source == other.source && self.target == other.target }
    }

    impl<T: MtVal> Eq for MinEditDistMtEphS<T> {}

    // 13. derive impls outside verus!

    impl<T: MtVal> Debug for MinEditDistMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistMtEphS")
                .field("source", &self.source)
                .field("target", &self.target)
                .finish()
        }
    }

    impl<T: MtVal> Display for MinEditDistMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            let memo_size = {
                let handle = self.memo.acquire_read();
                let size = handle.borrow().len();
                handle.release_read();
                size
            };
            write!(
                f,
                "MinEditDistMtEph(source: {}, target: {}, memo_entries: {})",
                self.source, self.target, memo_size
            )
        }
    }

    // Note: IntoIterator not implemented for ArraySeqMtEphS, so we don't provide it here
}

#[macro_export]
macro_rules! MinEditDistMtEphLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistMtEph::MinEditDistMtEph::MinEditDistMtEphS::from_sequences(
            $crate::ArraySeqMtEphChap19SLit![$($s),*],
            $crate::ArraySeqMtEphChap19SLit![$($t),*]
        )
    };
    () => {
        $crate::Chap49::MinEditDistMtEph::MinEditDistMtEph::MinEditDistMtEphS::new()
    };
}
