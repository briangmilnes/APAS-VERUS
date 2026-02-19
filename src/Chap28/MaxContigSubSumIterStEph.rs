// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT

//! Maximum Contiguous Subsequence Sum — Kadane's Iterative (Chapter 28, Algorithm 28.15).
//!
//! ## Table of Contents
//! 1. imports
//! 2. spec definitions
//! 3. proof lemmas
//! 4. exec functions

//  Table of Contents
//	1. module
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls

//		1. module


pub mod MaxContigSubSumIterStEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;

    verus! {

    //		6. spec fns

    // ─── 2. spec definitions ───

    /// Kadane recurrence: maximum sum of any contiguous subsequence ending at
    /// position j (i.e., the range ends at j+1 exclusive).
    pub open spec fn spec_max_ending_at(s: Seq<i32>, j: int) -> int
        decreases j,
    {
        if j <= 0 {
            s[0] as int
        } else {
            let prev = spec_max_ending_at(s, j - 1) + s[j] as int;
            let fresh = s[j] as int;
            if prev >= fresh { prev } else { fresh }
        }
    }

    // ─── 4. exec functions ───

    pub open spec fn spec_max_opt_i32(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Option::None, Option::None) => Option::None,
            (Option::None, Option::Some(_)) => b,
            (Option::Some(_), Option::None) => a,
            (Option::Some(x), Option::Some(y)) => if x >= y { a } else { b },
        }
    }


    //		7. proof fns/broadcast groups

    // ─── 3. proof lemmas ───

    /// spec_max_ending_at(s, j) is an upper bound on all contiguous subsequence
    /// sums ending at position j+1 (exclusive).
    proof fn lemma_max_ending_at_is_max(s: Seq<i32>, j: int, lo: int)
        requires
            0 <= lo <= j,
            j < s.len(),
        ensures
            spec_range_sum(s, lo, j + 1) <= spec_max_ending_at(s, j),
        decreases j,
    {
        reveal_with_fuel(spec_max_ending_at, 2);
        if j == 0 {
            lemma_range_sum_single(s, 0);
        } else {
            lemma_range_sum_snoc(s, lo, j + 1);
            if lo == j {
                lemma_range_sum_single(s, j);
            } else {
                lemma_max_ending_at_is_max(s, j - 1, lo);
            }
        }
    }

    /// spec_max_ending_at(s, j) is achieved by some contiguous subsequence
    /// ending at position j+1 (exclusive).
    proof fn lemma_max_ending_at_achieved(s: Seq<i32>, j: int)
        requires
            0 <= j < s.len(),
        ensures
            exists|lo: int|
                #![trigger spec_range_sum(s, lo, j + 1)]
                0 <= lo <= j && spec_range_sum(s, lo, j + 1) == spec_max_ending_at(s, j),
        decreases j,
    {
        reveal_with_fuel(spec_max_ending_at, 2);
        if j == 0 {
            lemma_range_sum_single(s, 0);
            assert(spec_range_sum(s, 0, 1) == spec_max_ending_at(s, 0));
        } else {
            lemma_max_ending_at_achieved(s, j - 1);
            let prev = spec_max_ending_at(s, j - 1) + s[j] as int;
            let fresh = s[j] as int;
            if prev >= fresh {
                let lo_prev = choose|lo: int|
                    #![trigger spec_range_sum(s, lo, j)]
                    0 <= lo <= j - 1 && spec_range_sum(s, lo, j) == spec_max_ending_at(s, j - 1);
                lemma_range_sum_snoc(s, lo_prev, j + 1);
            } else {
                lemma_range_sum_single(s, j);
            }
        }
    }


    //		8. traits

    pub trait MaxContigSubSumIterTrait {
        /// Compute MCSS using Kadane's iterative algorithm (Algorithm 28.15).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        fn max_contig_sub_sum_iter(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
            ensures
                a.seq@.len() == 0 ==> mcss.is_none(),
                a.seq@.len() > 0 ==> mcss.is_some(),
                mcss.is_some() ==> is_mcss_of(a.seq@, mcss.unwrap() as int);
    }


    //		9. impls

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> (max: Option<i32>)
        ensures max == spec_max_opt_i32(a, b),
    {
        match (a, b) {
            (None, None) => None,
            (None, Some(_)) => b,
            (Some(_), None) => a,
            (Some(x), Some(y)) => if x >= y { a } else { b },
        }
    }

    impl MaxContigSubSumIterTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_iter(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>) {
            let n = a.length();
            if n == 0 {
                return None;
            }

            let mut ending_max: i32 = *a.nth(0);
            let mut global_max: i32 = ending_max;

            proof {
                lemma_range_sum_single(a.seq@, 0);
                reveal_with_fuel(spec_max_ending_at, 1);
            }

            let mut i: usize = 1;
            while i < n
                invariant
                    1 <= i <= n,
                    n == a.seq@.len(),
                    n > 0,
                    sums_fit_i32(a.seq@),
                    // ending_max tracks the Kadane recurrence
                    ending_max as int == spec_max_ending_at(a.seq@, (i - 1) as int),
                    // global_max >= all range sums ending before position i
                    forall|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo < hi <= i ==>
                            spec_range_sum(a.seq@, lo, hi) <= global_max as int,
                    // global_max is achievable
                    exists|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo < hi <= i &&
                            spec_range_sum(a.seq@, lo, hi) == global_max as int,
                decreases n - i,
            {
                let val = *a.nth(i);

                proof {
                    lemma_max_ending_at_achieved(a.seq@, (i - 1) as int);
                    let lo_witness = choose|lo: int|
                        #![trigger spec_range_sum(a.seq@, lo, i as int)]
                        0 <= lo <= (i - 1) as int &&
                            spec_range_sum(a.seq@, lo, i as int) == spec_max_ending_at(a.seq@, (i - 1) as int);
                    lemma_range_sum_snoc(a.seq@, lo_witness, (i + 1) as int);
                }

                let new_ending = ending_max + val;
                if new_ending >= val {
                    ending_max = new_ending;
                } else {
                    ending_max = val;
                }

                proof {
                    reveal_with_fuel(spec_max_ending_at, 2);
                    // ending_max now == spec_max_ending_at(a.seq@, i as int)

                    lemma_max_ending_at_achieved(a.seq@, i as int);
                    // Prove forall for ranges ending at i+1
                    assert forall|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo < hi <= (i + 1)
                    implies
                        spec_range_sum(a.seq@, lo, hi) <= if ending_max > global_max { ending_max as int } else { global_max as int }
                    by {
                        if hi <= i as int {
                            // Covered by previous invariant
                        } else {
                            // hi == i + 1, so this range ends at position i
                            lemma_max_ending_at_is_max(a.seq@, i as int, lo);
                        }
                    };
                }

                if ending_max > global_max {
                    global_max = ending_max;
                    proof {
                        lemma_max_ending_at_achieved(a.seq@, i as int);
                        let lo_new = choose|lo: int|
                            #![trigger spec_range_sum(a.seq@, lo, (i as int) + 1)]
                            0 <= lo <= i as int &&
                                spec_range_sum(a.seq@, lo, (i as int) + 1) == spec_max_ending_at(a.seq@, i as int);
                        assert(spec_range_sum(a.seq@, lo_new, (i + 1) as int) == global_max as int);
                    }
                }

                i = i + 1;
            }

            Some(global_max)
        }
    }

    } // verus!
}
