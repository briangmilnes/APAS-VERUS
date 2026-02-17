//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Bottom-Up Dynamic Programming - Persistent Single-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! diagonal pebbling strategy for efficient computation of DP tables.

pub mod BottomUpDPStPer {

    use std::cmp::{max, min};
    use std::fmt::{Formatter, Debug, Display};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    /// Trait for bottom-up dynamic programming operations
    pub trait BottomUpDPStPerTrait<T: StT> {
        /// Create new bottom-up DP solver
        /// APAS: Work Θ(1), Span Θ(1)
        fn new()                     -> Self;

        /// Solve DP problem
        /// APAS: Work O(n³), Span O(n³)
        fn solve(&self, input: &[T]) -> T;
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BottomUpDPStPerS {
        /// Input sequence S
        seq_s: ArraySeqStPerS<char>,
        /// Input sequence T  
        seq_t: ArraySeqStPerS<char>,
    }

    impl BottomUpDPStPerS {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> Self { BottomUpDPStPerS { seq_s: s, seq_t: t } }

        /// Compute minimum edit distance using bottom-up diagonal pebbling
        /// claude-4-sonet: Work Θ(|S|×|T|), Span Θ(|S|+|T|), Parallelism Θ(1)
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

        /// Initialize base cases for DP table
        /// claude-4-sonet: Work Θ(|S|+|T|), Span Θ(|S|+|T|)
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

        /// Compute one diagonal of the DP table
        /// Claude Work: O(min(|S|,|T|)) - diagonal length
        /// Claude Span: O(min(|S|,|T|)) - sequential diagonal computation
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

        /// Compute value for a single DP table cell
        /// Claude Work: O(1) - constant time per cell
        /// Claude Span: O(1) - constant time per cell
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

        /// Get the length of sequence S
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn s_length(&self) -> usize { self.seq_s.length() }

        /// Get the length of sequence T
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn t_length(&self) -> usize { self.seq_t.length() }

        /// Check if sequences are empty
        /// Claude Work: O(1) - constant time check
        /// Claude Span: O(1) - constant time check
        pub fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }
    }

    impl Default for BottomUpDPStPerS {
        fn default() -> Self {
            let empty_s = ArraySeqStPerS::new(0, ' ');
            let empty_t = ArraySeqStPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
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
