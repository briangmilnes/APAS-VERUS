//! Experiment: #[derive(Display)] on struct inside verus!
//!
//! Hypothesis: derive_more::Display on struct inside verus! verifies.
//! Result: Not tested; Verus can't link derive_more. Module commented out.

pub mod derive_display_struct_in_verus {
    use derive_more::Display;
    use vstd::prelude::*;

    verus! {
        #[derive(Display)]
        pub struct S {
            pub x: i32,
        }
    }
}
