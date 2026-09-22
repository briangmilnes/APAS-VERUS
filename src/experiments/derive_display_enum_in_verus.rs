//! Experiment: #[derive(Display)] on enum inside verus!
//!
//! Hypothesis: derive_more::Display on enum inside verus! verifies.
//! Result: Not tested; Verus can't link derive_more. Module commented out.
//!
//! RESULT: FAILS — does not compile: `error[E0432]: unresolved import
//!         `derive_more``. Unchanged from the first run; `derive_more` is not
//!         a dependency Verus can link.
//! DATE: 2026-09-22 (re-run under r216)
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-derive_display_enum_in_verus.20260922-111920.log

pub mod derive_display_enum_in_verus {
    use derive_more::Display;
    use vstd::prelude::*;

    verus! {
        #[derive(Display)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
