//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Unsorted List

pub mod UnsortedListPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(PartialEq, Clone, Debug)]
    pub struct UnsortedListPQ<T: StT + Ord> {
        elements: ArraySeqStPerS<T>,
    }

    /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
    pub trait UnsortedListPQTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                         -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(element: T)                           -> Self;

        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// Returns the minimum element, or None if empty
        fn find_min(&self)                                 -> Option<&T>;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Inserts element into unsorted list
        fn insert(&self, element: T)                       -> Self;

        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// Removes and returns minimum element with new queue
        fn delete_min(&self)                               -> (Self, Option<T>)
        where
            Self: Sized;

        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        /// Melds two priority queues by concatenating lists
        fn meld(&self, other: &Self)                       -> Self;

        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// Creates priority queue from sequence
        fn from_seq(seq: &ArraySeqStPerS<T>)               -> Self;

        fn size(&self)                                     -> N;
        fn is_empty(&self)                                 -> bool;
        fn to_seq(&self)                                   -> ArraySeqStPerS<T>;
        fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self;
        fn extract_all_sorted(&self)                       -> ArraySeqStPerS<T>;
        fn from_vec(vec: Vec<T>)                           -> Self;
        fn to_vec(&self)                                   -> Vec<T>;
        fn to_sorted_vec(&self)                            -> Vec<T>;
    }

    impl<T: StT + Ord> UnsortedListPQTrait<T> for UnsortedListPQ<T> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn empty() -> Self {
            UnsortedListPQ {
                elements: ArraySeqStPerS::empty(),
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn singleton(element: T) -> Self {
            UnsortedListPQ {
                elements: ArraySeqStPerS::singleton(element),
            }
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Linear scan to find minimum element
        fn find_min(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                return None;
            }

            let mut min_element = self.elements.nth(0);
            for i in 1..self.elements.length() {
                let current = self.elements.nth(i);
                if current < min_element {
                    min_element = current;
                }
            }
            Some(min_element)
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Simply append to end of unsorted list
        fn insert(&self, element: T) -> Self {
            let single_seq = ArraySeqStPerS::singleton(element);
            UnsortedListPQ {
                elements: ArraySeqStPerS::append(&self.elements, &single_seq),
            }
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Find minimum and remove it, creating new list without that element
        fn delete_min(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }

            // Find minimum element and its index
            let mut min_element = self.elements.nth(0);
            let mut min_index = 0;

            for i in 1..self.elements.length() {
                let current = self.elements.nth(i);
                if current < min_element {
                    min_element = current;
                    min_index = i;
                }
            }

            // Create new sequence without the minimum element
            let mut new_elements = ArraySeqStPerS::empty();
            for i in 0..self.elements.length() {
                if i != min_index {
                    let element = self.elements.nth(i);
                    let single_seq = ArraySeqStPerS::singleton(element.clone());
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                }
            }

            let new_pq = UnsortedListPQ { elements: new_elements };

            (new_pq, Some(min_element.clone()))
        }

        /// Claude Work: Θ(m + n), Span: Θ(m + n)
        /// Concatenate the two unsorted lists
        fn meld(&self, other: &Self) -> Self {
            UnsortedListPQ {
                elements: ArraySeqStPerS::append(&self.elements, &other.elements),
            }
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Create priority queue from existing sequence
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self { UnsortedListPQ { elements: seq.clone() } }

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

        fn extract_all_sorted(&self) -> ArraySeqStPerS<T> {
            let mut result = ArraySeqStPerS::empty();
            let mut current_pq = self.clone();

            while !current_pq.is_empty() {
                let (new_pq, min_element) = current_pq.delete_min();
                if let Some(element) = min_element {
                    let single_seq = ArraySeqStPerS::singleton(element);
                    result = ArraySeqStPerS::append(&result, &single_seq);
                }
                current_pq = new_pq;
            }

            result
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
            let sorted_seq = self.extract_all_sorted();
            let mut result = Vec::new();
            for i in 0..sorted_seq.length() {
                result.push(sorted_seq.nth(i).clone());
            }
            result
        }
    }

    impl<T: StT + Ord> Default for UnsortedListPQ<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> Display for UnsortedListPQ<T> {
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

    // Macro for creating unsorted list priority queues
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
}
