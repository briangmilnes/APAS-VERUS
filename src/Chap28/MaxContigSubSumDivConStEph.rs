//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Divide and Conquer (Chapter 28, Algorithm 28.17).
//!
//! Historical Note: This divide-and-conquer algorithm was first designed by Michael Shamos
//! of Carnegie Mellon University CS in 1977, overnight, after hearing about the problem
//! from Ulf Grenander. See Jon Bentley, Programming Pearls (1st edition), page 76.

pub mod MaxContigSubSumDivConStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap27::ScanContractStEph::ScanContractStEph::ScanContractStEphTrait;
    use crate::Types::Types::*;
    pub type T = ArraySeqStEphS<i32>;

    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            | (None, None) => None,
            | (None, Some(_)) => b,
            | (Some(_), None) => a,
            | (Some(x), Some(y)) => Some(x.max(y)),
        }
    }

    /// find max suffix sum (MCSSE problem).
    fn max_suffix_sum(a: &ArraySeqStEphS<i32>) -> i32 {
        if a.length() == 0 {
            return i32::MIN / 2; // treat as -∞
        }

        // Compute all inclusive prefix sums
        let mut min_prefix = 0;
        let mut running_sum = 0;
        for i in 0..a.length() {
            running_sum += *a.nth(i);
            min_prefix = min_prefix.min(running_sum);
        }
        let total = running_sum;

        total - min_prefix
    }

    /// find max prefix sum (MCSSS problem).
    fn max_prefix_sum(a: &ArraySeqStEphS<i32>) -> i32 {
        if a.length() == 0 {
            return i32::MIN / 2; // treat as -∞
        }

        // Compute all inclusive prefix sums and return maximum
        // Note: Start with first element (not empty prefix) since empty sequence is not allowed
        let mut max_val = *a.nth(0);
        let mut running_sum = *a.nth(0);
        for i in 1..a.length() {
            running_sum += *a.nth(i);
            max_val = max_val.max(running_sum);
        }

        max_val
    }

    /// Trait for divide-and-conquer maximum contiguous subsequence sum.
    pub trait MaxContigSubSumDivConTrait {
        /// Compute maximum contiguous subsequence sum using divide-and-conquer.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n log n), Span Θ(log² n)
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        /// claude-4-sonnet: Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_divcon(a: &ArraySeqStEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumDivConTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_divcon(a: &ArraySeqStEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base cases
            if n == 0 {
                return None; // -∞
            }
            if n == 1 {
                return Some(*a.nth(0));
            }

            // Divide: split at midpoint
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            // Conquer: recursively solve subproblems
            let max_left = Self::max_contig_sub_sum_divcon(&left);
            let max_right = Self::max_contig_sub_sum_divcon(&right);

            // Combine: handle subsequence spanning the cut
            let max_suffix_left = max_suffix_sum(&left);
            let max_prefix_right = max_prefix_sum(&right);
            let max_crossing = max_suffix_left + max_prefix_right;

            // Return maximum of the three cases
            let result = max_with_neginf(max_left, max_right);
            max_with_neginf(result, Some(max_crossing))
        }
    }
}
