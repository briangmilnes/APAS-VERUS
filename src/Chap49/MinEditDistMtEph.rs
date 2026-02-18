//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - ephemeral, multi-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization (via Arc<Mutex<HashMap>>), which Verus does not support. Full
//! verification would require replacing HashMap with a verified equivalent.

pub mod MinEditDistMtEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::ArraySeqMtEphChap19SLit;

    // 4. type definitions
    // Struct contains Arc<Mutex<HashMap>> for memoization — cannot be inside verus!.

    #[derive(Clone)]
    pub struct MinEditDistMtEphS<T: MtVal> {
        pub source: ArraySeqMtEphS<T>,
        pub target: ArraySeqMtEphS<T>,
        pub memo: Arc<Mutex<HashMap<(usize, usize), usize>>>,
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

    impl<T: MtVal> MinEditDistMtEphS<T> {
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — parallel fork on delete/insert branches; outside verus!, not verified
        fn min_edit_distance_rec(&self, i: usize, j: usize) -> usize
        where
            T: Send + Sync + 'static,
        {
            {
                let memo_guard = self.memo.lock().unwrap();
                if let Some(&result) = memo_guard.get(&(i, j)) {
                    return result;
                }
            }

            let result = match (i, j) {
                | (i, 0) => i,
                | (0, j) => j,
                | (i, j) => {
                    let source_char = self.source.nth(i - 1).clone();
                    let target_char = self.target.nth(j - 1).clone();

                    if source_char == target_char {
                        self.min_edit_distance_rec(i - 1, j - 1)
                    } else {
                        let self_clone1 = self.clone();
                        let self_clone2 = self.clone();

                        let handle1 = thread::spawn(move || self_clone1.min_edit_distance_rec(i - 1, j));
                        let handle2 = thread::spawn(move || self_clone2.min_edit_distance_rec(i, j - 1));

                        let delete_cost = handle1.join().unwrap();
                        let insert_cost = handle2.join().unwrap();

                        1 + std::cmp::min(delete_cost, insert_cost)
                    }
                }
            };

            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.insert((i, j), result);
            }

            result
        }
    }

    impl<T: MtVal> MinEditDistMtEphTrait<T> for MinEditDistMtEphS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                source: ArraySeqMtEphS::new(0, T::default()),
                target: ArraySeqMtEphS::new(0, T::default()),
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn from_sequences(source: ArraySeqMtEphS<T>, target: ArraySeqMtEphS<T>) -> Self {
            Self {
                source,
                target,
                memo: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn min_edit_distance(&mut self) -> usize
        where
            T: Send + Sync + 'static,
        {
            {
                let mut memo_guard = self.memo.lock().unwrap();
                memo_guard.clear();
            }

            let source_len = self.source.length();
            let target_len = self.target.length();

            self.min_edit_distance_rec(source_len, target_len)
        }

        fn source(&self) -> &ArraySeqMtEphS<T> { &self.source }

        fn target(&self) -> &ArraySeqMtEphS<T> { &self.target }

        fn source_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.source }

        fn target_mut(&mut self) -> &mut ArraySeqMtEphS<T> { &mut self.target }

        fn set_source(&mut self, index: usize, value: T) {
            let _ = self.source.set(index, value);
            let mut memo_guard = self.memo.lock().unwrap();
            memo_guard.clear();
        }

        fn set_target(&mut self, index: usize, value: T) {
            let _ = self.target.set(index, value);
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
                let memo_guard = self.memo.lock().unwrap();
                memo_guard.len()
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
