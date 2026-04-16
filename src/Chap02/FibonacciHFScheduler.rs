// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Parallel Fibonacci demonstrating bounded parallelism with global pool.
//! Reviewed and is clean. briangmilnes@gmail.com 13 March 2026 


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 9. impls

//		Section 1. module

pub mod FibonacciHFScheduler {

    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;

    #[cfg(verus_keep_ghost)]
    use {
        vstd::arithmetic::power::pow,
        vstd::arithmetic::power2::{pow2, lemma_pow2_unfold, lemma_pow2_strictly_increases, lemma2_to64_rest},
    };

verus! 
{

    //		Section 6. spec fns


    pub open spec fn spec_fib(n: nat) -> nat
        decreases n
    {
        if n <= 1 { n }
        else { spec_fib((n - 1) as nat) + spec_fib((n - 2) as nat) }
    }

    //		Section 7. proof fns/broadcast groups


    proof fn lemma_pow2_mono(a: nat, b: nat)
        requires a <= b,
        ensures pow2(a) <= pow2(b),
    {
        if a < b { lemma_pow2_strictly_increases(a, b); }
    }

    proof fn lemma_pow2_46_lt_u64_max()
        ensures pow2(46) < u64::MAX as nat,
    {
        lemma2_to64_rest();
    }

    proof fn lemma_fib_bound(n: nat)
        ensures spec_fib(n) < pow2(n),
        decreases n,
    {
        reveal(pow);
        reveal(pow2);
        if n == 0 {
        } else if n == 1 {
            // Veracity: NEEDED assert
            assert(pow2(1) == 2) by(compute_only);
        } else {
            lemma_fib_bound((n - 1) as nat);
            lemma_fib_bound((n - 2) as nat);
            lemma_pow2_mono((n - 2) as nat, (n - 1) as nat);
            lemma_pow2_unfold(n);
        }
    }

    proof fn lemma_fib_fits_u64(n: nat)
        requires n <= 46,
        ensures spec_fib(n) <= u64::MAX as nat,
    {
        lemma_fib_bound(n);
        lemma_pow2_46_lt_u64_max();
        lemma_pow2_mono(n, 46);
    }

    proof fn lemma_fib_sum_fits_u64(n: nat)
        requires 2 <= n <= 46,
        ensures spec_fib((n-1) as nat) + spec_fib((n-2) as nat) <= u64::MAX as nat,
    {
        lemma_fib_fits_u64(n);
    }

    //		Section 9. impls


    /// - Alg Analysis: APAS (Ch11 Ex 11.1): Work O(φⁿ), Span O(φⁿ)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(φⁿ), Span O(φⁿ); sequential, work = span
    pub fn fib_seq(n: u64) -> (fibonacci: u64)
        requires n <= 46,
        ensures fibonacci == spec_fib(n as nat),
        decreases n,
    {
        if n <= 1 {
            n
        } else {
            // Veracity: NEEDED proof block
            proof { lemma_fib_sum_fits_u64(n as nat); }
            fib_seq(n - 1) + fib_seq(n - 2)
        }
    }

    /// - Alg Analysis: APAS (Ch11 Ex 11.1): Work O(φⁿ), Span O(n) — both branches recurse in parallel
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(φⁿ), Span O(n); recursive fib_par through join()
    pub fn fib_par(n: u64) -> (fibonacci: u64)
        requires n <= 46,
        ensures fibonacci == spec_fib(n as nat),
        decreases n,
    {
        if n <= 1 {
            n
        } else if n <= 10 {
            fib_seq(n)
        } else {
            let f1 = move || -> (r: u64)
                requires n >= 2, n <= 46,
                ensures r == spec_fib((n - 1) as nat),
            { fib_par(n - 1) };

            let f2 = move || -> (r: u64)
                requires n >= 2, n <= 46,
                ensures r == spec_fib((n - 2) as nat),
            { fib_par(n - 2) };

            // Veracity: NEEDED proof block
            let (a, b) = join(f1, f2);
            proof { lemma_fib_sum_fits_u64(n as nat); }
            a + b
        }
    }

} // verus!
} // mod
