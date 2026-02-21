//! Experiment: #[derive(Eq)] on enum inside verus!
//!
//! Hypothesis: #[derive(PartialEq, Eq)] on enum inside verus! verifies.
//! Result: Verifies. Eq requires PartialEq.

pub mod derive_eq_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, Eq)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
