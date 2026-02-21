//! Experiment: #[derive(PartialEq)] on enum inside verus!
//!
//! Hypothesis: #[derive(PartialEq)] on enum inside verus! verifies.
//! Result: Verifies.

pub mod derive_partial_eq_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
