//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Parallel Persistent (Chapter 35, Algorithm 35.2).
//! Randomized selection algorithm for finding kth order statistic with parallel partition.
//! Work: O(n) expected, Span: O(lg² n) expected.

pub mod OrderStatSelectMtPer {

    use rand::Rng;
    use rand::RngExt;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;
    pub type T<T> = ArraySeqMtPerS<T>;

    pub trait OrderStatSelectMtPerTrait<T: StTInMtT + Ord + 'static> {
        /// - APAS: Work O(n) expected, Span O(lg² n) expected
        /// - Claude-Opus-4.6: Work O(n) expected, Span O(lg n) expected — spawns n threads per level with mutex-based partition; O(n) work per level, O(lg n) span per level due to lock contention
        fn select(&self, k: N) -> Option<T>;
    }

    impl<T: StTInMtT + Ord + 'static> OrderStatSelectMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn select(&self, k: N) -> Option<T> {
            let n = self.length();
            if k >= n || n == 0 {
                return None;
            }
            if n == 1 {
                return Some(self.nth(0).clone());
            }

            // Algorithm 35.2: Randomized selection with parallel partition
            let pivot_idx = rand::rng().random_range(0..n);
            let pivot = self.nth(pivot_idx).clone();

            // Parallel partition using thread spawning (no rayon)
            use std::sync::{Arc, Mutex};
            
            let pivot_left = Arc::new(pivot.clone());
            let pivot_right = Arc::new(pivot.clone());
            
            let left_vec = Arc::new(Mutex::new(Vec::new()));
            let right_vec = Arc::new(Mutex::new(Vec::new()));
            
            // Spawn threads to partition in parallel
            std::thread::scope(|s| {
                for i in 0..n {
                    let self_ref = self;
                    let pivot_left_ref = Arc::clone(&pivot_left);
                    let pivot_right_ref = Arc::clone(&pivot_right);
                    let left_vec_ref = Arc::clone(&left_vec);
                    let right_vec_ref = Arc::clone(&right_vec);
                    
                    s.spawn(move || {
                        let elem = self_ref.nth(i);
                        if elem < pivot_left_ref.as_ref() {
                            left_vec_ref.lock().unwrap().push(elem.clone());
                        } else if elem > pivot_right_ref.as_ref() {
                            right_vec_ref.lock().unwrap().push(elem.clone());
                        }
                    });
                }
            });

            let left_elements = Arc::try_unwrap(left_vec).unwrap().into_inner().unwrap();
            let right_elements = Arc::try_unwrap(right_vec).unwrap().into_inner().unwrap();
            
            let left_count = left_elements.len();
            let right_count = right_elements.len();

            // Recursive selection based on partition sizes
            if k < left_count {
                let left = ArraySeqMtPerS::from_vec(left_elements);
                left.select(k)
            } else if k < n - right_count {
                Some(pivot)
            } else {
                let right = ArraySeqMtPerS::from_vec(right_elements);
                right.select(k - (n - right_count))
            }
        }
    }
}
