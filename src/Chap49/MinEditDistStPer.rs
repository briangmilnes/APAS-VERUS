//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum Edit Distance - persistent, single-threaded.

pub mod MinEditDistStPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::iter::{Map, Zip};

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::ArraySeqStPerSLit;

    verus! {
    } // verus!

    // 4. type definitions

    #[derive(Clone, PartialEq, Eq)]
    pub struct MinEditDistStPerS<T: StT> {
        pub source: ArraySeqStPerS<T>,
        pub target: ArraySeqStPerS<T>,
        pub memo: HashMap<(usize, usize), usize>,
    }

    // 8. traits

    /// Trait for minimum edit distance operations
    pub trait MinEditDistStPerTrait<T: StT>: Sized {
        /// Create new minimum edit distance solver
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn new()                                                                -> Self
        where
            T: Default;

        /// Create from source and target sequences
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn from_sequences(source: ArraySeqStPerS<T>, target: ArraySeqStPerS<T>) -> Self;

        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential, span equals work; outside verus!, not verified
        fn min_edit_distance(&self)                                             -> usize;

        /// Get the source sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn source(&self)                                                        -> &ArraySeqStPerS<T>;

        /// Get the target sequence
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn target(&self)                                                        -> &ArraySeqStPerS<T>;

        /// Get memoization table size
        /// - APAS: not specified
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — outside verus!, not verified
        fn memo_size(&self)                                                     -> usize;
    }

    // 9. impls

    /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|)
    /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential memoized recursion; outside verus!, not verified
    fn min_edit_distance_rec<T: StT>(table: &mut MinEditDistStPerS<T>, i: usize, j: usize) -> usize {
        if let Some(&result) = table.memo.get(&(i, j)) {
            return result;
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
                    let delete_cost = min_edit_distance_rec(table, i - 1, j);
                    let insert_cost = min_edit_distance_rec(table, i, j - 1);

                    1 + std::cmp::min(delete_cost, insert_cost)
                }
            }
        };

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
            let mut solver = self.clone();
            solver.memo.clear();

            let source_len = solver.source.length();
            let target_len = solver.target.length();

            min_edit_distance_rec(&mut solver, source_len, target_len)
        }

        fn source(&self) -> &ArraySeqStPerS<T> { &self.source }

        fn target(&self) -> &ArraySeqStPerS<T> { &self.target }

        fn memo_size(&self) -> usize { self.memo.len() }
    }

    // 13. derive impls outside verus!

    impl<T: StT> Debug for MinEditDistStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("MinEditDistStPerS")
                .field("source", &self.source)
                .field("target", &self.target)
                .field("memo", &self.memo)
                .finish()
        }
    }

    impl<T: StT> Display for MinEditDistStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
