//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Balanced Trees (AVL Tree).

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

pub mod BalancedTreePQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Types::Types::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus! {

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

// 4. type definitions
        #[verifier::reject_recursive_types(T)]
        pub struct BalancedTreePQ<T: StT + Ord + TotalOrder> {
            pub elements: AVLTreeSeqStPerS<T>,
        }

// 5. view impls
        impl<T: StT + Ord + TotalOrder> View for BalancedTreePQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

// 7. proof fns
        proof fn _balanced_tree_pq_verified() {}

// 8. traits
        /// Meldable Priority Queue ADT (Data Type 45.1) using balanced tree (AVL).
        pub trait BalancedTreePQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Seq<T::V>> {
            spec fn spec_balancedtreepq_wf(&self) -> bool;

            fn empty() -> (pq: Self)
                ensures pq@.len() == 0, pq.spec_balancedtreepq_wf();

            fn singleton(element: T) -> (pq: Self)
                ensures pq@.len() == 1, pq.spec_balancedtreepq_wf();

            fn find_min(&self) -> (min_elem: Option<&T>)
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some();

            fn insert(&self, element: T) -> (pq: Self)
                requires self.spec_balancedtreepq_wf(),
                ensures pq@.len() == self@.len() + 1, pq.spec_balancedtreepq_wf();

            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() > 0 ==> min_and_rest.1.is_some(),
                    self@.len() > 0 ==> min_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> min_and_rest.1.is_none(),
                    self@.len() == 0 ==> min_and_rest.0@.len() == self@.len(),
                    min_and_rest.0.spec_balancedtreepq_wf();

            fn meld(&self, other: &Self) -> (pq: Self)
                requires self.spec_balancedtreepq_wf(), other.spec_balancedtreepq_wf(),
                ensures pq@.len() == self@.len() + other@.len(), pq.spec_balancedtreepq_wf();

            fn from_seq(seq: &AVLTreeSeqStPerS<T>) -> (pq: Self)
                requires seq.spec_avltreeseqstper_wf(),
                ensures pq@.len() == seq@.len(), pq.spec_balancedtreepq_wf();

            fn size(&self) -> (n: usize)
                requires self.spec_balancedtreepq_wf(),
                ensures n as int == self@.len();

            fn is_empty(&self) -> (b: bool)
                requires self.spec_balancedtreepq_wf(),
                ensures b == (self@.len() == 0);

            fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures seq@ =~= self@;

            fn find_max(&self) -> (max_elem: Option<&T>)
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() == 0 ==> max_elem.is_none(),
                    self@.len() > 0 ==> max_elem.is_some();

            fn delete_max(&self) -> (max_and_rest: (Self, Option<T>))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    self@.len() > 0 ==> max_and_rest.1.is_some(),
                    self@.len() > 0 ==> max_and_rest.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> max_and_rest.1.is_none(),
                    self@.len() == 0 ==> max_and_rest.0@.len() == self@.len(),
                    max_and_rest.0.spec_balancedtreepq_wf();

            fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> (pq: Self)
                requires self.spec_balancedtreepq_wf(), elements.spec_avltreeseqstper_wf(),
                ensures pq@.len() == self@.len() + elements@.len(), pq.spec_balancedtreepq_wf();

            fn extract_all_sorted(&self) -> (sorted: AVLTreeSeqStPerS<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures sorted@.len() == self@.len();

            fn contains(&self, element: &T) -> (found: bool)
                requires self.spec_balancedtreepq_wf(),
                ensures found == self@.contains(element@);

            fn remove(&self, element: &T) -> (rest_and_found: (Self, bool))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    rest_and_found.1 ==> rest_and_found.0@.len() == self@.len() - 1,
                    !rest_and_found.1 ==> rest_and_found.0@.len() == self@.len(),
                    rest_and_found.0.spec_balancedtreepq_wf();

            fn range(&self, min_val: &T, max_val: &T) -> (sub: AVLTreeSeqStPerS<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures sub@.len() <= self@.len();

            fn from_vec(elements: Vec<T>) -> (pq: Self)
                ensures pq@.len() == elements@.len(), pq.spec_balancedtreepq_wf();

            fn to_vec(&self) -> (vec: Vec<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures vec@.len() == self@.len();

            fn to_sorted_vec(&self) -> (vec: Vec<T>)
                requires self.spec_balancedtreepq_wf(),
                ensures vec@.len() == self@.len();

            fn is_sorted(&self) -> (sorted: bool)
                requires self.spec_balancedtreepq_wf(),
                ensures self@.len() <= 1 ==> sorted;

            fn height(&self) -> (h: usize)
                requires self.spec_balancedtreepq_wf(),
                ensures self@.len() == 0 ==> h == 0;

            fn split(&self, element: &T) -> (parts: (Self, bool, Self))
                requires self.spec_balancedtreepq_wf(),
                ensures
                    parts.0@.len() + parts.2@.len() == self@.len(),
                    parts.0.spec_balancedtreepq_wf(),
                    parts.2.spec_balancedtreepq_wf();

            fn join(left: &Self, right: &Self) -> (pq: Self)
                requires left.spec_balancedtreepq_wf(), right.spec_balancedtreepq_wf(),
                ensures pq@.len() == left@.len() + right@.len(), pq.spec_balancedtreepq_wf();
        }

        /// Extended operations requiring closure parameters.
        pub trait BalancedTreePQExtTrait<T: StT + Ord + TotalOrder>: Sized {
            fn filter<F>(&self, predicate: F) -> (filtered: Self) where F: Fn(&T) -> bool
                ensures true;
            fn map<U, G>(&self, f: G) -> (mapped: BalancedTreePQ<U>) where U: StT + Ord + TotalOrder, G: Fn(&T) -> U
                ensures true;
        }

// 9. impls
        impl<T: StT + Ord + TotalOrder> BalancedTreePQTrait<T> for BalancedTreePQ<T> {
            open spec fn spec_balancedtreepq_wf(&self) -> bool {
                self.elements.spec_avltreeseqstper_wf()
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn empty() -> Self {
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::empty(),
                }
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn singleton(element: T) -> Self {
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::singleton(element),
                }
            }

            /// APAS Work Θ(1), Span Θ(1) — indexed access to first element.
            fn find_min(&self) -> Option<&T> {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(0))
                }
            }

            /// APAS Work Θ(n), Span Θ(n) — flatten, find position, rebuild.
            #[verifier::external_body]
            fn insert(&self, element: T) -> Self {
                let mut values = self.elements.values_in_order();
                let insert_pos = match values.binary_search(&element) {
                    Ok(pos) => pos,
                    Err(pos) => pos,
                };
                values.insert(insert_pos, element);
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::from_vec(values),
                }
            }

            /// APAS Work Θ(n), Span Θ(n) — subseq_copy to skip first element.
            #[verifier::external_body]
            fn delete_min(&self) -> (Self, Option<T>) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let min_element = self.elements.nth(0).clone();
                let n = self.elements.length();
                let remaining = self.elements.subseq_copy(1, n - 1);
                (BalancedTreePQ { elements: remaining }, Some(min_element))
            }

            /// APAS Work Θ(m+n), Span Θ(m+n) — flatten both, merge, rebuild.
            #[verifier::external_body]
            fn meld(&self, other: &Self) -> Self {
                let values1 = self.elements.values_in_order();
                let values2 = other.elements.values_in_order();
                let mut merged = Vec::with_capacity(values1.len() + values2.len());
                let mut i = 0;
                let mut j = 0;
                while i < values1.len() && j < values2.len() {
                    if values1[i] <= values2[j] {
                        merged.push(values1[i].clone());
                        i += 1;
                    } else {
                        merged.push(values2[j].clone());
                        j += 1;
                    }
                }
                while i < values1.len() {
                    merged.push(values1[i].clone());
                    i += 1;
                }
                while j < values2.len() {
                    merged.push(values2[j].clone());
                    j += 1;
                }
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::from_vec(merged),
                }
            }

            /// APAS Work Θ(n log n), Span Θ(log² n) — sort then build.
            #[verifier::external_body]
            fn from_seq(seq: &AVLTreeSeqStPerS<T>) -> Self {
                let mut values: Vec<T> = (0..seq.length()).map(|i| seq.nth(i).clone()).collect();
                values.sort();
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::from_vec(values),
                }
            }

            fn size(&self) -> usize { self.elements.length() }

            fn is_empty(&self) -> bool { self.elements.length() == 0 }

            fn to_seq(&self) -> AVLTreeSeqStPerS<T> { self.elements.clone() }

            fn find_max(&self) -> Option<&T> {
                let n = self.elements.length();
                if n == 0 {
                    None
                } else {
                    Some(self.elements.nth(n - 1))
                }
            }

            /// APAS Work Θ(n), Span Θ(n) — subseq_copy to skip last element.
            #[verifier::external_body]
            fn delete_max(&self) -> (Self, Option<T>) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let max_element = self.elements.nth(n - 1).clone();
                let remaining = self.elements.subseq_copy(0, n - 1);
                (BalancedTreePQ { elements: remaining }, Some(max_element))
            }

            #[verifier::external_body]
            fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> Self {
                let mut result = self.clone();
                for i in 0..elements.length() {
                    let element = elements.nth(i);
                    result = result.insert(element.clone());
                }
                result
            }

            /// Already sorted — clone the backing tree.
            fn extract_all_sorted(&self) -> AVLTreeSeqStPerS<T> { self.elements.clone() }

            #[verifier::external_body]
            fn contains(&self, element: &T) -> bool {
                for i in 0..self.elements.length() {
                    let current = self.elements.nth(i);
                    if current == element {
                        return true;
                    }
                    if current > element {
                        break;
                    }
                }
                false
            }

            #[verifier::external_body]
            fn remove(&self, element: &T) -> (Self, bool) {
                let mut values = self.elements.values_in_order();
                match values.binary_search(element) {
                    Ok(pos) => {
                        values.remove(pos);
                        (BalancedTreePQ { elements: AVLTreeSeqStPerS::from_vec(values) }, true)
                    }
                    Err(_) => (self.clone(), false),
                }
            }

            #[verifier::external_body]
            fn range(&self, min_val: &T, max_val: &T) -> AVLTreeSeqStPerS<T> {
                let values = self.elements.values_in_order();
                let mut range_values = Vec::new();
                for current in values.iter() {
                    if current >= min_val && current <= max_val {
                        range_values.push(current.clone());
                    } else if current > max_val {
                        break;
                    }
                }
                AVLTreeSeqStPerS::from_vec(range_values)
            }

            #[verifier::external_body]
            fn from_vec(elements: Vec<T>) -> Self {
                let mut values = elements;
                values.sort();
                BalancedTreePQ {
                    elements: AVLTreeSeqStPerS::from_vec(values),
                }
            }

            #[verifier::external_body]
            fn to_vec(&self) -> Vec<T> {
                let mut result = Vec::new();
                for i in 0..self.elements.length() {
                    result.push(self.elements.nth(i).clone());
                }
                result
            }

            #[verifier::external_body]
            fn to_sorted_vec(&self) -> Vec<T> {
                self.to_vec()
            }

            #[verifier::external_body]
            fn is_sorted(&self) -> bool {
                for i in 1..self.elements.length() {
                    let prev = self.elements.nth(i - 1);
                    let curr = self.elements.nth(i);
                    if prev > curr {
                        return false;
                    }
                }
                true
            }

            /// Approximate balanced tree height: ceil(log2(n)).
            #[verifier::external_body]
            fn height(&self) -> usize {
                if self.elements.length() == 0 {
                    0
                } else {
                    ((self.elements.length() as f64).log2().ceil() as usize).max(1)
                }
            }

            #[verifier::external_body]
            fn split(&self, element: &T) -> (Self, bool, Self) {
                let mut left = Self::empty();
                let mut right = Self::empty();
                let mut found = false;
                for i in 0..self.elements.length() {
                    let current = self.elements.nth(i);
                    if current < element {
                        left = left.insert(current.clone());
                    } else {
                        if current == element {
                            found = true;
                        }
                        right = right.insert(current.clone());
                    }
                }
                (left, found, right)
            }

            #[verifier::external_body]
            fn join(left: &Self, right: &Self) -> Self { left.meld(right) }
        }

        #[verifier::external]
        impl<T: StT + Ord + TotalOrder> Default for BalancedTreePQ<T> {
            fn default() -> Self { Self::empty() }
        }

        #[verifier::external]
        impl<T: StT + Ord + TotalOrder> BalancedTreePQExtTrait<T> for BalancedTreePQ<T> {
            fn filter<F>(&self, predicate: F) -> (filtered: Self)
            where
                F: Fn(&T) -> bool,
            {
                let mut result = Self::empty();
                for i in 0..self.elements.length() {
                    let current = self.elements.nth(i);
                    if predicate(current) {
                        result = result.insert(current.clone());
                    }
                }
                result
            }

            fn map<U, G>(&self, f: G) -> (mapped: BalancedTreePQ<U>)
            where
                U: StT + Ord + TotalOrder,
                G: Fn(&T) -> U,
            {
                let mut result = BalancedTreePQ::<U>::empty();
                for i in 0..self.elements.length() {
                    let current = self.elements.nth(i);
                    let mapped = f(current);
                    result = result.insert(mapped);
                }
                result
            }
        }

// 11. derive impls in verus!
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
                proof { accept(equal == (self@ == other@)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for BalancedTreePQ<T> {}

    }

// 12. macros
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

// 13. derive impls outside verus!
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
