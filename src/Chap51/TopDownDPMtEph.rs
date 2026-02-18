//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Ephemeral Multi-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using concurrent HashMap with in-place mutations for thread-safe subproblem caching.

pub mod TopDownDPMtEph {

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
    use std::sync::{Arc, Mutex};
    use std::thread;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone)]
    pub struct TopDownDPMtEphS {
        /// Input sequence S
        pub seq_s: ArraySeqMtEphS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqMtEphS<char>,
        /// Concurrent memoization table for subproblem results
        pub memo_table: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    // 8. traits
    /// Trait for top-down dynamic programming operations
    pub trait TopDownDPMtEphTrait<T: MtVal> : Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn new()                     -> Self;

        /// - APAS: Work O(|S|×|T|), Span O(|S|×|T|) — inherently sequential (memo threading).
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential despite concurrent memo table.
        fn solve(&self, input: &[T]) -> T;
    }

    // 9. impls
    impl TopDownDPMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> Self {
            TopDownDPMtEphS {
                seq_s: s,
                seq_t: t,
                memo_table: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Compute minimum edit distance using concurrent top-down memoization (Algorithm 51.4).
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — inherently sequential (memo threading).
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|×|T|) — sequential recursive calls despite concurrent memo.
        pub fn med_memoized_concurrent(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            self.med_recursive_concurrent(s_len, t_len)
        }

        /// Recursive MED with concurrent memoization.
        /// - APAS: Work Θ(1) amortized per call, Θ(|S|×|T|) total; Span Θ(|S|×|T|).
        /// - Claude-Opus-4.6: Work Θ(1) amortized, Span Θ(|S|×|T|) — sequential recursive calls.
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
                    let s_char = self.seq_s.nth(i - 1).clone();
                    let t_char = self.seq_t.nth(j - 1).clone();

                    if s_char == t_char {
                        // Characters match: no edit needed
                        self.med_recursive_concurrent(i - 1, j - 1)
                    } else {
                        // Characters don't match: insert or delete (APAS Algorithm 51.4)
                        let insert_cost = 1 + self.med_recursive_concurrent(i, j - 1);
                        let delete_cost = 1 + self.med_recursive_concurrent(i - 1, j);

                        insert_cost.min(delete_cost)
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

        /// Compute minimum edit distance with parallel subproblem exploration.
        /// - APAS: (no cost stated) — prose says top-down is inherently sequential.
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — parallel: thread::spawn per recursive branch.
        pub fn med_memoized_parallel(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            self.med_recursive_parallel(s_len, t_len)
        }

        /// Recursive MED with parallel branch exploration.
        /// - APAS: (no cost stated) — prose says top-down is inherently sequential.
        /// - Claude-Opus-4.6: Work Θ(1) amortized per call, Span Θ(|S|+|T|) — parallel recursive branches via thread::spawn.
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
                    let s_char = self.seq_s.nth(i - 1).clone();
                    let t_char = self.seq_t.nth(j - 1).clone();

                    if s_char == t_char {
                        // Characters match: no edit needed
                        self.med_recursive_parallel(i - 1, j - 1)
                    } else {
                        // Characters don't match: insert or delete in parallel (APAS Algorithm 51.4)
                        let self_clone1 = self.clone();
                        let self_clone2 = self.clone();

                        let handle1 = thread::spawn(move || 1 + self_clone1.med_recursive_parallel(i, j - 1));

                        let handle2 = thread::spawn(move || 1 + self_clone2.med_recursive_parallel(i - 1, j));

                        let insert_cost = handle1.join().unwrap();
                        let delete_cost = handle2.join().unwrap();

                        insert_cost.min(delete_cost)
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

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn memo_size(&self) -> usize {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.len()
        }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn is_memoized(&self, i: usize, j: usize) -> bool {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.contains_key(&(i, j))
        }

        /// - APAS: N/A — accessor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn get_memoized(&self, i: usize, j: usize) -> Option<usize> {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.get(&(i, j)).copied()
        }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn insert_memo(&mut self, i: usize, j: usize, value: usize) {
            let mut memo_guard = self.memo_table.lock().unwrap();
            memo_guard.insert((i, j), value);
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

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn clear_memo(&mut self) {
            let mut memo_guard = self.memo_table.lock().unwrap();
            memo_guard.clear();
        }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn set_s(&mut self, s: ArraySeqMtEphS<char>) {
            self.seq_s = s;
            self.clear_memo();
        }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn set_t(&mut self, t: ArraySeqMtEphS<char>) {
            self.seq_t = t;
            self.clear_memo();
        }
    }

    impl TopDownDPMtEphTrait<usize> for TopDownDPMtEphS {
        fn new() -> Self { Self::default() }

        fn solve(&self, _input: &[usize]) -> usize {
            let mut clone = self.clone();
            clone.med_memoized_concurrent()
        }
    }

    // 11. derive impls
    impl PartialEq for TopDownDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn eq(&self, other: &Self) -> bool {
            let self_memo = self.memo_table.lock().unwrap();
            let other_memo = other.memo_table.lock().unwrap();
            self.seq_s == other.seq_s && self.seq_t == other.seq_t && *self_memo == *other_memo
        }
    }

    impl Default for TopDownDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn default() -> Self {
            let empty_s = ArraySeqMtEphS::new(0, ' ');
            let empty_t = ArraySeqMtEphS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for TopDownDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPMtEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", &self.memo_table)
                .finish()
        }
    }

    impl Display for TopDownDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPMtEph(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
