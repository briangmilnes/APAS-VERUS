//! Experiment: #[derive(Default)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(Default)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. Vec::default() yields empty vec.

pub mod derive_default_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Default)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
