//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Balanced Trees (AVL Tree)
//!
//! Full verusification blocked: AVLTreeSeqStPerS::from_vec lacks ensures
//! (no spec_well_formed, no spec_seq guarantee), so moving the trait+impl
//! inside verus! would require assumes on nearly every method body.

pub mod BalancedTreePQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::Types::Types::*;

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
        pub struct BalancedTreePQ<T: StT + Ord> {
            pub elements: AVLTreeSeqStPerS<T>,
        }

// 5. view impls
        impl<T: StT + Ord> View for BalancedTreePQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

// 7. proof fns
        proof fn _balanced_tree_pq_verified() {}

// 11. derive impls in verus!
        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord> PartialEqSpecImpl for BalancedTreePQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord> Clone for BalancedTreePQ<T> {
            fn clone(&self) -> (result: Self)
                ensures result@ == self@
            {
                BalancedTreePQ { elements: self.elements.clone() }
            }
        }

        impl<T: StT + Ord> core::cmp::PartialEq for BalancedTreePQ<T> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self@ == other@)
            {
                let r = self.elements == other.elements;
                proof { accept(r == (self@ == other@)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for BalancedTreePQ<T> {}

    }

// 8. traits
    /// Meldable Priority Queue ADT (Data Type 45.1) using balanced tree (AVL).
    pub trait BalancedTreePQTrait<T: StT + Ord> {
        fn empty() -> Self;
        fn singleton(element: T) -> Self;
        fn find_min(&self) -> (Option<&T>);
        fn insert(&self, element: T) -> Self;
        fn delete_min(&self) -> (Self, Option<T>) where Self: Sized;
        fn meld(&self, other: &Self) -> Self;
        fn from_seq(seq: &AVLTreeSeqStPerS<T>) -> Self;
        fn size(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn to_seq(&self) -> AVLTreeSeqStPerS<T>;
        fn find_max(&self) -> Option<&T>;
        fn delete_max(&self) -> (Self, Option<T>) where Self: Sized;
        fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> Self;
        fn extract_all_sorted(&self) -> AVLTreeSeqStPerS<T>;
        fn contains(&self, element: &T) -> bool;
        fn remove(&self, element: &T) -> (Self, bool) where Self: Sized;
        fn range(&self, min_val: &T, max_val: &T) -> AVLTreeSeqStPerS<T>;
        fn from_vec(elements: Vec<T>) -> Self;
        fn to_vec(&self) -> Vec<T>;
        fn to_sorted_vec(&self) -> Vec<T>;
        fn is_sorted(&self) -> bool;
        fn height(&self) -> usize;
        fn split(&self, element: &T) -> (Self, bool, Self) where Self: Sized;
        fn join(left: &Self, right: &Self) -> Self;
    }

    /// Extended operations requiring closure parameters (outside verus!).
    pub trait BalancedTreePQExtTrait<T: StT + Ord> {
        fn filter<F>(&self, predicate: F) -> Self where F: Fn(&T) -> bool;
        fn map<U, G>(&self, f: G) -> BalancedTreePQ<U> where U: StT + Ord, G: Fn(&T) -> U;
    }

// 9. impls
    impl<T: StT + Ord> BalancedTreePQTrait<T> for BalancedTreePQ<T> {
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
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(self.elements.length() - 1))
            }
        }

        /// APAS Work Θ(n), Span Θ(n) — subseq_copy to skip last element.
        fn delete_max(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }
            let n = self.elements.length();
            let max_element = self.elements.nth(n - 1).clone();
            let remaining = self.elements.subseq_copy(0, n - 1);
            (BalancedTreePQ { elements: remaining }, Some(max_element))
        }

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

        fn from_vec(elements: Vec<T>) -> Self {
            let mut values = elements;
            values.sort();
            BalancedTreePQ {
                elements: AVLTreeSeqStPerS::from_vec(values),
            }
        }

        fn to_vec(&self) -> Vec<T> {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                result.push(self.elements.nth(i).clone());
            }
            result
        }

        fn to_sorted_vec(&self) -> Vec<T> {
            self.to_vec()
        }

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
        fn height(&self) -> usize {
            if self.elements.length() == 0 {
                0
            } else {
                ((self.elements.length() as f64).log2().ceil() as usize).max(1)
            }
        }

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

        fn join(left: &Self, right: &Self) -> Self { left.meld(right) }
    }

    impl<T: StT + Ord> BalancedTreePQExtTrait<T> for BalancedTreePQ<T> {
        fn filter<F>(&self, predicate: F) -> Self
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

        fn map<U, G>(&self, f: G) -> BalancedTreePQ<U>
        where
            U: StT + Ord,
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

    impl<T: StT + Ord> Default for BalancedTreePQ<T> {
        fn default() -> Self { Self::empty() }
    }

// 13. derive impls outside verus!
    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for BalancedTreePQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BalancedTreePQ").field("elements", &self.elements).finish()
        }
    }

    impl<T: StT + Ord> Display for BalancedTreePQ<T> {
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
}
