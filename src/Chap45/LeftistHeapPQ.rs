//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 45: Priority Queue implementation using Leftist Heap (Data Structure 45.3)

//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4. type definitions
//  6. spec fns
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!


pub mod LeftistHeapPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use vstd::multiset::Multiset;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

//  3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::std_specs::vec::group_vec_axioms,
};


//  4. type definitions
        #[verifier::reject_recursive_types(T)]
        pub enum LeftistHeapNode<T: StT + Ord + TotalOrder> {
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
        pub struct LeftistHeapPQ<T: StT + Ord + TotalOrder> {
            pub root: LeftistHeapNode<T>,
        }


//  6. spec fns
        pub open spec fn spec_sorted<T: TotalOrder>(s: Seq<T>) -> bool {
            forall|i: int, j: int| 0 <= i < j < s.len() ==>
                #[trigger] TotalOrder::le(s[i], s[j])
        }

//  7. proof fns/broadcast groups

        proof fn _leftist_heap_pq_verified() {}

        proof fn lemma_total_size_monotone<T: StT + Ord + TotalOrder>(heaps: Seq<LeftistHeapPQ<T>>, j: int, k: int)
            requires 0 <= j <= k <= heaps.len(),
            ensures LeftistHeapPQ::<T>::spec_total_size(heaps, j) <= LeftistHeapPQ::<T>::spec_total_size(heaps, k),
            decreases k - j,
        {
            if j < k {
                lemma_total_size_monotone(heaps, j, k - 1);
            }
        }


//  8. traits

        pub trait LeftistHeapNodeTrait<T: StT + Ord + TotalOrder>: Sized {
            spec fn spec_is_leaf(&self) -> bool;
            spec fn spec_node_size(&self) -> nat;

            fn rank(&self) -> (rank_val: usize)
                ensures self.spec_is_leaf() ==> rank_val == 0;
            fn make_node(key: T, left: LeftistHeapNode<T>, right: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>)
                requires left.spec_size() + right.spec_size() + 1 <= usize::MAX as nat,
                ensures node.spec_size() == left.spec_size() + right.spec_size() + 1;
            fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>)
                requires a.spec_size() + b.spec_size() <= usize::MAX as nat,
                ensures node.spec_size() == a.spec_size() + b.spec_size();
            fn size(&self) -> (n: usize)
                requires self.spec_node_size() <= usize::MAX as nat,
                ensures n as nat == self.spec_node_size();
            fn height(&self) -> (h: usize)
                requires self.spec_node_size() <= usize::MAX as nat,
                ensures
                    self.spec_is_leaf() ==> h == 0,
                    h as nat <= self.spec_node_size();
            fn is_leftist(&self) -> (b: bool)
                ensures self.spec_is_leaf() ==> b;
            fn is_heap(&self) -> (b: bool)
                ensures self.spec_is_leaf() ==> b;
            fn to_vec(&self) -> (v: Vec<T>)
                requires self.spec_node_size() <= usize::MAX as nat,
                ensures v@.len() as nat == self.spec_node_size();
        }

        /// Meldable Priority Queue ADT (Data Type 45.1) using leftist heap.
        pub trait LeftistHeapPQTrait<T: StT + Ord + TotalOrder>: Sized {
            spec fn spec_size(self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;

            fn empty() -> (pq: Self)
                ensures
                    pq.spec_size() == 0,
                    pq.spec_seq().to_multiset() =~= Multiset::empty();
            fn singleton(element: T) -> (pq: Self)
                ensures
                    pq.spec_size() == 1,
                    pq.spec_seq().to_multiset() =~= Multiset::empty().insert(element);
            fn find_min(&self) -> (min_elem: Option<&T>)
                ensures
                    self.spec_size() == 0 ==> min_elem.is_none(),
                    self.spec_size() > 0 ==> min_elem.is_some();
            fn insert(&self, element: T) -> (pq: Self)
                requires self.spec_size() + 1 <= usize::MAX as nat,
                ensures
                    pq.spec_size() == self.spec_size() + 1,
                    pq.spec_seq().to_multiset() =~= self.spec_seq().to_multiset().insert(element);
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires self.spec_size() <= usize::MAX as nat,
                ensures
                    self.spec_size() > 0 ==> min_and_rest.1.is_some(),
                    self.spec_size() > 0 ==> min_and_rest.0.spec_size() == self.spec_size() - 1,
                    self.spec_size() == 0 ==> min_and_rest.1.is_none(),
                    self.spec_size() == 0 ==> min_and_rest.0.spec_size() == self.spec_size(),
                    self.spec_size() > 0 ==> self.spec_seq().to_multiset() =~=
                        min_and_rest.0.spec_seq().to_multiset().insert(min_and_rest.1.unwrap());
            fn meld(&self, other: &Self) -> (pq: Self)
                requires self.spec_size() + other.spec_size() <= usize::MAX as nat,
                ensures
                    pq.spec_size() == self.spec_size() + other.spec_size(),
                    pq.spec_seq().to_multiset() =~= self.spec_seq().to_multiset().add(other.spec_seq().to_multiset());
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures pq.spec_size() == seq@.len();
            fn size(&self) -> (n: usize)
                requires self.spec_size() <= usize::MAX as nat,
                ensures n as nat == self.spec_size();
            fn is_empty(&self) -> (b: bool)
                ensures b == (self.spec_size() == 0);
            fn extract_all_sorted(&self) -> (sorted: Vec<T>)
                requires self.spec_size() <= usize::MAX as nat,
                ensures
                    sorted@.len() as nat == self.spec_size(),
                    spec_sorted(sorted@);
            fn height(&self) -> (levels: usize)
                requires self.spec_size() <= usize::MAX as nat,
                ensures self.spec_size() == 0 ==> levels == 0;
            fn root_rank(&self) -> (rank_val: usize)
                ensures self.spec_size() == 0 ==> rank_val == 0;
            fn is_valid_leftist_heap(&self) -> (b: bool)
                ensures self.spec_size() == 0 ==> b;
            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures pq.spec_size() == vec@.len();
            fn to_vec(&self) -> (v: Vec<T>)
                requires self.spec_size() <= usize::MAX as nat,
                ensures v@.len() as nat == self.spec_size();
            fn to_sorted_vec(&self) -> (v: Vec<T>)
                requires self.spec_size() <= usize::MAX as nat,
                ensures
                    v@.len() as nat == self.spec_size(),
                    spec_sorted(v@);
            spec fn spec_total_size(heaps: Seq<Self>, n: int) -> nat;

            fn meld_multiple(heaps: &Vec<Self>) -> (pq: Self)
                requires Self::spec_total_size(heaps@, heaps@.len() as int) <= usize::MAX as nat,
                ensures pq.spec_size() == Self::spec_total_size(heaps@, heaps@.len() as int);
            fn split(&self, key: &T) -> (parts: (Self, Self))
                requires self.spec_size() <= usize::MAX as nat;
        }


//  9. impls

        impl<T: StT + Ord + TotalOrder> LeftistHeapNode<T> {
            pub open spec fn spec_size(self) -> nat
                decreases self
            {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } =>
                        1 + (*left).spec_size() + (*right).spec_size(),
                }
            }

            pub open spec fn spec_seq(self) -> Seq<T>
                decreases self
            {
                match self {
                    LeftistHeapNode::Leaf => Seq::empty(),
                    LeftistHeapNode::Node { key, left, right, .. } =>
                        Seq::empty().push(key) + (*left).spec_seq() + (*right).spec_seq(),
                }
            }
        }

        impl<T: StT + Ord + TotalOrder> LeftistHeapNodeTrait<T> for LeftistHeapNode<T> {
            open spec fn spec_is_leaf(&self) -> bool {
                matches!(*self, LeftistHeapNode::Leaf)
            }

            open spec fn spec_node_size(&self) -> nat {
                (*self).spec_size()
            }

            fn rank(&self) -> (rank_val: usize) {
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
                // accept hole: rank ≤ log₂(size+1) < usize::MAX; proving requires spec_rank + wf invariant.
                proof { accept(fr < usize::MAX); }
                let node_rank = fr + 1;
                LeftistHeapNode::Node {
                    key,
                    left: Box::new(final_left),
                    right: Box::new(final_right),
                    rank: node_rank,
                }
            }

            /// Core meld operation following right spines (Data Structure 45.3).
            fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>)
                decreases a.spec_size() + b.spec_size()
            {
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
                        let left_len = result.len();
                        result.push(key.clone());
                        let right_vec = right.to_vec();
                        let n = right_vec.len();
                        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                        for i in 0..n
                            invariant
                                n == right_vec@.len(),
                                result@.len() == left_len + 1 + i,
                        {
                            result.push(right_vec[i].clone());
                        }
                        result
                    }
                }
            }
        }

        impl<T: StT + Ord + TotalOrder> LeftistHeapPQTrait<T> for LeftistHeapPQ<T> {
            open spec fn spec_size(self) -> nat {
                self.root.spec_size()
            }

            open spec fn spec_seq(&self) -> Seq<T> {
                self.root.spec_seq()
            }

            open spec fn spec_total_size(heaps: Seq<Self>, n: int) -> nat
                decreases n
            {
                if n <= 0 { 0nat } else { Self::spec_total_size(heaps, n - 1) + heaps[n - 1].spec_size() }
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn empty() -> (pq: Self) {
                let pq = LeftistHeapPQ { root: LeftistHeapNode::Leaf };
                // accept hole: Empty leaf has empty spec_seq.
                proof { accept(pq.spec_seq().to_multiset() =~= Multiset::empty()); }
                pq
            }

            /// APAS Work Θ(1), Span Θ(1).
            fn singleton(element: T) -> (pq: Self) {
                let pq = LeftistHeapPQ {
                    root: LeftistHeapNode::Node {
                        key: element,
                        left: Box::new(LeftistHeapNode::Leaf),
                        right: Box::new(LeftistHeapNode::Leaf),
                        rank: 1,
                    },
                };
                assert(LeftistHeapNode::<T>::Leaf.spec_size() == 0);
                // accept hole: Singleton node spec_seq is [element].
                proof { accept(pq.spec_seq().to_multiset() =~= Multiset::empty().insert(element)); }
                pq
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
                let pq = self.meld(&singleton);
                // accept hole: meld preserves multiset union; singleton is {element}.
                proof { accept(pq.spec_seq().to_multiset() =~= self.spec_seq().to_multiset().insert(element)); }
                pq
            }

            /// APAS Work Θ(log n), Span Θ(log n).
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>)) {
                match &self.root {
                    LeftistHeapNode::Leaf => (self.clone(), None),
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        let min_element = key.clone();
                        let melded_root = LeftistHeapNode::meld_nodes(
                            (**left).clone(), (**right).clone(),
                        );
                        let new_pq = LeftistHeapPQ { root: melded_root };
                        // accept hole: Root removal + meld of children preserves multiset minus root.
                        proof {
                            accept(self.spec_seq().to_multiset() =~=
                                new_pq.spec_seq().to_multiset().insert(min_element));
                        }
                        (new_pq, Some(min_element))
                    }
                }
            }

            /// APAS Work Θ(log m + log n), Span Θ(log m + log n).
            fn meld(&self, other: &Self) -> (pq: Self) {
                let pq = LeftistHeapPQ {
                    root: LeftistHeapNode::meld_nodes(self.root.clone(), other.root.clone()),
                };
                // accept hole: meld_nodes preserves multiset union.
                proof { accept(pq.spec_seq().to_multiset() =~= self.spec_seq().to_multiset().add(other.spec_seq().to_multiset())); }
                pq
            }

            /// APAS Work Θ(n log n), Span Θ(n log n) — sequential insert.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                let n = seq.length();
                let mut pq = Self::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == seq@.len(),
                        pq.spec_size() == i as nat,
                {
                    pq = pq.insert(seq.nth(i).clone());
                }
                pq
            }

            fn size(&self) -> (n: usize) {
                self.root.size()
            }

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
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while !current_heap.is_empty()
                    invariant
                        result@.len() as nat + current_heap.spec_size() == self.spec_size(),
                        self.spec_size() <= usize::MAX as nat,
                {
                    let (new_heap, min_element) = current_heap.delete_min();
                    if let Some(element) = min_element {
                        result.push(element);
                    }
                    current_heap = new_heap;
                }
                // accept hole: Proving sortedness requires heap + leftist invariant (task #9).
                proof { accept(spec_sorted(result@)); }
                result
            }

            fn height(&self) -> (levels: usize) {
                match &self.root {
                    LeftistHeapNode::Leaf => 0,
                    _ => self.root.height(),
                }
            }

            fn root_rank(&self) -> (rank_val: usize) {
                match &self.root {
                    LeftistHeapNode::Leaf => 0,
                    _ => self.root.rank(),
                }
            }

            fn is_valid_leftist_heap(&self) -> (b: bool) {
                match &self.root {
                    LeftistHeapNode::Leaf => true,
                    _ => self.root.is_leftist() && self.root.is_heap(),
                }
            }

            fn from_vec(vec: Vec<T>) -> (pq: Self) {
                let seq = ArraySeqStPerS::from_vec(vec);
                Self::from_seq(&seq)
            }

            fn to_vec(&self) -> (v: Vec<T>) {
                match &self.root {
                    LeftistHeapNode::Leaf => Vec::new(),
                    _ => self.root.to_vec(),
                }
            }

            fn to_sorted_vec(&self) -> (v: Vec<T>) { self.extract_all_sorted() }

            fn meld_multiple(heaps: &Vec<Self>) -> (pq: Self) {
                let mut result = Self::empty();
                let n = heaps.len();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == heaps@.len(),
                        result.spec_size() == Self::spec_total_size(heaps@, i as int),
                        Self::spec_total_size(heaps@, heaps@.len() as int) <= usize::MAX as nat,
                {
                    proof { lemma_total_size_monotone::<T>(heaps@, (i + 1) as int, n as int); }
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
                    invariant
                        n == all_elements@.len(),
                        n as nat == self.spec_size(),
                        self.spec_size() <= usize::MAX as nat,
                        less_than.spec_size() + equal_or_greater.spec_size() == i as nat,
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

//  11. derive impls in verus!

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for LeftistHeapNode<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self == other }
        }

        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for LeftistHeapPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self.root == other.root }
        }

        impl<T: StT + Ord + TotalOrder> Default for LeftistHeapPQ<T> {
            fn default() -> Self { Self::empty() }
        }


        impl<T: StT + Ord + TotalOrder> Clone for LeftistHeapNode<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned == *self
                decreases self
            {
                match self {
                    LeftistHeapNode::Leaf => LeftistHeapNode::Leaf,
                    LeftistHeapNode::Node { key, left, right, rank } => {
                        let cloned = LeftistHeapNode::Node {
                            key: key.clone(),
                            left: Box::new((**left).clone()),
                            right: Box::new((**right).clone()),
                            rank: *rank,
                        };
                        proof { accept(cloned == *self); }
                        cloned
                    }
                }
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for LeftistHeapNode<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (*self == *other)
                decreases self, other
            {
                let equal = match (self, other) {
                    (LeftistHeapNode::Leaf, LeftistHeapNode::Leaf) => true,
                    (LeftistHeapNode::Node { key: k1, left: l1, right: r1, rank: rk1 },
                     LeftistHeapNode::Node { key: k2, left: l2, right: r2, rank: rk2 }) => {
                        *k1 == *k2 && (**l1) == (**l2) && (**r1) == (**r2) && *rk1 == *rk2
                    }
                    _ => false,
                };
                proof { accept(equal == (*self == *other)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for LeftistHeapNode<T> {}

        impl<T: StT + Ord + TotalOrder> Clone for LeftistHeapPQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned.root == self.root
            {
                let cloned = LeftistHeapPQ { root: self.root.clone() };
                proof { accept(cloned.root == self.root); }
                cloned
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for LeftistHeapPQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self.root == other.root)
            {
                let equal = self.root == other.root;
                proof { accept(equal == (self.root == other.root)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for LeftistHeapPQ<T> {}

    }

//  12. macros

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

//  13. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder + Debug> Debug for LeftistHeapNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                LeftistHeapNode::Leaf => write!(f, "Leaf"),
                LeftistHeapNode::Node { key, left, right, rank } => {
                    write!(f, "Node({:?}, {:?}, {:?}, {})", key, left, right, rank)
                }
            }
        }
    }

    impl<T: StT + Ord + TotalOrder> Display for LeftistHeapNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            fn format_node<T: StT + Ord + TotalOrder>(node: &LeftistHeapNode<T>, f: &mut Formatter<'_>, depth: usize) -> Result {
                match node {
                    LeftistHeapNode::Leaf => Ok(()),
                    LeftistHeapNode::Node { key, left, right, rank } => {
                        let indent = "  ".repeat(depth);
                        writeln!(f, "{indent}{key}(rank:{rank})")?;
                        format_node(left, f, depth + 1)?;
                        format_node(right, f, depth + 1)
                    }
                }
            }
            format_node(self, f, 0)
        }
    }

    impl<T: StT + Ord + TotalOrder + Debug> Debug for LeftistHeapPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LeftistHeapPQ({:?})", self.root)
        }
    }

    impl<T: StT + Ord + TotalOrder> Display for LeftistHeapPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            fn format_node<T: StT + Ord + TotalOrder>(node: &LeftistHeapNode<T>, f: &mut Formatter<'_>, depth: usize) -> Result {
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
    pub fn efficient_multi_way_merge<T: StT + Ord + TotalOrder>(sequences: Vec<Vec<T>>) -> Vec<T> {
        let heaps = sequences.into_iter()
            .map(|seq| LeftistHeapPQ::from_vec(seq))
            .collect::<Vec<LeftistHeapPQ<T>>>();
        let merged_heap = LeftistHeapPQ::meld_multiple(&heaps);
        merged_heap.extract_all_sorted()
    }

    /// Parallel heap construction using reduce pattern.
    pub fn parallel_heap_construction<T: StT + Ord + TotalOrder>(elements: Vec<T>) -> LeftistHeapPQ<T> {
        LeftistHeapPQ::from_vec(elements)
    }
}
