//! Experiment: #[derive(Default)] on struct inside verus!
//!
//! Hypothesis: #[derive(Default)] on struct inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; the derived
//!         `default()` carries no postcondition
//!         (`derive_0913_default_spec.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_default_struct_in_verus.20260922-111919.log

pub mod derive_default_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Default)]
        pub struct S {
            pub x: i32,
        }
    }
}
