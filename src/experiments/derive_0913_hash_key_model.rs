// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r216): whether a derived `Hash` lets the prover conclude
//! `obeys_key_model`.
//!
//! Question. Every vstd `HashMap` and `HashSet` postcondition is guarded by
//! `obeys_key_model::<Key>()` (`vstd/std_specs/hash.rs`), and APAS carries two
//! `external_body` `Hash` bodies in Chap05 for that reason. `obeys_key_model`
//! is an `uninterp spec fn` with broadcast axioms only for the primitives and
//! `Box`. If a derived `Hash` on a struct whose fields are all primitives, plus
//! `StructuralEq` for the equality half of the model, discharged it, the two
//! `external_body` bodies could go. Does it?
//!
//! RESULT: FAILS — 0 verified, 1 error: "obeys_key_model() function is
//!         uninterpreted". No derive reaches it; vstd's own documentation says
//!         a user key type must `assume` it. The two Chap05 `external_body`
//!         `Hash` bodies stand until vstd supplies a proof rule.
//! DATE: 2026-09-22
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_0913_hash_key_model.20260922-112548.log
//! Run: scripts/validate-standard.sh --experiment derive_0913_hash_key_model

pub mod derive_0913_hash_key_model {

    use std::hash::Hash;

    use vstd::prelude::*;

    verus! {

    use vstd::std_specs::hash::{obeys_key_model, group_hash_axioms};

    broadcast use group_hash_axioms;

    #[derive(PartialEq, Eq, Hash, StructuralEq)]
    pub struct KeyS {
        pub k: u64,
    }

    /// The fact every `HashMap` and `HashSet` postcondition is guarded by.
    pub proof fn derived_hash_obeys_key_model()
        ensures
            obeys_key_model::<KeyS>(),
    {
    }

    } // verus!
} // pub mod derive_0913_hash_key_model
