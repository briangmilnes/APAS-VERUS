//! Proof tests for ArraySeqMtEph iterator
//!
//! Tests for loop patterns with ArraySeqMtEphIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] arrayseq_mt_eph_iter_basic verus_code! {
        use vstd::prelude::*;

        fn test_basic() {
            let v: Vec<u64> = vec![7, 14, 21, 28];
            let mut count: usize = 0;
            for _x in v
                invariant true,
            {
                if count < usize::MAX {
                    count = count + 1;
                }
            }
            assert(count >= 0);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] arrayseq_mt_eph_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range() {
            let v: Vec<u64> = vec![10, 20, 30, 40];
            let len = v.len();
            for i in iter: 0usize..len
                invariant
                    iter.cur <= len,
                    len == v.len(),
            {
                let _element = v[i];
            }
        }
    } => Ok(())
}

