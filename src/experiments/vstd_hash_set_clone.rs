// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: does vstd 0.2026.09.13 specify `HashSet::clone`? APAS's
//! `HashSetWithViewPlus::clone` ensures `clone@ == self@` behind
//! `external_body`, and `src/vstdplus/hash_set_specs.rs` declared an
//! `assume_specification` for `HashSet::clone` with no postcondition.
//! vstd's `std_specs/hash.rs` specifies `HashMap::clone` (per-key `cloned`)
//! but has no entry for `HashSet::clone`.
//!
//! RESULT: FAILS — 0 verified, 1 errors: "postcondition not satisfied"
//!         on `t@ == s@`. The call `s.clone()` is accepted but carries no
//!         postcondition, so nothing relates `t@` to `s@`. vstd needs an
//!         `assume_specification` for `HashSet::clone` like the one it has
//!         for `HashMap::clone` (std_specs/hash.rs:578).
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-vstd_hash_set_clone.20260920-222027.log
//! Run: scripts/validate-standard.sh --experiment vstd_hash_set_clone

pub mod vstd_hash_set_clone {
    use std::collections::HashSet;
    use vstd::prelude::*;
    use vstd::std_specs::hash::*;

    verus! {

    broadcast use group_hash_axioms;

    /// APAS `clone`: `clone@ == self@`.
    pub fn derived_clone(s: &HashSet<u64>) -> (t: HashSet<u64>)
        ensures
            t@ == s@,
    {
        s.clone()
    }

    } // verus!
}
