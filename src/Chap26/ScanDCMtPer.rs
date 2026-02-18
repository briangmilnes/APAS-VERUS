//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer scan - parallel implementation (Chapter 26, Section 3).
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	8. traits
//	9. impls

//		1. module

pub mod ScanDCMtPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap26::ScanDCStPer::ScanDCStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap26::DivConReduceStPer::DivConReduceStPer::{spec_sum_fn, spec_wrapping_add};
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    // Monoid fold_left lemma: fold_left(s, x, f) == f(x, fold_left(s, id, f)) for monoids.
    // Duplicated from ScanDCStPer since we operate on MtPer types and need it locally.
    proof fn lemma_fold_left_monoid(s: Seq<N>, x: N, f: spec_fn(N, N) -> N, id: N)
        requires spec_monoid(f, id),
        ensures s.fold_left(x, f) == f(x, s.fold_left(id, f)),
        decreases s.len(),
    {
        reveal(Seq::fold_left);
        if s.len() == 0 {
        } else {
            lemma_fold_left_monoid(s.drop_last(), x, f, id);
            lemma_fold_left_monoid(s.drop_last(), id, f, id);
        }
    }

    //		8. traits

    pub trait ScanDCMtTrait {
        /// Exclusive prefix sums via parallel divide-and-conquer scan.
        /// Returns (prefixes, total) where prefixes[i] = sum(a[0], ..., a[i-1]).
        /// - APAS: Work Θ(n lg n), Span Θ(lg n) — Algorithm 26.5 with parallel recursive calls.
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span depends on tabulate/append — Θ(lg n) if parallel O(1), Θ(n) if sequential.
        fn prefix_sums_dc_parallel(a: &ArraySeqMtPerS<N>) -> (result: (ArraySeqMtPerS<N>, N))
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_scan_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    spec_sum_fn(), 0,
                    Seq::new(result.0.spec_len(), |i: int| result.0.spec_index(i)),
                    result.1);
    }

    //		9. impls

    // Parallel prefix sums: structural logic verified, recursion parallelized.
    fn prefix_sums_dc_inner(a: &ArraySeqMtPerS<N>) -> (result: (ArraySeqMtPerS<N>, N))
        requires a.spec_len() <= usize::MAX,
        ensures
            spec_scan_post(
                Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                spec_sum_fn(), 0,
                Seq::new(result.0.spec_len(), |i: int| result.0.spec_index(i)),
                result.1),
        decreases a.spec_len(),
    {
        let n = a.length();
        let ghost input = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
        let ghost spec_f = spec_sum_fn();

        if n == 0 {
            proof {
                reveal(Seq::fold_left);
                assert(input =~= Seq::<N>::empty());
            }
            return (ArraySeqMtPerS::empty(), 0);
        }
        if n == 1 {
            let total = (*a.nth(0)).wrapping_add(0);
            proof {
                reveal(Seq::fold_left);
                assert(input.len() == 1);
                assert(input.drop_last() =~= Seq::<N>::empty());
                assert(input.last() == a.spec_index(0));
                assert(Seq::<N>::empty().fold_left(0 as N, spec_f) == 0 as N);
                assert(input.fold_left(0 as N, spec_f) == spec_f(0 as N, a.spec_index(0)));
                assert(input.take(0) =~= Seq::<N>::empty());
            }
            return (ArraySeqMtPerS::singleton(0), total);
        }

        let mid = n / 2;

        // Build left half.
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

        // Build right half.
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

        let ghost left_input = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
        let ghost right_input = Seq::new(right.spec_len(), |i: int| right.spec_index(i));

        proof {
            assert(left_input =~= input.subrange(0, mid as int));
            assert(right_input =~= input.subrange(mid as int, n as int));
            assert(left_input + right_input =~= input);
        }

        // Parallel recursive calls via help-first scheduler.
        let f1 = move || -> (r: (ArraySeqMtPerS<N>, N))
            ensures spec_scan_post(left_input, spec_sum_fn(), 0, Seq::new(r.0.spec_len(), |i: int| r.0.spec_index(i)), r.1)
        { prefix_sums_dc_inner(&left) };

        let f2 = move || -> (r: (ArraySeqMtPerS<N>, N))
            ensures spec_scan_post(right_input, spec_sum_fn(), 0, Seq::new(r.0.spec_len(), |i: int| r.0.spec_index(i)), r.1)
        { prefix_sums_dc_inner(&right) };

        let (left_result, right_result) = join(f1, f2);
        let (l_prefixes, l_total) = left_result;
        let (r_prefixes, r_total) = right_result;

        let ghost l_pref_view = Seq::new(l_prefixes.spec_len(), |i: int| l_prefixes.spec_index(i));
        let ghost r_pref_view = Seq::new(r_prefixes.spec_len(), |i: int| r_prefixes.spec_index(i));

        // Adjust right prefixes: r_adjusted[j] = l_total + r_prefixes[j].
        let r_len = r_prefixes.length();
        let mut adj_vec: Vec<N> = Vec::with_capacity(r_len);
        let mut i: usize = 0;
        while i < r_len
            invariant
                i <= r_len,
                r_len == r_prefixes.spec_len(),
                r_pref_view.len() == r_len as int,
                adj_vec@.len() == i as int,
                forall|j: int| #![trigger r_pref_view[j]] 0 <= j < r_len as int
                    ==> r_pref_view[j] == r_prefixes.spec_index(j),
                forall|j: int| #![trigger adj_vec@[j]] 0 <= j < i as int
                    ==> adj_vec@[j] == spec_wrapping_add(l_total, r_pref_view[j]),
            decreases r_len - i,
        {
            let r_val = *r_prefixes.nth(i);
            let v = l_total.wrapping_add(r_val);
            adj_vec.push(v);
            i = i + 1;
        }

        // Concatenate l_prefixes and adjusted right.
        let l_len = l_prefixes.length();
        let mut result_vec: Vec<N> = Vec::with_capacity(n);
        let mut i: usize = 0;
        while i < l_len
            invariant
                i <= l_len,
                l_len == l_prefixes.spec_len(),
                l_pref_view.len() == l_len as int,
                forall|j: int| #![trigger l_pref_view[j]] 0 <= j < l_len as int
                    ==> l_pref_view[j] == l_prefixes.spec_index(j),
                result_vec@.len() == i as int,
                forall|j: int| #![trigger result_vec@[j]] 0 <= j < i as int
                    ==> result_vec@[j] == l_pref_view[j],
            decreases l_len - i,
        {
            result_vec.push(*l_prefixes.nth(i));
            i = i + 1;
        }
        let mut i: usize = 0;
        while i < r_len
            invariant
                i <= r_len,
                r_len == r_prefixes.spec_len(),
                r_pref_view.len() == r_len as int,
                l_len == l_prefixes.spec_len(),
                l_pref_view.len() == l_len as int,
                l_len == mid,
                adj_vec@.len() == r_len as int,
                result_vec@.len() == (l_len + i) as int,
                forall|j: int| #![trigger adj_vec@[j]] 0 <= j < r_len as int
                    ==> adj_vec@[j] == spec_wrapping_add(l_total, r_pref_view[j]),
                forall|j: int| #![trigger result_vec@[j]] 0 <= j < l_len as int
                    ==> result_vec@[j] == l_pref_view[j],
                forall|j: int| #![trigger result_vec@[l_len as int + j]]
                    0 <= j < i as int
                    ==> result_vec@[l_len as int + j] == spec_wrapping_add(l_total, r_pref_view[j]),
            decreases r_len - i,
        {
            result_vec.push(adj_vec[i]);
            i = i + 1;
        }

        let total = l_total.wrapping_add(r_total);

        let ghost result_view = result_vec@;
        let result_prefixes = ArraySeqMtPerS { seq: result_vec };

        proof {
            let ghost rp = Seq::new(result_prefixes.spec_len(), |i: int| result_prefixes.spec_index(i));
            assert(rp =~= result_view);
            assert(result_view.len() == n as int);

            assert(left_input =~= input.subrange(0, mid as int));
            assert(right_input =~= input.subrange(mid as int, n as int));

            // Prove total == input.fold_left(0, spec_f).
            input.lemma_fold_left_split(0 as N, spec_f, mid as int);
            lemma_fold_left_monoid(right_input, l_total, spec_f, 0 as N);

            // Prove each prefix position.
            assert forall|i: int| #![trigger result_view[i]]
                0 <= i < n as int implies
                result_view[i] == spec_scan_at(input, spec_f, 0 as N, i)
            by {
                if i < mid as int {
                    assert(result_view[i] == l_pref_view[i]);
                    assert(input.take(i) =~= left_input.take(i));
                } else {
                    let j = i - mid as int;
                    assert(i == l_len as int + j);
                    assert(result_view[l_len as int + j] == spec_wrapping_add(l_total, r_pref_view[j]));
                    lemma_fold_left_monoid(right_input.take(j), l_total, spec_f, 0 as N);
                    assert(input.take(i).subrange(0, mid as int) =~= left_input);
                    assert(input.take(i).subrange(mid as int, i as int) =~= right_input.take(j));
                    input.take(i).lemma_fold_left_split(0 as N, spec_f, mid as int);
                }
            }
        }

        (result_prefixes, total)
    }

    impl ScanDCMtTrait for ArraySeqMtPerS<N> {
        fn prefix_sums_dc_parallel(a: &ArraySeqMtPerS<N>) -> (result: (ArraySeqMtPerS<N>, N)) {
            prefix_sums_dc_inner(a)
        }
    }

    } // verus!

} // mod
