// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Caller that uses the iterative sum (the alternative).
//!
//! RESULT: VERIFIES

pub mod trait_iter_caller {

    use vstd::prelude::*;
    use crate::experiments::trait_rec_vs_iter::trait_rec_vs_iter::{
        StackTrait, Stack,
    };
    #[cfg(verus_keep_ghost)]
    use crate::experiments::trait_rec_vs_iter::trait_rec_vs_iter::spec_sum;

    verus! {

    fn caller() -> (total: u64)
        ensures total == 60int,
    {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);
        s.push(30);
        assert(s@ == seq![10u64, 20u64, 30u64]);
        assert(spec_sum(s@, 0) == 0);
        assert(spec_sum(s@, 1) == 10);
        assert(spec_sum(s@, 2) == 30);
        assert(spec_sum(s@, 3) == 60);
        s.sum_iter()
    }

    } // verus!
}
