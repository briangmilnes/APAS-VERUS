// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT

//! Maximum Contiguous Subsequence Sum — Strengthened Divide and Conquer (Chapter 28, Algorithm 28.19).
//!
//! Historical Note: This work-optimal strengthened version builds on the original divide-and-conquer
//! algorithm by Michael Shamos (CMU CS, 1977), using the strengthening technique to avoid
//! redundant computation and achieve O(n) work complexity.
//!
//! ## Table of Contents
//! 1. imports
//! 2. spec definitions
//! 3. exec functions
//! 4. proof functions

//  Table of Contents
//	1. module
//	4. type definitions
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls

//		1. module


pub mod MaxContigSubSumDivConOptStEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		4. type definitions

    /// Strengthened return type: (mcss, max_prefix, max_suffix, total).
    type StrengthResult = (Option<i32>, i32, i32, i32);


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

    /// Combine lemma for the strengthened D&C.
    /// Given correct results for left and right halves, prove the combined result is correct.
    pub proof fn lemma_strength_combine(
        s: Seq<i32>,
        left: Seq<i32>,
        right: Seq<i32>,
        mid: int,
        m_left: int,
        p_left: int,
        s_left: int,
        t_left: int,
        m_right: int,
        p_right: int,
        s_right: int,
        t_right: int,
    )
        requires
            s.len() > 1,
            0 < mid < s.len() as int,
            left.len() == mid,
            right.len() == s.len() - mid,
            forall|i: int| #![trigger left[i]] 0 <= i < left.len() ==> left[i] == s[i],
            forall|i: int| #![trigger right[i]] 0 <= i < right.len() ==> right[i] == s[mid + i],
            is_mcss_of(left, m_left),
            is_max_prefix_sum(left, p_left),
            is_max_suffix_sum(left, s_left),
            t_left == spec_range_sum(left, 0, left.len() as int),
            is_mcss_of(right, m_right),
            is_max_prefix_sum(right, p_right),
            is_max_suffix_sum(right, s_right),
            t_right == spec_range_sum(right, 0, right.len() as int),
        ensures
            // mcss of whole
            ({
                let crossing = s_left + p_right;
                let m = if m_left >= m_right && m_left >= crossing { m_left }
                        else if m_right >= crossing { m_right }
                        else { crossing };
                is_mcss_of(s, m)
            }),
            // max prefix of whole
            ({
                let p = if p_left >= t_left + p_right { p_left } else { t_left + p_right };
                is_max_prefix_sum(s, p)
            }),
            // max suffix of whole
            ({
                let ss = if s_right >= s_left + t_right { s_right } else { s_left + t_right };
                is_max_suffix_sum(s, ss)
            }),
            // total
            t_left + t_right == spec_range_sum(s, 0, s.len() as int),
    {
        let n = s.len() as int;
        let crossing = s_left + p_right;
        let m = if m_left >= m_right && m_left >= crossing { m_left }
                else if m_right >= crossing { m_right }
                else { crossing };
        let p = if p_left >= t_left + p_right { p_left } else { t_left + p_right };
        let ss = if s_right >= s_left + t_right { s_right } else { s_left + t_right };

        // === Total ===
        lemma_range_sum_subseq(s, left, 0, 0, mid);
        lemma_range_sum_subseq(s, right, mid, 0, n - mid);
        lemma_range_sum_split(s, 0, mid, n);

        // === MCSS ===
        // Same proof as lemma_divcon_combine.
        // Part (a): m is achieved.
        if m_left >= m_right && m_left >= crossing {
            let (lo_l, hi_l) = choose|lo: int, hi: int|
                #![trigger spec_range_sum(left, lo, hi)]
                0 <= lo < hi <= left.len() as int &&
                spec_range_sum(left, lo, hi) == m_left;
            lemma_range_sum_subseq(s, left, 0, lo_l, hi_l);
        } else if m_right >= crossing {
            let (lo_r, hi_r) = choose|lo: int, hi: int|
                #![trigger spec_range_sum(right, lo, hi)]
                0 <= lo < hi <= right.len() as int &&
                spec_range_sum(right, lo, hi) == m_right;
            lemma_range_sum_subseq(s, right, mid, lo_r, hi_r);
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
        }
        // Part (b): m is maximal.
        assert forall|lo: int, hi: int|
            #![trigger spec_range_sum(s, lo, hi)]
            0 <= lo < hi <= n
        implies
            spec_range_sum(s, lo, hi) <= m
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

        // === Max prefix ===
        // Part (a): p is achieved.
        if p_left >= t_left + p_right {
            // p = p_left, achieved by some prefix in left.
            let hi_pl: int = choose|hi: int|
                #![trigger spec_range_sum(left, 0, hi)]
                1 <= hi <= left.len() as int &&
                spec_range_sum(left, 0, hi) == p_left;
            lemma_range_sum_subseq(s, left, 0, 0, hi_pl);
            assert(spec_range_sum(s, 0, hi_pl) == p);
        } else {
            // p = t_left + p_right, achieved by the whole left + a prefix of right.
            let hi_pr: int = choose|hi: int|
                #![trigger spec_range_sum(right, 0, hi)]
                1 <= hi <= right.len() as int &&
                spec_range_sum(right, 0, hi) == p_right;
            lemma_range_sum_subseq(s, left, 0, 0, mid);
            lemma_range_sum_subseq(s, right, mid, 0, hi_pr);
            lemma_range_sum_split(s, 0, mid, mid + hi_pr);
            assert(spec_range_sum(s, 0, mid + hi_pr) == p);
        }
        // Part (b): p is maximal.
        assert forall|hi: int|
            #![trigger spec_range_sum(s, 0, hi)]
            1 <= hi <= n
        implies
            spec_range_sum(s, 0, hi) <= p
        by {
            if hi <= mid {
                lemma_range_sum_subseq(s, left, 0, 0, hi);
            } else {
                lemma_range_sum_split(s, 0, mid, hi);
                lemma_range_sum_subseq(s, left, 0, 0, mid);
                lemma_range_sum_subseq(s, right, mid, 0, hi - mid);
            }
        };

        // === Max suffix ===
        // Part (a): ss is achieved.
        if s_right >= s_left + t_right {
            // ss = s_right, achieved by some suffix in right.
            let lo_sr: int = choose|lo: int|
                #![trigger spec_range_sum(right, lo, right.len() as int)]
                0 <= lo < right.len() as int &&
                spec_range_sum(right, lo, right.len() as int) == s_right;
            lemma_range_sum_subseq(s, right, mid, lo_sr, n - mid);
            assert(spec_range_sum(s, mid + lo_sr, n) == ss);
        } else {
            // ss = s_left + t_right, achieved by a suffix of left + whole right.
            let lo_sl: int = choose|lo: int|
                #![trigger spec_range_sum(left, lo, left.len() as int)]
                0 <= lo < left.len() as int &&
                spec_range_sum(left, lo, left.len() as int) == s_left;
            lemma_range_sum_subseq(s, left, 0, lo_sl, mid);
            lemma_range_sum_subseq(s, right, mid, 0, n - mid);
            lemma_range_sum_split(s, lo_sl, mid, n);
            assert(spec_range_sum(s, lo_sl, n) == ss);
        }
        // Part (b): ss is maximal.
        assert forall|lo: int|
            #![trigger spec_range_sum(s, lo, n)]
            0 <= lo < n
        implies
            spec_range_sum(s, lo, n) <= ss
        by {
            if lo >= mid {
                lemma_range_sum_subseq(s, right, mid, lo - mid, n - mid);
            } else {
                lemma_range_sum_split(s, lo, mid, n);
                lemma_range_sum_subseq(s, left, 0, lo, mid);
                lemma_range_sum_subseq(s, right, mid, 0, n - mid);
            }
        };
    }


    //		8. traits

    /// Trait for strengthened divide-and-conquer MCSS.
    pub trait MaxContigSubSumDivConOptTrait {
        /// Compute MCSS using strengthened divide-and-conquer (Algorithm 28.19).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n) — subseq_copy; sequential
        fn max_contig_sub_sum_divcon_opt(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
            ensures
                a.seq@.len() == 0 ==> mcss.is_none(),
                a.seq@.len() > 0 ==> mcss.is_some(),
                mcss.is_some() ==> is_mcss_of(a.seq@, mcss.unwrap() as int);
    }


    //		9. impls

    // ─── 3. exec functions ───

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

    /// Auxiliary function: returns (mcss, max_prefix, max_suffix, total).
    /// - APAS: Work Θ(n), Span Θ(log² n)
    /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n) — subseq_copy O(n) per level; sequential
    fn max_contig_sub_sum_aux(a: &ArraySeqStEphS<i32>) -> (mcss: StrengthResult)
        requires
            a.seq@.len() > 0,
            sums_fit_i32(a.seq@),
        ensures
            mcss.0.is_some(),
            is_mcss_of(a.seq@, mcss.0.unwrap() as int),
            is_max_prefix_sum(a.seq@, mcss.1 as int),
            is_max_suffix_sum(a.seq@, mcss.2 as int),
            mcss.3 as int == spec_range_sum(a.seq@, 0, a.seq@.len() as int),
        decreases a.seq@.len(),
    {
        let n = a.length();

        if n == 1 {
            let val = *a.nth(0);
            proof {
                lemma_range_sum_single(a.seq@, 0);
                // val == a.seq@[0], range_sum(a, 0, 1) == val
                assert(spec_range_sum(a.seq@, 0, 1) == val as int);
            }
            return (Some(val), val, val, val);
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

        let (m_left, p_left, s_left, t_left) = max_contig_sub_sum_aux(&left);
        let (m_right, p_right, s_right, t_right) = max_contig_sub_sum_aux(&right);

        // Prove all four combine operations don't overflow i32.
        proof {
            // Witnesses for suffix/prefix achievement.
            let lo_s: int = choose|lo: int|
                #![trigger spec_range_sum(left.seq@, lo, left.seq@.len() as int)]
                0 <= lo < left.seq@.len() as int &&
                spec_range_sum(left.seq@, lo, left.seq@.len() as int) == s_left as int;
            let hi_p: int = choose|hi: int|
                #![trigger spec_range_sum(right.seq@, 0, hi)]
                1 <= hi <= right.seq@.len() as int &&
                spec_range_sum(right.seq@, 0, hi) == p_right as int;

            // (1) s_left + p_right = range_sum(a, lo_s, mid + hi_p) fits.
            lemma_range_sum_subseq(a.seq@, left.seq@, 0, lo_s, mid as int);
            lemma_range_sum_subseq(a.seq@, right.seq@, mid as int, 0, hi_p);
            lemma_range_sum_split(a.seq@, lo_s, mid as int, mid as int + hi_p);

            // (2) t_left + p_right = range_sum(a, 0, mid + hi_p) fits.
            lemma_range_sum_subseq(a.seq@, left.seq@, 0, 0, mid as int);
            lemma_range_sum_split(a.seq@, 0, mid as int, mid as int + hi_p);

            // (3) s_left + t_right = range_sum(a, lo_s, n) fits.
            lemma_range_sum_subseq(a.seq@, right.seq@, mid as int, 0, right.seq@.len() as int);
            lemma_range_sum_split(a.seq@, lo_s, mid as int, n as int);

            // (4) t_left + t_right = range_sum(a, 0, n) fits.
            lemma_range_sum_split(a.seq@, 0, mid as int, n as int);
        }

        // Combine.
        let max_crossing = s_left + p_right;
        let max_sum = max_with_neginf(max_with_neginf(m_left, m_right), Some(max_crossing));
        let max_prefix = if p_left >= t_left + p_right { p_left } else { t_left + p_right };
        let max_suffix = if s_right >= s_left + t_right { s_right } else { s_left + t_right };
        let total = t_left + t_right;

        proof {
            lemma_strength_combine(
                a.seq@, left.seq@, right.seq@, mid as int,
                m_left.unwrap() as int, p_left as int, s_left as int, t_left as int,
                m_right.unwrap() as int, p_right as int, s_right as int, t_right as int,
            );
        }

        (max_sum, max_prefix, max_suffix, total)
    }

    impl MaxContigSubSumDivConOptTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_divcon_opt(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>) {
            if a.length() == 0 {
                return None;
            }
            let (max_sum, _, _, _) = max_contig_sub_sum_aux(a);
            max_sum
        }
    }

    } // verus!
}
