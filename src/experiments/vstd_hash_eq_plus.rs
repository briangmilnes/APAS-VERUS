// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: with the `PartialEq::eq` specifications for `HashSet` and
//! `HashMap` in `src/vstdplus/hash_specs_plus.rs`, does APAS's postcondition
//! `r == (a@ == b@)` follow for a concrete key type, where vstd's
//! `group_hash_axioms` gives `obeys_key_model::<u64>()` and
//! `builds_valid_hashers::<RandomState>()` and `laws_eq::group_laws_eq` gives
//! `obeys_eq::<u64>()` and `obeys_concrete_eq::<u64>()`? Two fns, one per
//! collection, each body the `==` operator.
//!
//! RESULT: SUCCEEDS — 182 verified, 0 errors (179 harness dependencies,
//!         `lemma_hash_set_clone_eq`, and the two derivations). The `==`
//!         operator resolves to the specified `PartialEq::eq` and the
//!         broadcast groups discharge every hypothesis for `u64` keys and
//!         values.
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-vstd_hash_eq_plus.20260921-083044.log
//! Run: scripts/validate-standard.sh --experiment vstd_hash_eq_plus deps

pub mod vstd_hash_eq_plus {
    use std::collections::{HashMap, HashSet};
    use vstd::prelude::*;

    #[allow(unused_imports)]
    use crate::vstdplus::hash_specs_plus::hash_specs_plus::*;

    verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::laws_eq::group_laws_eq,
    };

    /// APAS `SetStEph::eq` on the raw view: `r == (a@ == b@)`.
    pub fn derived_set_eq(a: &HashSet<u64>, b: &HashSet<u64>) -> (r: bool)
        ensures
            r == (a@ == b@),
    {
        *a == *b
    }

    /// The same for `HashMap` with integer values.
    pub fn derived_map_eq(a: &HashMap<u64, u64>, b: &HashMap<u64, u64>) -> (r: bool)
        ensures
            r == (a@ == b@),
    {
        *a == *b
    }

    } // verus!
}
