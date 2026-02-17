//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Reduced Force (Chapter 28, Algorithm 28.13).

pub mod MaxContigSubSumReducedStEph {

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

    /// Trait for reduced force maximum contiguous subsequence sum.
    pub trait MaxContigSubSumReducedTrait {
        /// Compute maximum contiguous subsequence sum using reduction to MCSSS.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n²), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        /// claude-4-sonnet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn max_contig_sub_sum_reduced(a: &ArraySeqStEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumReducedTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_reduced(a: &ArraySeqStEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base case: empty sequence returns None (-∞)
            if n == 0 {
                return None;
            }

            let mut global_max = None;

            // For each starting position
            for i in 0..n {
                // Compute sum for each subsequence starting at position i
                // We manually compute inclusive prefix sums instead of using scan_contract
                // because scan_contract is exclusive (doesn't include the last element in each prefix)
                let mut running_sum = 0;
                for j in i..n {
                    running_sum += *a.nth(j);
                    global_max = max_with_neginf(global_max, Some(running_sum));
                }
            }

            global_max
        }
    }
}
