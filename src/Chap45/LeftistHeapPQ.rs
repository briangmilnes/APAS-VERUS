//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Leftist Heap (Data Structure 45.3)

pub mod LeftistHeapPQ {

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
    vstd::std_specs::vec::group_vec_axioms,
};

// 4. type definitions
        #[verifier::reject_recursive_types(T)]
        pub enum LeftistHeapNode<T: StT + Ord> {
            Leaf,
            Node {
                key: T,
                left: Box<LeftistHeapNode<T>>,
                right: Box<LeftistHeapNode<T>>,
                rank: usize,
            },
        }

        /// Priority Queue implemented using Leftist Heap (Data Structure 45.3).
        #[verifier::reject_recursive_types(T)]
        pub struct LeftistHeapPQ<T: StT + Ord> {
            pub root: LeftistHeapNode<T>,
        }

// 6. spec fns
        impl<T: StT + Ord> LeftistHeapNode<T> {
            pub open spec fn spec_size(self) -> nat
                decreases self
            {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } =>
                        1 + (*left).spec_size() + (*right).spec_size(),
                }
            }
        }

// 7. proof fns
        proof fn _leftist_heap_pq_verified() {}

// 8. traits
        pub trait LeftistHeapNodeTrait<T: StT + Ord>: Sized {
            fn rank(&self) -> (r: usize);
            fn make_node(key: T, left: LeftistHeapNode<T>, right: LeftistHeapNode<T>) -> (node: Self);
            fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>);
            fn size(&self) -> (n: usize);
            fn height(&self) -> (h: usize);
            fn is_leftist(&self) -> (b: bool);
            fn is_heap(&self) -> (b: bool);
            fn to_vec(&self) -> (v: Vec<T>);
        }

        /// Meldable Priority Queue ADT (Data Type 45.1) using leftist heap.
        pub trait LeftistHeapPQTrait<T: StT + Ord>: Sized {
            fn empty() -> (pq: Self);
            fn singleton(element: T) -> (pq: Self);
            fn find_min(&self) -> (min_elem: Option<&T>);
            fn insert(&self, element: T) -> (pq: Self);
            fn delete_min(&self) -> (result: (Self, Option<T>));
            fn meld(&self, other: &Self) -> (pq: Self);
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>();
            fn size(&self) -> (n: usize);
            fn is_empty(&self) -> (b: bool);
            fn extract_all_sorted(&self) -> (sorted: Vec<T>);
            fn height(&self) -> (levels: usize);
            fn root_rank(&self) -> (r: usize);
            fn is_valid_leftist_heap(&self) -> (b: bool);
            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>();
            fn to_vec(&self) -> (v: Vec<T>);
            fn to_sorted_vec(&self) -> (v: Vec<T>);
            fn meld_multiple(heaps: &Vec<Self>) -> (pq: Self);
            fn split(&self, key: &T) -> (parts: (Self, Self));
        }

// 9. impls
        impl<T: StT + Ord> LeftistHeapNodeTrait<T> for LeftistHeapNode<T> {
            fn rank(&self) -> (r: usize) {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { rank, .. } => *rank,
                }
            }

            fn make_node(key: T, left: LeftistHeapNode<T>, right: LeftistHeapNode<T>) -> (node: Self) {
                let left_rank = left.rank();
                let right_rank = right.rank();
                let (final_left, final_right) = if left_rank >= right_rank {
                    (left, right)
                } else {
                    (right, left)
                };
                let fr = final_right.rank();
                proof { assume(fr < usize::MAX); }
                let node_rank = fr + 1;
                LeftistHeapNode::Node {
                    key,
                    left: Box::new(final_left),
                    right: Box::new(final_right),
                    rank: node_rank,
                }
            }

            /// Core meld operation following right spines (Data Structure 45.3).
            #[verifier::exec_allows_no_decreases_clause]
            fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>) {
                match (a, b) {
                    (LeftistHeapNode::Leaf, other) => other,
                    (other, LeftistHeapNode::Leaf) => other,
                    (
                        LeftistHeapNode::Node { key: ka, left: la, right: ra, .. },
                        LeftistHeapNode::Node { key: kb, left: lb, right: rb, .. },
                    ) => {
                        if ka <= kb {
                            let melded_right = Self::meld_nodes(
                                *ra,
                                LeftistHeapNode::Node { key: kb, left: lb, right: rb, rank: 0 },
                            );
                            Self::make_node(ka, *la, melded_right)
                        } else {
                            let melded_right = Self::meld_nodes(
                                LeftistHeapNode::Node { key: ka, left: la, right: ra, rank: 0 },
                                *rb,
                            );
                            Self::make_node(kb, *lb, melded_right)
                        }
                    }
                }
            }

            fn size(&self) -> (n: usize)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } => {
                        let ls = left.size();
                        let rs = right.size();
                        proof { assume(ls + rs + 1 <= usize::MAX); }
                        1 + ls + rs
                    }
                }
            }

            fn height(&self) -> (h: usize)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } => {
                        let lh = left.height();
                        let rh = right.height();
                        let mh = if lh >= rh { lh } else { rh };
                        proof { assume(mh + 1 <= usize::MAX); }
                        1 + mh
                    }
                }
            }

            fn is_leftist(&self) -> (b: bool)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { left, right, .. } => {
                        left.rank() >= right.rank() && left.is_leftist() && right.is_leftist()
                    }
                }
            }

            fn is_heap(&self) -> (b: bool)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        let left_ok = match &**left {
                            LeftistHeapNode::Leaf => true,
                            LeftistHeapNode::Node { key: left_key, .. } => *key <= *left_key,
                        };
                        let right_ok = match &**right {
                            LeftistHeapNode::Leaf => true,
                            LeftistHeapNode::Node { key: right_key, .. } => *key <= *right_key,
                        };
                        left_ok && right_ok && left.is_heap() && right.is_heap()
                    }
                }
            }

            fn to_vec(&self) -> (v: Vec<T>)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => Vec::new(),
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        let mut result = left.to_vec();
                        result.push(key.clone());
                        let right_vec = right.to_vec();
                        let n = right_vec.len();
                        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                        for i in 0..n
                            invariant n == right_vec@.len()
                        {
                            result.push(right_vec[i].clone());
                        }
                        result
                    }
                }
            }
        }

        impl<T: StT + Ord> LeftistHeapPQTrait<T> for LeftistHeapPQ<T> {
            /// APAS Work Θ(1), Span Θ(1).
            fn empty() -> (pq: Self) {
                LeftistHeapPQ { root: LeftistHeapNode::Leaf }
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn singleton(element: T) -> (pq: Self) {
                LeftistHeapPQ {
                    root: LeftistHeapNode::Node {
                        key: element,
                        left: Box::new(LeftistHeapNode::Leaf),
                        right: Box::new(LeftistHeapNode::Leaf),
                        rank: 1,
                    },
                }
            }

            /// APAS Work Θ(1), Span Θ(1) — root access.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                match &self.root {
                    LeftistHeapNode::Leaf => None,
                    LeftistHeapNode::Node { key, .. } => Some(key),
                }
            }

            /// APAS Work Θ(log n), Span Θ(log n).
            fn insert(&self, element: T) -> (pq: Self) {
                let singleton = Self::singleton(element);
                self.meld(&singleton)
            }

            /// APAS Work Θ(log n), Span Θ(log n).
            fn delete_min(&self) -> (result: (Self, Option<T>)) {
                match &self.root {
                    LeftistHeapNode::Leaf => (self.clone(), None),
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        let min_element = key.clone();
                        let melded_root = LeftistHeapNode::meld_nodes(
                            (**left).clone(), (**right).clone(),
                        );
                        (LeftistHeapPQ { root: melded_root }, Some(min_element))
                    }
                }
            }

            /// APAS Work Θ(log m + log n), Span Θ(log m + log n).
            fn meld(&self, other: &Self) -> (pq: Self) {
                LeftistHeapPQ {
                    root: LeftistHeapNode::meld_nodes(self.root.clone(), other.root.clone()),
                }
            }

            /// APAS Work Θ(n log n), Span Θ(n log n) — sequential insert.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                let n = seq.length();
                let mut pq = Self::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant n == seq@.len()
                {
                    pq = pq.insert(seq.nth(i).clone());
                }
                pq
            }

            fn size(&self) -> (n: usize) { self.root.size() }

            fn is_empty(&self) -> (b: bool) {
                match &self.root {
                    LeftistHeapNode::Leaf => true,
                    _ => false,
                }
            }

            #[verifier::exec_allows_no_decreases_clause]
            fn extract_all_sorted(&self) -> (sorted: Vec<T>) {
                let mut result: Vec<T> = Vec::new();
                let mut current_heap = self.clone();
                while !current_heap.is_empty() {
                    let (new_heap, min_element) = current_heap.delete_min();
                    if let Some(element) = min_element {
                        result.push(element);
                    }
                    current_heap = new_heap;
                }
                result
            }

            fn height(&self) -> (levels: usize) { self.root.height() }

            fn root_rank(&self) -> (r: usize) { self.root.rank() }

            fn is_valid_leftist_heap(&self) -> (b: bool) {
                self.root.is_leftist() && self.root.is_heap()
            }

            fn from_vec(vec: Vec<T>) -> (pq: Self) {
                let seq = ArraySeqStPerS::from_vec(vec);
                Self::from_seq(&seq)
            }

            fn to_vec(&self) -> (v: Vec<T>) { self.root.to_vec() }

            fn to_sorted_vec(&self) -> (v: Vec<T>) { self.extract_all_sorted() }

            fn meld_multiple(heaps: &Vec<Self>) -> (pq: Self) {
                let mut result = Self::empty();
                let n = heaps.len();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant n == heaps@.len()
                {
                    result = result.meld(&heaps[i]);
                }
                result
            }

            fn split(&self, key: &T) -> (parts: (Self, Self)) {
                let all_elements = self.to_vec();
                let mut less_than = Self::empty();
                let mut equal_or_greater = Self::empty();
                let n = all_elements.len();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant n == all_elements@.len()
                {
                    let element = all_elements[i].clone();
                    if element < *key {
                        less_than = less_than.insert(element);
                    } else {
                        equal_or_greater = equal_or_greater.insert(element);
                    }
                }
                (less_than, equal_or_greater)
            }
        }

// 11. derive impls in verus!
        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord> PartialEqSpecImpl for LeftistHeapNode<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self == other }
        }

        impl<T: StT + Ord> Clone for LeftistHeapNode<T> {
            fn clone(&self) -> (result: Self)
                ensures result == *self
                decreases self
            {
                match self {
                    LeftistHeapNode::Leaf => LeftistHeapNode::Leaf,
                    LeftistHeapNode::Node { key, left, right, rank } => {
                        let result = LeftistHeapNode::Node {
                            key: key.clone(),
                            left: Box::new((**left).clone()),
                            right: Box::new((**right).clone()),
                            rank: *rank,
                        };
                        proof { accept(result == *self); }
                        result
                    }
                }
            }
        }

        impl<T: StT + Ord> core::cmp::PartialEq for LeftistHeapNode<T> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (*self == *other)
                decreases self, other
            {
                let r = match (self, other) {
                    (LeftistHeapNode::Leaf, LeftistHeapNode::Leaf) => true,
                    (LeftistHeapNode::Node { key: k1, left: l1, right: r1, rank: rk1 },
                     LeftistHeapNode::Node { key: k2, left: l2, right: r2, rank: rk2 }) => {
                        *k1 == *k2 && (**l1) == (**l2) && (**r1) == (**r2) && *rk1 == *rk2
                    }
                    _ => false,
                };
                proof { accept(r == (*self == *other)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for LeftistHeapNode<T> {}

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord> PartialEqSpecImpl for LeftistHeapPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self.root == other.root }
        }

        impl<T: StT + Ord> Clone for LeftistHeapPQ<T> {
            fn clone(&self) -> (result: Self)
                ensures result.root == self.root
            {
                let result = LeftistHeapPQ { root: self.root.clone() };
                proof { accept(result.root == self.root); }
                result
            }
        }

        impl<T: StT + Ord> core::cmp::PartialEq for LeftistHeapPQ<T> {
            fn eq(&self, other: &Self) -> (r: bool)
                ensures r == (self.root == other.root)
            {
                let r = self.root == other.root;
                proof { accept(r == (self.root == other.root)); }
                r
            }
        }

        impl<T: StT + Ord> core::cmp::Eq for LeftistHeapPQ<T> {}

        impl<T: StT + Ord> Default for LeftistHeapPQ<T> {
            fn default() -> Self { Self::empty() }
        }
    }

// 12. macros
    #[macro_export]
    macro_rules! LeftistHeapPQLit {
        () => {
            $crate::Chap45::LeftistHeapPQ::LeftistHeapPQ::LeftistHeapPQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let elements = vec![$($x),*];
            $crate::Chap45::LeftistHeapPQ::LeftistHeapPQ::LeftistHeapPQ::from_vec(elements)
        }};
    }

// 13. derive impls outside verus!
    impl<T: StT + Ord + Debug> Debug for LeftistHeapNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                LeftistHeapNode::Leaf => write!(f, "Leaf"),
                LeftistHeapNode::Node { key, left, right, rank } => {
                    write!(f, "Node({:?}, {:?}, {:?}, {})", key, left, right, rank)
                }
            }
        }
    }

    impl<T: StT + Ord + Debug> Debug for LeftistHeapPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LeftistHeapPQ({:?})", self.root)
        }
    }

    impl<T: StT + Ord> Display for LeftistHeapPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            fn format_node<T: StT + Ord>(node: &LeftistHeapNode<T>, f: &mut Formatter<'_>, depth: usize) -> Result {
                match node {
                    LeftistHeapNode::Leaf => Ok(()),
                    LeftistHeapNode::Node { key, left, right, rank } => {
                        let indent = "  ".repeat(depth);
                        writeln!(f, "{indent}{key}(rank:{rank})")?;
                        format_node(left, f, depth + 1)?;
                        format_node(right, f, depth + 1)?;
                        Ok(())
                    }
                }
            }

            writeln!(f, "LeftistHeapPQ:")?;
            format_node(&self.root, f, 0)
        }
    }

    /// Efficient multi-way merge using O(log n) meld operations.
    pub fn efficient_multi_way_merge<T: StT + Ord>(sequences: Vec<Vec<T>>) -> Vec<T> {
        let heaps = sequences.into_iter()
            .map(|seq| LeftistHeapPQ::from_vec(seq))
            .collect::<Vec<LeftistHeapPQ<T>>>();
        let merged_heap = LeftistHeapPQ::meld_multiple(&heaps);
        merged_heap.extract_all_sorted()
    }

    /// Parallel heap construction using reduce pattern.
    pub fn parallel_heap_construction<T: StT + Ord>(elements: Vec<T>) -> LeftistHeapPQ<T> {
        LeftistHeapPQ::from_vec(elements)
    }
}
