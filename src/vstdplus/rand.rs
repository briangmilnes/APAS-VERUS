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

    // 4. type definitions

    #[verifier::external_type_specification]
    pub struct ExSeededRng(SeededRng);

    // 9. impls

    /// Returns a uniformly random usize in [lo, hi).
    #[verifier::external_body]
    pub fn random_usize_range(lo: usize, hi: usize) -> (result: usize)
        requires lo < hi,
        ensures lo <= result && result < hi,
    {
        random_usize_range_exec(lo, hi)
    }

    /// Creates a seeded random number generator.
    #[verifier::external_body]
    pub fn seeded_rng(seed: u64) -> (result: SeededRng)
    {
        seeded_rng_impl(seed)
    }

    /// Returns a random bool from a seeded RNG.
    #[verifier::external_body]
    pub fn random_bool_seeded(rng: &mut SeededRng) -> (result: bool)
    {
        random_bool_seeded_exec(rng)
    }

    } // verus!

    /// Opaque wrapper around a seeded RNG.
    pub struct SeededRng {
        #[cfg(not(verus_keep_ghost))]
        inner: rand::rngs::StdRng,
    }

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

    #[cfg(not(verus_keep_ghost))]
    fn seeded_rng_impl(seed: u64) -> SeededRng {
        use rand::SeedableRng;
        SeededRng { inner: rand::rngs::StdRng::seed_from_u64(seed) }
    }

    #[cfg(verus_keep_ghost)]
    fn seeded_rng_impl(_seed: u64) -> SeededRng {
        unimplemented!()
    }

    #[cfg(not(verus_keep_ghost))]
    fn random_bool_seeded_exec(rng: &mut SeededRng) -> bool {
        use rand::RngExt;
        rng.inner.random()
    }

    #[cfg(verus_keep_ghost)]
    fn random_bool_seeded_exec(_rng: &mut SeededRng) -> bool {
        unimplemented!()
    }
}
