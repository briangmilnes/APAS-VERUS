//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Strengthened Divide and Conquer (Chapter 28, Algorithm 28.19).
//!
//! Historical Note: This work-optimal strengthened version builds on the original divide-and-conquer
//! algorithm by Michael Shamos (CMU CS, 1977), using the strengthening technique to avoid
//! redundant computation and achieve O(n) work complexity.

pub mod MaxContigSubSumDivConOptStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    pub type T = ArraySeqStEphS<i32>;

    /// - APAS: Work Θ(1), Span Θ(1)
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            | (None, None) => None,
            | (None, Some(_)) => b,
            | (Some(_), None) => a,
            | (Some(x), Some(y)) => Some(x.max(y)),
        }
    }

    /// Strengthened return type: (max_sum, max_prefix, max_suffix, total).
    /// This avoids recomputing max_prefix and max_suffix in the combine step.
    type StrengthResult = (Option<i32>, i32, i32, i32);

    /// Auxiliary function that returns strengthened result (Algorithm 28.19 MCSSDCAux).
    /// - APAS: Work Θ(n), Span Θ(log² n) — W(n)=2W(n/2)+O(log n) with O(1) combine
    /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n) — subseq_copy adds O(n) per level; sequential
    fn max_contig_sub_sum_aux(a: &ArraySeqStEphS<i32>) -> StrengthResult {
        let n = a.length();

        // Base case: empty sequence
        if n == 0 {
            return (None, i32::MIN / 2, i32::MIN / 2, 0); // (-∞, -∞, -∞, 0)
        }

        // Base case: single element
        if n == 1 {
            let val = *a.nth(0);
            return (Some(val), val, val, val);
        }

        // Divide: split at midpoint
        let mid = n / 2;
        let left = a.subseq_copy(0, mid);
        let right = a.subseq_copy(mid, n - mid);

        // Conquer: recursively solve with strengthened interface
        let (m_left, p_left, s_left, t_left) = max_contig_sub_sum_aux(&left);
        let (m_right, p_right, s_right, t_right) = max_contig_sub_sum_aux(&right);

        // Combine: compute strengthened result
        // max_sum = max(m_left, m_right, s_left + p_right)
        let max_crossing = s_left + p_right;
        let max_sum = max_with_neginf(max_with_neginf(m_left, m_right), Some(max_crossing));

        // max_prefix = max(p_left, t_left + p_right)
        let max_prefix = p_left.max(t_left + p_right);

        // max_suffix = max(s_right, s_left + t_right)
        let max_suffix = s_right.max(s_left + t_right);

        // total = t_left + t_right
        let total = t_left + t_right;

        (max_sum, max_prefix, max_suffix, total)
    }

    /// Trait for strengthened divide-and-conquer maximum contiguous subsequence sum.
    pub trait MaxContigSubSumDivConOptTrait {
        /// Compute maximum contiguous subsequence sum using strengthened divide-and-conquer.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// claude-4-sonnet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_divcon_opt(a: &ArraySeqStEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumDivConOptTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_divcon_opt(a: &ArraySeqStEphS<i32>) -> Option<i32> {
            let (max_sum, _, _, _) = max_contig_sub_sum_aux(a);
            max_sum
        }
    }
}
