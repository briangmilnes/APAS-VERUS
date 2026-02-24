//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Binary Heap

pub mod BinaryHeapPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{axiom_cloned_implies_eq_owned, lemma_seq_map_cloned_view_eq, obeys_feq_clone};
    use crate::vstdplus::accept::accept;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::std_specs::vec::group_vec_axioms,
};

        proof fn _binary_heap_pq_verified() {}

        #[verifier::reject_recursive_types(T)]
        pub struct BinaryHeapPQ<T: StT + Ord> {
            pub elements: ArraySeqStPerS<T>,
        }

        impl<T: StT + Ord> View for BinaryHeapPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord> PartialEqSpecImpl for BinaryHeapPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord> Clone for BinaryHeapPQ<T> {
            fn clone(&self) -> (result: Self)
                ensures result@ == self@
            {
                let result = BinaryHeapPQ { elements: self.elements.clone() };
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

        impl<T: StT + Ord> core::cmp::PartialEq for BinaryHeapPQ<T> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self@ == other@)
            {
                let r = self.elements == other.elements;
                proof { accept(r == (self@ == other@)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for BinaryHeapPQ<T> {}

        fn left_child(i: usize) -> (r: usize)
            requires i <= usize::MAX / 2 - 1,
            ensures r as int == left_child_spec(i as int),
        {
            2 * i + 1
        }
        fn right_child(i: usize) -> (r: usize)
            requires i <= usize::MAX / 2 - 1,
            ensures r as int == right_child_spec(i as int),
        {
            2 * i + 2
        }
        fn parent(i: usize) -> (r: usize)
            ensures r as int == parent_spec(i as int),
        {
            if i == 0 { 0 } else { (i - 1) / 2 }
        }

        spec fn parent_spec(i: int) -> int {
            if i == 0 { 0 } else { (i - 1) / 2 }
        }

        spec fn left_child_spec(i: int) -> int {
            2 * i + 1
        }
        spec fn right_child_spec(i: int) -> int {
            2 * i + 2
        }

        fn swap_elements<T: StT + Ord>(seq: &ArraySeqStPerS<T>, i: usize, j: usize) -> (result: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                (i as int) < seq.view().len(),
                (j as int) < seq.view().len(),
                seq@.len() <= usize::MAX as int,
            ensures result@.len() == seq@.len()
        {
            let n = seq.length();
            let mut result = ArraySeqStPerS::empty();

            #[verifier::loop_isolation(false)]
            for k in 0..n
                invariant
                    n == seq@.len(),
                    result@.len() == k as int,
                    (i as int) < n,
                    (j as int) < n,
            {
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

        fn bubble_up<T: StT + Ord>(seq: &ArraySeqStPerS<T>, mut i: usize) -> (result: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                (i as int) < seq.view().len(),
                seq@.len() <= usize::MAX as int,
            ensures result@.len() == seq@.len()
        {
            let mut result = seq.clone();

            #[verifier::loop_isolation(false)]
            while i > 0
                invariant
                    result@.len() == seq@.len(),
                    result@.len() <= usize::MAX as int,
                    (i as int) < seq.view().len(),
                    parent_spec(i as int) < seq.view().len(),
                decreases i,
            {
                let parent_idx = parent(i);
                proof {
                    assert((parent_idx as int) < seq.view().len());
                }
                let current = result.nth(i);
                let parent_val = result.nth(parent_idx);

                if *current >= *parent_val {
                    i = 0;
                } else {
                    proof {
                        assert((i as int) < result.view().len());
                        assert((parent_idx as int) < result.view().len());
                    }
                    result = swap_elements(&result, i, parent_idx);
                    i = parent_idx;
                }
            }

            result
        }

        #[verifier::external_body]
        fn bubble_down<T: StT + Ord>(heap: &ArraySeqStPerS<T>, i: usize) -> (result: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                (i as int) < heap.view().len(),
                heap@.len() <= usize::MAX as int,
                heap@.len() * 2 <= usize::MAX as int,
            ensures result@.len() == heap@.len()
        {
            let mut result = heap.clone();
            let mut idx = i;

            loop {
                let left = left_child(idx);
                let right = right_child(idx);
                let mut smallest = idx;

                if left < result.length() && *result.nth(left) < *result.nth(smallest) {
                    smallest = left;
                }

                if right < result.length() && *result.nth(right) < *result.nth(smallest) {
                    smallest = right;
                }

                if smallest == idx {
                    break;
                }

                result = swap_elements(&result, idx, smallest);
                idx = smallest;
            }

            result
        }

        fn heapify<T: StT + Ord>(seq: &ArraySeqStPerS<T>) -> (result: ArraySeqStPerS<T>)
            requires
                obeys_feq_clone::<T>(),
                seq@.len() <= usize::MAX as int,
                seq@.len() * 2 <= usize::MAX as int,
            ensures result@.len() == seq@.len()
        {
            if seq.length() <= 1 {
                return seq.clone();
            }

            let mut result = seq.clone();
            let last_non_leaf = if seq.length() >= 2 { (seq.length() - 2) / 2 } else { 0 };

            let mut idx = last_non_leaf + 1;
            #[verifier::loop_isolation(false)]
            while idx > 0
                invariant
                    result@.len() == seq@.len(),
                    (idx as int) <= seq@.len(),
                    result@.len() <= usize::MAX as int,
                    result@.len() * 2 <= usize::MAX as int,
                decreases idx,
            {
                idx = idx - 1;
                result = bubble_down(&result, idx);
            }

            result
        }

        fn is_heap<T: StT + Ord>(elements: &ArraySeqStPerS<T>) -> (r: bool)
            requires elements@.len() * 2 <= usize::MAX as int,
        {
            let n = elements.length();
            let mut r = true;
            for i in 0..n
                invariant
                    n == elements@.len(),
                    (i as int) <= n,
                    (i as int) < n || i == n,
            {
                if i <= usize::MAX / 2 - 1 {
                let left = left_child(i);
                let right = right_child(i);

                if left < n && *elements.nth(i) > *elements.nth(left) {
                    r = false;
                }

                if right < n && *elements.nth(i) > *elements.nth(right) {
                    r = false;
                }
                }
            }
            r
        }

        pub open spec fn spec_heap_inv_at<T: StT + Ord>(seq: Seq<T::V>, i: int) -> bool {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            (left >= seq.len() || spec_leq_view::<T>(seq[i], seq[left]))
            && (right >= seq.len() || spec_leq_view::<T>(seq[i], seq[right]))
        }

        pub uninterp spec fn spec_leq_view<T: StT + Ord>(a: T::V, b: T::V) -> bool;

        pub open spec fn spec_is_heap<T: StT + Ord>(seq: Seq<T::V>) -> bool {
            forall|i: int| 0 <= i < seq.len() ==> spec_heap_inv_at::<T>(seq, i)
        }

        /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
        pub trait BinaryHeapPQTrait<T: StT + Ord>: Sized + View<V = Seq<T::V>> {
            spec fn spec_size(self) -> nat;

            fn empty() -> Self;

            fn singleton(element: T) -> Self
                requires obeys_feq_clone::<T>();

            fn find_min(&self) -> Option<&T>;

            fn insert(&self, element: T) -> Self
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + 1 <= usize::MAX as int;

            fn delete_min(&self) -> (Self, Option<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int;

            fn meld(&self, other: &Self) -> Self
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + other@.len() <= usize::MAX as int,
                    (self@.len() + other@.len()) * 2 <= usize::MAX as int;

            fn from_seq(seq: &ArraySeqStPerS<T>) -> Self
                requires
                    obeys_feq_clone::<T>(),
                    seq@.len() * 2 <= usize::MAX as int;

            fn size(&self) -> (r: usize)
                ensures r as int == self.spec_size();

            fn is_empty(&self) -> (r: bool)
                ensures r == (self.spec_size() == 0);

            fn to_seq(&self) -> ArraySeqStPerS<T>
                requires obeys_feq_clone::<T>();

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + elements@.len() <= usize::MAX as int,
                    (self@.len() + elements@.len()) * 2 <= usize::MAX as int;

            fn extract_all_sorted(&self) -> ArraySeqStPerS<T>
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int;

            fn is_valid_heap(&self) -> (r: bool)
                requires self@.len() * 2 <= usize::MAX as int;

            fn height(&self) -> usize;

            fn level_elements(&self, level: usize) -> ArraySeqStPerS<T>
                requires
                    obeys_feq_clone::<T>(),
                    level < 63;

            fn from_vec(vec: Vec<T>) -> Self
                requires
                    obeys_feq_clone::<T>(),
                    vec@.len() * 2 <= usize::MAX as int;

            fn to_vec(&self) -> Vec<T>
                requires obeys_feq_clone::<T>();

            fn to_sorted_vec(&self) -> Vec<T>
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() * 2 <= usize::MAX as int;
        }

        impl<T: StT + Ord> BinaryHeapPQTrait<T> for BinaryHeapPQ<T> {
            open spec fn spec_size(self) -> nat {
                self@.len()
            }

            fn empty() -> Self {
                BinaryHeapPQ {
                    elements: ArraySeqStPerS::empty(),
                }
            }

            fn singleton(element: T) -> Self {
                BinaryHeapPQ {
                    elements: ArraySeqStPerS::singleton(element),
                }
            }

            fn find_min(&self) -> Option<&T> {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(0))
                }
            }

            fn insert(&self, element: T) -> Self {
                let single_seq = ArraySeqStPerS::singleton(element);
                let new_elements = ArraySeqStPerS::append(&self.elements, &single_seq);

                let last_index = new_elements.length() - 1;
                let heapified = bubble_up(&new_elements, last_index);

                BinaryHeapPQ { elements: heapified }
            }

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

                let mut new_elements = ArraySeqStPerS::singleton(last_element);
                let n = self.elements.length();
                let end = n - 1;
                #[verifier::loop_isolation(false)]
                for i in 1..end
                    invariant
                        n == self.elements@.len(),
                        end == n - 1,
                        new_elements@.len() == (i - 1) as int + 1,
                        (i as int) < n,
                {
                    let elem = self.elements.nth(i);
                    let single_seq = ArraySeqStPerS::singleton(elem.clone());
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                }

                let heapified = bubble_down(&new_elements, 0);

                let new_pq = BinaryHeapPQ { elements: heapified };

                (new_pq, Some(min_element))
            }

            fn meld(&self, other: &Self) -> Self {
                let merged = ArraySeqStPerS::append(&self.elements, &other.elements);
                let heapified = heapify(&merged);

                BinaryHeapPQ { elements: heapified }
            }

            fn from_seq(seq: &ArraySeqStPerS<T>) -> (result: Self)
                ensures result@.len() == seq@.len(),
            {
                let heapified = heapify(seq);
                BinaryHeapPQ { elements: heapified }
            }

            fn size(&self) -> usize {
                self.elements.length()
            }

            fn is_empty(&self) -> bool {
                self.elements.length() == 0
            }

            fn to_seq(&self) -> ArraySeqStPerS<T> {
                self.elements.clone()
            }

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            #[verifier::exec_allows_no_decreases_clause]
            fn extract_all_sorted(&self) -> ArraySeqStPerS<T> {
                let mut result = ArraySeqStPerS::empty();
                let mut current_heap = self.clone();

                #[verifier::loop_isolation(false)]
                while !current_heap.is_empty()
                    invariant
                        current_heap@.len() * 2 <= usize::MAX as int,
                        result@.len() + current_heap@.len() == self@.len(),
                {
                    let (new_heap, min_element) = current_heap.delete_min();
                    proof {
                        assume(new_heap@.len() + 1 == current_heap@.len());
                        assume(min_element.is_some());
                    }
                    if let Some(element) = min_element {
                        proof {
                            assert(result@.len() + 1 + new_heap@.len() == self@.len());
                            assume(result@.len() + 1 <= usize::MAX as int);
                        }
                        let single_seq = ArraySeqStPerS::singleton(element);
                        result = ArraySeqStPerS::append(&result, &single_seq);
                    }
                    current_heap = new_heap;
                }

                result
            }

            fn is_valid_heap(&self) -> bool {
                is_heap(&self.elements)
            }

            #[verifier::external_body]
            fn height(&self) -> usize {
                if self.elements.length() == 0 {
                    0
                } else {
                    ((self.elements.length() as f64).log2().floor() as usize) + 1
                }
            }

            #[verifier::external_body]
            fn level_elements(&self, level: usize) -> ArraySeqStPerS<T> {
                let mut result = ArraySeqStPerS::empty();
                let n = self.elements.length();
                let start_idx = if level < 63 { (1usize << level) - 1 } else { 0 };
                let end_idx = if level < 63 {
                    ((1usize << (level + 1)) - 1).min(n)
                } else {
                    n
                };

                for i in start_idx..end_idx {
                    if i < n {
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
                let seq = self.to_seq();
                let n = seq.length();
                let mut result: Vec<T> = Vec::new();
                #[verifier::loop_isolation(false)]
                for i in 0..n
                    invariant
                        n == seq@.len(),
                        result@.len() == i as int,
                        forall|j: int| 0 <= j < i ==> (result@[j])@ == seq@[j],
                {
                    let elem = seq.nth(i).clone();
                    proof { axiom_cloned_implies_eq_owned(seq.spec_index(i as int), elem); }
                    result.push(elem);
                }
                result
            }

            fn to_sorted_vec(&self) -> Vec<T> {
                let sorted_seq = self.extract_all_sorted();
                let n = sorted_seq.length();
                let mut result: Vec<T> = Vec::new();
                #[verifier::loop_isolation(false)]
                for i in 0..n
                    invariant
                        n == sorted_seq@.len(),
                        result@.len() == i as int,
                        forall|j: int| 0 <= j < i ==> (result@[j])@ == sorted_seq@[j],
                {
                    let elem = sorted_seq.nth(i).clone();
                    proof { axiom_cloned_implies_eq_owned(sorted_seq.spec_index(i as int), elem); }
                    result.push(elem);
                }
                result
            }
        }

        impl<T: StT + Ord> Default for BinaryHeapPQ<T> {
            fn default() -> Self {
                Self::empty()
            }
        }
    }

    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for BinaryHeapPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BinaryHeapPQ").field("elements", &self.elements).finish()
        }
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
