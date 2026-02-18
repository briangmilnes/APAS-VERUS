//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Ephemeral Single-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using HashMap with in-place mutations for efficient subproblem caching.

pub mod TopDownDPStEph {

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
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone, PartialEq, Eq)]
    pub struct TopDownDPStEphS {
        /// Input sequence S
        pub seq_s: ArraySeqStEphS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqStEphS<char>,
        /// Memoization table for subproblem results
        pub memo_table: HashMap<(usize, usize), usize>,
    }

    // 8. traits
    /// Trait for top-down dynamic programming operations
    pub trait TopDownDPStEphTrait<T: StT> : Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn new()                     -> Self;

        /// - APAS: Work O(|S|×|T|), Span O(|S|×|T|) — inherently sequential (memo threading).
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — agrees with APAS.
        fn solve(&self, input: &[T]) -> T;
    }

    // 9. impls
    impl TopDownDPStEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn new(s: ArraySeqStEphS<char>, t: ArraySeqStEphS<char>) -> Self {
            TopDownDPStEphS {
                seq_s: s,
                seq_t: t,
                memo_table: HashMap::new(),
            }
        }

        /// Compute minimum edit distance using top-down memoization (Algorithm 51.4).
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — inherently sequential (memo threading).
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — agrees with APAS.
        pub fn med_memoized(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            self.med_recursive(s_len, t_len)
        }

        /// Recursive MED with memoization (medOne from Algorithm 51.4).
        /// - APAS: Work Θ(1) amortized per call, Θ(|S|×|T|) total; Span Θ(|S|×|T|).
        /// - Claude-Opus-4.6: Work Θ(1) amortized, Span Θ(|S|×|T|) — agrees with APAS.
        fn med_recursive(&mut self, i: usize, j: usize) -> usize {
            // Check memo table first
            if let Some(&cached_result) = self.memo_table.get(&(i, j)) {
                return cached_result;
            }

            // Base cases
            let result = match (i, j) {
                | (0, j) => j, // Insert all remaining characters from T
                | (i, 0) => i, // Delete all remaining characters from S
                | (i, j) => {
                    let s_char = *self.seq_s.nth(i - 1);
                    let t_char = *self.seq_t.nth(j - 1);

                    if s_char == t_char {
                        // Characters match: no edit needed
                        self.med_recursive(i - 1, j - 1)
                    } else {
                        // Characters don't match: insert or delete (APAS Algorithm 51.4)
                        let insert_cost = 1 + self.med_recursive(i, j - 1);
                        let delete_cost = 1 + self.med_recursive(i - 1, j);

                        insert_cost.min(delete_cost)
                    }
                }
            };

            // Store result in memo table
            self.memo_table.insert((i, j), result);
            result
        }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn memo_size(&self) -> usize { self.memo_table.len() }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn is_memoized(&self, i: usize, j: usize) -> bool { self.memo_table.contains_key(&(i, j)) }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn get_memoized(&self, i: usize, j: usize) -> Option<usize> { self.memo_table.get(&(i, j)).copied() }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn insert_memo(&mut self, i: usize, j: usize, value: usize) { self.memo_table.insert((i, j), value); }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn s_length(&self) -> usize { self.seq_s.length() }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn t_length(&self) -> usize { self.seq_t.length() }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn clear_memo(&mut self) { self.memo_table.clear(); }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn set_s(&mut self, s: ArraySeqStEphS<char>) {
            self.seq_s = s;
            self.clear_memo();
        }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn set_t(&mut self, t: ArraySeqStEphS<char>) {
            self.seq_t = t;
            self.clear_memo();
        }
    }

    // 11. derive impls
    impl Default for TopDownDPStEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn default() -> Self {
            let empty_s = ArraySeqStEphS::new(0, ' ');
            let empty_t = ArraySeqStEphS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for TopDownDPStEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPStEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", &self.memo_table)
                .finish()
        }
    }

    impl Display for TopDownDPStEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPStEph(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
