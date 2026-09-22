// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `PartialEq` alone gives the prover.
//!
//! Question. `rust_verify/src/automatic_derive.rs` maps `RustItem::PartialEq`
//! and `RustItem::Eq` to `AutomaticDeriveAction::Ignore`, so the derived impl
//! is not registered with the verifier. vstd specifies `PartialEq::eq` as
//! `Self::obeys_eq_spec() ==> r == self.eq_spec(other)`
//! (`vstd/std_specs/cmp.rs`), and `obeys_eq_spec` has no definition for a type
//! whose only `PartialEq` came from a bare derive. Can a caller of `==` then
//! conclude anything about the result?
//!
//! RESULT: FAILS — 0 verified, 1 error, no warning. A bare `PartialEq` derive
//!         compiles inside `verus!` and tells the prover nothing: both
//!         directions of `equal == (*a == *b)` fail, because
//!         `obeys_eq_spec()` has no definition and vstd's `eq` postcondition
//!         is guarded by it.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_partial_eq_struct.20260922-112440.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_partial_eq_struct

pub mod derive_0913_partial_eq_struct {

    use vstd::prelude::*;

    verus! {

    #[derive(PartialEq, Eq)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    /// The postcondition a caller needs: `==` decides spec equality.
    pub fn eq_decides_spec_equality(a: &PointS, b: &PointS) -> (equal: bool)
        ensures
            equal == (*a == *b),
    {
        *a == *b
    }

    } // verus!
} // pub mod derive_0913_partial_eq_struct
