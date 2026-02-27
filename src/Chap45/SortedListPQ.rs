//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Sorted List

pub mod SortedListPQ {

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
        pub struct SortedListPQ<T: StT + Ord> {
            pub elements: ArraySeqStPerS<T>,
        }

// 5. view impls
        impl<T: StT + Ord> View for SortedListPQ<T> {
            type V = Seq<T::V>;
            open spec fn view(&self) -> Seq<T::V> { self.elements@ }
        }

// 7. proof fns
        proof fn _sorted_list_pq_verified() {}

// 8. traits
        /// Meldable Priority Queue ADT (Data Type 45.1) using sorted list.
        pub trait SortedListPQTrait<T: StT + Ord>: Sized + View<V = Seq<T::V>> {
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
                requires obeys_feq_clone::<T>(),
                ensures sorted@.len() == self@.len();

            fn find_max(&self) -> (max_elem: Option<&T>)
                ensures
                    self@.len() == 0 ==> max_elem.is_none(),
                    self@.len() > 0 ==> max_elem.is_some();

            fn delete_max(&self) -> (result: (Self, Option<T>))
                requires obeys_feq_clone::<T>(),
                ensures
                    self@.len() > 0 ==> result.1.is_some(),
                    self@.len() > 0 ==> result.0@.len() == self@.len() - 1,
                    self@.len() == 0 ==> result.1.is_none(),
                    self@.len() == 0 ==> result.0@.len() == self@.len();

            fn from_vec(vec: Vec<T>) -> Self
                requires obeys_feq_clone::<T>();

            fn to_vec(&self) -> Vec<T>
                requires obeys_feq_clone::<T>();

            fn to_sorted_vec(&self) -> Vec<T>
                requires obeys_feq_clone::<T>();

            fn is_sorted(&self) -> bool;
        }

// 9. impls
        impl<T: StT + Ord> SortedListPQTrait<T> for SortedListPQ<T> {
            open spec fn spec_size(self) -> nat {
                self@.len()
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn empty() -> (pq: Self) {
                SortedListPQ {
                    elements: ArraySeqStPerS::empty(),
                }
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn singleton(element: T) -> (pq: Self) {
                SortedListPQ {
                    elements: ArraySeqStPerS::singleton(element),
                }
            }

            /// APAS Work Θ(1), Span Θ(1) — head of sorted list.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(0))
                }
            }

            /// APAS Work Θ(n), Span Θ(n).
            fn insert(&self, element: T) -> (pq: Self) {
                let n = self.elements.length();

                // Find insertion position (first element >= new element).
                let mut insert_pos: usize = n;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == self.elements@.len(),
                        insert_pos <= n,
                {
                    if insert_pos == n && element <= *self.elements.nth(i) {
                        insert_pos = i;
                    }
                }

                // Build new sequence with element inserted at insert_pos.
                let mut new_elements = ArraySeqStPerS::empty();

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for k in 0..insert_pos
                    invariant
                        n == self.elements@.len(),
                        insert_pos <= n,
                        new_elements@.len() == k as int,
                {
                    let single_seq = ArraySeqStPerS::singleton(self.elements.nth(k).clone());
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                }

                let new_elem_seq = ArraySeqStPerS::singleton(element);
                new_elements = ArraySeqStPerS::append(&new_elements, &new_elem_seq);

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for k in insert_pos..n
                    invariant
                        n == self.elements@.len(),
                        insert_pos <= n,
                        new_elements@.len() == (k as int) + 1,
                {
                    let single_seq = ArraySeqStPerS::singleton(self.elements.nth(k).clone());
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                }

                SortedListPQ { elements: new_elements }
            }

            /// APAS Work Θ(1), actual Work Θ(n) — rebuilds without first element.
            fn delete_min(&self) -> (result: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let min_element = self.elements.nth(0).clone();

                let mut new_elements = ArraySeqStPerS::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                        n > 0,
                        new_elements@.len() == (i - 1) as int,
                {
                    let single_seq = ArraySeqStPerS::singleton(self.elements.nth(i).clone());
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                }

                (SortedListPQ { elements: new_elements }, Some(min_element))
            }

            /// APAS Work Θ(m+n), Span Θ(m+n) — merge two sorted sequences.
            fn meld(&self, other: &Self) -> (pq: Self) {
                let n = self.elements.length();
                let m = other.elements.length();
                let mut result = ArraySeqStPerS::empty();
                let mut i: usize = 0;
                let mut j: usize = 0;

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n && j < m
                    invariant
                        n == self.elements@.len(),
                        m == other.elements@.len(),
                        i <= n, j <= m,
                        result@.len() == (i + j) as int,
                    decreases (n - i) + (m - j),
                {
                    if *self.elements.nth(i) <= *other.elements.nth(j) {
                        let single_seq = ArraySeqStPerS::singleton(self.elements.nth(i).clone());
                        result = ArraySeqStPerS::append(&result, &single_seq);
                        i = i + 1;
                    } else {
                        let single_seq = ArraySeqStPerS::singleton(other.elements.nth(j).clone());
                        result = ArraySeqStPerS::append(&result, &single_seq);
                        j = j + 1;
                    }
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < n
                    invariant
                        n == self.elements@.len(),
                        m == other.elements@.len(),
                        i <= n, j <= m,
                        result@.len() == (i + j) as int,
                    decreases n - i,
                {
                    let single_seq = ArraySeqStPerS::singleton(self.elements.nth(i).clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    i = i + 1;
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while j < m
                    invariant
                        n == self.elements@.len(),
                        m == other.elements@.len(),
                        i <= n, j <= m,
                        result@.len() == (i + j) as int,
                    decreases m - j,
                {
                    let single_seq = ArraySeqStPerS::singleton(other.elements.nth(j).clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    j = j + 1;
                }

                SortedListPQ { elements: result }
            }

            /// APAS Work Θ(n log n), actual Work Θ(n²) — repeated insert.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                let n = seq.length();
                let mut pq = Self::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == seq@.len(),
                        pq@.len() == i as int,
                {
                    pq = pq.insert(seq.nth(i).clone());
                }
                pq
            }

            fn size(&self) -> (n: usize) { self.elements.length() }

            fn is_empty(&self) -> (b: bool) { self.elements.length() == 0 }

            fn to_seq(&self) -> (seq: ArraySeqStPerS<T>) { self.elements.clone() }

            fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
                let other = Self::from_seq(elements);
                self.meld(&other)
            }

            /// Already sorted — just clone the backing sequence.
            fn extract_all_sorted(&self) -> (sorted: ArraySeqStPerS<T>) {
                self.elements.clone()
            }

            fn find_max(&self) -> (max_elem: Option<&T>) {
                if self.elements.length() == 0 {
                    None
                } else {
                    Some(self.elements.nth(self.elements.length() - 1))
                }
            }

            fn delete_max(&self) -> (result: (Self, Option<T>)) {
                if self.elements.length() == 0 {
                    return (self.clone(), None);
                }
                let n = self.elements.length();
                let max_element = self.elements.nth(n - 1).clone();
                let end = n - 1;
                let mut new_elements = ArraySeqStPerS::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..end
                    invariant
                        n == self.elements@.len(),
                        end == n - 1,
                        new_elements@.len() == i as int,
                {
                    let single_seq = ArraySeqStPerS::singleton(self.elements.nth(i).clone());
                    new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
                }
                (SortedListPQ { elements: new_elements }, Some(max_element))
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
                    result.push(self.elements.nth(i).clone());
                }
                result
            }

            fn to_sorted_vec(&self) -> Vec<T> {
                self.to_vec()
            }

            fn is_sorted(&self) -> bool {
                let n = self.elements.length();
                if n <= 1 {
                    return true;
                }
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 1..n
                    invariant
                        n == self.elements@.len(),
                {
                    if *self.elements.nth(i - 1) > *self.elements.nth(i) {
                        return false;
                    }
                }
                true
            }
        }

// 11. derive impls in verus!
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
                proof { accept(r == (self@ == other@)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for SortedListPQ<T> {}

        impl<T: StT + Ord> Default for SortedListPQ<T> {
            fn default() -> Self { Self::empty() }
        }
    }

// 13. derive impls outside verus!
    impl<T: StT + Ord + std::fmt::Debug> std::fmt::Debug for SortedListPQ<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SortedListPQ").field("elements", &self.elements).finish()
        }
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

// 12. macros
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
