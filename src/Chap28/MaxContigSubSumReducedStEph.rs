// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT

//! Maximum Contiguous Subsequence Sum — Reduced Force (Chapter 28, Algorithm 28.13).
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


pub mod MaxContigSubSumReducedStEph {
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

    pub trait MaxContigSubSumReducedTrait {
        /// Compute MCSS using reduced force (Algorithm 28.13).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n²), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential
        fn max_contig_sub_sum_reduced(a: &ArraySeqStEphS<i32>) -> (result: Option<i32>)
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

    impl MaxContigSubSumReducedTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_reduced(a: &ArraySeqStEphS<i32>) -> (result: Option<i32>) {
            let n = a.length();

            if n == 0 {
                return None;
            }

            let mut global_max: Option<i32> = None;

            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    n == a.seq@.len(),
                    sums_fit_i32(a.seq@),
                    i == 0 ==> global_max.is_none(),
                    i > 0 ==> global_max.is_some(),
                    i > 0 ==> (forall|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo && lo < i && lo < hi && hi <= n ==>
                            spec_range_sum(a.seq@, lo, hi) <= global_max.unwrap() as int),
                    i > 0 ==> (exists|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo && lo < i && lo < hi && hi <= n &&
                            spec_range_sum(a.seq@, lo, hi) == global_max.unwrap() as int),
                decreases n - i,
            {
                let mut running_sum: i32 = 0;
                let mut j: usize = i;

                while j < n
                    invariant
                        0 <= i < n,
                        i <= j <= n,
                        n == a.seq@.len(),
                        sums_fit_i32(a.seq@),
                        running_sum as int == spec_range_sum(a.seq@, i as int, j as int),
                        j == i && i == 0 ==> global_max.is_none(),
                        (j > i || i > 0) ==> global_max.is_some(),
                        (j > i || i > 0) ==> (forall|lo: int, hi: int|
                            #![trigger spec_range_sum(a.seq@, lo, hi)]
                            (0 <= lo && lo < i as int && lo < hi && hi <= n) ||
                            (lo == i as int && lo < hi && hi <= j as int) ==>
                                spec_range_sum(a.seq@, lo, hi) <= global_max.unwrap() as int),
                        (j > i || i > 0) ==> (exists|lo: int, hi: int|
                            #![trigger spec_range_sum(a.seq@, lo, hi)]
                            ((0 <= lo && lo < i as int && lo < hi && hi <= n) ||
                             (lo == i as int && lo < hi && hi <= j as int)) &&
                                spec_range_sum(a.seq@, lo, hi) == global_max.unwrap() as int),
                    decreases n - j,
                {
                    proof {
                        lemma_range_sum_snoc(a.seq@, i as int, (j + 1) as int);
                    }
                    running_sum = running_sum + *a.nth(j);
                    global_max = max_with_neginf(global_max, Some(running_sum));

                    j = j + 1;
                }

                i = i + 1;
            }

            global_max
        }
    }

    } // verus!
}
