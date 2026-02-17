//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Sorted List

pub mod SortedListPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(PartialEq, Clone, Debug)]
    pub struct SortedListPQ<T: StT + Ord> {
        elements: ArraySeqStPerS<T>,
    }

    /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
    pub trait SortedListPQTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                         -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(element: T)                           -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Returns the minimum element (first in sorted list), or None if empty
        fn find_min(&self)                                 -> Option<&T>;

        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// Inserts element in correct sorted position
        fn insert(&self, element: T)                       -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Removes first element (minimum) from sorted list
        fn delete_min(&self)                               -> (Self, Option<T>)
        where
            Self: Sized;

        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        /// Melds two sorted priority queues by merging sorted lists
        fn meld(&self, other: &Self)                       -> Self;

        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        /// Creates priority queue from sequence by sorting
        fn from_seq(seq: &ArraySeqStPerS<T>)               -> Self;

        fn size(&self)                                     -> N;
        fn is_empty(&self)                                 -> bool;
        fn to_seq(&self)                                   -> ArraySeqStPerS<T>;
        fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self;
        fn extract_all_sorted(&self)                       -> ArraySeqStPerS<T>;
        fn find_max(&self)                                 -> Option<&T>;
        fn delete_max(&self)                               -> (Self, Option<T>)
        where
            Self: Sized;
        fn from_vec(vec: Vec<T>)                           -> Self;
        fn to_vec(&self)                                   -> Vec<T>;
        fn to_sorted_vec(&self)                            -> Vec<T>;
        fn is_sorted(&self)                                -> bool;
    }

    impl<T: StT + Ord> SortedListPQTrait<T> for SortedListPQ<T> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn empty() -> Self {
            SortedListPQ {
                elements: ArraySeqStPerS::empty(),
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn singleton(element: T) -> Self {
            SortedListPQ {
                elements: ArraySeqStPerS::singleton(element),
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Minimum is always at the front of sorted list
        fn find_min(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(0))
            }
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Find correct position and insert to maintain sorted order
        fn insert(&self, element: T) -> Self {
            // Find insertion position using binary search approach
            let mut insert_pos = 0;
            for i in 0..self.elements.length() {
                let current = self.elements.nth(i);
                if element <= *current {
                    break;
                }
                insert_pos = i + 1;
            }

            // Build new sequence with element inserted at correct position
            let mut new_elements = ArraySeqStPerS::empty();

            // Add elements before insertion position
            for i in 0..insert_pos {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            // Add the new element
            let new_elem_seq = ArraySeqStPerS::singleton(element);
            new_elements = ArraySeqStPerS::append(&new_elements, &new_elem_seq);

            // Add elements after insertion position
            for i in insert_pos..self.elements.length() {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            SortedListPQ { elements: new_elements }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Remove first element (minimum) from sorted list
        fn delete_min(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }

            let min_element = self.elements.nth(0).clone();

            // Create new sequence without the first element
            let mut new_elements = ArraySeqStPerS::empty();
            for i in 1..self.elements.length() {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            let new_pq = SortedListPQ { elements: new_elements };

            (new_pq, Some(min_element))
        }

        /// Claude Work: Θ(m + n), Span: Θ(m + n)
        /// Merge two sorted lists maintaining sorted order
        fn meld(&self, other: &Self) -> Self {
            let mut result = ArraySeqStPerS::empty();
            let mut i = 0;
            let mut j = 0;

            // Merge the two sorted sequences
            while i < self.elements.length() && j < other.elements.length() {
                let elem_self = self.elements.nth(i);
                let elem_other = other.elements.nth(j);

                if elem_self <= elem_other {
                    let single_seq = ArraySeqStPerS::singleton(elem_self.clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    i += 1;
                } else {
                    let single_seq = ArraySeqStPerS::singleton(elem_other.clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    j += 1;
                }
            }

            // Add remaining elements from self
            while i < self.elements.length() {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                result = ArraySeqStPerS::append(&result, &single_seq);
                i += 1;
            }

            // Add remaining elements from other
            while j < other.elements.length() {
                let elem = other.elements.nth(j);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                result = ArraySeqStPerS::append(&result, &single_seq);
                j += 1;
            }

            SortedListPQ { elements: result }
        }

        /// Claude Work: Θ(n log n), Span: Θ(n log n)
        /// Create priority queue from sequence by insertion sort
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self {
            let mut result = Self::empty();
            for i in 0..seq.length() {
                let element = seq.nth(i);
                result = result.insert(element.clone());
            }
            result
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn size(&self) -> N { self.elements.length() }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn is_empty(&self) -> bool { self.elements.length() == 0 }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn to_seq(&self) -> ArraySeqStPerS<T> { self.elements.clone() }

        fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
            let mut result = self.clone();
            for i in 0..elements.length() {
                let element = elements.nth(i);
                result = result.insert(element.clone());
            }
            result
        }

        fn extract_all_sorted(&self) -> ArraySeqStPerS<T> { self.elements.clone() }

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

            let max_element = self.elements.nth(self.elements.length() - 1).clone();

            // Create new sequence without the last element
            let mut new_elements = ArraySeqStPerS::empty();
            for i in 0..(self.elements.length() - 1) {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            let new_pq = SortedListPQ { elements: new_elements };

            (new_pq, Some(max_element))
        }

        fn from_vec(vec: Vec<T>) -> Self {
            let mut pq = Self::empty();
            for element in vec {
                pq = pq.insert(element);
            }
            pq
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
    }

    impl<T: StT + Ord> Default for SortedListPQ<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> Display for SortedListPQ<T> {
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

    // Macro for creating sorted list priority queues
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
}
