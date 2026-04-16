// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Sequential merge sort implementation (Chapter 26).
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module


pub mod MergeSortStPer {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use vstd::seq_lib::lemma_multiset_commutative;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    //		Section 6. spec fns


    /// Spec function: result is a sorted permutation of the input.
    pub open spec fn spec_sorted(s: Seq<usize>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
    }

    /// Spec function: s2 is a permutation of s1 (same multiset of elements).
    pub open spec fn spec_is_permutation(s1: Seq<usize>, s2: Seq<usize>) -> bool {
        s1.to_multiset() =~= s2.to_multiset()
    }

    /// Spec function: result of merge of two sorted sequences is sorted and a permutation.
    pub open spec fn spec_merge_post(left: Seq<usize>, right: Seq<usize>, merged: Seq<usize>) -> bool {
        &&& merged.len() == left.len() + right.len()
        &&& spec_sorted(merged)
        &&& spec_is_permutation(left.add(right), merged)
    }

    /// Spec function: result of merge_sort is sorted and a permutation.
    pub open spec fn spec_sort_post(input: Seq<usize>, sorted: Seq<usize>) -> bool {
        &&& sorted.len() == input.len()
        &&& spec_sorted(sorted)
        &&& spec_is_permutation(input, sorted)
    }

    //		Section 7. proof fns/broadcast groups


    /// Helper: pushing an element >= all existing elements preserves sorted.
    pub proof fn lemma_push_sorted(s: Seq<usize>, v: usize)
        requires
            spec_sorted(s),
            s.len() > 0 ==> s.last() <= v,
        ensures
            spec_sorted(s.push(v)),
    {
        // Veracity: NEEDED assert
        assert forall|i: int, j: int|
            0 <= i < j < s.push(v).len() implies s.push(v)[i] <= s.push(v)[j]
        by {
            if j < s.len() as int {
            } else {
                // j == s.len(), s.push(v)[j] == v
                if s.len() > 0 {
                    // s[i] <= s.last() <= v
                } else {
                    // i < 0 < j, impossible since i >= 0
                }
            }
        }
    }

    //		Section 8. traits


    pub trait MergeSortStTrait {
        /// Merge two sorted sequences into one sorted sequence.
        /// - Alg Analysis: APAS (Ch26 Alg 26.4): Work O(n), Span O(lg n) — parallel merge assumed for merge sort analysis.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential two-pointer merge, Span = Work.
        fn merge(left: &ArraySeqStPerS<usize>, right: &ArraySeqStPerS<usize>) -> (merged: ArraySeqStPerS<usize>)
            requires
                spec_sorted(Seq::new(left.spec_len(), |i: int| left.spec_index(i))),
                spec_sorted(Seq::new(right.spec_len(), |i: int| right.spec_index(i))),
                left.spec_len() + right.spec_len() <= usize::MAX,
            ensures
                spec_merge_post(
                    Seq::new(left.spec_len(), |i: int| left.spec_index(i)),
                    Seq::new(right.spec_len(), |i: int| right.spec_index(i)),
                    Seq::new(merged.spec_len(), |i: int| merged.spec_index(i)));

        /// Sort a sequence using merge sort. Algorithm 26.4.
        /// - Alg Analysis: APAS (Ch26 Alg 26.4): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — ACCEPTED DIFFERENCE: sequential recursion and sequential merge
        fn merge_sort(a: &ArraySeqStPerS<usize>) -> (sorted: ArraySeqStPerS<usize>)
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_sort_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    Seq::new(sorted.spec_len(), |i: int| sorted.spec_index(i)));
    }

    //		Section 9. impls


    impl MergeSortStTrait for ArraySeqStPerS<usize> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — two-finger merge; St sequential.
        fn merge(left: &ArraySeqStPerS<usize>, right: &ArraySeqStPerS<usize>) -> (merged: ArraySeqStPerS<usize>) {
            let n_left = left.length();
            let n_right = right.length();
            let total = n_left + n_right;
            let ghost sl = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
            let ghost sr = Seq::new(right.spec_len(), |i: int| right.spec_index(i));

            let mut out: Vec<usize> = Vec::with_capacity(total);
            let mut li: usize = 0;
            let mut ri: usize = 0;

            while li < n_left || ri < n_right
                invariant
                    li <= n_left,
                    ri <= n_right,
                    n_left == left.spec_len(),
                    n_right == right.spec_len(),
                    total == n_left + n_right,
                    sl =~= Seq::new(left.spec_len(), |i: int| left.spec_index(i)),
                    sr =~= Seq::new(right.spec_len(), |i: int| right.spec_index(i)),
                    spec_sorted(sl),
                    spec_sorted(sr),
                    out@.len() == (li + ri) as int,
                    spec_sorted(out@),
                    out@.to_multiset() =~= (sl.take(li as int) + sr.take(ri as int)).to_multiset(),
                    out@.len() > 0 && li < n_left ==> out@.last() <= sl[li as int],
                    out@.len() > 0 && ri < n_right ==> out@.last() <= sr[ri as int],
                decreases (n_left - li) + (n_right - ri),
            {
                if li < n_left && (ri >= n_right || *left.nth(li) <= *right.nth(ri)) {
                    let v = *left.nth(li);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_push_sorted(out@, v);
                        // Veracity: NEEDED assert
                        assert(sl.take(li as int + 1) =~= sl.take(li as int).push(sl[li as int]));
                        lemma_multiset_commutative(sl.take(li as int + 1), sr.take(ri as int));
                        lemma_multiset_commutative(sl.take(li as int), sr.take(ri as int));
                    }
                    out.push(v);
                    li = li + 1;
                } else {
                    // Veracity: NEEDED proof block
                    let v = *right.nth(ri);
                    proof {
                        lemma_push_sorted(out@, v);
                        // Veracity: NEEDED assert
                        assert(sr.take(ri as int + 1) =~= sr.take(ri as int).push(sr[ri as int]));
                        lemma_multiset_commutative(sl.take(li as int), sr.take(ri as int + 1));
                        lemma_multiset_commutative(sl.take(li as int), sr.take(ri as int));
                    }
                    out.push(v);
                    ri = ri + 1;
                }
// Veracity: UNNEEDED proof block             }
// Veracity: UNNEEDED proof block 
// Veracity: UNNEEDED proof block             proof {
// Veracity: UNNEEDED proof block                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block                 assert(sl.take(n_left as int) =~= sl);
// Veracity: UNNEEDED assert                 assert(sr.take(n_right as int) =~= sr);
            }
            // Veracity: NEEDED proof block
            let ghost out_view = out@;
            let merged_result = ArraySeqStPerS { seq: out };
            proof {
                // Veracity: NEEDED assert
                assert(Seq::new(merged_result.spec_len(), |i: int| merged_result.spec_index(i)) =~= out_view);
                // Veracity: NEEDED assert (speed hint)
                assert(out_view.to_multiset() =~= (sl + sr).to_multiset());
                // Veracity: NEEDED assert (speed hint)
                assert(spec_merge_post(sl, sr, Seq::new(merged_result.spec_len(), |i: int| merged_result.spec_index(i))));
            }
            merged_result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — recursive D&C with O(n) merge; St sequential.
        fn merge_sort(a: &ArraySeqStPerS<usize>) -> (sorted: ArraySeqStPerS<usize>)
            decreases a.spec_len(),
        {
            let n = a.length();
            // Veracity: NEEDED proof block
            let ghost sa = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            if n == 0 {
                proof {
                    // Veracity: NEEDED assert (speed hint)
                    assert(sa =~= Seq::<usize>::empty());
                }
                // Veracity: NEEDED proof block
                return ArraySeqStPerS::empty();
            }
            if n == 1 {
                let s = ArraySeqStPerS::singleton(*a.nth(0));
                proof {
                    let s_view = Seq::new(s.spec_len(), |i: int| s.spec_index(i));
                    // Veracity: NEEDED assert (speed hint)
                    assert(sa.len() == 1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(s_view.len() == 1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(sa[0] == s_view[0]);
                    // Veracity: NEEDED assert
                    assert(sa =~= s_view);
                }
                return s;
            }

            let mid = n / 2;

            // Build left half [0..mid)
            let mut left_vec: Vec<usize> = Vec::with_capacity(mid);
            let mut i: usize = 0;
            while i < mid
                invariant
                    i <= mid, mid <= n, n == a.spec_len(),
                    left_vec@.len() == i as int,
                    forall|j: int| #![trigger left_vec@[j]] 0 <= j < i as int ==> left_vec@[j] == a.spec_index(j),
                decreases mid - i,
            {
                left_vec.push(*a.nth(i));
                i = i + 1;
            }
            let left = ArraySeqStPerS { seq: left_vec };

            // Build right half [mid..n)
            let right_len = n - mid;
            let mut right_vec: Vec<usize> = Vec::with_capacity(right_len);
            let mut i: usize = 0;
            while i < right_len
                invariant
                    i <= right_len, right_len == n - mid,
                    mid <= n, n == a.spec_len(),
                    right_vec@.len() == i as int,
                    forall|j: int| #![trigger right_vec@[j]] 0 <= j < i as int ==> right_vec@[j] == a.spec_index(mid as int + j),
                decreases right_len - i,
            {
                right_vec.push(*a.nth(mid + i));
                i = i + 1;
            }
            let right = ArraySeqStPerS { seq: right_vec };

            // Recursive sort
            let sorted_left = Self::merge_sort(&left);
            // Veracity: NEEDED proof block
            let sorted_right = Self::merge_sort(&right);

            // Merge sorted halves
            let merged = Self::merge(&sorted_left, &sorted_right);

            proof {
                let ghost sl_view = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
                let ghost sr_view = Seq::new(right.spec_len(), |i: int| right.spec_index(i));
                let ghost ssl = Seq::new(sorted_left.spec_len(), |i: int| sorted_left.spec_index(i));
                let ghost ssr = Seq::new(sorted_right.spec_len(), |i: int| sorted_right.spec_index(i));
                let ghost sm = Seq::new(merged.spec_len(), |i: int| merged.spec_index(i));

                // Veracity: NEEDED assert
                assert(sl_view + sr_view =~= sa);
                lemma_multiset_commutative(ssl, ssr);
                lemma_multiset_commutative(sl_view, sr_view);
            }
            merged
        }
    }

    } // verus!
} // mod
