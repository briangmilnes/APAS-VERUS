//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Sequential Ephemeral (Chapter 35, Algorithm 35.2).
//! Randomized selection algorithm for finding kth order statistic.
//! Work: O(n) expected, Span: O(lg² n) expected.

pub mod OrderStatSelectStEph {

    use rand::Rng;
    use rand::RngExt;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    pub type T<T> = ArraySeqStEphS<T>;

    pub trait OrderStatSelectStEphTrait<T: StT + Ord> {
        /// - APAS: Work O(n) expected, Span O(lg² n) expected
        /// - Claude-Opus-4.6: Work O(n²) expected per level (tabulate uses O(n) scan per element), Span O(n) (sequential) — partition via tabulate is O(n²) not O(n) filter
        fn select(&self, k: N) -> Option<T>;
    }

    impl<T: StT + Ord> OrderStatSelectStEphTrait<T> for ArraySeqStEphS<T> {
        fn select(&self, k: N) -> Option<T> {
            let n = self.length();
            if k >= n || n == 0 {
                return None;
            }
            if n == 1 {
                return Some(self.nth(0).clone());
            }

            let pivot_idx = rand::rng().random_range(0..n);
            let pivot = self.nth(pivot_idx).clone();

            let mut left_count = 0;
            let mut right_count = 0;

            for i in 0..n {
                let elem = self.nth(i);
                if elem < &pivot {
                    left_count += 1;
                } else if elem > &pivot {
                    right_count += 1;
                }
            }

            if k < left_count {
                let left = ArraySeqStEphS::tabulate(
                    &|i| {
                        let mut idx = 0;
                        for j in 0..n {
                            let elem = self.nth(j);
                            if elem < &pivot {
                                if idx == i {
                                    return elem.clone();
                                }
                                idx += 1;
                            }
                        }
                        panic!("Index out of bounds in left partition");
                    },
                    left_count,
                );
                left.select(k)
            } else if k < n - right_count {
                Some(pivot)
            } else {
                let right = ArraySeqStEphS::tabulate(
                    &|i| {
                        let mut idx = 0;
                        for j in 0..n {
                            let elem = self.nth(j);
                            if elem > &pivot {
                                if idx == i {
                                    return elem.clone();
                                }
                                idx += 1;
                            }
                        }
                        panic!("Index out of bounds in right partition");
                    },
                    right_count,
                );
                right.select(k - (n - right_count))
            }
        }
    }
}
