//! Experiment: #[derive(Clone)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(Clone)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. Verus warns it doesn't add spec for non-Copy clone (Vec is not Copy).

pub mod derive_clone_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Clone)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
