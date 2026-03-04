//! Proof tests for standards::view_standard.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] view_standard_simple verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_simple_view() {
            let s = SimpleS::new(42);
            assert(s@ == 42);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] view_standard_collection verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_collection_view() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            let c = CollectionS { seq: v };
            assert(c@.len() == 2);
            assert(c@[0] == 10);
            assert(c@[1] == 20);
        }
    } => Ok(())
}
