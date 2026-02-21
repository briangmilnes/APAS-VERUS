//! Experiment: #[derive(PartialOrd)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(PartialEq, PartialOrd)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. PartialOrd requires PartialEq.

pub mod derive_partial_ord_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, PartialOrd)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
