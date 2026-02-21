//! Experiment: #[derive(PartialEq)] on struct inside verus!
//!
//! Hypothesis: #[derive(PartialEq)] on struct inside verus! verifies.
//! Result: Verifies.

pub mod derive_partial_eq_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq)]
        pub struct S {
            pub x: i32,
        }
    }
}
