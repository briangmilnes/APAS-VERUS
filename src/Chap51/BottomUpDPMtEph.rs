//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Bottom-Up Dynamic Programming - Ephemeral Multi-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! parallel diagonal pebbling with in-place mutations for multi-threaded computation.

pub mod BottomUpDPMtEph {

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
    use std::sync::{Arc, Mutex};
    use std::thread;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    verus! {
    } // verus!

    // 4. type definitions
    #[derive(Clone, PartialEq, Eq)]
    pub struct BottomUpDPMtEphS {
        /// Input sequence S
        pub seq_s: ArraySeqMtEphS<char>,
        /// Input sequence T
        pub seq_t: ArraySeqMtEphS<char>,
    }

    // 8. traits
    /// Trait for bottom-up dynamic programming operations
    pub trait BottomUpDPMtEphTrait<T: MtVal> : Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn new()                     -> Self;

        /// - APAS: Work O(|S|×|T|), Span O(|S|+|T|) — diagonal parallelism.
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — agrees with APAS; diagonals processed in parallel.
        fn solve(&self, input: &[T]) -> T;
    }

    // 9. impls
    impl BottomUpDPMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> Self { BottomUpDPMtEphS { seq_s: s, seq_t: t } }

        /// Compute minimum edit distance using parallel bottom-up diagonal pebbling (Algorithm 51.1).
        /// - APAS: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — diagonal parallelism.
        /// - Claude-Opus-4.6: Work Θ(|S|×|T|), Span Θ(|S|+|T|) — parallel: thread::spawn per diagonal element.
        pub fn med_bottom_up_parallel(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            // Create shared mutable DP table for parallel computation
            let table = Arc::new(Mutex::new(self.initialize_base_cases()));

            // Process diagonals with parallel computation within each diagonal
            for k in 1..=(s_len + t_len) {
                self.compute_diagonal_parallel(Arc::clone(&table), k);
            }

            // Extract result from bottom-right corner
            let final_table = table.lock().unwrap();
            final_table[s_len][t_len]
        }

        /// Initialize base cases for DP table.
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|), Span Θ(|S|+|T|)
        fn initialize_base_cases(&self) -> Vec<Vec<usize>> {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            // Initialize with zeros - using Vec for 2D table as Mt sequences lack nested mutation
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

        /// Compute one diagonal of the DP table in parallel.
        /// - APAS: Work Θ(diagonal length), Span Θ(1) — each element computed in parallel.
        /// - Claude-Opus-4.6: Work Θ(min(|S|,|T|)), Span Θ(1) — agrees with APAS; thread::spawn per element.
        fn compute_diagonal_parallel(&self, table: Arc<Mutex<Vec<Vec<usize>>>>, k: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let start = max(1, k.saturating_sub(t_len));
            let end = min(k, s_len);

            // Collect diagonal positions
            let positions = (start..=end)
                .filter_map(|i| {
                    let j = k - i;
                    if j > 0 && j <= t_len { Some((i, j)) } else { None }
                }).collect::<Vec<(usize, usize)>>();

            // Process diagonal elements in parallel
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

            // Collect results from all threads FIRST (without holding lock)
            let results = handles.into_iter().map(|handle| handle.join().unwrap()).collect::<Vec<(usize, usize, usize)>>();

            // Then acquire lock once and write all results in-place
            let mut table_guard = table.lock().unwrap();
            for (i, j, new_value) in results {
                table_guard[i][j] = new_value;
            }
        }

        /// Compute value for a single DP table cell (medOne from Algorithm 51.1).
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn compute_cell_value_static(
            seq_s: &ArraySeqMtEphS<char>,
            seq_t: &ArraySeqMtEphS<char>,
            table: &Arc<Mutex<Vec<Vec<usize>>>>,
            i: usize,
            j: usize,
        ) -> usize {
            // Access sequences using nth_cloned as nth() not available for MtEph
            let s_char = seq_s.nth(i - 1).clone();
            let t_char = seq_t.nth(j - 1).clone();

            let table_guard = table.lock().unwrap();

            if s_char == t_char {
                // Characters match: take diagonal value
                table_guard[i - 1][j - 1]
            } else {
                // Characters don't match: take minimum of insert/delete + 1
                let delete_cost = table_guard[i - 1][j];
                let insert_cost = table_guard[i][j - 1];
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

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn set_s(&mut self, s: ArraySeqMtEphS<char>) { self.seq_s = s; }

        /// - APAS: N/A — mutator.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn set_t(&mut self, t: ArraySeqMtEphS<char>) { self.seq_t = t; }
    }

    impl BottomUpDPMtEphTrait<usize> for BottomUpDPMtEphS {
        fn new() -> Self { Self::default() }

        fn solve(&self, _input: &[usize]) -> usize {
            let mut clone = self.clone();
            clone.med_bottom_up_parallel()
        }
    }

    // 11. derive impls
    impl Default for BottomUpDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn default() -> Self {
            let empty_s = ArraySeqMtEphS::new(0, ' ');
            let empty_t = ArraySeqMtEphS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    // 13. derive impls outside verus!
    impl Debug for BottomUpDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(|S|+|T|), Span Θ(|S|+|T|)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPMtEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for BottomUpDPMtEphS {
        /// - APAS: N/A — infrastructure.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "BottomUpDPMtEph(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
