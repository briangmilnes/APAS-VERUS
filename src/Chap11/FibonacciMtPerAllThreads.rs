//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci (multi-threaded, persistent).
//!
//! Implements Example 11.10 using verified parallel pair abstraction.
//! This is a FULLY VERIFIED parallel recursive algorithm!

#[cfg(verus_keep_ghost)]
pub mod FibonacciMtPerAllThreads {

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::ParaPair;
    use crate::Chap11::FibonacciStEph::FibonacciStEph::*;

    verus! {

        /// Parallel Fibonacci using ParaPair! for symmetric binary parallelism.
        /// Implements Ex 11.10. Exponential work; demonstrates parallel recursion patterns.
        /// - APAS: Work Θ(φⁿ), Span Θ(n) — full recursive parallelism.
        /// - Claude-Opus-4.6: Work Θ(φⁿ), Span Θ(n) — agrees. ParaPair! at every level.
        pub fn fib(n: u64) -> (fibonacci: u64)
            requires
                n <= 46,
            ensures
                fibonacci == spec_fib(n as nat),
            decreases n,
        {
            if n <= 1 {
                n
            } else {
                // Closures with explicit specs - Verus extracts these via
                // f.requires() and f.ensures() for para_pair's contract.
                let f1 = move || -> (r: u64)
                    requires n - 1 <= 46
                    ensures r == spec_fib((n - 1) as nat)
                { fib(n - 1) };

                let f2 = move || -> (r: u64)
                    requires n - 2 <= 46
                    ensures r == spec_fib((n - 2) as nat)
                { fib(n - 2) };

                let Pair(left, right) = ParaPair!(f1, f2);

                // para_pair ensures the propagation of the closure postconditions.
                proof { lemma_fib_sum_fits_u64(n as nat); }
                left + right
            }
        }

    } // verus!
} // mod
