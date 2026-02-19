//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Sequential merge sort implementation (Chapter 26).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls

//		1. module




pub mod MergeSortStPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use vstd::seq_lib::lemma_multiset_commutative;


    //		3. broadcast use

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
    };


    //		6. spec fns

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


    //		7. proof fns/broadcast groups

    //		9. impls

    /// Helper: pushing an element >= all existing elements preserves sorted.
    pub proof fn lemma_push_sorted(s: Seq<N>, v: N)
        requires
            spec_sorted(s),
            s.len() > 0 ==> s.last() <= v,
        ensures
            spec_sorted(s.push(v)),
    {
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


    //		8. traits

    //		8. traits

    pub trait MergeSortStTrait {
        /// Merge two sorted sequences into one sorted sequence.
        /// - APAS: Work Θ(n), Span Θ(lg n) — parallel merge assumed for merge sort analysis.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential two-pointer merge, Span = Work.
        fn merge(left: &ArraySeqStPerS<N>, right: &ArraySeqStPerS<N>) -> (merged: ArraySeqStPerS<N>)
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
        /// - APAS: Work Θ(n lg n), Span Θ(lg² n) — with parallel merge and recursive parallelism.
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — sequential merge sort, Span = Work.
        fn merge_sort(a: &ArraySeqStPerS<N>) -> (sorted: ArraySeqStPerS<N>)
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_sort_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    Seq::new(sorted.spec_len(), |i: int| sorted.spec_index(i)));
    }


    //		9. impls

    impl MergeSortStTrait for ArraySeqStPerS<N> {
        fn merge(left: &ArraySeqStPerS<N>, right: &ArraySeqStPerS<N>) -> (merged: ArraySeqStPerS<N>) {
            let n_left = left.length();
            let n_right = right.length();
            let total = n_left + n_right;
            let ghost sl = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
            let ghost sr = Seq::new(right.spec_len(), |i: int| right.spec_index(i));

            let mut out: Vec<N> = Vec::with_capacity(total);
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
                    proof {
                        lemma_push_sorted(out@, v);
                        assert(sl.take(li as int + 1) =~= sl.take(li as int).push(sl[li as int]));
                        lemma_multiset_commutative(sl.take(li as int + 1), sr.take(ri as int));
                        lemma_multiset_commutative(sl.take(li as int), sr.take(ri as int));
                    }
                    out.push(v);
                    li = li + 1;
                } else {
                    let v = *right.nth(ri);
                    proof {
                        lemma_push_sorted(out@, v);
                        assert(sr.take(ri as int + 1) =~= sr.take(ri as int).push(sr[ri as int]));
                        lemma_multiset_commutative(sl.take(li as int), sr.take(ri as int + 1));
                        lemma_multiset_commutative(sl.take(li as int), sr.take(ri as int));
                    }
                    out.push(v);
                    ri = ri + 1;
                }
            }

            proof {
                assert(sl.take(n_left as int) =~= sl);
                assert(sr.take(n_right as int) =~= sr);
            }
            let ghost out_view = out@;
            let merged_result = ArraySeqStPerS { seq: out };
            proof {
                assert(Seq::new(merged_result.spec_len(), |i: int| merged_result.spec_index(i)) =~= out_view);
                assert(out_view.to_multiset() =~= (sl + sr).to_multiset());
                assert(spec_merge_post(sl, sr, Seq::new(merged_result.spec_len(), |i: int| merged_result.spec_index(i))));
            }
            merged_result
        }

        fn merge_sort(a: &ArraySeqStPerS<N>) -> (sorted: ArraySeqStPerS<N>)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost sa = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            if n == 0 {
                proof {
                    assert(sa =~= Seq::<N>::empty());
                }
                return ArraySeqStPerS::empty();
            }
            if n == 1 {
                let s = ArraySeqStPerS::singleton(*a.nth(0));
                proof {
                    let s_view = Seq::new(s.spec_len(), |i: int| s.spec_index(i));
                    assert(sa.len() == 1);
                    assert(s_view.len() == 1);
                    assert(sa[0] == s_view[0]);
                    assert(sa =~= s_view);
                }
                return s;
            }

            let mid = n / 2;

            // Build left half [0..mid)
            let mut left_vec: Vec<N> = Vec::with_capacity(mid);
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
            let mut right_vec: Vec<N> = Vec::with_capacity(right_len);
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
            let sorted_right = Self::merge_sort(&right);

            // Merge sorted halves
            let merged = Self::merge(&sorted_left, &sorted_right);

            proof {
                let ghost sl_view = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
                let ghost sr_view = Seq::new(right.spec_len(), |i: int| right.spec_index(i));
                let ghost ssl = Seq::new(sorted_left.spec_len(), |i: int| sorted_left.spec_index(i));
                let ghost ssr = Seq::new(sorted_right.spec_len(), |i: int| sorted_right.spec_index(i));
                let ghost sm = Seq::new(merged.spec_len(), |i: int| merged.spec_index(i));

                assert(sl_view + sr_view =~= sa);
                lemma_multiset_commutative(ssl, ssr);
                lemma_multiset_commutative(sl_view, sr_view);
            }
            merged
        }
    }

    } // verus!
} // mod
