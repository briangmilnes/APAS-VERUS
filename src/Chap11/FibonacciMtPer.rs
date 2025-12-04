//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci (multi-threaded).
//! Implements Example 11.10 using the project Parallel Pair abstraction.

// Verified specification module
#[cfg(verus_keep_ghost)]
pub mod FibonacciMtPer {

    use vstd::prelude::*;
    use crate::Chap11::FibonacciStEph::FibonacciStEph::spec_fib;

    verus! {

        /// Parallel Fibonacci using ParaPair! for symmetric binary parallelism.
        ///
        /// APAS: Work Θ(φⁿ), Span Θ(n)
        /// claude-4-sonet: Work Θ(φⁿ), Span Θ(n), Parallelism Θ(φⁿ/n) - parallel binary recursion via ParaPair!
        /// where φ = (1+√5)/2 ≈ 1.618 (golden ratio)
        ///
        /// Note: Exponential work makes this impractical for large n. This demonstrates
        /// parallel recursion patterns; real implementations use memoization or iteration.
        ///
        /// The actual implementation uses rayon::join via the ParaPair! macro.
        /// Since verus doesn't have access to cargo dependencies during verification,
        /// the implementation is in the non-verus module below.
        #[verifier::external_body]
        pub fn fib(n: u64) -> (result: u64)
            requires
                n <= 46,
            ensures
                result == spec_fib(n as nat),
        {
            unimplemented!()
        }

    } // verus!
} // mod

// Executable implementation module (not verified)
#[cfg(not(verus_keep_ghost))]
pub mod FibonacciMtPer {
    use crate::ParaPair;
    use crate::Types::Types::Pair;

    /// Parallel Fibonacci using ParaPair! for symmetric binary parallelism.
    pub fn fib(n: u64) -> u64 {
        if n <= 1 {
            n
        } else {
            let Pair(left, right) = ParaPair!(move || fib(n - 1), move || fib(n - 2));
            left + right
        }
    }
}
