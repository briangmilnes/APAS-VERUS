//! Experiment: #[derive(Clone)] on enum inside verus!
//!
//! Hypothesis: #[derive(Clone)] on enum inside verus! verifies.
//! Result: Verifies. Verus warns autoderive doesn't take expected form.
//!
//! RESULT: SUCCEEDS — 1 verified, 0 errors, 1 warning: "autoderive Clone impl
//!         does not take the form Verus expects". Compiles; no specification.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_clone_enum_in_verus.20260922-111914.log

pub mod derive_clone_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Clone)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
