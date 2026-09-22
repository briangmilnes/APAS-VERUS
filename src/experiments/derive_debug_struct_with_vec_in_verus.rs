//! Experiment: #[derive(Debug)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(Debug)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_debug_struct_with_vec_in_verus.20260922-111918.log

pub mod derive_debug_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Debug)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
