//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.6: Prime Sieve using ArraySeqPer and ninject.
//! Verusified.

pub mod Algorithm21_6 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::N;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    /// Algorithm 21.6 (Prime Sieve) using ArraySeqPer - simplified version.
    /// Construct primes using a sieve: generate composites, then filter candidates.
    ///
    /// APAS: Work Θ(n lg n), Span Θ(lg n)
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
                    ArraySeqStPerS::tabulate(
                        &(|j0: usize| -> (c: N)
                            requires
                                j0 < len,
                                i >= 2, i <= root,
                                limit == n / i,
                                len <= limit,
                                n > 2,
                            ensures c == i * (j0 + 2),
                        {
                            proof {
                                // j0+2 <= limit = n/i, so i*(j0+2) <= i*(n/i) <= n
                                assume(i as int * (j0 as int + 2) <= usize::MAX as int);
                            }
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
        let filtered: ArraySeqStPerS<N> =
            ArraySeqStPerS::filter(
                &candidates,
                &(|x: &N| -> (keep: bool) {
                    // Check if x is NOT in composites
                    let mut is_composite = false;
                    let clen = composites.length();
                    let mut idx: usize = 0;
                    while idx < clen
                        invariant
                            idx <= clen,
                            clen == composites.seq@.len(),
                        decreases clen - idx,
                    {
                        if *composites.nth(idx) == *x {
                            is_composite = true;
                            break;
                        }
                        idx += 1;
                    }
                    !is_composite
                }),
            );
        filtered
    }

    } // verus!
}
