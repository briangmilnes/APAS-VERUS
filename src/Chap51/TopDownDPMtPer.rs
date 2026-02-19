//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Persistent Multi-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using concurrent HashMap for thread-safe subproblem caching.

pub mod TopDownDPMtPer {

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
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone)]
    pub struct TopDownDPMtPerS {
        /// Input sequence S
        pub seq_s: ArraySeqMtPerS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqMtPerS<char>,
        /// Concurrent memoization table for subproblem results
        pub memo_table: Arc<Mutex<HashMap<(usize, usize), usize>>>,
    }

    // 8. traits

    pub trait TopDownDPMtPerTrait: Sized {
        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> Self;
        fn med_memoized_concurrent(&self) -> usize;
        fn med_memoized_parallel(&self) -> usize;
        fn with_memo_table(self, memo: HashMap<(usize, usize), usize>) -> Self;
        fn memo_size(&self) -> usize;
        fn is_memoized(&self, i: usize, j: usize) -> bool;
        fn get_memoized(&self, i: usize, j: usize) -> Option<usize>;
        fn s_length(&self) -> usize;
        fn t_length(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn clear_memo(self) -> Self;
        fn med_recursive_concurrent(&self, i: usize, j: usize) -> usize;
        fn med_recursive_parallel(&self, i: usize, j: usize) -> usize;
    }

    // 9. impls

    impl TopDownDPMtPerTrait for TopDownDPMtPerS {
        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> Self {
            TopDownDPMtPerS {
                seq_s: s,
                seq_t: t,
                memo_table: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Compute MED using concurrent top-down memoization (Algorithm 51.4).
        fn med_memoized_concurrent(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            self.med_recursive_concurrent(s_len, t_len)
        }

        /// Compute MED with parallel subproblem exploration.
        fn med_memoized_parallel(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            self.med_recursive_parallel(s_len, t_len)
        }

        fn with_memo_table(self, memo: HashMap<(usize, usize), usize>) -> Self {
            TopDownDPMtPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: Arc::new(Mutex::new(memo)),
            }
        }

        fn memo_size(&self) -> usize {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.len()
        }

        fn is_memoized(&self, i: usize, j: usize) -> bool {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.contains_key(&(i, j))
        }

        fn get_memoized(&self, i: usize, j: usize) -> Option<usize> {
            let memo_guard = self.memo_table.lock().unwrap();
            memo_guard.get(&(i, j)).copied()
        }

        fn s_length(&self) -> usize { self.seq_s.length() }
        fn t_length(&self) -> usize { self.seq_t.length() }
        fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }

        fn clear_memo(self) -> Self {
            TopDownDPMtPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn med_recursive_concurrent(&self, i: usize, j: usize) -> usize {
            {
                let memo_guard = self.memo_table.lock().unwrap();
                if let Some(&cached_result) = memo_guard.get(&(i, j)) {
                    return cached_result;
                }
            }

            let result = match (i, j) {
                | (0, j) => j,
                | (i, 0) => i,
                | (i, j) => {
                    let s_char = *self.seq_s.nth(i - 1);
                    let t_char = *self.seq_t.nth(j - 1);

                    if s_char == t_char {
                        self.med_recursive_concurrent(i - 1, j - 1)
                    } else {
                        let insert_cost = 1 + self.med_recursive_concurrent(i, j - 1);
                        let delete_cost = 1 + self.med_recursive_concurrent(i - 1, j);
                        insert_cost.min(delete_cost)
                    }
                }
            };

            {
                let mut memo_guard = self.memo_table.lock().unwrap();
                memo_guard.insert((i, j), result);
            }
            result
        }

        fn med_recursive_parallel(&self, i: usize, j: usize) -> usize {
            {
                let memo_guard = self.memo_table.lock().unwrap();
                if let Some(&cached_result) = memo_guard.get(&(i, j)) {
                    return cached_result;
                }
            }

            let result = match (i, j) {
                | (0, j) => j,
                | (i, 0) => i,
                | (i, j) => {
                    let s_char = *self.seq_s.nth(i - 1);
                    let t_char = *self.seq_t.nth(j - 1);

                    if s_char == t_char {
                        self.med_recursive_parallel(i - 1, j - 1)
                    } else {
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

            {
                let mut memo_guard = self.memo_table.lock().unwrap();
                memo_guard.insert((i, j), result);
            }
            result
        }
    }

    // 11. derive impls
    impl PartialEq for TopDownDPMtPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn eq(&self, other: &Self) -> bool {
            let self_memo = self.memo_table.lock().unwrap();
            let other_memo = other.memo_table.lock().unwrap();
            self.seq_s == other.seq_s && self.seq_t == other.seq_t && *self_memo == *other_memo
        }
    }

    impl Default for TopDownDPMtPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn default() -> Self {
            let empty_s = ArraySeqMtPerS::new(0, ' ');
            let empty_t = ArraySeqMtPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for TopDownDPMtPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPMtPerS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", &self.memo_table)
                .finish()
        }
    }

    impl Display for TopDownDPMtPerS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
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
