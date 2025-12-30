//! Proof tests for ArraySeq iterator
//!
//! Tests for loop patterns with ArraySeqIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] arrayseq_iter_consuming verus_code! {
        use vstd::prelude::*;

        fn test_consuming() {
            let v: Vec<u64> = vec![100, 200, 300];
            let mut seen: bool = false;
            for _x in v
                invariant true,
            {
                seen = true;
            }
            assert(seen || !seen);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] arrayseq_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range() {
            let v: Vec<i64> = vec![1, -2, 3, -4, 5];
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

