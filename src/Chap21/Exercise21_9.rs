//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.9: Composite generation proof.
//! Proves that it suffices to consider multiples of numbers up to sqrt(n).

//  Table of Contents
//	1. module
//	6. spec fns
//	7. proof fns/broadcast groups

//		1. module

pub mod Exercise21_9 {

    use vstd::prelude::*;

    verus! {

    //		6. spec fns

    /// A number m > 1 is composite iff it has a divisor d with 2 <= d < m.
    pub open spec fn spec_is_composite(m: int) -> bool {
        m > 1 && exists|d: int| 2 <= d < m && #[trigger] (m % d) == 0
    }

    //		7. proof fns/broadcast groups

    /// If d divides m (m % d == 0) and d > 0, then m == d * (m / d).
    proof fn lemma_div_exact(m: int, d: int)
        requires d > 0, m % d == 0, m >= 0,
        ensures m == d * (m / d),
    {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(m, d);
    }

    /// Every composite m has a divisor d with 2 <= d <= sqrt(m).
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
    proof fn lemma_composite_has_small_divisor(m: int)
        requires
            m > 1,
            spec_is_composite(m),
        ensures
            exists|d: int| 2 <= d && d * d <= m && #[trigger] (m % d) == 0,
    {
        let d0 = choose|d: int| 2 <= d < m && #[trigger] (m % d) == 0;
        lemma_div_exact(m, d0);
        let q = m / d0;
        // m == d0 * q, q >= 1 since m >= 2 and d0 < m
        assert(m == d0 * q);
        assert(q >= 2) by (nonlinear_arith)
            requires m == d0 * q, d0 < m, d0 >= 2, m > 1;
        if d0 * d0 <= m {
            // d0 itself is the small divisor.
        } else {
            // d0 * d0 > m, so q < d0. q is a smaller divisor.
            // If q >= d0, then m == d0 * q >= d0 * d0 > m — contradiction.
            assert(q < d0) by (nonlinear_arith)
                requires d0 * d0 > m, m == d0 * q, d0 >= 2, q >= 2;
            assert(q * q <= m) by (nonlinear_arith)
                requires q < d0, d0 * d0 > m, m == d0 * q, q >= 2;
            // q divides m because m == d0 * q means m % q == 0.
            assert(m % q == 0) by (nonlinear_arith)
                requires m == d0 * q, q >= 2;
        }
    }

    /// Exercise 21.9: To generate all composites in [2, n], it suffices to
    /// consider multiples of i for 2 <= i <= sqrt(n), with multipliers j >= 2
    /// such that i * j <= n.
    ///
    /// Formally: if m is composite and 2 <= m <= n, then there exist i, j with
    /// 2 <= i, i * i <= n, j >= 2, i * j == m.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
    pub proof fn lemma_composites_covered_by_small_multiples(m: int, n: int)
        requires
            n >= 2,
            2 <= m <= n,
            spec_is_composite(m),
        ensures
            exists|i: int, j: int|
                2 <= i && i * i <= n && j >= 2 && #[trigger] (i * j) == m,
    {
        lemma_composite_has_small_divisor(m);
        let d = choose|d: int| 2 <= d && d * d <= m && #[trigger] (m % d) == 0;
        lemma_div_exact(m, d);
        let q = m / d;
        assert(m == d * q);
        // q >= 2 because m == d*q, d >= 2, and m is composite (has divisor < m).
        // If q == 1 then m == d, but d divides m and d < m (from the small divisor
        // being chosen with d*d <= m and m composite implies m >= 4).
        // Actually: d*d <= m and d >= 2 implies m >= 4, and m == d*q with d >= 2
        // and q == 1 means m == d, so d*d <= d, contradiction with d >= 2.
        assert(q >= 2) by (nonlinear_arith)
            requires m == d * q, d >= 2, d * d <= m;
        assert(d * d <= n) by (nonlinear_arith)
            requires d * d <= m, m <= n;
        // Witness: i = d, j = q
        assert(d * q == m);
    }

    } // verus!
}
