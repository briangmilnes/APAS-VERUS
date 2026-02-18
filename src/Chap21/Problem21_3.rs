//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.3: Points in 3D using imperative triple loop.
//! Verusified.

pub mod Problem21_3 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    verus! {

    use vstd::arithmetic::power::pow;

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    /// Problem 21.3 (Points in 3D) using imperative triple loop.
    /// Generate points (x, y, z) with 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1.
    /// - APAS: Work Θ(n³), Span Θ(n³)
    /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³)
    pub fn points3d_loops(n: N) -> (result: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
        requires
            n + 2 <= usize::MAX,
            n as int * n as int <= usize::MAX as int,
            n as int * n as int * n as int <= usize::MAX as int,
        ensures
            n == 0 ==> result.seq@.len() == 0,
            n > 0  ==> result.seq@.len() == n as int * n as int * n as int,
            forall|k: int| 0 <= k < result.seq@.len() ==>
                (#[trigger] result.seq@[k]).0 < n
                && 1 <= result.seq@[k].1.0 && result.seq@[k].1.0 <= n
                && 2 <= result.seq@[k].1.1 && result.seq@[k].1.1 <= n + 1,
    {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }
        let nn: usize = n * n;
        let nnn: usize = nn * n;
        let mut v = Vec::<Pair<N, Pair<N, N>>>::with_capacity(nnn);
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
                    v.push(Pair(x, Pair(y, z)));
                    z = z + 1;
                }
                // After inner: added n elements, v.len() == v_len_mid + n
                y = y + 1;
            }
            // After middle: v.len() == v_len_before + n * n == (x+1) * nn
            proof {
                assert(v@.len() == v_len_before + n as int * n as int);
                assert((x as int + 1) * nn as int == x as int * nn as int + nn as int)
                    by (nonlinear_arith);
            }
            x = x + 1;
        }
        ArraySeqStPerS { seq: v }
    }

    } // verus!
}
