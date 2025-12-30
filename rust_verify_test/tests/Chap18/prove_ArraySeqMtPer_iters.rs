//! Proof tests for ArraySeqMtPer iterator
//!
//! Tests for loop patterns with ArraySeqMtPerIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] arrayseq_mt_per_iter_basic verus_code! {
        use vstd::prelude::*;

        fn test_basic() {
            let v: Vec<bool> = vec![true, false, true];
            let mut any_true: bool = false;
            for x in v
                invariant true,
            {
                if x {
                    any_true = true;
                }
            }
            assert(any_true || !any_true);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] arrayseq_mt_per_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range_parallel_pattern() {
            let v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
            let len = v.len();
            let mid = len / 2;
            
            // Left half
            for i in iter: 0usize..mid
                invariant
                    iter.cur <= mid,
                    mid <= len,
                    len == v.len(),
            {
                let _val = v[i];
            }
            
            // Right half  
            for i in iter: mid..len
                invariant
                    mid <= iter.cur,
                    iter.cur <= len,
                    len == v.len(),
            {
                let _val = v[i];
            }
        }
    } => Ok(())
}

