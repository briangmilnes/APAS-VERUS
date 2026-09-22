// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: with the `HashSet::clone` specification in
//! `src/vstdplus/hash_specs_plus.rs` (modelled on vstd 0.2026.09.13's
//! `HashMap::clone`, `std_specs/hash.rs:578`), does APAS's postcondition
//! `t@ == s@` follow, and for which key types? `vstd_hash_set_clone.rs`
//! showed that vstd alone gives nothing. Three fns: a concrete key, a generic
//! key with no hypothesis, and the mapped view `Set<Key::V>` that
//! `SetStEph::clone` ensures.
//!
//! RESULT: SUCCEEDS — 183 verified, 0 errors (179 harness dependencies,
//!         `lemma_hash_set_clone_eq`, and the three derivations). `t@ == s@`
//!         follows for every key type, with no hypothesis on `Key`.
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-vstd_hash_set_clone_plus.20260921-083043.log
//!      (082655 verified the inline-quantifier form; 083043 the opaque
//!      `spec_hash_set_cloned` form that Chap05 needed, see hash_specs_plus.rs)
//! Run: scripts/validate-standard.sh --experiment vstd_hash_set_clone_plus deps

pub mod vstd_hash_set_clone_plus {
    use std::collections::HashSet;
    use std::hash::Hash;
    use vstd::prelude::*;

    use crate::vstdplus::hash_specs_plus::hash_specs_plus::*;

    verus! {

    /// APAS `clone` on a concrete key type: `t@ == s@`.
    pub fn derived_clone_u64(s: &HashSet<u64>) -> (t: HashSet<u64>)
        ensures
            t@ == s@,
    {
        let t = s.clone();
        proof { lemma_hash_set_clone_eq(s@, t@); }
        t
    }

    /// The same for any key type: the derivation needs no hypothesis on `Key`,
    /// because `cloned(k, k)` holds by definition and the specification fixes
    /// the cardinality.
    pub fn derived_clone_generic<Key: Eq + Hash + Clone>(s: &HashSet<Key>) -> (t: HashSet<Key>)
        ensures
            t@ == s@,
    {
        let t = s.clone();
        proof { lemma_hash_set_clone_eq(s@, t@); }
        t
    }

    /// APAS's mapped view (`SetStEph::view` is `elements@.map(|x| x@)`): equal
    /// raw views give equal mapped views.
    pub fn derived_clone_view_plus<Key: View + Eq + Hash + Clone>(s: &HashSet<Key>) -> (t: HashSet<Key>)
        ensures
            t@.map(|k: Key| k@) == s@.map(|k: Key| k@),
    {
        let t = s.clone();
        proof { lemma_hash_set_clone_eq(s@, t@); }
        t
    }

    } // verus!
}
