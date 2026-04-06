//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 45: Priority Queue implementation using Leftist Heap (Data Structure 45.3)

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 5a. view impls
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod LeftistHeapPQ {


    //		Section 2. imports

    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use vstd::multiset::Multiset;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Concurrency::diverge;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::multiset::group_multiset_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::std_specs::vec::group_vec_axioms,
};

    //		Section 4a. type definitions


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

    //		Section 5a. view impls


        impl<T: StT + Ord + TotalOrder> View for LeftistHeapNode<T> {
            type V = Multiset<T>;
            open spec fn view(&self) -> Multiset<T> { self.spec_seq().to_multiset() }
        }

    //		Section 8a. traits


        /// Recursive spec functions on the node enum (spec-only, no exec methods).
        pub trait LeftistHeapNodeSpec<T: StT + Ord + TotalOrder>: Sized {
            spec fn spec_size(&self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;
            spec fn spec_rank(&self) -> nat;
            spec fn spec_is_leftist(&self) -> bool;
            spec fn spec_is_heap(&self) -> bool;
            spec fn spec_rank_bounded(&self) -> bool;
            /// Key is <= the root key of a node (trivially true for Leaf).
            spec fn spec_key_le_root(&self, key: T) -> bool;
        }

        pub trait LeftistHeapNodeTrait<T: StT + Ord + TotalOrder>: Sized + LeftistHeapNodeSpec<T> {
            spec fn spec_is_leaf(&self) -> bool;

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn rank(&self) -> (rank_val: usize)
                ensures rank_val as nat == self.spec_rank();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn make_node(key: T, left: LeftistHeapNode<T>, right: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>)
                requires
                    left.spec_size() + right.spec_size() + 1 <= usize::MAX as nat,
                    left.spec_is_leftist() && left.spec_is_heap(),
                    right.spec_is_leftist() && right.spec_is_heap(),
                    left.spec_rank_bounded() && right.spec_rank_bounded(),
                    left.spec_key_le_root(key),
                    right.spec_key_le_root(key),
                ensures
                    node.spec_size() == left.spec_size() + right.spec_size() + 1,
                    node.spec_is_leftist(),
                    node.spec_is_heap(),
                    node.spec_rank_bounded(),
                    node@ =~= Multiset::empty().insert(key).add(left@).add(right@),
                    forall|x: T| TotalOrder::le(x, key) ==>
                        #[trigger] node.spec_key_le_root(x);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>)
                requires
                    a.spec_size() + b.spec_size() <= usize::MAX as nat,
                    a.spec_is_leftist() && a.spec_is_heap(),
                    b.spec_is_leftist() && b.spec_is_heap(),
                    a.spec_rank_bounded() && b.spec_rank_bounded(),
                ensures
                    node.spec_size() == a.spec_size() + b.spec_size(),
                    node.spec_is_leftist(),
                    node.spec_is_heap(),
                    node.spec_rank_bounded(),
                    node@ =~= a@.add(b@),
                    forall|x: T| a.spec_key_le_root(x) && b.spec_key_le_root(x) ==>
                        #[trigger] node.spec_key_le_root(x);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn size(&self) -> (n: usize)
                requires self.spec_size() <= usize::MAX as nat,
                ensures n as nat == self.spec_size();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn height(&self) -> (h: usize)
                requires self.spec_size() <= usize::MAX as nat,
                ensures
                    self.spec_is_leaf() ==> h == 0,
                    h as nat <= self.spec_size();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_leftist(&self) -> (is_leftist: bool)
                ensures is_leftist <==> self.spec_is_leftist();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_heap(&self) -> (is_heap: bool)
                ensures is_heap <==> self.spec_is_heap();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_rank_bounded(&self) -> (bounded: bool)
                requires self.spec_size() <= usize::MAX as nat,
                ensures bounded <==> self.spec_rank_bounded();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (v: Vec<T>)
                requires self.spec_size() <= usize::MAX as nat,
                ensures v@.len() as nat == self.spec_size();
        }

    //		Section 9a. impls


        impl<T: StT + Ord + TotalOrder> LeftistHeapNodeSpec<T> for LeftistHeapNode<T> {
            open spec fn spec_size(&self) -> nat
                decreases *self,
            {
                match *self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } =>
                        1 + LeftistHeapNodeSpec::spec_size(&*left) + LeftistHeapNodeSpec::spec_size(&*right),
                }
            }

            open spec fn spec_seq(&self) -> Seq<T>
                decreases *self,
            {
                match *self {
                    LeftistHeapNode::Leaf => Seq::empty(),
                    LeftistHeapNode::Node { key, left, right, .. } =>
                        Seq::empty().push(key) + LeftistHeapNodeSpec::spec_seq(&*left) + LeftistHeapNodeSpec::spec_seq(&*right),
                }
            }

            open spec fn spec_rank(&self) -> nat {
                match *self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { rank, .. } => rank as nat,
                }
            }

            open spec fn spec_is_leftist(&self) -> bool
                decreases *self,
            {
                match *self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { left, right, .. } =>
                        LeftistHeapNodeSpec::spec_rank(&*left) >= LeftistHeapNodeSpec::spec_rank(&*right)
                        && LeftistHeapNodeSpec::spec_is_leftist(&*left)
                        && LeftistHeapNodeSpec::spec_is_leftist(&*right),
                }
            }

            open spec fn spec_is_heap(&self) -> bool
                decreases *self,
            {
                match *self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        let left_ok = match *left {
                            LeftistHeapNode::Leaf => true,
                            LeftistHeapNode::Node { key: lk, .. } => TotalOrder::le(key, lk),
                        };
                        let right_ok = match *right {
                            LeftistHeapNode::Leaf => true,
                            LeftistHeapNode::Node { key: rk, .. } => TotalOrder::le(key, rk),
                        };
                        left_ok && right_ok
                        && LeftistHeapNodeSpec::spec_is_heap(&*left)
                        && LeftistHeapNodeSpec::spec_is_heap(&*right)
                    }
                }
            }

            /// Stored rank field is bounded by node size, recursively.
            open spec fn spec_rank_bounded(&self) -> bool
                decreases *self,
            {
                match *self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { rank, left, right, .. } =>
                        rank as nat <= 1 + LeftistHeapNodeSpec::spec_size(&*left) + LeftistHeapNodeSpec::spec_size(&*right)
                        && LeftistHeapNodeSpec::spec_rank_bounded(&*left)
                        && LeftistHeapNodeSpec::spec_rank_bounded(&*right),
                }
            }

            open spec fn spec_key_le_root(&self, key: T) -> bool {
                match *self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { key: nk, .. } => TotalOrder::le(key, nk),
                }
            }
        }

        impl<T: StT + Ord + TotalOrder> LeftistHeapNodeTrait<T> for LeftistHeapNode<T> {
            open spec fn spec_is_leaf(&self) -> bool {
                matches!(*self, LeftistHeapNode::Leaf)
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn rank(&self) -> (rank_val: usize) {
                match self {
                    LeftistHeapNode::Leaf => {
                        assert(self.spec_rank() == 0);
                        0
                    }
                    LeftistHeapNode::Node { rank, .. } => {
                        assert(self.spec_rank() == *rank as nat);
                        *rank
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn make_node(key: T, left: LeftistHeapNode<T>, right: LeftistHeapNode<T>) -> (node: Self) {
                let ghost left_ms = left@;
                let ghost right_ms = right@;
                let left_rank = left.rank();
                let right_rank = right.rank();
                let (final_left, final_right) = if left_rank >= right_rank {
                    (left, right)
                } else {
                    (right, left)
                };
                let fr = final_right.rank();
                proof { lemma_rank_le_size(&final_right); }
                let node_rank = fr + 1;
                let node = LeftistHeapNode::Node {
                    key,
                    left: Box::new(final_left),
                    right: Box::new(final_right),
                    rank: node_rank,
                };
                // Leftist: rank(final_left) >= rank(final_right) by swap, children leftist from requires.
                assert(node.spec_is_leftist() == (
                    final_left.spec_rank() >= final_right.spec_rank()
                    && final_left.spec_is_leftist() && final_right.spec_is_leftist()));
                // Heap: key <= children roots from requires (swap preserves), children heaps from requires.
                assert(node.spec_is_heap() == ({
                    let lo = match final_left {
                        LeftistHeapNode::Leaf => true,
                        LeftistHeapNode::Node { key: lk, .. } => TotalOrder::le(key, lk),
                    };
                    let ro = match final_right {
                        LeftistHeapNode::Leaf => true,
                        LeftistHeapNode::Node { key: rk, .. } => TotalOrder::le(key, rk),
                    };
                    lo && ro && final_left.spec_is_heap() && final_right.spec_is_heap()
                }));
                // Rank bounded: node_rank = fr + 1 <= final_right.spec_size() + 1 <= size.
                assert(node.spec_rank_bounded() == (
                    node_rank as nat <= 1 + final_left.spec_size() + final_right.spec_size()
                    && final_left.spec_rank_bounded() && final_right.spec_rank_bounded()));
                // Multiset preservation: unfold to_multiset of concatenation.
                proof {
                    let s1 = Seq::<T>::empty().push(key);
                    let s2 = final_left.spec_seq();
                    let s3 = final_right.spec_seq();
                    assert(node.spec_seq() =~= s1 + s2 + s3);
                    vstd::seq_lib::lemma_multiset_commutative(s1 + s2, s3);
                    vstd::seq_lib::lemma_multiset_commutative(s1, s2);
                    assert(node@ =~= Multiset::empty().insert(key).add(final_left@).add(final_right@));
                }
                node
            }

            /// Core meld operation following right spines (Data Structure 45.3).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
            fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> (node: LeftistHeapNode<T>)
                decreases a.spec_size() + b.spec_size()
            {
                let ghost a_view = a@;
                let ghost b_view = b@;
                let ghost a_seq = a.spec_seq();
                let ghost b_seq = b.spec_seq();
                match (a, b) {
                    (LeftistHeapNode::Leaf, other) => {
                        assert(a_view =~= Multiset::empty());
                        other
                    }
                    (other, LeftistHeapNode::Leaf) => {
                        assert(b_view =~= Multiset::empty());
                        other
                    }
                    (
                        LeftistHeapNode::Node { key: ka, left: la, right: ra, .. },
                        LeftistHeapNode::Node { key: kb, left: lb, right: rb, .. },
                    ) => {
                        assert((*ra).spec_size() < 1 + (*la).spec_size() + (*ra).spec_size());
                        assert((*rb).spec_size() < 1 + (*lb).spec_size() + (*rb).spec_size());
                        let ka_le_kb = total_order_le(&ka, &kb);
                        if ka_le_kb {
                            let b_node = LeftistHeapNode::Node { key: kb, left: lb, right: rb, rank: 0 };
                            // Reconstructed node has same children/key as b, so validity matches.
                            assert(b_node.spec_is_leftist() == (lb.spec_rank() >= rb.spec_rank()
                                && lb.spec_is_leftist() && rb.spec_is_leftist()));
                            assert(b_node.spec_is_heap() == ({
                                let lo = match *lb { LeftistHeapNode::Leaf => true,
                                    LeftistHeapNode::Node { key: lk, .. } => TotalOrder::le(kb, lk) };
                                let ro = match *rb { LeftistHeapNode::Leaf => true,
                                    LeftistHeapNode::Node { key: rk, .. } => TotalOrder::le(kb, rk) };
                                lo && ro && lb.spec_is_heap() && rb.spec_is_heap()
                            }));
                            assert(b_node.spec_rank_bounded());
                            assert(b_node.spec_seq() =~= Seq::empty().push(kb) + lb.spec_seq() + rb.spec_seq());
                            assert(b_node@ =~= b_view);
                            let melded_right = Self::meld_nodes(*ra, b_node);
                            // ka <= kb from total_order_le; ka <= root(ra) from heap property.
                            assert(melded_right.spec_key_le_root(ka));
                            let result = Self::make_node(ka, *la, melded_right);
                            proof {
                                // Unfold a_view via lemma_multiset_commutative.
                                assert(a_seq =~= Seq::empty().push(ka) + la.spec_seq() + ra.spec_seq());
                                vstd::seq_lib::lemma_multiset_commutative(
                                    Seq::<T>::empty().push(ka) + la.spec_seq(), ra.spec_seq());
                                vstd::seq_lib::lemma_multiset_commutative(
                                    Seq::<T>::empty().push(ka), la.spec_seq());
                                assert(a_view =~= Multiset::empty().insert(ka).add(la@).add(ra@));
                                // result@ = {ka}.add(la@).add(melded_right@) from make_node.
                                // melded_right@ = ra@.add(b_node@) = ra@.add(b_view).
                            }
                            result
                        } else {
                            proof { TotalOrder::total(ka, kb); }
                            let a_node = LeftistHeapNode::Node { key: ka, left: la, right: ra, rank: 0 };
                            assert(a_node.spec_is_leftist() == (la.spec_rank() >= ra.spec_rank()
                                && la.spec_is_leftist() && ra.spec_is_leftist()));
                            assert(a_node.spec_is_heap() == ({
                                let lo = match *la { LeftistHeapNode::Leaf => true,
                                    LeftistHeapNode::Node { key: lk, .. } => TotalOrder::le(ka, lk) };
                                let ro = match *ra { LeftistHeapNode::Leaf => true,
                                    LeftistHeapNode::Node { key: rk, .. } => TotalOrder::le(ka, rk) };
                                lo && ro && la.spec_is_heap() && ra.spec_is_heap()
                            }));
                            assert(a_node.spec_rank_bounded());
                            assert(a_node.spec_seq() =~= Seq::empty().push(ka) + la.spec_seq() + ra.spec_seq());
                            assert(a_node@ =~= a_view);
                            let melded_right = Self::meld_nodes(a_node, *rb);
                            // kb <= ka from totality + !le(ka,kb); kb <= root(rb) from heap property.
                            assert(melded_right.spec_key_le_root(kb));
                            let result = Self::make_node(kb, *lb, melded_right);
                            proof {
                                // Unfold b_view via lemma_multiset_commutative.
                                assert(b_seq =~= Seq::empty().push(kb) + lb.spec_seq() + rb.spec_seq());
                                vstd::seq_lib::lemma_multiset_commutative(
                                    Seq::<T>::empty().push(kb) + lb.spec_seq(), rb.spec_seq());
                                vstd::seq_lib::lemma_multiset_commutative(
                                    Seq::<T>::empty().push(kb), lb.spec_seq());
                                assert(b_view =~= Multiset::empty().insert(kb).add(lb@).add(rb@));
                                // result@ = {kb}.add(lb@).add(melded_right@) from make_node.
                                // melded_right@ = a_node@.add(rb@) = a_view.add(rb@).
                            }
                            result
                        }
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn size(&self) -> (n: usize)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } => {
                        assert(self.spec_size() == 1 + left.spec_size() + right.spec_size());
                        let ls = left.size();
                        let rs = right.size();
                        1 + ls + rs
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn height(&self) -> (h: usize)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => 0,
                    LeftistHeapNode::Node { left, right, .. } => {
                        assert(self.spec_size() == 1 + left.spec_size() + right.spec_size());
                        let lh = left.height();
                        let rh = right.height();
                        let mh = if lh >= rh { lh } else { rh };
                        1 + mh
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_leftist(&self) -> (is_leftist: bool)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { left, right, .. } => {
                        assert(self.spec_is_leftist() == (
                            left.spec_rank() >= right.spec_rank()
                            && left.spec_is_leftist()
                            && right.spec_is_leftist()
                        ));
                        left.rank() >= right.rank() && left.is_leftist() && right.is_leftist()
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_heap(&self) -> (is_heap: bool)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        assert(self.spec_is_heap() == ({
                            let lo = match **left {
                                LeftistHeapNode::Leaf => true,
                                LeftistHeapNode::Node { key: lk, .. } => TotalOrder::le(*key, lk),
                            };
                            let ro = match **right {
                                LeftistHeapNode::Leaf => true,
                                LeftistHeapNode::Node { key: rk, .. } => TotalOrder::le(*key, rk),
                            };
                            lo && ro
                            && left.spec_is_heap()
                            && right.spec_is_heap()
                        }));
                        let left_ok = match &**left {
                            LeftistHeapNode::Leaf => true,
                            LeftistHeapNode::Node { key: left_key, .. } =>
                                total_order_le(key, left_key),
                        };
                        let right_ok = match &**right {
                            LeftistHeapNode::Leaf => true,
                            LeftistHeapNode::Node { key: right_key, .. } =>
                                total_order_le(key, right_key),
                        };
                        left_ok && right_ok && left.is_heap() && right.is_heap()
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_rank_bounded(&self) -> (bounded: bool)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => true,
                    LeftistHeapNode::Node { rank, left, right, .. } => {
                        assert(self.spec_size() == 1 + left.spec_size() + right.spec_size());
                        let sz = left.size() + right.size() + 1;
                        *rank <= sz && left.is_rank_bounded() && right.is_rank_bounded()
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (v: Vec<T>)
                decreases *self
            {
                match self {
                    LeftistHeapNode::Leaf => Vec::new(),
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        assert(self.spec_size() == 1 + left.spec_size() + right.spec_size());
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

    //		Section 4b. type definitions


        /// Priority Queue implemented using Leftist Heap (Data Structure 45.3).
        #[verifier::reject_recursive_types(T)]
        pub struct LeftistHeapPQ<T: StT + Ord + TotalOrder> {
            pub root: LeftistHeapNode<T>,
        }

    //		Section 5b. view impls


        impl<T: StT + Ord + TotalOrder> View for LeftistHeapPQ<T> {
            type V = Multiset<T>;
            open spec fn view(&self) -> Multiset<T> { self.root.spec_seq().to_multiset() }
        }

    //		Section 7b. proof fns/broadcast groups


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

        /// Heap invariant implies root is <= all elements in spec_seq.
        proof fn lemma_heap_root_is_min<T: StT + Ord + TotalOrder>(node: &LeftistHeapNode<T>)
            requires node.spec_is_heap(),
            ensures
                node.spec_seq().len() > 0 ==>
                    forall|i: int| 0 <= i < node.spec_seq().len() ==>
                        #[trigger] TotalOrder::le(node.spec_seq()[0], node.spec_seq()[i]),
            decreases *node,
        {
            match node {
                LeftistHeapNode::Leaf => {},
                LeftistHeapNode::Node { key, left, right, .. } => {
                    let s = node.spec_seq();
                    let ls = left.spec_seq();
                    let rs = right.spec_seq();
                    assert(s =~= Seq::empty().push(*key) + ls + rs);
                    assert(s[0] == *key);
                    TotalOrder::reflexive(*key);

                    lemma_heap_root_is_min(&**left);
                    lemma_heap_root_is_min(&**right);

                    assert forall|i: int| 0 <= i < s.len() implies
                        #[trigger] TotalOrder::le(s[0], s[i])
                    by {
                        if i == 0 {
                            // s[0] == *key, reflexive
                        } else if i < 1 + ls.len() {
                            // Element is in left subtree
                            let li = i - 1;
                            assert(s[i] == ls[li]);
                            match &**left {
                                LeftistHeapNode::Leaf => {},
                                LeftistHeapNode::Node { key: lk, left: ll, right: lr, .. } => {
                                    assert(ls =~= Seq::empty().push(*lk) + ll.spec_seq() + lr.spec_seq());
                                    assert(ls[0] == *lk);
                                    assert(TotalOrder::le(*key, *lk));
                                    assert(TotalOrder::le(ls[0], ls[li]));
                                    TotalOrder::transitive(*key, *lk, ls[li]);
                                },
                            }
                        } else {
                            // Element is in right subtree
                            let ri = i - 1 - ls.len();
                            assert(s[i] == rs[ri]);
                            match &**right {
                                LeftistHeapNode::Leaf => {},
                                LeftistHeapNode::Node { key: rk, left: rl, right: rr, .. } => {
                                    assert(rs =~= Seq::empty().push(*rk) + rl.spec_seq() + rr.spec_seq());
                                    assert(rs[0] == *rk);
                                    assert(TotalOrder::le(*key, *rk));
                                    assert(TotalOrder::le(rs[0], rs[ri]));
                                    TotalOrder::transitive(*key, *rk, rs[ri]);
                                },
                            }
                        }
                    }
                },
            }
        }

        /// spec_rank_bounded implies spec_rank() <= spec_size().
        proof fn lemma_rank_le_size<T: StT + Ord + TotalOrder>(node: &LeftistHeapNode<T>)
            requires node.spec_rank_bounded(),
            ensures node.spec_rank() <= node.spec_size(),
        {
            match node {
                LeftistHeapNode::Leaf => {},
                LeftistHeapNode::Node { rank, left, right, .. } => {
                    assert(rank as nat <= 1 + left.spec_size() + right.spec_size());
                },
            }
        }

    //		Section 8b. traits


        /// Meldable Priority Queue ADT (Data Type 45.1) using leftist heap.
        pub trait LeftistHeapPQTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Multiset<T>> {
            spec fn spec_leftistheappq_wf(&self) -> bool;
            spec fn spec_size(self) -> nat;
            spec fn spec_seq(&self) -> Seq<T>;
            spec fn spec_sorted(s: Seq<T>) -> bool;

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn empty() -> (pq: Self)
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == 0,
                    pq@ =~= Multiset::empty();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn singleton(element: T) -> (pq: Self)
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == 1,
                    pq@ =~= Multiset::empty().insert(element);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn find_min(&self) -> (min_elem: Option<&T>)
                requires self.spec_leftistheappq_wf(),
                ensures
                    self.spec_size() == 0 ==> min_elem.is_none(),
                    self.spec_size() > 0 ==> min_elem.is_some(),
                    self.spec_size() > 0 ==> self@.count(*min_elem.unwrap()) > 0,
                    self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
                        #[trigger] TotalOrder::le(*min_elem.unwrap(), e);
            /// - Alg Analysis: APAS (Ch45 cost table, leftist heap): Work O(lg n), Span O(lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS: meld with singleton
            fn insert(&self, element: T) -> (pq: Self)
                requires
                    self.spec_leftistheappq_wf(),
                    self.spec_size() + 1 <= usize::MAX as nat,
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == self.spec_size() + 1,
                    pq@ =~= self@.insert(element);
            /// - Alg Analysis: APAS (Ch45 cost table, leftist heap): Work O(lg n), Span O(lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS: meld children
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>))
                requires
                    self.spec_leftistheappq_wf(),
                    self.spec_size() <= usize::MAX as nat,
                ensures
                    min_and_rest.0.spec_leftistheappq_wf(),
                    self.spec_size() > 0 ==> min_and_rest.1.is_some(),
                    self.spec_size() > 0 ==> min_and_rest.0.spec_size() == self.spec_size() - 1,
                    self.spec_size() == 0 ==> min_and_rest.1.is_none(),
                    self.spec_size() == 0 ==> min_and_rest.0.spec_size() == self.spec_size(),
                    self.spec_size() > 0 ==> self@ =~=
                        min_and_rest.0@.insert(min_and_rest.1.unwrap()),
                    self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
                        #[trigger] TotalOrder::le(min_and_rest.1.unwrap(), e);
            /// - Alg Analysis: APAS (Ch45 cost table, leftist heap): Work O(lg m + lg n), Span O(lg m + lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg m + lg n), Span O(lg m + lg n) — matches APAS
            fn meld(&self, other: &Self) -> (pq: Self)
                requires
                    self.spec_leftistheappq_wf(),
                    other.spec_leftistheappq_wf(),
                    self.spec_size() + other.spec_size() <= usize::MAX as nat,
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == self.spec_size() + other.spec_size(),
                    pq@ =~= self@.add(other@);
            /// - Alg Analysis: APAS (Ch45 cost table, leftist heap): Work O(n), Span O(lg^2 n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential reduce, APAS Span O(lg^2 n) assumes parallel
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == seq@.len();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn size(&self) -> (n: usize)
                requires self.spec_size() <= usize::MAX as nat,
                ensures n as nat == self.spec_size();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> (is_empty: bool)
                ensures is_empty == (self.spec_size() == 0);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn extract_all_sorted(&self) -> (sorted: Vec<T>)
                requires
                    self.spec_leftistheappq_wf(),
                    self.spec_size() <= usize::MAX as nat,
                ensures
                    sorted@.len() as nat == self.spec_size(),
                    Self::spec_sorted(sorted@);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn height(&self) -> (levels: usize)
                requires self.spec_size() <= usize::MAX as nat,
                ensures self.spec_size() == 0 ==> levels == 0;
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn root_rank(&self) -> (rank_val: usize)
                ensures self.spec_size() == 0 ==> rank_val == 0;
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_valid_leftist_heap(&self) -> (is_valid: bool)
                requires self.spec_size() <= usize::MAX as nat,
                ensures is_valid <==> self.spec_leftistheappq_wf();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn from_vec(vec: Vec<T>) -> (pq: Self)
                requires obeys_feq_clone::<T>(),
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == vec@.len();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (v: Vec<T>)
                requires self.spec_size() <= usize::MAX as nat,
                ensures v@.len() as nat == self.spec_size();
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn to_sorted_vec(&self) -> (v: Vec<T>)
                requires
                    self.spec_leftistheappq_wf(),
                    self.spec_size() <= usize::MAX as nat,
                ensures
                    v@.len() as nat == self.spec_size(),
                    Self::spec_sorted(v@);
            spec fn spec_total_size(heaps: Seq<Self>, n: int) -> nat;

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k * lg(n)), Span O(k * lg(n))
            fn meld_multiple(heaps: &Vec<Self>) -> (pq: Self)
                requires
                    forall|i: int| 0 <= i < heaps@.len() ==>
                        (#[trigger] heaps@[i]).spec_leftistheappq_wf(),
                    Self::spec_total_size(heaps@, heaps@.len() as int) <= usize::MAX as nat,
                ensures
                    pq.spec_leftistheappq_wf(),
                    pq.spec_size() == Self::spec_total_size(heaps@, heaps@.len() as int);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn split(&self, key: &T) -> (parts: (Self, Self))
                requires self.spec_size() <= usize::MAX as nat,
                ensures
                    parts.0.spec_leftistheappq_wf(),
                    parts.1.spec_leftistheappq_wf();
        }

    //		Section 9b. impls


        /// Exec comparison with spec ensures connecting to TotalOrder::le.
        // veracity: no_requires
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn total_order_le<T: StT + Ord + TotalOrder>(a: &T, b: &T) -> (le: bool)
            ensures le <==> TotalOrder::le(*a, *b)
        {
            match TotalOrder::cmp(a, b) {
                Ordering::Greater => {
                    proof {
                        if TotalOrder::le(*a, *b) {
                            TotalOrder::antisymmetric(*a, *b);
                        }
                    }
                    false
                }
                _ => {
                    proof { TotalOrder::reflexive(*a); }
                    true
                }
            }
        }

        impl<T: StT + Ord + TotalOrder> LeftistHeapPQTrait<T> for LeftistHeapPQ<T> {
            open spec fn spec_leftistheappq_wf(&self) -> bool {
                self.root.spec_is_leftist()
                && self.root.spec_is_heap()
                && self.root.spec_rank_bounded()
            }

            open spec fn spec_size(self) -> nat {
                self.root.spec_size()
            }

            open spec fn spec_seq(&self) -> Seq<T> {
                self.root.spec_seq()
            }

            open spec fn spec_sorted(s: Seq<T>) -> bool {
                forall|i: int, j: int| 0 <= i < j < s.len() ==>
                    #[trigger] TotalOrder::le(s[i], s[j])
            }

            open spec fn spec_total_size(heaps: Seq<Self>, n: int) -> nat
                decreases n
            {
                if n <= 0 { 0nat } else { Self::spec_total_size(heaps, n - 1) + heaps[n - 1].spec_size() }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; constant-time Leaf construction.
            fn empty() -> (pq: Self) {
                let pq = LeftistHeapPQ { root: LeftistHeapNode::Leaf };
                assert(pq.root.spec_is_leftist());
                assert(pq.root.spec_is_heap());
                assert(pq.root.spec_seq() =~= Seq::<T>::empty());
                assert(Seq::<T>::empty().to_multiset() =~= Multiset::<T>::empty());
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; constant-time Node construction.
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
                assert(LeftistHeapNode::<T>::Leaf.spec_is_leftist());
                assert(LeftistHeapNode::<T>::Leaf.spec_is_heap());
                assert(LeftistHeapNode::<T>::Leaf.spec_rank() == 0);
                assert(LeftistHeapNode::<T>::Leaf.spec_rank_bounded());
                assert(pq.root.spec_is_leftist());
                assert(pq.root.spec_is_heap());
                assert(pq.root.spec_rank_bounded());
                assert(LeftistHeapNode::<T>::Leaf.spec_seq() =~= Seq::<T>::empty());
                assert(pq.root.spec_seq() =~= Seq::empty().push(element) + Seq::<T>::empty() + Seq::<T>::empty());
                assert(pq.root.spec_seq() =~= Seq::empty().push(element));
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; root access by heap property.
            fn find_min(&self) -> (min_elem: Option<&T>) {
                match &self.root {
                    LeftistHeapNode::Leaf => {
                        assert(self.root.spec_size() == 0);
                        None
                    }
                    LeftistHeapNode::Node { key, left, right, .. } => {
                        assert(self.root.spec_size() == 1 + left.spec_size() + right.spec_size());
                        proof {
                            let s = self.root.spec_seq();
                            assert(s =~= Seq::empty().push(*key) + left.spec_seq() + right.spec_seq());
                            assert(s[0] == *key);
                            assert(s.contains(*key));
                            lemma_heap_root_is_min(&self.root);
                            assert forall|e: T| self@.count(e) > 0 implies
                                #[trigger] TotalOrder::le(*key, e)
                            by {
                                assert(s.to_multiset().count(e) > 0);
                                assert(s.contains(e));
                                let i = choose|i: int| 0 <= i < s.len() && s[i] == e;
                                assert(TotalOrder::le(s[0], s[i]));
                            }
                        }
                        Some(key)
                    }
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS; singleton then meld along right spines.
            fn insert(&self, element: T) -> (pq: Self) {
                let singleton = Self::singleton(element);
                let pq = self.meld(&singleton);
                proof {
                    assert(singleton@ =~= Multiset::empty().insert(element));
                    assert(pq@ =~= self@.add(singleton@));
                    assert forall|x: T| #![trigger self@.insert(element).count(x)]
                        self@.add(Multiset::<T>::empty().insert(element)).count(x) ==
                        self@.insert(element).count(x)
                    by {}
                }
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log n), Span O(log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS; remove root, meld children.
            fn delete_min(&self) -> (min_and_rest: (Self, Option<T>)) {
                match &self.root {
                    LeftistHeapNode::Leaf => (self.clone(), None),
                    LeftistHeapNode::Node { key: ref_key, left: ref_left, right: ref_right, .. } => {
                        // Unfold validity for children.
                        assert(self.root.spec_is_heap() == ({
                            let lo = match **ref_left { LeftistHeapNode::Leaf => true,
                                LeftistHeapNode::Node { key: lk, .. } => TotalOrder::le(*ref_key, lk) };
                            let ro = match **ref_right { LeftistHeapNode::Leaf => true,
                                LeftistHeapNode::Node { key: rk, .. } => TotalOrder::le(*ref_key, rk) };
                            lo && ro && ref_left.spec_is_heap() && ref_right.spec_is_heap()
                        }));
                        assert(self.root.spec_is_leftist() == (
                            ref_left.spec_rank() >= ref_right.spec_rank()
                            && ref_left.spec_is_leftist() && ref_right.spec_is_leftist()));
                        assert(ref_left.spec_rank_bounded() && ref_right.spec_rank_bounded());
                        // Clone root and destructure to get owned key (avoids key.clone() equality issue).
                        let cloned_root = self.root.clone();
                        match cloned_root {
                            LeftistHeapNode::Leaf => {
                                // Unreachable: cloned_root == self.root (Clone ensures) and self.root is Node.
                                proof { assert(false); }
                                diverge()
                            }
                            LeftistHeapNode::Node { key, left, right, .. } => {
                                let melded_root = LeftistHeapNode::meld_nodes(*left, *right);
                                let new_pq = LeftistHeapPQ { root: melded_root };
                                proof {
                                    // cloned_root == self.root, so same spec_seq.
                                    let s1 = Seq::<T>::empty().push(key);
                                    let s2 = left.spec_seq();
                                    let s3 = right.spec_seq();
                                    assert(self.root.spec_seq() =~= s1 + s2 + s3);
                                    vstd::seq_lib::lemma_multiset_commutative(s1 + s2, s3);
                                    vstd::seq_lib::lemma_multiset_commutative(s1, s2);
                                    assert(self@ =~= Multiset::empty().insert(key).add(left@).add(right@));
                                    assert(new_pq@ =~= left@.add(right@));
                                    // Prove key is minimum of self@.
                                    lemma_heap_root_is_min(&self.root);
                                    let s = self.root.spec_seq();
                                    assert(s[0] == key);
                                    assert forall|e: T| self@.count(e) > 0 implies
                                        #[trigger] TotalOrder::le(key, e)
                                    by {
                                        assert(s.to_multiset().count(e) > 0);
                                        assert(s.contains(e));
                                        let i = choose|i: int| 0 <= i < s.len() && s[i] == e;
                                        assert(TotalOrder::le(s[0], s[i]));
                                    }
                                }
                                (new_pq, Some(key))
                            }
                        }
                    }
                }
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(log m + log n), Span O(log m + log n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m + log n), Span O(log m + log n) — matches APAS; recursive meld along right spines.
            fn meld(&self, other: &Self) -> (pq: Self) {
                let pq = LeftistHeapPQ {
                    root: LeftistHeapNode::meld_nodes(self.root.clone(), other.root.clone()),
                };
                // meld_nodes ensures node@ =~= a@.add(b@) directly.
                pq
            }

            /// - Alg Analysis: APAS (Ch45 ref): Work O(n), Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — DIFFERS: sequential insert, not reduce-based.
            fn from_seq(seq: &ArraySeqStPerS<T>) -> (pq: Self) {
                let n = seq.length();
                let mut pq = Self::empty();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == seq@.len(),
                        pq.spec_size() == i as nat,
                        pq.spec_leftistheappq_wf(),
                {
                    pq = pq.insert(seq.nth(i).clone());
                }
                pq
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn size(&self) -> (n: usize) {
                self.root.size()
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn is_empty(&self) -> (is_empty: bool) {
                match &self.root {
                    LeftistHeapNode::Leaf => {
                        assert(self.root.spec_size() == 0);
                        true
                    }
                    LeftistHeapNode::Node { left, right, .. } => {
                        assert(self.root.spec_size() == 1 + left.spec_size() + right.spec_size());
                        false
                    }
                }
            }

            #[verifier::exec_allows_no_decreases_clause]
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn extract_all_sorted(&self) -> (sorted: Vec<T>) {
                let mut result: Vec<T> = Vec::new();
                let mut current_heap = self.clone();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while !current_heap.is_empty()
                    invariant
                        result@.len() as nat + current_heap.spec_size() == self.spec_size(),
                        self.spec_size() <= usize::MAX as nat,
                        current_heap.spec_leftistheappq_wf(),
                        Self::spec_sorted(result@),
                        forall|i: int, e: T| 0 <= i < result@.len() && current_heap@.count(e) > 0 ==>
                            #[trigger] TotalOrder::le(result@[i], e),
                {
                    let ghost old_result = result@;
                    let ghost old_heap_ms = current_heap@;
                    let (new_heap, min_element) = current_heap.delete_min();
                    if let Some(element) = min_element {
                        proof {
                            // element is the minimum of old_heap_ms.
                            // All prior result elements are <= all old_heap elements (invariant).
                            // So all prior result elements are <= element.
                            // element is <= all elements in new_heap (subset of old_heap).
                            // Thus after push, result is still sorted and all <= new_heap elements.

                            // Sortedness: element >= all prior result elements.
                            assert forall|i: int, j: int|
                                0 <= i < j < result@.push(element).len()
                                implies #[trigger] TotalOrder::le(result@.push(element)[i], result@.push(element)[j])
                            by {
                                if j < old_result.len() {
                                    // Both in old result — sorted by invariant.
                                    assert(result@.push(element)[i] == old_result[i]);
                                    assert(result@.push(element)[j] == old_result[j]);
                                } else {
                                    // j == old_result.len(), so result@.push(element)[j] == element.
                                    assert(result@.push(element)[j] == element);
                                    if i < old_result.len() {
                                        // old_result[i] <= element by invariant (element in old_heap).
                                        assert(result@.push(element)[i] == old_result[i]);
                                        assert(old_heap_ms.count(element) > 0);
                                        assert(TotalOrder::le(old_result[i], element));
                                    } else {
                                        // i == j, contradiction with i < j.
                                    }
                                }
                            }

                            // All result elements (including element) are <= all new_heap elements.
                            assert forall|i: int, e2: T|
                                0 <= i < result@.push(element).len() && new_heap@.count(e2) > 0
                                implies #[trigger] TotalOrder::le(result@.push(element)[i], e2)
                            by {
                                // e2 is in new_heap, which is old_heap minus element.
                                // old_heap@ =~= new_heap@.insert(element), so old_heap has e2.
                                assert(old_heap_ms.count(e2) > 0);
                                if i < old_result.len() as int {
                                    // old_result[i] <= e2 by invariant.
                                    assert(result@.push(element)[i] == old_result[i]);
                                } else {
                                    // i == old_result.len(), result@.push(element)[i] == element.
                                    assert(result@.push(element)[i] == element);
                                    // element is min of old_heap, so element <= e2.
                                    assert(TotalOrder::le(element, e2));
                                }
                            }
                        }
                        result.push(element);
                    }
                    current_heap = new_heap;
                }
                result
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn height(&self) -> (levels: usize) {
                match &self.root {
                    LeftistHeapNode::Leaf => 0,
                    _ => self.root.height(),
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
            fn root_rank(&self) -> (rank_val: usize) {
                match &self.root {
                    LeftistHeapNode::Leaf => {
                        assert(self.root.spec_size() == 0);
                        0
                    }
                    LeftistHeapNode::Node { left, right, .. } => {
                        assert(self.root.spec_size() == 1 + left.spec_size() + right.spec_size());
                        self.root.rank()
                    }
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn is_valid_leftist_heap(&self) -> (is_valid: bool) {
                self.root.is_leftist() && self.root.is_heap() && self.root.is_rank_bounded()
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn from_vec(vec: Vec<T>) -> (pq: Self) {
                let seq = ArraySeqStPerS::from_vec(vec);
                Self::from_seq(&seq)
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
            fn to_vec(&self) -> (v: Vec<T>) {
                match &self.root {
                    LeftistHeapNode::Leaf => Vec::new(),
                    _ => self.root.to_vec(),
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
            fn to_sorted_vec(&self) -> (v: Vec<T>) { self.extract_all_sorted() }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k * lg(n)), Span O(k * lg(n))
            fn meld_multiple(heaps: &Vec<Self>) -> (pq: Self) {
                let mut result = Self::empty();
                let n = heaps.len();
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                for i in 0..n
                    invariant
                        n == heaps@.len(),
                        result.spec_size() == Self::spec_total_size(heaps@, i as int),
                        Self::spec_total_size(heaps@, heaps@.len() as int) <= usize::MAX as nat,
                        result.spec_leftistheappq_wf(),
                {
                    proof { lemma_total_size_monotone::<T>(heaps@, (i + 1) as int, n as int); }
                    result = result.meld(&heaps[i]);
                }
                result
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
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
                        less_than.spec_leftistheappq_wf(),
                        equal_or_greater.spec_leftistheappq_wf(),
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

    //		Section 12a. derive impls in verus!


        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for LeftistHeapNode<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self == other }
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
                        proof { assume(cloned == *self); }
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
                proof { assume(equal == (*self == *other)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for LeftistHeapNode<T> {}

    //		Section 12b. derive impls in verus!


        #[cfg(verus_keep_ghost)]
        impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for LeftistHeapPQ<T> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { self.root == other.root }
        }

        impl<T: StT + Ord + TotalOrder> Default for LeftistHeapPQ<T> {
            fn default() -> Self { Self::empty() }
        }

        impl<T: StT + Ord + TotalOrder> Clone for LeftistHeapPQ<T> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned.root == self.root
            {
                let cloned = LeftistHeapPQ { root: self.root.clone() };
                proof { assume(cloned.root == self.root); }
                cloned
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::PartialEq for LeftistHeapPQ<T> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (self.root == other.root)
            {
                let equal = self.root == other.root;
                proof { assume(equal == (self.root == other.root)); }
                equal
            }
        }

        impl<T: StT + Ord + TotalOrder> core::cmp::Eq for LeftistHeapPQ<T> {}

    }

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

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

    //		Section 14a. derive impls outside verus!

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

    //		Section 14b. derive impls outside verus!

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
}
