// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT
//! Maximum Contiguous Subsequence Sum — Parallel Strengthened D&C (Chapter 28, Algorithm 28.19).
//!
//! Verified sequential impl under verus_keep_ghost; parallel impl at runtime.

pub mod MaxContigSubSumDivConOptMtEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap28::MaxContigSubSumDivConOptStEph::MaxContigSubSumDivConOptStEph::lemma_strength_combine;
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

    type StrengthResult = (Option<i32>, i32, i32, i32);

    #[cfg(verus_keep_ghost)]
    fn max_contig_sub_sum_aux(a: &ArraySeqMtEphS<i32>) -> (result: StrengthResult)
        requires a.seq@.len() > 0, sums_fit_i32(a.seq@), obeys_feq_clone::<i32>(),
        ensures
            result.0.is_some(),
            is_mcss_of(a.seq@, result.0.unwrap() as int),
            is_max_prefix_sum(a.seq@, result.1 as int),
            is_max_suffix_sum(a.seq@, result.2 as int),
            result.3 as int == spec_range_sum(a.seq@, 0, a.seq@.len() as int),
        decreases a.seq@.len(),
    {
        let n = a.length();
        if n == 1 {
            let val = *a.nth(0);
            proof {
                lemma_range_sum_single(a.seq@, 0);
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
            lemma_range_sum_subseq(a.seq@, left.seq@, 0, 0, mid as int);
            lemma_range_sum_split(a.seq@, 0, mid as int, mid as int + hi_p);
            lemma_range_sum_subseq(a.seq@, right.seq@, mid as int, 0, right.seq@.len() as int);
            lemma_range_sum_split(a.seq@, lo_s, mid as int, n as int);
            lemma_range_sum_split(a.seq@, 0, mid as int, n as int);
        }

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

    pub trait MaxContigSubSumDivConOptMtTrait {
        /// Compute MCSS using parallel strengthened D&C (Algorithm 28.19).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n), Span Θ(log n)
        /// - Claude-Opus-4.6 (verified): Work Θ(n log n), Span Θ(n)
        fn max_contig_sub_sum_divcon_opt_mt(a: &ArraySeqMtEphS<i32>) -> (result: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
                obeys_feq_clone::<i32>(),
            ensures
                a.seq@.len() == 0 ==> result.is_none(),
                a.seq@.len() > 0 ==> result.is_some(),
                result.is_some() ==> is_mcss_of(a.seq@, result.unwrap() as int);
    }

    #[cfg(verus_keep_ghost)]
    impl MaxContigSubSumDivConOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_divcon_opt_mt(a: &ArraySeqMtEphS<i32>) -> (result: Option<i32>) {
            if a.length() == 0 { return None; }
            let (max_sum, _, _, _) = max_contig_sub_sum_aux(a);
            max_sum
        }
    }

    } // verus!

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
    type StrengthResultPar = (Option<i32>, i32, i32, i32);

    #[cfg(not(verus_keep_ghost))]
    fn max_contig_sub_sum_aux_par(a: &ArraySeqMtEphS<i32>) -> StrengthResultPar {
        let n = a.length();
        if n == 0 { return (None, i32::MIN / 2, i32::MIN / 2, 0); }
        if n == 1 {
            let val = a.nth(0).clone();
            return (Some(val), val, val, val);
        }

        let mid = n / 2;
        let left = a.subseq_copy(0, mid);
        let right = a.subseq_copy(mid, n - mid);

        let left_clone = left.clone();
        let right_clone = right.clone();
        let result_pair = ParaPair!(
            move || max_contig_sub_sum_aux_par(&left_clone),
            move || max_contig_sub_sum_aux_par(&right_clone)
        );
        let (m_left, p_left, s_left, t_left) = result_pair.0;
        let (m_right, p_right, s_right, t_right) = result_pair.1;

        let max_crossing = s_left + p_right;
        let max_sum = max_with_neginf_par(max_with_neginf_par(m_left, m_right), Some(max_crossing));
        let max_prefix = p_left.max(t_left + p_right);
        let max_suffix = s_right.max(s_left + t_right);
        let total = t_left + t_right;

        (max_sum, max_prefix, max_suffix, total)
    }

    #[cfg(not(verus_keep_ghost))]
    impl MaxContigSubSumDivConOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_divcon_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let (max_sum, _, _, _) = max_contig_sub_sum_aux_par(a);
            max_sum
        }
    }
}
