//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 45: Priority Queue implementation using Unsorted List

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod UnsortedListPQ {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use vstd::multiset::Multiset;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::accept::accept;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::multiset::group_multiset_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::std_specs::vec::group_vec_axioms,
};

    //		Section 4. type definitions


        #[verifier::reject_recursive_types(T)]
        pub struct UnsortedListPQ<T: StT + Ord + TotalOrder> {
            pub elements: ArraySeqStPerS<T>,
        }

    //		Section 5. view impls


        impl<T: StT + Ord + TotalOrder> View for UnsortedListPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

    //		Section 7. proof fns/broadcast groups


        proof fn _unsorted_list_pq_verified() {}

    //		Section 8. traits


        /// Meldable Priority Queue ADT (Data Type 45.1) using unsorted list.
        pub trait UnsortedListPQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Seq<T::V>> {
            spec fn spec_unsortedlistpq_wf(&self) -> bool;
            spec fn spec_size(self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;
            spec fn spec_sorted(s: Seq<T>) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn empty() -> (pq: Self)
                ensures
                    pq@.len() == 0,
                    pq@.to_multiset() =~= Multiset::empty(),
                    pq.spec_unsortedlistpq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn singleton(element: T) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq@.len() == 1,
                    pq@.to_multiset() =~= Multiset::empty().insert(element@),
                    pq.spec_unsortedlistpq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn find_min(&self) -> (min_elem: Option<&T>)
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some(),
                    self@.len() > 0 ==> forall|j: int|
                        0 <= j < self.spec_seq().len() ==>
                            #[trigger] TotalOrder::le(*min_elem.unwrap(), self.spec_seq()[j]);

            /// - Alg Analysis: APAS (Ch45 cost table): Work O(1), Span O(1)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: persistent array append copies entire array
            fn insert(&self, element: T) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + 1 <= usize::MAX as int,
                ensures
                    pq@.len() == self@.len() + 1,
                    pq@.to_multiset() =~= self@.to_multiset().insert(element@);

            /// - Alg Analysis: APAS (Ch45 cost table): Work O(n), Span O(n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires obeys_feq_clone::<T>(),
                ensures
                    self@.len() > 0 ==> min_and_rest.1.is_some(),
                    self@.len() > 0 ==> min_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> min_and_rest.1.is_none(),
                    self@.len() == 0 ==> min_and_rest.0@.len() == self@.len(),
                    self@.len() > 0 ==> forall|j: int|
                        0 <= j < self.spec_seq().len() ==>
                            #[trigger] TotalOrder::le(min_and_rest.1.unwrap(), self.spec_seq()[j]),
                    self@.len() > 0 ==> forall|j: int|
                        0 <= j < min_and_rest.0.spec_seq().len() ==>
                            #[trigger] TotalOrder::le(min_and_rest.1.unwrap(), min_and_rest.0.spec_seq()[j]),
                    self@.len() > 0 ==> exists|k: int|
                        #![trigger self.spec_seq()[k]]
                        0 <= k < self.spec_seq().len() &&
                        min_and_rest.1.unwrap() == self.spec_seq()[k],
                    self@.len() > 0 ==> self@.to_multiset() =~=
                        min_and_rest.0@.to_multiset().insert(min_and_rest.1.unwrap()@);

            /// - Alg Analysis: APAS (Ch45 cost table): Work O(m + n), Span O(m + n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m + n), Span O(m + n)
            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + other@.len() <= usize::MAX as int,
                ensures
                    pq@.len() == self@.len() + other@.len(),
                    pq@.to_multiset() =~= self@.to_multiset().add(other@.to_multiset());

            /// - Alg Analysis: APAS (Ch45 cost table): Work O(n), Span O(n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures pq@ =~= seq@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn size(&self) -> (n: usize)
                ensures n as int == self.spec_size();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> (b: bool)
                ensures b == (self.spec_size() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
                requires obeys_feq_clone::<T>(),
                ensures seq@ =~= self@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m + n), Span O(m + n)
            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + elements@.len() <= usize::MAX as int,
                ensures
                    pq@.len() == self@.len() + elements@.len(),
                    pq@.to_multiset() =~= self@.to_multiset().add(elements@.to_multiset());

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() <= usize::MAX as int,
                ensures
                    sorted@.len() == self@.len(),
                    Self::spec_sorted(sorted.seq@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures pq@ =~= vec@.map(|_i: int, t: T| t@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (v: Vec<T>)
                requires obeys_feq_clone::<T>(),
                ensures v@.len() == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
            fn to_sorted_vec(&self) -> (v: Vec<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() <= usize::MAX as int,
                ensures
                    v@.len() == self@.len(),
                    Self::spec_sorted(v@);
        }

    //		Section 9. impls


        impl<T: StT + Ord + TotalOrder> UnsortedListPQTrait<T> for UnsortedListPQ<T> {
            open spec fn spec_unsortedlistpq_wf(&self) -> bool {
                self@.len() <= usize::MAX as int
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

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); constant-time empty construction.
            fn empty() -> (pq: Self) {
                let pq = UnsortedListPQ {
                    elements: ArraySeqStPerS::empty(),
                };
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block (speed hint)
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert (speed hint)
                    assert(pq@ =~= Seq::<T::V>::empty());
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert (speed hint)
                    assert(Seq::<T::V>::empty().to_multiset()
                        =~= Multiset::<T::V>::empty());
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); constant-time singleton construction.
            fn singleton(element: T) -> (pq: Self) {
                let pq = UnsortedListPQ {
                    elements: ArraySeqStPerS::singleton(element),
                };
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(pq@ =~= Seq::<T::V>::empty().push(element@));
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); linear scan over unsorted list.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    return None;
                }
                let n = self.elements.length();
                // Veracity: NEEDED proof block
                let mut min_element = self.elements.nth(0);
                // Veracity: NEEDED proof block
                proof { T::reflexive(*min_element); }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                        n > 0,
                        forall|j: int| 0 <= j < i ==>
                            #[trigger] TotalOrder::le(*min_element, self.elements.seq@[j]),
                {
                    let current = self.elements.nth(i);
                    let c = <T as TotalOrder>::cmp(current, min_element);
                    // Veracity: NEEDED proof block
                    match c {
                        core::cmp::Ordering::Less => {
                            // Veracity: NEEDED proof block
                            proof {
                                let ghost old_min = *min_element;
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(TotalOrder::le(*current, old_min));
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall|j: int| 0 <= j < i implies
                                    #[trigger] TotalOrder::le(*current, self.elements.seq@[j]) by {
                                    T::transitive(*current, old_min, self.elements.seq@[j]);
                                };
                                T::reflexive(*current);
                            }
                            // Veracity: NEEDED proof block
                            min_element = current;
                        }
                        _ => {
                            // Veracity: NEEDED proof block
                            proof {
                                // Equal or Greater: le(min, current) holds.
                                T::total(*min_element, *current);
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(TotalOrder::le(*min_element, self.elements.seq@[i as int]));
                            }
                        }
                    }
                }
                Some(min_element)
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: append copies persistent array.
            fn insert(&self, element: T) -> (pq: Self) {
                // Veracity: NEEDED proof block
                let single_seq = ArraySeqStPerS::singleton(element);
                let pq = UnsortedListPQ {
                    elements: ArraySeqStPerS::append(&self.elements, &single_seq),
                };
                // Veracity: NEEDED proof block
                proof {
                    let sv = Seq::<T::V>::empty().push(element@);
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(pq@ =~= self@ + sv) by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert (speed hint)
                        assert(pq@.len() == self@.len() + sv.len());
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|i: int| 0 <= i < pq@.len()
                        implies #[trigger] pq@[i] == (self@ + sv)[i] by {
                            if i < self@.len() {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(pq.elements.spec_index(i)
                                    == self.elements.seq@[i]);
                            } else {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(pq.elements.spec_index(
                                    self.elements.seq@.len() as int
                                    + (i - self@.len()))
                                    == single_seq.seq@[i - self@.len()]);
                            }
                        };
                    };
                    vstd::seq_lib::lemma_multiset_commutative(self@, sv);
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); linear scan for min, then rebuild without it.
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    // Veracity: NEEDED proof block
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let mut min_element = self.elements.nth(0);
                let mut min_index: usize = 0;
                // Veracity: NEEDED proof block
                proof { T::reflexive(*min_element); }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                        n > 0,
                        (min_index as int) < n,
                        *min_element == self.elements.seq@[min_index as int],
                        forall|j: int| 0 <= j < i ==>
                            // Veracity: NEEDED proof block
                            #[trigger] TotalOrder::le(*min_element, self.elements.seq@[j]),
                {
                    let current = self.elements.nth(i);
                    let c = <T as TotalOrder>::cmp(current, min_element);
                    match c {
                        core::cmp::Ordering::Less => {
                            // Veracity: NEEDED proof block
                            proof {
                                let ghost old_min = *min_element;
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(TotalOrder::le(*current, old_min));
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert forall|j: int| 0 <= j < i implies
                                    #[trigger] TotalOrder::le(*current, self.elements.seq@[j]) by {
                                    T::transitive(*current, old_min, self.elements.seq@[j]);
                                // Veracity: NEEDED proof block
                                };
                                T::reflexive(*current);
                            }
                            min_element = current;
                            min_index = i;
                        }
                        _ => {
                            // Veracity: NEEDED proof block
                            proof {
                                T::total(*min_element, *current);
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(TotalOrder::le(*min_element, self.elements.seq@[i as int]));
                            }
                        }
                    }
                }

                // Rebuild sequence without the minimum element.
                let mut new_elements = ArraySeqStPerS::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        (min_index as int) < n,
                        forall|j: int| 0 <= j < n ==>
                            #[trigger] TotalOrder::le(
                                *min_element, self.elements.seq@[j]),
                        new_elements@.len() ==
                            i as int - if (min_index as int) < (i as int)
                                { 1int } else { 0int },
                        forall|j: int| 0 <= j < new_elements.seq@.len()
                            ==> TotalOrder::le(
                                *min_element, #[trigger] new_elements.seq@[j]),
                        new_elements@ =~= self@.remove(
                            min_index as int).take(
                                new_elements@.len() as int),
                {
                    // Veracity: NEEDED proof block
                    if i != min_index {
                        let element = self.elements.nth(i);
                        let single_seq = ArraySeqStPerS::singleton(element.clone());
                        let ghost old_ne_seq = new_elements.seq@;
                        let ghost old_ne_view = new_elements@;
                        let ghost old_len = new_elements.seq@.len();
                        new_elements = ArraySeqStPerS::append(
                            &new_elements, &single_seq);
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert (speed hint)
                            assert(cloned(self.elements.seq@[i as int],
                                single_seq.seq@[0]));
                            axiom_cloned_implies_eq_owned(
                                self.elements.seq@[i as int],
                                single_seq.seq@[0]);
                            // Raw: new element equals self's raw element.
                            // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                             assert(new_elements.seq@[old_len as int]
// Veracity: UNNEEDED assert                                 == self.elements.seq@[i as int]) by {
// Veracity: UNNEEDED assert                                 // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                                 assert(new_elements.spec_index(
// Veracity: UNNEEDED assert                                     old_len as int + 0)
// Veracity: UNNEEDED assert                                     == single_seq.seq@[0]);
// Veracity: UNNEEDED assert                             };
                            // Prove le invariant for all new_elements.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|j: int|
                                0 <= j < new_elements.seq@.len() implies
                                TotalOrder::le(
                                    *min_element, #[trigger] new_elements.seq@[j])
                            by {
                                if j < old_len as int {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(new_elements.spec_index(j)
                                        == old_ne_seq[j]);
                                }
                            };
                            // Prove view invariant: new_elements@ extends.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|j: int|
                                0 <= j < new_elements@.len() implies
                                #[trigger] new_elements@[j] == self@.remove(
                                    min_index as int).take(
                                        new_elements@.len() as int)[j]
                            by {
                                if j < old_ne_view.len() as int {
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert
                                    assert(new_elements.spec_index(j)
                                        == old_ne_seq[j]);
                                } else {
                                    // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                                     assert(new_elements.spec_index(
// Veracity: UNNEEDED assert                                         old_len as int + 0)
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                                         == single_seq.seq@[0]);
                                }
                            };
                        }
                    }
                }

                let returned_min = min_element.clone();
                let new_pq = UnsortedListPQ { elements: new_elements };
                // Veracity: NEEDED proof block
                proof {
                    // Clone gives raw T equality.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert (speed hint)
                    assert(cloned(*min_element, returned_min));
                    axiom_cloned_implies_eq(min_element, returned_min);
                    // #6: witness for exists.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(self.spec_seq()[min_index as int]
                        == returned_min);
                    // #5: new_pq@ == self@.remove(min_index).
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(new_pq@ =~= self@.remove(min_index as int));
                    // #5 le: each new_pq element was in self.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < new_pq.spec_seq().len()
                    implies #[trigger] TotalOrder::le(
                        returned_min, new_pq.spec_seq()[j]) by {};
                    // #7: multiset from broadcast to_multiset_remove.
                // Veracity: NEEDED proof block
                }
                (new_pq, Some(returned_min))
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(m+n), Span O(m+n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m+n), Span O(m+n); concatenates two persistent arrays.
            fn meld(&self, other: &Self) -> (pq: Self) {
                let pq = UnsortedListPQ {
                    elements: ArraySeqStPerS::append(&self.elements, &other.elements),
                };
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(pq@ =~= self@ + other@) by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|i: int| 0 <= i < pq@.len()
                        implies #[trigger] pq@[i] == (self@ + other@)[i] by {
                            if i < self@.len() {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(pq.elements.spec_index(i)
                                    == self.elements.seq@[i]);
                            } else {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(pq.elements.spec_index(
                                    self.elements.seq@.len() as int
                                    + (i - self@.len()))
                                    == other.elements.seq@[
                                        i - self@.len()]);
                            }
                        };
                    // Veracity: NEEDED proof block
                    };
                    vstd::seq_lib::lemma_multiset_commutative(
                        self@, other@);
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n); clones persistent array.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                let pq = UnsortedListPQ { elements: seq.clone() };
                // Veracity: NEEDED proof block
                proof {
                    lemma_seq_map_cloned_view_eq(
                        seq.seq@,
                        pq.elements.seq@,
                    );
                }
                pq
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn size(&self) -> (n: usize) { self.elements.length() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> (b: bool) { self.elements.length() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>) { self.elements.clone() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m + n), Span O(m + n)
            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            #[verifier::exec_allows_no_decreases_clause]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>) {
                let mut result = ArraySeqStPerS::empty();
                let mut current_pq = self.clone();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while !current_pq.is_empty()
                    invariant
                        result@.len() + current_pq@.len() == self@.len(),
                        // Veracity: NEEDED proof block
                        self@.len() <= usize::MAX as int,
                        Self::spec_sorted(result.seq@),
                        result.seq@.len() > 0 ==> forall|k: int|
                            0 <= k < current_pq.spec_seq().len() ==>
                                #[trigger] TotalOrder::le(
                                    result.seq@[result.seq@.len() - 1],
                                    current_pq.spec_seq()[k]),
                {
                    let ghost old_result_seq = result.seq@;
                    let ghost old_result_len = result.seq@.len() as int;
                    let (new_pq, min_element) = current_pq.delete_min();
                    if let Some(element) = min_element {
                        // Veracity: NEEDED proof block
                        proof {
                            if old_result_len > 0 {
                                // From membership postcondition: element == current_pq.spec_seq()[k0].
                                let k0 = choose|k: int|
                                    #![trigger current_pq.spec_seq()[k]]
                                    0 <= k < current_pq.spec_seq().len() &&
                                    element == current_pq.spec_seq()[k];
                                // From invariant: result.last() <= current_pq.spec_seq()[k0] = element.
                                // Veracity: NEEDED proof block
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(TotalOrder::le(
                                    old_result_seq[old_result_len - 1],
                                    current_pq.spec_seq()[k0]));
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(TotalOrder::le(
                                    old_result_seq[old_result_len - 1], element));
                            }
                        }
                        let single_seq = ArraySeqStPerS::singleton(element);
                        result = ArraySeqStPerS::append(&result, &single_seq);
                        // Veracity: NEEDED proof block
                        proof {
                            // Connect result elements to pre-append values via spec_index.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|idx: int| 0 <= idx < old_result_len implies
                                #[trigger] old_result_seq[idx] == result.seq@[idx] by {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert
                                assert(result.spec_index(idx) == old_result_seq[idx]);
                            };
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert (speed hint)
                            assert(result.seq@[old_result_len] == element) by {
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(single_seq.spec_index(0) == element);
                                // Veracity: NEEDED assert
                                // Veracity: NEEDED assert (speed hint)
                                assert(result.spec_index(old_result_len)
                                    == single_seq.seq@[0]);
                            };

                            // Prove spec_sorted for extended result.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall|i: int, j: int|
                                0 <= i < j < result.seq@.len()
                            implies
                                #[trigger] TotalOrder::le(result.seq@[i], result.seq@[j])
                            by {
                                if j < old_result_len {
                                    // Both in old result — already sorted.
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(result.seq@[i] == old_result_seq[i]);
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(result.seq@[j] == old_result_seq[j]);
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(TotalOrder::le(old_result_seq[i], old_result_seq[j]));
                                } else {
                                    // j == old_result_len, result[j] == element.
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(result.seq@[i] == old_result_seq[i]);
                                    // Veracity: NEEDED assert
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(result.seq@[j] == element);
                                    if old_result_len > 0 {
                                        if i < old_result_len - 1 {
                                            // Veracity: NEEDED assert
                                            // Veracity: NEEDED assert (speed hint)
                                            assert(TotalOrder::le(
                                                old_result_seq[i],
                                                old_result_seq[old_result_len - 1]));
                                            T::transitive(
                                                old_result_seq[i],
                                                old_result_seq[old_result_len - 1],
                                                element);
                                        }
                                    }
                                // Veracity: NEEDED proof block
                                }
                            };
                        }
                    }
                    current_pq = new_pq;
                }
                result
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn from_vec(vec: Vec<T>) -> Self {
                let ghost vec_view = vec@;
                let seq = ArraySeqStPerS::from_vec(vec);
                let pq = Self::from_seq(&seq);
                // Veracity: NEEDED proof block
                proof {
                    // from_vec gives seq.seq@ =~= vec@ (element-wise + length).
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(seq.seq@ =~= vec_view) by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|i: int| 0 <= i < seq.seq@.len()
                        implies #[trigger] seq.seq@[i] == vec_view[i] by {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(seq.spec_index(i) == vec_view[i]);
                        };
                    };
                    // seq@ = seq.seq@.map(view) =~= vec@.map(view), and pq@ =~= seq@.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert (speed hint)
                    assert(seq@ =~= vec_view.map(|_i: int, t: T| t@)) by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert (speed hint)
                        assert forall|i: int| 0 <= i < seq@.len()
                        implies #[trigger] seq@[i] == vec_view.map(|_i: int, t: T| t@)[i] by {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert (speed hint)
                            assert(seq.seq@[i] == vec_view[i]);
                        };
                    };
                }
                pq
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> Vec<T> {
                let n = self.elements.length();
                let mut result: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        result@.len() == i as int,
                {
                    let elem = self.elements.nth(i).clone();
                    result.push(elem);
                }
                result
            }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
            fn to_sorted_vec(&self) -> Vec<T> {
                let sorted_seq = self.extract_all_sorted();
                let n = sorted_seq.length();
                let mut result: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == sorted_seq@.len(),
                        // Veracity: NEEDED proof block
                        result@.len() == i as int,
                        Self::spec_sorted(sorted_seq.seq@),
                        forall|k: int| 0 <= k < i as int ==>
                            #[trigger] result@[k] == sorted_seq.seq@[k],
                {
                    let elem = sorted_seq.nth(i).clone();
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert (speed hint)
                        assert(cloned(sorted_seq.seq@[i as int], elem));
                        axiom_cloned_implies_eq_owned(
                            sorted_seq.seq@[i as int], elem);
                    }
                    result.push(elem);
                }
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|i: int, j: int|
                        0 <= i < j < result@.len() implies
                        #[trigger] TotalOrder::le(result@[i], result@[j])
                    by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert (speed hint)
                        assert(result@[i] == sorted_seq.seq@[i]);
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert (speed hint)
                        assert(result@[j] == sorted_seq.seq@[j]);
                    };
                }
                result
            }
        }

    //		Section 12. derive impls in verus!
// Veracity: NEEDED proof block (speed hint)


        impl<T: StT + Ord + TotalOrder> Default for UnsortedListPQ<T> {
            fn default() -> Self { Self::empty() }
        }

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for UnsortedListPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord + TotalOrder> Clone for UnsortedListPQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned@ == self@
            {
                // Veracity: NEEDED proof block
                let cloned = UnsortedListPQ { elements: self.elements.clone() };
                // Veracity: NEEDED proof block
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

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for UnsortedListPQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self@ == other@)
            {
                let equal = self.elements == other.elements;
                // Veracity: NEEDED proof block
                proof { accept(equal == (self@ == other@)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for UnsortedListPQ<T> {}
    }

    //		Section 13. macros


    #[macro_export]
    macro_rules! UnsortedListPQLit {
        () => {
            $crate::Chap45::UnsortedListPQ::UnsortedListPQ::UnsortedListPQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let mut pq = $crate::Chap45::UnsortedListPQ::UnsortedListPQ::UnsortedListPQ::empty();
            $(
                pq = pq.insert($x);
            )*
            pq
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder + std::fmt::Debug> std::fmt::Debug for UnsortedListPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UnsortedListPQ").field("elements", &self.elements).finish()
        }
    }

    impl<T: StT + Ord + TotalOrder> Display for UnsortedListPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "UnsortedListPQ[")?;
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
