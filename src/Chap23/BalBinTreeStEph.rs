//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral balanced binary tree utilities (Chapter 23). Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. spec functions
//	8. traits
//	9. impls
//	10. iterators
//	13. derive impls outside verus!

//		1. module

pub mod BalBinTreeStEph {

    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::vec::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub enum BalBinTree<T> {
        Leaf,
        Node(Box<BalBinNode<T>>),
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BalBinNode<T> {
        pub left: BalBinTree<T>,
        pub value: T,
        pub right: BalBinTree<T>,
    }

    //		5. spec functions

    impl<T> BalBinTree<T> {
        pub open spec fn spec_size(self) -> nat
            decreases self,
        {
            match self {
                BalBinTree::Leaf => 0,
                BalBinTree::Node(node) => 1 + node.left.spec_size() + node.right.spec_size(),
            }
        }

        pub open spec fn spec_height(self) -> nat
            decreases self,
        {
            match self {
                BalBinTree::Leaf => 0,
                BalBinTree::Node(node) => {
                    let lh = node.left.spec_height();
                    let rh = node.right.spec_height();
                    1 + if lh >= rh { lh } else { rh }
                }
            }
        }

        pub open spec fn spec_in_order(self) -> Seq<T>
            decreases self,
        {
            match self {
                BalBinTree::Leaf => Seq::empty(),
                BalBinTree::Node(node) =>
                    node.left.spec_in_order() + seq![node.value] + node.right.spec_in_order(),
            }
        }

        pub open spec fn spec_pre_order(self) -> Seq<T>
            decreases self,
        {
            match self {
                BalBinTree::Leaf => Seq::empty(),
                BalBinTree::Node(node) =>
                    seq![node.value] + node.left.spec_pre_order() + node.right.spec_pre_order(),
            }
        }

        pub open spec fn spec_is_leaf(self) -> bool {
            self is Leaf
        }
    }

    //		8. traits

    pub trait BalBinTreeTrait<T>: Sized {
        spec fn spec_size(self) -> nat;
        spec fn spec_height(self) -> nat;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;

        /// APAS: Work Θ(1), Span Θ(1).
        fn leaf() -> (l: Self)
            ensures l.spec_size() == 0,
                    l.spec_height() == 0,
                    l.spec_in_order() == Seq::<T>::empty(),
                    l.spec_pre_order() == Seq::<T>::empty();

        /// APAS: Work Θ(1), Span Θ(1).
        fn node(left: Self, value: T, right: Self) -> (n: Self)
            ensures n.spec_size() == 1 + left.spec_size() + right.spec_size(),
                    n.spec_height() == 1 + if left.spec_height() >= right.spec_height()
                                            { left.spec_height() } else { right.spec_height() },
                    n.spec_in_order() == left.spec_in_order() + seq![value] + right.spec_in_order(),
                    n.spec_pre_order() == seq![value] + left.spec_pre_order() + right.spec_pre_order();

        /// APAS: Work Θ(1), Span Θ(1).
        fn is_leaf(&self) -> (b: bool)
            ensures b == (self.spec_size() == 0);

        /// APAS: Work Θ(n), Span Θ(n).
        fn size(&self) -> (count: usize)
            requires self.spec_size() <= usize::MAX,
            ensures count == self.spec_size();

        /// APAS: Work Θ(n), Span Θ(n).
        fn height(&self) -> (h: usize)
            requires self.spec_height() <= usize::MAX,
            ensures h == self.spec_height();

        /// In-order traversal: left, root, right.
        /// APAS: Work Θ(n), Span Θ(n).
        fn in_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_in_order();

        /// Pre-order traversal: root, left, right.
        /// APAS: Work Θ(n), Span Θ(n).
        fn pre_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_pre_order();
    }

    //		9. impls

    impl<T> BalBinTreeTrait<T> for BalBinTree<T> {
        open spec fn spec_size(self) -> nat {
            BalBinTree::spec_size(self)
        }
        open spec fn spec_height(self) -> nat {
            BalBinTree::spec_height(self)
        }
        open spec fn spec_in_order(self) -> Seq<T> {
            BalBinTree::spec_in_order(self)
        }
        open spec fn spec_pre_order(self) -> Seq<T> {
            BalBinTree::spec_pre_order(self)
        }

        fn leaf() -> (l: Self)
        {
            BalBinTree::Leaf
        }

        fn node(left: Self, value: T, right: Self) -> (n: Self)
        {
            BalBinTree::Node(Box::new(BalBinNode { left, value, right }))
        }

        fn is_leaf(&self) -> (b: bool)
        {
            match self {
                BalBinTree::Leaf => true,
                BalBinTree::Node(_) => false,
            }
        }

        fn size(&self) -> (count: usize)
            decreases self.spec_size(),
        {
            match self {
                BalBinTree::Leaf => 0,
                BalBinTree::Node(node) => {
                    let left_sz = node.left.size();
                    let right_sz = node.right.size();
                    1 + left_sz + right_sz
                }
            }
        }

        fn height(&self) -> (h: usize)
            decreases self.spec_height(),
        {
            match self {
                BalBinTree::Leaf => 0,
                BalBinTree::Node(node) => {
                    let left_h = node.left.height();
                    let right_h = node.right.height();
                    1 + if left_h >= right_h { left_h } else { right_h }
                }
            }
        }

        fn in_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            decreases self.spec_size(),
        {
            match self {
                BalBinTree::Leaf => Vec::new(),
                BalBinTree::Node(node) => {
                    let mut left = node.left.in_order();
                    let val = node.value.clone_plus();
                    left.push(val);
                    let mut right = node.right.in_order();
                    left.append(&mut right);
                    left
                }
            }
        }

        fn pre_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            decreases self.spec_size(),
        {
            match self {
                BalBinTree::Leaf => Vec::new(),
                BalBinTree::Node(node) => {
                    let mut out = Vec::new();
                    let val = node.value.clone_plus();
                    out.push(val);
                    let mut left = node.left.pre_order();
                    let mut right = node.right.pre_order();
                    out.append(&mut left);
                    out.append(&mut right);
                    out
                }
            }
        }
    }

    //		10. iterators

    /// Iterator over in-order traversal of a BalBinTree.
    #[verifier::reject_recursive_types(T)]
    pub struct InOrderIter<T> {
        inner: IntoIter<T>,
    }

    /// Iterator over pre-order traversal of a BalBinTree.
    #[verifier::reject_recursive_types(T)]
    pub struct PreOrderIter<T> {
        inner: IntoIter<T>,
    }

    impl<T> View for InOrderIter<T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<T> View for PreOrderIter<T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support (in-order).
    #[verifier::reject_recursive_types(T)]
    pub struct InOrderGhostIterator<T> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    /// Ghost iterator for ForLoopGhostIterator support (pre-order).
    #[verifier::reject_recursive_types(T)]
    pub struct PreOrderGhostIterator<T> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T> View for InOrderGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T> View for PreOrderGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    pub open spec fn in_order_iter_invariant<T>(it: &InOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub open spec fn pre_order_iter_invariant<T>(it: &PreOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T> std::iter::Iterator for InOrderIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    impl<T> std::iter::Iterator for PreOrderIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    impl<T> vstd::pervasive::ForLoopGhostIteratorNew for InOrderIter<T> {
        type GhostIter = InOrderGhostIterator<T>;
        open spec fn ghost_iter(&self) -> InOrderGhostIterator<T> {
            InOrderGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T> vstd::pervasive::ForLoopGhostIterator for InOrderGhostIterator<T> {
        type ExecIter = InOrderIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &InOrderIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &InOrderIter<T>) -> InOrderGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<T> vstd::pervasive::ForLoopGhostIteratorNew for PreOrderIter<T> {
        type GhostIter = PreOrderGhostIterator<T>;
        open spec fn ghost_iter(&self) -> PreOrderGhostIterator<T> {
            PreOrderGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T> vstd::pervasive::ForLoopGhostIterator for PreOrderGhostIterator<T> {
        type ExecIter = PreOrderIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &PreOrderIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &PreOrderIter<T>) -> PreOrderGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<T: Clone + Eq> BalBinTree<T> {
        /// Returns an in-order iterator.
        pub fn iter_in_order(&self) -> (it: InOrderIter<T>)
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures
                it@.0 == 0,
                it@.1 =~= self.spec_in_order(),
                in_order_iter_invariant(&it),
        {
            InOrderIter { inner: self.in_order().into_iter() }
        }

        /// Returns a pre-order iterator.
        pub fn iter_pre_order(&self) -> (it: PreOrderIter<T>)
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures
                it@.0 == 0,
                it@.1 =~= self.spec_pre_order(),
                pre_order_iter_invariant(&it),
        {
            PreOrderIter { inner: self.pre_order().into_iter() }
        }
    }

    } // verus!

    //		13. derive impls outside verus!

    impl<T: Clone> Clone for BalBinTree<T> {
        fn clone(&self) -> Self {
            match self {
                BalBinTree::Leaf => BalBinTree::Leaf,
                BalBinTree::Node(node) => BalBinTree::Node(node.clone()),
            }
        }
    }

    impl<T: Clone> Clone for BalBinNode<T> {
        fn clone(&self) -> Self {
            BalBinNode { left: self.left.clone(), value: self.value.clone(), right: self.right.clone() }
        }
    }

    impl<T: PartialEq> PartialEq for BalBinTree<T> {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (BalBinTree::Leaf, BalBinTree::Leaf) => true,
                (BalBinTree::Node(a), BalBinTree::Node(b)) =>
                    a.left == b.left && a.value == b.value && a.right == b.right,
                _ => false,
            }
        }
    }

    impl<T: Eq> Eq for BalBinTree<T> {}

    impl<T: PartialEq> PartialEq for BalBinNode<T> {
        fn eq(&self, other: &Self) -> bool {
            self.left == other.left && self.value == other.value && self.right == other.right
        }
    }

    impl<T: Eq> Eq for BalBinNode<T> {}

    impl<T: std::fmt::Debug> std::fmt::Debug for BalBinTree<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BalBinTree::Leaf => write!(f, "Leaf"),
                BalBinTree::Node(node) =>
                    write!(f, "Node({:?}, {:?}, {:?})", node.left, node.value, node.right),
            }
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for BalBinNode<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BalBinNode {{ left: {:?}, value: {:?}, right: {:?} }}", self.left, self.value, self.right)
        }
    }

} // mod
