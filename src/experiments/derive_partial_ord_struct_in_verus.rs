//! Experiment: #[derive(PartialOrd)] on struct inside verus!
//!
//! Hypothesis: #[derive(PartialEq, PartialOrd)] on struct inside verus! verifies.
//! Result: Verifies. PartialOrd requires PartialEq.

pub mod derive_partial_ord_struct_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, PartialOrd)]
        pub struct S {
            pub x: i32,
        }
    }
}
