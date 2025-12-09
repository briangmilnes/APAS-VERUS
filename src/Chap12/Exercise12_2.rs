//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.2: implement fetch-and-add using compare-and-swap.

pub mod Exercise12_2 {
    use vstd::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

verus! {

    #[verifier::external_body]
    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> (previous: usize)
    {
        let mut current = target.load(Ordering::Relaxed);
        loop {
            let next = current.wrapping_add(delta);
            match target.compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire) {
                Ok(prev) => return prev,
                Err(observed) => current = observed,
            }
        }
    }

    pub fn efficiency_note() -> (note: &'static str)
    {
        "Hardware fetch_add completes in one atomic operation; the CAS loop may repeat under contention, so it cannot outperform native fetch_add."
    }

} // verus!

pub trait FetchAddCasTrait {
    fn fetch_add_cas(&self, delta: usize) -> usize;
}

impl FetchAddCasTrait for AtomicUsize {
    fn fetch_add_cas(&self, delta: usize) -> usize {
        fetch_add_cas(self, delta)
    }
}

} // mod

