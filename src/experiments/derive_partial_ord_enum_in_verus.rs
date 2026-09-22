//! Experiment: #[derive(PartialOrd)] on enum inside verus!
//!
//! Hypothesis: #[derive(PartialEq, PartialOrd)] on enum inside verus! verifies.
//! Result: Verifies. PartialOrd requires PartialEq.
//!
//! RESULT: SUCCEEDS — 0 verified, 0 errors, no warning. Compiles; `<` and
//!         `partial_cmp` prove nothing (`derive_0913_ord_obeys_cmp.rs`).
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_partial_ord_enum_in_verus.20260922-111926.log

pub mod derive_partial_ord_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, PartialOrd)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
