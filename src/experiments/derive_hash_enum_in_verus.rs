//! Experiment: #[derive(Hash)] on enum inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq, Hash)] on enum inside verus! verifies.
//! Result: Verifies. Hash alone works; Eq needed for HashSet.

pub mod derive_hash_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq, Hash)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
