//! Experiment: #[derive(Debug)] on enum inside verus!
//!
//! Hypothesis: #[derive(Debug)] on enum inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Nothing to verify; the
//!         derive is accepted inside `verus!`.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_debug_enum_in_verus.20260922-111917.log

pub mod derive_debug_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Debug)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
