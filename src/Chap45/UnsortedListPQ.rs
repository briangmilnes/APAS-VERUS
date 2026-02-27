//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Unsorted List

pub mod UnsortedListPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::std_specs::vec::group_vec_axioms,
};

// 4. type definitions
        #[verifier::reject_recursive_types(T)]
        pub struct UnsortedListPQ<T: StT + Ord> {
            pub elements: ArraySeqStPerS<T>,
        }

// 5. view impls
        impl<T: StT + Ord> View for UnsortedListPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

// 7. proof fns
        proof fn _unsorted_list_pq_verified() {}

// 8. traits
        /// Meldable Priority Queue ADT (Data Type 45.1) using unsorted list.
        pub trait UnsortedListPQTrait<T: StT + Ord>: Sized + View<V = Seq<T::V>> {
            spec fn spec_size(self) -> nat;

            fn empty() -> (pq: Self)
                ensures pq@.len() == 0;

            fn singleton(element: T) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures pq@.len() == 1;

            fn find_min(&self) -> (min_elem: Option<&T>)
                ensures
                    self@.len() == 0 ==> min_elem.is_none(),
                    self@.len() > 0 ==> min_elem.is_some();

            fn insert(&self, element: T) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + 1 <= usize::MAX as int,
                ensures pq@.len() == self@.len() + 1;

            fn delete_min(&self) -> (result: (Self, Option<T>))
                requires obeys_feq_clone::<T>(),
                ensures
                    self@.len() > 0 ==> result.1.is_some(),
                    self@.len() > 0 ==> result.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> result.1.is_none(),
                    self@.len() == 0 ==> result.0@.len() == self@.len();

            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + other@.len() <= usize::MAX as int,
                ensures pq@.len() == self@.len() + other@.len();

            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures pq@.len() == seq@.len();

            fn size(&self) -> (n: usize)
                ensures n as int == self.spec_size();

            fn is_empty(&self) -> (b: bool)
                ensures b == (self.spec_size() == 0);

            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
                requires obeys_feq_clone::<T>(),
                ensures seq@.len() == self@.len();

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() + elements@.len() <= usize::MAX as int;

            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>)
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() <= usize::MAX as int,
                ensures sorted@.len() == self@.len();

            fn from_vec(vec: Vec<T>) -> Self
                requires obeys_feq_clone::<T>();

            fn to_vec(&self) -> Vec<T>
                requires obeys_feq_clone::<T>();

            fn to_sorted_vec(&self) -> Vec<T>
                requires
                    obeys_feq_clone::<T>(),
                    self@.len() <= usize::MAX as int;
        }

// 9. impls
        impl<T: StT + Ord> UnsortedListPQTrait<T> for UnsortedListPQ<T> {
            open spec fn spec_size(self) -> nat {
                self@.len()
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn empty() -> (pq: Self) {
                UnsortedListPQ {
                    elements: ArraySeqStPerS::empty(),
                }
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn singleton(element: T) -> (pq: Self) {
                UnsortedListPQ {
                    elements: ArraySeqStPerS::singleton(element),
                }
            }

            /// APAS Work Θ(n), Span Θ(n) — linear scan over unsorted list.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    return None;
                }
                let n = self.elements.length();
                let mut min_element = self.elements.nth(0);
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                        n > 0,
                {
                    let current = self.elements.nth(i);
                    if *current < *min_element {
                        min_element = current;
                    }
                }
                Some(min_element)
            }

            /// APAS Work Θ(1), actual Work Θ(n) — append copies persistent array.
            fn insert(&self, element: T) -> (pq: Self) {
                let single_seq = ArraySeqStPerS::singleton(element);
                UnsortedListPQ {
                    elements: ArraySeqStPerS::append(&self.elements, &single_seq),
                }
            }

            /// APAS Work Θ(n), Span Θ(n).
            fn delete_min(&self) -> (result: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let mut min_element = self.elements.nth(0);
                let mut min_index: usize = 0;

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                        n > 0,
                        (min_index as int) < n,
                {
                    let current = self.elements.nth(i);
                    if *current < *min_element {
                        min_element = current;
                        min_index = i;
                    }
                }

                // Rebuild sequence without the minimum element.
                let mut new_elements = ArraySeqStPerS::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        (min_index as int) < n,
                        new_elements@.len() ==
                            i as int - if (min_index as int) < (i as int) { 1int } else { 0int },
                {
                    if i != min_index {
                        let element = self.elements.nth(i);
                        let single_seq = ArraySeqStPerS::singleton(element.clone());
                        new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                    }
                }

                let new_pq = UnsortedListPQ { elements: new_elements };
                (new_pq, Some(min_element.clone()))
            }

            /// APAS Work Θ(m+n), Span Θ(m+n).
            fn meld(&self, other: &Self) -> (pq: Self) {
                UnsortedListPQ {
                    elements: ArraySeqStPerS::append(&self.elements, &other.elements),
                }
            }

            /// APAS Work Θ(n), Span Θ(n).
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                UnsortedListPQ { elements: seq.clone() }
            }

            fn size(&self) -> (n: usize) { self.elements.length() }

            fn is_empty(&self) -> (b: bool) { self.elements.length() == 0 }

            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>) { self.elements.clone() }

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            #[verifier::exec_allows_no_decreases_clause]
            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>) {
                let mut result = ArraySeqStPerS::empty();
                let mut current_pq = self.clone();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while !current_pq.is_empty()
                    invariant
                        result@.len() + current_pq@.len() == self@.len(),
                        self@.len() <= usize::MAX as int,
                {
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
                let seq = ArraySeqStPerS::from_vec(vec);
                Self::from_seq(&seq)
            }

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

            fn to_sorted_vec(&self) -> Vec<T> {
                let sorted_seq = self.extract_all_sorted();
                let n = sorted_seq.length();
                let mut result: Vec<T> = Vec::new();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == sorted_seq@.len(),
                        result@.len() == i as int,
                {
                    let elem = sorted_seq.nth(i).clone();
                    result.push(elem);
                }
                result
            }
        }

// 11. derive impls in verus!
        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord> PartialEqSpecImpl for UnsortedListPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
        }

        impl<T: StT + Ord> Clone for UnsortedListPQ<T> {
            fn clone(&self) -> (result: Self)
                ensures result@ == self@
            {
                let result = UnsortedListPQ { elements: self.elements.clone() };
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

        impl<T: StT + Ord> core::cmp::PartialEq for UnsortedListPQ<T> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self@ == other@)
            {
                let r = self.elements == other.elements;
                proof { accept(r == (self@ == other@)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for UnsortedListPQ<T> {}

        impl<T: StT + Ord> Default for UnsortedListPQ<T> {
            fn default() -> Self { Self::empty() }
        }
    }

// 13. derive impls outside verus!
    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for UnsortedListPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UnsortedListPQ").field("elements", &self.elements).finish()
        }
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

// 12. macros
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
