//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer scan - parallel implementation (Chapter 26, Section 3).
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec fns
//	7. proof fns
//	8. traits
//	9. impls

//		1. module

pub mod ScanDCMtPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		4. spec fns

    /// Wrapping addition for usize — matches vstd wrapping_add spec with in-range casts.
    pub open spec fn spec_wrapping_add(x: N, y: N) -> N {
        if x + y > usize::MAX as int {
            ((x + y) - (usize::MAX as int + 1)) as N
        } else {
            (x + y) as N
        }
    }

    pub open spec fn spec_sum_fn() -> spec_fn(N, N) -> N { |x: N, y: N| spec_wrapping_add(x, y) }

    /// Spec function: exclusive prefix scan result at position i is the fold of elements [0..i).
    pub open spec fn spec_scan_at(s: Seq<N>, spec_f: spec_fn(N, N) -> N, id: N, i: int) -> N
        recommends 0 <= i <= s.len(),
    {
        s.take(i).fold_left(id, spec_f)
    }

    /// Spec function: full exclusive scan postcondition.
    pub open spec fn spec_scan_post(
        input: Seq<N>, spec_f: spec_fn(N, N) -> N, id: N,
        prefixes: Seq<N>, total: N,
    ) -> bool {
        &&& prefixes.len() == input.len()
        &&& forall|i: int| #![trigger prefixes[i]]
                0 <= i < input.len() ==> prefixes[i] == spec_scan_at(input, spec_f, id, i)
        &&& total == spec_iterate(input, spec_f, id)
    }

    //		7. proof fns

    // Monoid fold_left lemma: fold_left(s, x, f) == f(x, fold_left(s, id, f)) for monoids.
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
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n) — parallel recursion via join(), sequential Θ(n) combine: S(n) = S(n/2) + Θ(n) = Θ(n).
        fn prefix_sums_dc_parallel(a: &ArraySeqMtPerS<N>) -> (sums: (ArraySeqMtPerS<N>, N))
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_scan_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    spec_sum_fn(), 0,
                    Seq::new(sums.0.spec_len(), |i: int| sums.0.spec_index(i)),
                    sums.1);
    }

    //		9. impls

    // Parallel prefix sums: structural logic verified, recursion parallelized.
    fn prefix_sums_dc_inner(a: &ArraySeqMtPerS<N>) -> (sums: (ArraySeqMtPerS<N>, N))
        requires a.spec_len() <= usize::MAX,
        ensures
            spec_scan_post(
                Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                spec_sum_fn(), 0,
                Seq::new(sums.0.spec_len(), |i: int| sums.0.spec_index(i)),
                sums.1),
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

        // Parallel combine: copy left prefixes and adjust right prefixes via join().
        let l_len = l_prefixes.length();
        let r_len = r_prefixes.length();

        let ghost l_pv = l_pref_view;
        let ghost r_pv = r_pref_view;
        let lt = l_total;

        let copy_left = move || -> (r: Vec<N>)
            ensures
                r@.len() == l_pv.len(),
                forall|j: int| #![trigger r@[j]] 0 <= j < r@.len() ==> r@[j] == l_pv[j],
        {
            let ll = l_prefixes.length();
            let mut v: Vec<N> = Vec::with_capacity(ll);
            let mut i: usize = 0;
            while i < ll
                invariant
                    i <= ll, ll == l_prefixes.spec_len(),
                    l_pv.len() == ll as int,
                    v@.len() == i as int,
                    forall|j: int| #![trigger l_pv[j]] 0 <= j < ll as int
                        ==> l_pv[j] == l_prefixes.spec_index(j),
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int
                        ==> v@[j] == l_pv[j],
                decreases ll - i,
            { v.push(*l_prefixes.nth(i)); i += 1; }
            v
        };

        let adjust_right = move || -> (r: Vec<N>)
            ensures
                r@.len() == r_pv.len(),
                forall|j: int| #![trigger r@[j]] 0 <= j < r@.len()
                    ==> r@[j] == spec_wrapping_add(l_total, r_pv[j]),
        {
            let rl = r_prefixes.length();
            let mut v: Vec<N> = Vec::with_capacity(rl);
            let mut i: usize = 0;
            while i < rl
                invariant
                    i <= rl, rl == r_prefixes.spec_len(),
                    r_pv.len() == rl as int,
                    v@.len() == i as int,
                    forall|j: int| #![trigger r_pv[j]] 0 <= j < rl as int
                        ==> r_pv[j] == r_prefixes.spec_index(j),
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int
                        ==> v@[j] == spec_wrapping_add(lt, r_pv[j]),
                decreases rl - i,
            {
                let val = lt.wrapping_add(*r_prefixes.nth(i));
                v.push(val);
                i += 1;
            }
            v
        };

        let (left_part, right_part) = join(copy_left, adjust_right);

        // Sequential concatenation of the two halves.
        let mut result_vec: Vec<N> = left_part;
        let mut i: usize = 0;
        while i < r_len
            invariant
                i <= r_len,
                r_len == r_pref_view.len(),
                l_len == l_pref_view.len(),
                l_len == mid,
                right_part@.len() == r_len as int,
                result_vec@.len() == (l_len + i) as int,
                forall|j: int| #![trigger right_part@[j]] 0 <= j < r_len as int
                    ==> right_part@[j] == spec_wrapping_add(l_total, r_pref_view[j]),
                forall|j: int| #![trigger result_vec@[j]] 0 <= j < l_len as int
                    ==> result_vec@[j] == l_pref_view[j],
                forall|j: int| #![trigger result_vec@[l_len as int + j]]
                    0 <= j < i as int
                    ==> result_vec@[l_len as int + j] == spec_wrapping_add(l_total, r_pref_view[j]),
            decreases r_len - i,
        {
            result_vec.push(right_part[i]);
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
        fn prefix_sums_dc_parallel(a: &ArraySeqMtPerS<N>) -> (sums: (ArraySeqMtPerS<N>, N)) {
            prefix_sums_dc_inner(a)
        }
    }

    } // verus!

} // mod
