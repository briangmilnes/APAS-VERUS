// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Maximum Contiguous Subsequence Sum — Brute Force (Chapter 28, Algorithm 28.8).
//!
//! ## Table of Contents
//! 1. imports
//! 2. spec definitions
//! 3. exec functions
//! 4. proof functions

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls

//		Section 1. module


pub mod MaxContigSubSumBruteStEph {

    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;

    verus! 
{

    //		Section 6. spec fns


    // ─── 2. spec definitions ───

    /// Spec-level max of two Option<i32> values viewed as int.
    /// None represents -infinity.
    pub open spec fn spec_max_opt_i32(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Option::None, Option::None) => Option::None,
            (Option::None, Option::Some(_)) => b,
            (Option::Some(_), Option::None) => a,
            (Option::Some(x), Option::Some(y)) => if x >= y { a } else { b },
        }
    }

    //		Section 8. traits


    /// Trait for brute force maximum contiguous subsequence sum.
    pub trait MaxContigSubSumBruteTrait {
        /// Compute MCSS using brute force (Algorithm 28.8).
        /// Returns None for empty sequence (representing -infinity).
        /// - Alg Analysis: APAS (Ch28 Alg 28.6): Work O(n^3), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — ACCEPTED DIFFERENCE: triple-nested sequential loops
        fn max_contig_sub_sum_brute(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
            ensures
                a.seq@.len() == 0 ==> mcss.is_none(),
                a.seq@.len() > 0 ==> mcss.is_some(),
                mcss.is_some() ==> is_mcss_of(a.seq@, mcss.unwrap() as int);
    }

    //		Section 9. impls


    // ─── 3. exec functions ───

    /// - Alg Analysis: APAS (Ch28 Alg 28.6): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    // veracity: no_requires
    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> (max: Option<i32>)
        ensures
            max == spec_max_opt_i32(a, b),
    {
        match (a, b) {
            (None, None) => None,
            (None, Some(_)) => b,
            (Some(_), None) => a,
            (Some(x), Some(y)) => if x >= y { a } else { b },
        }
    }

    impl MaxContigSubSumBruteTrait for ArraySeqStEphS<i32> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — nested loops over all (lo,hi) pairs; St sequential.
        fn max_contig_sub_sum_brute(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>) {
            let n = a.length();

            if n == 0 {
                return None;
            }

            let mut max_sum: Option<i32> = None;

            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    n == a.seq@.len(),
                    sums_fit_i32(a.seq@),
                    i == 0 ==> max_sum.is_none(),
                    i > 0 ==> max_sum.is_some(),
                    i > 0 ==> (forall|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo && lo < i && lo < hi && hi <= n ==>
                            spec_range_sum(a.seq@, lo, hi) <= max_sum.unwrap() as int),
                    i > 0 ==> (exists|lo: int, hi: int|
                        #![trigger spec_range_sum(a.seq@, lo, hi)]
                        0 <= lo && lo < i && lo < hi && hi <= n &&
                            spec_range_sum(a.seq@, lo, hi) == max_sum.unwrap() as int),
                decreases n - i,
            {
                let mut j: usize = i;
                let mut sum: i32 = 0;

                while j < n
                    invariant
                        0 <= i < n,
                        i <= j <= n,
                        n == a.seq@.len(),
                        sums_fit_i32(a.seq@),
                        sum as int == spec_range_sum(a.seq@, i as int, j as int),
                        j == i && i == 0 ==> max_sum.is_none(),
                        (j > i || i > 0) ==> max_sum.is_some(),
                        (j > i || i > 0) ==> (forall|lo: int, hi: int|
                            #![trigger spec_range_sum(a.seq@, lo, hi)]
                            (0 <= lo && lo < i as int && lo < hi && hi <= n) ||
                            (lo == i as int && lo < hi && hi <= j as int) ==>
                                spec_range_sum(a.seq@, lo, hi) <= max_sum.unwrap() as int),
                        (j > i || i > 0) ==> (exists|lo: int, hi: int|
                            #![trigger spec_range_sum(a.seq@, lo, hi)]
                            ((0 <= lo && lo < i as int && lo < hi && hi <= n) ||
                             (lo == i as int && lo < hi && hi <= j as int)) &&
                                spec_range_sum(a.seq@, lo, hi) == max_sum.unwrap() as int),
                    decreases n - j,
                {
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_range_sum_snoc(a.seq@, i as int, (j + 1) as int);
                    }
                    sum = sum + *a.nth(j);

                    max_sum = max_with_neginf(max_sum, Some(sum));

                    j = j + 1;
                }

                i = i + 1;
            }

            max_sum
        }
    }

    } // verus!
}
