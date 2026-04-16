// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Order Statistics - Parallel Ephemeral (Chapter 35, Algorithm 35.2).
//! Randomized contraction-based selection for finding kth order statistic.
//! Verusified: select and select_inner are proven; rand is external_body in vstdplus.
//!
//! Parallelism: partition uses D&C parallel three-way partition via join() with O(lg n)
//! span for the divide phase. Both filter closures and the multiset decomposition are
//! fully verified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod OrderStatSelectMtEph {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Concurrency::Concurrency::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::rand::rand::random_usize_range;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use vstd::relations::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::multiset::group_multiset_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    //		Section 6. spec fns


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

    /// Extract the element sequence from a slice as Seq<T>.
    pub open spec fn spec_slice_elements<T: StTInMtT>(a: ArraySeqMtEphSliceS<T>) -> Seq<T> {
        Seq::new(a.spec_len(), |i: int| a.spec_index(i))
    }

    //		Section 7. proof fns/broadcast groups


    /// Bridge from the TotalOrder trait to vstd's total_ordering predicate.
    pub proof fn lemma_total_ordering<T: TotalOrder>()
        ensures total_ordering(spec_leq::<T>())
    {
        let leq = spec_leq::<T>();
        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(antisymmetric(leq)) by {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|x: T, y: T|
                #[trigger] leq(x, y) && #[trigger] leq(y, x) implies x == y by
            { T::antisymmetric(x, y); }
        };
        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(transitive(leq)) by {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|x: T, y: T, z: T|
                #[trigger] leq(x, y) && #[trigger] leq(y, z) implies leq(x, z) by
            { T::transitive(x, y, z); }
        };
        // Veracity: NEEDED assert
        // Veracity: NEEDED assert
        assert(strongly_connected(leq)) by {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
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
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(spec_const_seq(n, v) =~= spec_const_seq((n - 1) as nat, v).push(v));
        }
    }

    /// If all elements of a sequence equal v, then to_multiset().count(v) == len.
    proof fn lemma_all_equal_multiset<T>(s: Seq<T>, v: T)
        requires forall|i: int| 0 <= i < s.len() ==> (#[trigger] s[i]) == v,
        ensures
            s.to_multiset().count(v) == s.len(),
            forall|x: T| x != v ==> (#[trigger] s.to_multiset().count(x)) == 0nat,
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_equal_multiset::<T>(s.drop_last(), v);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(s =~= s.drop_last().push(v));
        }
    }

    //		Section 8. traits


    pub trait OrderStatSelectMtEphTrait<T: StTInMtT + TotalOrder> {
        /// Find the kth smallest element (0-indexed) using contraction-based selection.
        /// - Alg Analysis: APAS (Ch35 Alg 35.2): Work O(n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) expected, Span O(lg^2 n) expected — parallel D&C partition O(lg n) per round + parallel recursion
        fn select(a: &ArraySeqMtEphS<T>, k: usize) -> (found: Option<T>)
            requires
                a.spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                k >= a.spec_len() ==> found is None,
                k < a.spec_len() ==> found == Some(spec_kth::<T>(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), k as int));
    }

    //		Section 9. impls


    /// Append all elements of `b` onto the end of `a`.
    fn append_vec<T: Copy>(a: &mut Vec<T>, b: &Vec<T>)
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
                a@.subrange(0, alen as int) =~= a_orig,
                forall|k: int| 0 <= k < i as int ==>
                    #[trigger] a@[alen as int + k] == b@[k],
            decreases blen - i,
        {
            a.push(b[i]);
            i = i + 1;
        }
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
        proof {
            let ghost target = a_orig + b@;
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|k: int| 0 <= k < a@.len()
                implies a@[k] == #[trigger] target[k] by
            {
                if k < alen as int {
                } else {
                    let kp = k - alen as int;
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(a@[alen as int + kp] == b@[kp]);
                }
            };
        }
    }

    /// Parallel D&C three-way partition over a slice. Split into halves, partition each
    /// half in parallel via join, concatenate the three result Vecs.
    /// Work O(n), Span O(lg n) for the divide phase (plus O(n) sequential rejoin).
    /// - Alg Analysis: APAS (Ch35 Alg 35.2): Work O(n), Span O(lg n) — uses parallel filter.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel D&C partition via join, base case O(1), O(lg n) levels
    fn partition_three_dc<T: StTInMtT + TotalOrder + Copy>(
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
            spec_slice_elements(*a).to_multiset() =~=
                partitioned.0@.to_multiset().add(partitioned.2@.to_multiset()).add(partitioned.1@.to_multiset()),
            partitioned.0@.len() + partitioned.1@.len() + partitioned.2@.len() == a.spec_len(),
        decreases a.spec_len(),
    {
        let n = a.length();
        if n == 0 {
            (Vec::new(), Vec::new(), Vec::new())
        } else if n == 1 {
            let elem = a.nth_cloned(0);
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof {
            }
            match TotalOrder::cmp(&elem, pivot) {
                core::cmp::Ordering::Less => {
                    let mut v: Vec<T> = Vec::new();
                    // Veracity: NEEDED proof block
                    v.push(elem);
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(v@ =~= spec_slice_elements(*a));
                    }
                    (v, Vec::new(), Vec::new())
                }
                core::cmp::Ordering::Equal => {
                    // Veracity: NEEDED proof block
                    let mut v: Vec<T> = Vec::new();
                    v.push(elem);
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(v@ =~= spec_slice_elements(*a));
                    }
                    (Vec::new(), v, Vec::new())
                }
                // Veracity: NEEDED proof block
                core::cmp::Ordering::Greater => {
                    let mut v: Vec<T> = Vec::new();
                    v.push(elem);
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(v@ =~= spec_slice_elements(*a));
                    }
                    (Vec::new(), Vec::new(), v)
                }
            }
        } else {
            let mid = n / 2;
            let left_half = a.slice(0, mid);
            let right_half = a.slice(mid, n - mid);
            let p1 = *pivot;
            let p2 = *pivot;

            let ghost pivot_val = *pivot;
            let ghost left_elems = spec_slice_elements(left_half);
            let ghost right_elems = spec_slice_elements(right_half);

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

            // Veracity: NEEDED proof block
            append_vec(&mut l1, &l2);
            append_vec(&mut e1, &e2);
            append_vec(&mut r1, &r2);

            // Veracity: NEEDED proof block
            proof {
                let ghost ea = spec_slice_elements(*a);
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|k: int| 0 <= k < ea.len()
                    implies ea[k] == #[trigger] (left_elems + right_elems)[k] by
                {
                    if k < left_elems.len() {
                    } else {
                        let kp = k - left_elems.len();
                    }
                };
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(ea =~= left_elems + right_elems);

                vstd::seq_lib::lemma_multiset_commutative(left_elems, right_elems);
                vstd::seq_lib::lemma_multiset_commutative(l1_pre, l2@);
                vstd::seq_lib::lemma_multiset_commutative(e1_pre, e2@);
                vstd::seq_lib::lemma_multiset_commutative(r1_pre, r2@);

            }

            (l1, e1, r1)
        }
    }

    /// Parallel three-way partition: splits array into (elements < pivot, eq_count, elements > pivot).
    /// Uses D&C parallel partition via join() for O(lg n) span divide phase.
    /// - Alg Analysis: APAS (Ch35 Alg 35.2): Work O(n), Span O(lg n) — uses parallel filter.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel D&C partition via join
    fn parallel_three_way_partition<T: StTInMtT + TotalOrder + Copy>(
        a: &ArraySeqMtEphS<T>, pivot: T, pivot_idx: usize, n: usize,
    ) -> (partition: (Vec<T>, usize, Vec<T>))
        requires
            n == a.spec_len(),
            n <= usize::MAX,
            n >= 2,
            pivot_idx < n,
            pivot == a.spec_index(pivot_idx as int),
            obeys_feq_clone::<T>(),
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

        // Copy array data into a Vec, wrap in slice type for O(1) splitting.
        let mut data: Vec<T> = Vec::new();
        let mut ci: usize = 0;
        while ci < n
            invariant
                0 <= ci <= n,
                n == a.spec_len(),
                n <= usize::MAX,
                s == Seq::new(n as nat, |j: int| a.spec_index(j)),
                data@.len() == ci,
                forall|j: int| #![trigger data@[j]]
                    0 <= j < ci ==> data@[j] == s[j],
            decreases n - ci,
        {
            let elem = *a.nth(ci);
            data.push(elem);
            ci = ci + 1;
        }
// Veracity: NEEDED proof block


        let ghost data_view = data@;
        let slice_a = ArraySeqMtEphSliceS::from_vec(data);

        // Veracity: NEEDED proof block
        proof {
            let ghost se = spec_slice_elements(slice_a);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|i: int| 0 <= i < s.len()
                implies se[i] == #[trigger] s[i] by
            {
            };
        // Veracity: NEEDED proof block
        }

        let (left, eq_vec, right) = partition_three_dc(&slice_a, &pivot);

        let eq_count = eq_vec.len();

        // Veracity: NEEDED proof block
        proof {
            let ghost se = spec_slice_elements(slice_a);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(se =~= s);

            // From partition_three_dc: se.to_multiset() =~= left@ + right@ + eq_vec@
            // Pivot is in s at index pivot_idx, so s.to_multiset().count(pivot) >= 1.
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(s[pivot_idx as int] == pivot);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(s.to_multiset().count(pivot) >= 1nat) by {
            };

            // All pivot elements go into eq_vec. Elements in left and right are != pivot.
            // So eq_vec.to_multiset().count(pivot) == s.to_multiset().count(pivot) >= 1.



            // left.len() + right.len() < n because eq_count >= 1.

            // Build eq_seq for the postcondition.
            let eq_seq = Seq::new(eq_count as nat, |i: int| pivot);
            lemma_const_seq_multiset::<T>(eq_count as nat, pivot);

            // eq_vec@ and eq_seq have the same multiset: all elements are pivot.
            lemma_all_equal_multiset::<T>(eq_vec@, pivot);

        }

        (left, eq_count, right)
    }

    impl<T: StTInMtT + TotalOrder + Copy> OrderStatSelectMtEphTrait<T> for ArraySeqMtEphS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) expected, Span O(lg^2 n) expected — delegates to select_inner; D&C parallel partition via join
        fn select(a: &ArraySeqMtEphS<T>, k: usize) -> (found: Option<T>)
        {
            let n = a.length();
            if k >= n {
                return None;
            }
            select_inner(a, k)
        }
    }

    /// Recursive core of contraction-based select.
    /// - Alg Analysis: APAS (Ch35 Alg 35.2): Work O(n) expected, Span O(lg^2 n) w.h.p.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) expected, Span O(lg^2 n) expected — D&C partition is O(lg n) span
    ///   per round; geometric shrinkage over O(lg n) expected rounds gives O(lg^2 n).
    fn select_inner<T: StTInMtT + TotalOrder + Copy>(
        a: &ArraySeqMtEphS<T>, k: usize,
    ) -> (found: Option<T>)
        requires
            a.spec_len() <= usize::MAX,
            0 <= k < a.spec_len(),
            obeys_feq_clone::<T>(),
        ensures
            found == Some(spec_kth::<T>(
                Seq::new(a.spec_len(), |i: int| a.spec_index(i)), k as int)),
        decreases a.spec_len(),
    // Veracity: NEEDED proof block
    {
        let n = a.length();
        let ghost s = Seq::new(n as nat, |i: int| a.spec_index(i));
        let ghost leq = spec_leq::<T>();

        if n == 1 {
            let elem = *a.nth(0);
            // Veracity: NEEDED proof block
            proof {
                lemma_total_ordering::<T>();
                s.lemma_sort_by_ensures(leq);
                vstd::seq_lib::lemma_sorted_unique(s, s.sort_by(leq), leq);
            }
            return Some(elem);
        }

        let pivot_idx = random_usize_range(0, n);
        // Veracity: NEEDED proof block
        let pivot = *a.nth(pivot_idx);

        let (left, eq_count, right) = parallel_three_way_partition(a, pivot, pivot_idx, n);
        let ghost equals_seq: Seq<T> = Seq::new(eq_count as nat, |i: int| pivot);

        // Veracity: NEEDED proof block
        let left_count = left.len();
        let right_count = right.len();

        // Veracity: NEEDED proof block
        proof { lemma_total_ordering::<T>(); }
        let ghost sorted_left = left@.sort_by(leq);
        let ghost sorted_right = right@.sort_by(leq);
        let ghost candidate = sorted_left + equals_seq + sorted_right;

        // Veracity: NEEDED proof block
        proof {
            left@.lemma_sort_by_ensures(leq);
            right@.lemma_sort_by_ensures(leq);
            s.lemma_sort_by_ensures(leq);


            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(s.sort_by(leq).to_multiset().len() == s.sort_by(leq).len());


            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|j: int| 0 <= j < sorted_left.len() implies
                T::le(#[trigger] sorted_left[j], pivot) && sorted_left[j] != pivot by
            {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(left@.to_multiset().count(sorted_left[j]) > 0);
                let idx = choose|idx: int|
                    0 <= idx < left@.len() && left@[idx] == sorted_left[j];
            };

            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall|j: int| 0 <= j < sorted_right.len() implies
                T::le(pivot, #[trigger] sorted_right[j]) && sorted_right[j] != pivot by
            {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(right@.to_multiset().count(sorted_right[j]) > 0);
                let idx = choose|idx: int|
                    0 <= idx < right@.len() && right@[idx] == sorted_right[j];
            };

            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(sorted_by(candidate, leq)) by {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|ai: int, bi: int|
                    0 <= ai < bi < candidate.len()
                    implies (#[trigger] leq(candidate[ai], candidate[bi])) by
                {
                    let ll = sorted_left.len();
                    let el = equals_seq.len();
                    if ai < ll && bi < ll {
                    } else if ai < ll && bi < ll + el {
                    } else if ai < ll && bi >= ll + el {
                        T::transitive(candidate[ai], pivot, candidate[bi]);
                    } else if ai >= ll && ai < ll + el && bi >= ll && bi < ll + el {
                        T::reflexive(pivot);
                    } else if ai >= ll && ai < ll + el && bi >= ll + el {
                    } else {
                    }
                };
            };

            vstd::seq_lib::lemma_multiset_commutative(sorted_left, equals_seq);
            vstd::seq_lib::lemma_multiset_commutative(
                sorted_left + equals_seq, sorted_right);
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(candidate.to_multiset() =~= s.to_multiset());

            vstd::seq_lib::lemma_sorted_unique(
                s.sort_by(leq), candidate, leq);
        }

        if k < left_count {
            // Veracity: NEEDED proof block
            let left_a = ArraySeqMtEphS { seq: left };
            // Veracity: NEEDED proof block
            proof {
                let left_a_view = Seq::new(
                    left_a.spec_len(), |j: int| left_a.spec_index(j));
                // Veracity: NEEDED assert
                // Veracity: NEEDED proof block
                // Veracity: NEEDED assert
                assert(left_a_view =~= left@);
            }
            select_inner(&left_a, k)
        } else if k < n - right_count {
            // Veracity: NEEDED proof block
            proof {
            }
            Some(pivot)
        } else {
            let right_a = ArraySeqMtEphS { seq: right };
            let new_k = k - (n - right_count);
            // Veracity: NEEDED proof block
            proof {
                let ll = sorted_left.len();
                let el = equals_seq.len();
                let right_a_view = Seq::new(
                    right_a.spec_len(), |j: int| right_a.spec_index(j));
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(right_a_view =~= right@);
            }
            select_inner(&right_a, new_k)
        }
    }

    } // verus!
} // mod
