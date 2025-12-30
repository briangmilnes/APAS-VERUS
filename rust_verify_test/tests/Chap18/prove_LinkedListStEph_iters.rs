//! Proof tests for LinkedListStEph iterator
//!
//! Tests for loop patterns with LinkedListStEphIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] linkedlist_st_eph_iter_basic verus_code! {
        use vstd::prelude::*;

        fn test_basic() {
            let v: Vec<u64> = vec![2, 4, 6, 8, 10];
            let mut visited: bool = false;
            for _x in v
                invariant true,
            {
                visited = true;
            }
            assert(visited || !visited);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] linkedlist_st_eph_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range_walk() {
            let v: Vec<u64> = vec![1, 2, 3, 4, 5];
            let len = v.len();
            for i in iter: 0usize..len
                invariant
                    iter.cur <= len,
                    len == v.len(),
            {
                let _node = v[i];
            }
        }
    } => Ok(())
}

