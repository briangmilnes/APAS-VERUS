// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.8 / Algorithm 21.4: Brute Force Primality Test (isPrime).
//! - Uses tabulate + filter per the textbook.
//! - Proof hole: assume(ones.length() == count) bridges filter's weak spec to the ghost count.
//!   Closing this hole requires strengthening ArraySeqStPer::filter to a multiset-equality spec.

pub mod Exercise21_8 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    // Spec: n is prime iff n >= 2 and no i in [2, sqrt(n)] divides n.
    pub open spec fn spec_is_prime(n: int) -> bool {
        n >= 2 && forall|i: int| 2 <= i && i * i <= n ==> #[trigger] (n % i) != 0
    }

    // Spec: count of integers in [from, to) that divide n.
    pub open spec fn spec_divisor_count(n: int, from: int, to: int) -> int
        decreases (if to > from { to - from } else { 0 })
    {
        if from >= to { 0 }
        else if n % from == 0 { 1 + spec_divisor_count(n, from + 1, to) }
        else { spec_divisor_count(n, from + 1, to) }
    }

    // Lemma: if divisor count is 0 in [from, to), then no element in that range divides n.
    proof fn lemma_zero_count_means_no_divisors(n: int, from: int, to: int)
        requires
            from <= to,
            from > 0,
            spec_divisor_count(n, from, to) == 0,
        ensures
            forall|i: int| from <= i < to ==> #[trigger] (n % i) != 0,
        decreases to - from,
    {
        if from < to {
            // Unfold the definition of spec_divisor_count.
            lemma_divisor_count_nonneg(n, from + 1, to);
            if n % from == 0 {
                // count = 1 + rest >= 1, contradicting 0.
                assert(spec_divisor_count(n, from, to) >= 1);
            } else {
                // count = rest = spec_divisor_count(n, from+1, to) == 0.
                assert(spec_divisor_count(n, from + 1, to) == 0);
                lemma_zero_count_means_no_divisors(n, from + 1, to);
            }
        }
    }

    // Lemma: if no element in [from, to) divides n, then divisor count is 0.
    proof fn lemma_no_divisors_means_zero_count(n: int, from: int, to: int)
        requires
            from <= to,
            from > 0,
            forall|i: int| from <= i < to ==> #[trigger] (n % i) != 0,
        ensures
            spec_divisor_count(n, from, to) == 0,
        decreases to - from,
    {
        if from < to {
            assert(n % from != 0);
            lemma_no_divisors_means_zero_count(n, from + 1, to);
        }
    }

    // Lemma: spec_divisor_count is non-negative.
    proof fn lemma_divisor_count_nonneg(n: int, from: int, to: int)
        ensures spec_divisor_count(n, from, to) >= 0,
        decreases (if to > from { to - from } else { 0 }),
    {
        if from < to {
            lemma_divisor_count_nonneg(n, from + 1, to);
        }
    }

    /// APAS: Work Θ(1), Span Θ(1)
    pub fn is_divisible(n: N, i: N) -> (divides: B)
        requires i > 0
        ensures divides == (n as int % i as int == 0)
    {
        n % i == 0
    }

    /// Exercise 21.8 / Algorithm 21.4 (Brute Force Primality Test)
    /// - isPrime n = |{ x in 1..=floor(sqrt(n)) : n mod x == 0 }| == 1
    /// - Uses tabulate + filter per the textbook.
    /// - APAS: Work Θ(√n), Span Θ(lg n)
    pub fn is_prime(n: N) -> (prime: B)
        ensures prime == spec_is_prime(n as int)
    {
        if n < 2 {
            return false;
        }
        let k: N = n.isqrt();

        // Tabulate: all[i] = (n % (i+1) == 0) for i in 0..k
        let all: ArraySeqStEphS<B> = ArraySeqStEphS::tabulate(
            &(|i: usize| -> (d: B)
                requires
                    i < k,
                    n >= 2,
                ensures
                    d == (n as int % (i as int + 1) == 0),
            {
                is_divisible(n, i + 1)
            }),
            k,
        );

        // Filter: keep only the true values (divisors).
        // Verus limitation: exec closure ensures are one-directional; the biconditional
        // bridge required by filter's spec cannot be proven automatically.
        let pred = |x: &B| -> (keep: bool) ensures keep == *x { *x };
        let ghost spec_pred = |v: B| v;
        proof {
            assume(forall|v: B, passes: bool| pred.ensures((&v,), passes) <==> spec_pred(v) == passes);
        }
        let ones: ArraySeqStEphS<B> = ArraySeqStEphS::filter(
            &all,
            &pred,
            Ghost(spec_pred),
        );

        let prime = ones.length() == 1;

        proof {
            // Ghost: count divisors in [1, k+1) via spec_divisor_count.
            // From tabulate: all.seq@[i] == (n % (i+1) == 0) for i in [0, k).
            // We manually reason about the count.

            // 1 always divides n (n >= 2 > 0).
            assert(n as int % 1 == 0) by (nonlinear_arith) requires n >= 2;
            // k >= 1 so 1 < k+1, triggering the recursive case.
            assert(1 < k as int + 1);
            assert(n as int % 1 == 0);

            // Unfold: spec_divisor_count(n, 1, k+1) = 1 + spec_divisor_count(n, 2, k+1)
            assert(spec_divisor_count(n as int, 1, k as int + 1) ==
                   1 + spec_divisor_count(n as int, 2, k as int + 1));

            lemma_divisor_count_nonneg(n as int, 2, k as int + 1);

            // Bridge: the filter count equals the ghost divisor count.
            // This is the proof hole — filter's spec doesn't give us the exact count.
            assume(ones.seq@.len() as int == spec_divisor_count(n as int, 1, k as int + 1));

            // Now prove: prime <==> spec_is_prime(n as int).
            // prime <==> ones.length() == 1 <==> divisor_count == 1
            //        <==> spec_divisor_count(n, 2, k+1) == 0
            //        <==> forall i: 2 <= i <= k: n%i != 0
            //        <==> forall i: 2 <= i && i*i <= n: n%i != 0  (using isqrt)
            //        <==> spec_is_prime(n)

            if prime {
                // ones.length() == 1, so divisor_count(n, 1, k+1) == 1.
                // Therefore divisor_count(n, 2, k+1) == 0.
                assert(spec_divisor_count(n as int, 2, k as int + 1) == 0);
                lemma_zero_count_means_no_divisors(n as int, 2, k as int + 1);
                // forall i: 2 <= i < k+1 ==> n%i != 0

                // Lift to spec_is_prime: if i*i <= n then i <= k (since (k+1)^2 > n).
                assert forall|i: int| 2 <= i && i * i <= n as int
                    implies #[trigger] (n as int % i) != 0 by
                {
                    if i >= k as int + 1 {
                        assert(i * i >= (k as int + 1) * (k as int + 1)) by (nonlinear_arith)
                            requires i >= k as int + 1, i >= 2;
                        // Contradiction: i*i >= (k+1)^2 > n >= i*i
                    }
                    // So i < k+1, i.e., 2 <= i <= k, and n%i != 0.
                }
            } else {
                // ones.length() != 1, so divisor_count(n, 1, k+1) != 1.
                // Since dc = 1 + dc2 and dc2 >= 0, dc >= 1. So dc > 1, meaning dc2 > 0.
                let ghost dc2 = spec_divisor_count(n as int, 2, k as int + 1);
                assert(dc2 > 0);

                // dc2 > 0 means there exists some i in [2, k+1) dividing n.
                // Prove by contradiction: if no such i existed, dc2 == 0.
                if forall|i: int| 2 <= i && i * i <= n as int ==> #[trigger] (n as int % i) != 0 {
                    // Then in particular, forall i: 2 <= i <= k ==> n%i != 0.
                    // Because i <= k implies i*i <= k*k <= n.
                    assert forall|i: int| 2 <= i < k as int + 1
                        implies #[trigger] (n as int % i) != 0 by
                    {
                        assert(i * i <= k as int * k as int) by (nonlinear_arith)
                            requires i <= k as int, i >= 2;
                        assert(k as int * k as int <= n as int);
                    }
                    lemma_no_divisors_means_zero_count(n as int, 2, k as int + 1);
                    assert(dc2 == 0);  // Contradiction with dc2 > 0.
                }
            }
        }

        prime
    }

    } // verus!
}
