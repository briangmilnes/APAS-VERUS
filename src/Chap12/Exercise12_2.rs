//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.2: implement fetch-and-add using compare-and-swap.

pub mod Exercise12_2 {
    use vstd::prelude::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

verus! {

/// Trait for CAS-based fetch_add extension on AtomicUsize.
pub trait FetchAddCasTrait {
    /// Implements fetch_add using compare_exchange_weak (CAS loop).
    /// Returns the previous value, atomically adding delta to target.
    /// - APAS: no cost spec. Notes CAS-based FAA is less efficient than hardware FAA under contention.
    /// - Claude-Opus-4.6: amortized O(1), worst-case unbounded (CAS retries under contention).
    fn fetch_add_cas(&self, delta: usize) -> (previous: usize);
}

impl FetchAddCasTrait for AtomicUsize {
    /// Note: vstd's std_specs::atomic provides assume_specification for AtomicUsize
    /// methods but without value postconditions, so we cannot prove functional
    /// correctness. The implementation is verified to be well-formed.
    #[verifier::exec_allows_no_decreases_clause]
    fn fetch_add_cas(&self, delta: usize) -> (previous: usize)
    {
        let mut current = self.load(Ordering::Relaxed);
        loop {
            let next = current.wrapping_add(delta);
            match self.compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire) {
                Result::Ok(prev) => return prev,
                Result::Err(observed) => current = observed,
            }
        }
    }
}

} // verus!

} 
