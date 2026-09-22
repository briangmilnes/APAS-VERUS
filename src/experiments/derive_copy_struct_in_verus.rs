//! Experiment: #[derive(Copy)] on struct inside verus!
//!
//! Hypothesis: #[derive(Copy, Clone)] on struct inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors, no warning.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_copy_struct_in_verus.20260922-111916.log

pub mod derive_copy_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Copy, Clone)]
        pub struct S {
            pub x: i32,
        }
    }
}
