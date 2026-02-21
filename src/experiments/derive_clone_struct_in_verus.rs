//! Experiment: #[derive(Clone)] on struct inside verus!
//!
//! Hypothesis: #[derive(Clone)] on struct inside verus! verifies.
//! Result: Verifies. Verus warns it doesn't add spec for non-Copy clone.

pub mod derive_clone_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Clone)]
        pub struct S {
            pub x: i32,
        }
    }
}
