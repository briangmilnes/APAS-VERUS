//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Persistent Single-Threaded Implementation.
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using HashMap for efficient subproblem caching.

pub mod TopDownDPStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 8. traits
    // 9. impls
    // 11. derive impls
    // 13. derive impls outside verus!

    // 2. imports
    use std::collections::HashMap;
    use std::fmt::{Formatter, Debug, Display};

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone, PartialEq, Eq)]
    pub struct TopDownDPStPerS {
        /// Input sequence S
        pub seq_s: ArraySeqStPerS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqStPerS<char>,
        /// Memoization table for subproblem results
        pub memo_table: HashMap<(usize, usize), usize>,
    }

    // 8. traits

    pub trait TopDownDPStPerTrait: Sized {
        fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> Self;
        fn med_memoized(&self) -> usize;
        fn with_memo_table(self, memo: HashMap<(usize, usize), usize>) -> Self;
        fn memo_size(&self) -> usize;
        fn is_memoized(&self, i: usize, j: usize) -> bool;
        fn get_memoized(&self, i: usize, j: usize) -> Option<usize>;
        fn s_length(&self) -> usize;
        fn t_length(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn clear_memo(self) -> Self;
        fn med_recursive(&self, i: usize, j: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize;
    }

    // 9. impls

    impl TopDownDPStPerTrait for TopDownDPStPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> Self {
            TopDownDPStPerS {
                seq_s: s,
                seq_t: t,
                memo_table: HashMap::new(),
            }
        }

        /// Compute minimum edit distance using top-down memoization (Algorithm 51.4).
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|×|T|)
        fn med_memoized(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            let mut memo = self.memo_table.clone();
            self.med_recursive(s_len, t_len, &mut memo)
        }

        fn with_memo_table(self, memo: HashMap<(usize, usize), usize>) -> Self {
            TopDownDPStPerS { seq_s: self.seq_s, seq_t: self.seq_t, memo_table: memo }
        }

        fn memo_size(&self) -> usize { self.memo_table.len() }
        fn is_memoized(&self, i: usize, j: usize) -> bool { self.memo_table.contains_key(&(i, j)) }
        fn get_memoized(&self, i: usize, j: usize) -> Option<usize> { self.memo_table.get(&(i, j)).copied() }
        fn s_length(&self) -> usize { self.seq_s.length() }
        fn t_length(&self) -> usize { self.seq_t.length() }
        fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }

        fn clear_memo(self) -> Self {
            TopDownDPStPerS { seq_s: self.seq_s, seq_t: self.seq_t, memo_table: HashMap::new() }
        }

        /// Recursive MED with memoization (medOne from Algorithm 51.4).
        fn med_recursive(&self, i: usize, j: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
            if let Some(&cached_result) = memo.get(&(i, j)) {
                return cached_result;
            }

            let result = match (i, j) {
                | (0, j) => j,
                | (i, 0) => i,
                | (i, j) => {
                    let s_char = *self.seq_s.nth(i - 1);
                    let t_char = *self.seq_t.nth(j - 1);

                    if s_char == t_char {
                        self.med_recursive(i - 1, j - 1, memo)
                    } else {
                        let insert_cost = 1 + self.med_recursive(i, j - 1, memo);
                        let delete_cost = 1 + self.med_recursive(i - 1, j, memo);
                        insert_cost.min(delete_cost)
                    }
                }
            };

            memo.insert((i, j), result);
            result
        }
    }

    // 11. derive impls
    impl Default for TopDownDPStPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn default() -> Self {
            let empty_s = ArraySeqStPerS::new(0, ' ');
            let empty_t = ArraySeqStPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for TopDownDPStPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPStPerS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", &self.memo_table)
                .finish()
        }
    }

    impl Display for TopDownDPStPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPStPer(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
