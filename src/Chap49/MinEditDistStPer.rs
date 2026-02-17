//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - persistent, single-threaded.

pub mod MinEditDistStPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::{Map, Zip};

    use crate::ArraySeqStPerSLit;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MinEditDistStPerS<T: StT> {
        source: ArraySeqStPerS<T>,
        target: ArraySeqStPerS<T>,
        memo: HashMap<(usize, usize), usize>,
    }

    /// Trait for minimum edit distance operations
    pub trait MinEditDistStPerTrait<T: StT> {
        /// Create new minimum edit distance solver
        fn new()                                                                -> Self
        where
            T: Default;

        /// Create from source and target sequences
        fn from_sequences(source: ArraySeqStPerS<T>, target: ArraySeqStPerS<T>) -> Self;

        /// Compute minimum edit distance between sequences
        /// claude-4-sonet: Work Θ(|S|×|T|), Span Θ(|S|+|T|), Parallelism Θ(1)
        fn min_edit_distance(&self)                                             -> usize;

        /// Get the source sequence
        fn source(&self)                                                        -> &ArraySeqStPerS<T>;

        /// Get the target sequence
        fn target(&self)                                                        -> &ArraySeqStPerS<T>;

        /// Get memoization table size
        fn memo_size(&self)                                                     -> usize;
    }

    /// Internal recursive minimum edit distance with memoization
    /// Claude Work: O(|S|*|T|) - each subproblem computed once
    /// Claude Span: O(|S|+|T|) - maximum recursion depth
    fn min_edit_distance_rec<T: StT>(table: &mut MinEditDistStPerS<T>, i: usize, j: usize) -> usize {
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

    impl<T: StT> MinEditDistStPerTrait<T> for MinEditDistStPerS<T> {
        fn new() -> Self
        where
            T: Default,
        {
            Self {
                source: ArraySeqStPerS::new(0, T::default()),
                target: ArraySeqStPerS::new(0, T::default()),
                memo: HashMap::new(),
            }
        }

        fn from_sequences(source: ArraySeqStPerS<T>, target: ArraySeqStPerS<T>) -> Self {
            Self {
                source,
                target,
                memo: HashMap::new(),
            }
        }

        fn min_edit_distance(&self) -> usize {
            // Create mutable copy for memoization
            let mut solver = self.clone();
            solver.memo.clear(); // Fresh memo for each query

            let source_len = solver.source.length();
            let target_len = solver.target.length();

            min_edit_distance_rec(&mut solver, source_len, target_len)
        }

        fn source(&self) -> &ArraySeqStPerS<T> { &self.source }

        fn target(&self) -> &ArraySeqStPerS<T> { &self.target }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    impl<T: StT> Display for MinEditDistStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "MinEditDistStPer(source: {}, target: {}, memo_entries: {})",
                self.source,
                self.target,
                self.memo.len()
            )
        }
    }

    impl<T: StT> IntoIterator for MinEditDistStPerS<T> {
        type Item = Pair<T, T>;
        type IntoIter = Map<
            Zip<<ArraySeqStPerS<T> as IntoIterator>::IntoIter, <ArraySeqStPerS<T> as IntoIterator>::IntoIter>,
            fn((T, T)) -> Pair<T, T>,
        >;

        fn into_iter(self) -> Self::IntoIter { self.source.into_iter().zip(self.target).map(|(a, b)| Pair(a, b)) }
    }

    impl<T: StT> IntoIterator for &MinEditDistStPerS<T> {
        type Item = Pair<T, T>;
        type IntoIter = Map<
            Zip<<ArraySeqStPerS<T> as IntoIterator>::IntoIter, <ArraySeqStPerS<T> as IntoIterator>::IntoIter>,
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
macro_rules! MinEditDistStPerLit {
    (source: [$($s:expr),* $(,)?], target: [$($t:expr),* $(,)?]) => {
        $crate::Chap49::MinEditDistStPer::MinEditDistStPer::MinEditDistStPerS::from_sequences(
            $crate::ArraySeqStPerSLit![$($s),*],
            $crate::ArraySeqStPerSLit![$($t),*]
        )
    };
    () => {
        $crate::Chap49::MinEditDistStPer::MinEditDistStPer::MinEditDistStPerS::new()
    };
}
