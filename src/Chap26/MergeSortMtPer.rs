//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Parallel merge sort implementation (Chapter 26).
//! merge_parallel uses a parallel binary-search D&C merge via join().
//! Span is O(n) due to Vec concatenation at each level; O(lg² n) requires O(1) concat.
//! Note: Unconditionally parallel - no thresholding per APAS rules.
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




pub mod MergeSortMtPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    //		2. imports

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
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

    //		4. spec fns

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

    //		7. proof fns

    /// If s.to_multiset().count(x) > 0, then x appears somewhere in s.
    proof fn lemma_multiset_count_positive_implies_exists(s: Seq<N>, x: N)
        requires s.to_multiset().count(x) > 0,
        ensures exists|j: int| #![trigger s[j]] 0 <= j < s.len() && s[j] == x,
        decreases s.len(),
    {
        if s.len() == 0 {
        } else if s.last() == x {
            assert(s[s.len() - 1] == x);
        } else {
            assert(s =~= s.drop_last().push(s.last()));
            assert(s.drop_last().to_multiset().insert(s.last()).count(x)
                == s.drop_last().to_multiset().count(x));
            lemma_multiset_count_positive_implies_exists(s.drop_last(), x);
            let j = choose|j: int| 0 <= j < s.drop_last().len() && s.drop_last()[j] == x;
            assert(s[j] == s.drop_last()[j]);
        }
    }

    /// A permutation of a sequence bounded above by `bound` is itself bounded above.
    proof fn lemma_all_le_preserved_by_permutation(a: Seq<N>, b: Seq<N>, bound: N)
        requires
            a.to_multiset() =~= b.to_multiset(),
            forall|i: int| #![trigger b[i]] 0 <= i < b.len() ==> b[i] <= bound,
        ensures
            forall|i: int| #![trigger a[i]] 0 <= i < a.len() ==> a[i] <= bound,
    {
        assert forall|i: int| #![trigger a[i]] 0 <= i < a.len() implies a[i] <= bound
        by {
            assert(a.to_multiset().count(a[i]) > 0);
            assert(b.to_multiset().count(a[i]) > 0);
            lemma_multiset_count_positive_implies_exists(b, a[i]);
            let j = choose|j: int| #![trigger b[j]] 0 <= j < b.len() && b[j] == a[i];
        }
    }

    /// A permutation of a sequence bounded below by `bound` is itself bounded below.
    proof fn lemma_all_ge_preserved_by_permutation(a: Seq<N>, b: Seq<N>, bound: N)
        requires
            a.to_multiset() =~= b.to_multiset(),
            forall|i: int| #![trigger b[i]] 0 <= i < b.len() ==> b[i] >= bound,
        ensures
            forall|i: int| #![trigger a[i]] 0 <= i < a.len() ==> a[i] >= bound,
    {
        assert forall|i: int| #![trigger a[i]] 0 <= i < a.len() implies a[i] >= bound
        by {
            assert(a.to_multiset().count(a[i]) > 0);
            assert(b.to_multiset().count(a[i]) > 0);
            lemma_multiset_count_positive_implies_exists(b, a[i]);
            let j = choose|j: int| #![trigger b[j]] 0 <= j < b.len() && b[j] == a[i];
        }
    }

    /// Concatenating sorted_left ++ [pivot] ++ sorted_right yields a sorted sequence
    /// when all of left ≤ pivot and all of right ≥ pivot.
    proof fn lemma_sorted_concat_pivot(a: Seq<N>, pivot: N, c: Seq<N>)
        requires
            spec_sorted(a),
            spec_sorted(c),
            forall|i: int| #![trigger a[i]] 0 <= i < a.len() ==> a[i] <= pivot,
            forall|i: int| #![trigger c[i]] 0 <= i < c.len() ==> c[i] >= pivot,
        ensures
            spec_sorted(a.push(pivot) + c),
    {
        let result = a.push(pivot) + c;
        let ap = a.push(pivot);
        assert forall|i: int, j: int|
            0 <= i < j < result.len() implies result[i] <= result[j]
        by {
            if j < a.len() as int {
            } else if j == a.len() as int {
            } else if i < a.len() as int {
                assert(result[j] == c[j - a.len() as int - 1]);
            } else if i == a.len() as int {
                assert(result[j] == c[j - a.len() as int - 1]);
            } else {
                assert(result[i] == c[i - a.len() as int - 1]);
                assert(result[j] == c[j - a.len() as int - 1]);
            }
        }
    }


    //		8. traits

    //		8. traits

    pub trait MergeSortMtTrait {
        /// Merge two sorted sequences using parallel binary-search divide and conquer.
        /// - APAS: Work Θ(n), Span Θ(lg n) — assumed for merge sort Span analysis.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — parallel D&C merge via join(); Vec concat at each level is Θ(n), dominating span. Θ(lg² n) requires O(1) concat (balanced tree).
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
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n) — parallel recursion via join(), Θ(n) merge: S(n) = S(n/2) + Θ(n) = Θ(n).
        fn merge_sort_parallel(a: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_sort_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));
    }


    //		9. impls

    //		9. impls

    /// Binary search in a sorted array: find the count of elements ≤ pivot.
    fn binary_search_upper_bound(arr: &ArraySeqMtPerS<N>, pivot: N) -> (pos: usize)
        requires spec_sorted(Seq::new(arr.spec_len(), |i: int| arr.spec_index(i))),
        ensures
            pos as int <= arr.spec_len(),
            forall|j: int| #![trigger arr.spec_index(j)]
                0 <= j < pos as int ==> arr.spec_index(j) <= pivot,
            forall|j: int| #![trigger arr.spec_index(j)]
                pos as int <= j < arr.spec_len() ==> arr.spec_index(j) > pivot,
    {
        let n = arr.length();
        let ghost s = Seq::new(arr.spec_len(), |i: int| arr.spec_index(i));
        let mut lo: usize = 0;
        let mut hi: usize = n;
        while lo < hi
            invariant
                lo <= hi <= n,
                n == arr.spec_len(),
                spec_sorted(s),
                s =~= Seq::new(arr.spec_len(), |i: int| arr.spec_index(i)),
                forall|j: int| #![trigger arr.spec_index(j)]
                    0 <= j < lo as int ==> arr.spec_index(j) <= pivot,
                forall|j: int| #![trigger arr.spec_index(j)]
                    hi as int <= j < n as int ==> arr.spec_index(j) > pivot,
            decreases hi - lo,
        {
            let m = lo + (hi - lo) / 2;
            if *arr.nth(m) <= pivot {
                proof {
                    assert forall|j: int| #![trigger arr.spec_index(j)]
                        0 <= j < (m + 1) as int implies arr.spec_index(j) <= pivot
                    by {
                        if j < lo as int {
                        } else {
                            assert(s[j] <= s[m as int]);
                        }
                    }
                }
                lo = m + 1;
            } else {
                proof {
                    assert forall|j: int| #![trigger arr.spec_index(j)]
                        m as int <= j < n as int implies arr.spec_index(j) > pivot
                    by {
                        if j >= hi as int {
                        } else {
                            assert(s[m as int] <= s[j]);
                        }
                    }
                }
                hi = m;
            }
        }
        lo
    }

    /// Parallel binary-search merge. Picks the median of left, binary searches in right,
    /// then recursively merges both halves in parallel via join().
    fn merge_dc(left: &ArraySeqMtPerS<N>, right: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
        requires
            spec_sorted(Seq::new(left.spec_len(), |i: int| left.spec_index(i))),
            spec_sorted(Seq::new(right.spec_len(), |i: int| right.spec_index(i))),
            left.spec_len() + right.spec_len() <= usize::MAX,
        ensures
            spec_merge_post(
                Seq::new(left.spec_len(), |i: int| left.spec_index(i)),
                Seq::new(right.spec_len(), |i: int| right.spec_index(i)),
                Seq::new(result.spec_len(), |i: int| result.spec_index(i))),
        decreases left.spec_len() + right.spec_len(),
    {
        let nl = left.length();
        let nr = right.length();
        let ghost sl = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
        let ghost sr = Seq::new(right.spec_len(), |i: int| right.spec_index(i));

        // Base: left empty — return copy of right.
        if nl == 0 {
            let mut v: Vec<N> = Vec::with_capacity(nr);
            let mut i: usize = 0;
            while i < nr
                invariant
                    i <= nr, nr == right.spec_len(),
                    v@.len() == i as int,
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int ==> v@[j] == right.spec_index(j),
                decreases nr - i,
            { v.push(*right.nth(i)); i += 1; }
            let res = ArraySeqMtPerS { seq: v };
            proof {
                let rv = Seq::new(res.spec_len(), |i: int| res.spec_index(i));
                assert(rv =~= sr);
                assert(sl + sr =~= sr);
            }
            return res;
        }
        // Base: right empty — return copy of left.
        if nr == 0 {
            let mut v: Vec<N> = Vec::with_capacity(nl);
            let mut i: usize = 0;
            while i < nl
                invariant
                    i <= nl, nl == left.spec_len(),
                    v@.len() == i as int,
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int ==> v@[j] == left.spec_index(j),
                decreases nl - i,
            { v.push(*left.nth(i)); i += 1; }
            let res = ArraySeqMtPerS { seq: v };
            proof {
                let rv = Seq::new(res.spec_len(), |i: int| res.spec_index(i));
                assert(rv =~= sl);
                assert(sl + sr =~= sl);
            }
            return res;
        }

        // Recursive case: nl >= 1, nr >= 1.
        let mid = nl / 2;
        let pivot = *left.nth(mid);
        let pos = binary_search_upper_bound(right, pivot);

        // Build 4 subarrays.
        let mut ll_vec: Vec<N> = Vec::with_capacity(mid);
        let mut i: usize = 0;
        while i < mid
            invariant
                i <= mid, mid <= nl, nl == left.spec_len(),
                ll_vec@.len() == i as int,
                forall|j: int| #![trigger ll_vec@[j]] 0 <= j < i as int ==> ll_vec@[j] == left.spec_index(j),
            decreases mid - i,
        { ll_vec.push(*left.nth(i)); i += 1; }
        let left_l = ArraySeqMtPerS { seq: ll_vec };

        let lr_len = nl - mid - 1;
        let mut lr_vec: Vec<N> = Vec::with_capacity(lr_len);
        let mut i: usize = 0;
        while i < lr_len
            invariant
                i <= lr_len, lr_len == nl - mid - 1,
                mid < nl, nl == left.spec_len(),
                lr_vec@.len() == i as int,
                forall|j: int| #![trigger lr_vec@[j]] 0 <= j < i as int ==> lr_vec@[j] == left.spec_index(mid as int + 1 + j),
            decreases lr_len - i,
        { lr_vec.push(*left.nth(mid + 1 + i)); i += 1; }
        let left_r = ArraySeqMtPerS { seq: lr_vec };

        let mut rl_vec: Vec<N> = Vec::with_capacity(pos);
        let mut i: usize = 0;
        while i < pos
            invariant
                i <= pos, pos as int <= nr, nr == right.spec_len(),
                rl_vec@.len() == i as int,
                forall|j: int| #![trigger rl_vec@[j]] 0 <= j < i as int ==> rl_vec@[j] == right.spec_index(j),
            decreases pos - i,
        { rl_vec.push(*right.nth(i)); i += 1; }
        let right_l = ArraySeqMtPerS { seq: rl_vec };

        let rr_len = nr - pos;
        let mut rr_vec: Vec<N> = Vec::with_capacity(rr_len);
        let mut i: usize = 0;
        while i < rr_len
            invariant
                i <= rr_len, rr_len == nr - pos,
                pos as int <= nr, nr == right.spec_len(),
                rr_vec@.len() == i as int,
                forall|j: int| #![trigger rr_vec@[j]] 0 <= j < i as int ==> rr_vec@[j] == right.spec_index(pos as int + j),
            decreases rr_len - i,
        { rr_vec.push(*right.nth(pos + i)); i += 1; }
        let right_r = ArraySeqMtPerS { seq: rr_vec };

        // Ghost views for subarrays.
        let ghost sl_l = Seq::new(left_l.spec_len(), |i: int| left_l.spec_index(i));
        let ghost sl_r = Seq::new(left_r.spec_len(), |i: int| left_r.spec_index(i));
        let ghost sr_l = Seq::new(right_l.spec_len(), |i: int| right_l.spec_index(i));
        let ghost sr_r = Seq::new(right_r.spec_len(), |i: int| right_r.spec_index(i));
        let ghost pivot_seq = Seq::new(1, |unused: int| pivot);

        // Prove subarrays are sorted (subrange of sorted is sorted).
        proof {
            assert(spec_sorted(sl_l)) by {
                assert forall|i: int, j: int| 0 <= i < j < sl_l.len()
                    implies sl_l[i] <= sl_l[j]
                by {
                    assert(sl_l[i] == sl[i]);
                    assert(sl_l[j] == sl[j]);
                }
            }
            assert(spec_sorted(sl_r)) by {
                assert forall|i: int, j: int| 0 <= i < j < sl_r.len()
                    implies sl_r[i] <= sl_r[j]
                by {
                    assert(sl_r[i] == sl[mid as int + 1 + i]);
                    assert(sl_r[j] == sl[mid as int + 1 + j]);
                }
            }
            assert(spec_sorted(sr_l)) by {
                assert forall|i: int, j: int| 0 <= i < j < sr_l.len()
                    implies sr_l[i] <= sr_l[j]
                by {
                    assert(sr_l[i] == sr[i]);
                    assert(sr_l[j] == sr[j]);
                }
            }
            assert(spec_sorted(sr_r)) by {
                assert forall|i: int, j: int| 0 <= i < j < sr_r.len()
                    implies sr_r[i] <= sr_r[j]
                by {
                    assert(sr_r[i] == sr[pos as int + i]);
                    assert(sr_r[j] == sr[pos as int + j]);
                }
            }
        }

        // Parallel recursive merge via join().
        let f1 = move || -> (r: ArraySeqMtPerS<N>)
            ensures spec_merge_post(sl_l, sr_l, Seq::new(r.spec_len(), |i: int| r.spec_index(i)))
        { merge_dc(&left_l, &right_l) };

        let f2 = move || -> (r: ArraySeqMtPerS<N>)
            ensures spec_merge_post(sl_r, sr_r, Seq::new(r.spec_len(), |i: int| r.spec_index(i)))
        { merge_dc(&left_r, &right_r) };

        let (merged_l, merged_r) = join(f1, f2);

        let ghost ml_view = Seq::new(merged_l.spec_len(), |i: int| merged_l.spec_index(i));
        let ghost mr_view = Seq::new(merged_r.spec_len(), |i: int| merged_r.spec_index(i));

        // Build result: merged_l ++ [pivot] ++ merged_r.
        let ml_len = merged_l.length();
        let mr_len = merged_r.length();
        let total = ml_len + 1 + mr_len;
        let mut out: Vec<N> = Vec::with_capacity(total);

        let mut i: usize = 0;
        while i < ml_len
            invariant
                i <= ml_len, ml_len == merged_l.spec_len(),
                out@.len() == i as int,
                forall|j: int| #![trigger out@[j]] 0 <= j < i as int ==> out@[j] == merged_l.spec_index(j),
            decreases ml_len - i,
        { out.push(*merged_l.nth(i)); i += 1; }

        out.push(pivot);

        let mut i: usize = 0;
        while i < mr_len
            invariant
                i <= mr_len, mr_len == merged_r.spec_len(),
                ml_len == merged_l.spec_len(),
                out@.len() == (ml_len as int + 1 + i as int),
                forall|j: int| #![trigger out@[j]] 0 <= j < ml_len as int ==> out@[j] == merged_l.spec_index(j),
                out@[ml_len as int] == pivot,
                forall|j: int| #![trigger out@[ml_len as int + 1 + j]]
                    0 <= j < i as int ==> out@[ml_len as int + 1 + j] == merged_r.spec_index(j),
            decreases mr_len - i,
        { out.push(*merged_r.nth(i)); i += 1; }

        let ghost out_view = out@;
        let result = ArraySeqMtPerS { seq: out };

        proof {
            let ghost rv = Seq::new(result.spec_len(), |i: int| result.spec_index(i));
            assert(rv =~= out_view);

            // Length.
            assert(rv.len() == (nl + nr) as int);

            // Sequence decompositions — help the solver with extensional equality.
            assert(sl =~= sl_l + pivot_seq + sl_r) by {
                assert(sl.len() == sl_l.len() + 1 + sl_r.len());
                assert forall|k: int| #![trigger sl[k]] 0 <= k < sl.len() implies
                    sl[k] == (sl_l + pivot_seq + sl_r)[k]
                by {
                    if k < sl_l.len() {
                        assert(sl_l[k] == sl[k]);
                    } else if k == mid as int {
                    } else {
                        assert(sl_r[k - mid as int - 1] == sl[k]);
                    }
                }
            }
            assert(sr =~= sr_l + sr_r) by {
                assert(sr.len() == sr_l.len() + sr_r.len());
                assert forall|k: int| #![trigger sr[k]] 0 <= k < sr.len() implies
                    sr[k] == (sr_l + sr_r)[k]
                by {
                    if k < sr_l.len() {
                        assert(sr_l[k] == sr[k]);
                    } else {
                        assert(sr_r[k - sr_l.len()] == sr[k]);
                    }
                }
            }
            assert(out_view =~= ml_view + pivot_seq + mr_view) by {
                assert(out_view.len() == ml_view.len() + 1 + mr_view.len());
                assert forall|k: int| #![trigger out_view[k]] 0 <= k < out_view.len() implies
                    out_view[k] == (ml_view + pivot_seq + mr_view)[k]
                by {
                    if k < ml_view.len() {
                        assert(out_view[k] == ml_view[k]);
                    } else if k == ml_view.len() {
                        assert(out_view[k] == pivot);
                    } else {
                        let ghost j = k - ml_len as int - 1;
                        assert(out@[ml_len as int + 1 + j] == merged_r.spec_index(j));
                        assert(mr_view[j] == merged_r.spec_index(j));
                    }
                }
            }

            // Sorted proof: all of ml ≤ pivot, all of mr ≥ pivot.
            let ghost input_l = sl_l + sr_l;
            let ghost input_r = sl_r + sr_r;

            assert forall|k: int| #![trigger input_l[k]]
                0 <= k < input_l.len() implies input_l[k] <= pivot
            by {
                if k < sl_l.len() {
                    assert(sl_l[k] == sl[k]);
                    assert(sl[k] <= sl[mid as int]);
                } else {
                    assert(input_l[k] == sr_l[k - sl_l.len()]);
                }
            }

            assert forall|k: int| #![trigger input_r[k]]
                0 <= k < input_r.len() implies input_r[k] >= pivot
            by {
                if k < sl_r.len() {
                    assert(input_r[k] == sl_r[k]);
                    assert(sl_r[k] == sl[mid as int + 1 + k]);
                    assert(sl[mid as int] <= sl[mid as int + 1 + k]);
                } else {
                    assert(input_r[k] == sr_r[k - sl_r.len()]);
                    assert(sr_r[k - sl_r.len()] == sr[pos as int + (k - sl_r.len())]);
                }
            }

            lemma_all_le_preserved_by_permutation(ml_view, input_l, pivot);
            lemma_all_ge_preserved_by_permutation(mr_view, input_r, pivot);
            lemma_sorted_concat_pivot(ml_view, pivot, mr_view);
            assert(out_view =~= ml_view.push(pivot) + mr_view);
            assert(spec_sorted(out_view));

            // Permutation proof: rv.to_multiset() =~= (sl + sr).to_multiset().
            // Use lemma_multiset_commutative to trigger concat-to-multiset axioms.
            lemma_multiset_commutative(sl, sr);
            lemma_multiset_commutative(sl_l + pivot_seq, sl_r);
            lemma_multiset_commutative(sl_l, pivot_seq);
            lemma_multiset_commutative(sr_l, sr_r);
            lemma_multiset_commutative(ml_view + pivot_seq, mr_view);
            lemma_multiset_commutative(ml_view, pivot_seq);
            lemma_multiset_commutative(sl_l, sr_l);
            lemma_multiset_commutative(sl_r, sr_r);
            lemma_multiset_commutative(pivot_seq + sl_r, sr_l);
        }

        result
    }

    impl MergeSortMtTrait for ArraySeqMtPerS<N> {
        fn merge_parallel(left: &ArraySeqMtPerS<N>, right: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>) {
            merge_dc(left, right)
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
