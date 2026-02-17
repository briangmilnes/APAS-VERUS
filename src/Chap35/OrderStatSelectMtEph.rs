//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Parallel Ephemeral (Chapter 35, Algorithm 35.2).
//! Randomized selection algorithm for finding kth order statistic with parallel partition.
//! Work: O(n) expected, Span: O(lg² n) expected.

pub mod OrderStatSelectMtEph {

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use rand::Rng;
    use rand::RngExt;
    pub type T<T> = ArraySeqMtEphS<T>;

    pub trait OrderStatSelectMtEphTrait<T: StTInMtT + Ord> {
        /// Algorithm 35.2: Contraction-Based Select with parallel filter operations
        /// APAS: Work O(n) expected, Span O(lg² n) with high probability
        /// claude-4-sonet: Work Θ(n) expected, Θ(n²) worst case; Span Θ(log² n) expected, Parallelism Θ(n/log² n) expected
        fn select(&self, k: N) -> Option<T>;
    }

    impl<T: StTInMtT + Ord + Clone + Eq + Send + Sync + 'static> OrderStatSelectMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn select(&self, k: N) -> Option<T> {
            let n = self.length();
            
            // Base cases
            if k >= n || n == 0 {
                return None;
            }
            if n == 1 {
                return Some(self.nth(0).clone());
            }

            // Pick a uniformly random element from a
            let pivot_idx = rand::rng().random_range(0..n);
            let pivot = self.nth(pivot_idx).clone();

            // Algorithm 35.2 lines 4-5: parallel filter operations
            // ℓ = ⟨ x ∈ a | x < p ⟩
            // r = ⟨ x ∈ a | x > p ⟩
            let pivot_left = pivot.clone();
            let pivot_right = pivot.clone();
            let pivot_result = pivot.clone();
            
            let left = ArraySeqMtEphS::filter_par(self, move |x: &T| *x < pivot_left);
            let right = ArraySeqMtEphS::filter_par(self, move |x: &T| *x > pivot_right);

            let left_len = left.length();
            let right_len = right.length();

            // Algorithm 35.2 lines 7-9: determine which partition contains kth element
            if k < left_len {
                // kth element is in left partition
                left.select(k)
            } else if k < n - right_len {
                // kth element is the pivot (elements equal to pivot)
                Some(pivot_result)
            } else {
                // kth element is in right partition, adjust k
                right.select(k - (n - right_len))
            }
        }
    }
}
