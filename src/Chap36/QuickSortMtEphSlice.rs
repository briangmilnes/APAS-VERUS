//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Multi-threaded Slice): Quicksort over `ArraySeqMtEphSlice`.
//! Three self-recursive functions: qsort_first, qsort_median3, qsort_random.
//! Each applies its pivot strategy at every recursive level.
//! Parallel recursion via ParaPair! for left/right subarrays after partition.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls

// 1. module

pub mod QuickSortMtEphSlice {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Types::Types::Pair;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::rand::rand::random_usize_range;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use vstd::relations::*;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // 6. spec fns

    /// Spec-level leq closure for sort_by and sorted_by.
    pub open spec fn spec_leq<T: TotalOrder>() -> spec_fn(T, T) -> bool {
        |x: T, y: T| T::le(x, y)
    }

    /// Extract the element sequence from a slice as Seq<T>.
    pub open spec fn elements<T: Eq + Clone>(a: ArraySeqMtEphSliceS<T>) -> Seq<T> {
        Seq::new(a.spec_len(), |i: int| a.spec_index(i))
    }

    /// Median of three values: returns the value that is neither min nor max.
    spec fn spec_median_of_three<T: TotalOrder>(a: T, b: T, c: T) -> T {
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

    // 7. proof fns

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
    proof fn lemma_elements_from_vec<T: TotalOrder + Eq + Clone>(v: Seq<T>, a: ArraySeqMtEphSliceS<T>)
        requires
            a.slice_wf(),
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

    // 8. traits

    pub trait QuickSortMtEphSliceTrait<T: TotalOrder + Eq + Clone> {
        /// Quicksort with first-element pivot. ParaPair! recursion.
        /// - APAS: Work O(n^2) worst, Span O(n) worst — sequential partition.
        /// - Claude-Opus-4.6: Agrees.
        fn quick_sort_first(a: &mut ArraySeqMtEphSliceS<T>)
            requires
                old(a).slice_wf(),
                old(a).spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                a.slice_wf(),
                elements(*a) =~= elements(*old(a)).sort_by(spec_leq::<T>());

        /// Quicksort with median-of-three pivot. ParaPair! recursion.
        /// - APAS: Work O(n^2) worst / O(n lg n) sorted, Span O(n) — sequential partition.
        /// - Claude-Opus-4.6: Agrees.
        fn quick_sort_median3(a: &mut ArraySeqMtEphSliceS<T>)
            requires
                old(a).slice_wf(),
                old(a).spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                a.slice_wf(),
                elements(*a) =~= elements(*old(a)).sort_by(spec_leq::<T>());

        /// Quicksort with random pivot. ParaPair! recursion.
        /// - APAS: Work O(n lg n) expected, Span O(n) — sequential partition.
        /// - Claude-Opus-4.6: Agrees.
        fn quick_sort_random(a: &mut ArraySeqMtEphSliceS<T>)
            requires
                old(a).slice_wf(),
                old(a).spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                a.slice_wf(),
                elements(*a) =~= elements(*old(a)).sort_by(spec_leq::<T>());
    }

    // 9. impls

    /// Base case: arrays of length 0 or 1 are already sorted.
    fn qsort_base_case<T: TotalOrder + Eq + Clone>(
        a: &ArraySeqMtEphSliceS<T>,
    ) -> (sorted: Vec<T>)
        requires
            a.slice_wf(),
            a.spec_len() <= 1,
            a.spec_len() <= usize::MAX,
            obeys_feq_clone::<T>(),
        ensures sorted@ =~= elements(*a).sort_by(spec_leq::<T>())
    {
        let n = a.length();
        let ghost s = elements(*a);
        let ghost leq = spec_leq::<T>();
        if n == 0 {
            proof {
                lemma_total_ordering::<T>();
                s.lemma_sort_by_ensures(leq);
                assert(s.to_multiset().len() == s.len());
                assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());
                assert(s.sort_by(leq).to_multiset() =~= s.to_multiset());
                assert(s.sort_by(leq).len() == s.len());
            }
            Vec::new()
        } else {
            let elem = a.nth_cloned(0);
            let mut r: Vec<T> = Vec::new();
            r.push(elem);
            proof {
                lemma_total_ordering::<T>();
                s.lemma_sort_by_ensures(leq);
                assert(sorted_by(s, leq));
                vstd::seq_lib::lemma_sorted_unique(s, s.sort_by(leq), leq);
                assert(s.sort_by(leq) =~= s);
                assert(r@ =~= s);
            }
            r
        }
    }

    fn median_of_three<T: TotalOrder>(a: T, b: T, c: T) -> (median: T)
        ensures
            median == a || median == b || median == c,
            median == spec_median_of_three(a, b, c),
    {
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

    /// Returns index of median among a[0], a[n/2], a[n-1].
    fn median3_pivot_idx<T: TotalOrder + Eq + Clone>(
        a: &ArraySeqMtEphSliceS<T>,
        n: usize,
    ) -> (idx: usize)
        requires
            n >= 2,
            n == a.spec_len(),
            a.slice_wf(),
            obeys_feq_clone::<T>(),
        ensures
            idx < n,
            idx == 0 || idx == n / 2 || idx == n - 1,
            a.spec_index(idx as int) == spec_median_of_three(
                a.spec_index(0), a.spec_index((n / 2) as int), a.spec_index((n - 1) as int)),
    {
        let first = a.nth_cloned(0);
        let mid = a.nth_cloned(n / 2);
        let last = a.nth_cloned(n - 1);
        let median = median_of_three(first, mid, last);
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

    /// Concatenate three Vecs.
    fn concat_three_vecs<T: TotalOrder + Eq + Clone>(
        left: &Vec<T>,
        mid: &Vec<T>,
        right: &Vec<T>,
    ) -> (out: Vec<T>)
        requires left@.len() + mid@.len() + right@.len() <= usize::MAX,
        ensures out@ =~= left@ + mid@ + right@,
    {
        let sl = left.len();
        let el = mid.len();
        let sr = right.len();
        let mut out: Vec<T> = Vec::new();

        let mut j: usize = 0;
        while j < sl
            invariant
                0 <= j <= sl, sl == left@.len(), el == mid@.len(),
                sr == right@.len(), sl + el + sr <= usize::MAX,
                out@.len() == j as nat,
                forall|k: int| 0 <= k < j as int ==>
                    #[trigger] out@[k] == left@[k],
            decreases sl - j,
        {
            out.push(left[j].clone());
            j = j + 1;
        }

        j = 0;
        while j < el
            invariant
                0 <= j <= el, sl == left@.len(), el == mid@.len(),
                sr == right@.len(), sl + el + sr <= usize::MAX,
                out@.len() == (sl + j) as nat,
                forall|k: int| 0 <= k < sl as int ==>
                    #[trigger] out@[k] == left@[k],
                forall|k: int| 0 <= k < j as int ==>
                    #[trigger] out@[(sl + k) as int] == mid@[k],
            decreases el - j,
        {
            out.push(mid[j].clone());
            j = j + 1;
        }

        j = 0;
        while j < sr
            invariant
                0 <= j <= sr, sl == left@.len(), el == mid@.len(),
                sr == right@.len(), sl + el + sr <= usize::MAX,
                out@.len() == (sl + el + j) as nat,
                forall|k: int| 0 <= k < sl as int ==>
                    #[trigger] out@[k] == left@[k],
                forall|k: int| 0 <= k < el as int ==>
                    #[trigger] out@[(sl + k) as int] == mid@[k],
                forall|k: int| 0 <= k < j as int ==>
                    #[trigger] out@[(sl + el + k) as int] == right@[k],
            decreases sr - j,
        {
            out.push(right[j].clone());
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

    /// Quicksort with first-element pivot. Self-recursive via ParaPair!.
    fn qsort_first<T: TotalOrder + Eq + Clone + Send + 'static>(
        a: &ArraySeqMtEphSliceS<T>,
    ) -> (sorted: Vec<T>)
        requires
            a.slice_wf(),
            a.spec_len() <= usize::MAX,
            obeys_feq_clone::<T>(),
        ensures sorted@ =~= elements(*a).sort_by(spec_leq::<T>())
        decreases a.spec_len(),
    {
        let n = a.length();
        if n <= 1 { return qsort_base_case(a); }
        let ghost s = elements(*a);
        let ghost leq = spec_leq::<T>();
        let pivot_idx: usize = 0;
        let pivot = a.nth_cloned(pivot_idx);

        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();
        let mut equals: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                0 <= i <= n, n == a.spec_len() as usize, n <= usize::MAX, n >= 2,
                a.slice_wf(),
                obeys_feq_clone::<T>(),
                pivot_idx < n, pivot == s[pivot_idx as int], s == elements(*a),
                leq == spec_leq::<T>(),
                forall|j: int| 0 <= j < left@.len() ==>
                    (#[trigger] T::le(left@[j], pivot)) && left@[j] != pivot,
                forall|j: int| 0 <= j < right@.len() ==>
                    (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                forall|j: int| 0 <= j < equals@.len() ==>
                    (#[trigger] equals@[j]) == pivot,
                left@.len() + right@.len() + equals@.len() == i,
                i > pivot_idx ==> left@.len() + right@.len() < i,
                s.subrange(0, i as int).to_multiset() =~=
                    left@.to_multiset().add(right@.to_multiset()).add(equals@.to_multiset()),
            decreases n - i,
        {
            let elem = a.nth_cloned(i);
            proof {
                assert(s.subrange(0, (i + 1) as int) =~=
                    s.subrange(0, i as int).push(s[i as int]));
            }
            match TotalOrder::cmp(&elem, &pivot) {
                core::cmp::Ordering::Less => {
                    proof { assert(T::le(elem, pivot)); assert(elem != pivot); }
                    left.push(elem);
                },
                core::cmp::Ordering::Greater => {
                    proof { assert(T::le(pivot, elem)); assert(elem != pivot); }
                    right.push(elem);
                },
                core::cmp::Ordering::Equal => {
                    equals.push(elem);
                },
            }
            i = i + 1;
        }

        proof {
            assert(s.subrange(0, n as int) =~= s);
            assert(left@.len() + right@.len() < n);
            assert(equals@.len() >= 1);
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
        { qsort_first(&left_a) };
        let f2 = move || -> (r: Vec<T>)
            ensures r@ =~= right_view.sort_by(spec_leq::<T>())
        { qsort_first(&right_a) };
        let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

        proof {
            lemma_total_ordering::<T>();
            left_view.lemma_sort_by_ensures(leq);
            right_view.lemma_sort_by_ensures(leq);
            assert(left_view.to_multiset().len() == left_view.len());
            assert(left_view.sort_by(leq).to_multiset().len() == left_view.sort_by(leq).len());
            assert(left_view.sort_by(leq).to_multiset() =~= left_view.to_multiset());
            assert(sorted_left@.len() == left_view.len());
            assert(right_view.to_multiset().len() == right_view.len());
            assert(right_view.sort_by(leq).to_multiset().len() == right_view.sort_by(leq).len());
            assert(right_view.sort_by(leq).to_multiset() =~= right_view.to_multiset());
            assert(sorted_right@.len() == right_view.len());
        }

        let sorted = concat_three_vecs(&sorted_left, &equals, &sorted_right);
        proof {
            lemma_partition_sort_concat::<T>(
                s, left_view, right_view, equals@,
                sorted_left@, sorted_right@, pivot,
            );
        }
        sorted
    }

    /// Quicksort with median-of-three pivot. Self-recursive via ParaPair!.
    fn qsort_median3<T: TotalOrder + Eq + Clone + Send + 'static>(
        a: &ArraySeqMtEphSliceS<T>,
    ) -> (sorted: Vec<T>)
        requires
            a.slice_wf(),
            a.spec_len() <= usize::MAX,
            obeys_feq_clone::<T>(),
        ensures sorted@ =~= elements(*a).sort_by(spec_leq::<T>())
        decreases a.spec_len(),
    {
        let n = a.length();
        if n <= 1 { return qsort_base_case(a); }
        let ghost s = elements(*a);
        let ghost leq = spec_leq::<T>();
        let pivot_idx = median3_pivot_idx(a, n);
        let pivot = a.nth_cloned(pivot_idx);

        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();
        let mut equals: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                0 <= i <= n, n == a.spec_len() as usize, n <= usize::MAX, n >= 2,
                a.slice_wf(),
                obeys_feq_clone::<T>(),
                pivot_idx < n, pivot == s[pivot_idx as int], s == elements(*a),
                leq == spec_leq::<T>(),
                forall|j: int| 0 <= j < left@.len() ==>
                    (#[trigger] T::le(left@[j], pivot)) && left@[j] != pivot,
                forall|j: int| 0 <= j < right@.len() ==>
                    (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                forall|j: int| 0 <= j < equals@.len() ==>
                    (#[trigger] equals@[j]) == pivot,
                left@.len() + right@.len() + equals@.len() == i,
                i > pivot_idx ==> left@.len() + right@.len() < i,
                s.subrange(0, i as int).to_multiset() =~=
                    left@.to_multiset().add(right@.to_multiset()).add(equals@.to_multiset()),
            decreases n - i,
        {
            let elem = a.nth_cloned(i);
            proof {
                assert(s.subrange(0, (i + 1) as int) =~=
                    s.subrange(0, i as int).push(s[i as int]));
            }
            match TotalOrder::cmp(&elem, &pivot) {
                core::cmp::Ordering::Less => {
                    proof { assert(T::le(elem, pivot)); assert(elem != pivot); }
                    left.push(elem);
                },
                core::cmp::Ordering::Greater => {
                    proof { assert(T::le(pivot, elem)); assert(elem != pivot); }
                    right.push(elem);
                },
                core::cmp::Ordering::Equal => {
                    equals.push(elem);
                },
            }
            i = i + 1;
        }

        proof {
            assert(s.subrange(0, n as int) =~= s);
            assert(left@.len() + right@.len() < n);
            assert(equals@.len() >= 1);
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
        { qsort_median3(&left_a) };
        let f2 = move || -> (r: Vec<T>)
            ensures r@ =~= right_view.sort_by(spec_leq::<T>())
        { qsort_median3(&right_a) };
        let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

        proof {
            lemma_total_ordering::<T>();
            left_view.lemma_sort_by_ensures(leq);
            right_view.lemma_sort_by_ensures(leq);
            assert(left_view.to_multiset().len() == left_view.len());
            assert(left_view.sort_by(leq).to_multiset().len() == left_view.sort_by(leq).len());
            assert(left_view.sort_by(leq).to_multiset() =~= left_view.to_multiset());
            assert(sorted_left@.len() == left_view.len());
            assert(right_view.to_multiset().len() == right_view.len());
            assert(right_view.sort_by(leq).to_multiset().len() == right_view.sort_by(leq).len());
            assert(right_view.sort_by(leq).to_multiset() =~= right_view.to_multiset());
            assert(sorted_right@.len() == right_view.len());
        }

        let sorted = concat_three_vecs(&sorted_left, &equals, &sorted_right);
        proof {
            lemma_partition_sort_concat::<T>(
                s, left_view, right_view, equals@,
                sorted_left@, sorted_right@, pivot,
            );
        }
        sorted
    }

    /// Quicksort with random pivot. Self-recursive via ParaPair!.
    fn qsort_random<T: TotalOrder + Eq + Clone + Send + 'static>(
        a: &ArraySeqMtEphSliceS<T>,
    ) -> (sorted: Vec<T>)
        requires
            a.slice_wf(),
            a.spec_len() <= usize::MAX,
            obeys_feq_clone::<T>(),
        ensures sorted@ =~= elements(*a).sort_by(spec_leq::<T>())
        decreases a.spec_len(),
    {
        let n = a.length();
        if n <= 1 { return qsort_base_case(a); }
        let ghost s = elements(*a);
        let ghost leq = spec_leq::<T>();
        let pivot_idx = random_usize_range(0, n);
        let pivot = a.nth_cloned(pivot_idx);

        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();
        let mut equals: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                0 <= i <= n, n == a.spec_len() as usize, n <= usize::MAX, n >= 2,
                a.slice_wf(),
                obeys_feq_clone::<T>(),
                pivot_idx < n, pivot == s[pivot_idx as int], s == elements(*a),
                leq == spec_leq::<T>(),
                forall|j: int| 0 <= j < left@.len() ==>
                    (#[trigger] T::le(left@[j], pivot)) && left@[j] != pivot,
                forall|j: int| 0 <= j < right@.len() ==>
                    (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                forall|j: int| 0 <= j < equals@.len() ==>
                    (#[trigger] equals@[j]) == pivot,
                left@.len() + right@.len() + equals@.len() == i,
                i > pivot_idx ==> left@.len() + right@.len() < i,
                s.subrange(0, i as int).to_multiset() =~=
                    left@.to_multiset().add(right@.to_multiset()).add(equals@.to_multiset()),
            decreases n - i,
        {
            let elem = a.nth_cloned(i);
            proof {
                assert(s.subrange(0, (i + 1) as int) =~=
                    s.subrange(0, i as int).push(s[i as int]));
            }
            match TotalOrder::cmp(&elem, &pivot) {
                core::cmp::Ordering::Less => {
                    proof { assert(T::le(elem, pivot)); assert(elem != pivot); }
                    left.push(elem);
                },
                core::cmp::Ordering::Greater => {
                    proof { assert(T::le(pivot, elem)); assert(elem != pivot); }
                    right.push(elem);
                },
                core::cmp::Ordering::Equal => {
                    equals.push(elem);
                },
            }
            i = i + 1;
        }

        proof {
            assert(s.subrange(0, n as int) =~= s);
            assert(left@.len() + right@.len() < n);
            assert(equals@.len() >= 1);
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
        { qsort_random(&left_a) };
        let f2 = move || -> (r: Vec<T>)
            ensures r@ =~= right_view.sort_by(spec_leq::<T>())
        { qsort_random(&right_a) };
        let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);

        proof {
            lemma_total_ordering::<T>();
            left_view.lemma_sort_by_ensures(leq);
            right_view.lemma_sort_by_ensures(leq);
            assert(left_view.to_multiset().len() == left_view.len());
            assert(left_view.sort_by(leq).to_multiset().len() == left_view.sort_by(leq).len());
            assert(left_view.sort_by(leq).to_multiset() =~= left_view.to_multiset());
            assert(sorted_left@.len() == left_view.len());
            assert(right_view.to_multiset().len() == right_view.len());
            assert(right_view.sort_by(leq).to_multiset().len() == right_view.sort_by(leq).len());
            assert(right_view.sort_by(leq).to_multiset() =~= right_view.to_multiset());
            assert(sorted_right@.len() == right_view.len());
        }

        let sorted = concat_three_vecs(&sorted_left, &equals, &sorted_right);
        proof {
            lemma_partition_sort_concat::<T>(
                s, left_view, right_view, equals@,
                sorted_left@, sorted_right@, pivot,
            );
        }
        sorted
    }

    impl<T: TotalOrder + Eq + Clone + Send + 'static> QuickSortMtEphSliceTrait<T>
        for ArraySeqMtEphSliceS<T>
    {
        fn quick_sort_first(a: &mut ArraySeqMtEphSliceS<T>) {
            let ghost old_elems = elements(*a);
            let sorted = qsort_first(&*a);
            *a = ArraySeqMtEphSliceS::from_vec(sorted);
            proof {
                lemma_elements_from_vec::<T>(old_elems.sort_by(spec_leq::<T>()), *a);
            }
        }

        fn quick_sort_median3(a: &mut ArraySeqMtEphSliceS<T>) {
            let ghost old_elems = elements(*a);
            let sorted = qsort_median3(&*a);
            *a = ArraySeqMtEphSliceS::from_vec(sorted);
            proof {
                lemma_elements_from_vec::<T>(old_elems.sort_by(spec_leq::<T>()), *a);
            }
        }

        fn quick_sort_random(a: &mut ArraySeqMtEphSliceS<T>) {
            let ghost old_elems = elements(*a);
            let sorted = qsort_random(&*a);
            *a = ArraySeqMtEphSliceS::from_vec(sorted);
            proof {
                lemma_elements_from_vec::<T>(old_elems.sort_by(spec_leq::<T>()), *a);
            }
        }
    }

    } // verus!
} // mod
