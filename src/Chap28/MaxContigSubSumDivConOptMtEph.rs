//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Parallel Strengthened Divide and Conquer (Chapter 28, Algorithm 28.19).
//!
//! Historical Note: This parallel work-optimal version builds on the original divide-and-conquer
//! algorithm by Michael Shamos (CMU CS, 1977), using strengthening and ParaPair! with 32MB stack
//! for unconditional parallelism.

pub mod MaxContigSubSumDivConOptMtEph {

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;
    pub type T = ArraySeqMtEphS<i32>;

    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            | (None, None) => None,
            | (None, Some(_)) => b,
            | (Some(_), None) => a,
            | (Some(x), Some(y)) => Some(x.max(y)),
        }
    }

    /// Strengthened return type: (max_sum, max_prefix, max_suffix, total).
    type StrengthResult = (Option<i32>, i32, i32, i32);

    /// Auxiliary function that returns strengthened result, using parallel recursion.
    fn max_contig_sub_sum_aux_mt(a: &ArraySeqMtEphS<i32>) -> StrengthResult {
        let n = a.length();

        // Base case: empty sequence
        if n == 0 {
            return (None, i32::MIN / 2, i32::MIN / 2, 0); // (-∞, -∞, -∞, 0)
        }

        // Base case: single element
        if n == 1 {
            let val = a.nth(0).clone();
            return (Some(val), val, val, val);
        }

        // Divide: split at midpoint
        let mid = n / 2;
        let left = a.subseq_copy(0, mid);
        let right = a.subseq_copy(mid, n - mid);

        // Conquer: parallel recursive solve using ParaPair! with 32MB stack
        let left_clone = left.clone();
        let right_clone = right.clone();

        let result_pair = ParaPair!(move || max_contig_sub_sum_aux_mt(&left_clone), move || {
            max_contig_sub_sum_aux_mt(&right_clone)
        });
        let (m_left, p_left, s_left, t_left) = result_pair.0;
        let (m_right, p_right, s_right, t_right) = result_pair.1;

        // Combine: compute strengthened result
        let max_crossing = s_left + p_right;
        let max_sum = max_with_neginf(max_with_neginf(m_left, m_right), Some(max_crossing));
        let max_prefix = p_left.max(t_left + p_right);
        let max_suffix = s_right.max(s_left + t_right);
        let total = t_left + t_right;

        (max_sum, max_prefix, max_suffix, total)
    }

    /// Trait for parallel strengthened divide-and-conquer maximum contiguous subsequence sum.
    pub trait MaxContigSubSumDivConOptMtTrait {
        /// Compute maximum contiguous subsequence sum using parallel strengthened divide-and-conquer.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        /// claude-4-sonnet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_divcon_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumDivConOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_divcon_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let (max_sum, _, _, _) = max_contig_sub_sum_aux_mt(a);
            max_sum
        }
    }
}
