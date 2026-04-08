//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 21 — Problem 21.3: Points in 3D using imperative triple loop.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	9. impls

//		1. module

pub mod Problem21_3 {

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power::pow;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    //		9. impls

    /// Problem 21.3 (Points in 3D) using imperative triple loop.
    /// Generate points (x, y, z) with 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1.
    /// - Alg Analysis: APAS (Ch21 Prob 21.3): Work O(n³), Span O(n³)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n³), Span O(n³)
    pub fn points3d_loops(n: usize) -> (points: ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>)
        requires
            n + 2 <= usize::MAX,
            n as int * n as int <= usize::MAX as int,
            n as int * n as int * n as int <= usize::MAX as int,
        ensures
            n == 0 ==> points.seq@.len() == 0,
            n > 0  ==> points.seq@.len() == n as int * n as int * n as int,
            forall|k: int| 0 <= k < points.seq@.len() ==>
                (#[trigger] points.seq@[k]).0 < n
                && 1 <= points.seq@[k].1.0 && points.seq@[k].1.0 <= n
                && 2 <= points.seq@[k].1.1 && points.seq@[k].1.1 <= n + 1,
    {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }
        let nn: usize = n * n;
        let nnn: usize = nn * n;
        let mut v = Vec::<Pair<usize, Pair<usize, usize>>>::with_capacity(nnn);
        let mut x: usize = 0;
        while x < n
            invariant
                x <= n, n > 0,
                n + 2 <= usize::MAX,
                nn == n * n, nnn == nn * n,
                v@.len() == x as int * nn as int,
                forall|k: int| 0 <= k < v@.len() ==>
                    (#[trigger] v@[k]).0 < n
                    && 1 <= v@[k].1.0 && v@[k].1.0 <= n
                    && 2 <= v@[k].1.1 && v@[k].1.1 <= n + 1,
            decreases n - x,
        {
            let ghost v_len_before = v@.len();
            let mut y: usize = 1;
            while y <= n
                invariant
                    x < n, 1 <= y <= n + 1, n > 0,
                    n + 2 <= usize::MAX,
                    nn == n * n, nnn == nn * n,
                    v@.len() == v_len_before + (y as int - 1) * n as int,
                    v_len_before == x as int * nn as int,
                    forall|k: int| 0 <= k < v@.len() ==>
                        (#[trigger] v@[k]).0 < n
                        && 1 <= v@[k].1.0 && v@[k].1.0 <= n
                        && 2 <= v@[k].1.1 && v@[k].1.1 <= n + 1,
                decreases n + 1 - y,
            {
                let ghost v_len_mid = v@.len();
                proof {
                    // v_len_mid == v_len_before + (y-1)*n, v_len_before == x*nn, nn == n*n, nnn == nn*n
                    // x <= n-1, y <= n, so v_len_mid <= (n-1)*n*n + (n-1)*n = n*n*n - n < nnn
                    assert(v_len_mid <= nnn as int) by (nonlinear_arith)
                        requires
                            v_len_mid == v_len_before + (y as int - 1) * n as int,
                            v_len_before == x as int * nn as int,
                            nn == n as int * n as int,
                            nnn == nn as int * n as int,
                            x as int + 1 <= n as int,
                            1 <= y as int,
                            y as int <= n as int,
                            n as int > 0;
                }
                let mut z: usize = 2;
                while z <= n + 1
                    invariant
                        x < n, 1 <= y <= n, 2 <= z <= n + 2, n > 0,
                        n + 2 <= usize::MAX,
                        nn == n * n, nnn == nn * n,
                        v@.len() == v_len_mid + (z as int - 2),
                        v_len_mid == v_len_before + (y as int - 1) * n as int,
                        v_len_before == x as int * nn as int,
                        // Overflow safety: total <= nnn
                        v@.len() <= nnn as int,
                        forall|k: int| 0 <= k < v@.len() ==>
                            (#[trigger] v@[k]).0 < n
                            && 1 <= v@[k].1.0 && v@[k].1.0 <= n
                            && 2 <= v@[k].1.1 && v@[k].1.1 <= n + 1,
                    decreases n + 2 - z,
                {
                    proof {
                        // v.len() < nnn so push won't overflow capacity
                        assert(v_len_mid + (z as int - 2) + 1 <= nnn as int) by (nonlinear_arith)
                            requires
                                v_len_mid == v_len_before + (y as int - 1) * n as int,
                                v_len_before == x as int * nn as int,
                                nn == n as int * n as int,
                                nnn == nn as int * n as int,
                                x as int + 1 <= n as int,
                                y as int <= n as int,
                                z as int <= n as int + 1,
                                n as int > 0;
                    }
                    v.push(Pair(x, Pair(y, z)));
                    z = z + 1;
                }
                // After inner: added n elements, v.len() == v_len_mid + n
                proof {
                    // (y - 1) * n + n == y * n
                    assert((y as int - 1) * n as int + n as int == y as int * n as int)
                        by (nonlinear_arith);
                    // Overflow: v.len() <= nnn
                    assert(y as int * n as int <= n as int * n as int)
                        by (nonlinear_arith)
                        requires y as int <= n as int, n as int >= 0;
                    assert(v_len_before + n as int * n as int <= (x as int + 1) * nn as int)
                        by (nonlinear_arith)
                        requires v_len_before == x as int * nn as int, nn == n as int * n as int;
                }
                y = y + 1;
            }
            // After middle: v.len() == v_len_before + n * n == (x+1) * nn
            proof {
                assert((x as int + 1) * nn as int == x as int * nn as int + nn as int)
                    by (nonlinear_arith);
            }
            x = x + 1;
        }
        proof {
            assert(n as int * nn as int == n as int * n as int * n as int)
                by (nonlinear_arith)
                requires nn == n as int * n as int;
        }
        ArraySeqStPerS { seq: v }
    }

    } // verus!
}
