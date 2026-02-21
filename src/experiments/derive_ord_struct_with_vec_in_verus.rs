//! Experiment: #[derive(Ord)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(Eq, PartialEq, PartialOrd, Ord)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. Ord requires Eq and PartialOrd.

pub mod derive_ord_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Eq, PartialEq, PartialOrd, Ord)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
