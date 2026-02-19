// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT

//! Maximum Contiguous Subsequence Sum — Reduction to MCSSE (Chapter 28, Algorithm 28.14).
//!
//! For each ending position j, compute MCSSE(j) = prefix_sum(j+1) - min_prefix_sum(0..j)
//! using Algorithm 28.12, then take the max over all j.
//!
//! ## Table of Contents
//! 1. imports
//! 2. spec definitions
//! 3. exec functions

//  Table of Contents
//	1. module
//	6. spec fns
//	8. traits
//	9. impls

//		1. module


pub mod MaxContigSubSumReducedMcsseStEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;

    verus! {

    //		6. spec fns

    // ─── 2. spec definitions ───

    pub open spec fn spec_max_opt_i32(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Option::None, Option::None) => Option::None,
            (Option::None, Option::Some(_)) => b,
            (Option::Some(_), Option::None) => a,
            (Option::Some(x), Option::Some(y)) => if x >= y { a } else { b },
        }
    }


    //		8. traits

    pub trait MaxContigSubSumReducedMcsseTrait {
        /// Compute MCSS by enumerating all MCSSE instances (Algorithm 28.14).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work O(n²), Span O(log n)
        /// - Claude-Opus-4.6: Work O(n²), Span O(n²) — sequential
        fn max_contig_sub_sum_reduced_mcsse(a: &ArraySeqStEphS<i32>) -> (result: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
            ensures
                a.seq@.len() == 0 ==> result.is_none(),
                a.seq@.len() > 0 ==> result.is_some(),
                result.is_some() ==> is_mcss_of(a.seq@, result.unwrap() as int);
    }


    //		9. impls

    // ─── 3. exec functions ───

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> (result: Option<i32>)
        ensures result == spec_max_opt_i32(a, b),
    {
        match (a, b) {
            (None, None) => None,
            (None, Some(_)) => b,
            (Some(_), None) => a,
            (Some(x), Some(y)) => if x >= y { a } else { b },
        }
    }

    impl MaxContigSubSumReducedMcsseTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_reduced_mcsse(a: &ArraySeqStEphS<i32>) -> (result: Option<i32>) {
            let n = a.length();
            if n == 0 {
                return None;
            }

            let mut global_max: Option<i32> = None;

            let mut j: usize = 0;
            while j < n
                invariant
                    0 <= j <= n,
                    n == a.seq@.len(),
                    sums_fit_i32(a.seq@),
                    j == 0 ==> global_max.is_none(),
                    j > 0 ==> global_max.is_some(),
                    j > 0 ==> (forall|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo < hi <= j ==>
                            spec_range_sum(a.seq@, lo, hi) <= global_max.unwrap() as int),
                    j > 0 ==> (exists|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo < hi <= j &&
                            spec_range_sum(a.seq@, lo, hi) == global_max.unwrap() as int),
                decreases n - j,
            {
                // Compute MCSSE at ending position j using Algorithm 28.12:
                // prefix sums p[k] = range_sum(a, 0, k), min_p = min(p[0], ..., p[j])
                let mut running: i32 = 0;
                let mut min_p: i32 = 0;

                let mut k: usize = 0;
                while k < j
                    invariant
                        0 <= k <= j,
                        j < n,
                        n == a.seq@.len(),
                        sums_fit_i32(a.seq@),
                        running as int == spec_prefix_sum(a.seq@, k as int),
                        min_p as int == spec_min_prefix_sum(a.seq@, k as int),
                    decreases j - k,
                {
                    proof {
                        lemma_range_sum_snoc(a.seq@, 0, (k + 1) as int);
                    }
                    running = running + *a.nth(k);

                    proof {
                        reveal_with_fuel(spec_min_prefix_sum, 2);
                    }
                    if running < min_p {
                        min_p = running;
                    }

                    k = k + 1;
                }

                // total = prefix_sum(j+1), min_p = min_prefix_sum(j)
                proof {
                    lemma_range_sum_snoc(a.seq@, 0, (j + 1) as int);
                }
                let total = running + *a.nth(j);

                proof {
                    lemma_min_prefix_sum_achieved(a.seq@, j as int);
                    let w = choose|w: int|
                        #![trigger spec_prefix_sum(a.seq@, w)]
                        0 <= w <= j as int &&
                            spec_prefix_sum(a.seq@, w) == spec_min_prefix_sum(a.seq@, j as int);
                    lemma_range_sum_via_prefix(a.seq@, w, (j + 1) as int);
                    assert(total as int - min_p as int ==
                        spec_range_sum(a.seq@, w, (j + 1) as int));
                }
                let mcsse_j = total - min_p;

                proof {
                    // mcsse_j == prefix_sum(j+1) - min_prefix_sum(j)
                    // Show mcsse_j >= all range_sum(a, lo, j+1) for lo in 0..=j
                    assert forall|lo: int|
                        #![trigger spec_range_sum(a.seq@, lo, (j + 1) as int)]
                        0 <= lo <= j
                    implies
                        spec_range_sum(a.seq@, lo, (j + 1) as int) <= mcsse_j as int
                    by {
                        lemma_range_sum_via_prefix(a.seq@, lo, (j + 1) as int);
                        lemma_min_prefix_sum_is_min(a.seq@, j as int, lo);
                    };

                    // Show mcsse_j is achieved
                    let witness = choose|w: int|
                        #![trigger spec_prefix_sum(a.seq@, w)]
                        0 <= w <= j as int &&
                            spec_prefix_sum(a.seq@, w) == spec_min_prefix_sum(a.seq@, j as int);
                    lemma_range_sum_via_prefix(a.seq@, witness, (j + 1) as int);
                    assert(spec_range_sum(a.seq@, witness, (j + 1) as int) == mcsse_j as int);
                }

                global_max = max_with_neginf(global_max, Some(mcsse_j));

                proof {
                    // Extend forall to cover ranges ending at j+1
                    if j > 0 {
                        assert forall|lo: int, hi: int|
                            #![trigger spec_range_sum(a.seq@, lo, hi)]
                            0 <= lo < hi <= (j + 1) as int
                        implies
                            spec_range_sum(a.seq@, lo, hi) <= global_max.unwrap() as int
                        by {
                            if hi <= j as int {
                                // Covered by outer loop invariant (previous iterations)
                            } else {
                                // hi == j+1
                                assert(spec_range_sum(a.seq@, lo, hi) <= mcsse_j as int);
                            }
                        };
                    }
                }

                j = j + 1;
            }

            global_max
        }
    }

    } // verus!
}
