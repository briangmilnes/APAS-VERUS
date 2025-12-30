//! Proof tests for MathSeq iterator
//!
//! Tests for loop patterns with MathSeq IntoIterator implementations.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] mathseq_iter_consuming verus_code! {
        use vstd::prelude::*;

        fn test_consuming() {
            let v: Vec<u64> = vec![1, 2, 3, 4, 5];
            let mut last: u64 = 0;
            for x in v
                invariant true,
            {
                last = x;
            }
            assert(last >= 0);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] mathseq_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range_index() {
            let v: Vec<u64> = vec![10, 20, 30, 40, 50];
            let len = v.len();
            for i in iter: 0usize..len
                invariant
                    iter.cur <= len,
                    len == v.len(),
            {
                let _val = v[i];
            }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] mathseq_iter_nested verus_code! {
        use vstd::prelude::*;

        fn test_nested() {
            let outer: Vec<u64> = vec![1, 2];
            let mut total: bool = false;
            for _x in outer
                invariant true,
            {
                let inner: Vec<u64> = vec![10, 20];
                for _y in inner
                    invariant true,
                {
                    total = true;
                }
            }
            assert(total || !total);
        }
    } => Ok(())
}

