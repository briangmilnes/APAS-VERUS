// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what `#[derive(StructuralEq)]` adds to a derived
//! `PartialEq` on a struct of primitives.
//!
//! Question. `builtin_macros/src/structural.rs` shows `derive_structural_eq`
//! emitting two items: the `unsafe impl Structural`, and an
//! `impl vstd::std_specs::cmp::PartialEqSpecImpl` whose `obeys_eq_spec()` is
//! `true` and whose `eq_spec` is `spec_eq`. If that is enough, `==` decides
//! spec equality with no `accept`, and `vstd::laws_eq::obeys_concrete_eq`
//! becomes available to generic callers through
//! `axiom_structural_obeys_concrete_eq`. Does it?
//!
//! RESULT: SUCCEEDS — 3 verified, 0 errors. `StructuralEq` supplies the
//!         `PartialEqSpecImpl`, so `==` decides spec equality with no
//!         `accept`, and `axiom_structural_obeys_concrete_eq` fires, which
//!         discharges `obeys_concrete_eq::<PointS>()` for a generic caller.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_eq_struct.20260922-112440.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_eq_struct

pub mod derive_0913_structural_eq_struct {

    use vstd::prelude::*;

    verus! {

    use vstd::laws_eq::{obeys_concrete_eq, group_laws_eq};

    broadcast use group_laws_eq;

    #[derive(PartialEq, Eq, StructuralEq)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    /// The postcondition `derive_0913_partial_eq_struct.rs` could not prove.
    pub fn eq_decides_spec_equality(a: &PointS, b: &PointS) -> (equal: bool)
        ensures
            equal == (*a == *b),
    {
        *a == *b
    }

    /// A generic caller that needs the eq laws, not just this one type.
    pub fn eq_under_concrete_laws<T: Eq>(x: &T, y: &T) -> (equal: bool)
        requires
            obeys_concrete_eq::<T>(),
        ensures
            equal <==> *x == *y,
    {
        reveal(obeys_concrete_eq);
        x.eq(y)
    }

    /// The derived `Structural` discharges the generic function's `requires`.
    pub fn call_eq_under_concrete_laws(a: &PointS, b: &PointS) -> (equal: bool)
        ensures
            equal <==> *a == *b,
    {
        eq_under_concrete_laws(a, b)
    }

    } // verus!
} // pub mod derive_0913_structural_eq_struct
