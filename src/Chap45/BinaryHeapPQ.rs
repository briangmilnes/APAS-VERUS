//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 45: Priority Queue implementation using Binary Heap

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod BinaryHeapPQ {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use vstd::multiset::Multiset;
    use vstd::assert_multisets_equal;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power2::{pow2, lemma_pow2_pos, lemma_pow2_unfold};
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::logarithm::{log, lemma_log0, lemma_log_s, lemma_log_nonnegative};

    verus! 
{


        use crate::Types::Types::*;
        use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
        use crate::vstdplus::total_order::total_order::TotalOrder;
        use crate::vstdplus::pervasives_plus::pervasives_plus::vec_swap;
        use core::cmp::Ordering;
        #[cfg(verus_keep_ghost)]
        use crate::vstdplus::feq::feq::*;

    //		Section 3. broadcast use


        broadcast use {
            crate::vstdplus::feq::feq::group_feq_axioms,
            vstd::seq::group_seq_axioms,
            vstd::seq_lib::group_seq_properties,
            vstd::seq_lib::group_to_multiset_ensures,
            vstd::std_specs::vec::group_vec_axioms,
        };

    //		Section 4. type definitions


        #[verifier::reject_recursive_types(T)]
        pub struct BinaryHeapPQ<T: StT + Ord + TotalOrder> {
            pub elements: ArraySeqStPerS<T>,
        }

    //		Section 5. view impls


        impl<T: StT + Ord + TotalOrder> View for BinaryHeapPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

    //		Section 6. spec fns


        /// Almost-heap: heap property holds at every position except possibly i.
        spec fn spec_almost_exec_heap<T: StT + Ord + TotalOrder>(seq: Seq<T>, i: int) -> bool {
            forall|k: int| 0 <= k < seq.len() && k != i ==>
                #[trigger] BinaryHeapPQ::<T>::spec_exec_heap_inv_at(seq, k)
        }

    //		Section 7. proof fns/broadcast groups


        proof fn lemma_log2_bound(n: int, bits: nat)
            requires
                n >= 1,
                bits >= 1,
                n < vstd::arithmetic::power::pow(2, bits),
            ensures
                log(2, n) < bits as int,
            decreases n,
        {
            lemma_log_nonnegative(2, n);
            if n < 2 {
                lemma_log0(2, n);
            } else {
                lemma_log_s(2, n);
                reveal(vstd::arithmetic::power::pow);
                // Veracity: NEEDED assert
                assert(n / 2 < vstd::arithmetic::power::pow(2, (bits - 1) as nat));
                lemma_log2_bound(n / 2, (bits - 1) as nat);
            }
        }

        /// In a binary heap, the root element is <= all elements.
        proof fn lemma_heap_root_le_all<T: StT + Ord + TotalOrder>(seq: Seq<T>, i: int)
            requires
                seq.len() > 0,
                0 <= i < seq.len(),
                BinaryHeapPQ::<T>::spec_is_exec_heap(seq),
            ensures
                TotalOrder::le(seq[0], seq[i]),
            decreases i,
        {
            if i == 0 {
                T::reflexive(seq[0]);
            } else {
                let p = (i - 1) / 2;
                lemma_heap_root_le_all::<T>(seq, p);
                // Veracity: NEEDED assert
                assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(seq, p));
                let left = 2 * p + 1;
                let right = 2 * p + 2;
                if i == left {
                } else {
                }
                T::transitive(seq[0], seq[p], seq[i]);
            }
        }

        /// In a heap, parent(i) <= i for all i > 0.
        proof fn lemma_heap_parent_le<T: StT + Ord + TotalOrder>(seq: Seq<T>, i: int)
            requires
                seq.len() > 0,
                0 < i < seq.len(),
                BinaryHeapPQ::<T>::spec_is_exec_heap(seq),
            ensures
                TotalOrder::le(seq[(i - 1) / 2], seq[i]),
        {
            let p = (i - 1) / 2;
            // Veracity: NEEDED assert
            assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(seq, p));
            let left = 2 * p + 1;
            let right = 2 * p + 2;
            if i == left {
            } else {
            }
        }

        proof fn lemma_swap_preserves_multiset<A>(s: Seq<A>, i: int, j: int)
            requires 0 <= i < s.len(), 0 <= j < s.len(),
            ensures s.update(i, s[j]).update(j, s[i]).to_multiset() =~= s.to_multiset(),
        {
            let s1 = s.update(i, s[j]);
            let s2 = s1.update(j, s[i]);
            vstd::seq_lib::to_multiset_update(s, i, s[j]);
            // s1.to_multiset() == s.to_multiset().insert(s[j]).remove(s[i])
            vstd::seq_lib::to_multiset_update(s1, j, s[i]);
            // s2.to_multiset() == s1.to_multiset().insert(s[i]).remove(s1[j])
            //                   == s.to_multiset().insert(s[j]).remove(s[i]).insert(s[i]).remove(s[j])
            let m = s.to_multiset();
            let a = s[i];
            let b = s[j];
            // Need: m.insert(b).remove(a).insert(a).remove(b) == m
            vstd::seq_lib::to_multiset_contains(s, s[i]);
            assert_multisets_equal!(m.insert(b).remove(a).insert(a).remove(b), m, key => {
                vstd::multiset::lemma_insert_increases_count_by_1(m, b);
                vstd::multiset::lemma_insert_other_elements_unchanged(m, b, key);
                vstd::multiset::lemma_insert_increases_count_by_1(m.insert(b).remove(a), a);
                vstd::multiset::lemma_insert_other_elements_unchanged(m.insert(b).remove(a), a, key);
            });
        }

        /// Transfers le(min, a[j]) for all j to le(min, b[i]) for all i
        /// when a and b have equal multisets.
        proof fn lemma_le_preserved_by_multiset_eq<T: TotalOrder>(
            min_elem: T, a: Seq<T>, b: Seq<T>,
        )
            requires
                a.to_multiset() =~= b.to_multiset(),
                forall|j: int| 0 <= j < a.len() ==>
                    #[trigger] TotalOrder::le(min_elem, a[j]),
            ensures
                forall|i: int| 0 <= i < b.len() ==>
                    #[trigger] TotalOrder::le(min_elem, b[i]),
        {
            // Veracity: NEEDED assert
            assert forall|i: int| 0 <= i < b.len()
                implies #[trigger] TotalOrder::le(min_elem, b[i]) by {
                // b[i] appears in b, so it's in b's multiset.
                vstd::seq_lib::to_multiset_contains(b, b[i]);
                // Same multiset, so b[i] is in a's multiset, hence in a.
                vstd::seq_lib::to_multiset_contains(a, b[i]);
            }
        }

    //		Section 8. traits


        /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
        pub trait BinaryHeapPQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Seq<T::V>> {
            spec fn spec_binaryheappq_wf(&self) -> bool;
            spec fn spec_size(self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;
            spec fn spec_sorted(s: Seq<T>) -> bool;
            spec fn spec_heap_inv_at(seq: Seq<T::V>, i: int) -> bool;
            spec fn spec_leq_view(a: T::V, b: T::V) -> bool;
            spec fn spec_is_heap(seq: Seq<T::V>) -> bool;
            spec fn spec_exec_heap_inv_at(seq: Seq<T>, i: int) -> bool;
            spec fn spec_is_exec_heap(seq: Seq<T>) -> bool;
            spec fn spec_is_exec_heap_except(seq: Seq<T>, pos: int) -> bool;
            spec fn parent_spec(i: int) -> int;
            spec fn left_child_spec(i: int) -> int;
            spec fn right_child_spec(i: int) -> int;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn empty() -> (pq: Self)
                ensures
                    pq@.len() == 0,
                    pq@.to_multiset() =~= Multiset::empty(),
                    pq.spec_binaryheappq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn singleton(element: T) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq@.len() == 1,
                    pq@.to_multiset() =~= Multiset::empty().insert(element@),
                    pq.spec_binaryheappq_wf(),
                    Self::spec_is_exec_heap(pq.spec_seq());

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn find_min(&self) -> (min_elem: Option<&T>)
                requires
                    Self::spec_is_exec_heap(self.spec_seq()),
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some(),
                    self@.len() > 0 ==> self@.to_multiset().count(min_elem.unwrap()@) > 0,
                    self@.len() > 0 ==> forall|i: int| 0 <= i < self.spec_seq().len() ==>
                        #[trigger] TotalOrder::le(*min_elem.unwrap(), self.spec_seq()[i]);

            /// - Alg Analysis: APAS (Ch45 cost table, binary heaps): Work O(lg n), Span O(lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n): sift up
            fn insert(&self, element: T) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + 1 <= usize::MAX as int,
                    Self::spec_is_exec_heap(self.spec_seq()),
                ensures
                    pq@.len() == self@.len() + 1,
                    pq@.to_multiset() =~= self@.to_multiset().insert(element@),
                    Self::spec_is_exec_heap(pq.spec_seq());

            /// - Alg Analysis: APAS (Ch45 cost table, binary heaps): Work O(lg n), Span O(lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n): sift down
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int,
                    Self::spec_is_exec_heap(self.spec_seq()),
                ensures
                    self@.len() > 0 ==> min_and_rest.1.is_some(),
                    self@.len() > 0 ==> min_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> min_and_rest.1.is_none(),
                    self@.len() == 0 ==> min_and_rest.0@.len() == 0,
                    self@.len() > 0 ==> self@.to_multiset() =~=
                        min_and_rest.0@.to_multiset().insert(min_and_rest.1.unwrap()@),
                    self@.len() > 0 ==> Self::spec_is_exec_heap(min_and_rest.0.spec_seq()),
                    self@.len() > 0 ==> min_and_rest.1.unwrap() == self.spec_seq()[0],
                    self@.len() > 0 ==> (forall|i: int|
                        0 <= i < min_and_rest.0.spec_seq().len() ==>
                        #[trigger] TotalOrder::le(min_and_rest.1.unwrap(),
                            min_and_rest.0.spec_seq()[i]));

            /// - Alg Analysis: APAS (Ch45 cost table, binary heaps): Work O(m + n), Span O(m + n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m + n), Span O(m + n): rebuild heap from combined arrays
            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + other@.len() <= usize::MAX as int,
                    (self@.len() + other@.len()) * 2 <= usize::MAX as int,
                ensures
                    pq@.len() == self@.len() + other@.len(),
                    pq@.to_multiset() =~= self@.to_multiset().add(other@.to_multiset());

            /// - Alg Analysis: APAS (Ch45 cost table, binary heaps): Work O(n), Span O(n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) Work: bottom-up heapify
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    seq@.len() * 2 <= usize::MAX as int,
                ensures
                    pq@.len() == seq@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn size(&self) -> (n: usize)
                ensures n as int == self.spec_size();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> (empty: bool)
                ensures empty == (self.spec_size() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
                requires obeys_feq_clone::<T>(),
                ensures seq@ =~= self@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m + n), Span O(m + n)
            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + elements@.len() <= usize::MAX as int,
                    (self@.len() + elements@.len()) * 2 <= usize::MAX as int,
                ensures pq@.len() == self@.len() + elements@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int,
                    Self::spec_is_exec_heap(self.spec_seq()),
                ensures
                    sorted@.len() == self@.len(),
                    Self::spec_sorted(sorted.seq@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_valid_heap(&self) -> (valid: bool)
                requires self@.len() * 2 <= usize::MAX as int;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn height(&self) -> (levels: usize)
                requires self@.len() <= usize::MAX as int,
                ensures self@.len() == 0 ==> levels == 0;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(2^level), Span O(2^level)
            fn level_elements(&self, level: usize) -> (elts: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    level < 63,
                    usize::BITS >= 64;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    vec@.len() * 2 <= usize::MAX as int,
                ensures pq@.len() == vec@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (v: Vec<T>)
                requires obeys_feq_clone::<T>(),
                ensures v@.len() == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn to_sorted_vec(&self) -> (v: Vec<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int,
                    Self::spec_is_exec_heap(self.spec_seq()),
                ensures
                    v@.len() == self@.len(),
                    Self::spec_sorted(v@);
        }

    //		Section 9. impls


        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn left_child(i: usize) -> (child_idx: usize)
            requires i <= usize::MAX / 2 - 1,
            ensures child_idx as int == 2 * (i as int) + 1,
        {
            2 * i + 1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn right_child(i: usize) -> (child_idx: usize)
            requires i <= usize::MAX / 2 - 1,
            ensures child_idx as int == 2 * (i as int) + 2,
        {
            2 * i + 2
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn parent(i: usize) -> (parent_idx: usize)
            requires i > 0,
            ensures parent_idx as int == (i as int - 1) / 2,
        {
            (i - 1) / 2
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn swap_elements<T: StT + Ord + TotalOrder>(seq: &ArraySeqStPerS<T>, i: usize, j: usize) -> (swapped: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                (i as int) < seq.view().len(),
                (j as int) < seq.view().len(),
                seq@.len() <= usize::MAX as int,
            ensures
                swapped@.len() == seq@.len(),
                swapped@.to_multiset() =~= seq@.to_multiset(),
        {
            let n = seq.length();
            let mut result = ArraySeqStPerS::empty();
            let ghost sv = seq@.update(i as int, seq@[j as int]).update(j as int, seq@[i as int]);

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for k in 0..n
                invariant
                    n == seq@.len(),
                    result@.len() == k as int,
                    (i as int) < n,
                    (j as int) < n,
                    obeys_feq_clone::<T>(),
                    sv == seq@.update(i as int, seq@[j as int]).update(j as int, seq@[i as int]),
                    forall|m: int| 0 <= m < k as int ==> #[trigger] result@[m] == sv[m],
            {
                let element = if k == i {
                    seq.nth(j).clone()
                } else if k == j {
                    seq.nth(i).clone()
                } else {
                    seq.nth(k).clone()
                };

                let single_seq = ArraySeqStPerS::singleton(element);
                let ghost pre_seq = result.seq@;
                let ghost pre_view = result@;
                result = ArraySeqStPerS::append(&result, &single_seq);

                // Veracity: NEEDED proof block
                proof {
                    let pos: int = if k == i { j as int } else if k == j { i as int } else { k as int };
                    axiom_cloned_implies_eq_owned(seq.spec_index(pos), element);
                    // New element at position k.
                    // Previous elements preserved by append.
                    // Veracity: NEEDED assert
                    assert forall|m: int| 0 <= m < k as int
                        implies #[trigger] result@[m] == sv[m] by {
                        // Veracity: NEEDED assert
                        assert(result.spec_index(m)@ == result@[m]);
                        // Veracity: NEEDED assert
                        assert(pre_view[m] == sv[m]);
                    }
                }
            }

            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(result@ =~= sv);
                lemma_swap_preserves_multiset(seq@, i as int, j as int);
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn bubble_up<T: StT + Ord + TotalOrder>(seq: &ArraySeqStPerS<T>, mut i: usize) -> (heaped: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                (i as int) < seq.view().len(),
                seq@.len() <= usize::MAX as int,
            ensures
                heaped@.len() == seq@.len(),
                heaped@.to_multiset() =~= seq@.to_multiset(),
        {
            let mut result = seq.clone();

            // Veracity: NEEDED proof block
            proof {
                // Clone bridge: cloned elements are T-level equal, so views match.
                // Veracity: NEEDED assert
                assert(result@ =~= seq@);
            }

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i > 0
                invariant
                    result@.len() == seq@.len(),
                    result@.len() <= usize::MAX as int,
                    (i as int) < seq.view().len(),
                    BinaryHeapPQ::<T>::parent_spec(i as int) < seq.view().len(),
                    result@.to_multiset() =~= seq@.to_multiset(),
                    obeys_feq_clone::<T>(),
                decreases i,
            {
                let parent_idx = parent(i);
                // Veracity: NEEDED proof block
                proof {
                }
                let current = result.nth(i);
                let parent_val = result.nth(parent_idx);

                if *current >= *parent_val {
                    i = 0;
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                    }
                    result = swap_elements(&result, i, parent_idx);
                    i = parent_idx;
                }
            }

            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn bubble_down<T: StT + Ord + TotalOrder>(heap: &ArraySeqStPerS<T>, i: usize) -> (heaped: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                (i as int) < heap.view().len(),
                heap@.len() <= usize::MAX as int,
                heap@.len() * 2 <= usize::MAX as int,
            ensures
                heaped@.len() == heap@.len(),
                heaped@.to_multiset() =~= heap@.to_multiset(),
        {
            let mut result = heap.clone();
            let n = result.length();
            let mut idx = i;

            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(result@ =~= heap@);
            }

            let mut done = false;
            let ghost mut old_idx: int = idx as int;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while !done
                invariant
                    result@.len() == heap@.len(),
                    result@.len() == n,
                    (idx as int) < n,
                    n <= usize::MAX as int,
                    n * 2 <= usize::MAX as int,
                    obeys_feq_clone::<T>(),
                    !done ==> old_idx == idx as int,
                    result@.to_multiset() =~= heap@.to_multiset(),
                decreases (if !done { 1int } else { 0int }), n - idx,
            {
                let left = left_child(idx);
                let right = right_child(idx);
                let mut smallest = idx;

                if left < n && *result.nth(left) < *result.nth(smallest) {
                    smallest = left;
                }

                if right < n && *result.nth(right) < *result.nth(smallest) {
                    smallest = right;
                }

                if smallest == idx {
                    done = true;
                } else {
                    result = swap_elements(&result, idx, smallest);
                    idx = smallest;
                }
                // Veracity: NEEDED proof block
                proof { old_idx = idx as int; }
            }

            result
        }

        /// Sifts down from position 0, restoring full heap property.
        /// Takes ownership to enable in-place vec_swap for T-level identity.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn bubble_down_heap<T: StT + Ord + TotalOrder>(heap: ArraySeqStPerS<T>) -> (heaped: ArraySeqStPerS<T>)
            requires
                heap.seq@.len() > 0,
                heap.seq@.len() * 2 <= usize::MAX as int,
                BinaryHeapPQ::<T>::spec_is_exec_heap_except(heap.seq@, 0),
            ensures
                heaped.seq@.len() == heap.seq@.len(),
                BinaryHeapPQ::<T>::spec_is_exec_heap(heaped.seq@),
                heaped@.to_multiset() =~= heap@.to_multiset(),
                heaped.seq@.to_multiset() =~= heap.seq@.to_multiset(),
        {
            let ghost orig = heap.seq@;
            let ghost orig_view = heap@;
            let mut v = heap.seq;
            let n = v.len();
            if n <= 1 {
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqStPerS { seq: v };
            }
            let mut idx: usize = 0;
            let mut done = false;
            let ghost mut view_seq: Seq<T::V> = orig_view;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while !done
                invariant
                    v@.len() == n,
                    n > 1,
                    (n as int) * 2 <= usize::MAX as int,
                    (idx as int) < (n as int),
                    BinaryHeapPQ::<T>::spec_is_exec_heap_except(v@, idx as int),
                    idx > 0 ==> (
                        (2 * (idx as int) + 1 < n as int ==>
                            TotalOrder::le(v@[((idx as int) - 1) / 2], v@[2 * (idx as int) + 1]))
                        && (2 * (idx as int) + 2 < n as int ==>
                            TotalOrder::le(v@[((idx as int) - 1) / 2], v@[2 * (idx as int) + 2]))
                    ),
                    v@.to_multiset() =~= orig.to_multiset(),
                    view_seq.len() == n as int,
                    view_seq.to_multiset() =~= orig_view.to_multiset(),
                    forall|k: int| 0 <= k < n as int ==> #[trigger] view_seq[k] == v@[k]@,
                    done ==> BinaryHeapPQ::<T>::spec_is_exec_heap(v@),
                decreases (if !done { 1int } else { 0int }), n - idx,
            {
                let left = 2 * idx + 1;
                if left >= n {
                    // Leaf: heap_inv_at trivially holds.
                    done = true;
                } else {
                    let right = left + 1;
                    let ghost pre = v@;
                    let mut smallest = idx;

                    // Compare left child with current using TotalOrder::cmp.
                    let c1 = TotalOrder::cmp(&v[left], &v[idx]);
                    match c1 {
                        Ordering::Less => { smallest = left; }
                        Ordering::Equal => { proof { T::reflexive(pre[idx as int]); } }
                        _ => { }
                    }

                    // Compare right child with current smallest.
                    if right < n {
                        let ghost sbc = smallest as int;
                        let c2 = TotalOrder::cmp(&v[right], &v[smallest]);
                        match c2 {
                            Ordering::Less => {
                                // Veracity: NEEDED proof block
                                proof {
                                    // c2 gives le(right, pre[sbc]).
                                    // Establish le(right, idx) and le(right, left) while
                                    // branch context connects sbc to c1 outcome.
                                    if sbc == left as int {
                                        // c1 was Less: le(right, left) from c2.
                                        T::transitive(pre[right as int], pre[left as int], pre[idx as int]);
                                    } else {
                                        // sbc == idx. le(right, idx) from c2.
                                        // c1 not Less: Equal gives pre[left]==pre[idx],
                                        // Greater gives le(idx, left). Either way le(idx, left).
                                        T::reflexive(pre[idx as int]);
                                        T::total(pre[idx as int], pre[left as int]);
                                        if TotalOrder::le(pre[idx as int], pre[left as int]) {
                                            T::transitive(pre[right as int], pre[idx as int], pre[left as int]);
                                        }
                                    }
                                    T::reflexive(pre[right as int]);
                                }
                                smallest = right;
                            }
                            Ordering::Equal => { proof { T::reflexive(pre[smallest as int]); } }
                            _ => { }
                        }
                    }

                    if smallest == idx {
                        done = true;
                    } else {
                        // Veracity: NEEDED proof block
                        proof {
                            T::reflexive(pre[smallest as int]);
                            if right < n {
                            }
                        }

                        let ghost pre_view = view_seq;
                        vec_swap(&mut v, idx, smallest);

                        // Veracity: NEEDED proof block
                        proof {
                            let id = idx as int;
                            let sc = smallest as int;
                            // Veracity: NEEDED assert
                            assert(v@ =~= pre.update(id, pre[sc]).update(sc, pre[id]));
                            lemma_swap_preserves_multiset(pre, id, sc);

                            // View tracking.
                            view_seq = pre_view.update(id, pre_view[sc]).update(sc, pre_view[id]);
                            lemma_swap_preserves_multiset(pre_view, id, sc);
                            // Veracity: NEEDED assert
                            assert forall|k: int| 0 <= k < n as int
                                implies #[trigger] view_seq[k] == v@[k]@ by {
                                if k == id {
                                } else if k == sc {
                                }
                            }

                            // C2 at new position sc: parent(sc) = id.
                            // Veracity: NEEDED assert
                            assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(pre, sc));

                            // Heap-except at sc.
                            // Veracity: NEEDED assert
                            assert forall|j: int| 0 <= j < n as int && j != sc
                                implies #[trigger] BinaryHeapPQ::<T>::spec_exec_heap_inv_at(v@, j) by {
                                if j == id {
                                    // v@[id] = pre[sc]. For each child c of id:
                                    //   c == sc → v@[c] = pre[id], le(pre[sc], pre[id]) ✓
                                    //   c != sc → v@[c] = pre[c], le(pre[sc], pre[c]) ✓
                                } else if id > 0 && j == (id - 1) / 2 {
                                    // Parent of id. v@[j] unchanged.
                                    // Child at id: v@[id] = pre[sc], le(pre[j], pre[sc]) from C2.
                                    // Sibling child: unchanged, from old heap.
                                    // Veracity: NEEDED assert
                                    assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(pre, j));
                                } else {
                                    // Positions j, 2j+1, 2j+2 are all ∉ {id, sc}.
                                    // Veracity: NEEDED assert
                                    assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(pre, j));
                                }
                            };
                        }
                        idx = smallest;
                    }
                }
            }

            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(ArraySeqStPerS::<T> { seq: v }.view() =~= view_seq) by {
                }
            }
            ArraySeqStPerS { seq: v }
        }

        /// Sifts up from position i, restoring full heap property.
        /// Takes ownership to enable in-place vec_swap for T-level identity.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn bubble_up_heap<T: StT + Ord + TotalOrder>(heap: ArraySeqStPerS<T>, i: usize) -> (heaped: ArraySeqStPerS<T>)
            requires
                heap.seq@.len() > 0,
                (i as int) < heap.seq@.len(),
                i == 0 ==> BinaryHeapPQ::<T>::spec_is_exec_heap(heap.seq@),
                i > 0 ==> BinaryHeapPQ::<T>::spec_is_exec_heap_except(heap.seq@, (i as int - 1) / 2),
                // Parent satisfies heap_inv for all children of parent(i) except child i.
                i > 0 ==> (
                    (2 * ((i as int - 1) / 2) + 1 < heap.seq@.len() && 2 * ((i as int - 1) / 2) + 1 != i as int
                        ==> TotalOrder::le(heap.seq@[(i as int - 1) / 2], heap.seq@[2 * ((i as int - 1) / 2) + 1]))
                    && (2 * ((i as int - 1) / 2) + 2 < heap.seq@.len() && 2 * ((i as int - 1) / 2) + 2 != i as int
                        ==> TotalOrder::le(heap.seq@[(i as int - 1) / 2], heap.seq@[2 * ((i as int - 1) / 2) + 2]))
                ),
                // Parent of i is <= all children of i.
                i > 0 ==> (
                    (2 * (i as int) + 1 < heap.seq@.len()
                        ==> TotalOrder::le(heap.seq@[(i as int - 1) / 2], heap.seq@[2 * (i as int) + 1]))
                    && (2 * (i as int) + 2 < heap.seq@.len()
                        ==> TotalOrder::le(heap.seq@[(i as int - 1) / 2], heap.seq@[2 * (i as int) + 2]))
                ),
            ensures
                heaped.seq@.len() == heap.seq@.len(),
                BinaryHeapPQ::<T>::spec_is_exec_heap(heaped.seq@),
                heaped@.to_multiset() =~= heap@.to_multiset(),
                heaped.seq@.to_multiset() =~= heap.seq@.to_multiset(),
        {
            let ghost orig = heap.seq@;
            let ghost orig_view = heap@;
            let mut v = heap.seq;
            let n = v.len();
            if i == 0 {
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqStPerS { seq: v };
            }
            let mut i: usize = i;
            let mut done = false;
            let ghost mut view_seq: Seq<T::V> = orig_view;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while !done
                invariant
                    v@.len() == n,
                    n > 0,
                    (i as int) < (n as int),
                    !done ==> i > 0,
                    !done ==> BinaryHeapPQ::<T>::spec_is_exec_heap_except(v@, (i as int - 1) / 2),
                    !done ==> (
                        (2 * ((i as int - 1) / 2) + 1 < n as int && 2 * ((i as int - 1) / 2) + 1 != i as int
                            ==> TotalOrder::le(v@[(i as int - 1) / 2], v@[2 * ((i as int - 1) / 2) + 1]))
                        && (2 * ((i as int - 1) / 2) + 2 < n as int && 2 * ((i as int - 1) / 2) + 2 != i as int
                            ==> TotalOrder::le(v@[(i as int - 1) / 2], v@[2 * ((i as int - 1) / 2) + 2]))
                    ),
                    !done ==> (
                        (2 * (i as int) + 1 < n as int
                            ==> TotalOrder::le(v@[(i as int - 1) / 2], v@[2 * (i as int) + 1]))
                        && (2 * (i as int) + 2 < n as int
                            ==> TotalOrder::le(v@[(i as int - 1) / 2], v@[2 * (i as int) + 2]))
                    ),
                    done ==> BinaryHeapPQ::<T>::spec_is_exec_heap(v@),
                    v@.to_multiset() =~= orig.to_multiset(),
                    view_seq.len() == n as int,
                    view_seq.to_multiset() =~= orig_view.to_multiset(),
                    forall|k: int| 0 <= k < n as int ==> #[trigger] view_seq[k] == v@[k]@,
                decreases (if !done { 1int } else { 0int }), i,
            {
                let p = parent(i);
                let ghost pre = v@;
                let ghost ii = i as int;
                let ghost pp = p as int;

                let cmp = TotalOrder::cmp(&v[i], &v[p]);
                match cmp {
                    Ordering::Less => {
                        // v[i] < v[p]: swap and continue sifting up.
                        let ghost pre_view = view_seq;
                        vec_swap(&mut v, i, p);

                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert(v@ =~= pre.update(ii, pre[pp]).update(pp, pre[ii]));
                            lemma_swap_preserves_multiset(pre, ii, pp);
                            view_seq = pre_view.update(ii, pre_view[pp]).update(pp, pre_view[ii]);
                            lemma_swap_preserves_multiset(pre_view, ii, pp);
                            // Veracity: NEEDED assert
                            assert forall|k: int| 0 <= k < n as int
                                implies #[trigger] view_seq[k] == v@[k]@ by {
                                if k == ii {
                                } else if k == pp {
                                }
                            }

                            // heap_inv_at(v@, pp): v@[pp] = pre[ii], children are ii and sibling.
                            // le(pre[ii], pre[pp]) from cmp. Sibling: transitivity.
                            if 2 * pp + 1 < n as int && 2 * pp + 1 != ii {
                                T::transitive(pre[ii], pre[pp], pre[2 * pp + 1]);
                            }
                            if 2 * pp + 2 < n as int && 2 * pp + 2 != ii {
                                T::transitive(pre[ii], pre[pp], pre[2 * pp + 2]);
                            }

                            // heap_inv_at(v@, ii): v@[ii] = pre[pp], children unchanged.
                            // From children invariant: le(pre[pp], pre[child]).
                        }

                        if p == 0 {
                            // Veracity: NEEDED proof block
                            proof {
                                // Veracity: NEEDED assert
                                assert forall|j: int| 0 <= j < n as int
                                    implies #[trigger] BinaryHeapPQ::<T>::spec_exec_heap_inv_at(v@, j) by {
                                    if j == pp || j == ii {
                                    } else {
                                        // Veracity: NEEDED assert
                                        assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(pre, j));
                                    }
                                };
                            }
                            done = true;
                        } else {
                            // Veracity: NEEDED proof block
                            proof {
                                let gp = (pp - 1) / 2;
                                // spec_is_exec_heap_except(v@, gp)
                                // Veracity: NEEDED assert
                                assert forall|j: int| 0 <= j < n as int && j != gp
                                    implies #[trigger] BinaryHeapPQ::<T>::spec_exec_heap_inv_at(v@, j) by {
                                    if j == pp {
                                    } else if j == ii {
                                    } else {
                                        // Veracity: NEEDED assert
                                        assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(pre, j));
                                    }
                                };

                                // New sibling condition: le(v@[gp], v@[sibling_of_pp]).
                                // gp and sibling unchanged. From heap_inv_at(pre, gp).
                                // Veracity: NEEDED assert
                                assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(pre, gp));

                                // New children condition: le(v@[gp], v@[child_of_pp]).
                                // Children of pp: ii (= pre[pp]) and sibling_of_ii.
                                // le(v@[gp], v@[ii]) = le(pre[gp], pre[pp]) from heap_inv_at(pre, gp).
                                // le(v@[gp], v@[sib]) = le(pre[gp], pre[sib]):
                                //   from le(pre[gp], pre[pp]) + le(pre[pp], pre[sib]) by transitivity.
                                if 2 * pp + 1 < n as int && 2 * pp + 1 != ii {
                                    T::transitive(pre[gp], pre[pp], pre[2 * pp + 1]);
                                }
                                if 2 * pp + 2 < n as int && 2 * pp + 2 != ii {
                                    T::transitive(pre[gp], pre[pp], pre[2 * pp + 2]);
                                }
                            }
                            i = p;
                        }
                    }
                    _ => {
                        // v[i] >= v[p]: heap_inv_at(parent(i)) now holds, completing the heap.
                        // Veracity: NEEDED proof block
                        proof {
                            match cmp {
                                Ordering::Equal => { T::reflexive(pre[pp]); }
                                _ => {}
                            }
                        }
                        done = true;
                    }
                }
            }

            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(ArraySeqStPerS::<T> { seq: v }.view() =~= view_seq) by {
                }
            }
            ArraySeqStPerS { seq: v }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn heapify<T: StT + Ord + TotalOrder>(seq: &ArraySeqStPerS<T>) -> (heap: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                seq@.len() <= usize::MAX as int,
                seq@.len() * 2 <= usize::MAX as int,
            ensures
                heap@.len() == seq@.len(),
                heap@.to_multiset() =~= seq@.to_multiset(),
        {
            if seq.length() <= 1 {
                let r = seq.clone();
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(r@ =~= seq@);
                }
                return r;
            }

            let mut result = seq.clone();

            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(result@ =~= seq@);
            }

            let last_non_leaf = if seq.length() >= 2 { (seq.length() - 2) / 2 } else { 0 };

            let mut idx = last_non_leaf + 1;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while idx > 0
                invariant
                    result@.len() == seq@.len(),
                    (idx as int) <= seq@.len(),
                    result@.len() <= usize::MAX as int,
                    result@.len() * 2 <= usize::MAX as int,
                    obeys_feq_clone::<T>(),
                    result@.to_multiset() =~= seq@.to_multiset(),
                decreases idx,
            {
                idx = idx - 1;
                result = bubble_down(&result, idx);
            }

            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn is_heap<T: StT + Ord + TotalOrder>(elements: &ArraySeqStPerS<T>) -> (valid: bool)
            requires elements@.len() * 2 <= usize::MAX as int,
            ensures true,
        {
            let n = elements.length();
            let mut valid = true;
            for i in 0..n
                invariant
                    n == elements@.len(),
                    (i as int) <= n,
                    (i as int) < n || i == n,
            {
                if i <= usize::MAX / 2 - 1 {
                let left = left_child(i);
                let right = right_child(i);

                if left < n && *elements.nth(i) > *elements.nth(left) {
                    valid = false;
                }

                if right < n && *elements.nth(i) > *elements.nth(right) {
                    valid = false;
                }
                }
            }
            valid
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(e), Span O(e)
        fn exec_pow2(e: usize) -> (power: usize)
            requires pow2(e as nat) <= usize::MAX as int,
            ensures power as int == pow2(e as nat),
        {
            // Veracity: NEEDED proof block
            proof {
                lemma_pow2_pos(e as nat);
                vstd::arithmetic::power::lemma_pow0(2);
            }
            let mut power: usize = 1;
            for i in 0..e
                invariant
                    power as int == pow2(i as nat),
                    pow2(i as nat) <= pow2(e as nat),
                    pow2(e as nat) <= usize::MAX as int,
            {
                // Veracity: NEEDED proof block
                proof {
                    lemma_pow2_unfold((i + 1) as nat);
                    lemma_pow2_pos(i as nat);
                    if (i + 1) < e {
                        vstd::arithmetic::power2::lemma_pow2_strictly_increases((i + 1) as nat, e as nat);
                    }
                }
                power = power * 2;
            }
            power
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn exec_log2(n: usize) -> (log_val: usize)
            requires n >= 1,
            ensures log_val as int == log(2, n as int),
            decreases n,
        {
            if n < 2 {
                // Veracity: NEEDED proof block
                proof { lemma_log0(2, n as int); }
                0
            } else {
                // Veracity: NEEDED proof block
                proof { lemma_log_s(2, n as int); }
                let rest = exec_log2(n / 2);
                // Veracity: NEEDED proof block
                proof {
                    lemma_log_nonnegative(2, (n / 2) as int);
                    vstd::layout::unsigned_int_max_values();
                    vstd::arithmetic::power2::lemma_pow2(usize::BITS as nat);
                    lemma_log2_bound(n as int, usize::BITS as nat);
                }
                rest + 1
            }
        }

        impl<T: StT + Ord + TotalOrder> BinaryHeapPQTrait<T> for BinaryHeapPQ<T> {
            open spec fn spec_binaryheappq_wf(&self) -> bool {
                self@.len() * 2 <= usize::MAX as int
            }

            open spec fn spec_size(self) -> nat {
                self@.len()
            }

            open spec fn spec_seq(&self) -> Seq<T> {
                self.elements.seq@
            }

            open spec fn spec_sorted(s: Seq<T>) -> bool {
                forall|i: int, j: int| 0 <= i < j < s.len() ==>
                    #[trigger] TotalOrder::le(s[i], s[j])
            }

            open spec fn spec_heap_inv_at(seq: Seq<T::V>, i: int) -> bool {
                let left = 2 * i + 1;
                let right = 2 * i + 2;
                (left >= seq.len() || Self::spec_leq_view(seq[i], seq[left]))
                && (right >= seq.len() || Self::spec_leq_view(seq[i], seq[right]))
            }

            uninterp spec fn spec_leq_view(a: T::V, b: T::V) -> bool;

            open spec fn spec_is_heap(seq: Seq<T::V>) -> bool {
                forall|i: int| 0 <= i < seq.len() ==> Self::spec_heap_inv_at(seq, i)
            }

            open spec fn spec_exec_heap_inv_at(seq: Seq<T>, i: int) -> bool {
                let left = 2 * i + 1;
                let right = 2 * i + 2;
                (left >= seq.len() || TotalOrder::le(seq[i], seq[left]))
                && (right >= seq.len() || TotalOrder::le(seq[i], seq[right]))
            }

            open spec fn spec_is_exec_heap(seq: Seq<T>) -> bool {
                forall|i: int| 0 <= i < seq.len() ==>
                    #[trigger] Self::spec_exec_heap_inv_at(seq, i)
            }

            open spec fn spec_is_exec_heap_except(seq: Seq<T>, pos: int) -> bool {
                forall|i: int| 0 <= i < seq.len() && i != pos ==>
                    #[trigger] Self::spec_exec_heap_inv_at(seq, i)
            }

            open spec fn parent_spec(i: int) -> int {
                if i == 0 { 0 } else { (i - 1) / 2 }
            }

            open spec fn left_child_spec(i: int) -> int {
                2 * i + 1
            }

            open spec fn right_child_spec(i: int) -> int {
                2 * i + 2
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); constant-time empty construction.
            fn empty() -> (pq: Self) {
                let pq = BinaryHeapPQ {
                    elements: ArraySeqStPerS::empty(),
                };
                // Veracity: NEEDED proof block
                proof {
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); constant-time singleton construction.
            fn singleton(element: T) -> (pq: Self) {
                let pq = BinaryHeapPQ {
                    elements: ArraySeqStPerS::singleton(element),
                };
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(pq@ =~= Seq::<T::V>::empty().push(element@));
                    // Heap property: 1-element sequence is trivially a heap.
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); root of heap is minimum.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    None
                } else {
                    let root = self.elements.nth(0);
                    // Veracity: NEEDED proof block
                    proof {
                        // Root is in the multiset.
                        // Veracity: NEEDED assert
                        assert(self@[0] == root@);
                        vstd::seq_lib::to_multiset_contains(self@, root@);
                        // Root <= all elements via heap ordering.
                        // Veracity: NEEDED assert
                        assert forall|i: int| 0 <= i < self.spec_seq().len()
                            implies #[trigger] TotalOrder::le(*root, self.spec_seq()[i]) by {
                            lemma_heap_root_le_all::<T>(self.spec_seq(), i);
                        };
                    }
                    Some(root)
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: append copies persistent array O(n), then bubble_up O(log n).
            fn insert(&self, element: T) -> Self {
                let single_seq = ArraySeqStPerS::singleton(element);
                let new_elements = ArraySeqStPerS::append(&self.elements, &single_seq);

                let last_index = new_elements.length() - 1;
                let ghost ne_seq = new_elements.seq@;
                let ghost ne_view = new_elements@;

                // Veracity: NEEDED proof block
                proof {
                    let n = self.elements.spec_len() as int;
                    let li = last_index as int; // = n
                    let orig = self.elements.seq@;

                    // T-level element identity from append.
                    // Veracity: NEEDED assert
                    assert forall|k: int| 0 <= k < n
                        implies #[trigger] ne_seq[k] == orig[k] by {
                        // Veracity: NEEDED assert
                        assert(new_elements.spec_index(k) == orig[k]);
                    }

                    // View-level correspondence for post-call multiset proof.
                    // Veracity: NEEDED assert
                    assert forall|k: int| 0 <= k < n
                        implies #[trigger] ne_view[k] == self@[k] by {
                    }
                    // Veracity: NEEDED assert
                    assert(ne_view =~= self@.push(element@));

                    if li > 0 {
                        let p = (li - 1) / 2;
                        // spec_is_exec_heap_except(ne_seq, p).
                        // Veracity: NEEDED assert
                        assert forall|j: int| 0 <= j < (n + 1) && j != p
                            implies #[trigger] BinaryHeapPQ::<T>::spec_exec_heap_inv_at(ne_seq, j) by {
                            if j == li {
                                // Last position: no children.
                            } else {
                                // Veracity: NEEDED assert
                                assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(orig, j));
                            }
                        };

                        // Sibling condition from original heap_inv_at(parent(li)).
                        // Veracity: NEEDED assert
                        assert(BinaryHeapPQ::<T>::spec_exec_heap_inv_at(orig, p));
                    }
                }

                let heapified = bubble_up_heap(new_elements, last_index);

                let pq = BinaryHeapPQ { elements: heapified };
                // Veracity: NEEDED proof block
                proof {
                    // heapified@.to_multiset() =~= ne_view.to_multiset() from bubble_up_heap.
                    // ne_view =~= self@.push(element@), established above.
                    // to_multiset_build broadcast completes the chain.
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: rebuild array O(n), then bubble_down O(log n).
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }

                if self.elements.length() == 1 {
                    let min_element = self.elements.nth(0).clone();
                    let empty_pq = Self::empty();
                    // Veracity: NEEDED proof block
                    proof {
                        axiom_cloned_implies_eq_owned(self.elements.spec_index(0), min_element);
                        // Veracity: NEEDED assert
                        assert(self@ =~= Seq::<T::V>::empty().push(self@[0]));
                    }
                    return (empty_pq, Some(min_element));
                }

                let min_element = self.elements.nth(0).clone();
                let last_element = self.elements.nth(self.elements.length() - 1).clone();

                let mut new_elements = ArraySeqStPerS::singleton(last_element);
                let n = self.elements.length();
                let end = n - 1;

                // Veracity: NEEDED proof block
                proof {
                    axiom_cloned_implies_eq_owned(self.elements.spec_index(0), min_element);
                    axiom_cloned_implies_eq_owned(self.elements.spec_index(n as int - 1), last_element);
                    // Establish initial element: new_elements@[0] == self@[n-1].
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..end
                    invariant
                        n == self.elements@.len(),
                        n >= 2,
                        end == n - 1,
                        new_elements@.len() == (i - 1) as int + 1,
                        new_elements@.len() == i as int,
                        (i as int) < n,
                        obeys_feq_clone::<T>(),
                        new_elements@[0] == self@[n as int - 1],
                        forall|m: int| 1 <= m < i as int ==>
                            #[trigger] new_elements@[m] == self@[m],
                        // T-level identity for heap-except proof.
                        new_elements.seq@[0] == last_element,
                        forall|m: int| 1 <= m < i as int ==>
                            #[trigger] new_elements.seq@[m] == self.elements.seq@[m],
                {
                    let elem = self.elements.nth(i);
                    let single_seq = ArraySeqStPerS::singleton(elem.clone());
                    let ghost pre_seq = new_elements.seq@;
                    let ghost pre_view = new_elements@;
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);

                    // Veracity: NEEDED proof block
                    proof {
                        // single_seq.spec_index(0) is the cloned element.
                        axiom_cloned_implies_eq_owned(self.elements.spec_index(i as int), single_seq.spec_index(0));
                        // New element at position i.
                        // Element at position 0 preserved.
                        // Previous elements preserved.
                        // Veracity: NEEDED assert
                        assert forall|m: int| 1 <= m < i as int
                            implies #[trigger] new_elements@[m] == self@[m] by {
                            // Veracity: NEEDED assert
                            assert(new_elements.spec_index(m)@ == new_elements@[m]);
                        }
                        // T-level: new element at position i.
                        // T-level: position 0 preserved.
                        // Veracity: NEEDED assert
                        assert(new_elements.seq@[0] == last_element) by {
                            // Veracity: NEEDED assert
                            assert(new_elements.spec_index(0) == pre_seq[0]);
                        }
                        // T-level: previous elements preserved.
                        // Veracity: NEEDED assert
                        assert forall|m: int| 1 <= m < i as int
                            implies #[trigger] new_elements.seq@[m] == self.elements.seq@[m] by {
                            // Veracity: NEEDED assert
                            assert(new_elements.spec_index(m) == pre_seq[m]);
                        }
                    }
                }

                // Prove heap-except-0 for new_elements.
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < new_elements.seq@.len() && j != 0
                        implies #[trigger] Self::spec_exec_heap_inv_at(new_elements.seq@, j) by {
                        // Veracity: NEEDED assert
                        assert(Self::spec_exec_heap_inv_at(self.elements.seq@, j));
                        let left = 2 * j + 1;
                        let right = 2 * j + 2;
                        if left < new_elements.seq@.len() {
                        }
                        if right < new_elements.seq@.len() {
                        }
                    }
                }

                // Capture ghost state before ownership transfer to bubble_down_heap.
                let ghost new_elem_view = new_elements@;
                let ghost new_elem_seq = new_elements.seq@;
                let heapified = bubble_down_heap(new_elements);

                let new_pq = BinaryHeapPQ { elements: heapified };
                // Veracity: NEEDED proof block
                proof {
                    // le(min_element, every element in new_elem_seq).
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < new_elem_seq.len()
                        implies #[trigger] TotalOrder::le(min_element, new_elem_seq[j]) by {
                        if j == 0 {
                            lemma_heap_root_le_all::<T>(self.spec_seq(), (n as int - 1));
                        } else {
                            lemma_heap_root_le_all::<T>(self.spec_seq(), j);
                        }
                    }
                    // Transfer le through multiset equality to heapified.
                    lemma_le_preserved_by_multiset_eq(min_element, new_elem_seq, heapified.seq@);

                    // Multiset proof.
                    let sr = self@.subrange(1, n as int);
                    // Show multisets are equal by relating to subrange.
                    // sr = [self@[1], ..., self@[n-1]], sr.last() = self@[n-1]
                    // new_elements@ = [sr.last()] + sr.subrange(0, sr.len()-1)

                    // Build the rearranged sequence: last element first, then rest.
                    let first = Seq::<T::V>::empty().push(sr.last());
                    let rest = sr.subrange(0, sr.len() - 1);
                    let rearranged = first + rest;

                    // Show new_elem_view =~= rearranged.
                    // Veracity: NEEDED assert
                    assert(new_elem_view =~= rearranged) by {
                    }

                    // rearranged.to_multiset() == sr.to_multiset()
                    // Because: sr = rest.push(sr.last()), so sr.to_multiset() = rest.to_multiset().insert(sr.last())
                    // And: rearranged = first + rest, rearranged.to_multiset() = first.to_multiset().add(rest.to_multiset())
                    //     = singleton(sr.last()).add(rest.to_multiset())
                    // Veracity: NEEDED assert
                    assert(sr =~= rest.push(sr.last())) by {
                    }
                    // sr.to_multiset() = rest.push(sr.last()).to_multiset() = rest.to_multiset().insert(sr.last())
                    // rearranged.to_multiset() = first.to_multiset().add(rest.to_multiset())
                    vstd::seq_lib::lemma_multiset_commutative(first, rest);
                    // (first + rest).to_multiset() = first.to_multiset().add(rest.to_multiset())
                    // first = Seq::empty().push(sr.last())
                    // first.to_multiset() = singleton(sr.last())
                    // So rearranged.to_multiset() = Multiset::singleton(sr.last()).add(rest.to_multiset())
                    //                             = rest.to_multiset().add(Multiset::singleton(sr.last()))
                    //                             = rest.to_multiset().insert(sr.last())
                    //                             = sr.to_multiset()

                    // Use to_multiset_remove to connect to self@.to_multiset()
                    vstd::seq_lib::to_multiset_remove(self@, 0int);
                    // self@.remove(0).to_multiset() == self@.to_multiset().remove(self@[0])
                    // So sr.to_multiset() == self@.to_multiset().remove(self@[0])
                    // And new_elem_view.to_multiset() == sr.to_multiset()
                    // And heapified@.to_multiset() == new_elem_view.to_multiset() (from bubble_down_heap)
                    // And min_element@ == self@[0]
                    // So: new_pq@.to_multiset().insert(min_element@)
                    //   = heapified@.to_multiset().insert(self@[0])
                    //   = new_elem_view.to_multiset().insert(self@[0])
                    //   = self@.to_multiset().remove(self@[0]).insert(self@[0])
                    //   = self@.to_multiset()
                    vstd::seq_lib::to_multiset_contains(self@, self@[0]);
                    assert_multisets_equal!(
                        self@.to_multiset().remove(self@[0]).insert(self@[0]),
                        self@.to_multiset(),
                        key => {
                            vstd::multiset::lemma_insert_increases_count_by_1(
                                self@.to_multiset().remove(self@[0]),
                                self@[0],
                            );
                            vstd::multiset::lemma_insert_other_elements_unchanged(
                                self@.to_multiset().remove(self@[0]),
                                self@[0],
                                key,
                            );
                        }
                    );
                }
                (new_pq, Some(min_element))
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(m+n), Span O(m+n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m+n), Span O(m+n); append O(m+n), then heapify O(m+n).
            fn meld(&self, other: &Self) -> Self {
                let merged = ArraySeqStPerS::append(&self.elements, &other.elements);
                let heapified = heapify(&merged);

                let pq = BinaryHeapPQ { elements: heapified };
                // Veracity: NEEDED proof block
                proof {
                    // Bridge: append ensures at T-level → view-level sequence equality.
                    let a_len = self.elements.spec_len() as int;
                    let b_len = other.elements.spec_len() as int;
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < a_len
                        implies #[trigger] merged@[i] == self@[i] by {
                        // Veracity: NEEDED assert
                        assert(merged.spec_index(i)@ == merged@[i]);
                    }
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < b_len
                        implies #[trigger] merged@[a_len + i] == other@[i] by {
                    }
                    // Veracity: NEEDED assert
                    assert(merged@ =~= self@ + other@);
                    vstd::seq_lib::lemma_multiset_commutative(self@, other@);
                    // heapify preserves multiset: heapified@.to_multiset() =~= merged@.to_multiset()
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); heapify via bottom-up bubble_down.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
            {
                let heapified = heapify(seq);
                BinaryHeapPQ { elements: heapified }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); field access.
            fn size(&self) -> usize {
                self.elements.length()
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); field access and comparison.
            fn is_empty(&self) -> bool {
                self.elements.length() == 0
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); clones persistent array.
            fn to_seq(&self) -> ArraySeqStPerS<T> {
                self.elements.clone()
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O((m+n) log(m+n)), Span O((m+n) log(m+n)).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m+n), Span O(m+n) — DIFFERS: from_seq O(m), then meld O(m+n).
            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n log n), Span O(n log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — DIFFERS: n iterations of delete_min each O(n).
            #[verifier::exec_allows_no_decreases_clause]
            fn extract_all_sorted(&self) -> ArraySeqStPerS<T> {
                let mut result = ArraySeqStPerS::empty();
                let mut current_heap = self.clone();

                // Clone preserves T-level seq equality, so heap property transfers.
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < current_heap.spec_seq().len()
                        implies #[trigger] Self::spec_exec_heap_inv_at(
                            current_heap.spec_seq(), j) by {
                        // Veracity: NEEDED assert
                        assert(Self::spec_exec_heap_inv_at(self.elements.seq@, j));
                        let left = 2 * j + 1;
                        let right = 2 * j + 2;
                        if left < current_heap.spec_seq().len() {
                        }
                        if right < current_heap.spec_seq().len() {
                        }
                    }
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while !current_heap.is_empty()
                    invariant
                        current_heap@.len() * 2 <= usize::MAX as int,
                        result@.len() + current_heap@.len() == self@.len(),
                        obeys_feq_clone::<T>(),
                        Self::spec_is_exec_heap(current_heap.spec_seq()),
                        Self::spec_sorted(result.seq@),
                        forall|r: int, h: int|
                            #![trigger result.seq@[r], current_heap.spec_seq()[h]]
                            0 <= r < result.seq@.len()
                                && 0 <= h < current_heap.spec_seq().len() ==>
                            TotalOrder::le(result.seq@[r], current_heap.spec_seq()[h]),
                {
                    let ghost old_result_seq = result.seq@;
                    let ghost old_result_len = result.seq@.len() as int;
                    let (new_heap, min_element) = current_heap.delete_min();
                    if let Some(element) = min_element {
                        let single_seq = ArraySeqStPerS::singleton(element);
                        let ghost pre_seq = result.seq@;
                        result = ArraySeqStPerS::append(&result, &single_seq);

                        // Veracity: NEEDED proof block
                        proof {
                            // element == current_heap.spec_seq()[0] from delete_min.
                            // le(element, new_heap[h]) for all h from delete_min.

                            // T-level identity through append.

                            // Sorted: old elements + new element.
                            // Veracity: NEEDED assert
                            assert forall|i: int, j: int|
                                0 <= i < j < result.seq@.len()
                                implies #[trigger] TotalOrder::le(
                                    result.seq@[i], result.seq@[j]) by {
                                if i < old_result_len && j < old_result_len {
                                    // Veracity: NEEDED assert
                                    assert(result.spec_index(i) == pre_seq[i]);
                                    // Veracity: NEEDED assert
                                    assert(result.spec_index(j) == pre_seq[j]);
                                } else if i < old_result_len {
                                    // j == old_result_len; new element is the min.
                                    // Veracity: NEEDED assert
                                    assert(result.spec_index(i) == pre_seq[i]);
                                    // le(old[i], current_heap root) from boundary.
                                    // Use boundary: le(old_result_seq[i],
                                    //   current_heap.spec_seq()[0])
                                    // and element == current_heap.spec_seq()[0].
                                }
                            };

                            // Boundary: all result elements <= all new_heap elements.
                            // Veracity: NEEDED assert
                            assert forall|r: int, h: int|
                                0 <= r < result.seq@.len()
                                    && 0 <= h < new_heap.spec_seq().len()
                                implies TotalOrder::le(result.seq@[r],
                                    new_heap.spec_seq()[h]) by {
                                if r < old_result_len {
                                    // le(old[r], element) from boundary with h=0.
                                    // le(element, new_heap[h]) from delete_min.
                                    T::transitive(result.seq@[r], element,
                                        new_heap.spec_seq()[h]);
                                }
                                // r == old_result_len: result[r] == element,
                                // le(element, new_heap[h]) from delete_min.
                            };
                        }
                    }
                    current_heap = new_heap;
                }

                result
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); linear scan checking parent-child ordering.
            fn is_valid_heap(&self) -> bool {
                is_heap(&self.elements)
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n); computes floor(log2(n)) + 1.
            fn height(&self) -> usize {
                let n = self.elements.length();
                if n == 0 {
                    0
                } else {
                    let h = exec_log2(n);
                    // Veracity: NEEDED proof block
                    proof {
                        vstd::layout::unsigned_int_max_values();
                        vstd::arithmetic::power2::lemma_pow2(usize::BITS as nat);
                        lemma_log2_bound(n as int, usize::BITS as nat);
                    }
                    h + 1
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(2^level), Span O(2^level).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(2^level), Span O(2^level); copies elements in [2^level - 1, 2^(level+1) - 1).
            fn level_elements(&self, level: usize) -> ArraySeqStPerS<T> {
                let mut result = ArraySeqStPerS::empty();
                let n = self.elements.length();
                // Veracity: NEEDED proof block
                proof {
                    lemma_pow2_pos(level as nat);
                    lemma_pow2_pos((level + 1) as nat);
                    vstd::arithmetic::power2::lemma_pow2_strictly_increases(level as nat, 64nat);
                    vstd::arithmetic::power2::lemma_pow2_strictly_increases((level + 1) as nat, 64nat);
                    vstd::arithmetic::power2::lemma2_to64();
                    vstd::layout::unsigned_int_max_values();
                    if usize::BITS > 64 {
                        vstd::arithmetic::power2::lemma_pow2_strictly_increases(64nat, usize::BITS as nat);
                    }
                }
                let p = exec_pow2(level);
                let p2 = exec_pow2(level + 1);
                let start_idx = p - 1;
                let end_idx = if p2 - 1 < n { p2 - 1 } else { n };

                let mut i = start_idx;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < end_idx
                    invariant
                        start_idx <= i,
                        end_idx <= n,
                        n == self.elements@.len(),
                        result@.len() == (i - start_idx) as int,
                        result@.len() + 1 <= usize::MAX as int,
                    decreases end_idx - i,
                {
                    let elem = self.elements.nth(i);
                    let single_seq = ArraySeqStPerS::singleton(elem.clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    i = i + 1;
                }

                result
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); from_vec then heapify.
            fn from_vec(vec: Vec<T>) -> Self {
                let seq = ArraySeqStPerS::from_vec(vec);
                Self::from_seq(&seq)
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); linear copy to Vec.
            fn to_vec(&self) -> Vec<T> {
                let seq = self.to_seq();
                let n = seq.length();
                let mut result: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == seq@.len(),
                        result@.len() == i as int,
                        forall|j: int| 0 <= j < i ==> (result@[j])@ == #[trigger] seq@[j],
                {
                    let elem = seq.nth(i).clone();
                    // Veracity: NEEDED proof block
                    proof { axiom_cloned_implies_eq_owned(seq.spec_index(i as int), elem); }
                    result.push(elem);
                }
                result
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n log n), Span O(n log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — DIFFERS: extract_all_sorted O(n^2), then linear copy.
            fn to_sorted_vec(&self) -> Vec<T> {
                let sorted_seq = self.extract_all_sorted();
                let n = sorted_seq.length();
                let mut result: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == sorted_seq@.len(),
                        n == sorted_seq.seq@.len(),
                        result@.len() == i as int,
                        forall|j: int| 0 <= j < i ==> (result@[j])@ == #[trigger] sorted_seq@[j],
                        forall|j: int| 0 <= j < i ==> #[trigger] result@[j] == sorted_seq.seq@[j],
                        Self::spec_sorted(sorted_seq.seq@),
                        obeys_feq_clone::<T>(),
                {
                    let elem = sorted_seq.nth(i).clone();
                    // Veracity: NEEDED proof block
                    proof { axiom_cloned_implies_eq_owned(sorted_seq.spec_index(i as int), elem); }
                    result.push(elem);
                }
                // Veracity: NEEDED proof block
                proof {
                }
                result
            }
        }

    //		Section 12. derive impls in verus!


        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for BinaryHeapPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord + TotalOrder> Default for BinaryHeapPQ<T> {
            fn default() -> Self {
                Self::empty()
            }
        }


        impl<T: StT + Ord + TotalOrder> Clone for BinaryHeapPQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures
                    cloned@ == self@,
                    cloned.elements.seq@.len() == self.elements.seq@.len(),
                    forall|i: int| 0 <= i < self.elements.seq@.len() ==>
                        #[trigger] cloned.elements.seq@[i] == self.elements.seq@[i],
            {
                let cloned = BinaryHeapPQ { elements: self.elements.clone() };
                // Veracity: NEEDED proof block
                proof {
                    assume(obeys_feq_clone::<T>());
                    lemma_seq_map_cloned_view_eq(
                        self.elements.seq@,
                        cloned.elements.seq@,
                    );
                    // T-level seq equality from clone axiom.
                }
                cloned
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for BinaryHeapPQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self@ == other@)
            {
                let equal = self.elements == other.elements;
                // Veracity: NEEDED proof block
                proof { assume(equal == (self@ == other@)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for BinaryHeapPQ<T> {}

    }

    //		Section 13. macros


    #[macro_export]
    macro_rules! BinaryHeapPQLit {
        () => {
            $crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::BinaryHeapPQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let mut pq = $crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::BinaryHeapPQ::empty();
            $(
                pq = pq.insert($x);
            )*
            pq
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder + std::fmt::Debug> std::fmt::Debug for BinaryHeapPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BinaryHeapPQ").field("elements", &self.elements).finish()
        }
    }

    impl<T: StT + Ord + TotalOrder> Display for BinaryHeapPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "BinaryHeapPQ[")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "]")
        }
    }
}
