//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Work Optimal (Chapter 28, Algorithm 28.16).

pub mod MaxContigSubSumOptStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap27::ScanContractStEph::ScanContractStEph::ScanContractStEphTrait;
    use crate::Types::Types::*;
    pub type T = ArraySeqStEphS<i32>;

    pub trait MaxContigSubSumOptTrait {
        /// Compute maximum contiguous subsequence sum using optimal scan-based algorithm.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// claude-4-sonnet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_opt(a: &ArraySeqStEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumOptTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_opt(a: &ArraySeqStEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base case: empty sequence returns None (-∞)
            if n == 0 {
                return None;
            }

            // Compute all prefix sums manually (inclusive)
            let mut all_prefixes_vec = Vec::with_capacity(n + 1);
            all_prefixes_vec.push(0); // empty prefix
            let mut running_sum = 0;
            for i in 0..n {
                running_sum += *a.nth(i);
                all_prefixes_vec.push(running_sum);
            }
            let all_prefixes = ArraySeqStEphS::from_vec(all_prefixes_vec);

            // Compute minimum prefix up to each position (inclusive)
            let mut min_prefixes_vec = Vec::with_capacity(n + 1);
            let mut running_min = i32::MAX;
            for i in 0..=n {
                running_min = running_min.min(*all_prefixes.nth(i));
                min_prefixes_vec.push(running_min);
            }
            let min_prefixes = ArraySeqStEphS::from_vec(min_prefixes_vec);

            // For each position i > 0, compute all_prefixes[i] - min_prefixes[i-1]
            // This gives the max subsequence ending at position i-1 in original array
            let mut max_sum = None;
            for i in 1..=n {
                let ending_max = *all_prefixes.nth(i) - *min_prefixes.nth(i - 1);
                max_sum = match max_sum {
                    | None => Some(ending_max),
                    | Some(current_max) => Some(current_max.max(ending_max)),
                };
            }

            max_sum
        }
    }
}
