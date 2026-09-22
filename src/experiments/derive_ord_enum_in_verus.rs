//! Experiment: #[derive(Ord)] on enum inside verus!
//!
//! Hypothesis: #[derive(Eq, PartialEq, PartialOrd, Ord)] on enum verifies.
//! Result: Verifies. Ord requires Eq and PartialOrd.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; `<=` and
//!         `cmp` prove nothing (`derive_0913_ord_obeys_cmp.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_ord_enum_in_verus.20260922-111923.log

pub mod derive_ord_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Eq, PartialEq, PartialOrd, Ord)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
