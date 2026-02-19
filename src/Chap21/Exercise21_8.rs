// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.8 / Algorithm 21.4: Brute Force Primality Test (isPrime).
//! - Uses tabulate + filter per the textbook.
//! - Fully verified: no proof holes.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	6. spec fns
//	7. proof fns/broadcast groups
//	9. impls

//		1. module

pub mod Exercise21_8 {

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::multiset::multiset::spec_filter_len;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    //		6. spec fns

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

    //		7. proof fns/broadcast groups

    // Lemma: if divisor count is 0 in [from, to), then no element in that range divides n.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
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
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
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
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
    proof fn lemma_divisor_count_nonneg(n: int, from: int, to: int)
        ensures spec_divisor_count(n, from, to) >= 0,
        decreases (if to > from { to - from } else { 0 }),
    {
        if from < to {
            lemma_divisor_count_nonneg(n, from + 1, to);
        }
    }

    // Bridge: spec_filter_len over a boolean divisibility sequence equals spec_divisor_count.
    proof fn lemma_filter_len_eq_divisor_count(n: int, k: int)
        requires k >= 0, n >= 2,
        ensures spec_filter_len(
            Seq::new(k as nat, |i: int| (n % (i + 1) == 0)),
            |v: B| v,
        ) == spec_divisor_count(n, 1, k + 1),
        decreases k,
    {
        if k > 0 {
            lemma_filter_len_eq_divisor_count(n, k - 1);
            let s = Seq::new(k as nat, |i: int| (n % (i + 1) == 0));
            let s_prev = Seq::new((k - 1) as nat, |i: int| (n % (i + 1) == 0));
            let pred = |v: B| v;
            assert(s.drop_last() =~= s_prev);
            assert(s.last() == (n % (k as int) == 0));

            // Unfold spec_divisor_count from the top: range [1, k+1) = [1, k) ++ {k}
            // spec_divisor_count(n, 1, k+1) steps down from k to k-1 at the end
            // We need to relate the recursive unfolding.
            // Actually spec_divisor_count unfolds from `from` upward, while spec_filter_len
            // unfolds from the end downward. Let's use a helper that peels from the end.
            lemma_divisor_count_split_last(n, 1, k + 1);
        }
    }

    // Split the last element off spec_divisor_count: count(n, from, to) = count(n, from, to-1) + last.
    proof fn lemma_divisor_count_split_last(n: int, from: int, to: int)
        requires from > 0, from < to,
        ensures spec_divisor_count(n, from, to) ==
            spec_divisor_count(n, from, to - 1) +
            if n % (to - 1) == 0 { 1int } else { 0int },
        decreases to - from,
    {
        if from == to - 1 {
            // Range [from, from+1). count(n, from, from+1) unfolds to:
            //   if n%from==0 { 1 + count(n, from+1, from+1) } else { count(n, from+1, from+1) }
            // count(n, from+1, from+1) = 0 (base case from >= to).
            // count(n, from, from) = 0 (base case).
            // So count(n, from, from+1) = (if n%from==0 {1} else {0}) + 0.
            assert(spec_divisor_count(n, from + 1, to) == 0);
            assert(spec_divisor_count(n, from, to - 1) == 0);
        } else {
            lemma_divisor_count_split_last(n, from + 1, to);
            // IH: count(n, from+1, to) = count(n, from+1, to-1) + last
            // count(n, from, to) = head + count(n, from+1, to)   [by definition]
            //                    = head + count(n, from+1, to-1) + last  [by IH]
            // count(n, from, to-1) = head + count(n, from+1, to-1)  [by definition, since from < to-1]
            // Therefore count(n, from, to) = count(n, from, to-1) + last.
        }
    }

    //		9. impls

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
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
    /// - Claude-Opus-4.6: Work Θ(√n), Span Θ(√n) — sequential StEph tabulate + filter.
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
        let pred = |x: &B| -> (keep: bool) ensures keep == *x { *x };
        let ghost spec_pred = |v: B| v;
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
            // k = isqrt(n) >= 1 when n >= 2: if k == 0 then (k+1)^2 = 1 > n, contradicting n >= 2.
            assert(1 < k as int + 1) by (nonlinear_arith)
                requires n >= 2int, k as int * k as int <= n as int,
                         (k as int + 1) * (k as int + 1) > n as int;
            assert(n as int % 1 == 0);

            // Unfold: spec_divisor_count(n, 1, k+1) = 1 + spec_divisor_count(n, 2, k+1)
            assert(spec_divisor_count(n as int, 1, k as int + 1) ==
                   1 + spec_divisor_count(n as int, 2, k as int + 1));

            lemma_divisor_count_nonneg(n as int, 2, k as int + 1);

            // Bridge filter length to divisor count via the recursive lemma.
            lemma_filter_len_eq_divisor_count(n as int, k as int);
            // The filter ensures ones.spec_len() == spec_filter_len(Seq::new(k, |i| all@[i]), id).
            // tabulate ensures all.seq@[i] == (n%(i+1)==0), so the sequences match.
            assert(Seq::new(all.seq@.len(), |i: int| all.seq@[i])
                =~= Seq::new(k as nat, |i: int| (n as int % (i + 1) == 0)));

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
