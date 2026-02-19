// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.1: Points in 2D using imperative loops.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	9. impls

//		1. module

pub mod Problem21_1 {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    //		9. impls

    /// Problem 21.1 (Points in 2D) - Imperative approach using nested loops.
    /// Construct the sequence of 2D points (x, y) with 0 ≤ x < n and 1 ≤ y < n,
    /// ordered by x major, then y.
    /// - APAS: Work Θ(n²), Span Θ(n²) (sequential due to imperative loops)
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²)
    pub fn points2d(n: N) -> (points: ArraySeqStPerS<Pair<N, N>>)
        requires
            n as int * (n as int - 1) <= usize::MAX as int,
        ensures
            n == 0 ==> points.seq@.len() == 0,
            n > 0  ==> points.seq@.len() == n as int * (n as int - 1),
            forall|k: int| 0 <= k < points.seq@.len() ==>
                (#[trigger] points.seq@[k]).0 < n
                && 1 <= points.seq@[k].1
                && points.seq@[k].1 < n,
    {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }
        let len = n * (n - 1);
        let mut v = Vec::<Pair<N, N>>::with_capacity(len);
        let mut x: usize = 0;
        while x < n
            invariant
                x <= n,
                n > 0,
                v@.len() == x as int * (n as int - 1),
                n as int * (n as int - 1) <= usize::MAX as int,
                forall|k: int| 0 <= k < v@.len() ==>
                    (#[trigger] v@[k]).0 < n
                    && 1 <= v@[k].1
                    && v@[k].1 < n,
            decreases n - x,
        {
            let mut y: usize = 1;
            while y < n
                invariant
                    x < n,
                    n > 0,
                    1 <= y <= n,
                    v@.len() == x as int * (n as int - 1) + (y as int - 1),
                    n as int * (n as int - 1) <= usize::MAX as int,
                    forall|k: int| 0 <= k < v@.len() ==>
                        (#[trigger] v@[k]).0 < n
                        && 1 <= v@[k].1
                        && v@[k].1 < n,
                decreases n - y,
            {
                v.push(Pair(x, y));
                y = y + 1;
            }
            assert(v@.len() == x as int * (n as int - 1) + (n as int - 1));
            assert((x as int + 1) * (n as int - 1) == x as int * (n as int - 1) + (n as int - 1))
                by (nonlinear_arith);
            x = x + 1;
        }
        ArraySeqStPerS { seq: v }
    }

    } // verus!
}
