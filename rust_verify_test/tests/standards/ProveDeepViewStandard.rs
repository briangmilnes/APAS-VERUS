//! Proof tests for standards::deep_view_standard.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] deep_view_standard_simple verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::deep_view_standard::deep_view_standard::*;

        fn test_simple_deep_view() {
            let s = SimpleS { val: Some(42) };
            assert(s@ == Some(42usize));
            assert(s.deep_view() == Some(42usize));
        }
    } => Ok(())
}
