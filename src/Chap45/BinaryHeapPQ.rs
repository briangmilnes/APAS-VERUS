//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Binary Heap

pub mod BinaryHeapPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(PartialEq, Clone, Debug)]
    pub struct BinaryHeapPQ<T: StT + Ord> {
        elements: ArraySeqStPerS<T>,
    }

    /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
    pub trait BinaryHeapPQTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                           -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(element: T)             -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Returns the minimum element (root of min-heap), or None if empty
        fn find_min(&self)                   -> Option<&T>;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Inserts element and bubbles up to maintain heap property
        fn insert(&self, element: T)         -> Self;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Removes root (minimum) and bubbles down to maintain heap property
        fn delete_min(&self)                 -> (Self, Option<T>)
        where
            Self: Sized;

        /// claude-4-sonet: Work Θ(m + n), Span Θ(m + n), Parallelism Θ(1)
        /// Melds two heaps by concatenating and re-heapifying
        fn meld(&self, other: &Self)         -> Self;

        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        /// Creates heap from sequence using bottom-up heapify
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self;

        fn size(&self)                       -> N;
        fn is_empty(&self)                   -> bool;
        fn to_seq(&self)                     -> ArraySeqStPerS<T>;
        fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self;
        fn extract_all_sorted(&self)         -> ArraySeqStPerS<T>;
        fn is_valid_heap(&self)              -> bool;
        fn height(&self)                     -> N;
        fn level_elements(&self, level: N)   -> ArraySeqStPerS<T>;
        fn from_vec(vec: Vec<T>)             -> Self;
        fn to_vec(&self)                     -> Vec<T>;
        fn to_sorted_vec(&self)              -> Vec<T>;
    }

    fn left_child(i: N) -> N { 2 * i + 1 }
    fn right_child(i: N) -> N { 2 * i + 2 }
    fn parent(i: N) -> N { if i == 0 { 0 } else { (i - 1) / 2 } }

    fn swap_elements<T: StT + Ord>(seq: &ArraySeqStPerS<T>, i: N, j: N) -> ArraySeqStPerS<T> {
        let mut result = ArraySeqStPerS::empty();

        for k in 0..seq.length() {
            let element = if k == i {
                seq.nth(j).clone()
            } else if k == j {
                seq.nth(i).clone()
            } else {
                seq.nth(k).clone()
            };

            let single_seq = ArraySeqStPerS::singleton(element);
            result = ArraySeqStPerS::append(&result, &single_seq);
        }

        result
    }

    fn bubble_up<T: StT + Ord>(seq: &ArraySeqStPerS<T>, mut i: N) -> ArraySeqStPerS<T> {
        let mut result = seq.clone();

        while i > 0 {
            let parent_idx = parent(i);
            let current = result.nth(i);
            let parent_val = result.nth(parent_idx);

            if current >= parent_val {
                break;
            }

            result = swap_elements(&result, i, parent_idx);
            i = parent_idx;
        }

        result
    }

    fn bubble_down<T: StT + Ord>(heap: &ArraySeqStPerS<T>, mut i: N) -> ArraySeqStPerS<T> {
        let mut result = heap.clone();

        loop {
            let left = left_child(i);
            let right = right_child(i);
            let mut smallest = i;

            if left < result.length() && result.nth(left) < result.nth(smallest) {
                smallest = left;
            }

            if right < result.length() && result.nth(right) < result.nth(smallest) {
                smallest = right;
            }

            if smallest == i {
                break;
            }

            result = swap_elements(&result, i, smallest);
            i = smallest;
        }

        result
    }

    fn heapify<T: StT + Ord>(seq: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
        if seq.length() <= 1 {
            return seq.clone();
        }

        let mut result = seq.clone();
        let last_non_leaf = if seq.length() >= 2 { (seq.length() - 2) / 2 } else { 0 };

        for i in (0..=last_non_leaf).rev() {
            result = bubble_down(&result, i);
        }

        result
    }

    fn is_heap<T: StT + Ord>(elements: &ArraySeqStPerS<T>) -> bool {
        for i in 0..elements.length() {
            let left = left_child(i);
            let right = right_child(i);

            if left < elements.length() && elements.nth(i) > elements.nth(left) {
                return false;
            }

            if right < elements.length() && elements.nth(i) > elements.nth(right) {
                return false;
            }
        }
        true
    }

    impl<T: StT + Ord> BinaryHeapPQTrait<T> for BinaryHeapPQ<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn empty() -> Self {
            BinaryHeapPQ {
                elements: ArraySeqStPerS::empty(),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn singleton(element: T) -> Self {
            BinaryHeapPQ {
                elements: ArraySeqStPerS::singleton(element),
            }
        }

        /// - APAS: (no cost stated — implied Θ(1) from heap-root access)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — index-0 access.
        fn find_min(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(0))
            }
        }

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) — each swap rebuilds array O(n), O(log n) swaps.
        fn insert(&self, element: T) -> Self {
            // Add element at the end
            let single_seq = ArraySeqStPerS::singleton(element);
            let new_elements = ArraySeqStPerS::append(&self.elements, &single_seq);

            // Bubble up from the last position
            let last_index = new_elements.length() - 1;
            let heapified = bubble_up(&new_elements, last_index);

            BinaryHeapPQ { elements: heapified }
        }

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) — each swap rebuilds array O(n), O(log n) swaps.
        fn delete_min(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }

            if self.elements.length() == 1 {
                let min_element = self.elements.nth(0).clone();
                return (Self::empty(), Some(min_element));
            }

            let min_element = self.elements.nth(0).clone();
            let last_element = self.elements.nth(self.elements.length() - 1).clone();

            // Create new sequence with last element at root and without the last element
            let mut new_elements = ArraySeqStPerS::singleton(last_element);
            for i in 1..(self.elements.length() - 1) {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            // Bubble down from root
            let heapified = bubble_down(&new_elements, 0);

            let new_pq = BinaryHeapPQ { elements: heapified };

            (new_pq, Some(min_element))
        }

        /// - APAS: Work Θ(m + n), Span Θ(m + n)
        /// - Claude-Opus-4.6: Work Θ((m+n) log(m+n)), Span Θ((m+n) log(m+n)) — heapify does O(n) bubble_downs, each O(n) from swap.
        fn meld(&self, other: &Self) -> Self {
            let merged = ArraySeqStPerS::append(&self.elements, &other.elements);
            let heapified = heapify(&merged);

            BinaryHeapPQ { elements: heapified }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n² log n), Span Θ(n² log n) — heapify: O(n) bubble_downs, each O(n) swap cost.
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self {
            let heapified = heapify(seq);

            BinaryHeapPQ { elements: heapified }
        }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn size(&self) -> N { self.elements.length() }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> bool { self.elements.length() == 0 }

        /// - APAS: N/A — utility function not in prose.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone is O(n).
        fn to_seq(&self) -> ArraySeqStPerS<T> { self.elements.clone() }

        fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
            self.meld(&Self::from_seq(elements))
        }

        fn extract_all_sorted(&self) -> ArraySeqStPerS<T> {
            let mut result = ArraySeqStPerS::empty();
            let mut current_heap = self.clone();

            while !current_heap.is_empty() {
                let (new_heap, min_element) = current_heap.delete_min();
                if let Some(element) = min_element {
                    let single_seq = ArraySeqStPerS::singleton(element);
                    result = ArraySeqStPerS::append(&result, &single_seq);
                }
                current_heap = new_heap;
            }

            result
        }

        fn is_valid_heap(&self) -> bool { is_heap(&self.elements) }

        fn height(&self) -> N {
            if self.elements.length() == 0 {
                0
            } else {
                ((self.elements.length() as f64).log2().floor() as N) + 1
            }
        }

        fn level_elements(&self, level: N) -> ArraySeqStPerS<T> {
            let mut result = ArraySeqStPerS::empty();
            let start_idx = (1 << level) - 1;
            let end_idx = ((1 << (level + 1)) - 1).min(self.elements.length());

            for i in start_idx..end_idx {
                if i < self.elements.length() {
                    let elem = self.elements.nth(i);
                    let single_seq = ArraySeqStPerS::singleton(elem.clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                }
            }

            result
        }

        fn from_vec(vec: Vec<T>) -> Self {
            let seq = ArraySeqStPerS::from_vec(vec);
            Self::from_seq(&seq)
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

    impl<T: StT + Ord> Default for BinaryHeapPQ<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> Display for BinaryHeapPQ<T> {
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

    // Macro for creating binary heap priority queues
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
