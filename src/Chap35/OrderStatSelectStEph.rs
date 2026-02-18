//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Sequential Ephemeral (Chapter 35, Algorithm 35.2).
//! Randomized contraction-based selection for finding kth order statistic.
//! Verusified: select and select_inner are proven; rand is external_body in vstdplus.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls

// 1. module

pub mod OrderStatSelectStEph {

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

    /// The kth order statistic is the kth element of the sorted sequence.
    /// Definition 35.1: the element of rank k in a sequence.
    pub open spec fn spec_kth<T: TotalOrder>(s: Seq<T>, k: int) -> T
        recommends 0 <= k < s.len()
    {
        s.sort_by(spec_leq::<T>())[k]
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

    // 8. traits

    pub trait OrderStatSelectStEphTrait<T: TotalOrder> {
        /// Find the kth smallest element (0-indexed) using contraction-based selection.
        /// - APAS: Work O(n) expected, Span O(lg^2 n) expected — Algorithm 35.2.
        /// - Claude-Opus-4.6: Work O(n) expected, Span O(n) — sequential, no parallelism.
        fn select(a: &ArraySeqStEphS<T>, k: usize) -> (result: Option<T>)
            requires a.spec_len() <= usize::MAX,
            ensures
                k >= a.spec_len() ==> result is None,
                k < a.spec_len() ==> result == Some(spec_kth::<T>(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), k as int));
    }

    // 9. impls

    impl<T: TotalOrder + Copy> OrderStatSelectStEphTrait<T> for ArraySeqStEphS<T> {
        fn select(a: &ArraySeqStEphS<T>, k: usize) -> (result: Option<T>)
        {
            let n = a.length();
            if k >= n {
                return None;
            }
            select_inner(a, k)
        }
    }

    /// Recursive contraction-based selection. Fully verified: the only external_body
    /// in the call chain is vstdplus::rand::random_usize_range.
    fn select_inner<T: TotalOrder + Copy>(a: &ArraySeqStEphS<T>, k: usize) -> (result: Option<T>)
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

        // Base case: single element.
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

        // Partition into left (< pivot), right (> pivot), equals (== pivot).
        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();
        let ghost mut equals_seq: Seq<T> = Seq::empty();
        let mut i: usize = 0;

        while i < n
            invariant
                0 <= i <= n,
                n == a.spec_len(),
                n <= usize::MAX,
                n >= 2,
                pivot_idx < n,
                pivot == a.spec_index(pivot_idx as int),
                s == Seq::new(n as nat, |j: int| a.spec_index(j)),
                leq == spec_leq::<T>(),
                // Content invariants
                forall|j: int| 0 <= j < left@.len() ==>
                    (#[trigger] T::le(left@[j], pivot)) && left@[j] != pivot,
                forall|j: int| 0 <= j < right@.len() ==>
                    (#[trigger] T::le(pivot, right@[j])) && right@[j] != pivot,
                forall|j: int| 0 <= j < equals_seq.len() ==>
                    (#[trigger] equals_seq[j]) == pivot,
                // Exact count: every processed element is in exactly one partition
                left@.len() + right@.len() + equals_seq.len() == i,
                // Pivot tracking: after processing pivot_idx, one element is in equals
                i > pivot_idx ==> left@.len() + right@.len() < i,
                // Multiset decomposition
                s.subrange(0, i as int).to_multiset() =~=
                    left@.to_multiset().add(right@.to_multiset()).add(equals_seq.to_multiset()),
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
                    proof {
                        assert(elem == pivot);
                        equals_seq = equals_seq.push(elem);
                    }
                },
            }
            i = i + 1;
        }

        let left_count = left.len();
        let right_count = right.len();

        // Post-loop ghost state for the partition-sort proof.
        proof { lemma_total_ordering::<T>(); }
        let ghost sorted_left = left@.sort_by(leq);
        let ghost sorted_right = right@.sort_by(leq);
        let ghost candidate = sorted_left + equals_seq + sorted_right;

        proof {
            // The full sequence multiset decomposes into the three partitions.
            assert(s.subrange(0, n as int) =~= s);

            left@.lemma_sort_by_ensures(leq);
            right@.lemma_sort_by_ensures(leq);
            s.lemma_sort_by_ensures(leq);

            // Multiset equality from sort_by.
            assert(left@.to_multiset() =~= sorted_left.to_multiset());
            assert(right@.to_multiset() =~= sorted_right.to_multiset());
            assert(s.to_multiset() =~= s.sort_by(leq).to_multiset());

            // Trigger to_multiset_len broadcast for each sequence.
            assert(left@.to_multiset().len() == left@.len());
            assert(sorted_left.to_multiset().len() == sorted_left.len());
            assert(right@.to_multiset().len() == right@.len());
            assert(sorted_right.to_multiset().len() == sorted_right.len());
            assert(s.to_multiset().len() == s.len());
            assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());

            // Length preservation: sort_by doesn't change length.
            assert(sorted_left.len() == left@.len());
            assert(sorted_right.len() == right@.len());
            assert(s.sort_by(leq).len() == s.len());
            assert(left@.len() + right@.len() + equals_seq.len() == n);
            assert(candidate.len() == n);

            // sorted_left has same elements as left: all < pivot.
            assert forall|j: int| 0 <= j < sorted_left.len() implies
                T::le(#[trigger] sorted_left[j], pivot) && sorted_left[j] != pivot by
            {
                assert(sorted_left.to_multiset().count(sorted_left[j]) > 0);
                assert(left@.to_multiset().count(sorted_left[j]) > 0);
                assert(left@.contains(sorted_left[j]));
                let idx = choose|idx: int|
                    0 <= idx < left@.len() && left@[idx] == sorted_left[j];
            };

            // sorted_right has same elements as right: all > pivot.
            assert forall|j: int| 0 <= j < sorted_right.len() implies
                T::le(pivot, #[trigger] sorted_right[j]) && sorted_right[j] != pivot by
            {
                assert(sorted_right.to_multiset().count(sorted_right[j]) > 0);
                assert(right@.to_multiset().count(sorted_right[j]) > 0);
                assert(right@.contains(sorted_right[j]));
                let idx = choose|idx: int|
                    0 <= idx < right@.len() && right@[idx] == sorted_right[j];
            };

            // The three-part concatenation is sorted because left < pivot == equals < right.
            assert(sorted_by(candidate, leq)) by {
                assert forall|ai: int, bi: int|
                    0 <= ai < bi < candidate.len()
                    implies (#[trigger] leq(candidate[ai], candidate[bi])) by
                {
                    let ll = sorted_left.len();
                    let el = equals_seq.len();
                    if ai < ll && bi < ll {
                        // Both in sorted_left — already sorted.
                    } else if ai < ll && bi < ll + el {
                        // a in sorted_left (< pivot), b in equals (== pivot).
                        assert(candidate[bi] == pivot);
                    } else if ai < ll && bi >= ll + el {
                        // a in sorted_left (< pivot), b in sorted_right (> pivot).
                        T::transitive(candidate[ai], pivot, candidate[bi]);
                    } else if ai >= ll && ai < ll + el && bi >= ll && bi < ll + el {
                        // Both in equals (== pivot).
                        assert(candidate[ai] == pivot && candidate[bi] == pivot);
                        T::reflexive(pivot);
                    } else if ai >= ll && ai < ll + el && bi >= ll + el {
                        // a in equals (== pivot), b in sorted_right (> pivot).
                        assert(candidate[ai] == pivot);
                    } else {
                        // Both in sorted_right — already sorted.
                        assert(ai >= ll + el && bi >= ll + el);
                    }
                };
            };

            // candidate has the same multiset as s.
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
            assert(candidate.to_multiset() =~= s.to_multiset());

            // By uniqueness of sorting: sort(s) == candidate.
            vstd::seq_lib::lemma_sorted_unique(
                s.sort_by(leq), candidate, leq);

            // Key fact for all branches: sort(s) and candidate agree element-wise.
            assert(s.sort_by(leq) =~= candidate);
        }

        if k < left_count {
            let left_a = ArraySeqStEphS { seq: left };
            proof {
                // k < left_count == left@.len() == sorted_left.len()
                assert((k as int) < sorted_left.len());
                assert((k as int) < candidate.len());
                // sort(s)[k] == candidate[k] == sorted_left[k]
                assert(candidate[k as int] == sorted_left[k as int]);
                // left_a.view == left@, so spec_kth(left_a_view, k) == spec_kth(left@, k)
                let left_a_view = Seq::new(
                    left_a.spec_len(), |j: int| left_a.spec_index(j));
                assert(left_a_view =~= left@);
            }
            select_inner(&left_a, k)
        } else if k < n - right_count {
            proof {
                // n - right_count == left@.len() + equals_seq.len() == sorted_left.len() + equals_seq.len()
                assert(n - right_count == left_count + equals_seq.len());
                assert(left_count == sorted_left.len());
                // k is in the equals region: sorted_left.len() <= k < sorted_left.len() + equals_seq.len()
                assert(k as int >= sorted_left.len());
                assert((k as int) < sorted_left.len() + equals_seq.len());
                assert((k as int) < candidate.len());
                // sort(s)[k] == candidate[k] == equals_seq[k - sorted_left.len()] == pivot
                assert(candidate[k as int] == pivot);
            }
            Some(pivot)
        } else {
            let right_a = ArraySeqStEphS { seq: right };
            let new_k = k - (n - right_count);
            proof {
                // n - right_count == sorted_left.len() + equals_seq.len()
                assert(n - right_count == left_count + equals_seq.len());
                assert(left_count == sorted_left.len());
                let ll = sorted_left.len();
                let el = equals_seq.len();
                assert(new_k as int == k as int - ll - el);
                assert(new_k as int >= 0);
                assert((new_k as int) < sorted_right.len());
                assert((k as int) < candidate.len());
                // sort(s)[k] == candidate[k] == sorted_right[k - ll - el] == sorted_right[new_k]
                assert(candidate[k as int] == sorted_right[new_k as int]);
                // right_a.view == right@, so spec_kth(right_a_view, new_k) == spec_kth(right@, new_k)
                let right_a_view = Seq::new(
                    right_a.spec_len(), |j: int| right_a.spec_index(j));
                assert(right_a_view =~= right@);
            }
            select_inner(&right_a, new_k)
        }
    }

    } // verus!
} // mod
