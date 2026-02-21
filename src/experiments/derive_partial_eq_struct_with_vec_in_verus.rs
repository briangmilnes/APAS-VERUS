//! Experiment: #[derive(PartialEq)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(PartialEq)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies.

pub mod derive_partial_eq_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
