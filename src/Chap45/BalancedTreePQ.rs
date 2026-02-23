//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Balanced Trees (AVL Tree)

pub mod BalancedTreePQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};
        /// Placeholder; PQ uses Vec, Option<&T>, Fn bounds.
        proof fn _balanced_tree_pq_verified() {}

        #[verifier::reject_recursive_types(T)]
        pub struct BalancedTreePQ<T: StT + Ord> {
            pub elements: AVLTreeSeqStPerS<T>,
        }

        impl<T: StT + Ord> View for BalancedTreePQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

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

    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for BalancedTreePQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BalancedTreePQ").field("elements", &self.elements).finish()
        }
    }

    /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
    pub trait BalancedTreePQTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                           -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(element: T)                             -> Self;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Returns the minimum element (leftmost in balanced tree), or None if empty
        fn find_min(&self)                                   -> Option<&T>;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Inserts element into balanced tree maintaining order
        fn insert(&self, element: T)                         -> Self;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Removes minimum element (leftmost) from balanced tree
        fn delete_min(&self)                                 -> (Self, Option<T>)
        where
            Self: Sized;

        /// claude-4-sonet: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// Melds two balanced trees using union operation
        fn meld(&self, other: &Self)                         -> Self;

        /// claude-4-sonet: Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log² n)
        /// Creates priority queue from sequence using balanced tree construction
        fn from_seq(seq: &AVLTreeSeqStPerS<T>)               -> Self;

        fn size(&self)                                       -> usize;
        fn is_empty(&self)                                   -> bool;
        fn to_seq(&self)                                     -> AVLTreeSeqStPerS<T>;
        fn find_max(&self)                                   -> Option<&T>;
        fn delete_max(&self)                                 -> (Self, Option<T>)
        where
            Self: Sized;
        fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> Self;
        fn extract_all_sorted(&self)                         -> AVLTreeSeqStPerS<T>;
        fn contains(&self, element: &T)                      -> bool;
        fn remove(&self, element: &T)                        -> (Self, bool)
        where
            Self: Sized;
        fn range(&self, min_val: &T, max_val: &T)            -> AVLTreeSeqStPerS<T>;
        fn from_vec(elements: Vec<T>)                        -> Self;
        fn to_vec(&self)                                     -> Vec<T>;
        fn to_sorted_vec(&self)                              -> Vec<T>;
        fn is_sorted(&self)                                  -> bool;
        fn height(&self)                                     -> usize;
        fn split(&self, element: &T)                         -> (Self, bool, Self)
        where
            Self: Sized;
        fn join(left: &Self, right: &Self)                   -> Self;
        fn filter<F>(&self, predicate: F)                    -> Self
        where
            F: Fn(&T) -> bool;
        fn map<U, G>(&self, f: G)                           -> BalancedTreePQ<U>
        where
            U: StT + Ord,
            G: Fn(&T) -> U;
    }

    impl<T: StT + Ord> BalancedTreePQTrait<T> for BalancedTreePQ<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn empty() -> Self {
            BalancedTreePQ {
                elements: AVLTreeSeqStPerS::empty(),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn singleton(element: T) -> Self {
            BalancedTreePQ {
                elements: AVLTreeSeqStPerS::singleton(element),
            }
        }

        /// - APAS: (no cost stated — implied Θ(log n) for balanced tree find-leftmost)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — indexed access to nth(0), not tree traversal.
        fn find_min(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                // Minimum is at index 0 in sorted sequence
                Some(self.elements.nth(0))
            }
        }

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Tree has no value-based insert; use sorted-sequence approach: binary search for
        ///   position, insert into vec, rebuild. O(n) for flatten+rebuild.
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

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Uses tree's subseq_copy to get elements [1..n] without Vec round-trip.
        fn delete_min(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }
            let min_element = self.elements.nth(0).clone();
            let n = self.elements.length();
            let remaining = self.elements.subseq_copy(1, n - 1);
            (
                BalancedTreePQ {
                    elements: remaining,
                },
                Some(min_element),
            )
        }

        /// - APAS: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// - Claude-Opus-4.6: Work Θ(m + n), Span Θ(m + n) — flattens both to Vec, merges, rebuilds.
        fn meld(&self, other: &Self) -> Self {
            // Get sorted values from both trees
            let values1 = self.elements.values_in_order();
            let values2 = other.elements.values_in_order();

            // Merge the two sorted vectors
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

            // Add remaining elements
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

        /// - APAS: Work Θ(n log n), Span Θ(log² n)
        /// - Sorted-sequence approach: collect values, sort, build tree in one pass.
        fn from_seq(seq: &AVLTreeSeqStPerS<T>) -> Self {
            let mut values: Vec<T> = (0..seq.length()).map(|i| seq.nth(i).clone()).collect();
            values.sort();
            BalancedTreePQ {
                elements: AVLTreeSeqStPerS::from_vec(values),
            }
        }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn size(&self) -> usize { self.elements.length() }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> bool { self.elements.length() == 0 }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone is O(n).
        fn to_seq(&self) -> AVLTreeSeqStPerS<T> { self.elements.clone() }

        fn find_max(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(self.elements.length() - 1))
            }
        }

        fn delete_max(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }

            let max_index = self.elements.length() - 1;
            let max_element = self.elements.nth(max_index).clone();

            // Convert to vector, remove last element, rebuild tree
            let mut values = self.elements.values_in_order();
            values.remove(max_index);

            let new_pq = BalancedTreePQ {
                elements: AVLTreeSeqStPerS::from_vec(values),
            };

            (new_pq, Some(max_element))
        }

        fn insert_all(&self, elements: &AVLTreeSeqStPerS<T>) -> Self {
            let mut result = self.clone();
            for i in 0..elements.length() {
                let element = elements.nth(i);
                result = result.insert(element.clone());
            }
            result
        }

        fn extract_all_sorted(&self) -> AVLTreeSeqStPerS<T> { self.elements.clone() }

        fn contains(&self, element: &T) -> bool {
            for i in 0..self.elements.length() {
                let current = self.elements.nth(i);
                if current == element {
                    return true;
                }
                if current > element {
                    // Since sequence is sorted, we can stop early
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
                    (
                        BalancedTreePQ {
                            elements: AVLTreeSeqStPerS::from_vec(values),
                        },
                        true,
                    )
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
                    // Since sequence is sorted, we can stop
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
            // Already sorted, just convert to vector
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
                } else if current == element {
                    found = true;
                    right = right.insert(current.clone());
                } else {
                    right = right.insert(current.clone());
                }
            }

            (left, found, right)
        }

        fn join(left: &Self, right: &Self) -> Self { left.meld(right) }

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

    // Macro for creating balanced tree priority queues
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
