//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral balanced binary tree utilities (Chapter 23). Verusified.

//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4. type definitions
//  6. spec fns
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  10. iterators
//  11. derive impls in verus!
//  13. derive impls outside verus!

//		1. module




pub mod BalBinTreeStEph {

    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::vec::*,
        vstd::std_specs::cmp::PartialEqSpecImpl,
    };
    use crate::vstdplus::accept::accept;
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


    //		6. spec fns

    pub open spec fn in_order_iter_invariant<T>(it: &InOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub open spec fn pre_order_iter_invariant<T>(it: &PreOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub open spec fn post_order_iter_invariant<T>(it: &PostOrderIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }


    // spec_size, spec_height, spec_in_order, spec_pre_order, spec_post_order
    // are defined in impl BalBinTreeTrait for BalBinTree below.

    //		7. proof fns/broadcast groups

    /// The in-order and pre-order traversals of a tree are permutations of each other.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
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
                assert(tree.spec_in_order() =~= (l_in + v) + r_in);
                assert(tree.spec_pre_order() =~= (v + l_pre) + r_pre);

                // Decompose each concatenation into multiset additions
                vstd::seq_lib::lemma_multiset_commutative(l_in + v, r_in);
                vstd::seq_lib::lemma_multiset_commutative(l_in, v);
                vstd::seq_lib::lemma_multiset_commutative(v + l_pre, r_pre);
                vstd::seq_lib::lemma_multiset_commutative(v, l_pre);

                // Now use commutativity of Multiset::add
                assert(tree.spec_in_order().to_multiset()
                    =~= l_in.to_multiset().add(v.to_multiset()).add(r_in.to_multiset()));
                assert(tree.spec_pre_order().to_multiset()
                    =~= v.to_multiset().add(l_pre.to_multiset()).add(r_pre.to_multiset()));

                // By IH + commutativity of add, these are equal
                assert(l_in.to_multiset().add(v.to_multiset()).add(r_in.to_multiset())
                    =~= v.to_multiset().add(l_in.to_multiset()).add(r_in.to_multiset()));
            },
        }
    }

    /// The pre-order and post-order traversals of a tree are permutations of each other.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
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

                assert(tree.spec_pre_order() =~= (v + l_pre) + r_pre);
                assert(tree.spec_post_order() =~= (l_post + r_post) + v);

                vstd::seq_lib::lemma_multiset_commutative(v + l_pre, r_pre);
                vstd::seq_lib::lemma_multiset_commutative(v, l_pre);
                vstd::seq_lib::lemma_multiset_commutative(l_post + r_post, v);
                vstd::seq_lib::lemma_multiset_commutative(l_post, r_post);

                assert(tree.spec_pre_order().to_multiset()
                    =~= v.to_multiset().add(l_pre.to_multiset()).add(r_pre.to_multiset()));
                assert(tree.spec_post_order().to_multiset()
                    =~= l_post.to_multiset().add(r_post.to_multiset()).add(v.to_multiset()));

                // By IH: l_pre.to_multiset() =~= l_post.to_multiset() (and same for right)
                // Multiset add is commutative and associative
                assert(l_post.to_multiset().add(r_post.to_multiset()).add(v.to_multiset())
                    =~= v.to_multiset().add(l_post.to_multiset()).add(r_post.to_multiset()));
            },
        }
    }

    //		8. traits

    pub trait BalBinTreeTrait<T>: Sized {
        spec fn spec_size(self) -> nat;
        spec fn spec_height(self) -> nat;
        spec fn spec_in_order(self) -> Seq<T>;
        spec fn spec_pre_order(self) -> Seq<T>;
        spec fn spec_post_order(self) -> Seq<T>;
        spec fn spec_is_leaf(self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn leaf() -> (l: Self)
            ensures l.spec_size() == 0,
                    l.spec_height() == 0,
                    l.spec_in_order() == Seq::<T>::empty(),
                    l.spec_pre_order() == Seq::<T>::empty(),
                    l.spec_post_order() == Seq::<T>::empty();

        /// - APAS: Work Θ(1), Span Θ(1).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn node(left: Self, value: T, right: Self) -> (n: Self)
            ensures n.spec_size() == 1 + left.spec_size() + right.spec_size(),
                    n.spec_height() == 1 + if left.spec_height() >= right.spec_height()
                                            { left.spec_height() } else { right.spec_height() },
                    n.spec_in_order() == left.spec_in_order() + seq![value] + right.spec_in_order(),
                    n.spec_pre_order() == seq![value] + left.spec_pre_order() + right.spec_pre_order(),
                    n.spec_post_order() == left.spec_post_order() + right.spec_post_order() + seq![value];

        /// - APAS: Work Θ(1), Span Θ(1).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn is_leaf(&self) -> (b: bool)
            ensures b == (self.spec_size() == 0);

        /// - APAS: Work Θ(n), Span Θ(n).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential recursive traversal, no stored size.
        fn size(&self) -> (count: usize)
            requires self.spec_size() <= usize::MAX,
            ensures count == self.spec_size();

        /// - APAS: Work Θ(n), Span Θ(n).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential recursive traversal.
        fn height(&self) -> (h: usize)
            requires self.spec_height() <= usize::MAX,
            ensures h == self.spec_height();

        /// In-order traversal: left, root, right.
        /// - APAS: Work Θ(n), Span Θ(n).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential recursive traversal with Vec building.
        fn in_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_in_order();

        /// Pre-order traversal: root, left, right.
        /// - APAS: Work Θ(n), Span Θ(n).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential recursive traversal with Vec building.
        fn pre_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_pre_order();

        /// Post-order traversal: left, right, root.
        /// - APAS: Work Θ(n), Span Θ(n).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential recursive traversal with Vec building.
        fn post_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires self.spec_size() <= usize::MAX,
                     obeys_feq_clone::<T>(),
            ensures traversal@ =~= self.spec_post_order();
    }


    //		9. impls

    // Inherent impl: recursive spec fn bodies with `decreases self`.
    // Verus cannot unfold `open spec fn` through trait dispatch, so these
    // must live here; the trait impl delegates to them with one-liners.
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
        open spec fn spec_size(self) -> nat { BalBinTree::spec_size(self) }
        open spec fn spec_height(self) -> nat { BalBinTree::spec_height(self) }
        open spec fn spec_in_order(self) -> Seq<T> { BalBinTree::spec_in_order(self) }
        open spec fn spec_pre_order(self) -> Seq<T> { BalBinTree::spec_pre_order(self) }
        open spec fn spec_post_order(self) -> Seq<T> { BalBinTree::spec_post_order(self) }

        open spec fn spec_is_leaf(self) -> bool {
            self is Leaf
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

    //  10. iterators

    impl<T: Clone + Eq> BalBinTree<T> {
        /// Returns an in-order iterator.
        /// - APAS: Work Θ(n), Span Θ(n) — dominated by in_order traversal.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — calls in_order() then wraps in iterator.
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
        /// - APAS: Work Θ(n), Span Θ(n) — dominated by pre_order traversal.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — calls pre_order() then wraps in iterator.
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
        /// - APAS: Work Θ(n), Span Θ(n) — dominated by post_order traversal.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — calls post_order() then wraps in iterator.
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

    /// Iterator over post-order traversal of a BalBinTree.
    #[verifier::reject_recursive_types(T)]
    pub struct PostOrderIter<T> {
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

    impl<T> View for PostOrderIter<T> {
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

    impl<T> std::iter::Iterator for PostOrderIter<T> {
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

    //  11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEqSpecImpl for BalBinTree<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEqSpecImpl for BalBinNode<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
    }

    impl<T: Eq> Eq for BalBinTree<T> {}

    impl<T: PartialEq> PartialEq for BalBinTree<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (*self == *other)
            decreases self,
        {
            match (self, other) {
                (BalBinTree::Leaf, BalBinTree::Leaf) => true,
                (BalBinTree::Node(a), BalBinTree::Node(b)) => {
                    let equal = a.left == b.left && a.value == b.value && a.right == b.right;
                    proof { accept(equal == (*self == *other)); }
                    equal
                },
                _ => false,
            }
        }
    }

    impl<T: Eq> Eq for BalBinNode<T> {}

    impl<T: PartialEq> PartialEq for BalBinNode<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (*self == *other)
        {
            let equal = self.left == other.left && self.value == other.value && self.right == other.right;
            proof { accept(equal == (*self == *other)); }
            equal
        }
    }


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
        proof { accept(c == *t); }
        c
    }

    impl<T: Clone> Clone for BalBinTree<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            clone_tree(self)
        }
    }

    impl<T: Clone> Clone for BalBinNode<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = BalBinNode {
                left: clone_tree(&self.left),
                value: self.value.clone(),
                right: clone_tree(&self.right),
            };
            proof { accept(cloned == *self); }
            cloned
        }
    }

    } // verus!


    //		13. derive impls outside verus!

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
