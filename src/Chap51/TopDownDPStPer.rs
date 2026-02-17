//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Persistent Single-Threaded Implementation.
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using HashMap for efficient subproblem caching.

pub mod TopDownDPStPer {

    use std::collections::HashMap;
    use std::fmt::{Formatter, Debug, Display};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    /// Trait for top-down dynamic programming operations
    pub trait TopDownDPStPerTrait<T: StT> {
        /// Create new top-down DP solver
        /// APAS: Work Θ(1), Span Θ(1)
        fn new()                     -> Self;

        /// Solve DP problem with memoization
        /// APAS: Work O(n²), Span O(n²)
        fn solve(&self, input: &[T]) -> T;
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct TopDownDPStPerS {
        /// Input sequence S
        seq_s: ArraySeqStPerS<char>,
        /// Input sequence T  
        seq_t: ArraySeqStPerS<char>,
        /// Memoization table for subproblem results
        memo_table: HashMap<(usize, usize), usize>,
    }

    impl TopDownDPStPerS {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> Self {
            TopDownDPStPerS {
                seq_s: s,
                seq_t: t,
                memo_table: HashMap::new(),
            }
        }

        /// Compute minimum edit distance using top-down memoization
        /// claude-4-sonet: Work Θ(|S|×|T|), Span Θ(|S|+|T|), Parallelism Θ(1)
        pub fn med_memoized(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            // Create mutable copy of memo table for computation
            let mut memo = self.memo_table.clone();

            self.med_recursive(s_len, t_len, &mut memo)
        }

        /// claude-4-sonet: Work Θ(1) amortized per call, Θ(|S|×|T|) total; Span Θ(|S|+|T|)
        fn med_recursive(&self, i: usize, j: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
            // Check memo table first
            if let Some(&cached_result) = memo.get(&(i, j)) {
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
                        self.med_recursive(i - 1, j - 1, memo)
                    } else {
                        // Characters don't match: try insert, delete, or substitute
                        let insert_cost = 1 + self.med_recursive(i, j - 1, memo);
                        let delete_cost = 1 + self.med_recursive(i - 1, j, memo);
                        let substitute_cost = 1 + self.med_recursive(i - 1, j - 1, memo);

                        insert_cost.min(delete_cost).min(substitute_cost)
                    }
                }
            };

            // Store result in memo table
            memo.insert((i, j), result);
            result
        }

        /// Create new instance with updated memoization table
        /// Claude Work: O(1) - constant time update
        /// Claude Span: O(1) - constant time update
        pub fn with_memo_table(self, memo: HashMap<(usize, usize), usize>) -> Self {
            TopDownDPStPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: memo,
            }
        }

        /// Get current memoization table size
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn memo_size(&self) -> usize { self.memo_table.len() }

        /// Check if subproblem is memoized
        /// Claude Work: O(1) - constant time lookup
        /// Claude Span: O(1) - constant time lookup
        pub fn is_memoized(&self, i: usize, j: usize) -> bool { self.memo_table.contains_key(&(i, j)) }

        /// Get memoized result if available
        /// Claude Work: O(1) - constant time lookup
        /// Claude Span: O(1) - constant time lookup
        pub fn get_memoized(&self, i: usize, j: usize) -> Option<usize> { self.memo_table.get(&(i, j)).copied() }

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

        /// Clear memoization table
        /// Claude Work: O(1) - constant time clear
        /// Claude Span: O(1) - constant time clear
        pub fn clear_memo(self) -> Self {
            TopDownDPStPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: HashMap::new(),
            }
        }
    }

    impl Default for TopDownDPStPerS {
        fn default() -> Self {
            let empty_s = ArraySeqStPerS::new(0, ' ');
            let empty_t = ArraySeqStPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    impl Display for TopDownDPStPerS {
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
