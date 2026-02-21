//! Experiment: #[derive(Copy)] on struct inside verus!
//!
//! Hypothesis: #[derive(Copy, Clone)] on struct inside verus! verifies.
//! Result: Verifies.

pub mod derive_copy_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Copy, Clone)]
        pub struct S {
            pub x: i32,
        }
    }
}
