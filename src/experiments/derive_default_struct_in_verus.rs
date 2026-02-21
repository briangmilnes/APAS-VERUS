//! Experiment: #[derive(Default)] on struct inside verus!
//!
//! Hypothesis: #[derive(Default)] on struct inside verus! verifies.
//! Result: Verifies.

pub mod derive_default_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Default)]
        pub struct S {
            pub x: i32,
        }
    }
}
