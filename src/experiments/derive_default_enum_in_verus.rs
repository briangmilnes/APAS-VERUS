//! Experiment: #[derive(Default)] on enum inside verus!
//!
//! Hypothesis: #[derive(Default)] on enum inside verus! verifies.
//! Result: Verifies.

pub mod derive_default_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Default)]
        pub enum E {
            #[default]
            A,
            B(i32),
        }
    }
}
