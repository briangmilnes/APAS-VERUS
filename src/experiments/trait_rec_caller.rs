//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Caller that uses the recursive implementation.
//!
//! To switch to iterative: change `RecStack` to `IterStack` in the use line.
//!
//! RESULT: VERIFIES

pub mod trait_rec_caller {

    use vstd::prelude::*;
    use crate::experiments::trait_rec_vs_iter::trait_rec_vs_iter::{
        StackTrait, RecStack,
    };
    #[cfg(verus_keep_ghost)]
    use crate::experiments::trait_rec_vs_iter::trait_rec_vs_iter::spec_sum;

    verus! {

    // Generic code — works with any StackTrait impl.
    fn build_and_sum<S: StackTrait>() -> (total: u64)
        ensures total == spec_sum(seq![10u64, 20u64, 30u64], 3),
    {
        let mut s = S::new();
        s.push(10);
        s.push(20);
        s.push(30);
        assert(s@ == seq![10u64, 20u64, 30u64]);
        assert(spec_sum(s@, 0) == 0);
        assert(spec_sum(s@, 1) == 10);
        assert(spec_sum(s@, 2) == 30);
        assert(spec_sum(s@, 3) == 60);
        s.sum()
    }

    // This file's choice: recursive.
    fn caller() -> (total: u64)
        ensures total == 60int,
    {
        assert(spec_sum(seq![10u64, 20u64, 30u64], 0) == 0);
        assert(spec_sum(seq![10u64, 20u64, 30u64], 1) == 10);
        assert(spec_sum(seq![10u64, 20u64, 30u64], 2) == 30);
        assert(spec_sum(seq![10u64, 20u64, 30u64], 3) == 60);
        build_and_sum::<RecStack>()
    }

    } // verus!
}
