//! Experiment: #[derive(Default)] on enum inside verus!
//!
//! Hypothesis: #[derive(Default)] on enum inside verus! verifies.
//! Result: Verifies.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; the derived
//!         `default()` carries no postcondition
//!         (`derive_0913_default_spec.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_default_enum_in_verus.20260922-111918.log

pub mod derive_default_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Default)]
        pub enum E {
            #[default]
            A,
            B(i32),
        }
    }
}
