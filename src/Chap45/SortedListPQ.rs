//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Sorted List

pub mod SortedListPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_seq_map_cloned_view_eq, obeys_feq_clone};

    verus! {
        proof fn _sorted_list_pq_verified() {}

        #[verifier::reject_recursive_types(T)]
        pub struct SortedListPQ<T: StT + Ord> {
            pub elements: ArraySeqStPerS<T>,
        }

        impl<T: StT + Ord> View for SortedListPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord> PartialEqSpecImpl for SortedListPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord> Clone for SortedListPQ<T> {
            fn clone(&self) -> (result: Self)
                ensures result@ == self@
            {
                let result = SortedListPQ { elements: self.elements.clone() };
                proof {
                    assume(obeys_feq_clone::<T>());
                    lemma_seq_map_cloned_view_eq(
                        self.elements.seq@,
                        result.elements.seq@,
                    );
                }
                result
            }
        }

        impl<T: StT + Ord> core::cmp::PartialEq for SortedListPQ<T> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self@ == other@)
            {
                let r = self.elements == other.elements;
                proof { assume(r == (self@ == other@)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for SortedListPQ<T> {}
    }

    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for SortedListPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SortedListPQ").field("elements", &self.elements).finish()
        }
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

        fn size(&self)                                     -> usize;
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
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn empty() -> Self {
            SortedListPQ {
                elements: ArraySeqStPerS::empty(),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn singleton(element: T) -> Self {
            SortedListPQ {
                elements: ArraySeqStPerS::singleton(element),
            }
        }

        /// - APAS: (no cost stated — implied Θ(1) from sorted-list structure)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — head of sorted list.
        fn find_min(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(0))
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — agrees with APAS.
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

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — rebuilds array without first element; O(n) copy.
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

        /// - APAS: Work Θ(m + n), Span Θ(m + n)
        /// - Claude-Opus-4.6: Work Θ(m + n), Span Θ(m + n) — agrees with APAS.
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

        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Sort input once, then build sorted list directly.
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self {
            let mut vec: Vec<T> = (0..seq.length()).map(|i| seq.nth(i).clone()).collect();
            vec.sort();
            let sorted = ArraySeqStPerS::from_vec(vec);
            SortedListPQ { elements: sorted }
        }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn size(&self) -> usize { self.elements.length() }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> bool { self.elements.length() == 0 }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone is O(n).
        fn to_seq(&self) -> ArraySeqStPerS<T> { self.elements.clone() }

        fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
            self.meld(&Self::from_seq(elements))
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
            let mut v = vec;
            v.sort();
            let seq = ArraySeqStPerS::from_vec(v);
            SortedListPQ { elements: seq }
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
