//! Experiment: #[derive(Default)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(Default)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. Vec::default() yields empty vec.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; the claim
//!         that `Vec::default()` yields the empty vector is not available to
//!         the prover (`derive_0913_default_spec.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_default_struct_with_vec_in_verus.20260922-111919.log

pub mod derive_default_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Default)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
