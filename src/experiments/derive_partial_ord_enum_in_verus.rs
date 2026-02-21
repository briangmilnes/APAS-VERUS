//! Experiment: #[derive(PartialOrd)] on enum inside verus!
//!
//! Hypothesis: #[derive(PartialEq, PartialOrd)] on enum inside verus! verifies.
//! Result: Verifies. PartialOrd requires PartialEq.

pub mod derive_partial_ord_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(PartialEq, PartialOrd)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
