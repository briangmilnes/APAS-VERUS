//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel merge sort implementation (Chapter 26).
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod MergeSortMtPer {

    use std::sync::Arc;
    use std::thread;
    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap26::MergeSortStPer::MergeSortStPer::{
        spec_sorted, spec_is_permutation, spec_merge_post, spec_sort_post,
    };
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait MergeSortMtTrait {
        /// Merge two sorted sequences in parallel using binary search.
        /// APAS: Work Θ(n), Span Θ(log n)
        fn merge_parallel(left: &ArraySeqMtPerS<N>, right: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
            requires
                spec_sorted(Seq::new(left.spec_len(), |i: int| left.spec_index(i))),
                spec_sorted(Seq::new(right.spec_len(), |i: int| right.spec_index(i))),
                left.spec_len() + right.spec_len() <= usize::MAX,
            ensures
                spec_merge_post(
                    Seq::new(left.spec_len(), |i: int| left.spec_index(i)),
                    Seq::new(right.spec_len(), |i: int| right.spec_index(i)),
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));

        /// Sort a sequence using parallel merge sort.
        /// APAS: Work Θ(n log n), Span Θ(log² n)
        fn merge_sort_parallel(a: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_sort_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));
    }

    //		9. impls

    impl MergeSortMtTrait for ArraySeqMtPerS<N> {
        #[verifier::external_body]
        fn merge_parallel(left: &ArraySeqMtPerS<N>, right: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>) {
            let n_left = left.length();
            let n_right = right.length();
            let total = n_left + n_right;

            if total == 0 {
                return ArraySeqMtPerS::empty();
            }
            if n_left == 0 {
                return right.clone();
            }
            if n_right == 0 {
                return left.clone();
            }

            // Base case for small merges
            if total <= 2 {
                if n_left == 1 && n_right == 1 {
                    if left.nth(0) <= right.nth(0) {
                        return ArraySeqMtPerS::append(left, right);
                    } else {
                        return ArraySeqMtPerS::append(right, left);
                    }
                }
                return ArraySeqMtPerS::tabulate(
                    &|i| {
                        if i < n_left {
                            left.nth(i).clone()
                        } else {
                            right.nth(i - n_left).clone()
                        }
                    },
                    total,
                );
            }

            // Parallel merge: split the longer sequence
            if n_left >= n_right {
                let mid_left = n_left / 2;
                let pivot = left.nth(mid_left);

                // Binary search in right sequence
                let mut lo = 0;
                let mut hi = n_right;
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if right.nth(mid) < pivot {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                let mid_right = lo;

                let left_left = left.subseq_copy(0, mid_left);
                let left_right = left.subseq_copy(mid_left + 1, n_left - mid_left - 1);
                let right_left = right.subseq_copy(0, mid_right);
                let right_right = right.subseq_copy(mid_right, n_right - mid_right);

                let left_left_arc = Arc::new(left_left);
                let right_left_arc = Arc::new(right_left);
                let left_right_arc = Arc::new(left_right);
                let right_right_arc = Arc::new(right_right);

                let (merged_left, merged_right) = thread::scope(|s| {
                    let ll = left_left_arc.clone();
                    let rl = right_left_arc.clone();
                    let handle_left = s.spawn(move || Self::merge_parallel(&*ll, &*rl));

                    let lr = left_right_arc.clone();
                    let rr = right_right_arc.clone();
                    let merged_right = Self::merge_parallel(&*lr, &*rr);

                    let merged_left = handle_left.join().unwrap();
                    (merged_left, merged_right)
                });

                let pivot_seq = ArraySeqMtPerS::singleton(pivot.clone());
                let left_with_pivot = ArraySeqMtPerS::append(&merged_left, &pivot_seq);
                ArraySeqMtPerS::append(&left_with_pivot, &merged_right)
            } else {
                let mid_right = n_right / 2;
                let pivot = right.nth(mid_right);

                let mut lo = 0;
                let mut hi = n_left;
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if left.nth(mid) <= pivot {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                let mid_left = lo;

                let left_left = left.subseq_copy(0, mid_left);
                let left_right = left.subseq_copy(mid_left, n_left - mid_left);
                let right_left = right.subseq_copy(0, mid_right);
                let right_right = right.subseq_copy(mid_right + 1, n_right - mid_right - 1);

                let left_left_arc = Arc::new(left_left);
                let right_left_arc = Arc::new(right_left);
                let left_right_arc = Arc::new(left_right);
                let right_right_arc = Arc::new(right_right);

                let (merged_left, merged_right) = thread::scope(|s| {
                    let ll = left_left_arc.clone();
                    let rl = right_left_arc.clone();
                    let handle_left = s.spawn(move || Self::merge_parallel(&*ll, &*rl));

                    let lr = left_right_arc.clone();
                    let rr = right_right_arc.clone();
                    let merged_right = Self::merge_parallel(&*lr, &*rr);

                    let merged_left = handle_left.join().unwrap();
                    (merged_left, merged_right)
                });

                let pivot_seq = ArraySeqMtPerS::singleton(pivot.clone());
                let left_with_pivot = ArraySeqMtPerS::append(&merged_left, &pivot_seq);
                ArraySeqMtPerS::append(&left_with_pivot, &merged_right)
            }
        }

        #[verifier::external_body]
        fn merge_sort_parallel(a: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>) {
            let n = a.length();

            if n <= 1 {
                return a.clone();
            }

            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            let left_arc = Arc::new(left);
            let right_arc = Arc::new(right);

            let Pair(sorted_left, sorted_right) = crate::ParaPair!(
                {
                    let l = left_arc.clone();
                    move || Self::merge_sort_parallel(&*l)
                },
                {
                    let r = right_arc.clone();
                    move || Self::merge_sort_parallel(&*r)
                }
            );

            Self::merge_parallel(&sorted_left, &sorted_right)
        }
    }

    } // verus!
} // mod
