// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): what a derived `PartialOrd` and `Ord` give the prover.
//!
//! Question. `automatic_derive.rs` maps both to
//! `AutomaticDeriveAction::Ignore`, so neither `obeys_partial_cmp_spec` nor
//! `obeys_cmp_spec` gets a definition, and vstd's `cmp`, `lt` and `le`
//! postconditions are all guarded by them (`vstd/std_specs/cmp.rs`). Can a
//! caller of `<=` on a derived-`Ord` struct conclude the lexicographic order,
//! and can `vstd::laws_cmp::obeys_cmp` be proved for such a type? The struct
//! also derives `StructuralEq`, so the equality half of the laws is present.
//!
//! RESULT: FAILS — 0 verified, 2 errors. `<=` tells the prover nothing, and
//!         `obeys_cmp` breaks down as `obeys_eq_spec() OK` — supplied by
//!         `StructuralEq` — with `obeys_eq_spec_properties`,
//!         `obeys_cmp_partial_ord`, `obeys_cmp_ord` and
//!         `obeys_partial_cmp_spec_properties` all uninterpreted. There is no
//!         `StructuralOrd` counterpart to `StructuralEq`.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_ord_obeys_cmp.20260922-112612.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_ord_obeys_cmp

pub mod derive_0913_ord_obeys_cmp {

    use vstd::prelude::*;

    verus! {

    use vstd::laws_cmp::{obeys_cmp, group_laws_cmp};
    use vstd::laws_eq::group_laws_eq;

    broadcast use group_laws_eq, group_laws_cmp;

    #[derive(PartialEq, Eq, PartialOrd, Ord, StructuralEq)]
    pub struct RankS {
        pub major: u64,
        pub minor: u64,
    }

    /// The lexicographic order the derive implements.
    pub open spec fn spec_le(a: RankS, b: RankS) -> bool {
        a.major < b.major || (a.major == b.major && a.minor <= b.minor)
    }

    /// The postcondition an ordered APAS collection needs from `<=`.
    pub fn le_decides_lexicographic_order(a: &RankS, b: &RankS) -> (ordered: bool)
        ensures
            ordered == spec_le(*a, *b),
    {
        *a <= *b
    }

    /// The fact vstd's ordering laws are stated over.
    pub proof fn derived_ord_obeys_cmp()
        ensures
            obeys_cmp::<RankS>(),
    {
    }

    } // verus!
} // pub mod derive_0913_ord_obeys_cmp
