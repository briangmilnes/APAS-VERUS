//! Experiment: #[derive(Ord)] on enum inside verus!
//!
//! Hypothesis: #[derive(Eq, PartialEq, PartialOrd, Ord)] on enum verifies.
//! Result: Verifies. Ord requires Eq and PartialOrd.

pub mod derive_ord_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Eq, PartialEq, PartialOrd, Ord)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
