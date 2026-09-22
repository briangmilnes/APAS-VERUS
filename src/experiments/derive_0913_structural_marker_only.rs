// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what `#[derive(Structural)]` alone adds over a derived
//! `PartialEq`.
//!
//! Question. vstd exports two markers, `Structural` and `StructuralEq`
//! (`vstd/prelude.rs`). `laws_eq::axiom_structural_obeys_concrete_eq` is stated
//! for `T: PartialEq + Structural` and requires `T::obeys_eq_spec()`, which the
//! bare `Structural` derive does not supply. Is `Structural` on its own enough
//! to reach `obeys_concrete_eq`, or is `StructuralEq` the only form that helps?
//!
//! RESULT: FAILS — 0 verified, 1 error: "obeys_concrete_eq() function is
//!         uninterpreted". `Structural` alone is a marker only; the axiom's
//!         `T::obeys_eq_spec()` premise is unreachable without the
//!         `PartialEqSpecImpl` that `StructuralEq` emits. Use `StructuralEq`,
//!         not `Structural`.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_marker_only.20260922-112442.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_marker_only

pub mod derive_0913_structural_marker_only {

    use vstd::prelude::*;

    verus! {

    use vstd::laws_eq::{obeys_concrete_eq, group_laws_eq};

    broadcast use group_laws_eq;

    #[derive(PartialEq, Eq, Structural)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    pub proof fn structural_gives_concrete_eq()
        ensures
            obeys_concrete_eq::<PointS>(),
    {
    }

    } // verus!
} // pub mod derive_0913_structural_marker_only
