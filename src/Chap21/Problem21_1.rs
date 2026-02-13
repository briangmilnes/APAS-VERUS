// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.1: Points in 2D using imperative loops.

pub mod Problem21_1 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::{N, Pair};

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    /// Problem 21.1 (Points in 2D) - Imperative approach using nested loops.
    /// Construct the sequence of 2D points (x, y) with 0 ≤ x < n and 1 ≤ y < n,
    /// ordered by x major, then y.
    /// APAS: Work Θ(n²), Span Θ(n²) (sequential due to imperative loops)
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

    // Non-Verus implementation for cargo test compatibility.
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(not(verus_keep_ghost))]
    use crate::Types::Types::{N, Pair};

    #[cfg(not(verus_keep_ghost))]
    pub fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>> {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }
        let mut v = Vec::<Pair<N, N>>::with_capacity(n * (n - 1));
        for x in 0..n {
            for y in 1..n {
                v.push(Pair(x, y));
            }
        }
        ArraySeqStPerS { seq: v }
    }
}
