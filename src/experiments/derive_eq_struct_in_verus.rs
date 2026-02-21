//! Experiment: #[derive(Eq)] on struct inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq)] on struct inside verus! verifies.
//! Result: Verifies. Eq requires PartialEq.

pub mod derive_eq_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq)]
        pub struct S {
            pub x: i32,
        }
    }
}
