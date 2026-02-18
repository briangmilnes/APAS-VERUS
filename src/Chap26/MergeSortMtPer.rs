//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel merge sort implementation (Chapter 26).
//! merge_parallel uses a verified sequential two-pointer merge (same proof as StPer).
//! The parallel binary-search merge can be restored when its proof is written.
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	7. proof fns
//	8. traits
//	9. impls

//		1. module

pub mod MergeSortMtPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap26::MergeSortStPer::MergeSortStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
    };

    #[cfg(verus_keep_ghost)]
    use vstd::seq_lib::lemma_multiset_commutative;

    //		7. proof fns

    // Pushing an element >= all existing preserves sorted.
    // (Duplicated from MergeSortStPer for use with MtPer Seq<N>.)
    proof fn lemma_push_sorted(s: Seq<N>, v: N)
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
                if s.len() > 0 {
                } else {
                }
            }
        }
    }

    //		8. traits

    pub trait MergeSortMtTrait {
        /// Merge two sorted sequences. Currently uses sequential two-pointer merge for
        /// verified correctness; the parallel binary-search merge can be restored later.
        /// - APAS: Work Θ(n), Span Θ(lg n) — assumed for merge sort Span analysis.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential two-pointer merge (verified).
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

        /// Sort a sequence using parallel merge sort. Algorithm 26.4.
        /// - APAS: Work Θ(n lg n), Span Θ(lg² n) — with O(lg n)-span merge.
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — sequential merge, parallel recursion.
        fn merge_sort_parallel(a: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_sort_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));
    }

    //		9. impls

    impl MergeSortMtTrait for ArraySeqMtPerS<N> {
        // Verified sequential two-pointer merge (same algorithm as MergeSortStPer::merge).
        fn merge_parallel(left: &ArraySeqMtPerS<N>, right: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>) {
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
            let merged_result = ArraySeqMtPerS { seq: out };
            proof {
                assert(Seq::new(merged_result.spec_len(), |i: int| merged_result.spec_index(i)) =~= out_view);
                assert(out_view.to_multiset() =~= (sl + sr).to_multiset());
                assert(spec_merge_post(sl, sr, Seq::new(merged_result.spec_len(), |i: int| merged_result.spec_index(i))));
            }
            merged_result
        }

        // Verified parallel merge sort: structural logic proven, recursion parallelized.
        fn merge_sort_parallel(a: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost sa = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            if n == 0 {
                proof {
                    assert(sa =~= Seq::<N>::empty());
                }
                return ArraySeqMtPerS::empty();
            }
            if n == 1 {
                let s = ArraySeqMtPerS::singleton(*a.nth(0));
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

            // Build left half [0..mid).
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
            let left = ArraySeqMtPerS { seq: left_vec };

            // Build right half [mid..n).
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
            let right = ArraySeqMtPerS { seq: right_vec };

            // Capture ghost views before moving data into closures.
            let ghost left_view = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
            let ghost right_view = Seq::new(right.spec_len(), |i: int| right.spec_index(i));

            // Parallel recursive sort via help-first scheduler.
            let f1 = move || -> (r: ArraySeqMtPerS<N>)
                ensures spec_sort_post(left_view, Seq::new(r.spec_len(), |i: int| r.spec_index(i)))
            { <ArraySeqMtPerS<N> as MergeSortMtTrait>::merge_sort_parallel(&left) };

            let f2 = move || -> (r: ArraySeqMtPerS<N>)
                ensures spec_sort_post(right_view, Seq::new(r.spec_len(), |i: int| r.spec_index(i)))
            { <ArraySeqMtPerS<N> as MergeSortMtTrait>::merge_sort_parallel(&right) };

            let (sorted_left, sorted_right) = join(f1, f2);

            // Merge sorted halves.
            let merged = Self::merge_parallel(&sorted_left, &sorted_right);

            proof {
                let ghost ssl = Seq::new(sorted_left.spec_len(), |i: int| sorted_left.spec_index(i));
                let ghost ssr = Seq::new(sorted_right.spec_len(), |i: int| sorted_right.spec_index(i));
                let ghost sm = Seq::new(merged.spec_len(), |i: int| merged.spec_index(i));

                assert(left_view + right_view =~= sa);
                lemma_multiset_commutative(ssl, ssr);
                lemma_multiset_commutative(left_view, right_view);
            }
            merged
        }
    }

    } // verus!

} // mod
