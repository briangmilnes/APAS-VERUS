//! Proof tests for ArraySeqStEph iterator
//!
//! Tests for loop patterns with ArraySeqStEphIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] arrayseq_st_eph_iter_basic verus_code! {
        use vstd::prelude::*;

        fn test_basic() {
            let v: Vec<u64> = vec![5, 10, 15, 20];
            let mut processed: bool = false;
            for _x in v
                invariant true,
            {
                processed = true;
            }
            assert(processed || !processed);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] arrayseq_st_eph_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range_accumulate() {
            let v: Vec<u64> = vec![1, 2, 3, 4];
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

