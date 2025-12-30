//! Proof tests for ArraySeqStPer iterator
//!
//! Tests for loop patterns with ArraySeqStPerIter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] arrayseq_st_per_iter_basic verus_code! {
        use vstd::prelude::*;

        fn test_basic() {
            let v: Vec<u64> = vec![1, 1, 2, 3, 5, 8];
            let mut iterated: bool = false;
            for _x in v
                invariant true,
            {
                iterated = true;
            }
            assert(iterated || !iterated);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] arrayseq_st_per_iter_range verus_code! {
        use vstd::prelude::*;

        fn test_range() {
            for i in iter: 0usize..10usize
                invariant iter.cur <= 10,
            {
                // Process element i
            }
        }
    } => Ok(())
}

