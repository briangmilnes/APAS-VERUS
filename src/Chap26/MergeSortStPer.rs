//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential merge sort implementation (Chapter 26).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod MergeSortStPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		4. spec functions

    /// Spec function: result is a sorted permutation of the input.
    pub open spec fn spec_sorted(s: Seq<N>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
    }

    /// Spec function: s2 is a permutation of s1 (same multiset of elements).
    pub open spec fn spec_is_permutation(s1: Seq<N>, s2: Seq<N>) -> bool {
        s1.to_multiset() =~= s2.to_multiset()
    }

    /// Spec function: result of merge of two sorted sequences is sorted and a permutation.
    pub open spec fn spec_merge_post(left: Seq<N>, right: Seq<N>, result: Seq<N>) -> bool {
        &&& result.len() == left.len() + right.len()
        &&& spec_sorted(result)
        &&& spec_is_permutation(left.add(right), result)
    }

    /// Spec function: result of merge_sort is sorted and a permutation.
    pub open spec fn spec_sort_post(input: Seq<N>, result: Seq<N>) -> bool {
        &&& result.len() == input.len()
        &&& spec_sorted(result)
        &&& spec_is_permutation(input, result)
    }

    //		8. traits

    pub trait MergeSortStTrait {
        /// Merge two sorted sequences into one sorted sequence.
        /// APAS: Work Θ(n), Span Θ(n)
        fn merge(left: &ArraySeqStPerS<N>, right: &ArraySeqStPerS<N>) -> (result: ArraySeqStPerS<N>)
            requires
                spec_sorted(Seq::new(left.spec_len(), |i: int| left.spec_index(i))),
                spec_sorted(Seq::new(right.spec_len(), |i: int| right.spec_index(i))),
                left.spec_len() + right.spec_len() <= usize::MAX,
            ensures
                spec_merge_post(
                    Seq::new(left.spec_len(), |i: int| left.spec_index(i)),
                    Seq::new(right.spec_len(), |i: int| right.spec_index(i)),
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));

        /// Sort a sequence using merge sort.
        /// APAS: Work Θ(n log n), Span Θ(n log n)
        fn merge_sort(a: &ArraySeqStPerS<N>) -> (result: ArraySeqStPerS<N>)
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_sort_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));
    }

    //		9. impls

    impl MergeSortStTrait for ArraySeqStPerS<N> {
        #[verifier::external_body]
        fn merge(left: &ArraySeqStPerS<N>, right: &ArraySeqStPerS<N>) -> (result: ArraySeqStPerS<N>) {
            let n_left = left.length();
            let n_right = right.length();
            let total = n_left + n_right;

            if total == 0 {
                return ArraySeqStPerS::empty();
            }
            if n_left == 0 {
                return right.clone();
            }
            if n_right == 0 {
                return left.clone();
            }

            // Build result using tabulate
            ArraySeqStPerS::tabulate(
                &|i| {
                    // Determine position in left and right sequences
                    let mut l_idx = 0;
                    let mut r_idx = 0;
                    let mut count = 0;

                    // Simulate the merge to find element at position i
                    while count < i {
                        if l_idx < n_left && r_idx < n_right {
                            if left.nth(l_idx) <= right.nth(r_idx) {
                                l_idx += 1;
                            } else {
                                r_idx += 1;
                            }
                        } else if l_idx < n_left {
                            l_idx += 1;
                        } else {
                            r_idx += 1;
                        }
                        count += 1;
                    }

                    // Get the element at position i
                    if l_idx < n_left && r_idx < n_right {
                        if left.nth(l_idx) <= right.nth(r_idx) {
                            left.nth(l_idx).clone()
                        } else {
                            right.nth(r_idx).clone()
                        }
                    } else if l_idx < n_left {
                        left.nth(l_idx).clone()
                    } else {
                        right.nth(r_idx).clone()
                    }
                },
                total,
            )
        }

        #[verifier::external_body]
        fn merge_sort(a: &ArraySeqStPerS<N>) -> (result: ArraySeqStPerS<N>) {
            let n = a.length();

            // Base case: sequences of length 0 or 1 are already sorted
            if n <= 1 {
                return a.clone();
            }

            // Divide: split at midpoint
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            // Recur: sequential recursive calls
            let sorted_left = Self::merge_sort(&left);
            let sorted_right = Self::merge_sort(&right);

            // Combine: merge the two sorted halves
            Self::merge(&sorted_left, &sorted_right)
        }
    }

    } // verus!
} // mod
