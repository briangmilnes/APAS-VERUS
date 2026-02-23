//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Parallel Persistent (Chapter 35, Algorithm 35.2).
//! Randomized contraction-based selection for finding kth order statistic.
//! Verusified: select and select_inner are proven; rand is external_body in vstdplus.
//!
//! Parallelism: partition uses thread::scope to run left/right filters in parallel,
//! wrapped in external_body with the sequential spec. The structural select algorithm
//! and all multiset/sorting proofs remain fully verified.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 8. traits
// 9. impls

// 1. module

pub mod OrderStatSelectMtPer {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap35::OrderStatSelectStEph::OrderStatSelectStEph::{
        spec_kth, spec_leq, lemma_total_ordering};
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::rand::rand::random_usize_range;
    use vstd::relations::*;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    // 8. traits

    pub trait OrderStatSelectMtPerTrait<T: TotalOrder> {
        /// Find the kth smallest element (0-indexed) using contraction-based selection.
        /// - APAS: Work O(n) expected, Span O(lg^2 n) expected — Algorithm 35.2.
        fn select(a: &ArraySeqMtPerS<T>, k: usize) -> (result: Option<T>)
            requires a.spec_len() <= usize::MAX,
            ensures
                k >= a.spec_len() ==> result is None,
                k < a.spec_len() ==> result == Some(spec_kth::<T>(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), k as int));
    }

    // 9. impls

    #[verifier::external_body]
    fn parallel_three_way_partition<T: TotalOrder + Copy + Send + Sync + Eq + 'static>(
        a: &ArraySeqMtPerS<T>, pivot: T, pivot_idx: usize, n: usize,
    ) -> (result: (Vec<T>, usize, Vec<T>))
        requires
            n == a.spec_len(),
            n <= usize::MAX,
            n >= 2,
            pivot_idx < n,
            pivot == a.spec_index(pivot_idx as int),
        ensures
            forall|j: int| 0 <= j < result.0@.len() ==>
                (#[trigger] T::le(result.0@[j], pivot)) && result.0@[j] != pivot,
            forall|j: int| 0 <= j < result.2@.len() ==>
                (#[trigger] T::le(pivot, result.2@[j])) && result.2@[j] != pivot,
            result.0@.len() + result.1 + result.2@.len() == n,
            result.0@.len() + result.2@.len() < n,
            ({
                let s = Seq::new(n as nat, |i: int| a.spec_index(i));
                let eq_seq = Seq::new(result.1 as nat, |i: int| pivot);
                s.to_multiset() =~=
                    result.0@.to_multiset().add(result.2@.to_multiset()).add(eq_seq.to_multiset())
            }),
            result.1 >= 1,
    {
        std::thread::scope(|scope| {
            let left_handle = scope.spawn(|| {
                let mut left = Vec::new();
                for i in 0..n {
                    let elem = *a.nth(i);
                    if TotalOrder::cmp(&elem, &pivot) == core::cmp::Ordering::Less {
                        left.push(elem);
                    }
                }
                left
            });
            let right_handle = scope.spawn(|| {
                let mut right = Vec::new();
                for i in 0..n {
                    let elem = *a.nth(i);
                    if TotalOrder::cmp(&elem, &pivot) == core::cmp::Ordering::Greater {
                        right.push(elem);
                    }
                }
                right
            });
            let left = left_handle.join().unwrap();
            let right = right_handle.join().unwrap();
            let eq_count = n - left.len() - right.len();
            (left, eq_count, right)
        })
    }

    impl<T: TotalOrder + Copy + Send + Sync + Eq + 'static> OrderStatSelectMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn select(a: &ArraySeqMtPerS<T>, k: usize) -> (result: Option<T>)
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
    ) -> (result: Option<T>)
        requires
            a.spec_len() <= usize::MAX,
            0 <= k < a.spec_len(),
        ensures
            result == Some(spec_kth::<T>(
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
