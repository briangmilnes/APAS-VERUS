// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `Clone` gives the prover on a struct
//! generic in `T: Clone`.
//!
//! Question. The APAS collections are generic, so the derive that matters is
//! the one on `S<T>`. A generic `T: Clone` has no view-preservation guarantee
//! of its own — APAS supplies one through `obeys_feq_clone::<T>()` in the
//! module's well-formedness predicate. Does a derived `Clone` on `S<T>` give
//! the caller `cloned == *s` when `T` is `Copy`, and anything at all when it is
//! only `Clone`?
//!
//! RESULT: FAILS — 1 verified, 1 error, with the same
//!         `autoderive_clone_without_spec` warning as
//!         `derive_0913_clone_vec_struct.rs`. Even `cloned.value == s.value`,
//!         the weakest fact a caller could want, is not provable.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_clone_generic_struct.20260922-112333.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_clone_generic_struct

pub mod derive_0913_clone_generic_struct {

    use vstd::prelude::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    #[derive(Clone)]
    pub struct BoxS<T> {
        pub value: T,
    }

    /// The weakest useful postcondition: the clone holds an equal value.
    pub fn clone_preserves_field<T: Clone>(s: &BoxS<T>) -> (cloned: BoxS<T>)
        ensures
            cloned.value == s.value,
    {
        s.clone()
    }

    } // verus!
} // pub mod derive_0913_clone_generic_struct
