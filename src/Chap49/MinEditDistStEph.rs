//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - ephemeral, single-threaded.

pub mod MinEditDistStEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::{Map, Zip};

    use crate::ArraySeqStEphSLit;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MinEditDistStEphS<T: StT> {
        source: ArraySeqStEphS<T>,
        target: ArraySeqStEphS<T>,
        memo: HashMap<(usize, usize), usize>,
    }

    /// Trait for minimum edit distance operations
    pub trait MinEditDistStEphTrait<T: StT> {
        /// Create new minimum edit distance solver
        fn new()                                                                -> Self
        where
            T: Default;

        /// Create from source and target sequences
        fn from_sequences(source: ArraySeqStEphS<T>, target: ArraySeqStEphS<T>) -> Self;

        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential, span equals work
        fn min_edit_distance(&mut self)                                         -> usize;

        /// Get the source sequence
        fn source(&self)                                                        -> &ArraySeqStEphS<T>;

        /// Get the target sequence
        fn target(&self)                                                        -> &ArraySeqStEphS<T>;

        /// Get mutable source sequence (ephemeral allows mutation)
        fn source_mut(&mut self)                                                -> &mut ArraySeqStEphS<T>;

        /// Get mutable target sequence (ephemeral allows mutation)
        fn target_mut(&mut self)                                                -> &mut ArraySeqStEphS<T>;

        /// Set element in source sequence
        fn set_source(&mut self, index: usize, value: T);

        /// Set element in target sequence
        fn set_target(&mut self, index: usize, value: T);

        /// Clear memoization table
        fn clear_memo(&mut self);

        /// Get memoization table size
        fn memo_size(&self)                                                     -> usize;
    }

    /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
    /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential memoized recursion
    fn min_edit_distance_rec<T: StT>(table: &mut MinEditDistStEphS<T>, i: usize, j: usize) -> usize {
        // Check memo first
        if let Some(&result) = table.memo.get(&(i, j)) {
            return result;
        }

        let result = match (i, j) {
            | (i, 0) => i, // Base case: need i deletions
            | (0, j) => j, // Base case: need j insertions
            | (i, j) => {
                let source_char = table.source.nth(i - 1);
                let target_char = table.target.nth(j - 1);

                if source_char == target_char {
                    // Characters match, no edit needed
                    min_edit_distance_rec(table, i - 1, j - 1)
                } else {
                    // Characters don't match, try both operations
                    let delete_cost = min_edit_distance_rec(table, i - 1, j);
                    let insert_cost = min_edit_distance_rec(table, i, j - 1);

                    1 + std::cmp::min(delete_cost, insert_cost)
                }
            }
        };

        // Memoize result
        table.memo.insert((i, j), result);
        result
    }

    impl<T: StT> MinEditDistStEphTrait<T> for MinEditDistStEphS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                source: ArraySeqStEphS::new(0, T::default()),
                target: ArraySeqStEphS::new(0, T::default()),
                memo: HashMap::new(),
            }
        }

        fn from_sequences(source: ArraySeqStEphS<T>, target: ArraySeqStEphS<T>) -> Self {
            Self {
                source,
                target,
                memo: HashMap::new(),
            }
        }

        fn min_edit_distance(&mut self) -> usize {
            // Clear memo for fresh computation
            self.memo.clear();

            let source_len = self.source.length();
            let target_len = self.target.length();

            min_edit_distance_rec(self, source_len, target_len)
        }

        fn source(&self) -> &ArraySeqStEphS<T> { &self.source }

        fn target(&self) -> &ArraySeqStEphS<T> { &self.target }

        fn source_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.source }

        fn target_mut(&mut self) -> &mut ArraySeqStEphS<T> { &mut self.target }

        fn set_source(&mut self, index: usize, value: T) {
            let _ = self.source.set(index, value);
            // Clear memo since source changed
            self.memo.clear();
        }

        fn set_target(&mut self, index: usize, value: T) {
            let _ = self.target.set(index, value);
            // Clear memo since target changed
            self.memo.clear();
        }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    impl<T: StT> Display for MinEditDistStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "MinEditDistStEph(source: {}, target: {}, memo_entries: {})",
                self.source,
                self.target,
                self.memo.len()
            )
        }
    }

    impl<T: StT> IntoIterator for MinEditDistStEphS<T> {
        type Item = Pair<T, T>;
        type IntoIter = Map<
            Zip<<ArraySeqStEphS<T> as IntoIterator>::IntoIter, <ArraySeqStEphS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter { self.source.into_iter().zip(self.target).map(|(a, b)| Pair(a, b)) }
    }

    impl<T: StT> IntoIterator for &MinEditDistStEphS<T> {
        type Item = Pair<T, T>;
        type IntoIter = Map<
            Zip<<ArraySeqStEphS<T> as IntoIterator>::IntoIter, <ArraySeqStEphS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter {
            self.source
                .clone()
                .into_iter()
                .zip(self.target.clone())
                .map(|(a, b)| Pair(a, b))
        }
    }

    impl<T: StT> IntoIterator for &mut MinEditDistStEphS<T> {
        type Item = Pair<T, T>;
        type IntoIter = Map<
            Zip<<ArraySeqStEphS<T> as IntoIterator>::IntoIter, <ArraySeqStEphS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter {
            self.source
                .clone()
                .into_iter()
                .zip(self.target.clone())
                .map(|(a, b)| Pair(a, b))
        }
    }
}

#[macro_export]
macro_rules! MinEditDistStEphLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistStEph::MinEditDistStEph::MinEditDistStEphS::from_sequences(
            $crate::ArraySeqStEphSLit![$($s),*],
            $crate::ArraySeqStEphSLit![$($t),*]
        )
    };
    () => {
        $crate::Chap49::MinEditDistStEph::MinEditDistStEph::MinEditDistStEphS::new()
    };
}
