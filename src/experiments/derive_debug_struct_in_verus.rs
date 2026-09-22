//! Experiment: #[derive(Debug)] on struct inside verus!
//!
//! Hypothesis: #[derive(Debug)] on struct inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_debug_struct_in_verus.20260922-111917.log

pub mod derive_debug_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Debug)]
        pub struct S {
            pub x: i32,
        }
    }
}
