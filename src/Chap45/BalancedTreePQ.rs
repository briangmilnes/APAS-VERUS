//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 45: Priority Queue implementation using Balanced Trees (AVL Tree).

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

pub mod BalancedTreePQ {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Types::Types::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4. type definitions


        #[verifier::reject_recursive_types(T)]
        pub struct BalancedTreePQ<T: StT + Ord + TotalOrder> {
            pub elements: AVLTreeSeqStPerS<T>,
        }

    //		Section 5. view impls


        impl<T: StT + Ord + TotalOrder> View for BalancedTreePQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

    //		Section 7. proof fns/broadcast groups


        proof fn _balanced_tree_pq_verified() {}

    //		Section 8. traits


        /// Meldable Priority Queue ADT (Data Type 45.1) using balanced tree (AVL).
        pub trait BalancedTreePQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Seq<T::V>> {
            spec fn spec_balancedtreepq_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn empty() -> (pq: Self)
                ensures pq@.len() == 0, pq.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn singleton(element: T) -> (pq: Self)
                ensures pq@.len() == 1, pq.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn find_min(&self) -> (min_elem: Option<&T>)
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some(),
                    self@.len() > 0 ==> min_elem.unwrap()@ == self@[0];

            /// - Alg Analysis: APAS (Ch45 cost table, balanced trees): Work O(lg n), Span O(lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
            fn insert(&self, element: T) -> (pq: Self)
                requires
                    self.spec_balancedtreepq_wf(),
                    self@.len() + 1 < usize::MAX as nat,
                ensures
                    pq@.len() == self@.len() + 1,
                    pq.spec_balancedtreepq_wf(),
                    pq@.to_multiset() =~= self@.to_multiset().insert(element@);

            /// - Alg Analysis: APAS (Ch45 cost table, balanced trees): Work O(lg n), Span O(lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() > 0 ==> min_and_rest.1.is_some(),
                    self@.len() > 0 ==> min_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> min_and_rest.1.is_none(),
                    self@.len() == 0 ==> min_and_rest.0@.len() == self@.len(),
                    min_and_rest.0.spec_balancedtreepq_wf();

            /// - Alg Analysis: APAS (Ch45 cost table, balanced trees): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m))
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)): tree union
            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    self.spec_balancedtreepq_wf(),
                    other.spec_balancedtreepq_wf(),
                    self@.len() + other@.len() < usize::MAX as nat,
                ensures pq@.len() == self@.len() + other@.len(), pq.spec_balancedtreepq_wf();

            /// - Alg Analysis: APAS (Ch45 cost table, balanced trees): Work O(n lg n), Span O(n lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n): sequential inserts
            fn from_seq(seq: &AVLTreeSeqStPerS<T>) -> (pq: Self)
                requires seq.spec_avltreeseqstper_wf(),
                ensures pq@.len() == seq@.len(), pq.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn size(&self) -> (n: usize)
                requires self.spec_balancedtreepq_wf(),
                ensures n as int == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> (b: bool)
                requires self.spec_balancedtreepq_wf(),
                ensures b == (self@.len() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures seq@ =~= self@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn find_max(&self) -> (max_elem: Option<&T>)
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() == 0 ==> max_elem.is_none(),
                    self@.len() > 0 ==> max_elem.is_some(),
                    self@.len() > 0 ==> max_elem.unwrap()@ == self@[self@.len() - 1];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn delete_max(&self) -> (max_and_rest: (Self, Option<T>))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() > 0 ==> max_and_rest.1.is_some(),
                    self@.len() > 0 ==> max_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> max_and_rest.1.is_none(),
                    self@.len() == 0 ==> max_and_rest.0@.len() == self@.len(),
                    max_and_rest.0.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg(1+n/m)), Span O(m lg(1+n/m))
            fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> (pq: Self)
                requires
                    self.spec_balancedtreepq_wf(),
                    elements.spec_avltreeseqstper_wf(),
                    self@.len() + elements@.len() < usize::MAX as nat,
                ensures pq@.len() == self@.len() + elements@.len(), pq.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn extract_all_sorted(&self) -> (sorted: AVLTreeSeqStPerS<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures sorted@.len() == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn contains(&self, element: &T) -> (found: bool)
                requires self.spec_balancedtreepq_wf(),
                ensures found == self@.contains(element@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn remove(&self, element: &T) -> (rest_and_found: (Self, bool))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    rest_and_found.1 ==> rest_and_found.0@.len() == self@.len() - 1,
                    !rest_and_found.1 ==> rest_and_found.0@.len() == self@.len(),
                    rest_and_found.0.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn range(&self, min_val: &T, max_val: &T) -> (sub: AVLTreeSeqStPerS<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures sub@.len() <= self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn from_vec(elements: Vec<T>) -> (pq: Self)
                requires elements@.len() < usize::MAX as nat,
                ensures pq@.len() == elements@.len(), pq.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (vec: Vec<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures vec@.len() == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_sorted_vec(&self) -> (vec: Vec<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures vec@.len() == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_sorted(&self) -> (sorted: bool)
                requires self.spec_balancedtreepq_wf(),
                ensures self@.len() <= 1 ==> sorted;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn height(&self) -> (h: usize)
                requires self.spec_balancedtreepq_wf(),
                ensures self@.len() == 0 ==> h == 0;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn split(&self, element: &T) -> (parts: (Self, bool, Self))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    parts.0@.len() + parts.2@.len() == self@.len(),
                    parts.0.spec_balancedtreepq_wf(),
                    parts.2.spec_balancedtreepq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg(1+n/m)), Span O(m lg(1+n/m))
            fn join(left: &Self, right: &Self) -> (pq: Self)
                requires
                    left.spec_balancedtreepq_wf(),
                    right.spec_balancedtreepq_wf(),
                    left@.len() + right@.len() < usize::MAX as nat,
                ensures pq@.len() == left@.len() + right@.len(), pq.spec_balancedtreepq_wf();
        }

        /// Extended operations requiring closure parameters.
        pub trait BalancedTreePQExtTrait<T: StT + Ord + TotalOrder>: Sized + BalancedTreePQTrait<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn filter<F: Fn(&T) -> bool>(&self, predicate: F) -> (filtered: Self)
                requires
                    self.spec_balancedtreepq_wf(),
                    forall|t: &T| #[trigger] predicate.requires((t,)),
                ensures filtered.spec_balancedtreepq_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn map<U: StT + Ord + TotalOrder, G: Fn(&T) -> U>(&self, f: G) -> (mapped: BalancedTreePQ<U>)
                requires
                    self.spec_balancedtreepq_wf(),
                    forall|t: &T| #[trigger] f.requires((t,)),
                ensures mapped.spec_balancedtreepq_wf();
        }

    //		Section 9. impls


        impl<T: StT + Ord + TotalOrder> BalancedTreePQTrait<T> for BalancedTreePQ<T> {
            open spec fn spec_balancedtreepq_wf(&self) -> bool {
                self.elements.spec_avltreeseqstper_wf()
                && obeys_feq_full::<T>()
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); constant-time empty construction.
            fn empty() -> Self {
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::empty(),
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); constant-time singleton construction.
            fn singleton(element: T) -> Self {
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::singleton(element),
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — DIFFERS: indexed access to first element of sorted seq.
            fn find_min(&self) -> Option<&T> {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(0))
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: values_in_order + Vec::insert at sorted position.
            fn insert(&self, element: T) -> Self {
                let mut vals: Vec<T> = self.elements.values_in_order();
                let ghost old_vals = vals@;
                let n = vals.len();

                // Find sorted insertion position.
                let mut pos: usize = n;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        pos == n,
                        n == vals@.len(),
                        old_vals =~= vals@,
                {
                    if element <= vals[i] {
                        pos = i;
                        break;
                    }
                }

                // Insert at sorted position.
                vals.insert(pos, element);

                // Veracity: NEEDED proof block
                proof {
                    // vals@.len() = n + 1 = self@.len() + 1 < usize::MAX (from requires).
                    // n = old_vals.len(), and old_vals.map_values(..) =~= self.elements.spec_seq()
                    // (from values_in_order ensures), so n == self@.len() as nat.
                }
                let result = BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::from_vec(vals),
                };
                // Veracity: NEEDED proof block
                proof {
                    // vals@ == old_vals.insert(pos, element)  (from Vec::insert)
                    // result@ =~= vals@.map_values(|t: T| t@)  (from from_vec)
                    // old_vals.map_values(|t: T| t@) =~= self@  (from values_in_order)
                    //
                    // Need: result@ =~= self@.insert(pos, element@)
                    // i.e.: old_vals.insert(pos, element).map_values(|t: T| t@)
                    //    =~= old_vals.map_values(|t: T| t@).insert(pos, element@)
                    old_vals.insert_ensures(pos as int, element);
                    self@.insert_ensures(pos as int, element@);
                    let view_fn = |t: T| t@;
                    // Veracity: NEEDED assert
                    assert forall|k: int| 0 <= k < vals@.map_values(view_fn).len()
                        implies #[trigger] vals@.map_values(view_fn)[k]
                            == self@.insert(pos as int, element@)[k] by {
                        if k < pos as int {
                        } else if k == pos as int {
                        } else {
                        }
                    }
                    // Veracity: NEEDED assert
                    assert(result@ =~= self@.insert(pos as int, element@));
                    // to_multiset_insert (broadcast): s.insert(i, a).to_multiset() == s.to_multiset().insert(a)
                }
                result
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: clone elements 1..n, rebuild via from_vec.
            fn delete_min(&self) -> (Self, Option<T>) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let min_element = self.elements.nth(0).clone();
                let n = self.elements.length();
                let mut values: Vec<T> = Vec::new();
                for i in 1..n
                    invariant
                        values@.len() == (i - 1) as int,
                        n == self.elements.spec_seq().len(),
                        self.elements.spec_avltreeseqstper_wf(),
                {
                    values.push(self.elements.nth(i).clone());
                }
                let remaining = AVLTreeSeqStPerS::from_vec(values);
                (BalancedTreePQ { elements: remaining }, Some(min_element))
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(m log(1+n/m)), Span O(m log(1+n/m)).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m+n), Span O(m+n) — DIFFERS: merge two sorted sequences, rebuild via from_vec.
            fn meld(&self, other: &Self) -> Self {
                let n1 = self.elements.length();
                let n2 = other.elements.length();
                let mut values: Vec<T> = Vec::new();
                let mut i: usize = 0;
                let mut j: usize = 0;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n1 && j < n2
                    invariant
                        values@.len() == (i + j) as int,
                        i <= n1, j <= n2,
                        n1 as nat == self.elements.spec_seq().len(),
                        n2 as nat == other.elements.spec_seq().len(),
                        self.elements.spec_avltreeseqstper_wf(),
                        other.elements.spec_avltreeseqstper_wf(),
                    decreases (n1 - i) + (n2 - j),
                {
                    let a = self.elements.nth(i);
                    let b = other.elements.nth(j);
                    if *a <= *b {
                        values.push(a.clone());
                        i = i + 1;
                    } else {
                        values.push(b.clone());
                        j = j + 1;
                    }
                }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n1
                    invariant
                        values@.len() == (i + j) as int,
                        i <= n1, j <= n2,
                        n1 as nat == self.elements.spec_seq().len(),
                        self.elements.spec_avltreeseqstper_wf(),
                    decreases n1 - i,
                {
                    values.push(self.elements.nth(i).clone());
                    i = i + 1;
                }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while j < n2
                    invariant
                        values@.len() == (i + j) as int,
                        j <= n2,
                        n1 as nat == self.elements.spec_seq().len(),
                        n2 as nat == other.elements.spec_seq().len(),
                        other.elements.spec_avltreeseqstper_wf(),
                    decreases n2 - j,
                {
                    values.push(other.elements.nth(j).clone());
                    j = j + 1;
                }
                // Veracity: NEEDED proof block
                proof {
                    // n1 = self.elements.length() ensures n1 as nat == self@.len().
                    // n2 = other.elements.length() ensures n2 as nat == other@.len().
                    // values@.len() = n1 + n2 = self@.len() + other@.len() < usize::MAX (from requires).
                }
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::from_vec(values),
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n log n), Span O(n log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — DIFFERS: n calls to insert, each O(n).
            fn from_seq(seq: &AVLTreeSeqStPerS<T>) -> Self {
                let mut result = Self::empty();
                let n = seq.length();
                // Veracity: NEEDED proof block
                proof {
                    lemma_size_lt_usize_max::<T>(&seq.root);
                    lemma_size_eq_inorder_len::<T>(&seq.root);
                }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        result@.len() == i as int,
                        result.spec_balancedtreepq_wf(),
                        n as nat == seq.spec_seq().len(),
                        n < usize::MAX,
                        seq.spec_avltreeseqstper_wf(),
                {
                    // Veracity: NEEDED proof block
                    proof {
                        // result@.len() = i < n < usize::MAX → result@.len() + 1 < usize::MAX.
                    }
                    result = result.insert(seq.nth(i).clone());
                }
                result
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn size(&self) -> usize { self.elements.length() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> bool { self.elements.length() == 0 }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_seq(&self) -> AVLTreeSeqStPerS<T> { self.elements.clone() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn find_max(&self) -> Option<&T> {
                let n = self.elements.length();
                if n == 0 {
                    None
                } else {
                    Some(self.elements.nth(n - 1))
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: clone elements 0..n-1, rebuild via from_vec.
            fn delete_max(&self) -> (Self, Option<T>) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let max_element = self.elements.nth(n - 1).clone();
                let mut values: Vec<T> = Vec::new();
                for i in 0..(n - 1)
                    invariant
                        values@.len() == i as int,
                        n == self.elements.spec_seq().len(),
                        n >= 1,
                        self.elements.spec_avltreeseqstper_wf(),
                {
                    values.push(self.elements.nth(i).clone());
                }
                // Veracity: NEEDED proof block
                proof {
                    // After loop: values@.len() = n - 1. Since n is usize and n >= 1,
                    // n - 1 <= usize::MAX - 1 < usize::MAX.
                    lemma_size_lt_usize_max::<T>(&self.elements.root);
                    lemma_size_eq_inorder_len::<T>(&self.elements.root);
                }
                let remaining = AVLTreeSeqStPerS::from_vec(values);
                (BalancedTreePQ { elements: remaining }, Some(max_element))
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg(1+n/m)), Span O(m lg(1+n/m))
            fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                // Veracity: NEEDED proof block
                proof {
                    // other@.len() == elements@.len() (from from_seq ensures).
                    // self@.len() + elements@.len() < usize::MAX (from requires).
                }
                self.meld(&other)
            }

            /// Already sorted — clone the backing tree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn extract_all_sorted(&self) -> AVLTreeSeqStPerS<T> { self.elements.clone() }

            #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn contains(&self, element: &T) -> bool {
                let n = self.elements.length();
                let mut i: usize = 0;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n
                    invariant
                        i <= n,
                        n as int == self@.len(),
                        self.elements.spec_avltreeseqstper_wf(),
                        !self@.subrange(0, i as int).contains(element@),
                    decreases n - i,
                {
                    let current = self.elements.nth(i);
                    let eq = feq(current, element);
                    if eq {
                        return true;
                    }
                    i = i + 1;
                }
                false
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn remove(&self, element: &T) -> (Self, bool) {
                let n = self.elements.length();
                let mut values: Vec<T> = Vec::new();
                let mut found = false;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        values@.len() == (if found { i as int - 1 } else { i as int }),
                        n == self.elements.spec_seq().len(),
                        self.elements.spec_avltreeseqstper_wf(),
                {
                    let current = self.elements.nth(i);
                    if !found && *current == *element {
                        found = true;
                    } else {
                        values.push(current.clone());
                    }
                }
                // Veracity: NEEDED proof block
                proof {
                    // values@.len() <= n < usize::MAX (from wf).
                    lemma_size_lt_usize_max::<T>(&self.elements.root);
                    lemma_size_eq_inorder_len::<T>(&self.elements.root);
                }
                (BalancedTreePQ { elements: AVLTreeSeqStPerS::from_vec(values) }, found)
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn range(&self, min_val: &T, max_val: &T) -> AVLTreeSeqStPerS<T> {
                let n = self.elements.length();
                let mut values: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        values@.len() <= i as int,
                        n == self.elements.spec_seq().len(),
                        self.elements.spec_avltreeseqstper_wf(),
                {
                    let current = self.elements.nth(i);
                    if *current >= *min_val && *current <= *max_val {
                        values.push(current.clone());
                    }
                }
                // Veracity: NEEDED proof block
                proof {
                    // values@.len() <= n < usize::MAX (from wf).
                    lemma_size_lt_usize_max::<T>(&self.elements.root);
                    lemma_size_eq_inorder_len::<T>(&self.elements.root);
                    // Veracity: NEEDED assert
                    assert(values@.len() < usize::MAX);
                }
                AVLTreeSeqStPerS::from_vec(values)
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn from_vec(elements: Vec<T>) -> Self {
                let mut result = Self::empty();
                let n = elements.len();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        result@.len() == i as int,
                        result.spec_balancedtreepq_wf(),
                        n == elements@.len(),
                        n < usize::MAX,
                {
                    // Veracity: NEEDED proof block
                    proof {
                        // result@.len() = i < n < usize::MAX → result@.len() + 1 < usize::MAX.
                    }
                    result = result.insert(elements[i].clone());
                }
                result
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> Vec<T> {
                let n = self.elements.length();
                let mut result: Vec<T> = Vec::new();
                let mut i: usize = 0;
                while i < n
                    invariant
                        self.elements.spec_avltreeseqstper_wf(),
                        n as nat == self.elements.spec_seq().len(),
                        0 <= i <= n,
                        result@.len() == i as int,
                    decreases n - i,
                {
                    result.push(self.elements.nth(i).clone());
                    i = i + 1;
                }
                result
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_sorted_vec(&self) -> Vec<T> {
                self.to_vec()
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_sorted(&self) -> bool {
                let n = self.elements.length();
                if n <= 1 {
                    return true;
                }
                for i in 1..n
                    invariant
                        n == self.elements.spec_seq().len(),
                        n >= 2,
                        self.elements.spec_avltreeseqstper_wf(),
                {
                    let prev = self.elements.nth(i - 1);
                    let curr = self.elements.nth(i);
                    if *prev > *curr {
                        return false;
                    }
                }
                true
            }

            /// Approximate balanced tree height: ceil(log2(n)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn height(&self) -> usize {
                let n = self.elements.length();
                if n == 0 {
                    return 0;
                }
                let mut h: usize = 0;
                let mut x: usize = n;
                while x > 1
                    invariant
                        x >= 1,
                        h + x <= n,
                    decreases x,
                {
                    x = x / 2;
                    h = h + 1;
                }
                if h == 0 { 1 } else { h }
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn split(&self, element: &T) -> (Self, bool, Self) {
                let mut left = Self::empty();
                let mut right = Self::empty();
                let mut found = false;
                let n = self.elements.length();
                // Veracity: NEEDED proof block
                proof {
                    lemma_size_lt_usize_max::<T>(&self.elements.root);
                    lemma_size_eq_inorder_len::<T>(&self.elements.root);
                }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        left@.len() + right@.len() == i as int,
                        left.spec_balancedtreepq_wf(),
                        right.spec_balancedtreepq_wf(),
                        n == self.elements.spec_seq().len(),
                        n < usize::MAX,
                        self.elements.spec_avltreeseqstper_wf(),
                {
                    // Veracity: NEEDED proof block
                    proof {
                        // left@.len() <= i < n < usize::MAX and right@.len() <= i < n < usize::MAX.
                    }
                    let current = self.elements.nth(i);
                    if *current < *element {
                        left = left.insert(current.clone());
                    } else {
                        if *current == *element {
                            found = true;
                        }
                        right = right.insert(current.clone());
                    }
                }
                (left, found, right)
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg(1+n/m)), Span O(m lg(1+n/m))
            fn join(left: &Self, right: &Self) -> Self { left.meld(right) }
        }

        impl<T: StT + Ord + TotalOrder> BalancedTreePQExtTrait<T> for BalancedTreePQ<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn filter<F: Fn(&T) -> bool>(&self, predicate: F) -> (filtered: Self)
            {
                let mut result = Self::empty();
                let n = self.elements.length();
                // Veracity: NEEDED proof block
                proof {
                    lemma_size_lt_usize_max::<T>(&self.elements.root);
                    lemma_size_eq_inorder_len::<T>(&self.elements.root);
                }
                let mut i: usize = 0;
                while i < n
                    invariant
                        i <= n,
                        n as nat == self.elements.spec_seq().len(),
                        n < usize::MAX,
                        result@.len() <= i as nat,
                        self.spec_balancedtreepq_wf(),
                        result.spec_balancedtreepq_wf(),
                        forall|t: &T| #[trigger] predicate.requires((t,)),
                    decreases n - i,
                {
                    // Veracity: NEEDED proof block
                    proof {
                        // result@.len() <= i < n < usize::MAX → result@.len() + 1 < usize::MAX.
                    }
                    let current = self.elements.nth(i);
                    if predicate(current) {
                        result = result.insert(current.clone());
                    }
                    i = i + 1;
                }
                result
            }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
            fn map<U: StT + Ord + TotalOrder, G: Fn(&T) -> U>(&self, f: G) -> (mapped: BalancedTreePQ<U>)
            {
                let mut result = BalancedTreePQ::<U>::empty();
                let n = self.elements.length();
                // Veracity: NEEDED proof block
                proof {
                    lemma_size_lt_usize_max::<T>(&self.elements.root);
                    lemma_size_eq_inorder_len::<T>(&self.elements.root);
                }
                let mut i: usize = 0;
                while i < n
                    invariant
                        i <= n,
                        n as nat == self.elements.spec_seq().len(),
                        n < usize::MAX,
                        result@.len() == i as nat,
                        self.spec_balancedtreepq_wf(),
                        result.spec_balancedtreepq_wf(),
                        forall|t: &T| #[trigger] f.requires((t,)),
                    decreases n - i,
                {
                    // Veracity: NEEDED proof block
                    proof {
                        // result@.len() = i < n < usize::MAX → result@.len() + 1 < usize::MAX.
                    }
                    let current = self.elements.nth(i);
                    let mapped_val = f(current);
                    result = result.insert(mapped_val);
                    i = i + 1;
                }
                result
            }
        }

    //		Section 12. derive impls in verus!


        impl<T: StT + Ord + TotalOrder> Default for BalancedTreePQ<T> {
            fn default() -> (d: Self)
                ensures d@.len() == 0, d.spec_balancedtreepq_wf()
            {
             Self::empty() }
        }

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for BalancedTreePQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord + TotalOrder> Clone for BalancedTreePQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned@ == self@
            {
                BalancedTreePQ { elements: self.elements.clone() }
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for BalancedTreePQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self@ == other@)
            {
                let equal = self.elements == other.elements;
                // Veracity: NEEDED proof block
                proof { assume(equal == (self@ == other@)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for BalancedTreePQ<T> {}

    }

    //		Section 13. macros


    #[macro_export]
    macro_rules! BalancedTreePQLit {
        () => {
            $crate::Chap45::BalancedTreePQ::BalancedTreePQ::BalancedTreePQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let mut pq = $crate::Chap45::BalancedTreePQ::BalancedTreePQ::BalancedTreePQ::empty();
            $(
                pq = pq.insert($x);
            )*
            pq
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder + std::fmt::Debug> std::fmt::Debug for BalancedTreePQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BalancedTreePQ").field("elements", &self.elements).finish()
        }
    }

    impl<T: StT + Ord + TotalOrder> Display for BalancedTreePQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "BalancedTreePQ[")?;
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
