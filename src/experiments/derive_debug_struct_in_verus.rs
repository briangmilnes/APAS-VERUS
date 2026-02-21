//! Experiment: #[derive(Debug)] on struct inside verus!
//!
//! Hypothesis: #[derive(Debug)] on struct inside verus! verifies.
//! Result: Verifies.

pub mod derive_debug_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Debug)]
        pub struct S {
            pub x: i32,
        }
    }
}
