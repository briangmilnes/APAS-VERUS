//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Bottom-Up Dynamic Programming - Persistent Multi-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! parallel diagonal pebbling for multi-threaded computation.

pub mod BottomUpDPMtPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 9. impls
    // 8. traits
    // 11. derive impls
    // 13. derive impls outside verus!

    // 2. imports
    use std::cmp::{max, min};
    use std::fmt::{Formatter, Debug, Display};
    use std::sync::Arc;
    use vstd::rwlock::*;
    use std::thread;

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    pub struct BottomUpDPMtPerS {
        pub seq_s: ArraySeqMtPerS<char>,
        pub seq_t: ArraySeqMtPerS<char>,
    }

    pub struct BottomUpDPMtPerInv;
    impl RwLockPredicate<Vec<Vec<usize>>> for BottomUpDPMtPerInv {
        open spec fn inv(self, v: Vec<Vec<usize>>) -> bool { true }
    }

    #[verifier::external_body]
    fn new_bu_per_lock(val: Vec<Vec<usize>>) -> (lock: RwLock<Vec<Vec<usize>>, BottomUpDPMtPerInv>) {
        RwLock::new(val, Ghost(BottomUpDPMtPerInv))
    }

    } // verus!

    // 8. traits

    pub trait BottomUpDPMtPerTrait: Sized {
        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> Self;
        fn s_length(&self) -> usize;
        fn t_length(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn med_bottom_up_parallel(&self) -> usize;
        fn initialize_base_cases(&self) -> Vec<Vec<usize>>;
        fn compute_diagonal_parallel(&self, table: Arc<RwLock<Vec<Vec<usize>>, BottomUpDPMtPerInv>>, k: usize);
        fn compute_cell_value_static(
            seq_s: &ArraySeqMtPerS<char>,
            seq_t: &ArraySeqMtPerS<char>,
            table: &Arc<RwLock<Vec<Vec<usize>>, BottomUpDPMtPerInv>>,
            i: usize,
            j: usize,
        ) -> usize;
    }

    // 9. impls

    impl BottomUpDPMtPerTrait for BottomUpDPMtPerS {
        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> Self {
            BottomUpDPMtPerS { seq_s: s, seq_t: t }
        }

        fn s_length(&self) -> usize { self.seq_s.length() }
        fn t_length(&self) -> usize { self.seq_t.length() }
        fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }

        /// Compute MED using parallel bottom-up diagonal pebbling (Algorithm 51.1).
        fn med_bottom_up_parallel(&self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let table = Arc::new(new_bu_per_lock(self.initialize_base_cases()));

            for k in 1..=(s_len + t_len) {
                self.compute_diagonal_parallel(Arc::clone(&table), k);
            }

            let read_handle = table.acquire_read();
            let result = read_handle.borrow()[s_len][t_len];
            read_handle.release_read();
            result
        }

        fn initialize_base_cases(&self) -> Vec<Vec<usize>> {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let mut table = vec![vec![0usize; t_len + 1]; s_len + 1];

            for (i, row) in table.iter_mut().enumerate().take(s_len + 1) {
                row[0] = i;
            }
            for (j, cell) in table[0].iter_mut().enumerate().take(t_len + 1) {
                *cell = j;
            }

            table
        }

        fn compute_diagonal_parallel(&self, table: Arc<RwLock<Vec<Vec<usize>>, BottomUpDPMtPerInv>>, k: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let start = max(1, k.saturating_sub(t_len));
            let end = min(k, s_len);

            let positions = (start..=end)
                .filter_map(|i| {
                    let j = k - i;
                    if j > 0 && j <= t_len { Some((i, j)) } else { None }
                }).collect::<Vec<(usize, usize)>>();

            let handles = positions
                .into_iter()
                .map(|(i, j)| {
                    let table_clone = Arc::clone(&table);
                    let seq_s_clone = self.seq_s.clone();
                    let seq_t_clone = self.seq_t.clone();

                    thread::spawn(move || {
                        let new_value = Self::compute_cell_value_static(&seq_s_clone, &seq_t_clone, &table_clone, i, j);
                        (i, j, new_value)
                    })
                }).collect::<Vec<_>>();

            let results = handles.into_iter().map(|handle| handle.join().unwrap()).collect::<Vec<(usize, usize, usize)>>();

            let (mut current, write_handle) = table.acquire_write();
            for (i, j, new_value) in results {
                current[i][j] = new_value;
            }
            write_handle.release_write(current);
        }

        fn compute_cell_value_static(
            seq_s: &ArraySeqMtPerS<char>,
            seq_t: &ArraySeqMtPerS<char>,
            table: &Arc<RwLock<Vec<Vec<usize>>, BottomUpDPMtPerInv>>,
            i: usize,
            j: usize,
        ) -> usize {
            let s_char = *seq_s.nth(i - 1);
            let t_char = *seq_t.nth(j - 1);

            let read_handle = table.acquire_read();
            let result = if s_char == t_char {
                read_handle.borrow()[i - 1][j - 1]
            } else {
                let delete_cost = read_handle.borrow()[i - 1][j];
                let insert_cost = read_handle.borrow()[i][j - 1];
                1 + min(delete_cost, insert_cost)
            };
            read_handle.release_read();
            result
        }
    }

    // 11. derive impls
    impl Clone for BottomUpDPMtPerS {
        fn clone(&self) -> Self {
            BottomUpDPMtPerS { seq_s: self.seq_s.clone(), seq_t: self.seq_t.clone() }
        }
    }

    impl PartialEq for BottomUpDPMtPerS {
        fn eq(&self, other: &Self) -> bool {
            self.seq_s == other.seq_s && self.seq_t == other.seq_t
        }
    }

    impl Eq for BottomUpDPMtPerS {}

    impl Default for BottomUpDPMtPerS {
        fn default() -> Self {
            let empty_s = ArraySeqMtPerS::new(0, ' ');
            let empty_t = ArraySeqMtPerS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for BottomUpDPMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPMtPerS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for BottomUpDPMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "BottomUpDPMtPer(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
