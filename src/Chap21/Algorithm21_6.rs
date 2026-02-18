//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.6: Prime Sieve using ArraySeqPer and ninject.
//! Verusified.

pub mod Algorithm21_6 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    /// Algorithm 21.6 (Prime Sieve) using ArraySeqPer - simplified version.
    /// Construct primes using a sieve: generate composites, then filter candidates.
    ///
    /// - APAS: Work Θ(n lg n), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n² lg n), Span Θ(n² lg n) — filter uses O(m) linear scan per candidate instead of ninject sieve.
    pub fn prime_sieve(n: N) -> (result: ArraySeqStPerS<N>)
        ensures
            n <= 2 ==> result.spec_len() == 0,
            n > 2  ==> result.spec_len() <= n - 1,
    {
        if n <= 2 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        // cs = 〈 i * j : 2 ≤ i ≤ floor(sqrt(n)) , 2 ≤ j ≤ n/i 〉
        let root: N = n.isqrt();
        let outer_len: N = if root >= 2 { root - 1 } else { 0 };
        let nested: ArraySeqStPerS<ArraySeqStPerS<N>> =
            ArraySeqStPerS::tabulate(
                &(|i0: usize| -> (row: ArraySeqStPerS<N>)
                    requires
                        i0 < outer_len,
                        root as int * root as int <= n as int,
                        n > 2,
                {
                    let i = i0 + 2; // i in [2..=root]
                    let limit = if i == 0 { 0 } else { n / i };
                    let len = if limit >= 2 { limit - 1 } else { 0 };
                    proof {
                        // For all j0 < len: j0+2 <= limit = n/i, so i*(j0+2) <= i*(n/i) <= n
                        assert forall|j0: usize| j0 < len implies
                            #[trigger] (i as int * (j0 as int + 2)) <= n as int by
                        {
                            assert(j0 as int + 2 <= limit as int);
                            assert(i as int * limit as int <= n as int) by (nonlinear_arith)
                                requires limit == n / i, i >= 2;
                            assert(i as int * (j0 as int + 2) <= i as int * limit as int) by (nonlinear_arith)
                                requires j0 as int + 2 <= limit as int, i >= 2;
                        }
                    }
                    ArraySeqStPerS::tabulate(
                        &(|j0: usize| -> (c: N)
                            requires
                                j0 < len,
                                i >= 2, i <= root,
                                limit == n / i,
                                len <= limit,
                                n > 2,
                                i as int * (j0 as int + 2) <= n as int,
                            ensures c == i * (j0 + 2),
                        {
                            i * (j0 + 2)
                        }),
                        len,
                    )
                }),
                outer_len,
            );
        let composites: ArraySeqStPerS<N> = ArraySeqStPerS::flatten(&nested);

        // Create candidates: 2, 3, ..., n
        let candidates: ArraySeqStPerS<N> =
            ArraySeqStPerS::tabulate(
                &(|i: usize| -> (v: N)
                    requires i < n - 1, n > 2,
                    ensures v == i + 2,
                { i + 2 }),
                n - 1,
            );

        // Filter out composites to get primes
        let ghost spec_not_composite: spec_fn(N) -> bool =
            |x: N| !composites.seq@.contains(x);
        let pred = |x: &N| -> (keep: bool)
            ensures keep == !composites.seq@.contains(*x),
        {
            let mut is_composite = false;
            let clen = composites.length();
            let mut idx: usize = 0;
            while idx < clen && !is_composite
                invariant
                    idx <= clen,
                    clen == composites.seq@.len(),
                    is_composite ==> composites.seq@.contains(*x),
                    !is_composite ==> forall|k: int| 0 <= k < idx ==> composites.seq@[k] != *x,
                decreases clen - idx,
            {
                if *composites.nth(idx) == *x {
                    is_composite = true;
                }
                idx += 1;
            }
            // After loop (no break): !(idx < clen && !is_composite).
            // If !is_composite, then idx >= clen. With invariant idx <= clen: idx == clen.
            // So forall|k| 0 <= k < clen ==> composites.seq@[k] != *x, hence !contains.
            !is_composite
        };
        let filtered: ArraySeqStPerS<N> =
            ArraySeqStPerS::filter(
                &candidates,
                &pred,
                Ghost(spec_not_composite),
            );
        filtered
    }

    } // verus!
}
