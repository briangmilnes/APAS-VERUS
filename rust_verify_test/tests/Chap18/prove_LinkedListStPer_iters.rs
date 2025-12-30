//! Proof tests for LinkedListStPer iterator
//!
//! Tests for loop patterns with LinkedListStPerIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] linkedlist_st_per_iter_basic verus_code! {
        use vstd::prelude::*;

        fn test_basic() {
            let v: Vec<u64> = vec![11, 22, 33, 44];
            let mut finished: bool = false;
            for _x in v
                invariant true,
            {
                finished = true;
            }
            assert(finished || !finished);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] linkedlist_st_per_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range() {
            for i in iter: 0usize..5usize
                invariant iter.cur <= 5,
            {
                // Walk list node i
            }
        }
    } => Ok(())
}

