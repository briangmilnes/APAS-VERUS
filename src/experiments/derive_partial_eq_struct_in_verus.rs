//! Experiment: #[derive(PartialEq)] on struct inside verus!
//!
//! Hypothesis: #[derive(PartialEq)] on struct inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; `==` proves
//!         nothing without `StructuralEq`
//!         (`derive_0913_partial_eq_struct.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_partial_eq_struct_in_verus.20260922-111925.log

pub mod derive_partial_eq_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq)]
        pub struct S {
            pub x: i32,
        }
    }
}
