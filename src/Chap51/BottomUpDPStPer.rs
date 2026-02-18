//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Bottom-Up Dynamic Programming - Persistent Single-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! diagonal pebbling strategy for efficient computation of DP tables.

pub mod BottomUpDPStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 8. traits
    // 9. impls
    // 11. derive impls
    // 13. derive impls outside verus!

    // 2. imports
    use std::cmp::{max, min};
    use std::fmt::{Formatter, Debug, Display};

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone, PartialEq, Eq)]
    pub struct BottomUpDPStPerS {
        /// Input sequence S
        pub seq_s: ArraySeqStPerS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqStPerS<char>,
    }

    // 8. traits
    /// Trait for bottom-up dynamic programming operations
    pub trait BottomUpDPStPerTrait<T: StT> : Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn new()                     -> Self;

        /// - APAS: Work O(|S|×|T|), Span O(|S|+|T|)
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential: no parallelism within diagonals.
        fn solve(&self, input: &[T]) -> T;
    }

    // 9. impls
    impl BottomUpDPStPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> Self { BottomUpDPStPerS { seq_s: s, seq_t: t } }

        /// Compute minimum edit distance using bottom-up diagonal pebbling (Algorithm 51.1).
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — diagonal parallelism.
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential: no parallelism within diagonals.
        pub fn med_bottom_up(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            // Create mutable copy for computation
            let mut table = self.initialize_base_cases();

            // Process diagonals from top-left to bottom-right
            for k in 1..=(s_len + t_len) {
                table = self.compute_diagonal(table, k);
            }

            // Extract result from bottom-right corner
            table[s_len][t_len]
        }

        /// Initialize base cases for DP table.
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|), Span Θ(|S|+|T|)
        fn initialize_base_cases(&self) -> Vec<Vec<usize>> {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            // Initialize with zeros - using Vec for 2D table as sequences lack nested indexing
            let mut table = vec![vec![0usize; t_len + 1]; s_len + 1];

            // Set base cases: empty string transformations
            for (i, row) in table.iter_mut().enumerate().take(s_len + 1) {
                row[0] = i;
            }
            for (j, cell) in table[0].iter_mut().enumerate().take(t_len + 1) {
                *cell = j;
            }

            table
        }

        /// Compute one diagonal of the DP table.
        /// - APAS: Work Θ(diagonal length), Span Θ(1) — each element computed in parallel.
        /// - Claude-Opus-4.6: Work Θ(min(|S|,|T|)), Span Θ(min(|S|,|T|)) — sequential loop, no parallelism.
        fn compute_diagonal(&self, mut table: Vec<Vec<usize>>, k: usize) -> Vec<Vec<usize>> {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let start = max(1, k.saturating_sub(t_len));
            let end = min(k, s_len);

            for i in start..=end {
                let j = k - i;
                if j > 0 && j <= t_len {
                    let new_value = self.compute_cell_value(&table, i, j);
                    table[i][j] = new_value;
                }
            }

            table
        }

        /// Compute value for a single DP table cell (medOne from Algorithm 51.1).
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn compute_cell_value(&self, table: &[Vec<usize>], i: usize, j: usize) -> usize {
            let s_char = *self.seq_s.nth(i - 1);
            let t_char = *self.seq_t.nth(j - 1);

            if s_char == t_char {
                // Characters match: take diagonal value
                table[i - 1][j - 1]
            } else {
                // Characters don't match: take minimum of insert/delete + 1
                let delete_cost = table[i - 1][j];
                let insert_cost = table[i][j - 1];
                1 + min(delete_cost, insert_cost)
            }
        }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn s_length(&self) -> usize { self.seq_s.length() }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn t_length(&self) -> usize { self.seq_t.length() }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }
    }

    // 11. derive impls
    impl Default for BottomUpDPStPerS {
        fn default() -> Self {
            let empty_s = ArraySeqStPerS::new(0, ' ');
            let empty_t = ArraySeqStPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for BottomUpDPStPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPStPerS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for BottomUpDPStPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "BottomUpDPStPer(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
