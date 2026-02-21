//! Experiment: #[derive(Copy)] on enum inside verus!
//!
//! Hypothesis: #[derive(Copy, Clone)] on enum inside verus! verifies.
//! Result: Verifies.

pub mod derive_copy_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Copy, Clone)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
