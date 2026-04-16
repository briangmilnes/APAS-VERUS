// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Ephemeral balanced binary tree utilities (Chapter 23). Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 9b. impls
//	Section 10b. iterators
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module


pub mod BalBinTreeStEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::vec::*,
        vstd::std_specs::cmp::PartialEqSpecImpl,
    };
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::accept::accept;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub enum BalBinTree<T> {
        Leaf,
        Node(Box<BalBinNode<T>>),
    }

    //		Section 8a. traits


    pub trait BalBinTreeTrait<T>: Sized {
        spec fn spec_balbintreesteph_wf(&self) -> bool;
        spec fn spec_size(self) -> nat;
        spec fn spec_height(self) -> nat;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;
        spec fn spec_post_order(self) -> Seq<T>;
        spec fn spec_is_leaf(self) -> bool;

        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn leaf() -> (l: Self)
            ensures l.spec_balbintreesteph_wf(),
                    l.spec_size() == 0,
                    l.spec_height() == 0,
                    l.spec_in_order() == Seq::<T>::empty(),
                    l.spec_pre_order() == Seq::<T>::empty(),
                    l.spec_post_order() == Seq::<T>::empty();

        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn node(left: Self, value: T, right: Self) -> (n: Self)
            ensures n.spec_balbintreesteph_wf(),
                    n.spec_size() == 1 + left.spec_size() + right.spec_size(),
                    n.spec_height() == 1 + if left.spec_height() >= right.spec_height()
                                            { left.spec_height() } else { right.spec_height() },
                    n.spec_in_order() == left.spec_in_order() + seq![value] + right.spec_in_order(),
                    n.spec_pre_order() == seq![value] + left.spec_pre_order() + right.spec_pre_order(),
                    n.spec_post_order() == left.spec_post_order() + right.spec_post_order() + seq![value];

        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_leaf(&self) -> (b: bool)
            requires self.spec_balbintreesteph_wf(),
            ensures b == (self.spec_size() == 0);

        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential recursive traversal, no stored size.
        fn size(&self) -> (count: usize)
            requires self.spec_balbintreesteph_wf(),
                     self.spec_size() <= usize::MAX,
            ensures count == self.spec_size();

        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential recursive traversal.
        fn height(&self) -> (h: usize)
            requires self.spec_balbintreesteph_wf(),
                     self.spec_height() <= usize::MAX,
            ensures h == self.spec_height();

        /// In-order traversal: left, root, right.
        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential recursive traversal with Vec building.
        fn in_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_balbintreesteph_wf(),
                     self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_in_order();

        /// Pre-order traversal: root, left, right.
        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential recursive traversal with Vec building.
        fn pre_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_balbintreesteph_wf(),
                     self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_pre_order();

        /// Post-order traversal: left, right, root.
        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential recursive traversal with Vec building.
        fn post_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_balbintreesteph_wf(),
                     self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_post_order();
    }

    //		Section 9a. impls


    // Inherent impl: recursive spec fn bodies with `decreases self`.
    // Verus cannot unfold `open spec fn` through trait dispatch when the
    // caller is an exec method in the same impl, so these must live here;
    // the trait impl delegates to them with one-liners.
    // Exception: bare_impl allowed per trait-impl-pattern (recursive-spec-fn).
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

        pub open spec fn spec_post_order(self) -> Seq<T>
            decreases self,
        {
            match self {
                BalBinTree::Leaf => Seq::empty(),
                BalBinTree::Node(node) =>
                    node.left.spec_post_order() + node.right.spec_post_order() + seq![node.value],
            }
        }
    }

    // Trait impl: delegates recursive specs to inherent methods.
    impl<T> BalBinTreeTrait<T> for BalBinTree<T> {
        open spec fn spec_balbintreesteph_wf(&self) -> bool {
            (self is Leaf) == (self.spec_size() == 0)
        }
        open spec fn spec_size(self) -> nat { BalBinTree::spec_size(self) }
        open spec fn spec_height(self) -> nat { BalBinTree::spec_height(self) }
        open spec fn spec_in_order(self) -> Seq<T> { BalBinTree::spec_in_order(self) }
        open spec fn spec_pre_order(self) -> Seq<T> { BalBinTree::spec_pre_order(self) }
        open spec fn spec_post_order(self) -> Seq<T> { BalBinTree::spec_post_order(self) }

        open spec fn spec_is_leaf(self) -> bool {
            self is Leaf
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant construction.
        fn leaf() -> (l: Self)
        {
            BalBinTree::Leaf
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Box allocation.
        fn node(left: Self, value: T, right: Self) -> (n: Self)
        {
            BalBinTree::Node(Box::new(BalBinNode { left, value, right }))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — pattern match.
        fn is_leaf(&self) -> (b: bool)
        {
            match self {
                BalBinTree::Leaf => true,
                BalBinTree::Node(_) => false,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive traversal of all n nodes; St sequential.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive traversal of all n nodes; St sequential.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive in-order traversal + append; St sequential.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive pre-order traversal + append; St sequential.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive post-order traversal + append; St sequential.
        fn post_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            decreases self.spec_size(),
        {
            match self {
                BalBinTree::Leaf => Vec::new(),
                BalBinTree::Node(node) => {
                    let mut left = node.left.post_order();
                    let mut right = node.right.post_order();
                    left.append(&mut right);
                    let val = node.value.clone_plus();
                    left.push(val);
                    left
                }
            }
        }
    }


    impl<T: Clone + Eq> BalBinTree<T> {
        /// Returns an in-order iterator.
        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n) — dominated by in_order traversal.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — calls in_order() then wraps in iterator.
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
        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n) — dominated by pre_order traversal.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — calls pre_order() then wraps in iterator.
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

        /// Returns a post-order iterator.
        /// - Alg Analysis: APAS (Ch23 DT 23.1): Work O(n), Span O(n) — dominated by post_order traversal.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — calls post_order() then wraps in iterator.
        pub fn iter_post_order(&self) -> (it: PostOrderIter<T>)
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures
                it@.0 == 0,
                it@.1 =~= self.spec_post_order(),
                post_order_iter_invariant(&it),
        {
            PostOrderIter { inner: self.post_order().into_iter() }
        }
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BalBinNode<T> {
        pub left: BalBinTree<T>,
        pub value: T,
        pub right: BalBinTree<T>,
    }

    //		Section 6b. spec fns


    pub open spec fn in_order_iter_invariant<T>(it: &InOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub open spec fn pre_order_iter_invariant<T>(it: &PreOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub open spec fn post_order_iter_invariant<T>(it: &PostOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    //		Section 7b. proof fns/broadcast groups


    // spec_size, spec_height, spec_in_order, spec_pre_order, spec_post_order
    // are defined in impl BalBinTree (section 9); trait impl delegates to them.


    /// The in-order and pre-order traversals of a tree are permutations of each other.
    /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — proof function, no runtime cost.
    pub proof fn lemma_in_order_pre_order_permutation<T>(tree: BalBinTree<T>)
        ensures tree.spec_in_order().to_multiset() =~= tree.spec_pre_order().to_multiset()
        decreases tree,
    {
        match tree {
            BalBinTree::Leaf => {},
            BalBinTree::Node(node) => {
                let l_in = node.left.spec_in_order();
                let r_in = node.right.spec_in_order();
                let l_pre = node.left.spec_pre_order();
                let r_pre = node.right.spec_pre_order();
                let v = seq![node.value];

                lemma_in_order_pre_order_permutation(node.left);
                lemma_in_order_pre_order_permutation(node.right);

                // in_order  = l_in + v + r_in  = (l_in + v) + r_in
                // pre_order = v + l_pre + r_pre = (v + l_pre) + r_pre

                // Seq concatenation is associative

                // Decompose each concatenation into multiset additions
                vstd::seq_lib::lemma_multiset_commutative(l_in + v, r_in);
                vstd::seq_lib::lemma_multiset_commutative(l_in, v);
                vstd::seq_lib::lemma_multiset_commutative(v + l_pre, r_pre);
                vstd::seq_lib::lemma_multiset_commutative(v, l_pre);

                // Now use commutativity of Multiset::add

                // By IH + commutativity of add, these are equal
            },
        }
    }

    /// The pre-order and post-order traversals of a tree are permutations of each other.
    /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — proof function, no runtime cost.
    pub proof fn lemma_pre_order_post_order_permutation<T>(tree: BalBinTree<T>)
        ensures tree.spec_pre_order().to_multiset() =~= tree.spec_post_order().to_multiset()
        decreases tree,
    {
        match tree {
            BalBinTree::Leaf => {},
            BalBinTree::Node(node) => {
                let l_pre = node.left.spec_pre_order();
                let r_pre = node.right.spec_pre_order();
                let l_post = node.left.spec_post_order();
                let r_post = node.right.spec_post_order();
                let v = seq![node.value];

                lemma_pre_order_post_order_permutation(node.left);
                lemma_pre_order_post_order_permutation(node.right);

                // pre_order  = v + l_pre + r_pre  = (v + l_pre) + r_pre
                // post_order = l_post + r_post + v = (l_post + r_post) + v


                vstd::seq_lib::lemma_multiset_commutative(v + l_pre, r_pre);
                vstd::seq_lib::lemma_multiset_commutative(v, l_pre);
                vstd::seq_lib::lemma_multiset_commutative(l_post + r_post, v);
                vstd::seq_lib::lemma_multiset_commutative(l_post, r_post);


                // By IH: l_pre.to_multiset() =~= l_post.to_multiset() (and same for right)
                // Multiset add is commutative and associative
            },
        }
    }

    //		Section 9b. impls


    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n), Span Theta(n) — recursive deep clone.
    // veracity: no_requires
    fn clone_tree<T: Clone>(t: &BalBinTree<T>) -> (c: BalBinTree<T>)
        ensures c == *t
        decreases t,
    {
        let c = match t {
            BalBinTree::Leaf => BalBinTree::Leaf,
            BalBinTree::Node(node) => BalBinTree::Node(Box::new(BalBinNode {
                left: clone_tree(&node.left),
                value: node.value.clone(),
                right: clone_tree(&node.right),
            })),
        };
        // Veracity: NEEDED proof block
        // Veracity: NEEDED proof block
        proof { accept(c == *t); }
        c
    }

    //		Section 10b. iterators


    /// Iterator over in-order traversal of a BalBinTree.
    #[verifier::reject_recursive_types(T)]
    pub struct InOrderIter<T> {
        pub inner: IntoIter<T>,
    }

    /// Iterator over pre-order traversal of a BalBinTree.
    #[verifier::reject_recursive_types(T)]
    pub struct PreOrderIter<T> {
        pub inner: IntoIter<T>,
    }

    /// Iterator over post-order traversal of a BalBinTree.
    #[verifier::reject_recursive_types(T)]
    pub struct PostOrderIter<T> {
        pub inner: IntoIter<T>,
    }

    impl<T> View for InOrderIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<T> View for PreOrderIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<T> View for PostOrderIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
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

    /// Ghost iterator for ForLoopGhostIterator support (post-order).
    #[verifier::reject_recursive_types(T)]
    pub struct PostOrderGhostIterator<T> {
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

    impl<T> View for PostOrderGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T> std::iter::Iterator for InOrderIter<T> {
        type Item = T;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(1), Span Theta(1) — delegates to Vec::IntoIter::next.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(1), Span Theta(1) — delegates to Vec::IntoIter::next.
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

    impl<T> std::iter::Iterator for PostOrderIter<T> {
        type Item = T;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(1), Span Theta(1) — delegates to Vec::IntoIter::next.
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

    impl<T> vstd::pervasive::ForLoopGhostIteratorNew for PostOrderIter<T> {
        type GhostIter = PostOrderGhostIterator<T>;
        open spec fn ghost_iter(&self) -> PostOrderGhostIterator<T> {
            PostOrderGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T> vstd::pervasive::ForLoopGhostIterator for PostOrderGhostIterator<T> {
        type ExecIter = PostOrderIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &PostOrderIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &PostOrderIter<T>) -> PostOrderGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    //		Section 12a. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEqSpecImpl for BalBinTree<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
    }

    impl<T: Eq> Eq for BalBinTree<T> {}

    impl<T: PartialEq> PartialEq for BalBinTree<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n), Span Theta(n) — recursive structural comparison.
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (*self == *other)
            decreases self,
        {
            match (self, other) {
                (BalBinTree::Leaf, BalBinTree::Leaf) => true,
                (BalBinTree::Node(a), BalBinTree::Node(b)) => {
                    let equal = a.left == b.left && a.value == b.value && a.right == b.right;
                    // Veracity: NEEDED proof block (speed hint)
                    // Veracity: NEEDED proof block
                    proof { accept(equal == (*self == *other)); }
                    equal
                },
                _ => false,
            }
        }
    }

    impl<T: Clone> Clone for BalBinTree<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n), Span Theta(n) — delegates to clone_tree.
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            clone_tree(self)
        }
    }

    //		Section 12b. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEqSpecImpl for BalBinNode<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
    }

    impl<T: Eq> Eq for BalBinNode<T> {}

    impl<T: PartialEq> PartialEq for BalBinNode<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n), Span Theta(n) — compares subtrees recursively.
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (*self == *other)
        {
            // Veracity: NEEDED proof block
            let equal = self.left == other.left && self.value == other.value && self.right == other.right;
            // Veracity: NEEDED proof block
            proof { accept(equal == (*self == *other)); }
            equal
        }
    }

    impl<T: Clone> Clone for BalBinNode<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n), Span Theta(n) — clones left/right subtrees recursively.
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = BalBinNode {
                left: clone_tree(&self.left),
                value: self.value.clone(),
                // Veracity: NEEDED proof block
                right: clone_tree(&self.right),
            };
            // Veracity: NEEDED proof block
            proof { accept(cloned == *self); }
            cloned
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<T: Debug> Debug for InOrderIter<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InOrderIter({:?})", self.inner)
        }
    }

    impl<T> Display for InOrderIter<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InOrderIter")
        }
    }

    impl<T: Debug> Debug for PreOrderIter<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PreOrderIter({:?})", self.inner)
        }
    }

    impl<T> Display for PreOrderIter<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PreOrderIter")
        }
    }

    impl<T: Debug> Debug for PostOrderIter<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PostOrderIter({:?})", self.inner)
        }
    }

    impl<T> Display for PostOrderIter<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PostOrderIter")
        }
    }

    impl<T> Debug for InOrderGhostIterator<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InOrderGhostIterator")
        }
    }

    impl<T> Display for InOrderGhostIterator<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InOrderGhostIterator")
        }
    }

    impl<T> Debug for PreOrderGhostIterator<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PreOrderGhostIterator")
        }
    }

    impl<T> Display for PreOrderGhostIterator<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PreOrderGhostIterator")
        }
    }

    impl<T> Debug for PostOrderGhostIterator<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PostOrderGhostIterator")
        }
    }

    impl<T> Display for PostOrderGhostIterator<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "PostOrderGhostIterator")
        }
    }

    //		Section 14a. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for BalBinTree<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BalBinTree::Leaf => write!(f, "Leaf"),
                BalBinTree::Node(node) =>
                    write!(f, "Node({:?}, {:?}, {:?})", node.left, node.value, node.right),
            }
        }
    }

    impl<T: Display> Display for BalBinTree<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                BalBinTree::Leaf => write!(f, "Leaf"),
                BalBinTree::Node(node) =>
                    write!(f, "({}, {}, {})", node.left, node.value, node.right),
            }
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for BalBinNode<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BalBinNode {{ left: {:?}, value: {:?}, right: {:?} }}", self.left, self.value, self.right)
        }
    }

    impl<T: Display> Display for BalBinNode<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.left, self.value, self.right)
        }
    }
} // mod
