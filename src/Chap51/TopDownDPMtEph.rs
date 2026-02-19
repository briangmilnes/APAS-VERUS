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
    use std::sync::Arc;
    use vstd::rwlock::*;
    use std::thread;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    verus! {

    pub struct TopDownDPMtEphWf;
    impl RwLockPredicate<HashMap<(usize, usize), usize>> for TopDownDPMtEphWf {
        open spec fn inv(self, v: HashMap<(usize, usize), usize>) -> bool { true }
    }

    #[verifier::external_body]
    fn new_td_eph_lock(val: HashMap<(usize, usize), usize>) -> (lock: RwLock<HashMap<(usize, usize), usize>, TopDownDPMtEphWf>) {
        RwLock::new(val, Ghost(TopDownDPMtEphWf))
    }

    } // verus!

    // 4. type definitions
    #[derive(Clone)]
    pub struct TopDownDPMtEphS {
        /// Input sequence S
        pub seq_s: ArraySeqMtEphS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqMtEphS<char>,
        /// Concurrent memoization table for subproblem results
        pub memo_table: Arc<RwLock<HashMap<(usize, usize), usize>, TopDownDPMtEphWf>>,
    }

    // 8. traits

    pub trait TopDownDPMtEphTrait: Sized {
        fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> Self;
        fn med_memoized_concurrent(&mut self) -> usize;
        fn med_memoized_parallel(&mut self) -> usize;
        fn memo_size(&self) -> usize;
        fn is_memoized(&self, i: usize, j: usize) -> bool;
        fn get_memoized(&self, i: usize, j: usize) -> Option<usize>;
        fn insert_memo(&mut self, i: usize, j: usize, value: usize);
        fn s_length(&self) -> usize;
        fn t_length(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn clear_memo(&mut self);
        fn set_s(&mut self, s: ArraySeqMtEphS<char>);
        fn set_t(&mut self, t: ArraySeqMtEphS<char>);
        fn med_recursive_concurrent(&self, i: usize, j: usize) -> usize;
        fn med_recursive_parallel(&self, i: usize, j: usize) -> usize;
    }

    // 9. impls

    impl TopDownDPMtEphTrait for TopDownDPMtEphS {
        fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> Self {
            TopDownDPMtEphS {
                seq_s: s,
                seq_t: t,
                memo_table: Arc::new(new_td_eph_lock(HashMap::new())),
            }
        }

        /// Compute MED using concurrent top-down memoization (Algorithm 51.4).
        fn med_memoized_concurrent(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            self.med_recursive_concurrent(s_len, t_len)
        }

        /// Compute MED with parallel subproblem exploration.
        fn med_memoized_parallel(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            self.med_recursive_parallel(s_len, t_len)
        }

        fn memo_size(&self) -> usize {
            let read_handle = self.memo_table.acquire_read();
            let len = read_handle.borrow().len();
            read_handle.release_read();
            len
        }

        fn is_memoized(&self, i: usize, j: usize) -> bool {
            let read_handle = self.memo_table.acquire_read();
            let result = read_handle.borrow().contains_key(&(i, j));
            read_handle.release_read();
            result
        }

        fn get_memoized(&self, i: usize, j: usize) -> Option<usize> {
            let read_handle = self.memo_table.acquire_read();
            let result = read_handle.borrow().get(&(i, j)).copied();
            read_handle.release_read();
            result
        }

        fn insert_memo(&mut self, i: usize, j: usize, value: usize) {
            let (mut current, write_handle) = self.memo_table.acquire_write();
            current.insert((i, j), value);
            write_handle.release_write(current);
        }

        fn s_length(&self) -> usize { self.seq_s.length() }
        fn t_length(&self) -> usize { self.seq_t.length() }
        fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }

        fn clear_memo(&mut self) {
            let (mut current, write_handle) = self.memo_table.acquire_write();
            current.clear();
            write_handle.release_write(current);
        }

        fn set_s(&mut self, s: ArraySeqMtEphS<char>) {
            self.seq_s = s;
            self.clear_memo();
        }

        fn set_t(&mut self, t: ArraySeqMtEphS<char>) {
            self.seq_t = t;
            self.clear_memo();
        }

        fn med_recursive_concurrent(&self, i: usize, j: usize) -> usize {
            {
                let read_handle = self.memo_table.acquire_read();
                let cached = read_handle.borrow().get(&(i, j)).copied();
                read_handle.release_read();
                if let Some(cached_result) = cached {
                    return cached_result;
                }
            }

            let result = match (i, j) {
                | (0, j) => j,
                | (i, 0) => i,
                | (i, j) => {
                    let s_char = self.seq_s.nth(i - 1).clone();
                    let t_char = self.seq_t.nth(j - 1).clone();

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
                let (mut current, write_handle) = self.memo_table.acquire_write();
                current.insert((i, j), result);
                write_handle.release_write(current);
            }
            result
        }

        fn med_recursive_parallel(&self, i: usize, j: usize) -> usize {
            {
                let read_handle = self.memo_table.acquire_read();
                let cached = read_handle.borrow().get(&(i, j)).copied();
                read_handle.release_read();
                if let Some(cached_result) = cached {
                    return cached_result;
                }
            }

            let result = match (i, j) {
                | (0, j) => j,
                | (i, 0) => i,
                | (i, j) => {
                    let s_char = self.seq_s.nth(i - 1).clone();
                    let t_char = self.seq_t.nth(j - 1).clone();

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
                let (mut current, write_handle) = self.memo_table.acquire_write();
                current.insert((i, j), result);
                write_handle.release_write(current);
            }
            result
        }
    }

    // 11. derive impls
    impl PartialEq for TopDownDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|+|memo|), Span Θ(|S|+|T|+|memo|)
        fn eq(&self, other: &Self) -> bool {
            let self_handle = self.memo_table.acquire_read();
            let other_handle = other.memo_table.acquire_read();
            let result = self.seq_s == other.seq_s && self.seq_t == other.seq_t
                && *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            result
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
            let read_handle = self.memo_table.acquire_read();
            let result = f.debug_struct("TopDownDPMtEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", read_handle.borrow())
                .finish();
            read_handle.release_read();
            result
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
