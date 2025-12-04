//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci exploring TSM patterns.
//!
//! Step 1: Simple spawn/join at top level only.
//! fib(n) spawns fib(n-1) and fib(n-2), each runs sequentially.

use vstd::prelude::*;
use vstd::thread::*;
use crate::Concurrency::diverge;

verus! {

use crate::Chap11::FibonacciStEph::FibonacciStEph::{spec_fib, fib as seq_fib, lemma_fib_sum_fits_u64};

/// Top-level parallel fib: spawn two threads, each runs sequential fib.
pub fn fib_two_threads(n: u64) -> (result: u64)
    requires n <= 46,
    ensures result == spec_fib(n as nat),
{
    if n <= 1 {
        n
    } else {
        let n1 = n - 1;
        let n2 = n - 2;
        
        // Closures with specs
        let f1 = move || -> (r: u64)
            requires n1 <= 46
            ensures r == spec_fib(n1 as nat)
        { seq_fib(n1) };
        
        let f2 = move || -> (r: u64)
            requires n2 <= 46
            ensures r == spec_fib(n2 as nat)
        { seq_fib(n2) };
        
        let h1 = spawn(f1);
        let h2 = spawn(f2);
        
        let left = match h1.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }
        };
        let right = match h2.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }
        };
        
        assert(left == spec_fib((n - 1) as nat));
        assert(right == spec_fib((n - 2) as nat));
        
        proof { lemma_fib_sum_fits_u64(n as nat); }
        left + right
    }
}

} // verus!
