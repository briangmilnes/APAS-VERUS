//! Experiment: #[derive(Eq)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. Eq requires PartialEq.

pub mod derive_eq_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
