//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Multi-threaded): Quicksort over `ArraySeqMtEph`.
//! Verusified: sort functions are proven via partition-sort-concat decomposition.
//! Uses parallel recursion via ParaPair! for left/right subarrays after partition.
//! Pivot: first-element (median3 and random use first-element, TODO).

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 9. impls

// 1. module

pub mod Chapter36Mt {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::Pair;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::relations::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap35::OrderStatSelectStEph::OrderStatSelectStEph::{
        spec_leq, lemma_total_ordering,
    };

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
    };

    // 9. impls

    /// Recursive non-mutating quicksort for MtEph arrays. Same proof as StEph variant.
    fn sort_vec<T: TotalOrder + Copy + Send + 'static>(a: &ArraySeqMtEphS<T>) -> (result: Vec<T>)
        requires a.spec_len() <= usize::MAX,
        ensures result@ =~= a.seq@.sort_by(spec_leq::<T>())
        decreases a.spec_len(),
    {
        let n = a.length();
        let ghost s = a.seq@;
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
            return Vec::new();
        }
        if n == 1 {
            let elem = *a.nth(0);
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
            return r;
        }

        let pivot = *a.nth(0);

        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();
        let mut equals: Vec<T> = Vec::new();
        let mut i: usize = 0;

        while i < n
            invariant
                0 <= i <= n,
                n == a.spec_len(),
                n <= usize::MAX,
                n >= 2,
                pivot == s[0],
                s == a.seq@,
                leq == spec_leq::<T>(),
                forall|j: int| 0 <= j < left@.len() ==>
                    (#[trigger] T::le(left@[j], pivot)) && left@[j] != pivot,
                forall|j: int| 0 <= j < right@.len() ==>
                    (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                forall|j: int| 0 <= j < equals@.len() ==>
                    (#[trigger] equals@[j]) == pivot,
                left@.len() + right@.len() + equals@.len() == i,
                i > 0 ==> left@.len() + right@.len() < i,
                s.subrange(0, i as int).to_multiset() =~=
                    left@.to_multiset().add(right@.to_multiset()).add(equals@.to_multiset()),
            decreases n - i,
        {
            let elem = *a.nth(i);

            proof {
                assert(s.subrange(0, (i + 1) as int) =~=
                    s.subrange(0, i as int).push(s[i as int]));
                assert(elem == s[i as int]);
            }

            match TotalOrder::cmp(&elem, &pivot) {
                core::cmp::Ordering::Less => {
                    proof {
                        assert(T::le(elem, pivot));
                        assert(elem != pivot);
                    }
                    left.push(elem);
                },
                core::cmp::Ordering::Greater => {
                    proof {
                        assert(T::le(pivot, elem));
                        assert(elem != pivot);
                    }
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
            assert(left@.len() + right@.len() + equals@.len() == n);
            assert(left@.len() + right@.len() < n);
            assert(equals@.len() >= 1);
        }

        let ghost left_view = left@;
        let ghost right_view = right@;
        let ghost equals_view = equals@;

        let left_a = ArraySeqMtEphS { seq: left };
        let right_a = ArraySeqMtEphS { seq: right };
        let equals_a = ArraySeqMtEphS { seq: equals };
        let f1 = move || -> (r: Vec<T>)
            ensures r@ =~= left_view.sort_by(spec_leq::<T>())
        {
            sort_vec(&left_a)
        };
        let f2 = move || -> (r: Vec<T>)
            ensures r@ =~= right_view.sort_by(spec_leq::<T>())
        {
            sort_vec(&right_a)
        };
        let Pair(sorted_left, sorted_right) = crate::ParaPair!(f1, f2);
        let sorted_left_a = ArraySeqMtEphS { seq: sorted_left };
        let sorted_right_a = ArraySeqMtEphS { seq: sorted_right };

        let sl = sorted_left_a.length();
        let el = equals_a.length();
        let sr = sorted_right_a.length();

        proof {
            lemma_total_ordering::<T>();
            assert(left_a.seq@ =~= left_view);
            assert(right_a.seq@ =~= right_view);
            assert(equals_a.seq@ =~= equals_view);
            left_a.seq@.lemma_sort_by_ensures(leq);
            right_a.seq@.lemma_sort_by_ensures(leq);
            assert(left_a.seq@.to_multiset().len() == left_a.seq@.len());
            assert(left_a.seq@.sort_by(leq).to_multiset().len() == left_a.seq@.sort_by(leq).len());
            assert(left_a.seq@.sort_by(leq).to_multiset() =~= left_a.seq@.to_multiset());
            assert(sorted_left_a.seq@.len() == left_view.len());
            assert(right_a.seq@.to_multiset().len() == right_a.seq@.len());
            assert(right_a.seq@.sort_by(leq).to_multiset().len() == right_a.seq@.sort_by(leq).len());
            assert(right_a.seq@.sort_by(leq).to_multiset() =~= right_a.seq@.to_multiset());
            assert(sorted_right_a.seq@.len() == right_view.len());
            assert(sl + el + sr == n);
        }

        let mut result: Vec<T> = Vec::new();

        let mut j: usize = 0;
        while j < sl
            invariant
                0 <= j <= sl,
                sl == sorted_left_a.spec_len(),
                el == equals_a.spec_len(),
                sr == sorted_right_a.spec_len(),
                sl + el + sr == n,
                n <= usize::MAX,
                result@.len() == j as nat,
                forall|k: int| 0 <= k < j as int ==>
                    #[trigger] result@[k] == sorted_left_a.seq@[k],
            decreases sl - j,
        {
            result.push(*sorted_left_a.nth(j));
            j = j + 1;
        }

        j = 0;
        while j < el
            invariant
                0 <= j <= el,
                el == equals_a.spec_len(),
                sl == sorted_left_a.spec_len(),
                sr == sorted_right_a.spec_len(),
                sl + el + sr == n,
                n <= usize::MAX,
                result@.len() == (sl + j) as nat,
                forall|k: int| 0 <= k < sl as int ==>
                    #[trigger] result@[k] == sorted_left_a.seq@[k],
                forall|k: int| 0 <= k < j as int ==>
                    #[trigger] result@[(sl + k) as int] == equals_a.seq@[k],
            decreases el - j,
        {
            result.push(*equals_a.nth(j));
            j = j + 1;
        }

        j = 0;
        while j < sr
            invariant
                0 <= j <= sr,
                sr == sorted_right_a.spec_len(),
                sl == sorted_left_a.spec_len(),
                el == equals_a.spec_len(),
                sl + el + sr == n,
                n <= usize::MAX,
                result@.len() == (sl + el + j) as nat,
                forall|k: int| 0 <= k < sl as int ==>
                    #[trigger] result@[k] == sorted_left_a.seq@[k],
                forall|k: int| 0 <= k < el as int ==>
                    #[trigger] result@[(sl + k) as int] == equals_a.seq@[k],
                forall|k: int| 0 <= k < j as int ==>
                    #[trigger] result@[(sl + el + k) as int] == sorted_right_a.seq@[k],
            decreases sr - j,
        {
            result.push(*sorted_right_a.nth(j));
            j = j + 1;
        }

        proof {
            lemma_total_ordering::<T>();

            let ghost sl_seq = sorted_left_a.seq@;
            let ghost sr_seq = sorted_right_a.seq@;
            let ghost eq_seq = equals_a.seq@;
            let ghost candidate = sl_seq + eq_seq + sr_seq;

            assert(result@.len() == candidate.len());
            assert forall|k: int| 0 <= k < result@.len()
                implies result@[k] == #[trigger] candidate[k] by
            {
                if k < sl as int {
                    assert(result@[k] == sorted_left_a.seq@[k]);
                    assert(candidate[k] == sl_seq[k]);
                } else if k < (sl + el) as int {
                    let k_prime = k - sl as int;
                    assert(0 <= k_prime < el as int);
                    assert(result@[(sl as int + k_prime)] == equals_a.seq@[k_prime]);
                    assert(candidate[k] == eq_seq[k_prime]);
                } else {
                    let k_prime = k - sl as int - el as int;
                    assert(0 <= k_prime < sr as int);
                    assert(result@[(sl as int + el as int + k_prime)] == sorted_right_a.seq@[k_prime]);
                    assert(candidate[k] == sr_seq[k_prime]);
                }
            };
            assert(result@ =~= candidate);

            assert(sl_seq =~= left_view.sort_by(leq));
            assert(sr_seq =~= right_view.sort_by(leq));

            left_view.lemma_sort_by_ensures(leq);
            right_view.lemma_sort_by_ensures(leq);
            s.lemma_sort_by_ensures(leq);

            assert(sl_seq.to_multiset() =~= left_view.to_multiset());
            assert(sr_seq.to_multiset() =~= right_view.to_multiset());

            assert(left_view.to_multiset().len() == left_view.len());
            assert(sl_seq.to_multiset().len() == sl_seq.len());
            assert(right_view.to_multiset().len() == right_view.len());
            assert(sr_seq.to_multiset().len() == sr_seq.len());
            assert(sl_seq.len() == left_view.len());
            assert(sr_seq.len() == right_view.len());
            assert(candidate.len() == n);

            assert forall|j: int| 0 <= j < sl_seq.len() implies
                T::le(#[trigger] sl_seq[j], pivot) && sl_seq[j] != pivot by
            {
                assert(sl_seq.to_multiset() =~= left_view.to_multiset());
                assert(sl_seq.to_multiset().count(sl_seq[j]) > 0);
                assert(left_view.to_multiset().count(sl_seq[j]) > 0);
                assert(left_view.contains(sl_seq[j]));
                let idx = choose|idx: int|
                    0 <= idx < left_view.len() && left_view[idx] == sl_seq[j];
            };

            assert forall|j: int| 0 <= j < sr_seq.len() implies
                T::le(pivot, #[trigger] sr_seq[j]) && sr_seq[j] != pivot by
            {
                assert(sr_seq.to_multiset() =~= right_view.to_multiset());
                assert(sr_seq.to_multiset().count(sr_seq[j]) > 0);
                assert(right_view.to_multiset().count(sr_seq[j]) > 0);
                assert(right_view.contains(sr_seq[j]));
                let idx = choose|idx: int|
                    0 <= idx < right_view.len() && right_view[idx] == sr_seq[j];
            };

            assert(sorted_by(candidate, leq)) by {
                assert forall|ai: int, bi: int|
                    0 <= ai < bi < candidate.len()
                    implies (#[trigger] leq(candidate[ai], candidate[bi])) by
                {
                    let ll = sl_seq.len();
                    let el_len = eq_seq.len();
                    if ai < ll && bi < ll {
                    } else if ai < ll && bi < ll + el_len {
                        assert(candidate[bi] == pivot);
                    } else if ai < ll && bi >= ll + el_len {
                        T::transitive(candidate[ai], pivot, candidate[bi]);
                    } else if ai >= ll && ai < ll + el_len && bi >= ll && bi < ll + el_len {
                        assert(candidate[ai] == pivot && candidate[bi] == pivot);
                        T::reflexive(pivot);
                    } else if ai >= ll && ai < ll + el_len && bi >= ll + el_len {
                        assert(candidate[ai] == pivot);
                    } else {
                        assert(ai >= ll + el_len && bi >= ll + el_len);
                    }
                };
            };

            vstd::seq_lib::lemma_multiset_commutative(sl_seq, eq_seq);
            vstd::seq_lib::lemma_multiset_commutative(sl_seq + eq_seq, sr_seq);
            assert(candidate.to_multiset() =~=
                sl_seq.to_multiset().add(eq_seq.to_multiset()).add(sr_seq.to_multiset()));
            assert(candidate.to_multiset() =~=
                left_view.to_multiset().add(eq_seq.to_multiset()).add(right_view.to_multiset()));
            assert(candidate.to_multiset() =~= s.to_multiset());

            vstd::seq_lib::lemma_sorted_unique(s.sort_by(leq), candidate, leq);
            assert(s.sort_by(leq) =~= candidate);
            assert(result@ =~= s.sort_by(leq));
        }

        result
    }

    /// Quicksort with first-element pivot. Parallel recursion via ParaPair!.
    pub fn quick_sort_first<T: TotalOrder + Copy + Send + 'static>(a: &mut ArraySeqMtEphS<T>)
        requires old(a).spec_len() <= usize::MAX,
        ensures a.seq@ =~= old(a).seq@.sort_by(spec_leq::<T>())
    {
        let result = sort_vec(&*a);
        a.seq = result;
    }

    /// Quicksort with median-of-three pivot. Currently uses first-element pivot (TODO).
    pub fn quick_sort_median3<T: TotalOrder + Copy + Send + 'static>(a: &mut ArraySeqMtEphS<T>)
        requires old(a).spec_len() <= usize::MAX,
        ensures a.seq@ =~= old(a).seq@.sort_by(spec_leq::<T>())
    {
        let result = sort_vec(&*a);
        a.seq = result;
    }

    /// Quicksort with random pivot. Currently uses first-element pivot (TODO).
    pub fn quick_sort_random<T: TotalOrder + Copy + Send + 'static>(a: &mut ArraySeqMtEphS<T>)
        requires old(a).spec_len() <= usize::MAX,
        ensures a.seq@ =~= old(a).seq@.sort_by(spec_leq::<T>())
    {
        let result = sort_vec(&*a);
        a.seq = result;
    }

    } // verus!
} // mod
