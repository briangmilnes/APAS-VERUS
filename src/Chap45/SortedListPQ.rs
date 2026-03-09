//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Sorted List

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

pub mod SortedListPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use vstd::multiset::Multiset;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::multiset::group_multiset_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::std_specs::vec::group_vec_axioms,
};

// 4. type definitions
        #[verifier::reject_recursive_types(T)]
        pub struct SortedListPQ<T: StT + Ord + TotalOrder> {
            pub elements: ArraySeqStPerS<T>,
        }

// 5. view impls
        impl<T: StT + Ord + TotalOrder> View for SortedListPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

// 7. proof fns
        proof fn _sorted_list_pq_verified() {}

// 8. traits
        /// Meldable Priority Queue ADT (Data Type 45.1) using sorted list.
        pub trait SortedListPQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Seq<T::V>> {
            spec fn spec_size(self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;
            spec fn spec_sorted(s: Seq<T>) -> bool;

            proof fn lemma_push_preserves_sorted(s: Seq<T>, x: T)
                requires
                    Self::spec_sorted(s),
                    s.len() > 0 ==> TotalOrder::le(s.last(), x),
                ensures
                    Self::spec_sorted(s.push(x));

            fn empty() -> (pq: Self)
                ensures
                    pq@.len() == 0,
                    pq@.to_multiset() =~= Multiset::empty(),
                    Self::spec_sorted(pq.spec_seq());

            fn singleton(element: T) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq@.len() == 1,
                    pq@.to_multiset() =~= Multiset::empty().insert(element@),
                    Self::spec_sorted(pq.spec_seq());

            fn find_min(&self) -> (min_elem: Option<&T>)
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some(),
                    self@.len() > 0 ==> min_elem.unwrap()@ == self@[0];

            fn insert(&self, element: T) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + 1 <= usize::MAX as int,
                    Self::spec_sorted(self.spec_seq()),
                ensures
                    pq@.len() == self@.len() + 1,
                    pq@.to_multiset() =~= self@.to_multiset().insert(element@),
                    Self::spec_sorted(pq.spec_seq());

            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires
                    obeys_feq_clone::<T>(),
                    Self::spec_sorted(self.spec_seq()),
                ensures
                    self@.len() > 0 ==> min_and_rest.1.is_some(),
                    self@.len() > 0 ==> min_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> min_and_rest.1.is_none(),
                    self@.len() == 0 ==> min_and_rest.0@.len() == self@.len(),
                    self@.len() > 0 ==> self@.to_multiset() =~=
                        min_and_rest.0@.to_multiset().insert(min_and_rest.1.unwrap()@),
                    Self::spec_sorted(min_and_rest.0.spec_seq());

            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + other@.len() <= usize::MAX as int,
                    Self::spec_sorted(self.spec_seq()),
                    Self::spec_sorted(other.spec_seq()),
                ensures
                    pq@.len() == self@.len() + other@.len(),
                    pq@.to_multiset() =~= self@.to_multiset().add(other@.to_multiset()),
                    Self::spec_sorted(pq.spec_seq());

            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq@.len() == seq@.len(),
                    pq@.to_multiset() =~= seq@.to_multiset(),
                    Self::spec_sorted(pq.spec_seq());

            fn size(&self) -> (n: usize)
                ensures n as int == self.spec_size();

            fn is_empty(&self) -> (b: bool)
                ensures b == (self.spec_size() == 0);

            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
                requires obeys_feq_clone::<T>(),
                ensures seq@ =~= self@;

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + elements@.len() <= usize::MAX as int,
                    Self::spec_sorted(self.spec_seq()),
                ensures
                    pq@.len() == self@.len() + elements@.len(),
                    pq@.to_multiset() =~= self@.to_multiset().add(elements@.to_multiset()),
                    Self::spec_sorted(pq.spec_seq());

            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    Self::spec_sorted(self.spec_seq()),
                ensures
                    sorted@.len() == self@.len(),
                    Self::spec_sorted(sorted.seq@);

            fn find_max(&self) -> (max_elem: Option<&T>)
                ensures
                    self@.len() == 0 ==> max_elem.is_none(),
                    self@.len() > 0 ==> max_elem.is_some(),
                    self@.len() > 0 ==> max_elem.unwrap()@ == self@[self@.len() - 1];

            fn delete_max(&self) -> (max_and_rest: (Self, Option<T>))
                requires
                    obeys_feq_clone::<T>(),
                    Self::spec_sorted(self.spec_seq()),
                ensures
                    self@.len() > 0 ==> max_and_rest.1.is_some(),
                    self@.len() > 0 ==> max_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> max_and_rest.1.is_none(),
                    self@.len() == 0 ==> max_and_rest.0@.len() == self@.len(),
                    self@.len() > 0 ==> self@.to_multiset() =~=
                        max_and_rest.0@.to_multiset().insert(max_and_rest.1.unwrap()@),
                    Self::spec_sorted(max_and_rest.0.spec_seq());

            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq@.len() == vec@.len(),
                    pq@.to_multiset() =~= vec@.map(|_i: int, t: T| t@).to_multiset(),
                    Self::spec_sorted(pq.spec_seq());

            fn to_vec(&self) -> (v: Vec<T>)
                requires obeys_feq_clone::<T>(),
                ensures v@.len() == self@.len();

            fn to_sorted_vec(&self) -> (v: Vec<T>)
                requires
                    obeys_feq_clone::<T>(),
                    Self::spec_sorted(self.spec_seq()),
                ensures
                    v@.len() == self@.len(),
                    Self::spec_sorted(v@);

            fn is_sorted(&self) -> (sorted: bool)
                ensures
                    sorted == Self::spec_sorted(self.spec_seq()),
                    self@.len() <= 1 ==> sorted;
        }

// 9. impls
        impl<T: StT + Ord + TotalOrder> SortedListPQTrait<T> for SortedListPQ<T> {
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

            /// Pushing an element >= the last onto a sorted seq preserves sorted.
            proof fn lemma_push_preserves_sorted(s: Seq<T>, x: T) {
                assert forall|a: int, b: int|
                    0 <= a < b < s.push(x).len() implies
                    #[trigger] TotalOrder::le(s.push(x)[a], s.push(x)[b])
                by {
                    if b == s.len() as int && a < s.len() as int - 1 {
                        TotalOrder::transitive(s[a], s.last(), x);
                    }
                }
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn empty() -> (pq: Self) {
                let pq = SortedListPQ {
                    elements: ArraySeqStPerS::empty(),
                };
                proof {
                    assert(pq@ =~= Seq::<T::V>::empty());
                    assert(Seq::<T::V>::empty().to_multiset() =~= Multiset::<T::V>::empty());
                }
                pq
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn singleton(element: T) -> (pq: Self) {
                let pq = SortedListPQ {
                    elements: ArraySeqStPerS::singleton(element),
                };
                proof {
                    assert(pq@ =~= Seq::<T::V>::empty().push(element@));
                }
                pq
            }

            /// APAS Work Θ(1), Span Θ(1) — head of sorted list.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(0))
                }
            }

            /// APAS Work Θ(n), Span Θ(n).
            fn insert(&self, element: T) -> (pq: Self) {
                let n = self.elements.length();

                // Find insertion position via TotalOrder::cmp for spec-level ordering.
                let mut insert_pos: usize = n;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        insert_pos <= n,
                        insert_pos < n ==> (insert_pos < i),
                        insert_pos < n as int ==>
                            TotalOrder::le(
                                element, self.elements.seq@[insert_pos as int]),
                        forall|k: int| #![auto] 0 <= k < i as int
                            && k < insert_pos as int ==>
                            TotalOrder::le(self.elements.seq@[k], element),
                {
                    if insert_pos == n {
                        match TotalOrder::cmp(
                            &element, self.elements.nth(i))
                        {
                            core::cmp::Ordering::Less => {
                                insert_pos = i;
                            }
                            core::cmp::Ordering::Equal => {
                                proof { TotalOrder::reflexive(element); }
                                insert_pos = i;
                            }
                            core::cmp::Ordering::Greater => {}
                        }
                    }
                }

                // Build prefix ++ [element] ++ suffix via subseq_copy + append.
                let prefix = self.elements.subseq_copy(0, insert_pos);
                let suffix = self.elements.subseq_copy(insert_pos, n - insert_pos);
                let ghost element_view = element@;
                let ghost raw_element = element;
                let elem_seq = ArraySeqStPerS::singleton(element);
                let prefix_elem = ArraySeqStPerS::append(&prefix, &elem_seq);
                let new_elements = ArraySeqStPerS::append(&prefix_elem, &suffix);

                let pq = SortedListPQ { elements: new_elements };
                proof {
                    let target: Seq<T::V> = Seq::new((n + 1) as nat, |j: int|
                        if j < insert_pos as int { self@[j] }
                        else if j == insert_pos as int { element_view }
                        else { self@[j - 1] });
                    // Chain spec_index equalities through append/subseq_copy.
                    assert forall|j: int| 0 <= j < (n + 1) as int implies
                        #[trigger] pq@[j] == target[j]
                    by {
                        if j < insert_pos as int {
                            assert(prefix.spec_index(j)
                                == self.elements.spec_index(j));
                            assert(prefix_elem.spec_index(j) == prefix.seq@[j]);
                            assert(new_elements.spec_index(j)
                                == prefix_elem.seq@[j]);
                        } else if j == insert_pos as int {
                            assert(new_elements.spec_index(j)
                                == prefix_elem.seq@[j]);
                            assert(prefix_elem.spec_index(
                                prefix.seq@.len() as int + 0)
                                == elem_seq.seq@[0]);
                        } else {
                            let si = j - (insert_pos as int + 1);
                            assert(suffix.spec_index(si)
                                == self.elements.spec_index(
                                    insert_pos as int + si));
                            assert(new_elements.spec_index(
                                prefix_elem.seq@.len() as int + si)
                                == suffix.seq@[si]);
                        }
                    }
                    assert(pq@ =~= target);
                    // Multiset: target = prefix_view ++ [element_view] ++ suffix_view.
                    let pv = self@.take(insert_pos as int);
                    let sv = self@.subrange(insert_pos as int, n as int);
                    assert(self@ =~= pv + sv);
                    assert(target =~= pv
                        + Seq::<T::V>::empty().push(element_view) + sv);
                    vstd::seq_lib::lemma_multiset_commutative(
                        pv + Seq::<T::V>::empty().push(element_view), sv);
                    vstd::seq_lib::lemma_multiset_commutative(
                        pv, Seq::<T::V>::empty().push(element_view));
                    vstd::seq_lib::lemma_multiset_commutative(pv, sv);
                    // Sorted: result is prefix ++ [element] ++ suffix.
                    assert(elem_seq.seq@[0] == raw_element);
                    assert forall|a: int, b: int|
                        0 <= a < b < new_elements.seq@.len() implies
                        #[trigger] TotalOrder::le(
                            new_elements.seq@[a], new_elements.seq@[b])
                    by {
                        // Establish raw-level position mappings.
                        if a < insert_pos as int {
                            assert(prefix.spec_index(a)
                                == self.elements.spec_index(a));
                            assert(prefix_elem.spec_index(a)
                                == prefix.seq@[a]);
                            assert(new_elements.spec_index(a)
                                == prefix_elem.seq@[a]);
                        } else if a == insert_pos as int {
                            assert(new_elements.spec_index(a)
                                == prefix_elem.seq@[a]);
                            assert(prefix_elem.spec_index(
                                prefix.seq@.len() as int + 0)
                                == elem_seq.seq@[0]);
                        } else {
                            let sa = a - (insert_pos as int + 1);
                            assert(suffix.spec_index(sa)
                                == self.elements.spec_index(
                                    insert_pos as int + sa));
                            assert(new_elements.spec_index(
                                prefix_elem.seq@.len() as int + sa)
                                == suffix.seq@[sa]);
                        }
                        if b > insert_pos as int {
                            let sb = b - (insert_pos as int + 1);
                            assert(suffix.spec_index(sb)
                                == self.elements.spec_index(
                                    insert_pos as int + sb));
                            assert(new_elements.spec_index(
                                prefix_elem.seq@.len() as int + sb)
                                == suffix.seq@[sb]);
                        } else if b == insert_pos as int {
                            assert(new_elements.spec_index(b)
                                == prefix_elem.seq@[b]);
                            assert(prefix_elem.spec_index(
                                prefix.seq@.len() as int + 0)
                                == elem_seq.seq@[0]);
                        } else {
                            assert(prefix.spec_index(b)
                                == self.elements.spec_index(b));
                            assert(prefix_elem.spec_index(b)
                                == prefix.seq@[b]);
                            assert(new_elements.spec_index(b)
                                == prefix_elem.seq@[b]);
                        }
                        // Element-to-suffix needs transitivity.
                        if a == insert_pos as int
                            && b > insert_pos as int
                        {
                            let sb = b - (insert_pos as int + 1);
                            if sb > 0 {
                                TotalOrder::transitive(
                                    raw_element,
                                    self.elements.seq@[insert_pos as int],
                                    self.elements.seq@[
                                        insert_pos as int + sb]);
                            }
                        }
                    }
                }
                pq
            }

            /// APAS Work Θ(1), actual Work Θ(n) — rebuilds without first element.
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let min_element = self.elements.nth(0).clone();
                proof {
                    assert(cloned(self.elements.seq@[0], min_element));
                    axiom_cloned_implies_eq_owned(self.elements.seq@[0], min_element);
                }
                let new_elements = self.elements.subseq_copy(1, n - 1);
                let new_pq = SortedListPQ { elements: new_elements };
                proof {
                    assert forall|i: int| 0 <= i < (n - 1) as int implies
                        #[trigger] new_pq@[i] == self@[i + 1]
                    by {
                        assert(new_elements.spec_index(i)
                            == self.elements.spec_index(1 + i));
                    }
                    assert(new_pq@ =~= self@.subrange(1, n as int));
                    assert(self@.take(1) =~= Seq::<T::V>::empty().push(min_element@));
                    assert(self@ =~= self@.take(1) + self@.subrange(1, n as int));
                    vstd::seq_lib::lemma_multiset_commutative(
                        self@.take(1), self@.subrange(1, n as int));
                    // Sorted: subsequence of sorted seq is sorted.
                    assert forall|i: int, j: int|
                        0 <= i < j < new_elements.seq@.len() implies
                        #[trigger] TotalOrder::le(
                            new_elements.seq@[i], new_elements.seq@[j])
                    by {
                        assert(new_elements.spec_index(i)
                            == self.elements.spec_index(1 + i));
                        assert(new_elements.spec_index(j)
                            == self.elements.spec_index(1 + j));
                    }
                }
                (new_pq, Some(min_element))
            }

            /// APAS Work Θ(m+n), Span Θ(m+n) — merge two sorted sequences.
            fn meld(&self, other: &Self) -> (pq: Self) {
                let n = self.elements.length();
                let m = other.elements.length();
                let mut result = ArraySeqStPerS::empty();
                let mut i: usize = 0;
                let mut j: usize = 0;

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n && j < m
                    invariant
                        n == self.elements@.len(),
                        m == other.elements@.len(),
                        i <= n, j <= m,
                        result@.len() == (i + j) as int,
                        result@.to_multiset() =~=
                            self@.take(i as int).to_multiset().add(
                                other@.take(j as int).to_multiset()),
                        Self::spec_sorted(result.seq@),
                        result.seq@.len() > 0 && i < n ==>
                            TotalOrder::le(result.seq@.last(),
                                self.elements.seq@[i as int]),
                        result.seq@.len() > 0 && j < m ==>
                            TotalOrder::le(result.seq@.last(),
                                other.elements.seq@[j as int]),
                    decreases (n - i) + (m - j),
                {
                    let ghost old_result_seq = result.seq@;
                    let ghost old_result_view = result@;
                    let si = self.elements.nth(i);
                    let oj = other.elements.nth(j);
                    match TotalOrder::cmp(si, oj) {
                        core::cmp::Ordering::Less
                        | core::cmp::Ordering::Equal => {
                            proof { TotalOrder::reflexive(*si); }
                            let single_seq =
                                ArraySeqStPerS::singleton(si.clone());
                            result = ArraySeqStPerS::append(
                                &result, &single_seq);
                            proof {
                                assert(cloned(
                                    self.elements.seq@[i as int],
                                    single_seq.seq@[0]));
                                axiom_cloned_implies_eq_owned(
                                    self.elements.seq@[i as int],
                                    single_seq.seq@[0]);
                                // Raw push for sorted proof.
                                assert forall|k: int| #![auto]
                                    0 <= k < result.seq@.len() implies
                                    result.seq@[k] == old_result_seq.push(
                                        self.elements.seq@[i as int])[k]
                                by {
                                    if k < old_result_seq.len() as int {
                                        assert(result.spec_index(k)
                                            == old_result_seq[k]);
                                    } else {
                                        assert(result.spec_index(
                                            old_result_seq.len() as int + 0)
                                            == single_seq.seq@[0]);
                                    }
                                }
                                assert(result.seq@ =~= old_result_seq.push(
                                    self.elements.seq@[i as int]));
                                Self::lemma_push_preserves_sorted(
                                    old_result_seq,
                                    self.elements.seq@[i as int]);
                                assert(result.seq@.last()
                                    == self.elements.seq@[i as int]);
                                // View push for multiset proof.
                                assert forall|k: int|
                                    0 <= k < result@.len() implies
                                    #[trigger] result@[k]
                                        == old_result_view.push(
                                            self@[i as int])[k]
                                by {
                                    if k < old_result_view.len() {
                                        assert(result.spec_index(k)
                                            == old_result_seq[k]);
                                    } else {
                                        assert(result.spec_index(
                                            old_result_seq.len() as int + 0)
                                            == single_seq.seq@[0]);
                                    }
                                }
                                assert(result@ =~= old_result_view.push(
                                    self@[i as int]));
                                self@.lemma_take_succ_push(i as int);
                            }
                            i = i + 1;
                        }
                        core::cmp::Ordering::Greater => {
                            let single_seq =
                                ArraySeqStPerS::singleton(oj.clone());
                            result = ArraySeqStPerS::append(
                                &result, &single_seq);
                            proof {
                                assert(cloned(
                                    other.elements.seq@[j as int],
                                    single_seq.seq@[0]));
                                axiom_cloned_implies_eq_owned(
                                    other.elements.seq@[j as int],
                                    single_seq.seq@[0]);
                                // Raw push for sorted proof.
                                assert forall|k: int| #![auto]
                                    0 <= k < result.seq@.len() implies
                                    result.seq@[k] == old_result_seq.push(
                                        other.elements.seq@[j as int])[k]
                                by {
                                    if k < old_result_seq.len() as int {
                                        assert(result.spec_index(k)
                                            == old_result_seq[k]);
                                    } else {
                                        assert(result.spec_index(
                                            old_result_seq.len() as int + 0)
                                            == single_seq.seq@[0]);
                                    }
                                }
                                assert(result.seq@ =~= old_result_seq.push(
                                    other.elements.seq@[j as int]));
                                Self::lemma_push_preserves_sorted(
                                    old_result_seq,
                                    other.elements.seq@[j as int]);
                                assert(result.seq@.last()
                                    == other.elements.seq@[j as int]);
                                // View push for multiset proof.
                                assert forall|k: int|
                                    0 <= k < result@.len() implies
                                    #[trigger] result@[k]
                                        == old_result_view.push(
                                            other@[j as int])[k]
                                by {
                                    if k < old_result_view.len() {
                                        assert(result.spec_index(k)
                                            == old_result_seq[k]);
                                    } else {
                                        assert(result.spec_index(
                                            old_result_seq.len() as int + 0)
                                            == single_seq.seq@[0]);
                                    }
                                }
                                assert(result@ =~= old_result_view.push(
                                    other@[j as int]));
                                other@.lemma_take_succ_push(j as int);
                            }
                            j = j + 1;
                        }
                    }
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n
                    invariant
                        n == self.elements@.len(),
                        m == other.elements@.len(),
                        i <= n, j <= m,
                        result@.len() == (i + j) as int,
                        result@.to_multiset() =~=
                            self@.take(i as int).to_multiset().add(
                                other@.take(j as int).to_multiset()),
                        Self::spec_sorted(result.seq@),
                        result.seq@.len() > 0 && i < n ==>
                            TotalOrder::le(result.seq@.last(),
                                self.elements.seq@[i as int]),
                        i >= n || j >= m,
                        j < m && result.seq@.len() > 0 ==>
                            TotalOrder::le(result.seq@.last(),
                                other.elements.seq@[j as int]),
                    decreases n - i,
                {
                    let ghost old_result_seq = result.seq@;
                    let ghost old_result_view = result@;
                    let single_seq = ArraySeqStPerS::singleton(
                        self.elements.nth(i).clone());
                    result = ArraySeqStPerS::append(
                        &result, &single_seq);
                    proof {
                        assert(cloned(self.elements.seq@[i as int],
                            single_seq.seq@[0]));
                        axiom_cloned_implies_eq_owned(
                            self.elements.seq@[i as int],
                            single_seq.seq@[0]);
                        assert forall|k: int| #![auto]
                            0 <= k < result.seq@.len() implies
                            result.seq@[k] == old_result_seq.push(
                                self.elements.seq@[i as int])[k]
                        by {
                            if k < old_result_seq.len() as int {
                                assert(result.spec_index(k)
                                    == old_result_seq[k]);
                            } else {
                                assert(result.spec_index(
                                    old_result_seq.len() as int + 0)
                                    == single_seq.seq@[0]);
                            }
                        }
                        assert(result.seq@ =~= old_result_seq.push(
                            self.elements.seq@[i as int]));
                        Self::lemma_push_preserves_sorted(
                            old_result_seq,
                            self.elements.seq@[i as int]);
                        assert(result.seq@.last()
                            == self.elements.seq@[i as int]);
                        assert forall|k: int|
                            0 <= k < result@.len() implies
                            #[trigger] result@[k]
                                == old_result_view.push(
                                    self@[i as int])[k]
                        by {
                            if k < old_result_view.len() {
                                assert(result.spec_index(k)
                                    == old_result_seq[k]);
                            } else {
                                assert(result.spec_index(
                                    old_result_seq.len() as int + 0)
                                    == single_seq.seq@[0]);
                            }
                        }
                        assert(result@ =~= old_result_view.push(
                            self@[i as int]));
                        self@.lemma_take_succ_push(i as int);
                    }
                    i = i + 1;
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while j < m
                    invariant
                        n == self.elements@.len(),
                        m == other.elements@.len(),
                        i <= n, j <= m,
                        result@.len() == (i + j) as int,
                        result@.to_multiset() =~=
                            self@.take(i as int).to_multiset().add(
                                other@.take(j as int).to_multiset()),
                        Self::spec_sorted(result.seq@),
                        result.seq@.len() > 0 && j < m ==>
                            TotalOrder::le(result.seq@.last(),
                                other.elements.seq@[j as int]),
                    decreases m - j,
                {
                    let ghost old_result_seq = result.seq@;
                    let ghost old_result_view = result@;
                    let single_seq = ArraySeqStPerS::singleton(
                        other.elements.nth(j).clone());
                    result = ArraySeqStPerS::append(
                        &result, &single_seq);
                    proof {
                        assert(cloned(other.elements.seq@[j as int],
                            single_seq.seq@[0]));
                        axiom_cloned_implies_eq_owned(
                            other.elements.seq@[j as int],
                            single_seq.seq@[0]);
                        assert forall|k: int| #![auto]
                            0 <= k < result.seq@.len() implies
                            result.seq@[k] == old_result_seq.push(
                                other.elements.seq@[j as int])[k]
                        by {
                            if k < old_result_seq.len() as int {
                                assert(result.spec_index(k)
                                    == old_result_seq[k]);
                            } else {
                                assert(result.spec_index(
                                    old_result_seq.len() as int + 0)
                                    == single_seq.seq@[0]);
                            }
                        }
                        assert(result.seq@ =~= old_result_seq.push(
                            other.elements.seq@[j as int]));
                        Self::lemma_push_preserves_sorted(
                            old_result_seq,
                            other.elements.seq@[j as int]);
                        assert(result.seq@.last()
                            == other.elements.seq@[j as int]);
                        assert forall|k: int|
                            0 <= k < result@.len() implies
                            #[trigger] result@[k]
                                == old_result_view.push(
                                    other@[j as int])[k]
                        by {
                            if k < old_result_view.len() {
                                assert(result.spec_index(k)
                                    == old_result_seq[k]);
                            } else {
                                assert(result.spec_index(
                                    old_result_seq.len() as int + 0)
                                    == single_seq.seq@[0]);
                            }
                        }
                        assert(result@ =~= old_result_view.push(
                            other@[j as int]));
                        other@.lemma_take_succ_push(j as int);
                    }
                    j = j + 1;
                }

                let pq = SortedListPQ { elements: result };
                proof {
                    assert(self@.take(n as int) =~= self@);
                    assert(other@.take(m as int) =~= other@);
                }
                pq
            }

            /// APAS Work Θ(n log n), actual Work Θ(n²) — repeated insert.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                let n = seq.length();
                let mut pq = Self::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == seq@.len(),
                        pq@.len() == i as int,
                        pq@.to_multiset() =~= seq@.take(i as int).to_multiset(),
                        Self::spec_sorted(pq.spec_seq()),
                {
                    let elem = seq.nth(i).clone();
                    proof {
                        assert(cloned(seq.seq@[i as int], elem));
                        axiom_cloned_implies_eq_owned(seq.seq@[i as int], elem);
                        // elem@ == seq@[i].
                    }
                    let ghost old_pq_view = pq@;
                    pq = pq.insert(elem);
                    proof {
                        // pq@.to_multiset() =~= old_pq.ms.insert(elem@)
                        //                    =~= seq@.take(i).ms.insert(seq@[i])
                        //                    =~= seq@.take(i+1).ms
                        seq@.lemma_take_succ_push(i as int);
                    }
                }
                pq
            }

            fn size(&self) -> (n: usize) { self.elements.length() }

            fn is_empty(&self) -> (b: bool) { self.elements.length() == 0 }

            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>) { self.elements.clone() }

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            /// Already sorted — just copy the backing sequence.
            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>) {
                let n = self.elements.length();
                let sorted = self.elements.subseq_copy(0, n);
                proof {
                    assert forall|i: int| 0 <= i < sorted@.len() implies
                        #[trigger] sorted@[i] == self@[i]
                    by {
                        assert(sorted.spec_index(i)
                            == self.elements.spec_index(0 + i));
                    }
                    assert(sorted@ =~= self@);
                    assert forall|i: int, j: int|
                        0 <= i < j < sorted.seq@.len() implies
                        #[trigger] TotalOrder::le(
                            sorted.seq@[i], sorted.seq@[j])
                    by {
                        assert(sorted.spec_index(i)
                            == self.elements.spec_index(0 + i));
                        assert(sorted.spec_index(j)
                            == self.elements.spec_index(0 + j));
                    }
                }
                sorted
            }

            fn find_max(&self) -> (max_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(self.elements.length() - 1))
                }
            }

            fn delete_max(&self) -> (max_and_rest: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let max_element = self.elements.nth(n - 1).clone();
                proof {
                    assert(cloned(self.elements.seq@[(n - 1) as int], max_element));
                    axiom_cloned_implies_eq_owned(
                        self.elements.seq@[(n - 1) as int], max_element);
                }
                let new_elements = self.elements.subseq_copy(0, n - 1);
                let new_pq = SortedListPQ { elements: new_elements };
                proof {
                    assert forall|i: int| 0 <= i < (n - 1) as int implies
                        #[trigger] new_pq@[i] == self@[i]
                    by {
                        assert(new_elements.spec_index(i)
                            == self.elements.spec_index(0 + i));
                    }
                    assert(new_pq@ =~= self@.take((n - 1) as int));
                    assert(self@.take((n - 1) as int)
                        =~= self@.subrange(0, (n - 1) as int));
                    let sv = self@.subrange((n - 1) as int, n as int);
                    assert(sv =~= Seq::<T::V>::empty().push(max_element@));
                    assert(self@ =~= self@.take((n - 1) as int) + sv);
                    vstd::seq_lib::lemma_multiset_commutative(
                        self@.take((n - 1) as int), sv);
                    // Sorted: prefix of sorted seq is sorted.
                    assert forall|i: int, j: int|
                        0 <= i < j < new_elements.seq@.len() implies
                        #[trigger] TotalOrder::le(
                            new_elements.seq@[i], new_elements.seq@[j])
                    by {
                        assert(new_elements.spec_index(i)
                            == self.elements.spec_index(0 + i));
                        assert(new_elements.spec_index(j)
                            == self.elements.spec_index(0 + j));
                    }
                }
                (new_pq, Some(max_element))
            }

            fn from_vec(vec: Vec<T>) -> Self {
                let ghost vec_view = vec@;
                let seq = ArraySeqStPerS::from_vec(vec);
                let pq = Self::from_seq(&seq);
                proof {
                    // seq@ =~= vec@.map(view): chain through spec_index.
                    assert(seq@ =~= vec_view.map(|_i: int, t: T| t@)) by {
                        assert forall|i: int| #![auto] 0 <= i < seq@.len()
                        implies seq@[i] == vec_view.map(|_i: int, t: T| t@)[i] by {
                            assert(seq.spec_index(i) == vec_view[i]);
                        };
                    };
                }
                pq
            }

            fn to_vec(&self) -> Vec<T> {
                let n = self.elements.length();
                let mut result: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        result@.len() == i as int,
                {
                    result.push(self.elements.nth(i).clone());
                }
                result
            }

            fn to_sorted_vec(&self) -> Vec<T> {
                let n = self.elements.length();
                let mut v: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        v@.len() == i as int,
                        forall|k: int| #![auto] 0 <= k < i as int ==>
                            v@[k] == self.elements.seq@[k],
                {
                    let elem = self.elements.nth(i).clone();
                    proof {
                        assert(cloned(self.elements.seq@[i as int], elem));
                        axiom_cloned_implies_eq_owned(
                            self.elements.seq@[i as int], elem);
                    }
                    v.push(elem);
                }
                proof {
                    assert forall|i: int, j: int|
                        0 <= i < j < v@.len() implies
                        #[trigger] TotalOrder::le(v@[i], v@[j])
                    by {}
                }
                v
            }

            fn is_sorted(&self) -> (sorted: bool) {
                let n = self.elements.length();
                if n <= 1 {
                    return true;
                }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                        n > 1,
                        forall|a: int, b: int| 0 <= a < b < i as int ==>
                            #[trigger] TotalOrder::le(
                                self.elements.seq@[a], self.elements.seq@[b]),
                {
                    let prev = self.elements.nth(i - 1);
                    let curr = self.elements.nth(i);
                    match TotalOrder::cmp(prev, curr) {
                        core::cmp::Ordering::Greater => {
                            proof {
                                // le(curr, prev) && prev != curr.
                                // If le(prev, curr) too, antisymmetric gives
                                // prev == curr — contradiction. So !le(prev, curr).
                                assert(!TotalOrder::le(*prev, *curr)) by {
                                    if TotalOrder::le(*prev, *curr) {
                                        TotalOrder::antisymmetric(*prev, *curr);
                                    }
                                };
                            }
                            return false;
                        }
                        core::cmp::Ordering::Equal => {
                            proof {
                                // prev == curr, so le(prev, curr) by reflexive.
                                TotalOrder::reflexive(*prev);
                                assert(TotalOrder::le(
                                    self.elements.seq@[(i - 1) as int],
                                    self.elements.seq@[i as int]));
                                assert forall|a: int, b: int|
                                    0 <= a < b < (i + 1) as int implies
                                    #[trigger] TotalOrder::le(
                                        self.elements.seq@[a], self.elements.seq@[b])
                                by {
                                    if b == i as int && a < (i - 1) as int {
                                        TotalOrder::transitive(
                                            self.elements.seq@[a],
                                            self.elements.seq@[(i - 1) as int],
                                            self.elements.seq@[i as int]);
                                    }
                                }
                            }
                        }
                        core::cmp::Ordering::Less => {
                            proof {
                                // le(prev, curr) directly.
                                assert forall|a: int, b: int|
                                    0 <= a < b < (i + 1) as int implies
                                    #[trigger] TotalOrder::le(
                                        self.elements.seq@[a], self.elements.seq@[b])
                                by {
                                    if b == i as int && a < (i - 1) as int {
                                        TotalOrder::transitive(
                                            self.elements.seq@[a],
                                            self.elements.seq@[(i - 1) as int],
                                            self.elements.seq@[i as int]);
                                    }
                                }
                            }
                        }
                    }
                }
                true
            }
        }

        impl<T: StT + Ord + TotalOrder> Default for SortedListPQ<T> {
            fn default() -> Self { Self::empty() }
        }

// 11. derive impls in verus!
        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for SortedListPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord + TotalOrder> Clone for SortedListPQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned@ == self@
            {
                let cloned = SortedListPQ { elements: self.elements.clone() };
                proof {
                    assume(obeys_feq_clone::<T>());
                    lemma_seq_map_cloned_view_eq(
                        self.elements.seq@,
                        cloned.elements.seq@,
                    );
                }
                cloned
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for SortedListPQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self@ == other@)
            {
                let equal = self.elements == other.elements;
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for SortedListPQ<T> {}
    }

// 12. macros
    #[macro_export]
    macro_rules! SortedListPQLit {
        () => {
            $crate::Chap45::SortedListPQ::SortedListPQ::SortedListPQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let mut pq = $crate::Chap45::SortedListPQ::SortedListPQ::SortedListPQ::empty();
            $(
                pq = pq.insert($x);
            )*
            pq
        }};
    }

// 13. derive impls outside verus!
    impl<T: StT + Ord + TotalOrder + std::fmt::Debug> std::fmt::Debug for SortedListPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SortedListPQ").field("elements", &self.elements).finish()
        }
    }

    impl<T: StT + Ord + TotalOrder> Display for SortedListPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "SortedListPQ[")?;
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
