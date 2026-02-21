//! Experiment: #[derive(Display)] on enum inside verus!
//!
//! Hypothesis: derive_more::Display on enum inside verus! verifies.
//! Result: Not tested; Verus can't link derive_more. Module commented out.

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
