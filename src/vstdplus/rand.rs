//! Copyright (c) 2025 Brian G. Milnes
//! Verified wrapper around rand crate primitives.
//! The external_body trust boundary: rand returns a value in the requested range.

// Table of Contents
// 1. module
// 9. impls

// 1. module

pub mod rand {

    use vstd::prelude::*;

    verus! {

    // 9. impls

    /// Returns a uniformly random usize in [lo, hi).
    #[verifier::external_body]
    pub fn random_usize_range(lo: usize, hi: usize) -> (result: usize)
        requires lo < hi,
        ensures lo <= result && result < hi,
    {
        random_usize_range_exec(lo, hi)
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    fn random_usize_range_exec(lo: usize, hi: usize) -> usize {
        use rand::Rng;
        use rand::RngExt;
        rand::rng().random_range(lo..hi)
    }

    #[cfg(verus_keep_ghost)]
    fn random_usize_range_exec(_lo: usize, _hi: usize) -> usize {
        unimplemented!()
    }
}
