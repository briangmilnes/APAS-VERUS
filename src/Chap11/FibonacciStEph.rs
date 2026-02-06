//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Fibonacci.
//! Verified specification and implementation of the Fibonacci sequence.

pub mod FibonacciStEph {
    use vstd::prelude::*;
    
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power::pow;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power2::{pow2, lemma_pow2_unfold};
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::arithmetic::power2_plus::power2_plus::*;

    verus! {

        /// - Specification of Fibonacci sequence.
        /// - fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2) for n >= 2
        pub open spec fn spec_fib(n: nat) -> nat
            decreases n,
        {
            if n == 0 {
                0
            } else if n == 1 {
                1
            } else {
                spec_fib((n - 1) as nat) + spec_fib((n - 2) as nat)
            }
        }

        /// - Iterative Fibonacci implementation.
        /// - Work: Θ(n), Space: O(1)
        pub fn fib(n: u64) -> (result: u64)
            requires
                n <= 46,
            ensures
                result == spec_fib(n as nat),
        {
            if n == 0 {
                return 0;
            }
            if n == 1 {
                return 1;
            }

            let mut prev2: u64 = 0;
            let mut prev1: u64 = 1;
            let mut i: u64 = 2;

            while i <= n
                invariant
                    2 <= i <= n + 1,
                    n <= 46,
                    prev2 == spec_fib((i - 2) as nat),
                    prev1 == spec_fib((i - 1) as nat),
                decreases n - i + 1,
            {
                proof { lemma_fib_sum_fits_u64(i as nat); }
                let next = prev1 + prev2;
                prev2 = prev1;
                prev1 = next;
                i = i + 1;
            }

            prev1
        }

        /// - Recursive Fibonacci implementation (matches APAS structure).
        /// - Work: Θ(φⁿ), Span: Θ(n), where φ ≈ 1.618 (golden ratio)
        /// - Note: Exponential work - for demonstration only.
        pub fn fib_recursive(n: u64) -> (result: u64)
            requires
                n <= 46,
            ensures
                result == spec_fib(n as nat),
            decreases n,
        {
            if n <= 1 {
                n
            } else {
                let left = fib_recursive(n - 1);
                let right = fib_recursive(n - 2);
                proof { lemma_fib_sum_fits_u64(n as nat); }
                left + right
            }
        }

        /// Fibonacci is bounded by 2^n
        proof fn lemma_fib_bound(n: nat)
            ensures spec_fib(n) < pow2(n),
            decreases n,
        {
            reveal(pow);
            reveal(pow2);
            if n == 0 {
            } else if n == 1 {
                assert(pow2(1) == 2) by(compute_only);
            } else {
                lemma_fib_bound((n - 1) as nat);
                lemma_fib_bound((n - 2) as nat);
                lemma_pow2_mono((n - 2) as nat, (n - 1) as nat);
                lemma_pow2_unfold(n);
            }
        }

        /// Fibonacci values fit in u64 for n <= 46
        proof fn lemma_fib_fits_u64(n: nat)
            requires n <= 46,
            ensures spec_fib(n) <= u64::MAX as nat,
        {
            lemma_fib_bound(n);
            lemma_pow2_46_lt_u64_max();
            lemma_pow2_mono(n, 46);
        }

        /// Sum of adjacent Fibonacci values fits in u64 for n <= 46
        pub proof fn lemma_fib_sum_fits_u64(n: nat)
            requires 2 <= n <= 46,
            ensures spec_fib((n-1) as nat) + spec_fib((n-2) as nat) <= u64::MAX as nat,
        {
            lemma_fib_fits_u64(n);
        }

    } // verus!
} // mod
