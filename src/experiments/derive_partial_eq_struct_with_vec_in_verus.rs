//! Experiment: #[derive(PartialEq)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(PartialEq)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; `==` proves
//!         nothing, and `StructuralEq` is unavailable because `Vec` is not
//!         `Structural` (`derive_0913_structural_eq_vec_struct.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_partial_eq_struct_with_vec_in_verus.20260922-111926.log

pub mod derive_partial_eq_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
