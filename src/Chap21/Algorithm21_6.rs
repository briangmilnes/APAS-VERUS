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

    /// Algorithm 21.6 (Prime Sieve) using ninject-based boolean sieve.
    /// 1. Generate composite numbers via nested tabulate + flatten.
    /// 2. Build boolean sieve array, marking composites false (the ninject step).
    /// 3. Collect indices where sieve is true.
    ///
    /// - APAS: Work Θ(n lg n), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — sequential StPer; O(|composites|) ninject + O(n) collect.
    pub fn prime_sieve(n: N) -> (result: ArraySeqStPerS<N>)
        requires n < usize::MAX,
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
                    let i = i0 + 2;
                    let limit = if i == 0 { 0 } else { n / i };
                    let len = if limit >= 2 { limit - 1 } else { 0 };
                    proof {
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

        // Ninject: build boolean sieve of size n+1, mark composites false.
        let mut sieve: Vec<bool> = vec![true; n + 1];
        let clen = composites.length();
        let mut ci: usize = 0;
        while ci < clen
            invariant
                ci <= clen,
                clen == composites.seq@.len(),
                sieve@.len() == n + 1,
            decreases clen - ci,
        {
            let c = *composites.nth(ci);
            if c <= n {
                sieve.set(c, false);
            }
            ci = ci + 1;
        }

        // Collect primes: indices 2..=n where sieve[i] is true.
        let mut primes: Vec<N> = Vec::new();
        let mut idx: usize = 2;
        while idx <= n
            invariant
                2 <= idx <= n + 1,
                n < usize::MAX,
                primes@.len() <= idx - 2,
                sieve@.len() == n + 1,
            decreases n - idx + 1,
        {
            if sieve[idx] {
                primes.push(idx);
            }
            idx = idx + 1;
        }
        ArraySeqStPerS::from_vec(primes)
    }

    } // verus!
}
