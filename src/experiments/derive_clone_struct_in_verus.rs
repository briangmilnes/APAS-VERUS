//! Experiment: #[derive(Clone)] on struct inside verus!
//!
//! Hypothesis: #[derive(Clone)] on struct inside verus! verifies.
//! Result: Verifies. Verus warns it doesn't add spec for non-Copy clone.
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors, 1 warning:
//!         `autoderive_clone_without_spec`. Compiles; no specification.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_clone_struct_in_verus.20260922-111914.log

pub mod derive_clone_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Clone)]
        pub struct S {
            pub x: i32,
        }
    }
}
