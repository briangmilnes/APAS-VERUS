//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 36 (Multi-threaded Slice): Quicksort over `ArraySeqMtEphSlice`.
//! Three self-recursive trait methods: quick_sort_first, quick_sort_median3, quick_sort_random.
//! Each applies its pivot strategy at every recursive level.
//! Parallel D&C three-way partition via join; parallel recursion via ParaPair!.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod QuickSortMtEphSlice {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Types::Types::Pair;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::rand::rand::random_usize_range;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use vstd::multiset::Multiset;
    use vstd::relations::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		Section 6. spec fns


    /// Spec-level leq closure for sort_by and sorted_by.
    pub open spec fn spec_leq<T: TotalOrder>() -> spec_fn(T, T) -> bool {
        |x: T, y: T| T::le(x, y)
    }

    /// Extract the element sequence from a slice as Seq<T>.
    pub open spec fn elements<T: StTInMtT>(a: ArraySeqMtEphSliceS<T>) -> Seq<T> {
        Seq::new(a.spec_len(), |i: int| a.spec_index(i))
    }

    /// Median of three values: returns the value that is neither min nor max.
    pub open spec fn spec_median_of_three<T: TotalOrder>(a: T, b: T, c: T) -> T {
        if T::le(a, b) {
            if T::le(b, c) { b }
            else if T::le(a, c) { c }
            else { a }
        } else {
            if T::le(a, c) { a }
            else if T::le(b, c) { c }
            else { b }
        }
    }

    //		Section 7. proof fns/broadcast groups


    /// Bridge from the TotalOrder trait to vstd's total_ordering predicate.
    proof fn lemma_total_ordering<T: TotalOrder>()
        ensures total_ordering(spec_leq::<T>())
    {
        let leq = spec_leq::<T>();
        assert(reflexive(leq)) by {
            assert forall|x: T| #[trigger] leq(x, x) by { T::reflexive(x); }
        };
        assert(antisymmetric(leq)) by {
            assert forall|x: T, y: T|
                #[trigger] leq(x, y) && #[trigger] leq(y, x) implies x == y by
            { T::antisymmetric(x, y); }
        };
        assert(transitive(leq)) by {
            assert forall|x: T, y: T, z: T|
                #[trigger] leq(x, y) && #[trigger] leq(y, z) implies leq(x, z) by
            { T::transitive(x, y, z); }
        };
        assert(strongly_connected(leq)) by {
            assert forall|x: T, y: T|
                #[trigger] leq(x, y) || #[trigger] leq(y, x) by
            { T::total(x, y); }
        };
    }

    /// Given a three-way partition and independently sorted halves, prove
    /// sorted_left ++ equals ++ sorted_right equals original.sort_by(leq).
    proof fn lemma_partition_sort_concat<T: TotalOrder>(
        original: Seq<T>,
        left_view: Seq<T>,
        right_view: Seq<T>,
        eq_view: Seq<T>,
        sorted_left: Seq<T>,
        sorted_right: Seq<T>,
        pivot: T,
    )
        requires
            total_ordering(spec_leq::<T>()),
            sorted_left =~= left_view.sort_by(spec_leq::<T>()),
            sorted_right =~= right_view.sort_by(spec_leq::<T>()),
            forall|j: int| 0 <= j < left_view.len() ==>
                (#[trigger] T::le(left_view[j], pivot)) && left_view[j] != pivot,
            forall|j: int| 0 <= j < right_view.len() ==>
                (#[trigger] T::le(pivot, right_view[j])) && right_view[j] != pivot,
            forall|j: int| 0 <= j < eq_view.len() ==>
                (#[trigger] eq_view[j]) == pivot,
            eq_view.len() >= 1,
            original.to_multiset() =~=
                left_view.to_multiset().add(right_view.to_multiset()).add(eq_view.to_multiset()),
        ensures
            (sorted_left + eq_view + sorted_right) =~= original.sort_by(spec_leq::<T>()),
    {
        let leq = spec_leq::<T>();
        let candidate = sorted_left + eq_view + sorted_right;

        left_view.lemma_sort_by_ensures(leq);
        right_view.lemma_sort_by_ensures(leq);
        original.lemma_sort_by_ensures(leq);

        assert(sorted_left.to_multiset() =~= left_view.to_multiset());
        assert(sorted_right.to_multiset() =~= right_view.to_multiset());

        assert forall|j: int| 0 <= j < sorted_left.len() implies
            T::le(#[trigger] sorted_left[j], pivot) && sorted_left[j] != pivot by
        {
            assert(sorted_left.to_multiset().count(sorted_left[j]) > 0);
            assert(left_view.to_multiset().count(sorted_left[j]) > 0);
            assert(left_view.contains(sorted_left[j]));
            let idx = choose|idx: int|
                0 <= idx < left_view.len() && left_view[idx] == sorted_left[j];
        };

        assert forall|j: int| 0 <= j < sorted_right.len() implies
            T::le(pivot, #[trigger] sorted_right[j]) && sorted_right[j] != pivot by
        {
            assert(sorted_right.to_multiset().count(sorted_right[j]) > 0);
            assert(right_view.to_multiset().count(sorted_right[j]) > 0);
            assert(right_view.contains(sorted_right[j]));
            let idx = choose|idx: int|
                0 <= idx < right_view.len() && right_view[idx] == sorted_right[j];
        };

        assert(sorted_by(candidate, leq)) by {
            assert forall|ai: int, bi: int|
                0 <= ai < bi < candidate.len()
                implies (#[trigger] leq(candidate[ai], candidate[bi])) by
            {
                let ll = sorted_left.len();
                let ell = eq_view.len();
                if ai < ll && bi < ll {
                } else if ai < ll && bi < ll + ell {
                    assert(candidate[bi] == pivot);
                } else if ai < ll && bi >= ll + ell {
                    T::transitive(candidate[ai], pivot, candidate[bi]);
                } else if ai >= ll && ai < ll + ell && bi >= ll && bi < ll + ell {
                    assert(candidate[ai] == pivot && candidate[bi] == pivot);
                    T::reflexive(pivot);
                } else if ai >= ll && ai < ll + ell && bi >= ll + ell {
                    assert(candidate[ai] == pivot);
                } else {
                    assert(ai >= ll + ell && bi >= ll + ell);
                }
            };
        };

        vstd::seq_lib::lemma_multiset_commutative(sorted_left, eq_view);
        vstd::seq_lib::lemma_multiset_commutative(sorted_left + eq_view, sorted_right);
        assert(candidate.to_multiset() =~=
            sorted_left.to_multiset().add(eq_view.to_multiset()).add(sorted_right.to_multiset()));
        assert(candidate.to_multiset() =~=
            left_view.to_multiset().add(eq_view.to_multiset()).add(right_view.to_multiset()));
        assert(candidate.to_multiset() =~= original.to_multiset());

        vstd::seq_lib::lemma_sorted_unique(original.sort_by(leq), candidate, leq);
    }

    /// Prove elements(from_vec(v)) =~= v@.
    proof fn lemma_elements_from_vec<T: StTInMtT + TotalOrder>(v: Seq<T>, a: ArraySeqMtEphSliceS<T>)
        requires
            a.spec_arrayseqmtephslice_wf(),
            a.spec_len() == v.len(),
            forall|i: int| #![trigger a.spec_index(i)]
                0 <= i < v.len() ==> a.spec_index(i) == v[i],
        ensures elements(a) =~= v,
    {
        assert(elements(a).len() == v.len());
        assert forall|i: int| 0 <= i < v.len()
            implies elements(a)[i] == #[trigger] v[i] by
        {
            assert(elements(a)[i] == a.spec_index(i));
        };
    }

    //		Section 8. traits


    pub trait QuickSortMtEphSliceTrait<T: StTInMtT + TotalOrder> {
        /// Quicksort with first-element pivot. ParaPair! recursion.
        /// - Alg Analysis: APAS (Ch36 Alg 36.1): Work O(n^2), Span O(n lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2) worst, Span O(n lg n) worst — parallel D&C partition via join + parallel recursion via ParaPair
        fn quick_sort_first(a: &mut ArraySeqMtEphSliceS<T>)
            requires
                old(a).spec_arrayseqmtephslice_wf(),
                old(a).spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                a.spec_arrayseqmtephslice_wf(),
                elements(*a) =~= elements(*old(a)).sort_by(spec_leq::<T>()),
                a.spec_len() == old(a).spec_len(),
            decreases old(a).spec_len();

        /// Quicksort with median-of-three pivot. ParaPair! recursion.
        /// - Alg Analysis: APAS (Ch36 Alg 36.1): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n) — parallel D&C partition O(lg n) per level + parallel recursion via ParaPair
        fn quick_sort_median3(a: &mut ArraySeqMtEphSliceS<T>)
            requires
                old(a).spec_arrayseqmtephslice_wf(),
                old(a).spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                a.spec_arrayseqmtephslice_wf(),
                elements(*a) =~= elements(*old(a)).sort_by(spec_leq::<T>()),
                a.spec_len() == old(a).spec_len(),
            decreases old(a).spec_len();

        /// Quicksort with random pivot. ParaPair! recursion.
        /// - Alg Analysis: APAS (Ch36 Alg 36.1): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n) expected, Span O(lg^2 n) expected — parallel D&C partition O(lg n) per level + parallel recursion via ParaPair
        fn quick_sort_random(a: &mut ArraySeqMtEphSliceS<T>)
            requires
                old(a).spec_arrayseqmtephslice_wf(),
                old(a).spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                a.spec_arrayseqmtephslice_wf(),
                elements(*a) =~= elements(*old(a)).sort_by(spec_leq::<T>()),
                a.spec_len() == old(a).spec_len(),
            decreases old(a).spec_len();

        /// Compute the median of three values.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three comparisons.
        fn median_of_three(a: T, b: T, c: T) -> (median: T)
            ensures
                median == a || median == b || median == c,
                median == spec_median_of_three(a, b, c);

        /// Returns index of median among a[0], a[n/2], a[n-1].
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three element reads + median comparison.
        fn median3_pivot_idx(a: &ArraySeqMtEphSliceS<T>, n: usize) -> (idx: usize)
            requires
                n >= 2, n == a.spec_len(),
                a.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
            ensures
                idx < n,
                idx == 0 || idx == n / 2 || idx == n - 1,
                a.spec_index(idx as int) == spec_median_of_three(
                    a.spec_index(0), a.spec_index((n / 2) as int), a.spec_index((n - 1) as int));

        /// Concatenate three Vecs.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — copies three Vecs into one.
        fn concat_three_vecs(
            left: &Vec<T>,
            mid: &Vec<T>,
            right: &Vec<T>,
        ) -> (out: Vec<T>)
            requires
                left@.len() + mid@.len() + right@.len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures out@ =~= left@ + mid@ + right@;
    }

    //		Section 9. impls


    /// Append all elements of `b` onto the end of `a`.
    fn append_vec<T: Eq + Clone>(a: &mut Vec<T>, b: &Vec<T>)
        requires obeys_feq_clone::<T>()
        ensures a@ =~= old(a)@ + b@
    {
        let ghost a_orig = a@;
        let alen = a.len();
        let blen = b.len();
        let mut i: usize = 0;
        while i < blen
            invariant
                i <= blen,
                blen == b@.len(),
                alen == a_orig.len(),
                a@.len() == alen + i,
                obeys_feq_clone::<T>(),
                a@.subrange(0, alen as int) =~= a_orig,
                forall|k: int| 0 <= k < i as int ==>
                    #[trigger] a@[alen as int + k] == b@[k],
            decreases blen - i,
        {
            a.push(b[i].clone_plus());
            i = i + 1;
        }
        proof {
            let ghost target = a_orig + b@;
            assert(a@.len() == target.len());
            assert forall|k: int| 0 <= k < a@.len()
                implies a@[k] == #[trigger] target[k] by
            {
                if k < alen as int {
                    assert(a@[k] == a_orig[k]);
                } else {
                    let kp = k - alen as int;
                    assert(a@[alen as int + kp] == b@[kp]);
                }
            };
        }
    }

    /// Parallel D&C three-way partition.  Split into halves, partition each half
    /// in parallel via `join`, concatenate the three result Vecs.
    /// Work O(n), Span O(lg n) for the partition itself (plus O(n) rejoin).
    fn partition_three_dc<T: StTInMtT + TotalOrder>(
        a: &ArraySeqMtEphSliceS<T>,
        pivot: &T,
    ) -> (partitioned: (Vec<T>, Vec<T>, Vec<T>))
        requires
            a.spec_arrayseqmtephslice_wf(),
            obeys_feq_clone::<T>(),
        ensures
            forall|j: int| #![trigger partitioned.0@[j]] 0 <= j < partitioned.0@.len() ==>
                T::le(partitioned.0@[j], *pivot) && partitioned.0@[j] != *pivot,
            forall|j: int| #![trigger partitioned.1@[j]] 0 <= j < partitioned.1@.len() ==>
                partitioned.1@[j] == *pivot,
            forall|j: int| #![trigger partitioned.2@[j]] 0 <= j < partitioned.2@.len() ==>
                T::le(*pivot, partitioned.2@[j]) && partitioned.2@[j] != *pivot,
            elements(*a).to_multiset() =~=
                partitioned.0@.to_multiset().add(partitioned.2@.to_multiset()).add(partitioned.1@.to_multiset()),
            partitioned.0@.len() + partitioned.1@.len() + partitioned.2@.len() == a.spec_len(),
        decreases a.spec_len(),
    {
        let n = a.length();
        if n == 0 {
            (Vec::new(), Vec::new(), Vec::new())
        } else if n == 1 {
            let elem = a.nth_cloned(0);
            proof {
                assert(elements(*a).len() == 1);
                assert(elements(*a)[0] == elem);
            }
            match TotalOrder::cmp(&elem, pivot) {
                core::cmp::Ordering::Less => {
                    proof { assert(T::le(elem, *pivot)); assert(elem != *pivot); }
                    let mut v: Vec<T> = Vec::new();
                    v.push(elem);
                    proof {
                        assert(v@ =~= elements(*a));
                    }
                    (v, Vec::new(), Vec::new())
                }
                core::cmp::Ordering::Equal => {
                    let mut v: Vec<T> = Vec::new();
                    v.push(elem);
                    proof {
                        assert(v@ =~= elements(*a));
                    }
                    (Vec::new(), v, Vec::new())
                }
                core::cmp::Ordering::Greater => {
                    proof { assert(T::le(*pivot, elem)); assert(elem != *pivot); }
                    let mut v: Vec<T> = Vec::new();
                    v.push(elem);
                    proof {
                        assert(v@ =~= elements(*a));
                    }
                    (Vec::new(), Vec::new(), v)
                }
            }
        } else {
            let mid = n / 2;
            let left_half = a.slice(0, mid);
            let right_half = a.slice(mid, n - mid);
            let p1 = pivot.clone_plus();
            let p2 = pivot.clone_plus();

            let ghost pivot_val = *pivot;
            let ghost left_elems = elements(left_half);
            let ghost right_elems = elements(right_half);

            let fa = move || -> (r: (Vec<T>, Vec<T>, Vec<T>))
                requires
                    left_half.spec_arrayseqmtephslice_wf(),
                    obeys_feq_clone::<T>(),
                ensures
                    forall|j: int| #![trigger r.0@[j]] 0 <= j < r.0@.len() ==>
                        T::le(r.0@[j], pivot_val) && r.0@[j] != pivot_val,
                    forall|j: int| #![trigger r.1@[j]] 0 <= j < r.1@.len() ==>
                        r.1@[j] == pivot_val,
                    forall|j: int| #![trigger r.2@[j]] 0 <= j < r.2@.len() ==>
                        T::le(pivot_val, r.2@[j]) && r.2@[j] != pivot_val,
                    left_elems.to_multiset() =~=
                        r.0@.to_multiset().add(r.2@.to_multiset()).add(r.1@.to_multiset()),
                    r.0@.len() + r.1@.len() + r.2@.len() == left_elems.len(),
            {
                partition_three_dc(&left_half, &p1)
            };

            let fb = move || -> (r: (Vec<T>, Vec<T>, Vec<T>))
                requires
                    right_half.spec_arrayseqmtephslice_wf(),
                    obeys_feq_clone::<T>(),
                ensures
                    forall|j: int| #![trigger r.0@[j]] 0 <= j < r.0@.len() ==>
                        T::le(r.0@[j], pivot_val) && r.0@[j] != pivot_val,
                    forall|j: int| #![trigger r.1@[j]] 0 <= j < r.1@.len() ==>
                        r.1@[j] == pivot_val,
                    forall|j: int| #![trigger r.2@[j]] 0 <= j < r.2@.len() ==>
                        T::le(pivot_val, r.2@[j]) && r.2@[j] != pivot_val,
                    right_elems.to_multiset() =~=
                        r.0@.to_multiset().add(r.2@.to_multiset()).add(r.1@.to_multiset()),
                    r.0@.len() + r.1@.len() + r.2@.len() == right_elems.len(),
            {
                partition_three_dc(&right_half, &p2)
            };

            let ((mut l1, mut e1, mut r1), (l2, e2, r2)) = join(fa, fb);

            let ghost l1_pre = l1@;
            let ghost e1_pre = e1@;
            let ghost r1_pre = r1@;

            append_vec(&mut l1, &l2);
            append_vec(&mut e1, &e2);
            append_vec(&mut r1, &r2);

            proof {
                // elements(*a) =~= left_elems + right_elems
                let ghost ea = elements(*a);
                assert(ea.len() == left_elems.len() + right_elems.len());
                assert forall|k: int| 0 <= k < ea.len()
                    implies ea[k] == #[trigger] (left_elems + right_elems)[k] by
                {
                    if k < left_elems.len() {
                        assert(left_half.spec_index(k) == a.spec_index(k));
                    } else {
                        let kp = k - left_elems.len();
                        assert(right_half.spec_index(kp) == a.spec_index(mid as int + kp));
                    }
                };
                assert(ea =~= left_elems + right_elems);

                vstd::seq_lib::lemma_multiset_commutative(left_elems, right_elems);
                vstd::seq_lib::lemma_multiset_commutative(l1_pre, l2@);
                vstd::seq_lib::lemma_multiset_commutative(e1_pre, e2@);
                vstd::seq_lib::lemma_multiset_commutative(r1_pre, r2@);

                // Multiset rearrangement: Z3 handles integer arithmetic.
                assert(ea.to_multiset() =~=
                    l1@.to_multiset().add(r1@.to_multiset()).add(e1@.to_multiset()));
            }

            (l1, e1, r1)
        }
    }


    impl<T: StTInMtT + TotalOrder> QuickSortMtEphSliceTrait<T>
        for ArraySeqMtEphSliceS<T>
    {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three comparisons.
        fn median_of_three(a: T, b: T, c: T) -> (median: T) {
            match TotalOrder::cmp(&a, &b) {
                core::cmp::Ordering::Less | core::cmp::Ordering::Equal => {
                    proof { T::reflexive(a); }
                    match TotalOrder::cmp(&b, &c) {
                        core::cmp::Ordering::Less | core::cmp::Ordering::Equal => {
                            proof { T::reflexive(b); }
                            b
                        }
                        core::cmp::Ordering::Greater => {
                            proof { if T::le(b, c) { T::antisymmetric(b, c); } }
                            match TotalOrder::cmp(&a, &c) {
                                core::cmp::Ordering::Less | core::cmp::Ordering::Equal => {
                                    proof { T::reflexive(a); }
                                    c
                                }
                                core::cmp::Ordering::Greater => {
                                    proof { if T::le(a, c) { T::antisymmetric(a, c); } }
                                    a
                                }
                            }
                        }
                    }
                }
                core::cmp::Ordering::Greater => {
                    proof { if T::le(a, b) { T::antisymmetric(a, b); } }
                    match TotalOrder::cmp(&a, &c) {
                        core::cmp::Ordering::Less | core::cmp::Ordering::Equal => {
                            proof { T::reflexive(a); }
                            a
                        }
                        core::cmp::Ordering::Greater => {
                            proof { if T::le(a, c) { T::antisymmetric(a, c); } }
                            match TotalOrder::cmp(&b, &c) {
                                core::cmp::Ordering::Less | core::cmp::Ordering::Equal => {
                                    proof { T::reflexive(b); }
                                    c
                                }
                                core::cmp::Ordering::Greater => {
                                    proof { if T::le(b, c) { T::antisymmetric(b, c); } }
                                    b
                                }
                            }
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three element reads + median comparison.
        fn median3_pivot_idx(a: &ArraySeqMtEphSliceS<T>, n: usize) -> (idx: usize) {
            let first = a.nth_cloned(0);
            let mid = a.nth_cloned(n / 2);
            let last = a.nth_cloned(n - 1);
            let median = Self::median_of_three(first, mid, last);
            let c0 = a.nth_cloned(0);
            match TotalOrder::cmp(&c0, &median) {
                core::cmp::Ordering::Equal => 0,
                _ => {
                    let cm = a.nth_cloned(n / 2);
                    match TotalOrder::cmp(&cm, &median) {
                        core::cmp::Ordering::Equal => n / 2,
                        _ => n - 1,
                    }
                },
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — copies three Vecs into one.
        fn concat_three_vecs(
            left: &Vec<T>,
            mid: &Vec<T>,
            right: &Vec<T>,
        ) -> (out: Vec<T>) {
            let sl = left.len();
            let el = mid.len();
            let sr = right.len();
            let mut out: Vec<T> = Vec::new();

            let mut j: usize = 0;
            while j < sl
                invariant
                    0 <= j <= sl, sl == left@.len(), el == mid@.len(),
                    sr == right@.len(), sl + el + sr <= usize::MAX,
                    obeys_feq_clone::<T>(),
                    out@.len() == j as nat,
                    forall|k: int| 0 <= k < j as int ==>
                        #[trigger] out@[k] == left@[k],
                decreases sl - j,
            {
                out.push(left[j].clone_plus());
                j = j + 1;
            }

            j = 0;
            while j < el
                invariant
                    0 <= j <= el, sl == left@.len(), el == mid@.len(),
                    sr == right@.len(), sl + el + sr <= usize::MAX,
                    obeys_feq_clone::<T>(),
                    out@.len() == (sl + j) as nat,
                    forall|k: int| 0 <= k < sl as int ==>
                        #[trigger] out@[k] == left@[k],
                    forall|k: int| 0 <= k < j as int ==>
                        #[trigger] out@[(sl + k) as int] == mid@[k],
                decreases el - j,
            {
                out.push(mid[j].clone_plus());
                j = j + 1;
            }

            j = 0;
            while j < sr
                invariant
                    0 <= j <= sr, sl == left@.len(), el == mid@.len(),
                    sr == right@.len(), sl + el + sr <= usize::MAX,
                    obeys_feq_clone::<T>(),
                    out@.len() == (sl + el + j) as nat,
                    forall|k: int| 0 <= k < sl as int ==>
                        #[trigger] out@[k] == left@[k],
                    forall|k: int| 0 <= k < el as int ==>
                        #[trigger] out@[(sl + k) as int] == mid@[k],
                    forall|k: int| 0 <= k < j as int ==>
                        #[trigger] out@[(sl + el + k) as int] == right@[k],
                decreases sr - j,
            {
                out.push(right[j].clone_plus());
                j = j + 1;
            }

            proof {
                let ghost l = left@;
                let ghost m = mid@;
                let ghost r = right@;
                let ghost target = l + m + r;
                assert(out@.len() == target.len());
                assert forall|k: int| 0 <= k < out@.len()
                    implies out@[k] == #[trigger] target[k] by
                {
                    if k < sl as int {
                        assert(out@[k] == left@[k]);
                        assert(target[k] == l[k]);
                    } else if k < (sl + el) as int {
                        let kp = k - sl as int;
                        assert(out@[(sl as int + kp)] == mid@[kp]);
                        assert(target[k] == m[kp]);
                    } else {
                        let kp = k - sl as int - el as int;
                        assert(out@[(sl as int + el as int + kp)] == right@[kp]);
                        assert(target[k] == r[kp]);
                    }
                };
            }

            out
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2) worst, Span O(n lg n) worst — parallel D&C partition + parallel recursion via ParaPair.
        fn quick_sort_first(a: &mut ArraySeqMtEphSliceS<T>)
            decreases old(a).spec_len(),
        {
            let n = a.length();
            if n <= 1 {
                let ghost s = elements(*a);
                let ghost leq = spec_leq::<T>();
                proof {
                    lemma_total_ordering::<T>();
                    s.lemma_sort_by_ensures(leq);
                    if s.len() == 0 {
                        assert(s.to_multiset().len() == s.len());
                        assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());
                        assert(s.sort_by(leq).to_multiset() =~= s.to_multiset());
                        assert(s.sort_by(leq).len() == s.len());
                    } else {
                        assert(sorted_by(s, leq));
                        vstd::seq_lib::lemma_sorted_unique(s, s.sort_by(leq), leq);
                    }
                }
                return;
            }
            let ghost s = elements(*a);
            let ghost leq = spec_leq::<T>();
            let pivot_idx: usize = 0;
            let pivot = a.nth_cloned(pivot_idx);

            let (left, equals, right) = partition_three_dc(a, &pivot);

            proof {
                // The pivot itself is in elements(*a), so it lands in equals.
                assert(s[pivot_idx as int] == pivot);
                assert(s.to_multiset().count(pivot) > 0) by {
                    s.to_multiset_ensures();
                    assert(s.contains(pivot));
                };
                // left and right contain no copies of pivot.
                assert(left@.to_multiset().count(pivot) == 0nat) by {
                    if left@.to_multiset().count(pivot) > 0 {
                        left@.to_multiset_ensures();
                        assert(left@.contains(pivot));
                        let j = choose|j: int| 0 <= j < left@.len() && left@[j] == pivot;
                        assert(left@[j] != pivot);
                    }
                };
                assert(right@.to_multiset().count(pivot) == 0nat) by {
                    if right@.to_multiset().count(pivot) > 0 {
                        right@.to_multiset_ensures();
                        assert(right@.contains(pivot));
                        let j = choose|j: int| 0 <= j < right@.len() && right@[j] == pivot;
                        assert(right@[j] != pivot);
                    }
                };
                assert(equals@.to_multiset().count(pivot) > 0nat);
                assert(equals@.len() >= 1) by {
                    if equals@.len() == 0 {
                        assert(equals@.to_multiset() =~= Multiset::empty());
                    }
                };
                assert(left@.len() + right@.len() < n);
            }

            let ghost left_view = left@;
            let ghost right_view = right@;
            let left_a = ArraySeqMtEphSliceS::from_vec(left);
            let right_a = ArraySeqMtEphSliceS::from_vec(right);
            proof {
                lemma_elements_from_vec::<T>(left_view, left_a);
                lemma_elements_from_vec::<T>(right_view, right_a);
            }

            let f1 = move || -> (r: Vec<T>)
                ensures r@ =~= left_view.sort_by(spec_leq::<T>())
            {
                let mut la = left_a;
                proof {
                    lemma_total_ordering::<T>();
                    lemma_elements_from_vec::<T>(left_view, la);
                }
                let ghost pre_elems = elements(la);
                Self::quick_sort_first(&mut la);
                let v = la.to_vec();
                proof {
                    assert(elements(la) =~= pre_elems.sort_by(spec_leq::<T>()));
                    assert forall|i: int| 0 <= i < la.spec_len()
                        implies v@[i] == #[trigger] elements(la)[i] by
                    { assert(v@[i] == la.spec_index(i)); };
                    assert(v@ =~= elements(la));
                }
                v
            };
            let f2 = move || -> (r: Vec<T>)
                ensures r@ =~= right_view.sort_by(spec_leq::<T>())
            {
                let mut ra = right_a;
                proof {
                    lemma_total_ordering::<T>();
                    lemma_elements_from_vec::<T>(right_view, ra);
                }
                let ghost pre_elems = elements(ra);
                Self::quick_sort_first(&mut ra);
                let v = ra.to_vec();
                proof {
                    assert(elements(ra) =~= pre_elems.sort_by(spec_leq::<T>()));
                    assert forall|i: int| 0 <= i < ra.spec_len()
                        implies v@[i] == #[trigger] elements(ra)[i] by
                    { assert(v@[i] == ra.spec_index(i)); };
                    assert(v@ =~= elements(ra));
                }
                v
            };
            let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

            proof {
                lemma_total_ordering::<T>();
                left_view.lemma_sort_by_ensures(leq);
                right_view.lemma_sort_by_ensures(leq);
                assert(sorted_left@ =~= left_view.sort_by(spec_leq::<T>()));
                assert(sorted_right@ =~= right_view.sort_by(spec_leq::<T>()));
                assert(sorted_left@.len() == left_view.sort_by(leq).len());
                assert(left_view.sort_by(leq).len() == left_view.len()) by {
                    assert(left_view.sort_by(leq).to_multiset() =~= left_view.to_multiset());
                    assert(left_view.sort_by(leq).to_multiset().len() == left_view.sort_by(leq).len());
                    assert(left_view.to_multiset().len() == left_view.len());
                };
                assert(right_view.sort_by(leq).len() == right_view.len()) by {
                    assert(right_view.sort_by(leq).to_multiset() =~= right_view.to_multiset());
                    assert(right_view.sort_by(leq).to_multiset().len() == right_view.sort_by(leq).len());
                    assert(right_view.to_multiset().len() == right_view.len());
                };
            }

            let sorted = Self::concat_three_vecs(&sorted_left, &equals, &sorted_right);
            proof {
                lemma_partition_sort_concat::<T>(
                    s, left_view, right_view, equals@,
                    sorted_left@, sorted_right@, pivot,
                );
            }
            *a = ArraySeqMtEphSliceS::from_vec(sorted);
            proof {
                lemma_elements_from_vec::<T>(s.sort_by(leq), *a);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(lg^2 n) — parallel D&C partition + parallel recursion via ParaPair.
        fn quick_sort_median3(a: &mut ArraySeqMtEphSliceS<T>)
            decreases old(a).spec_len(),
        {
            let n = a.length();
            if n <= 1 {
                let ghost s = elements(*a);
                let ghost leq = spec_leq::<T>();
                proof {
                    lemma_total_ordering::<T>();
                    s.lemma_sort_by_ensures(leq);
                    if s.len() == 0 {
                        assert(s.to_multiset().len() == s.len());
                        assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());
                        assert(s.sort_by(leq).to_multiset() =~= s.to_multiset());
                        assert(s.sort_by(leq).len() == s.len());
                    } else {
                        assert(sorted_by(s, leq));
                        vstd::seq_lib::lemma_sorted_unique(s, s.sort_by(leq), leq);
                    }
                }
                return;
            }
            let ghost s = elements(*a);
            let ghost leq = spec_leq::<T>();
            let pivot_idx = Self::median3_pivot_idx(&*a, n);
            let pivot = a.nth_cloned(pivot_idx);

            let (left, equals, right) = partition_three_dc(a, &pivot);

            proof {
                assert(s[pivot_idx as int] == pivot);
                assert(s.to_multiset().count(pivot) > 0) by {
                    s.to_multiset_ensures();
                    assert(s.contains(pivot));
                };
                assert(left@.to_multiset().count(pivot) == 0nat) by {
                    if left@.to_multiset().count(pivot) > 0 {
                        left@.to_multiset_ensures();
                        assert(left@.contains(pivot));
                        let j = choose|j: int| 0 <= j < left@.len() && left@[j] == pivot;
                        assert(left@[j] != pivot);
                    }
                };
                assert(right@.to_multiset().count(pivot) == 0nat) by {
                    if right@.to_multiset().count(pivot) > 0 {
                        right@.to_multiset_ensures();
                        assert(right@.contains(pivot));
                        let j = choose|j: int| 0 <= j < right@.len() && right@[j] == pivot;
                        assert(right@[j] != pivot);
                    }
                };
                assert(equals@.to_multiset().count(pivot) > 0nat);
                assert(equals@.len() >= 1) by {
                    if equals@.len() == 0 {
                        assert(equals@.to_multiset() =~= Multiset::empty());
                    }
                };
                assert(left@.len() + right@.len() < n);
            }

            let ghost left_view = left@;
            let ghost right_view = right@;
            let left_a = ArraySeqMtEphSliceS::from_vec(left);
            let right_a = ArraySeqMtEphSliceS::from_vec(right);
            proof {
                lemma_elements_from_vec::<T>(left_view, left_a);
                lemma_elements_from_vec::<T>(right_view, right_a);
            }

            let f1 = move || -> (r: Vec<T>)
                ensures r@ =~= left_view.sort_by(spec_leq::<T>())
            {
                let mut la = left_a;
                proof {
                    lemma_total_ordering::<T>();
                    lemma_elements_from_vec::<T>(left_view, la);
                }
                let ghost pre_elems = elements(la);
                Self::quick_sort_median3(&mut la);
                let v = la.to_vec();
                proof {
                    assert(elements(la) =~= pre_elems.sort_by(spec_leq::<T>()));
                    assert forall|i: int| 0 <= i < la.spec_len()
                        implies v@[i] == #[trigger] elements(la)[i] by
                    { assert(v@[i] == la.spec_index(i)); };
                    assert(v@ =~= elements(la));
                }
                v
            };
            let f2 = move || -> (r: Vec<T>)
                ensures r@ =~= right_view.sort_by(spec_leq::<T>())
            {
                let mut ra = right_a;
                proof {
                    lemma_total_ordering::<T>();
                    lemma_elements_from_vec::<T>(right_view, ra);
                }
                let ghost pre_elems = elements(ra);
                Self::quick_sort_median3(&mut ra);
                let v = ra.to_vec();
                proof {
                    assert(elements(ra) =~= pre_elems.sort_by(spec_leq::<T>()));
                    assert forall|i: int| 0 <= i < ra.spec_len()
                        implies v@[i] == #[trigger] elements(ra)[i] by
                    { assert(v@[i] == ra.spec_index(i)); };
                    assert(v@ =~= elements(ra));
                }
                v
            };
            let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

            proof {
                lemma_total_ordering::<T>();
                left_view.lemma_sort_by_ensures(leq);
                right_view.lemma_sort_by_ensures(leq);
                assert(sorted_left@ =~= left_view.sort_by(spec_leq::<T>()));
                assert(sorted_right@ =~= right_view.sort_by(spec_leq::<T>()));
                assert(sorted_left@.len() == left_view.sort_by(leq).len());
                assert(left_view.sort_by(leq).len() == left_view.len()) by {
                    assert(left_view.sort_by(leq).to_multiset() =~= left_view.to_multiset());
                    assert(left_view.sort_by(leq).to_multiset().len() == left_view.sort_by(leq).len());
                    assert(left_view.to_multiset().len() == left_view.len());
                };
                assert(right_view.sort_by(leq).len() == right_view.len()) by {
                    assert(right_view.sort_by(leq).to_multiset() =~= right_view.to_multiset());
                    assert(right_view.sort_by(leq).to_multiset().len() == right_view.sort_by(leq).len());
                    assert(right_view.to_multiset().len() == right_view.len());
                };
            }

            let sorted = Self::concat_three_vecs(&sorted_left, &equals, &sorted_right);
            proof {
                lemma_partition_sort_concat::<T>(
                    s, left_view, right_view, equals@,
                    sorted_left@, sorted_right@, pivot,
                );
            }
            *a = ArraySeqMtEphSliceS::from_vec(sorted);
            proof {
                lemma_elements_from_vec::<T>(s.sort_by(leq), *a);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n) expected, Span O(lg^2 n) expected — parallel D&C partition + parallel recursion via ParaPair.
        fn quick_sort_random(a: &mut ArraySeqMtEphSliceS<T>)
            decreases old(a).spec_len(),
        {
            let n = a.length();
            if n <= 1 {
                let ghost s = elements(*a);
                let ghost leq = spec_leq::<T>();
                proof {
                    lemma_total_ordering::<T>();
                    s.lemma_sort_by_ensures(leq);
                    if s.len() == 0 {
                        assert(s.to_multiset().len() == s.len());
                        assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());
                        assert(s.sort_by(leq).to_multiset() =~= s.to_multiset());
                        assert(s.sort_by(leq).len() == s.len());
                    } else {
                        assert(sorted_by(s, leq));
                        vstd::seq_lib::lemma_sorted_unique(s, s.sort_by(leq), leq);
                    }
                }
                return;
            }
            let ghost s = elements(*a);
            let ghost leq = spec_leq::<T>();
            let pivot_idx = random_usize_range(0, n);
            let pivot = a.nth_cloned(pivot_idx);

            let (left, equals, right) = partition_three_dc(a, &pivot);

            proof {
                assert(s[pivot_idx as int] == pivot);
                assert(s.to_multiset().count(pivot) > 0) by {
                    s.to_multiset_ensures();
                    assert(s.contains(pivot));
                };
                assert(left@.to_multiset().count(pivot) == 0nat) by {
                    if left@.to_multiset().count(pivot) > 0 {
                        left@.to_multiset_ensures();
                        assert(left@.contains(pivot));
                        let j = choose|j: int| 0 <= j < left@.len() && left@[j] == pivot;
                        assert(left@[j] != pivot);
                    }
                };
                assert(right@.to_multiset().count(pivot) == 0nat) by {
                    if right@.to_multiset().count(pivot) > 0 {
                        right@.to_multiset_ensures();
                        assert(right@.contains(pivot));
                        let j = choose|j: int| 0 <= j < right@.len() && right@[j] == pivot;
                        assert(right@[j] != pivot);
                    }
                };
                assert(equals@.to_multiset().count(pivot) > 0nat);
                assert(equals@.len() >= 1) by {
                    if equals@.len() == 0 {
                        assert(equals@.to_multiset() =~= Multiset::empty());
                    }
                };
                assert(left@.len() + right@.len() < n);
            }

            let ghost left_view = left@;
            let ghost right_view = right@;
            let left_a = ArraySeqMtEphSliceS::from_vec(left);
            let right_a = ArraySeqMtEphSliceS::from_vec(right);
            proof {
                lemma_elements_from_vec::<T>(left_view, left_a);
                lemma_elements_from_vec::<T>(right_view, right_a);
            }

            let f1 = move || -> (r: Vec<T>)
                ensures r@ =~= left_view.sort_by(spec_leq::<T>())
            {
                let mut la = left_a;
                proof {
                    lemma_total_ordering::<T>();
                    lemma_elements_from_vec::<T>(left_view, la);
                }
                let ghost pre_elems = elements(la);
                Self::quick_sort_random(&mut la);
                let v = la.to_vec();
                proof {
                    assert(elements(la) =~= pre_elems.sort_by(spec_leq::<T>()));
                    assert forall|i: int| 0 <= i < la.spec_len()
                        implies v@[i] == #[trigger] elements(la)[i] by
                    { assert(v@[i] == la.spec_index(i)); };
                    assert(v@ =~= elements(la));
                }
                v
            };
            let f2 = move || -> (r: Vec<T>)
                ensures r@ =~= right_view.sort_by(spec_leq::<T>())
            {
                let mut ra = right_a;
                proof {
                    lemma_total_ordering::<T>();
                    lemma_elements_from_vec::<T>(right_view, ra);
                }
                let ghost pre_elems = elements(ra);
                Self::quick_sort_random(&mut ra);
                let v = ra.to_vec();
                proof {
                    assert(elements(ra) =~= pre_elems.sort_by(spec_leq::<T>()));
                    assert forall|i: int| 0 <= i < ra.spec_len()
                        implies v@[i] == #[trigger] elements(ra)[i] by
                    { assert(v@[i] == ra.spec_index(i)); };
                    assert(v@ =~= elements(ra));
                }
                v
            };
            let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

            proof {
                lemma_total_ordering::<T>();
                left_view.lemma_sort_by_ensures(leq);
                right_view.lemma_sort_by_ensures(leq);
                assert(sorted_left@ =~= left_view.sort_by(spec_leq::<T>()));
                assert(sorted_right@ =~= right_view.sort_by(spec_leq::<T>()));
                assert(sorted_left@.len() == left_view.sort_by(leq).len());
                assert(left_view.sort_by(leq).len() == left_view.len()) by {
                    assert(left_view.sort_by(leq).to_multiset() =~= left_view.to_multiset());
                    assert(left_view.sort_by(leq).to_multiset().len() == left_view.sort_by(leq).len());
                    assert(left_view.to_multiset().len() == left_view.len());
                };
                assert(right_view.sort_by(leq).len() == right_view.len()) by {
                    assert(right_view.sort_by(leq).to_multiset() =~= right_view.to_multiset());
                    assert(right_view.sort_by(leq).to_multiset().len() == right_view.sort_by(leq).len());
                    assert(right_view.to_multiset().len() == right_view.len());
                };
            }

            let sorted = Self::concat_three_vecs(&sorted_left, &equals, &sorted_right);
            proof {
                lemma_partition_sort_concat::<T>(
                    s, left_view, right_view, equals@,
                    sorted_left@, sorted_right@, pivot,
                );
            }
            *a = ArraySeqMtEphSliceS::from_vec(sorted);
            proof {
                lemma_elements_from_vec::<T>(s.sort_by(leq), *a);
            }
        }
    }

    } // verus!
} // mod
