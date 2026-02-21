//! Experiment: #[derive(Hash)] on struct with Vec inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq, Hash)] on struct containing Vec<T> inside verus! verifies.
//! Result: Verifies. Hash alone works; Eq needed for HashSet.

pub mod derive_hash_struct_with_vec_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq, Hash)]
        pub struct S {
            pub data: Vec<i32>,
        }
    }
}
