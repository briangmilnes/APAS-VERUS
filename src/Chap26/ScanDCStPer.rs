//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Divide-and-conquer scan - sequential implementation (Chapter 26, Section 3).
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


pub mod ScanDCStPer {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap26::DivConReduceStPer::DivConReduceStPer::{spec_sum_fn, spec_wrapping_add};
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 6. spec fns


    /// Spec function: exclusive prefix scan result at position i is the fold of elements [0..i).
    pub open spec fn spec_scan_at(s: Seq<usize>, spec_f: spec_fn(usize, usize) -> usize, id: usize, i: int) -> usize
        recommends 0 <= i <= s.len(),
    {
        s.take(i).fold_left(id, spec_f)
    }

    /// Spec function: full exclusive scan postcondition.
    pub open spec fn spec_scan_post(
        input: Seq<usize>, spec_f: spec_fn(usize, usize) -> usize, id: usize,
        prefixes: Seq<usize>, total: usize,
    ) -> bool {
        &&& prefixes.len() == input.len()
        &&& forall|i: int| #![trigger prefixes[i]]
                0 <= i < input.len() ==> prefixes[i] == spec_scan_at(input, spec_f, id, i)
        &&& total == spec_iterate(input, spec_f, id)
    }

    //		Section 7. proof fns/broadcast groups


    /// Monoid fold_left lemma: fold_left(s, x, f) == f(x, fold_left(s, id, f))
    /// when (f, id) is a monoid.
    pub proof fn lemma_fold_left_monoid(s: Seq<usize>, x: usize, f: spec_fn(usize, usize) -> usize, id: usize)
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

    //		Section 8. traits


    pub trait ScanDCStTrait {
        /// Algorithm 26.5: Exclusive prefix scan via divide and conquer.
        /// Returns (prefixes, total) where prefixes[i] = f(id, a[0], ..., a[i-1]).
        /// - Alg Analysis: APAS (Ch26 Alg 26.5): Work O(n lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — ACCEPTED DIFFERENCE: sequential recursion, no parallel split
        fn scan_dc<F: Fn(&usize, &usize) -> usize>(a: &ArraySeqStPerS<usize>, f: &F, Ghost(spec_f): Ghost<spec_fn(usize, usize) -> usize>, id: usize) -> (scanned: (ArraySeqStPerS<usize>, usize))
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_f, id),
                forall|x: &usize, y: &usize| #[trigger] f.requires((x, y)),
                forall|x: usize, y: usize, ret: usize| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                spec_scan_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    spec_f, id,
                    Seq::new(scanned.0.spec_len(), |i: int| scanned.0.spec_index(i)),
                    scanned.1);

        /// Exclusive prefix sums via divide-and-conquer scan.
        /// Convenience: scan_dc with (+, 0).
        /// - Alg Analysis: APAS (Ch26 Alg 26.5): Work O(n lg n), Span O(lg n) — same as scan_dc.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — delegates to sequential scan_dc.
        fn prefix_sums_dc(a: &ArraySeqStPerS<usize>) -> (sums: (ArraySeqStPerS<usize>, usize))
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_scan_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    spec_sum_fn(), 0,
                    Seq::new(sums.0.spec_len(), |i: int| sums.0.spec_index(i)),
                    sums.1);
    }

    //		Section 9. impls


    impl ScanDCStTrait for ArraySeqStPerS<usize> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive D&C scan halving n; St sequential.
        fn scan_dc<F: Fn(&usize, &usize) -> usize>(a: &ArraySeqStPerS<usize>, f: &F, Ghost(spec_f): Ghost<spec_fn(usize, usize) -> usize>, id: usize) -> (scanned: (ArraySeqStPerS<usize>, usize))
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost input = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            if n == 0 {
// Veracity: UNNEEDED proof block                 proof {
// Veracity: UNNEEDED proof block                     reveal(Seq::fold_left);
// Veracity: UNNEEDED proof block                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block                     assert(input =~= Seq::<usize>::empty());
// Veracity: UNNEEDED proof block                 }
                return (ArraySeqStPerS::empty(), id);
            }
            if n == 1 {
                let total = f(&id, a.nth(0));
                // Veracity: NEEDED proof block
                proof {
                    reveal(Seq::fold_left);
                    // Veracity: NEEDED assert (speed hint)
                    assert(input.len() == 1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(input.drop_last() =~= Seq::<usize>::empty());
                    // Veracity: NEEDED assert (speed hint)
                    assert(input.last() == a.spec_index(0));
                    // Veracity: NEEDED assert
                    assert(Seq::<usize>::empty().fold_left(id, spec_f) == id);
                    // Veracity: NEEDED assert (speed hint)
                    assert(input.fold_left(id, spec_f) == spec_f(id, a.spec_index(0)));
                    // Veracity: NEEDED assert (speed hint)
                    assert(input.take(0) =~= Seq::<usize>::empty());
                }
                return (ArraySeqStPerS::singleton(id), total);
            }

            let mid = n / 2;

            // Build left half
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

            // Build right half
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

            let ghost left_input = Seq::new(left.spec_len(), |i: int| left.spec_index(i));
            let ghost right_input = Seq::new(right.spec_len(), |i: int| right.spec_index(i));
// Veracity: NEEDED proof block

            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(left_input =~= input.subrange(0, mid as int));
                // Veracity: NEEDED assert (speed hint)
                assert(right_input =~= input.subrange(mid as int, n as int));
                // Veracity: NEEDED assert (speed hint)
                assert(left_input + right_input =~= input);
            }

            // Recursive calls
            let (l_prefixes, l_total) = Self::scan_dc(&left, f, Ghost(spec_f), id);
            let (r_prefixes, r_total) = Self::scan_dc(&right, f, Ghost(spec_f), id);

            let ghost l_pref_view = Seq::new(l_prefixes.spec_len(), |i: int| l_prefixes.spec_index(i));
            let ghost r_pref_view = Seq::new(r_prefixes.spec_len(), |i: int| r_prefixes.spec_index(i));

            // Adjust right prefixes: r_adjusted[j] = f(l_total, r_prefixes[j])
            let r_len = r_prefixes.length();
            let mut adj_vec: Vec<usize> = Vec::with_capacity(r_len);
            let mut i: usize = 0;
            while i < r_len
                invariant
                    i <= r_len,
                    r_len == r_prefixes.spec_len(),
                    r_pref_view.len() == r_len as int,
                    adj_vec@.len() == i as int,
                    forall|x: &usize, y: &usize| #[trigger] f.requires((x, y)),
                    forall|x: usize, y: usize, ret: usize| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    forall|j: int| #![trigger r_pref_view[j]] 0 <= j < r_len as int
                        ==> r_pref_view[j] == r_prefixes.spec_index(j),
                    forall|j: int| #![trigger adj_vec@[j]] 0 <= j < i as int
                        ==> adj_vec@[j] == spec_f(l_total, r_pref_view[j]),
                decreases r_len - i,
            {
                let r_val = *r_prefixes.nth(i);
                let v = f(&l_total, &r_val);
                adj_vec.push(v);
                i = i + 1;
            }

            // Concatenate l_prefixes and adjusted right
            let l_len = l_prefixes.length();
            let mut result_vec: Vec<usize> = Vec::with_capacity(n);
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
                        ==> adj_vec@[j] == spec_f(l_total, r_pref_view[j]),
                    forall|j: int| #![trigger result_vec@[j]] 0 <= j < l_len as int
                        ==> result_vec@[j] == l_pref_view[j],
                    forall|j: int| #![trigger result_vec@[l_len as int + j]]
                        0 <= j < i as int
                        ==> result_vec@[l_len as int + j] == spec_f(l_total, r_pref_view[j]),
                decreases r_len - i,
            {
                result_vec.push(adj_vec[i]);
                i = i + 1;
            }

            let total = f(&l_total, &r_total);

            let ghost result_view = result_vec@;
            // Veracity: NEEDED proof block
            let result_prefixes = ArraySeqStPerS { seq: result_vec };

            proof {
                let ghost rp = Seq::new(result_prefixes.spec_len(), |i: int| result_prefixes.spec_index(i));
                // Veracity: NEEDED assert (speed hint)
                assert(rp =~= result_view);
                // Veracity: NEEDED assert (speed hint)
                assert(result_view.len() == n as int);

                // Establish subrange equalities for fold_left_split
                // Veracity: NEEDED assert (speed hint)
                assert(left_input =~= input.subrange(0, mid as int));
                // Veracity: NEEDED assert (speed hint)
                assert(right_input =~= input.subrange(mid as int, n as int));

                // Prove total == input.fold_left(id, spec_f)
                // fold_left_split: right_input.fold_left(l_total, spec_f) == input.fold_left(id, spec_f)
                input.lemma_fold_left_split(id, spec_f, mid as int);
                // monoid: right_input.fold_left(l_total, spec_f) == spec_f(l_total, r_total)
                lemma_fold_left_monoid(right_input, l_total, spec_f, id);

                // Prove each prefix position
                // Veracity: NEEDED assert
                assert forall|i: int| #![trigger result_view[i]]
                    0 <= i < n as int implies
                    result_view[i] == spec_scan_at(input, spec_f, id, i)
                by {
                    if i < mid as int {
// Veracity: UNNEEDED assert                         assert(result_view[i] == l_pref_view[i]);
                        // Veracity: NEEDED assert
                        assert(input.take(i) =~= left_input.take(i));
                    } else {
                        let j = i - mid as int;
                        // Veracity: NEEDED assert (speed hint)
                        assert(i == l_len as int + j);
                        // Veracity: NEEDED assert (speed hint)
                        assert(result_view[l_len as int + j] == spec_f(l_total, r_pref_view[j]));
                        lemma_fold_left_monoid(right_input.take(j), l_total, spec_f, id);
                        // Veracity: NEEDED assert
                        assert(input.take(i).subrange(0, mid as int) =~= left_input);
                        // Veracity: NEEDED assert
                        assert(input.take(i).subrange(mid as int, i as int) =~= right_input.take(j));
                        input.take(i).lemma_fold_left_split(id, spec_f, mid as int);
                    }
                }
            }

            (result_prefixes, total)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to scan_dc with addition; St sequential.
        fn prefix_sums_dc(a: &ArraySeqStPerS<usize>) -> (sums: (ArraySeqStPerS<usize>, usize)) {
            Self::scan_dc(a,
                &(|x: &usize, y: &usize| -> (ret: usize)
                    ensures ret == spec_wrapping_add(*x, *y)
                { (*x).wrapping_add(*y) }),
                Ghost(spec_sum_fn()), 0)
        }
    }

    } // verus!
} // mod
