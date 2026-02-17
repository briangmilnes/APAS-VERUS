//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Persistent Multi-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using concurrent HashMap for thread-safe subproblem caching.

pub mod TopDownDPMtPer {

    use std::collections::HashMap;
    use std::fmt::{Formatter, Debug, Display};
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    /// Trait for top-down dynamic programming operations
    pub trait TopDownDPMtPerTrait<T: MtVal> {
        /// Create new top-down DP solver
        /// APAS: Work Θ(1), Span Θ(1)
        fn new()                     -> Self;

        /// Solve DP problem with memoization
        /// APAS: Work O(n²), Span O(lg n)
        fn solve(&self, input: &[T]) -> T;
    }

    #[derive(Clone, Debug)]
    pub struct TopDownDPMtPerS {
        /// Input sequence S
        seq_s: ArraySeqMtPerS<char>,
        /// Input sequence T  
        seq_t: ArraySeqMtPerS<char>,
        /// Concurrent memoization table for subproblem results
        memo_table: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    impl TopDownDPMtPerS {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> Self {
            TopDownDPMtPerS {
                seq_s: s,
                seq_t: t,
                memo_table: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Compute minimum edit distance using concurrent top-down memoization
        /// claude-4-sonet: Work Θ(|S|×|T|), Span Θ(|S|+|T|), Parallelism Θ(min(|S|,|T|))
        pub fn med_memoized_concurrent(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            self.med_recursive_concurrent(s_len, t_len)
        }

        /// claude-4-sonet: Work Θ(1) amortized per call, Θ(|S|×|T|) total; Span Θ(|S|+|T|)
        fn med_recursive_concurrent(&self, i: usize, j: usize) -> usize {
            // Check memo table first
            {
                let memo_guard = self.memo_table.lock().unwrap();
                if let Some(&cached_result) = memo_guard.get(&(i, j)) {
                    return cached_result;
                }
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
                        self.med_recursive_concurrent(i - 1, j - 1)
                    } else {
                        // Characters don't match: try insert, delete, or substitute
                        // For better parallelism, we could spawn threads for each branch
                        // but dependencies limit effectiveness
                        let insert_cost = 1 + self.med_recursive_concurrent(i, j - 1);
                        let delete_cost = 1 + self.med_recursive_concurrent(i - 1, j);
                        let substitute_cost = 1 + self.med_recursive_concurrent(i - 1, j - 1);

                        insert_cost.min(delete_cost).min(substitute_cost)
                    }
                }
            };

            // Store result in memo table
            {
                let mut memo_guard = self.memo_table.lock().unwrap();
                memo_guard.insert((i, j), result);
            }

            result
        }

        /// Compute minimum edit distance with parallel subproblem exploration
        /// Claude Work: O(|S|*|T|) where |S|=source length, |T|=target length
        /// Claude Span: O(log(|S|+|T|)) with aggressive parallelism
        pub fn med_memoized_parallel(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            self.med_recursive_parallel(s_len, t_len)
        }

        /// Claude Work: O(1) per call with memoization, O(|S|*|T|) total
        /// Claude Span: O(log(|S|+|T|)) - parallel recursive branches
        fn med_recursive_parallel(&self, i: usize, j: usize) -> usize {
            // Check memo table first
            {
                let memo_guard = self.memo_table.lock().unwrap();
                if let Some(&cached_result) = memo_guard.get(&(i, j)) {
                    return cached_result;
                }
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
                        self.med_recursive_parallel(i - 1, j - 1)
                    } else {
                        // Characters don't match: explore branches in parallel
                        let self_clone1 = self.clone();
                        let self_clone2 = self.clone();
                        let self_clone3 = self.clone();

                        let handle1 = thread::spawn(move || 1 + self_clone1.med_recursive_parallel(i, j - 1));

                        let handle2 = thread::spawn(move || 1 + self_clone2.med_recursive_parallel(i - 1, j));

                        let handle3 = thread::spawn(move || 1 + self_clone3.med_recursive_parallel(i - 1, j - 1));

                        let insert_cost = handle1.join().unwrap();
                        let delete_cost = handle2.join().unwrap();
                        let substitute_cost = handle3.join().unwrap();

                        insert_cost.min(delete_cost).min(substitute_cost)
                    }
                }
            };

            // Store result in memo table
            {
                let mut memo_guard = self.memo_table.lock().unwrap();
                memo_guard.insert((i, j), result);
            }

            result
        }

        /// Create new instance with updated memoization table
        /// Claude Work: O(1) - constant time update
        /// Claude Span: O(1) - constant time update
        pub fn with_memo_table(self, memo: HashMap<(usize, usize), usize>) -> Self {
            TopDownDPMtPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: Arc::new(Mutex::new(memo)),
            }
        }

        /// Get current memoization table size
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn memo_size(&self) -> usize {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.len()
        }

        /// Check if subproblem is memoized
        /// Claude Work: O(1) - constant time lookup
        /// Claude Span: O(1) - constant time lookup
        pub fn is_memoized(&self, i: usize, j: usize) -> bool {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.contains_key(&(i, j))
        }

        /// Get memoized result if available
        /// Claude Work: O(1) - constant time lookup
        /// Claude Span: O(1) - constant time lookup
        pub fn get_memoized(&self, i: usize, j: usize) -> Option<usize> {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.get(&(i, j)).copied()
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

        /// Clear memoization table
        /// Claude Work: O(1) - constant time clear
        /// Claude Span: O(1) - constant time clear
        pub fn clear_memo(self) -> Self {
            TopDownDPMtPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl PartialEq for TopDownDPMtPerS {
        fn eq(&self, other: &Self) -> bool {
            let self_memo = self.memo_table.lock().unwrap();
            let other_memo = other.memo_table.lock().unwrap();
            self.seq_s == other.seq_s && self.seq_t == other.seq_t && *self_memo == *other_memo
        }
    }

    impl Default for TopDownDPMtPerS {
        fn default() -> Self {
            let empty_s = ArraySeqMtPerS::new(0, ' ');
            let empty_t = ArraySeqMtPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    impl Display for TopDownDPMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPMtPer(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
