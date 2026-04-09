//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 11 — Parallel Fibonacci (multi-threaded, persistent).
//!
//! Implements Example 11.10 using verified parallel pair abstraction.
//! This is a FULLY VERIFIED parallel recursive algorithm!

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 9. impls

#[cfg(verus_keep_ghost)]

//		Section 1. module

pub mod FibonacciMtPerAllThreads {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::ParaPair;
    use crate::Chap11::FibonacciStEph::FibonacciStEph::*;

    verus! 
{

    //		Section 9. impls


        /// Parallel Fibonacci using ParaPair! for symmetric binary parallelism.
        /// Implements Ex 11.10. Exponential work; demonstrates parallel recursion patterns.
        /// - Alg Analysis: APAS (Ch11 Ex 11.10): Work O(φⁿ), Span O(n) — full recursive parallelism
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(φⁿ), Span O(n); ParaPair! at every level
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
                // Veracity: NEEDED proof block
                proof { lemma_fib_sum_fits_u64(n as nat); }
                left + right
            }
        }

    } // verus!
} // mod
