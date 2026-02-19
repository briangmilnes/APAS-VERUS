// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT

//! Maximum Contiguous Subsequence Sum — Divide and Conquer (Chapter 28, Algorithm 28.17).
//!
//! Historical Note: This divide-and-conquer algorithm was first designed by Michael Shamos
//! of Carnegie Mellon University CS in 1977, overnight, after hearing about the problem
//! from Ulf Grenander. See Jon Bentley, Programming Pearls (1st edition), page 76.
//!
//! ## Table of Contents
//! 1. imports
//! 2. spec definitions
//! 3. exec functions
//! 4. proof functions

//  Table of Contents
//	1. module
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls

//		1. module


pub mod MaxContigSubSumDivConStEph {
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


    //		7. proof fns/broadcast groups

    // ─── 4. proof functions ───

    /// Key lemma: MCSS of the whole = max(MCSS-left, MCSS-right, max-crossing).
    pub proof fn lemma_divcon_combine(
        s: Seq<i32>,
        left: Seq<i32>,
        right: Seq<i32>,
        mid: int,
        m_left: int,
        m_right: int,
        s_left: int,
        p_right: int,
    )
        requires
            s.len() > 1,
            0 < mid < s.len() as int,
            left.len() == mid,
            right.len() == s.len() - mid,
            forall|i: int| #![trigger left[i]] 0 <= i < left.len() ==> left[i] == s[i],
            forall|i: int| #![trigger right[i]] 0 <= i < right.len() ==> right[i] == s[mid + i],
            is_mcss_of(left, m_left),
            is_mcss_of(right, m_right),
            is_max_suffix_sum(left, s_left),
            is_max_prefix_sum(right, p_right),
        ensures
            is_mcss_of(s, ({
                let crossing = s_left + p_right;
                if m_left >= m_right && m_left >= crossing { m_left }
                else if m_right >= crossing { m_right }
                else { crossing }
            })),
    {
        let n = s.len() as int;
        let crossing = s_left + p_right;
        let result = if m_left >= m_right && m_left >= crossing { m_left }
                     else if m_right >= crossing { m_right }
                     else { crossing };

        // Part (a): result is achieved.
        if m_left >= m_right && m_left >= crossing {
            let (lo_l, hi_l) = choose|lo: int, hi: int|
                #![trigger spec_range_sum(left, lo, hi)]
                0 <= lo < hi <= left.len() as int &&
                spec_range_sum(left, lo, hi) == m_left;
            lemma_range_sum_subseq(s, left, 0, lo_l, hi_l);
            assert(spec_range_sum(s, lo_l, hi_l) == m_left);
        } else if m_right >= crossing {
            let (lo_r, hi_r) = choose|lo: int, hi: int|
                #![trigger spec_range_sum(right, lo, hi)]
                0 <= lo < hi <= right.len() as int &&
                spec_range_sum(right, lo, hi) == m_right;
            lemma_range_sum_subseq(s, right, mid, lo_r, hi_r);
            assert(spec_range_sum(s, mid + lo_r, mid + hi_r) == m_right);
        } else {
            let lo_s: int = choose|lo: int|
                #![trigger spec_range_sum(left, lo, left.len() as int)]
                0 <= lo < left.len() as int &&
                spec_range_sum(left, lo, left.len() as int) == s_left;
            let hi_p: int = choose|hi: int|
                #![trigger spec_range_sum(right, 0, hi)]
                1 <= hi <= right.len() as int &&
                spec_range_sum(right, 0, hi) == p_right;
            lemma_range_sum_subseq(s, left, 0, lo_s, mid);
            lemma_range_sum_subseq(s, right, mid, 0, hi_p);
            lemma_range_sum_split(s, lo_s, mid, mid + hi_p);
            assert(spec_range_sum(s, lo_s, mid + hi_p) == crossing);
        }

        // Part (b): result is maximal.
        assert forall|lo: int, hi: int|
            #![trigger spec_range_sum(s, lo, hi)]
            0 <= lo < hi <= n
        implies
            spec_range_sum(s, lo, hi) <= result
        by {
            if hi <= mid {
                lemma_range_sum_subseq(s, left, 0, lo, hi);
            } else if lo >= mid {
                lemma_range_sum_subseq(s, right, mid, lo - mid, hi - mid);
            } else {
                lemma_range_sum_split(s, lo, mid, hi);
                lemma_range_sum_subseq(s, left, 0, lo, mid);
                lemma_range_sum_subseq(s, right, mid, 0, hi - mid);
            }
        };
    }


    //		8. traits

    /// Trait for divide-and-conquer MCSS.
    pub trait MaxContigSubSumDivConTrait {
        /// Compute MCSS using divide-and-conquer (Algorithm 28.17).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n log n), Span Θ(log² n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) — sequential
        fn max_contig_sub_sum_divcon(a: &ArraySeqStEphS<i32>) -> (result: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
            ensures
                a.seq@.len() == 0 ==> result.is_none(),
                a.seq@.len() > 0 ==> result.is_some(),
                result.is_some() ==> is_mcss_of(a.seq@, result.unwrap() as int);
    }


    //		9. impls

    // ─── 3. exec functions ───

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

    /// Find max suffix sum (MCSSE problem, Algorithm 28.12).
    /// max over lo in 0..n of range_sum(a, lo, n).
    /// Uses prefix-sum approach: result = total - min(prefix(0), ..., prefix(n-1)).
    /// - APAS: Work Θ(n), Span Θ(log n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop
    fn max_suffix_sum(a: &ArraySeqStEphS<i32>) -> (result: i32)
        requires
            a.seq@.len() > 0,
            sums_fit_i32(a.seq@),
        ensures
            is_max_suffix_sum(a.seq@, result as int),
    {
        let n = a.length();
        let mut running_sum: i32 = 0;
        let mut min_prefix: i32 = 0;  // prefix(0) = 0

        let mut i: usize = 0;
        while i < n
            invariant
                0 <= i <= n,
                n as int == a.seq@.len(),
                sums_fit_i32(a.seq@),
                running_sum as int == spec_prefix_sum(a.seq@, i as int),
                // min_prefix = min of prefix(0), ..., prefix(i-1) when i > 0;
                // min_prefix = prefix(0) = 0 when i == 0.
                (forall|j: int|
                    #![trigger spec_prefix_sum(a.seq@, j)]
                    0 <= j < i || j == 0 ==> min_prefix as int <= spec_prefix_sum(a.seq@, j)),
                (exists|j: int|
                    #![trigger spec_prefix_sum(a.seq@, j)]
                    0 <= j <= i && j < n && min_prefix as int == spec_prefix_sum(a.seq@, j))
                    || (i == 0 && min_prefix == 0),
            decreases n - i,
        {
            // Update min_prefix with prefix(i) BEFORE adding a[i].
            // This ensures we only include prefix(0), ..., prefix(n-1).
            if running_sum < min_prefix {
                min_prefix = running_sum;
            }

            proof { lemma_range_sum_snoc(a.seq@, 0, (i + 1) as int); }
            running_sum = running_sum + *a.nth(i);
            i = i + 1;
        }

        // Now: running_sum = prefix(n) = total,
        //      min_prefix = min(prefix(0), ..., prefix(n-1)).
        // result = total - min_prefix = max suffix sum.
        proof {
            let total = running_sum as int;

            // (a) Achieved: min_prefix = prefix(lo_w) for some lo_w < n.
            // range_sum(a, lo_w, n) = prefix(n) - prefix(lo_w) = total - min_prefix.
            if n > 0 {
                // min_prefix is achieved at some j
                let lo_w: int = choose|j: int|
                    #![trigger spec_prefix_sum(a.seq@, j)]
                    0 <= j < n as int && min_prefix as int == spec_prefix_sum(a.seq@, j);
                lemma_range_sum_via_prefix(a.seq@, lo_w, n as int);
                assert(spec_range_sum(a.seq@, lo_w, a.seq@.len() as int) == total - min_prefix as int);
            }

            // (b) Maximal: for all lo in 0..n: range_sum(a, lo, n) <= total - min_prefix.
            assert forall|lo: int|
                #![trigger spec_range_sum(a.seq@, lo, a.seq@.len() as int)]
                0 <= lo < a.seq@.len() as int
            implies
                spec_range_sum(a.seq@, lo, a.seq@.len() as int) <= (running_sum - min_prefix) as int
            by {
                lemma_range_sum_via_prefix(a.seq@, lo, n as int);
                // range_sum = prefix(n) - prefix(lo) <= prefix(n) - min_prefix
            };
        }

        running_sum - min_prefix
    }

    /// Find max prefix sum (MCSSS problem, Algorithm 28.11).
    /// max over hi in 1..=n of range_sum(a, 0, hi).
    /// - APAS: Work Θ(n), Span Θ(log n)
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop
    fn max_prefix_sum(a: &ArraySeqStEphS<i32>) -> (result: i32)
        requires
            a.seq@.len() > 0,
            sums_fit_i32(a.seq@),
        ensures
            is_max_prefix_sum(a.seq@, result as int),
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
            if running_sum > max_val {
                max_val = running_sum;
            }
            i = i + 1;
        }

        max_val
    }

    impl MaxContigSubSumDivConTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_divcon(a: &ArraySeqStEphS<i32>) -> (result: Option<i32>)
            decreases a.seq@.len(),
        {
            let n = a.length();

            if n == 0 {
                return None;
            }
            if n == 1 {
                proof {
                    lemma_range_sum_single(a.seq@, 0);
                }
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

            let max_left = Self::max_contig_sub_sum_divcon(&left);
            let max_right = Self::max_contig_sub_sum_divcon(&right);

            let s_left = max_suffix_sum(&left);
            let p_right = max_prefix_sum(&right);

            proof {
                // Show s_left + p_right fits in i32.
                // s_left = range_sum(left, lo_s, mid) for some lo_s.
                // p_right = range_sum(right, 0, hi_p) for some hi_p.
                // Their sum = range_sum(a, lo_s, mid + hi_p), which fits by sums_fit_i32.
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
                    a.seq@,
                    left.seq@,
                    right.seq@,
                    mid as int,
                    max_left.unwrap() as int,
                    max_right.unwrap() as int,
                    s_left as int,
                    p_right as int,
                );
            }

            result
        }
    }

    } // verus!
}
