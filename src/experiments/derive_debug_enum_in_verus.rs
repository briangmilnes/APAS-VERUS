//! Experiment: #[derive(Debug)] on enum inside verus!
//!
//! Hypothesis: #[derive(Debug)] on enum inside verus! verifies.
//! Result: Verifies.

pub mod derive_debug_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Debug)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
