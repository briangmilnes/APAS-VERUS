// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): `#[derive(Clone, Copy, PartialEq, Eq, Debug,
//! StructuralEq)]` on one struct inside `verus!`.
//!
//! Question. `derive_0913_all_five_struct.rs` measures the user's five-derive
//! line and finds one of its two facts missing. Adding the sixth derive,
//! `StructuralEq`, should supply the `PartialEqSpecImpl` the bare `PartialEq`
//! derive omits (`builtin_macros/src/structural.rs`). Does the six-derive line
//! give both facts, plus `obeys_concrete_eq` for generic callers, with no
//! `accept` and no hand-written impl?
//!
//! RESULT: SUCCEEDS — 4 verified, 0 errors, no warning. The six-derive line on
//!         a non-generic struct of primitives gives the clone fact, both
//!         equality facts, and `obeys_concrete_eq`, replacing a hand-written
//!         `Clone`, `PartialEq`, `Eq`, `PartialEqSpecImpl` and `Debug` and two
//!         `accept` holes with one attribute.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_all_five_plus_structural.20260922-112704.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_all_five_plus_structural

pub mod derive_0913_all_five_plus_structural {

    use vstd::prelude::*;

    verus! {

    use vstd::laws_eq::{obeys_concrete_eq, group_laws_eq};

    broadcast use group_laws_eq;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, StructuralEq)]
    pub struct PointS {
        pub x: u64,
        pub y: u64,
    }

    impl View for PointS {
        type V = (int, int);

        open spec fn view(&self) -> (int, int) {
            (self.x as int, self.y as int)
        }
    }

    /// Fact one: the clone equals its source, and so has the same view.
    pub fn clone_gives_structural_equality(p: &PointS) -> (cloned: PointS)
        ensures
            cloned == *p,
            cloned@ == p@,
    {
        p.clone()
    }

    /// Fact two: `==` decides spec equality, and the view is injective here, so
    /// it decides view equality too.
    pub fn eq_decides_view_equality(a: &PointS, b: &PointS) -> (equal: bool)
        ensures
            equal == (*a == *b),
            equal == (a@ == b@),
    {
        *a == *b
    }

    /// Fact three: the eq laws are available to generic callers.
    pub proof fn structural_eq_gives_concrete_eq()
        ensures
            obeys_concrete_eq::<PointS>(),
    {
    }

    } // verus!

    // 14. derive impls outside verus!

    pub fn render(p: &PointS) -> String {
        format!("{:?}", p)
    }
} // pub mod derive_0913_all_five_plus_structural
