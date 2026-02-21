//! Experiment: #[derive(Hash)] on struct inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq, Hash)] on struct inside verus! verifies.
//! Result: Verifies. Hash alone works; Eq needed for HashSet.

pub mod derive_hash_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq, Hash)]
        pub struct S {
            pub x: i32,
        }
    }
}
