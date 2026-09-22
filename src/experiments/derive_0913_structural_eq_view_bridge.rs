// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether `#[derive(StructuralEq)]` discharges the APAS
//! `PartialEq` postcondition, which is stated over `View` rather than over
//! structural equality.
//!
//! Question. APAS writes `ensures equal == (self@ == other@)`
//! (`src/standards/partial_eq_eq_clone_standard.rs`), while `StructuralEq`
//! supplies `eq_spec == spec_eq`. The two agree only when the view is
//! injective. This file measures both cases: `KeyS`, whose view is its only
//! field, and `StampedS`, whose view drops a field.
//!
//! RESULT: SUCCEEDS — 2 verified, 0 errors. `StructuralEq` discharges the APAS
//!         postcondition in full when the view is injective, and gives the
//!         `equal ==> a@ == b@` direction otherwise. A module whose view drops
//!         a field keeps a hand-written `PartialEq` if it wants the biconditional.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_structural_eq_view_bridge.20260922-112546.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_structural_eq_view_bridge

pub mod derive_0913_structural_eq_view_bridge {

    use vstd::prelude::*;

    verus! {

    #[derive(PartialEq, Eq, StructuralEq)]
    pub struct KeyS {
        pub k: u64,
    }

    impl View for KeyS {
        type V = u64;

        open spec fn view(&self) -> u64 {
            self.k
        }
    }

    #[derive(PartialEq, Eq, StructuralEq)]
    pub struct StampedS {
        pub k: u64,
        pub stamp: u64,
    }

    impl View for StampedS {
        type V = u64;

        open spec fn view(&self) -> u64 {
            self.k
        }
    }

    /// Injective view: the APAS postcondition holds in both directions.
    pub fn eq_decides_view_equality(a: &KeyS, b: &KeyS) -> (equal: bool)
        ensures
            equal == (a@ == b@),
    {
        *a == *b
    }

    /// Non-injective view: structural equality still implies view equality, so
    /// this direction survives.
    pub fn eq_implies_view_equality(a: &StampedS, b: &StampedS) -> (equal: bool)
        ensures
            equal ==> a@ == b@,
    {
        *a == *b
    }

    } // verus!
} // pub mod derive_0913_structural_eq_view_bridge
