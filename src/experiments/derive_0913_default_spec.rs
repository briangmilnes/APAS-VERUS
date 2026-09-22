// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `Default` gives the prover.
//!
//! Question. `automatic_derive.rs` maps `RustItem::Default` to
//! `AutomaticDeriveAction::Ignore`, so the derived `default()` is not verified
//! and carries no `ensures`. APAS's 38 hand-written `Default` impls inside
//! `verus!` state the value or the view of the default. Can a caller of
//! `S::default()` learn the field values?
//!
//! RESULT: FAILS — 0 verified, 1 error. The derive compiles inside `verus!`
//!         but `c.n == 0` is unprovable: the derived `default()` carries no
//!         postcondition at all.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_default_spec.20260922-112548.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_default_spec

pub mod derive_0913_default_spec {

    use vstd::prelude::*;

    verus! {

    #[derive(Default)]
    pub struct CounterS {
        pub n: u64,
    }

    /// The postcondition a hand-written `Default` states.
    pub fn default_is_zero() -> (c: CounterS)
        ensures
            c.n == 0,
    {
        CounterS::default()
    }

    } // verus!
} // pub mod derive_0913_default_spec
