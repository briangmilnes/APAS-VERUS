//! Experiment: #[derive(Debug)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(Debug)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies.

pub mod derive_debug_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Debug)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
