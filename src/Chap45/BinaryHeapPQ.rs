//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 45: Priority Queue implementation using Binary Heap

//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4. type definitions
//  5. view impls
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!

//  1. module


pub mod BinaryHeapPQ {

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

    verus! {

//  2. imports

        use crate::Types::Types::*;
        use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
        use crate::vstdplus::accept::accept;
        use crate::vstdplus::total_order::total_order::TotalOrder;
        #[cfg(verus_keep_ghost)]
        use crate::vstdplus::feq::feq::*;


//  3. broadcast use

        broadcast use {
            crate::vstdplus::feq::feq::group_feq_axioms,
            vstd::seq::group_seq_axioms,
            vstd::seq_lib::group_seq_properties,
            vstd::seq_lib::group_to_multiset_ensures,
            vstd::std_specs::vec::group_vec_axioms,
        };


//  4. type definitions

        #[verifier::reject_recursive_types(T)]
        pub struct BinaryHeapPQ<T: StT + Ord + TotalOrder> {
            pub elements: ArraySeqStPerS<T>,
        }


//  5. view impls

        impl<T: StT + Ord + TotalOrder> View for BinaryHeapPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }



//  7. proof fns/broadcast groups

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
                assert(n / 2 < vstd::arithmetic::power::pow(2, (bits - 1) as nat));
                lemma_log2_bound(n / 2, (bits - 1) as nat);
            }
        }


//  8. traits

        /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
        pub trait BinaryHeapPQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Seq<T::V>> {
            spec fn spec_binaryheappq_wf(&self) -> bool;
            spec fn spec_size(self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;
            spec fn spec_sorted(s: Seq<T>) -> bool;
            spec fn spec_heap_inv_at(seq: Seq<T::V>, i: int) -> bool;
            spec fn spec_leq_view(a: T::V, b: T::V) -> bool;
            spec fn spec_is_heap(seq: Seq<T::V>) -> bool;
            spec fn parent_spec(i: int) -> int;
            spec fn left_child_spec(i: int) -> int;
            spec fn right_child_spec(i: int) -> int;

            fn empty() -> (pq: Self)
                ensures
                    pq@.len() == 0,
                    pq@.to_multiset() =~= Multiset::empty(),
                    pq.spec_binaryheappq_wf();

            fn singleton(element: T) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq@.len() == 1,
                    pq@.to_multiset() =~= Multiset::empty().insert(element@),
                    pq.spec_binaryheappq_wf();

            fn find_min(&self) -> (min_elem: Option<&T>)
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some(),
                    self@.len() > 0 ==> min_elem.unwrap()@ == self@[0];

            fn insert(&self, element: T) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + 1 <= usize::MAX as int,
                ensures
                    pq@.len() == self@.len() + 1,
                    pq@.to_multiset() =~= self@.to_multiset().insert(element@);

            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int,
                ensures
                    self@.len() > 0 ==> min_and_rest.1.is_some(),
                    self@.len() > 0 ==> min_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> min_and_rest.1.is_none(),
                    self@.len() == 0 ==> min_and_rest.0@.len() == 0,
                    self@.len() > 0 ==> self@.to_multiset() =~=
                        min_and_rest.0@.to_multiset().insert(min_and_rest.1.unwrap()@);

            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + other@.len() <= usize::MAX as int,
                    (self@.len() + other@.len()) * 2 <= usize::MAX as int,
                ensures
                    pq@.len() == self@.len() + other@.len(),
                    pq@.to_multiset() =~= self@.to_multiset().add(other@.to_multiset());

            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    seq@.len() * 2 <= usize::MAX as int,
                ensures
                    pq@.len() == seq@.len(),
                    pq@.to_multiset() =~= seq@.to_multiset();

            fn size(&self) -> (n: usize)
                ensures n as int == self.spec_size();

            fn is_empty(&self) -> (empty: bool)
                ensures empty == (self.spec_size() == 0);

            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
                requires obeys_feq_clone::<T>(),
                ensures seq@ =~= self@;

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + elements@.len() <= usize::MAX as int,
                    (self@.len() + elements@.len()) * 2 <= usize::MAX as int,
                ensures pq@.len() == self@.len() + elements@.len();

            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int,
                ensures
                    sorted@.len() == self@.len(),
                    Self::spec_sorted(sorted.seq@);

            fn is_valid_heap(&self) -> (valid: bool)
                requires self@.len() * 2 <= usize::MAX as int;

            fn height(&self) -> (levels: usize)
                requires self@.len() <= usize::MAX as int,
                ensures self@.len() == 0 ==> levels == 0;

            fn level_elements(&self, level: usize) -> (elts: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    level < 63,
                    usize::BITS >= 64;

            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    vec@.len() * 2 <= usize::MAX as int,
                ensures pq@.len() == vec@.len();

            fn to_vec(&self) -> (v: Vec<T>)
                requires obeys_feq_clone::<T>(),
                ensures v@.len() == self@.len();

            fn to_sorted_vec(&self) -> (v: Vec<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int,
                ensures
                    v@.len() == self@.len(),
                    Self::spec_sorted(v@);
        }


//  9. impls

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for BinaryHeapPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        fn left_child(i: usize) -> (child_idx: usize)
            requires i <= usize::MAX / 2 - 1,
            ensures child_idx as int == 2 * (i as int) + 1,
        {
            2 * i + 1
        }

        fn right_child(i: usize) -> (child_idx: usize)
            requires i <= usize::MAX / 2 - 1,
            ensures child_idx as int == 2 * (i as int) + 2,
        {
            2 * i + 2
        }

        fn parent(i: usize) -> (parent_idx: usize)
            requires true,
            ensures parent_idx as int == (if i == 0 { 0int } else { (i as int - 1) / 2 }),
        {
            if i == 0 { 0 } else { (i - 1) / 2 }
        }

        proof fn lemma_swap_preserves_multiset<A>(s: Seq<A>, i: int, j: int)
            requires 0 <= i < s.len(), 0 <= j < s.len(),
            ensures s.update(i, s[j]).update(j, s[i]).to_multiset() =~= s.to_multiset(),
        {
            let s1 = s.update(i, s[j]);
            let s2 = s1.update(j, s[i]);
            vstd::seq_lib::to_multiset_update(s, i, s[j]);
            // s1.to_multiset() == s.to_multiset().insert(s[j]).remove(s[i])
            assert(s1[j] == s[j]);  // update at i doesn't affect j (or i==j: updated to s[j])
            vstd::seq_lib::to_multiset_update(s1, j, s[i]);
            // s2.to_multiset() == s1.to_multiset().insert(s[i]).remove(s1[j])
            //                   == s.to_multiset().insert(s[j]).remove(s[i]).insert(s[i]).remove(s[j])
            let m = s.to_multiset();
            let a = s[i];
            let b = s[j];
            // Need: m.insert(b).remove(a).insert(a).remove(b) == m
            assert(s.contains(s[i]));
            vstd::seq_lib::to_multiset_contains(s, s[i]);
            assert(m.count(a) >= 1nat);
            assert_multisets_equal!(m.insert(b).remove(a).insert(a).remove(b), m, key => {
                vstd::multiset::lemma_insert_increases_count_by_1(m, b);
                vstd::multiset::lemma_insert_other_elements_unchanged(m, b, key);
                vstd::multiset::lemma_insert_increases_count_by_1(m.insert(b).remove(a), a);
                vstd::multiset::lemma_insert_other_elements_unchanged(m.insert(b).remove(a), a, key);
            });
        }

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

                proof {
                    let pos: int = if k == i { j as int } else if k == j { i as int } else { k as int };
                    axiom_cloned_implies_eq_owned(seq.spec_index(pos), element);
                    assert(element@ == seq@[pos]) by {
                        assert(seq.spec_index(pos)@ == seq@[pos]);
                    }
                    // New element at position k.
                    assert(result@[k as int] == element@) by {
                        assert(result.spec_index(k as int) == single_seq.seq@[0]);
                        assert(single_seq.spec_index(0) == element);
                        assert(result.spec_index(k as int)@ == result@[k as int]);
                    }
                    // Previous elements preserved by append.
                    assert forall|m: int| 0 <= m < k as int
                        implies #[trigger] result@[m] == sv[m] by {
                        assert(result.spec_index(m) == pre_seq[m]);
                        assert(result.spec_index(m)@ == result@[m]);
                        assert(pre_seq[m]@ == pre_view[m]);
                        assert(pre_view[m] == sv[m]);
                    }
                }
            }

            proof {
                assert(result@ =~= sv);
                lemma_swap_preserves_multiset(seq@, i as int, j as int);
            }
            result
        }

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

            proof {
                // Clone bridge: cloned elements are T-level equal, so views match.
                assert(result@.len() == seq@.len());
                assert forall|k: int| 0 <= k < seq@.len()
                    implies #[trigger] result@[k] == seq@[k] by {}
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
                proof {
                    assert((parent_idx as int) < seq.view().len());
                }
                let current = result.nth(i);
                let parent_val = result.nth(parent_idx);

                if *current >= *parent_val {
                    i = 0;
                } else {
                    proof {
                        assert((i as int) < result.view().len());
                        assert((parent_idx as int) < result.view().len());
                    }
                    result = swap_elements(&result, i, parent_idx);
                    i = parent_idx;
                }
            }

            result
        }

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

            proof {
                assert(result@.len() == heap@.len());
                assert forall|k: int| 0 <= k < heap@.len()
                    implies #[trigger] result@[k] == heap@[k] by {}
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
                    assert(smallest > idx);
                    result = swap_elements(&result, idx, smallest);
                    idx = smallest;
                }
                proof { old_idx = idx as int; }
            }

            result
        }

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
                proof {
                    assert(r@.len() == seq@.len());
                    assert forall|k: int| 0 <= k < seq@.len()
                        implies #[trigger] r@[k] == seq@[k] by {}
                    assert(r@ =~= seq@);
                }
                return r;
            }

            let mut result = seq.clone();

            proof {
                assert(result@.len() == seq@.len());
                assert forall|k: int| 0 <= k < seq@.len()
                    implies #[trigger] result@[k] == seq@[k] by {}
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

        fn exec_pow2(e: usize) -> (power: usize)
            requires pow2(e as nat) <= usize::MAX as int,
            ensures power as int == pow2(e as nat),
        {
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

        fn exec_log2(n: usize) -> (log_val: usize)
            requires n >= 1,
            ensures log_val as int == log(2, n as int),
            decreases n,
        {
            if n < 2 {
                proof { lemma_log0(2, n as int); }
                0
            } else {
                proof { lemma_log_s(2, n as int); }
                let rest = exec_log2(n / 2);
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

            open spec fn parent_spec(i: int) -> int {
                if i == 0 { 0 } else { (i - 1) / 2 }
            }

            open spec fn left_child_spec(i: int) -> int {
                2 * i + 1
            }

            open spec fn right_child_spec(i: int) -> int {
                2 * i + 2
            }

            fn empty() -> (pq: Self) {
                let pq = BinaryHeapPQ {
                    elements: ArraySeqStPerS::empty(),
                };
                proof {
                    assert(pq@.len() == 0);
                }
                pq
            }

            fn singleton(element: T) -> (pq: Self) {
                let pq = BinaryHeapPQ {
                    elements: ArraySeqStPerS::singleton(element),
                };
                proof {
                    assert(pq@.len() == 1);
                    assert(pq@[0] == element@);
                    assert(pq@ =~= Seq::<T::V>::empty().push(element@));
                }
                pq
            }

            fn find_min(&self) -> (min_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(0))
                }
            }

            fn insert(&self, element: T) -> Self {
                let single_seq = ArraySeqStPerS::singleton(element);
                let new_elements = ArraySeqStPerS::append(&self.elements, &single_seq);

                let last_index = new_elements.length() - 1;
                let heapified = bubble_up(&new_elements, last_index);

                let pq = BinaryHeapPQ { elements: heapified };
                proof {
                    // Bridge: append ensures at T-level → view-level sequence equality.
                    let n = self.elements.spec_len() as int;
                    assert(new_elements@.len() == n + 1);
                    assert forall|i: int| 0 <= i < n
                        implies #[trigger] new_elements@[i] == self@[i] by {
                        // T-level: append ensures spec_index correspondence.
                        assert(new_elements.spec_index(i) == self.elements.seq@[i]);
                        // View bridge: spec_index(i)@ == @[i] by map definition.
                        assert(new_elements.spec_index(i)@ == new_elements@[i]);
                        assert(self.elements.seq@[i]@ == self@[i]);
                    }
                    assert(new_elements@[n] == element@) by {
                        assert(new_elements.spec_index(n) == single_seq.seq@[0]);
                        assert(single_seq.spec_index(0) == element);
                        assert(new_elements.spec_index(n)@ == new_elements@[n]);
                    }
                    assert(new_elements@ =~= self@.push(element@));
                    // to_multiset_build broadcast: push(a).to_multiset() =~= to_multiset().insert(a)
                    // heapified@.to_multiset() =~= new_elements@.to_multiset() from bubble_up
                }
                pq
            }

            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }

                if self.elements.length() == 1 {
                    let min_element = self.elements.nth(0).clone();
                    let empty_pq = Self::empty();
                    proof {
                        axiom_cloned_implies_eq_owned(self.elements.spec_index(0), min_element);
                        assert(self@.len() == 1);
                        assert(self@ =~= Seq::<T::V>::empty().push(self@[0]));
                    }
                    return (empty_pq, Some(min_element));
                }

                let min_element = self.elements.nth(0).clone();
                let last_element = self.elements.nth(self.elements.length() - 1).clone();

                let mut new_elements = ArraySeqStPerS::singleton(last_element);
                let n = self.elements.length();
                let end = n - 1;

                proof {
                    axiom_cloned_implies_eq_owned(self.elements.spec_index(0), min_element);
                    assert(min_element@ == self@[0]) by {
                        assert(self.elements.spec_index(0)@ == self@[0]);
                    }
                    axiom_cloned_implies_eq_owned(self.elements.spec_index(n as int - 1), last_element);
                    assert(last_element@ == self@[n as int - 1]) by {
                        assert(self.elements.spec_index(n as int - 1)@ == self@[n as int - 1]);
                    }
                    // Establish initial element: new_elements@[0] == self@[n-1].
                    assert(new_elements@[0] == self@[n as int - 1]) by {
                        assert(new_elements.spec_index(0) == last_element);
                        assert(new_elements.spec_index(0)@ == new_elements@[0]);
                    }
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
                {
                    let elem = self.elements.nth(i);
                    let single_seq = ArraySeqStPerS::singleton(elem.clone());
                    let ghost pre_seq = new_elements.seq@;
                    let ghost pre_view = new_elements@;
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);

                    proof {
                        // single_seq.spec_index(0) is the cloned element.
                        axiom_cloned_implies_eq_owned(self.elements.spec_index(i as int), single_seq.spec_index(0));
                        assert(single_seq.spec_index(0)@ == self@[i as int]) by {
                            assert(self.elements.spec_index(i as int)@ == self@[i as int]);
                        }
                        // New element at position i.
                        assert(new_elements@[i as int] == self@[i as int]) by {
                            assert(new_elements.spec_index(i as int) == single_seq.seq@[0]);
                            assert(new_elements.spec_index(i as int)@ == new_elements@[i as int]);
                        }
                        // Element at position 0 preserved.
                        assert(new_elements@[0] == self@[n as int - 1]) by {
                            assert(new_elements.spec_index(0) == pre_seq[0]);
                            assert(new_elements.spec_index(0)@ == new_elements@[0]);
                            assert(pre_seq[0]@ == pre_view[0]);
                        }
                        // Previous elements preserved.
                        assert forall|m: int| 1 <= m < i as int
                            implies #[trigger] new_elements@[m] == self@[m] by {
                            assert(new_elements.spec_index(m) == pre_seq[m]);
                            assert(new_elements.spec_index(m)@ == new_elements@[m]);
                            assert(pre_seq[m]@ == pre_view[m]);
                        }
                    }
                }

                let heapified = bubble_down(&new_elements, 0);

                let new_pq = BinaryHeapPQ { elements: heapified };
                proof {
                    // new_elements has elements: [self@[n-1], self@[1], ..., self@[n-2]]
                    // These are exactly self@.subrange(1, n) rearranged.
                    let sr = self@.subrange(1, n as int);
                    // Show multisets are equal by relating to subrange.
                    // sr = [self@[1], ..., self@[n-1]], sr.last() = self@[n-1]
                    // new_elements@ = [sr.last()] + sr.subrange(0, sr.len()-1)
                    assert(sr.len() == n as int - 1);
                    assert(sr.last() == self@[n as int - 1]) by {
                        assert(sr[sr.len() - 1] == self@[sr.len() - 1 + 1]);
                    }

                    // Build the rearranged sequence: last element first, then rest.
                    let first = Seq::<T::V>::empty().push(sr.last());
                    let rest = sr.subrange(0, sr.len() - 1);
                    let rearranged = first + rest;

                    // Show new_elements@ =~= rearranged.
                    assert(rearranged.len() == sr.len());
                    assert(new_elements@.len() == sr.len());
                    assert(new_elements@ =~= rearranged) by {
                        assert(new_elements@[0] == rearranged[0]);
                        assert forall|m: int| 1 <= m < new_elements@.len()
                            implies #[trigger] new_elements@[m] == rearranged[m] by {
                            assert(new_elements@[m] == self@[m]);
                            assert(0 <= m - 1 < rest.len());
                            assert(rearranged[m] == rest[m - 1]);
                            assert(0 <= m - 1 < sr.len());
                            assert(rest[m - 1] == sr[m - 1]);
                            assert(sr[m - 1] == self@[m - 1 + 1]);
                        }
                    }

                    // rearranged.to_multiset() == sr.to_multiset()
                    // Because: sr = rest.push(sr.last()), so sr.to_multiset() = rest.to_multiset().insert(sr.last())
                    // And: rearranged = first + rest, rearranged.to_multiset() = first.to_multiset().add(rest.to_multiset())
                    //     = singleton(sr.last()).add(rest.to_multiset())
                    assert(sr =~= rest.push(sr.last())) by {
                        assert(sr.len() == rest.push(sr.last()).len());
                        assert forall|m: int| 0 <= m < sr.len()
                            implies #[trigger] sr[m] == rest.push(sr.last())[m] by {
                            if m < sr.len() - 1 {
                                assert(0 <= m < rest.len());
                                assert(rest.push(sr.last())[m] == rest[m]);
                                assert(rest[m] == sr[m]);
                            }
                        }
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
                    assert(self@.remove(0) =~= sr);
                    // So sr.to_multiset() == self@.to_multiset().remove(self@[0])
                    // And new_elements@.to_multiset() == sr.to_multiset()
                    // And heapified@.to_multiset() == new_elements@.to_multiset() (from bubble_down)
                    // And min_element@ == self@[0]
                    // So: new_pq@.to_multiset().insert(min_element@)
                    //   = heapified@.to_multiset().insert(self@[0])
                    //   = new_elements@.to_multiset().insert(self@[0])
                    //   = self@.to_multiset().remove(self@[0]).insert(self@[0])
                    //   = self@.to_multiset()
                    assert(self@.contains(self@[0]));
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

            fn meld(&self, other: &Self) -> Self {
                let merged = ArraySeqStPerS::append(&self.elements, &other.elements);
                let heapified = heapify(&merged);

                let pq = BinaryHeapPQ { elements: heapified };
                proof {
                    // Bridge: append ensures at T-level → view-level sequence equality.
                    let a_len = self.elements.spec_len() as int;
                    let b_len = other.elements.spec_len() as int;
                    assert(merged@.len() == a_len + b_len);
                    assert forall|i: int| 0 <= i < a_len
                        implies #[trigger] merged@[i] == self@[i] by {
                        assert(merged.spec_index(i) == self.elements.seq@[i]);
                        assert(merged.spec_index(i)@ == merged@[i]);
                        assert(self.elements.seq@[i]@ == self@[i]);
                    }
                    assert forall|i: int| 0 <= i < b_len
                        implies #[trigger] merged@[a_len + i] == other@[i] by {
                        assert(merged.spec_index(a_len + i) == other.elements.seq@[i]);
                        assert(merged.spec_index(a_len + i)@ == merged@[a_len + i]);
                        assert(other.elements.seq@[i]@ == other@[i]);
                    }
                    assert(merged@ =~= self@ + other@);
                    vstd::seq_lib::lemma_multiset_commutative(self@, other@);
                    // heapify preserves multiset: heapified@.to_multiset() =~= merged@.to_multiset()
                }
                pq
            }

            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
            {
                let heapified = heapify(seq);
                BinaryHeapPQ { elements: heapified }
            }

            fn size(&self) -> usize {
                self.elements.length()
            }

            fn is_empty(&self) -> bool {
                self.elements.length() == 0
            }

            fn to_seq(&self) -> ArraySeqStPerS<T> {
                self.elements.clone()
            }

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            #[verifier::exec_allows_no_decreases_clause]
            fn extract_all_sorted(&self) -> ArraySeqStPerS<T> {
                let mut result = ArraySeqStPerS::empty();
                let mut current_heap = self.clone();

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while !current_heap.is_empty()
                    invariant
                        current_heap@.len() * 2 <= usize::MAX as int,
                        result@.len() + current_heap@.len() == self@.len(),
                {
                    let (new_heap, min_element) = current_heap.delete_min();
                    if let Some(element) = min_element {
                        let single_seq = ArraySeqStPerS::singleton(element);
                        result = ArraySeqStPerS::append(&result, &single_seq);
                    }
                    current_heap = new_heap;
                }

                // accept hole: Proving sortedness requires heap property invariant (task #8).
                proof { assume(Self::spec_sorted(result.seq@)); }
                result
            }

            fn is_valid_heap(&self) -> bool {
                is_heap(&self.elements)
            }

            fn height(&self) -> usize {
                let n = self.elements.length();
                if n == 0 {
                    0
                } else {
                    let h = exec_log2(n);
                    proof {
                        vstd::layout::unsigned_int_max_values();
                        vstd::arithmetic::power2::lemma_pow2(usize::BITS as nat);
                        lemma_log2_bound(n as int, usize::BITS as nat);
                    }
                    h + 1
                }
            }

            fn level_elements(&self, level: usize) -> ArraySeqStPerS<T> {
                let mut result = ArraySeqStPerS::empty();
                let n = self.elements.length();
                proof {
                    lemma_pow2_pos(level as nat);
                    lemma_pow2_pos((level + 1) as nat);
                    vstd::arithmetic::power2::lemma_pow2_strictly_increases(level as nat, 64nat);
                    vstd::arithmetic::power2::lemma_pow2_strictly_increases((level + 1) as nat, 64nat);
                    vstd::arithmetic::power2::lemma2_to64();
                    vstd::layout::unsigned_int_max_values();
                    assert(pow2(64) == 0x10000000000000000nat);
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

            fn from_vec(vec: Vec<T>) -> Self {
                let seq = ArraySeqStPerS::from_vec(vec);
                Self::from_seq(&seq)
            }

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
                    proof { axiom_cloned_implies_eq_owned(seq.spec_index(i as int), elem); }
                    result.push(elem);
                }
                result
            }

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
                    proof { axiom_cloned_implies_eq_owned(sorted_seq.spec_index(i as int), elem); }
                    result.push(elem);
                }
                proof {
                    assert forall|ii: int, jj: int| 0 <= ii < jj < result@.len()
                        implies #[trigger] TotalOrder::le(result@[ii], result@[jj]) by {
                        assert(result@[ii] == sorted_seq.seq@[ii]);
                        assert(result@[jj] == sorted_seq.seq@[jj]);
                    }
                }
                result
            }
        }

        impl<T: StT + Ord + TotalOrder> Default for BinaryHeapPQ<T> {
            fn default() -> Self {
                Self::empty()
            }
        }


//  11. derive impls in verus!

        impl<T: StT + Ord + TotalOrder> Clone for BinaryHeapPQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned@ == self@
            {
                let cloned = BinaryHeapPQ { elements: self.elements.clone() };
                proof {
                    accept(obeys_feq_clone::<T>());
                    lemma_seq_map_cloned_view_eq(
                        self.elements.seq@,
                        cloned.elements.seq@,
                    );
                }
                cloned
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for BinaryHeapPQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self@ == other@)
            {
                let equal = self.elements == other.elements;
                proof { accept(equal == (self@ == other@)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for BinaryHeapPQ<T> {}

    }


//  13. derive impls outside verus!

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


//  12. macros

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

}
