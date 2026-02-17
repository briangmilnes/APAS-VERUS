//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Parallel Optimal (Chapter 28, Algorithm 28.16).

pub mod MaxContigSubSumOptMtEph {

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphBaseTrait;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    pub type T = ArraySeqMtEphS<i32>;

    pub trait MaxContigSubSumOptMtTrait {
        /// Compute maximum contiguous subsequence sum using parallel optimal scan-based algorithm.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        /// claude-4-sonnet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base case: empty sequence returns None (-∞)
            if n == 0 {
                return None;
            }

            // Algorithm 28.16: MCSSOpt using parallel scan operations
            // (b, v) = scan '+' 0 a
            let (inclusive_prefixes, _total) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(a, &|x, y| x + y, 0);

            // c = append ⟨0⟩ b  (prepend 0 to convert inclusive to exclusive-like sequence)
            let zero_seq = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::singleton(0);
            let all_prefixes = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&zero_seq, &inclusive_prefixes);

            // (d, _) = scan min ∞ c
            let (min_prefixes, _) = <ArraySeqMtEphS<i32> as ArraySeqMtEphBaseTrait<i32>>::scan(&all_prefixes, &|x, y| (*x).min(*y), i32::MAX);

            // e = ⟨c[i] − d[i−1] : 0 < i ≤ |a|⟩
            // For each ending position i (1-indexed), compute max subsequence ending there
            let differences = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(
                &|i| {
                    // c[i+1] - d[i] since we prepended 0 to make c
                    all_prefixes.nth(i + 1) - min_prefixes.nth_cloned(i).clone()
                },
                n,
            );

            // reduce max −∞ e
            Some(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(&differences, |x, y| (*x).max(*y), i32::MIN))
        }
    }
}
