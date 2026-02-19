//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Bottom-Up Dynamic Programming - Ephemeral Single-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! diagonal pebbling strategy with in-place mutations for efficiency.

pub mod BottomUpDPStEph {

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

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    pub struct BottomUpDPStEphS {
        pub seq_s: ArraySeqStEphS<char>,
        pub seq_t: ArraySeqStEphS<char>,
    }

    } // verus!

    // 8. traits

    pub trait BottomUpDPStEphTrait: Sized {
        fn new(s: ArraySeqStEphS<char>, t: ArraySeqStEphS<char>) -> Self;
        fn s_length(&self) -> usize;
        fn t_length(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn set_s(&mut self, s: ArraySeqStEphS<char>);
        fn set_t(&mut self, t: ArraySeqStEphS<char>);
        fn med_bottom_up(&mut self) -> usize;
    }

    // 9. impls

    impl BottomUpDPStEphTrait for BottomUpDPStEphS {
        fn new(s: ArraySeqStEphS<char>, t: ArraySeqStEphS<char>) -> Self {
            BottomUpDPStEphS { seq_s: s, seq_t: t }
        }

        fn s_length(&self) -> usize { self.seq_s.length() }
        fn t_length(&self) -> usize { self.seq_t.length() }
        fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }
        fn set_s(&mut self, s: ArraySeqStEphS<char>) { self.seq_s = s; }
        fn set_t(&mut self, t: ArraySeqStEphS<char>) { self.seq_t = t; }

        /// Compute MED using bottom-up diagonal pebbling (Algorithm 51.1).
        fn med_bottom_up(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let mut table = self.initialize_base_cases();

            for k in 1..=(s_len + t_len) {
                self.compute_diagonal(&mut table, k);
            }

            table[s_len][t_len]
        }
    }

    impl BottomUpDPStEphS {
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

        fn compute_diagonal(&self, table: &mut [Vec<usize>], k: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let start = max(1, k.saturating_sub(t_len));
            let end = min(k, s_len);

            for i in start..=end {
                let j = k - i;
                if j > 0 && j <= t_len {
                    let new_value = self.compute_cell_value(table, i, j);
                    table[i][j] = new_value;
                }
            }
        }

        fn compute_cell_value(&self, table: &[Vec<usize>], i: usize, j: usize) -> usize {
            let s_char = *self.seq_s.nth(i - 1);
            let t_char = *self.seq_t.nth(j - 1);

            if s_char == t_char {
                table[i - 1][j - 1]
            } else {
                let delete_cost = table[i - 1][j];
                let insert_cost = table[i][j - 1];
                1 + min(delete_cost, insert_cost)
            }
        }
    }

    // 11. derive impls
    impl Clone for BottomUpDPStEphS {
        fn clone(&self) -> Self {
            BottomUpDPStEphS { seq_s: self.seq_s.clone(), seq_t: self.seq_t.clone() }
        }
    }

    impl PartialEq for BottomUpDPStEphS {
        fn eq(&self, other: &Self) -> bool {
            self.seq_s == other.seq_s && self.seq_t == other.seq_t
        }
    }

    impl Eq for BottomUpDPStEphS {}

    impl Default for BottomUpDPStEphS {
        fn default() -> Self {
            let empty_s = ArraySeqStEphS::new(0, ' ');
            let empty_t = ArraySeqStEphS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for BottomUpDPStEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPStEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for BottomUpDPStEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "BottomUpDPStEph(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
