//! Experiment: #[derive(Hash)] on struct inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq, Hash)] on struct inside verus! verifies.
//! Result: Verifies. Hash alone works; Eq needed for HashSet.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; the derive
//!         does not give `obeys_key_model` (`derive_0913_hash_key_model.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_hash_struct_in_verus.20260922-111922.log

pub mod derive_hash_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq, Hash)]
        pub struct S {
            pub x: i32,
        }
    }
}
