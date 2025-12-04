//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci (multi-threaded).
//! Implements Example 11.10 using the project Parallel Pair abstraction.

#[cfg(verus_keep_ghost)]
pub mod FibonacciMtPer {

    use vstd::prelude::*;
    use crate::ParaPair;
    use crate::Types::Types::*;
    use crate::Chap11::FibonacciStEph::FibonacciStEph::{
        spec_fib, lemma_fib_sum_fits_u64
    };

    verus! {

        /// Parallel Fibonacci using ParaPair! for symmetric binary parallelism.
        ///
        /// APAS: Work Θ(φⁿ), Span Θ(n)
        /// where φ = (1+√5)/2 ≈ 1.618 (golden ratio)
        ///
        /// Note: Exponential work makes this impractical for large n. This demonstrates
        /// parallel recursion patterns; real implementations use memoization or iteration.
        #[verifier::external_body]
        pub fn fib(n: u64) -> (result: u64)
            requires
                n <= 46,
            ensures
                result == spec_fib(n as nat),
        {
            if n <= 1 {
                n
            } else {
                let Pair(left, right) = ParaPair!(move || fib(n - 1), move || fib(n - 2));
                proof { lemma_fib_sum_fits_u64(n as nat); }
                left + right
            }
        }

    } // verus!
} // mod
