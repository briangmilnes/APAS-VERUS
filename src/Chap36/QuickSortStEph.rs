//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Single-threaded): Quicksort over `ArraySeqStEph`.
//! Three self-recursive trait methods: quick_sort_first, quick_sort_median3, quick_sort_random.
//! Each applies its pivot strategy at every recursive level.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls

// 1. module

pub mod QuickSortStEph {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::rand::rand::random_usize_range;
    use vstd::relations::*;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
    };

    // 6. spec fns

    /// Spec-level leq closure for sort_by and sorted_by.
    pub open spec fn spec_leq<T: TotalOrder>() -> spec_fn(T, T) -> bool {
        |x: T, y: T| T::le(x, y)
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

    // 8. traits

    pub trait QuickSortStEphTrait<T: TotalOrder> {
        /// Quicksort with first-element pivot.
        /// - APAS: Work O(n^2) worst, Span = Work — sequential.
        /// - Claude-Opus-4.6: Agrees.
        fn quick_sort_first(a: &mut ArraySeqStEphS<T>)
            requires old(a).spec_len() <= usize::MAX,
            ensures
                a.seq@ =~= old(a).seq@.sort_by(spec_leq::<T>()),
                a.spec_len() == old(a).spec_len(),
            decreases old(a).spec_len();

        /// Quicksort with median-of-three pivot.
        /// - APAS: Work O(n^2) worst / O(n lg n) sorted, Span = Work — sequential.
        /// - Claude-Opus-4.6: Agrees.
        fn quick_sort_median3(a: &mut ArraySeqStEphS<T>)
            requires old(a).spec_len() <= usize::MAX,
            ensures
                a.seq@ =~= old(a).seq@.sort_by(spec_leq::<T>()),
                a.spec_len() == old(a).spec_len(),
            decreases old(a).spec_len();

        /// Quicksort with random pivot.
        /// - APAS: Work O(n lg n) expected, Span = Work — sequential.
        /// - Claude-Opus-4.6: Agrees.
        fn quick_sort_random(a: &mut ArraySeqStEphS<T>)
            requires old(a).spec_len() <= usize::MAX,
            ensures
                a.seq@ =~= old(a).seq@.sort_by(spec_leq::<T>()),
                a.spec_len() == old(a).spec_len(),
            decreases old(a).spec_len();

        /// Compute the median of three values.
        fn median_of_three(a: T, b: T, c: T) -> (median: T)
            ensures
                median == a || median == b || median == c,
                median == spec_median_of_three(a, b, c);

        /// Returns index of median among a[0], a[n/2], a[n-1].
        fn median3_pivot_idx(a: &ArraySeqStEphS<T>, n: usize) -> (idx: usize)
            requires n >= 2, n == a.spec_len(),
            ensures
                idx < n,
                idx == 0 || idx == n / 2 || idx == n - 1,
                a.seq@[idx as int] == spec_median_of_three(a.seq@[0], a.seq@[(n / 2) as int], a.seq@[(n - 1) as int]);

        /// Concatenate three ArraySeqStEphS into one Vec.
        fn concat_three(
            left: &ArraySeqStEphS<T>,
            mid: &ArraySeqStEphS<T>,
            right: &ArraySeqStEphS<T>,
        ) -> (out: Vec<T>)
            requires left.spec_len() + mid.spec_len() + right.spec_len() <= usize::MAX,
            ensures out@ =~= left.seq@ + mid.seq@ + right.seq@;
    }

    // 9. impls

    impl<T: TotalOrder + Copy> QuickSortStEphTrait<T> for ArraySeqStEphS<T> {
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

        fn median3_pivot_idx(a: &ArraySeqStEphS<T>, n: usize) -> (idx: usize) {
            let first = *a.nth(0);
            let mid = *a.nth(n / 2);
            let last = *a.nth(n - 1);
            let median = Self::median_of_three(first, mid, last);
            match TotalOrder::cmp(a.nth(0), &median) {
                core::cmp::Ordering::Equal => 0,
                _ => match TotalOrder::cmp(a.nth(n / 2), &median) {
                    core::cmp::Ordering::Equal => n / 2,
                    _ => n - 1,
                },
            }
        }

        fn concat_three(
            left: &ArraySeqStEphS<T>,
            mid: &ArraySeqStEphS<T>,
            right: &ArraySeqStEphS<T>,
        ) -> (out: Vec<T>) {
            let sl = left.length();
            let el = mid.length();
            let sr = right.length();
            let mut out: Vec<T> = Vec::new();

            let mut j: usize = 0;
            while j < sl
                invariant
                    0 <= j <= sl, sl == left.spec_len(), el == mid.spec_len(),
                    sr == right.spec_len(), sl + el + sr <= usize::MAX,
                    out@.len() == j as nat,
                    forall|k: int| 0 <= k < j as int ==>
                        #[trigger] out@[k] == left.seq@[k],
                decreases sl - j,
            {
                out.push(*left.nth(j));
                j = j + 1;
            }

            j = 0;
            while j < el
                invariant
                    0 <= j <= el, sl == left.spec_len(), el == mid.spec_len(),
                    sr == right.spec_len(), sl + el + sr <= usize::MAX,
                    out@.len() == (sl + j) as nat,
                    forall|k: int| 0 <= k < sl as int ==>
                        #[trigger] out@[k] == left.seq@[k],
                    forall|k: int| 0 <= k < j as int ==>
                        #[trigger] out@[(sl + k) as int] == mid.seq@[k],
                decreases el - j,
            {
                out.push(*mid.nth(j));
                j = j + 1;
            }

            j = 0;
            while j < sr
                invariant
                    0 <= j <= sr, sl == left.spec_len(), el == mid.spec_len(),
                    sr == right.spec_len(), sl + el + sr <= usize::MAX,
                    out@.len() == (sl + el + j) as nat,
                    forall|k: int| 0 <= k < sl as int ==>
                        #[trigger] out@[k] == left.seq@[k],
                    forall|k: int| 0 <= k < el as int ==>
                        #[trigger] out@[(sl + k) as int] == mid.seq@[k],
                    forall|k: int| 0 <= k < j as int ==>
                        #[trigger] out@[(sl + el + k) as int] == right.seq@[k],
                decreases sr - j,
            {
                out.push(*right.nth(j));
                j = j + 1;
            }

            proof {
                let ghost l = left.seq@;
                let ghost m = mid.seq@;
                let ghost r = right.seq@;
                let ghost target = l + m + r;
                assert(out@.len() == target.len());
                assert forall|k: int| 0 <= k < out@.len()
                    implies out@[k] == #[trigger] target[k] by
                {
                    if k < sl as int {
                        assert(out@[k] == left.seq@[k]);
                        assert(target[k] == l[k]);
                    } else if k < (sl + el) as int {
                        let kp = k - sl as int;
                        assert(out@[(sl as int + kp)] == mid.seq@[kp]);
                        assert(target[k] == m[kp]);
                    } else {
                        let kp = k - sl as int - el as int;
                        assert(out@[(sl as int + el as int + kp)] == right.seq@[kp]);
                        assert(target[k] == r[kp]);
                    }
                };
            }

            out
        }

        fn quick_sort_first(a: &mut ArraySeqStEphS<T>)
            decreases old(a).spec_len(),
        {
            let n = a.length();
            if n <= 1 {
                let ghost s = a.seq@;
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
            let ghost s = a.seq@;
            let ghost leq = spec_leq::<T>();
            let pivot_idx: usize = 0;
            let pivot = *a.nth(pivot_idx);

            let mut left: Vec<T> = Vec::new();
            let mut right: Vec<T> = Vec::new();
            let mut equals: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n, n == a.spec_len(), n <= usize::MAX, n >= 2,
                    pivot_idx < n, pivot == s[pivot_idx as int], s == a.seq@,
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
                let elem = *a.nth(i);
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
            let mut left_a = ArraySeqStEphS { seq: left };
            let mut right_a = ArraySeqStEphS { seq: right };
            let equals_a = ArraySeqStEphS { seq: equals };
            Self::quick_sort_first(&mut left_a);
            Self::quick_sort_first(&mut right_a);

            proof {
                lemma_total_ordering::<T>();
                left_view.lemma_sort_by_ensures(leq);
                right_view.lemma_sort_by_ensures(leq);
            }

            let sorted = Self::concat_three(&left_a, &equals_a, &right_a);
            proof {
                lemma_partition_sort_concat::<T>(
                    s, left_view, right_view, equals_a.seq@,
                    left_a.seq@, right_a.seq@, pivot,
                );
            }
            a.seq = sorted;
        }

        fn quick_sort_median3(a: &mut ArraySeqStEphS<T>)
            decreases old(a).spec_len(),
        {
            let n = a.length();
            if n <= 1 {
                let ghost s = a.seq@;
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
            let ghost s = a.seq@;
            let ghost leq = spec_leq::<T>();
            let pivot_idx = Self::median3_pivot_idx(&*a, n);
            let pivot = *a.nth(pivot_idx);

            let mut left: Vec<T> = Vec::new();
            let mut right: Vec<T> = Vec::new();
            let mut equals: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n, n == a.spec_len(), n <= usize::MAX, n >= 2,
                    pivot_idx < n, pivot == s[pivot_idx as int], s == a.seq@,
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
                let elem = *a.nth(i);
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
            let mut left_a = ArraySeqStEphS { seq: left };
            let mut right_a = ArraySeqStEphS { seq: right };
            let equals_a = ArraySeqStEphS { seq: equals };
            Self::quick_sort_median3(&mut left_a);
            Self::quick_sort_median3(&mut right_a);

            proof {
                lemma_total_ordering::<T>();
                left_view.lemma_sort_by_ensures(leq);
                right_view.lemma_sort_by_ensures(leq);
                assert(left_a.seq@.len() == left_view.len());
                assert(right_a.seq@.len() == right_view.len());
            }

            let sorted = Self::concat_three(&left_a, &equals_a, &right_a);
            proof {
                lemma_partition_sort_concat::<T>(
                    s, left_view, right_view, equals_a.seq@,
                    left_a.seq@, right_a.seq@, pivot,
                );
            }
            a.seq = sorted;
        }

        fn quick_sort_random(a: &mut ArraySeqStEphS<T>)
            decreases old(a).spec_len(),
        {
            let n = a.length();
            if n <= 1 {
                let ghost s = a.seq@;
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
            let ghost s = a.seq@;
            let ghost leq = spec_leq::<T>();
            let pivot_idx = random_usize_range(0, n);
            let pivot = *a.nth(pivot_idx);

            let mut left: Vec<T> = Vec::new();
            let mut right: Vec<T> = Vec::new();
            let mut equals: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n, n == a.spec_len(), n <= usize::MAX, n >= 2,
                    pivot_idx < n, pivot == s[pivot_idx as int], s == a.seq@,
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
                let elem = *a.nth(i);
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
            let mut left_a = ArraySeqStEphS { seq: left };
            let mut right_a = ArraySeqStEphS { seq: right };
            let equals_a = ArraySeqStEphS { seq: equals };
            Self::quick_sort_random(&mut left_a);
            Self::quick_sort_random(&mut right_a);

            proof {
                lemma_total_ordering::<T>();
                left_view.lemma_sort_by_ensures(leq);
                right_view.lemma_sort_by_ensures(leq);
                assert(left_a.seq@.len() == left_view.len());
                assert(right_a.seq@.len() == right_view.len());
            }

            let sorted = Self::concat_three(&left_a, &equals_a, &right_a);
            proof {
                lemma_partition_sort_concat::<T>(
                    s, left_view, right_view, equals_a.seq@,
                    left_a.seq@, right_a.seq@, pivot,
                );
            }
            a.seq = sorted;
        }
    }

    } // verus!
} // mod
