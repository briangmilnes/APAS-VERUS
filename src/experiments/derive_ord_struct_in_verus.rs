//! Experiment: #[derive(Ord)] on struct inside verus!
//!
//! Hypothesis: #[derive(Eq, PartialEq, PartialOrd, Ord)] on struct verifies.
//! Result: Verifies. Ord requires Eq and PartialOrd.

pub mod derive_ord_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Eq, PartialEq, PartialOrd, Ord)]
        pub struct S {
            pub x: i32,
        }
    }
}
