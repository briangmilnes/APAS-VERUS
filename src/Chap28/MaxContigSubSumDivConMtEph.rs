// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT
//! Maximum Contiguous Subsequence Sum — Parallel Divide and Conquer (Chapter 28, Algorithm 28.17).
//!
//! Verified sequential impl under verus_keep_ghost; parallel impl at runtime.

pub mod MaxContigSubSumDivConMtEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap28::MaxContigSubSumDivConStEph::MaxContigSubSumDivConStEph::lemma_divcon_combine;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    #[cfg(not(verus_keep_ghost))]
    use crate::ParaPair;

    pub type T = ArraySeqMtEphS<i32>;

    verus! {

    pub open spec fn spec_max_opt_i32(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Option::None, Option::None) => Option::None,
            (Option::None, Option::Some(_)) => b,
            (Option::Some(_), Option::None) => a,
            (Option::Some(x), Option::Some(y)) => if x >= y { a } else { b },
        }
    }

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

    #[cfg(verus_keep_ghost)]
    fn max_suffix_sum(a: &ArraySeqMtEphS<i32>) -> (result: i32)
        requires a.seq@.len() > 0, sums_fit_i32(a.seq@),
        ensures is_max_suffix_sum(a.seq@, result as int),
    {
        let n = a.length();
        let mut running_sum: i32 = 0;
        let mut min_prefix: i32 = 0;
        let mut i: usize = 0;
        while i < n
            invariant
                0 <= i <= n,
                n as int == a.seq@.len(),
                sums_fit_i32(a.seq@),
                running_sum as int == spec_prefix_sum(a.seq@, i as int),
                (forall|j: int|
                    #![trigger spec_prefix_sum(a.seq@, j)]
                    0 <= j < i || j == 0 ==> min_prefix as int <= spec_prefix_sum(a.seq@, j)),
                (exists|j: int|
                    #![trigger spec_prefix_sum(a.seq@, j)]
                    0 <= j <= i && j < n && min_prefix as int == spec_prefix_sum(a.seq@, j))
                    || (i == 0 && min_prefix == 0),
            decreases n - i,
        {
            if running_sum < min_prefix { min_prefix = running_sum; }
            proof { lemma_range_sum_snoc(a.seq@, 0, (i + 1) as int); }
            running_sum = running_sum + *a.nth(i);
            i = i + 1;
        }

        proof {
            let total = running_sum as int;
            if n > 0 {
                let lo_w: int = choose|j: int|
                    #![trigger spec_prefix_sum(a.seq@, j)]
                    0 <= j < n as int && min_prefix as int == spec_prefix_sum(a.seq@, j);
                lemma_range_sum_via_prefix(a.seq@, lo_w, n as int);
                assert(spec_range_sum(a.seq@, lo_w, a.seq@.len() as int) == total - min_prefix as int);
            }
            assert forall|lo: int|
                #![trigger spec_range_sum(a.seq@, lo, a.seq@.len() as int)]
                0 <= lo < a.seq@.len() as int
            implies
                spec_range_sum(a.seq@, lo, a.seq@.len() as int) <= (running_sum - min_prefix) as int
            by {
                lemma_range_sum_via_prefix(a.seq@, lo, n as int);
            };
        }
        running_sum - min_prefix
    }

    #[cfg(verus_keep_ghost)]
    fn max_prefix_sum(a: &ArraySeqMtEphS<i32>) -> (result: i32)
        requires a.seq@.len() > 0, sums_fit_i32(a.seq@),
        ensures is_max_prefix_sum(a.seq@, result as int),
    {
        let n = a.length();
        let mut max_val: i32 = *a.nth(0);
        let mut running_sum: i32 = *a.nth(0);
        proof { lemma_range_sum_single(a.seq@, 0); }
        let mut i: usize = 1;
        while i < n
            invariant
                1 <= i <= n,
                n as int == a.seq@.len(),
                sums_fit_i32(a.seq@),
                running_sum as int == spec_prefix_sum(a.seq@, i as int),
                (forall|j: int|
                    #![trigger spec_range_sum(a.seq@, 0, j)]
                    1 <= j <= i ==> max_val as int >= spec_range_sum(a.seq@, 0, j)),
                (exists|j: int|
                    #![trigger spec_range_sum(a.seq@, 0, j)]
                    1 <= j <= i && max_val as int == spec_range_sum(a.seq@, 0, j)),
            decreases n - i,
        {
            proof { lemma_range_sum_snoc(a.seq@, 0, (i + 1) as int); }
            running_sum = running_sum + *a.nth(i);
            if running_sum > max_val { max_val = running_sum; }
            i = i + 1;
        }
        max_val
    }

    pub trait MaxContigSubSumDivConMtTrait {
        /// Compute MCSS using parallel divide-and-conquer (Algorithm 28.17).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n log n), Span Θ(log² n)
        /// - Claude-Opus-4.6 (verified): Work Θ(n log n), Span Θ(n log n)
        fn max_contig_sub_sum_divcon_mt(a: &ArraySeqMtEphS<i32>) -> (result: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
                obeys_feq_clone::<i32>(),
            ensures
                a.seq@.len() == 0 ==> result.is_none(),
                a.seq@.len() > 0 ==> result.is_some(),
                result.is_some() ==> is_mcss_of(a.seq@, result.unwrap() as int);
    }

    #[cfg(verus_keep_ghost)]
    impl MaxContigSubSumDivConMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_divcon_mt(a: &ArraySeqMtEphS<i32>) -> (result: Option<i32>)
            decreases a.seq@.len(),
        {
            let n = a.length();
            if n == 0 { return None; }
            if n == 1 {
                proof { lemma_range_sum_single(a.seq@, 0); }
                return Some(*a.nth(0));
            }

            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            proof {
                assert forall|i: int| #![trigger left.seq@[i]]
                    0 <= i < left.seq@.len() implies left.seq@[i] == a.seq@[i]
                by { assert(left.spec_index(i) == a.spec_index(0 + i)); };
                lemma_sums_fit_subseq(a.seq@, left.seq@, 0);

                assert forall|i: int| #![trigger right.seq@[i]]
                    0 <= i < right.seq@.len() implies right.seq@[i] == a.seq@[mid as int + i]
                by { assert(right.spec_index(i) == a.spec_index(mid as int + i)); };
                lemma_sums_fit_subseq(a.seq@, right.seq@, mid as int);
            }

            let max_left = Self::max_contig_sub_sum_divcon_mt(&left);
            let max_right = Self::max_contig_sub_sum_divcon_mt(&right);
            let s_left = max_suffix_sum(&left);
            let p_right = max_prefix_sum(&right);

            proof {
                let lo_s: int = choose|lo: int|
                    #![trigger spec_range_sum(left.seq@, lo, left.seq@.len() as int)]
                    0 <= lo < left.seq@.len() as int &&
                    spec_range_sum(left.seq@, lo, left.seq@.len() as int) == s_left as int;
                let hi_p: int = choose|hi: int|
                    #![trigger spec_range_sum(right.seq@, 0, hi)]
                    1 <= hi <= right.seq@.len() as int &&
                    spec_range_sum(right.seq@, 0, hi) == p_right as int;
                lemma_range_sum_subseq(a.seq@, left.seq@, 0, lo_s, mid as int);
                lemma_range_sum_subseq(a.seq@, right.seq@, mid as int, 0, hi_p);
                lemma_range_sum_split(a.seq@, lo_s, mid as int, mid as int + hi_p);
            }

            let max_crossing: i32 = s_left + p_right;
            let result = max_with_neginf(max_left, max_right);
            let result = max_with_neginf(result, Some(max_crossing));

            proof {
                lemma_divcon_combine(
                    a.seq@, left.seq@, right.seq@, mid as int,
                    max_left.unwrap() as int, max_right.unwrap() as int,
                    s_left as int, p_right as int,
                );
            }
            result
        }
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    fn max_suffix_sum_par(a: &ArraySeqMtEphS<i32>) -> i32 {
        if a.length() == 0 { return i32::MIN / 2; }
        let (prefix_sums, total) =
            <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::scan(a, &|x, y| x + y, 0);
        let zero_seq = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::singleton(0);
        let all_prefixes =
            <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&zero_seq, &prefix_sums);
        let min_prefix = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(
            &all_prefixes, &|x: &i32, y: &i32| (*x).min(*y), i32::MAX,
        );
        total - min_prefix
    }

    #[cfg(not(verus_keep_ghost))]
    fn max_prefix_sum_par(a: &ArraySeqMtEphS<i32>) -> i32 {
        if a.length() == 0 { return i32::MIN / 2; }
        let (prefix_sums, _total) =
            <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::scan(a, &|x, y| x + y, 0);
        <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(
            &prefix_sums, &|x: &i32, y: &i32| (*x).max(*y), i32::MIN,
        )
    }

    #[cfg(not(verus_keep_ghost))]
    fn max_with_neginf_par(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (None, None) => None,
            (None, Some(_)) => b,
            (Some(_), None) => a,
            (Some(x), Some(y)) => Some(x.max(y)),
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl MaxContigSubSumDivConMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_divcon_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let n = a.length();
            if n == 0 { return None; }
            if n == 1 { return Some(*a.nth(0)); }

            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            let left_clone = left.clone();
            let right_clone = right.clone();
            let result_pair = ParaPair!(
                move || Self::max_contig_sub_sum_divcon_mt(&left_clone),
                move || Self::max_contig_sub_sum_divcon_mt(&right_clone)
            );
            let max_left = result_pair.0;
            let max_right = result_pair.1;

            let left_for_suffix = left.clone();
            let right_for_prefix = right.clone();
            let crossing_pair = ParaPair!(
                move || max_suffix_sum_par(&left_for_suffix),
                move || max_prefix_sum_par(&right_for_prefix)
            );
            let max_crossing = crossing_pair.0 + crossing_pair.1;

            let result = max_with_neginf_par(max_left, max_right);
            max_with_neginf_par(result, Some(max_crossing))
        }
    }
}
