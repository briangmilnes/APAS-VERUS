//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Parallel Persistent (Chapter 35, Algorithm 35.2).
//! Randomized contraction-based selection for finding kth order statistic.
//! Verusified: select and select_inner are proven; rand is external_body in vstdplus.
//!
//! Parallelism: partition uses join() from HFSchedulerMtEph to run left/right filters
//! in parallel. Both filter closures and the multiset decomposition are fully verified.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls

// 1. module

pub mod OrderStatSelectMtPer {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::rand::rand::random_usize_range;
    use vstd::relations::*;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    // 6. spec fns

    /// Spec-level leq closure for sort_by and sorted_by.
    pub open spec fn spec_leq<T: TotalOrder>() -> spec_fn(T, T) -> bool {
        |x: T, y: T| T::le(x, y)
    }

    /// The kth order statistic is the kth element of the sorted sequence.
    /// Definition 35.1: the element of rank k in a sequence.
    pub open spec fn spec_kth<T: TotalOrder>(s: Seq<T>, k: int) -> T
        recommends 0 <= k < s.len()
    {
        s.sort_by(spec_leq::<T>())[k]
    }

    /// Constant sequence of n copies of v.
    pub open spec fn spec_const_seq<T>(n: nat, v: T) -> Seq<T> {
        Seq::new(n, |unused: int| v)
    }

    // 7. proof fns

    /// Bridge from the TotalOrder trait to vstd's total_ordering predicate.
    pub proof fn lemma_total_ordering<T: TotalOrder>()
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

    /// Multiset of a constant sequence: count(v) == n, count(x) == 0 for x != v.
    proof fn lemma_const_seq_multiset<T>(n: nat, v: T)
        ensures
            spec_const_seq(n, v).to_multiset().count(v) == n,
            forall|x: T| x != v ==>
                (#[trigger] spec_const_seq(n, v).to_multiset().count(x)) == 0nat,
        decreases n,
    {
        if n > 0 {
            lemma_const_seq_multiset::<T>((n - 1) as nat, v);
            assert(spec_const_seq(n, v) =~= spec_const_seq((n - 1) as nat, v).push(v));
        }
    }

    // 8. traits

    pub trait OrderStatSelectMtPerTrait<T: TotalOrder> {
        /// Find the kth smallest element (0-indexed) using contraction-based selection.
        /// - APAS: Work O(n) expected, Span O(lg^2 n) expected — Algorithm 35.2.
        fn select(a: &ArraySeqMtPerS<T>, k: usize) -> (found: Option<T>)
            requires a.spec_len() <= usize::MAX,
            ensures
                k >= a.spec_len() ==> found is None,
                k < a.spec_len() ==> found == Some(spec_kth::<T>(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), k as int));
    }

    // 9. impls

    /// Parallel three-way partition: splits array into (elements < pivot, eq_count, elements > pivot).
    /// Uses join() for genuine parallelism: left and right filters run concurrently.
    /// Both filter closures and the post-join multiset assembly are fully verified.
    fn parallel_three_way_partition<T: TotalOrder + Copy + Send + Sync + Eq + 'static>(
        a: &ArraySeqMtPerS<T>, pivot: T, pivot_idx: usize, n: usize,
    ) -> (partition: (Vec<T>, usize, Vec<T>))
        requires
            n == a.spec_len(),
            n <= usize::MAX,
            n >= 2,
            pivot_idx < n,
            pivot == a.spec_index(pivot_idx as int),
        ensures
            forall|j: int| 0 <= j < partition.0@.len() ==>
                (#[trigger] T::le(partition.0@[j], pivot)) && partition.0@[j] != pivot,
            forall|j: int| 0 <= j < partition.2@.len() ==>
                (#[trigger] T::le(pivot, partition.2@[j])) && partition.2@[j] != pivot,
            partition.0@.len() + partition.1 + partition.2@.len() == n,
            partition.0@.len() + partition.2@.len() < n,
            ({
                let s = Seq::new(n as nat, |i: int| a.spec_index(i));
                let eq_seq = Seq::new(partition.1 as nat, |i: int| pivot);
                s.to_multiset() =~=
                    partition.0@.to_multiset().add(partition.2@.to_multiset()).add(eq_seq.to_multiset())
            }),
            partition.1 >= 1,
    {
        let ghost s = Seq::new(n as nat, |i: int| a.spec_index(i));

        // Copy array data so each closure owns its own Vec.
        let mut data_l: Vec<T> = Vec::new();
        let mut data_r: Vec<T> = Vec::new();
        let mut ci: usize = 0;
        while ci < n
            invariant
                0 <= ci <= n,
                n == a.spec_len(),
                n <= usize::MAX,
                s == Seq::new(n as nat, |j: int| a.spec_index(j)),
                data_l@.len() == ci,
                data_r@.len() == ci,
                forall|j: int| #![trigger data_l@[j]]
                    0 <= j < ci ==> data_l@[j] == s[j],
                forall|j: int| #![trigger data_r@[j]]
                    0 <= j < ci ==> data_r@[j] == s[j],
            decreases n - ci,
        {
            let elem = *a.nth(ci);
            data_l.push(elem);
            data_r.push(elem);
            ci = ci + 1;
        }

        proof {
            assert(data_l@ =~= s);
            assert(data_r@ =~= s);
        }

        let ghost s_l = data_l@;
        let ghost s_r = data_r@;

        // f_left: filter < pivot, count == pivot.
        let f_left = move || -> (pair: (Vec<T>, usize))
            ensures
                forall|j: int| 0 <= j < pair.0@.len() ==>
                    (#[trigger] T::le(pair.0@[j], pivot)) && pair.0@[j] != pivot,
                forall|x: T| (T::le(x, pivot) && x != pivot) ==>
                    (#[trigger] pair.0@.to_multiset().count(x)) == s_l.to_multiset().count(x),
                forall|x: T| !(T::le(x, pivot) && x != pivot) ==>
                    (#[trigger] pair.0@.to_multiset().count(x)) == 0nat,
                pair.1 == s_l.to_multiset().count(pivot),
                pair.1 >= 1,
                pair.0@.len() + pair.1 <= n,
        {
            let mut left: Vec<T> = Vec::new();
            let mut eq_count: usize = 0;
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    n == data_l.len(),
                    n <= usize::MAX,
                    data_l@ == s_l,
                    pivot_idx < n,
                    pivot == s_l[pivot_idx as int],
                    forall|j: int| 0 <= j < left@.len() ==>
                        (#[trigger] T::le(left@[j], pivot)) && left@[j] != pivot,
                    forall|x: T| (T::le(x, pivot) && x != pivot) ==>
                        (#[trigger] left@.to_multiset().count(x)) ==
                            s_l.subrange(0, i as int).to_multiset().count(x),
                    forall|x: T| !(T::le(x, pivot) && x != pivot) ==>
                        (#[trigger] left@.to_multiset().count(x)) == 0nat,
                    eq_count == s_l.subrange(0, i as int).to_multiset().count(pivot),
                    i > pivot_idx ==> eq_count >= 1,
                    left@.len() + eq_count <= i,
                decreases n - i,
            {
                let elem = data_l[i];
                proof {
                    assert(s_l.subrange(0, (i + 1) as int) =~=
                        s_l.subrange(0, i as int).push(s_l[i as int]));
                    assert(elem == s_l[i as int]);
                }

                match TotalOrder::cmp(&elem, &pivot) {
                    core::cmp::Ordering::Less => {
                        proof {
                            assert(T::le(elem, pivot));
                            assert(elem != pivot);
                        }
                        left.push(elem);
                    },
                    core::cmp::Ordering::Equal => {
                        proof { assert(elem == pivot); }
                        eq_count = eq_count + 1;
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            assert(T::le(pivot, elem));
                            assert(elem != pivot);
                        }
                    },
                }
                proof {
                    assert forall|x: T| (T::le(x, pivot) && x != pivot) implies
                        (#[trigger] left@.to_multiset().count(x)) ==
                            s_l.subrange(0, (i + 1) as int).to_multiset().count(x)
                    by {
                        if x == elem && T::le(pivot, elem) {
                            T::antisymmetric(elem, pivot);
                        }
                    };
                    assert forall|x: T| !(T::le(x, pivot) && x != pivot) implies
                        (#[trigger] left@.to_multiset().count(x)) == 0nat
                    by {
                        if x == elem && T::le(elem, pivot) && elem != pivot {
                            assert(T::le(x, pivot) && x != pivot);
                        }
                    };
                }
                i = i + 1;
            }
            proof { assert(s_l.subrange(0, n as int) =~= s_l); }
            (left, eq_count)
        };

        // f_right: filter > pivot.
        let f_right = move || -> (right: Vec<T>)
            ensures
                forall|j: int| 0 <= j < right@.len() ==>
                    (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                forall|x: T| (T::le(pivot, x) && x != pivot) ==>
                    (#[trigger] right@.to_multiset().count(x)) == s_r.to_multiset().count(x),
                forall|x: T| !(T::le(pivot, x) && x != pivot) ==>
                    (#[trigger] right@.to_multiset().count(x)) == 0nat,
        {
            let mut right: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    n == data_r.len(),
                    n <= usize::MAX,
                    data_r@ == s_r,
                    forall|j: int| 0 <= j < right@.len() ==>
                        (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                    forall|x: T| (T::le(pivot, x) && x != pivot) ==>
                        (#[trigger] right@.to_multiset().count(x)) ==
                            s_r.subrange(0, i as int).to_multiset().count(x),
                    forall|x: T| !(T::le(pivot, x) && x != pivot) ==>
                        (#[trigger] right@.to_multiset().count(x)) == 0nat,
                    right@.len() <= i,
                decreases n - i,
            {
                let elem = data_r[i];
                proof {
                    assert(s_r.subrange(0, (i + 1) as int) =~=
                        s_r.subrange(0, i as int).push(s_r[i as int]));
                    assert(elem == s_r[i as int]);
                }

                match TotalOrder::cmp(&elem, &pivot) {
                    core::cmp::Ordering::Greater => {
                        proof {
                            assert(T::le(pivot, elem));
                            assert(elem != pivot);
                        }
                        right.push(elem);
                    },
                    _ => {},
                }
                proof {
                    assert forall|x: T| (T::le(pivot, x) && x != pivot) implies
                        (#[trigger] right@.to_multiset().count(x)) ==
                            s_r.subrange(0, (i + 1) as int).to_multiset().count(x)
                    by {
                        if x == elem && T::le(elem, pivot) {
                            T::antisymmetric(elem, pivot);
                        }
                    };
                    assert forall|x: T| !(T::le(pivot, x) && x != pivot) implies
                        (#[trigger] right@.to_multiset().count(x)) == 0nat
                    by {
                        if x == elem && T::le(pivot, elem) && elem != pivot {
                            assert(T::le(pivot, x) && x != pivot);
                        }
                    };
                }
                i = i + 1;
            }
            proof { assert(s_r.subrange(0, n as int) =~= s_r); }
            right
        };

        let ((left, eq_count), right) = join(f_left, f_right);

        proof {
            assert(s_l =~= s);
            assert(s_r =~= s);

            let eq_seq = Seq::new(eq_count as nat, |unused: int| pivot);
            lemma_const_seq_multiset::<T>(eq_count as nat, pivot);

            // Multiset decomposition by extensional equality over element counts.
            assert forall|x: T|
                s.to_multiset().count(x) ==
                (#[trigger] left@.to_multiset().count(x)) +
                right@.to_multiset().count(x) +
                eq_seq.to_multiset().count(x)
            by {
                T::total(x, pivot);
                if x == pivot {
                    assert(!(T::le(pivot, pivot) && pivot != pivot));
                    assert(left@.to_multiset().count(pivot) == 0nat);
                    assert(right@.to_multiset().count(pivot) == 0nat);
                } else if T::le(x, pivot) {
                    assert(T::le(x, pivot) && x != pivot);
                    if T::le(pivot, x) {
                        T::antisymmetric(x, pivot);
                    }
                    assert(!(T::le(pivot, x) && x != pivot));
                    assert(right@.to_multiset().count(x) == 0nat);
                    assert(eq_seq.to_multiset().count(x) == 0nat);
                } else {
                    assert(T::le(pivot, x) && x != pivot);
                    if T::le(x, pivot) {
                        T::antisymmetric(x, pivot);
                    }
                    assert(!(T::le(x, pivot) && x != pivot));
                    assert(left@.to_multiset().count(x) == 0nat);
                    assert(eq_seq.to_multiset().count(x) == 0nat);
                }
            };

            assert(s.to_multiset() =~=
                left@.to_multiset().add(right@.to_multiset()).add(eq_seq.to_multiset()));

            // Derive size constraints from multiset lengths.
            assert(s.to_multiset().len() == n);
            assert(left@.to_multiset().len() == left@.len());
            assert(right@.to_multiset().len() == right@.len());
            assert(eq_seq.to_multiset().len() == eq_seq.len());
        }

        (left, eq_count, right)
    }

    impl<T: TotalOrder + Copy + Send + Sync + Eq + 'static> OrderStatSelectMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn select(a: &ArraySeqMtPerS<T>, k: usize) -> (found: Option<T>)
        {
            let n = a.length();
            if k >= n {
                return None;
            }
            select_inner(a, k)
        }
    }

    fn select_inner<T: TotalOrder + Copy + Send + Sync + Eq + 'static>(
        a: &ArraySeqMtPerS<T>, k: usize,
    ) -> (found: Option<T>)
        requires
            a.spec_len() <= usize::MAX,
            0 <= k < a.spec_len(),
        ensures
            found == Some(spec_kth::<T>(
                Seq::new(a.spec_len(), |i: int| a.spec_index(i)), k as int)),
        decreases a.spec_len(),
    {
        let n = a.length();
        let ghost s = Seq::new(n as nat, |i: int| a.spec_index(i));
        let ghost leq = spec_leq::<T>();

        if n == 1 {
            let elem = *a.nth(0);
            proof {
                lemma_total_ordering::<T>();
                s.lemma_sort_by_ensures(leq);
                assert(sorted_by(s, leq));
                vstd::seq_lib::lemma_sorted_unique(s, s.sort_by(leq), leq);
                assert(k as int == 0);
            }
            return Some(elem);
        }

        let pivot_idx = random_usize_range(0, n);
        let pivot = *a.nth(pivot_idx);

        let (left, eq_count, right) = parallel_three_way_partition(a, pivot, pivot_idx, n);
        let ghost equals_seq: Seq<T> = Seq::new(eq_count as nat, |i: int| pivot);

        let left_count = left.len();
        let right_count = right.len();

        proof { lemma_total_ordering::<T>(); }
        let ghost sorted_left = left@.sort_by(leq);
        let ghost sorted_right = right@.sort_by(leq);
        let ghost candidate = sorted_left + equals_seq + sorted_right;

        proof {
            left@.lemma_sort_by_ensures(leq);
            right@.lemma_sort_by_ensures(leq);
            s.lemma_sort_by_ensures(leq);

            assert(left@.to_multiset() =~= sorted_left.to_multiset());
            assert(right@.to_multiset() =~= sorted_right.to_multiset());
            assert(s.to_multiset() =~= s.sort_by(leq).to_multiset());

            assert(left@.to_multiset().len() == left@.len());
            assert(sorted_left.to_multiset().len() == sorted_left.len());
            assert(right@.to_multiset().len() == right@.len());
            assert(sorted_right.to_multiset().len() == sorted_right.len());
            assert(s.to_multiset().len() == s.len());
            assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());

            assert(sorted_left.len() == left@.len());
            assert(sorted_right.len() == right@.len());
            assert(s.sort_by(leq).len() == s.len());
            assert(left@.len() + right@.len() + equals_seq.len() == n);
            assert(candidate.len() == n);

            assert forall|j: int| 0 <= j < sorted_left.len() implies
                T::le(#[trigger] sorted_left[j], pivot) && sorted_left[j] != pivot by
            {
                assert(sorted_left.to_multiset().count(sorted_left[j]) > 0);
                assert(left@.to_multiset().count(sorted_left[j]) > 0);
                assert(left@.contains(sorted_left[j]));
                let idx = choose|idx: int|
                    0 <= idx < left@.len() && left@[idx] == sorted_left[j];
            };

            assert forall|j: int| 0 <= j < sorted_right.len() implies
                T::le(pivot, #[trigger] sorted_right[j]) && sorted_right[j] != pivot by
            {
                assert(sorted_right.to_multiset().count(sorted_right[j]) > 0);
                assert(right@.to_multiset().count(sorted_right[j]) > 0);
                assert(right@.contains(sorted_right[j]));
                let idx = choose|idx: int|
                    0 <= idx < right@.len() && right@[idx] == sorted_right[j];
            };

            assert(sorted_by(candidate, leq)) by {
                assert forall|ai: int, bi: int|
                    0 <= ai < bi < candidate.len()
                    implies (#[trigger] leq(candidate[ai], candidate[bi])) by
                {
                    let ll = sorted_left.len();
                    let el = equals_seq.len();
                    if ai < ll && bi < ll {
                    } else if ai < ll && bi < ll + el {
                        assert(candidate[bi] == pivot);
                    } else if ai < ll && bi >= ll + el {
                        T::transitive(candidate[ai], pivot, candidate[bi]);
                    } else if ai >= ll && ai < ll + el && bi >= ll && bi < ll + el {
                        assert(candidate[ai] == pivot && candidate[bi] == pivot);
                        T::reflexive(pivot);
                    } else if ai >= ll && ai < ll + el && bi >= ll + el {
                        assert(candidate[ai] == pivot);
                    } else {
                        assert(ai >= ll + el && bi >= ll + el);
                    }
                };
            };

            vstd::seq_lib::lemma_multiset_commutative(sorted_left, equals_seq);
            vstd::seq_lib::lemma_multiset_commutative(
                sorted_left + equals_seq, sorted_right);
            assert(candidate.to_multiset() =~=
                sorted_left.to_multiset().add(
                    equals_seq.to_multiset()).add(
                    sorted_right.to_multiset()));
            assert(candidate.to_multiset() =~=
                left@.to_multiset().add(
                    equals_seq.to_multiset()).add(
                    right@.to_multiset()));
            assert(s.to_multiset() =~=
                left@.to_multiset().add(right@.to_multiset()).add(equals_seq.to_multiset()));
            assert(candidate.to_multiset() =~= s.to_multiset());

            vstd::seq_lib::lemma_sorted_unique(
                s.sort_by(leq), candidate, leq);
            assert(s.sort_by(leq) =~= candidate);
        }

        if k < left_count {
            let left_a = ArraySeqMtPerS { seq: left };
            proof {
                assert((k as int) < sorted_left.len());
                assert((k as int) < candidate.len());
                assert(candidate[k as int] == sorted_left[k as int]);
                let left_a_view = Seq::new(
                    left_a.spec_len(), |j: int| left_a.spec_index(j));
                assert(left_a_view =~= left@);
            }
            select_inner(&left_a, k)
        } else if k < n - right_count {
            proof {
                assert(n - right_count == left_count + equals_seq.len());
                assert(left_count == sorted_left.len());
                assert(k as int >= sorted_left.len());
                assert((k as int) < sorted_left.len() + equals_seq.len());
                assert((k as int) < candidate.len());
                assert(candidate[k as int] == pivot);
            }
            Some(pivot)
        } else {
            let right_a = ArraySeqMtPerS { seq: right };
            let new_k = k - (n - right_count);
            proof {
                assert(n - right_count == left_count + equals_seq.len());
                assert(left_count == sorted_left.len());
                let ll = sorted_left.len();
                let el = equals_seq.len();
                assert(new_k as int == k as int - ll - el);
                assert(new_k as int >= 0);
                assert((new_k as int) < sorted_right.len());
                assert((k as int) < candidate.len());
                assert(candidate[k as int] == sorted_right[new_k as int]);
                let right_a_view = Seq::new(
                    right_a.spec_len(), |j: int| right_a.spec_index(j));
                assert(right_a_view =~= right@);
            }
            select_inner(&right_a, new_k)
        }
    }

    } // verus!
} // mod
