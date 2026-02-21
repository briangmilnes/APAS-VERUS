//! Experiment: #[derive(Clone)] on enum inside verus!
//!
//! Hypothesis: #[derive(Clone)] on enum inside verus! verifies.
//! Result: Verifies. Verus warns autoderive doesn't take expected form.

pub mod derive_clone_enum_in_verus {
    use vstd::prelude::*;

    verus! {
        #[derive(Clone)]
        pub enum E {
            A,
            B(i32),
        }
    }
}
