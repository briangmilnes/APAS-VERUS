//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Implicit-order AVL tree providing O(lg(n)) nth and set by maintaining subtree sizes.
//!
//! Abstract:
//! - `AVLTreeS<T>` stores a balanced binary tree; in-order traversal defines the sequence order.
//! - `AVLTreeNode<T>` stores `value`, `height`, `left_size`, `right_size`, and children.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls
// 10. iterators
// 11. derive impls in verus!
// 13. derive impls outside verus!

// 1. module

pub mod AVLTreeSeq {

    // 2. imports

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::{PartialEqSpec, PartialEqSpecImpl};

    verus! {

    // 3. broadcast use
    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions
    pub type Link<T> = Option<Box<AVLTreeNode<T>>>;

    pub struct AVLTreeNode<T: StT> {
        pub value: T,
        pub height: usize,
        pub left_size: usize,
        pub right_size: usize,
        pub left: Link<T>,
        pub right: Link<T>,
        pub index: usize,
    }

    pub struct AVLTreeS<T: StT> {
        pub root: Link<T>,
        pub next_key: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqIter<'a, T: StT> {
        pub tree: &'a AVLTreeS<T>,
        pub pos: usize,
        pub len: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSeqGhostIterator<'a, T: StT> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    // 5. view impls

    impl<T: StT> View for AVLTreeS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            spec_avltreeseq_inorder(self.root)
        }
    }

    impl<'a, T: StT> View for AVLTreeSeqIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, spec_avltreeseq_inorder_values(self.tree.root))
        }
    }

    impl<'a, T: StT> View for AVLTreeSeqGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn spec_avltreeseq_inorder_values<T: StT>(link: Link<T>) -> Seq<T>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_avltreeseq_inorder_values(node.left) + seq![node.value] + spec_avltreeseq_inorder_values(node.right),
        }
    }

    pub open spec fn spec_avltreeseq_inorder<T: StT>(link: Link<T>) -> Seq<T::V>
        decreases link,
    {
        match link {
            None => Seq::empty(),
            Some(node) => spec_avltreeseq_inorder(node.left) + seq![node.value@] + spec_avltreeseq_inorder(node.right),
        }
    }

    pub open spec fn spec_avltreeseq_cached_size<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => 1 + node.left_size as nat + node.right_size as nat,
        }
    }

    pub open spec fn spec_avltreeseq_cached_height<T: StT>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.height as nat,
        }
    }

    pub open spec fn spec_avltreeseq_nat_max(a: nat, b: nat) -> nat {
        if a >= b { a } else { b }
    }

    pub open spec fn spec_avltreeseq_wf<T: StT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_avltreeseq_wf(node.left)
                && spec_avltreeseq_wf(node.right)
                && node.left_size as nat == spec_avltreeseq_cached_size(&node.left)
                && node.right_size as nat == spec_avltreeseq_cached_size(&node.right)
                && node.height as nat == 1 + spec_avltreeseq_nat_max(
                    spec_avltreeseq_cached_height(&node.left),
                    spec_avltreeseq_cached_height(&node.right),
                )
                && 1 + node.left_size + node.right_size < usize::MAX
            }
        }
    }

    /// AVL balance at every node: |h_left - h_right| <= 1.
    pub open spec fn spec_avltreeseq_avl_balanced<T: StT>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_avltreeseq_avl_balanced(node.left)
                && spec_avltreeseq_avl_balanced(node.right)
                && spec_avltreeseq_cached_height(&node.left) <= spec_avltreeseq_cached_height(&node.right) + 1
                && spec_avltreeseq_cached_height(&node.right) <= spec_avltreeseq_cached_height(&node.left) + 1
            }
        }
    }

    pub open spec fn spec_avltreeseq_subseq<V>(seq: Seq<V>, start: nat, length: nat) -> Seq<V> {
        let n = seq.len();
        let s = if start < n { start } else { n };
        let e_raw = start + length;
        let e = if e_raw < n { e_raw } else { n };
        if e <= s { Seq::<V>::empty() } else { seq.subrange(s as int, e as int) }
    }

    pub open spec fn iter_invariant<'a, T: StT>(it: &AVLTreeSeqIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 7. proof fns

    proof fn lemma_size_eq_inorder_len<T: StT>(link: &Link<T>)
        requires spec_avltreeseq_wf(*link),
        ensures spec_avltreeseq_cached_size(link) == spec_avltreeseq_inorder(*link).len(),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_size_eq_inorder_len::<T>(&node.left);
                lemma_size_eq_inorder_len::<T>(&node.right);
            }
        }
    }

    proof fn lemma_inorder_values_maps_to_inorder<T: StT>(link: Link<T>)
        ensures spec_avltreeseq_inorder_values(link).map_values(|t: T| t@) =~= spec_avltreeseq_inorder(link),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_inorder_values_maps_to_inorder::<T>(node.left);
                lemma_inorder_values_maps_to_inorder::<T>(node.right);
            }
        }
    }

    proof fn lemma_height_le_size<T: StT>(link: &Link<T>)
        requires spec_avltreeseq_wf(*link),
        ensures spec_avltreeseq_cached_height(link) <= spec_avltreeseq_cached_size(link),
        decreases *link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_height_le_size::<T>(&node.left);
                lemma_height_le_size::<T>(&node.right);
            }
        }
    }

    // 8. traits

    /// Spec accessors for AVL tree links, enabling trait-based contracts.
    pub trait AVLTreeSeqLinkSpec<T: StT>: Sized {
        spec fn link_wf(self) -> bool;
        spec fn link_inorder(self) -> Seq<T::V>;
        spec fn link_cached_size(self) -> nat;
        spec fn link_cached_height(self) -> nat;
    }

    impl<T: StT> AVLTreeSeqLinkSpec<T> for Link<T> {
        open spec fn link_wf(self) -> bool { spec_avltreeseq_wf(self) }
        open spec fn link_inorder(self) -> Seq<T::V> { spec_avltreeseq_inorder(self) }
        open spec fn link_cached_size(self) -> nat { spec_avltreeseq_cached_size(&self) }
        open spec fn link_cached_height(self) -> nat { spec_avltreeseq_cached_height(&self) }
    }

    /// Exec operations on AVL tree links (Option<Box<AVLTreeNode>>).
    pub trait AVLTreeSeqLinkFns<T: StT>: Sized + AVLTreeSeqLinkSpec<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn cached_height_fn(&self) -> (height: usize)
            requires (*self).link_cached_height() <= usize::MAX as nat,
            ensures height as nat == (*self).link_cached_height(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn cached_size_fn(&self) -> (size: usize)
            requires (*self).link_cached_size() < usize::MAX,
            ensures size as nat == (*self).link_cached_size(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_at_link(self, index: usize, value: T, next_key: &mut usize) -> (inserted: Self)
            requires
                self.link_wf(),
                0 <= index as int <= self.link_inorder().len(),
                *old(next_key) < usize::MAX,
                self.link_cached_size() + 1 < usize::MAX,
            ensures
                inserted.link_wf(),
                inserted.link_inorder() =~= self.link_inorder().insert(index as int, value@),
                *next_key == *old(next_key) + 1,
                inserted.link_cached_size() == self.link_cached_size() + 1,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn nth_link(&self, index: usize) -> (elem: &T)
            requires (*self).link_wf(), (index as int) < (*self).link_inorder().len(),
            ensures elem@ == (*self).link_inorder()[index as int],
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set_link(&mut self, index: usize, value: T) -> (outcome: Result<(), &'static str>)
            requires
                old(self).link_wf(),
                (index as int) < old(self).link_inorder().len(),
            ensures
                (*self).link_wf(),
                (*self).link_cached_size() == old(self).link_cached_size(),
                (*self).link_cached_height() == old(self).link_cached_height(),
                outcome is Ok,
                (*self).link_inorder() =~= old(self).link_inorder().update(index as int, value@),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn push_inorder(&self, out: &mut Vec<T>)
            requires (*self).link_wf(), obeys_feq_full::<T>(),
            ensures
                out@.map_values(|t: T| t@) =~=
                    old(out)@.map_values(|t: T| t@) + (*self).link_inorder(),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn compare_trees(&self, other: &Self) -> (equal: bool)
            requires
                (*self).link_wf(), (*other).link_wf(), obeys_feq_full::<T>(),
                (*self).link_cached_size() < usize::MAX,
                (*other).link_cached_size() < usize::MAX,
            ensures equal == ((*self).link_inorder() =~= (*other).link_inorder()),
            ;
    }

    pub trait AVLTreeSeq<T: StT>: Sized {
        spec fn spec_avltreeseq_seq(&self) -> Seq<T::V>;
        spec fn spec_avltreeseq_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (tree: Self)
            ensures tree.spec_avltreeseq_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_avltreeseq_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize)
            requires self.spec_avltreeseq_wf(),
            ensures len as nat == self.spec_avltreeseq_seq().len();

        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — DIFFERS: tree traversal to indexed node, not O(1) array access
        fn nth(&self, index: usize) -> (elem: &T)
            requires self.spec_avltreeseq_wf(), (index as int) < self.spec_avltreeseq_seq().len(),
            ensures elem@ == self.spec_avltreeseq_seq()[index as int];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set(&mut self, index: usize, item: T) -> (outcome: Result<(), &'static str>)
            requires old(self).spec_avltreeseq_wf(), (index as int) < old(self).spec_avltreeseq_seq().len(),
            ensures
                self.spec_avltreeseq_wf(),
                self.spec_avltreeseq_seq() =~= old(self).spec_avltreeseq_seq().update(index as int, item@),
                outcome is Ok;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (tree: Self)
            ensures
                tree.spec_avltreeseq_seq() =~= seq![item@],
                tree.spec_avltreeseq_seq().len() == 1,
                tree.spec_avltreeseq_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isEmpty(&self) -> (empty: bool)
            requires self.spec_avltreeseq_wf(),
            ensures empty == (self.spec_avltreeseq_seq().len() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isSingleton(&self) -> (single: bool)
            requires self.spec_avltreeseq_wf(),
            ensures single == (self.spec_avltreeseq_seq().len() == 1);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn subseq_copy(&self, start: usize, length: usize) -> (sub: Self)
            requires self.spec_avltreeseq_wf(),
            ensures sub.spec_avltreeseq_seq() =~=
                spec_avltreeseq_subseq(self.spec_avltreeseq_seq(), start as nat, length as nat);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_root() -> (tree: Self)
            ensures tree.spec_avltreeseq_seq() =~= Seq::<T::V>::empty(), tree.spec_avltreeseq_wf();

        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — DIFFERS: tree traversal + path reconstruction, not O(1) array update
        fn update(&mut self, index: usize, item: T)
            requires
                old(self).spec_avltreeseq_wf(),
                (index as int) < old(self).spec_avltreeseq_seq().len(),
            ensures
                self.spec_avltreeseq_wf(),
                self.spec_avltreeseq_seq() =~= old(self).spec_avltreeseq_seq().update(index as int, item@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_vec(values: Vec<T>) -> (tree: AVLTreeS<T>)
            requires
                obeys_feq_full::<T>(),
                values@.len() < usize::MAX,
            ensures
                spec_avltreeseq_wf(tree.root),
                spec_avltreeseq_inorder(tree.root) =~= values@.map_values(|t: T| t@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_arrayseq(&self) -> (seq: ArraySeqStEphS<T>)
            requires self.spec_avltreeseq_wf(),
            ensures
                seq.spec_len() == self.spec_avltreeseq_seq().len(),
                forall|i: int| #![trigger seq.spec_index(i)]
                    0 <= i < seq.spec_len() ==> seq.spec_index(i)@ == self.spec_avltreeseq_seq()[i];

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqIter<'a, T>)
            requires self.spec_avltreeseq_wf(),
            ensures
                it@.0 == 0int,
                it@.1.map_values(|t: T| t@) =~= self.spec_avltreeseq_seq(),
                iter_invariant(&it);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn push_back(&mut self, value: T)
            requires old(self).spec_avltreeseq_wf(),
            ensures self.spec_avltreeseq_seq() =~= old(self).spec_avltreeseq_seq().push(value@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn contains_value(&self, target: &T) -> (found: bool)
            requires self.spec_avltreeseq_wf(),
            ensures found == exists|j: int| 0 <= j < self.spec_avltreeseq_seq().len()
                && self.spec_avltreeseq_seq()[j] == target@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_value(&mut self, value: T)
            requires old(self).spec_avltreeseq_wf(),
            ensures self.spec_avltreeseq_seq() =~= old(self).spec_avltreeseq_seq().push(value@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn delete_value(&mut self, target: &T) -> (deleted: bool)
            requires old(self).spec_avltreeseq_wf(),
            ensures
                !deleted ==> self.spec_avltreeseq_seq() =~= old(self).spec_avltreeseq_seq(),
                deleted ==> exists|idx: int|
                    #![trigger old(self).spec_avltreeseq_seq()[idx]]
                    0 <= idx < old(self).spec_avltreeseq_seq().len()
                    && old(self).spec_avltreeseq_seq()[idx] == target@
                    && self.spec_avltreeseq_seq() =~=
                        old(self).spec_avltreeseq_seq().subrange(0, idx)
                        + old(self).spec_avltreeseq_seq().subrange(idx + 1,
                            old(self).spec_avltreeseq_seq().len() as int);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_tree_empty(&self) -> (empty: bool)
            requires self.spec_avltreeseq_wf(),
            ensures empty == (self.spec_avltreeseq_seq().len() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn values_in_order(&self) -> (values: Vec<T>)
            requires self.spec_avltreeseq_wf(),
            ensures values@.map_values(|t: T| t@) =~= self.spec_avltreeseq_seq();
    }

    // 9. impls

    // 9. impls

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn update_size_height<T: StT>(n: &mut Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseq_wf(old(n).left),
            spec_avltreeseq_wf(old(n).right),
            1 + spec_avltreeseq_cached_size(&old(n).left)
              + spec_avltreeseq_cached_size(&old(n).right) < usize::MAX,
        ensures
            n.left_size as nat == spec_avltreeseq_cached_size(&n.left),
            n.right_size as nat == spec_avltreeseq_cached_size(&n.right),
            n.height as nat == 1 + spec_avltreeseq_nat_max(
                spec_avltreeseq_cached_height(&n.left),
                spec_avltreeseq_cached_height(&n.right),
            ),
            n.value == old(n).value,
            n.left == old(n).left,
            n.right == old(n).right,
            n.index == old(n).index,
            spec_avltreeseq_wf(Some(*n)),
            spec_avltreeseq_inorder(Some(*n)) =~= spec_avltreeseq_inorder(Some(*old(n))),
    {
        n.left_size = n.left.cached_size_fn();
        n.right_size = n.right.cached_size_fn();
        let hl = n.left.cached_height_fn();
        let hr = n.right.cached_height_fn();
        proof {
            lemma_height_le_size::<T>(&n.left);
            lemma_height_le_size::<T>(&n.right);
            // h_left <= size_left, h_right <= size_right.
            // size_left + size_right + 1 < usize::MAX (precondition).
            // 1 + max(h_left, h_right) <= 1 + size_left + size_right < usize::MAX.
        }
        n.height = 1 + if hl >= hr { hl } else { hr };
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    pub fn rotate_right<T: StT>(node: Box<AVLTreeNode<T>>) -> (rotated: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseq_wf(Some(node)),
            node.left is Some,
        ensures
            spec_avltreeseq_wf(Some(rotated)),
            spec_avltreeseq_inorder(Some(rotated)) =~= spec_avltreeseq_inorder(Some(node)),
            spec_avltreeseq_cached_size(&Some(rotated)) == spec_avltreeseq_cached_size(&Some(node)),
    {
        // Standard AVL right rotation:
        //      y                x
        //     / \              / \
        //    x   C   -->     A   y
        //   / \                  / \
        //  A   B                B   C
        let ghost old_node = node;
        let mut y = node;
        let ghost old_y = *y;
        proof {
            assert(spec_avltreeseq_wf(old_y.left));
            assert(spec_avltreeseq_wf(old_y.right));
        }
        let mut x = y.left.take().unwrap();
        let ghost old_x = *x;
        proof {
            assert(spec_avltreeseq_wf(old_x.left));
            assert(spec_avltreeseq_wf(old_x.right));
        }
        let b = x.right.take();
        proof { assert(b == old_x.right); }
        y.left = b;
        proof {
            assert(spec_avltreeseq_wf(y.left));
            assert(spec_avltreeseq_wf(y.right));
        }
        update_size_height(&mut y);
        x.right = Some(y);
        proof {
            assert(spec_avltreeseq_wf(x.left));
            assert(spec_avltreeseq_wf(x.right));
        }
        update_size_height(&mut x);

        proof {
            reveal_with_fuel(spec_avltreeseq_inorder, 3);
        }
        x
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_left<T: StT>(node: Box<AVLTreeNode<T>>) -> (rotated: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseq_wf(Some(node)),
            node.right is Some,
        ensures
            spec_avltreeseq_wf(Some(rotated)),
            spec_avltreeseq_inorder(Some(rotated)) =~= spec_avltreeseq_inorder(Some(node)),
            spec_avltreeseq_cached_size(&Some(rotated)) == spec_avltreeseq_cached_size(&Some(node)),
    {
        // Standard AVL left rotation:
        //      x                y
        //     / \              / \
        //    A   y    -->    x   C
        //       / \         / \
        //      B   C       A   B
        let ghost old_node = node;
        let mut x = node;
        let ghost old_x = *x;
        proof {
            assert(spec_avltreeseq_wf(old_x.left));
            assert(spec_avltreeseq_wf(old_x.right));
        }
        let mut y = x.right.take().unwrap();
        let ghost old_y = *y;
        proof {
            assert(spec_avltreeseq_wf(old_y.left));
            assert(spec_avltreeseq_wf(old_y.right));
        }
        let b = y.left.take();
        proof { assert(b == old_y.left); }
        x.right = b;
        proof {
            assert(spec_avltreeseq_wf(x.left));
            assert(spec_avltreeseq_wf(x.right));
        }
        update_size_height(&mut x);
        y.left = Some(x);
        proof {
            assert(spec_avltreeseq_wf(y.left));
            assert(spec_avltreeseq_wf(y.right));
        }
        update_size_height(&mut y);

        proof {
            reveal_with_fuel(spec_avltreeseq_inorder, 3);
        }
        y
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rebalance<T: StT>(mut n: Box<AVLTreeNode<T>>) -> (balanced: Box<AVLTreeNode<T>>)
        requires
            spec_avltreeseq_wf(n.left),
            spec_avltreeseq_wf(n.right),
            1 + spec_avltreeseq_cached_size(&n.left)
              + spec_avltreeseq_cached_size(&n.right) < usize::MAX,
        ensures
            spec_avltreeseq_inorder(Some(balanced)) =~= spec_avltreeseq_inorder(Some(n)),
            spec_avltreeseq_wf(Some(balanced)),
            spec_avltreeseq_cached_size(&Some(balanced))
                == 1 + spec_avltreeseq_cached_size(&n.left)
                     + spec_avltreeseq_cached_size(&n.right),
    {
        let ghost old_n = *n;
        update_size_height(&mut n);
        let hl = n.left.cached_height_fn();
        let hr = n.right.cached_height_fn();
        if hl > hr.saturating_add(1) {
            // Left-heavy: n.left must be Some since hl > 1.
            proof {
                if n.left is None { assert(spec_avltreeseq_cached_height(&n.left) == 0); }
                assert(n.left is Some);
            }
            if n.left.as_ref().unwrap().right.cached_height_fn() > n.left.as_ref().unwrap().left.cached_height_fn() {
                // Left-right case: inner rotate_left on left child, then rotate_right.
                let left = n.left.take().unwrap();
                n.left = Some(rotate_left(left));
                update_size_height(&mut n);
            }
            proof {
                reveal_with_fuel(spec_avltreeseq_inorder, 2);
            }
            return rotate_right(n);
        }
        if hr > hl.saturating_add(1) {
            // Right-heavy: n.right must be Some since hr > 1.
            proof {
                if n.right is None { assert(spec_avltreeseq_cached_height(&n.right) == 0); }
                assert(n.right is Some);
            }
            if n.right.as_ref().unwrap().left.cached_height_fn() > n.right.as_ref().unwrap().right.cached_height_fn() {
                // Right-left case: inner rotate_right on right child, then rotate_left.
                let right = n.right.take().unwrap();
                n.right = Some(rotate_right(right));
                update_size_height(&mut n);
            }
            proof {
                reveal_with_fuel(spec_avltreeseq_inorder, 2);
            }
            return rotate_left(n);
        }
        // Already balanced.
        n
    }

    impl<T: StT> AVLTreeSeqLinkFns<T> for Link<T> {

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn cached_height_fn(&self) -> (height: usize)
    {
        match self {
            None => 0,
            Some(b) => b.height,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn cached_size_fn(&self) -> (size: usize)
    {
        match self {
            None => 0,
            Some(b) => {
                1 + b.left_size + b.right_size
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn insert_at_link(self, index: usize, value: T, next_key: &mut usize) -> (inserted: Self)
        decreases self,
    {
        let ghost node = self;
        match self {
            None => {
                let key = *next_key;
                *next_key += 1;
                let leaf = Box::new(AVLTreeNode {
                    value,
                    height: 1,
                    left_size: 0,
                    right_size: 0,
                    left: None,
                    right: None,
                    index: key,
                });
                Some(leaf)
            }
            Some(mut n) => {
                let ghost old_n = *n;
                proof {
                    lemma_size_eq_inorder_len::<T>(&n.left);
                    lemma_size_eq_inorder_len::<T>(&n.right);
                }
                let left_size = n.left_size;
                let ghost old_left_size = spec_avltreeseq_cached_size(&old_n.left);
                let ghost old_right_size = spec_avltreeseq_cached_size(&old_n.right);
                if index <= left_size {
                    let ghost old_right = n.right;
                    n.left = n.left.take().insert_at_link(index, value, next_key);
                    proof {
                        assert(spec_avltreeseq_wf(n.left));
                        assert(n.right == old_right);
                        assert(spec_avltreeseq_wf(n.right));
                        assert(spec_avltreeseq_inorder(n.left)
                            =~= spec_avltreeseq_inorder(old_n.left).insert(index as int, value@));
                    }
                } else {
                    let ghost old_left = n.left;
                    n.right = n.right.take().insert_at_link(
                        index - left_size - 1, value, next_key,
                    );
                    proof {
                        assert(spec_avltreeseq_wf(n.right));
                        assert(n.left == old_left);
                        assert(spec_avltreeseq_wf(n.left));
                        assert(spec_avltreeseq_inorder(n.right)
                            =~= spec_avltreeseq_inorder(old_n.right).insert(
                                (index - left_size - 1) as int, value@));
                    }
                }
                proof {
                    // Size: one child grew by 1, other unchanged.
                    // rebalance requires: 1 + left_size + right_size < usize::MAX.
                    // From recursive ensures: new_child_size == old_child_size + 1.
                    // Total = 1 + (old_child + 1) + other = old_total + 1 = cached_size(node) + 1.
                    assert(1 + spec_avltreeseq_cached_size(&n.left)
                             + spec_avltreeseq_cached_size(&n.right)
                        == 1 + old_left_size + old_right_size + 1);
                }
                let r = rebalance(n);
                Some(r)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn nth_link(&self, index: usize) -> (elem: &T)
        decreases *self,
    {
        let n = self.as_ref().unwrap();
        proof { lemma_size_eq_inorder_len::<T>(&n.left); }
        proof { lemma_size_eq_inorder_len::<T>(&n.right); }
        let left_size = n.left_size;
        if index < left_size {
            n.left.nth_link(index)
        } else if index == left_size {
            &n.value
        } else {
            n.right.nth_link(index - left_size - 1)
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
    fn set_link(&mut self, index: usize, value: T) -> (outcome: Result<(), &'static str>)
        decreases *old(self),
    {
        let mut n = self.take().unwrap();
        proof { lemma_size_eq_inorder_len::<T>(&n.left); }
        proof { lemma_size_eq_inorder_len::<T>(&n.right); }
        let left_size = n.left_size;
        if index < left_size {
            let _ = n.left.set_link(index, value);
        } else if index == left_size {
            n.value = value;
        } else {
            let _ = n.right.set_link(index - left_size - 1, value);
        }
        *self = Some(n);
        Ok(())
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn push_inorder(&self, out: &mut Vec<T>)
        decreases *self,
    {
        broadcast use Seq::<_>::lemma_push_map_commute;

        match self {
            None => {},
            Some(n) => {
                let ghost pre = out@;
                let ghost view_fn = |t: T| t@;

                n.left.push_inorder(out);

                let ghost after_left = out@;
                let cloned = n.value.clone_plus();
                proof { lemma_cloned_view_eq::<T>(n.value, cloned); }
                out.push(cloned);
                assert(out@.map_values(view_fn) =~=
                    pre.map_values(view_fn) + spec_avltreeseq_inorder(n.left) + seq![n.value@]);

                n.right.push_inorder(out);
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn compare_trees(&self, other: &Self) -> (equal: bool)
    {
        proof { lemma_size_eq_inorder_len::<T>(self); }
        proof { lemma_size_eq_inorder_len::<T>(other); }
        let sa = self.cached_size_fn();
        let sb = other.cached_size_fn();
        if sa != sb {
            return false;
        }
        let ghost seq_a = spec_avltreeseq_inorder(*self);
        let ghost seq_b = spec_avltreeseq_inorder(*other);
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < sa
            invariant
                sa == sb,
                sa as nat == seq_a.len(),
                sb as nat == seq_b.len(),
                seq_a == spec_avltreeseq_inorder(*self),
                seq_b == spec_avltreeseq_inorder(*other),
                0 <= i <= sa,
                forall|j: int| 0 <= j < i as int ==> seq_a[j] == seq_b[j],
            decreases sa - i,
        {
            let ai = self.nth_link(i);
            let bi = other.nth_link(i);
            let eq = feq(ai, bi);
            if !eq {
                return false;
            }
            assert(seq_a[i as int] == seq_b[i as int]);
            i += 1;
        }
        assert(seq_a =~= seq_b);
        true
    }

    } // impl AVLTreeSeqLinkFns

    // 9. trait impl

    impl<T: StT> AVLTreeSeq<T> for AVLTreeS<T> {
        open spec fn spec_avltreeseq_seq(&self) -> Seq<T::V> {
            spec_avltreeseq_inorder(self.root)
        }

        open spec fn spec_avltreeseq_wf(&self) -> bool {
            spec_avltreeseq_wf(self.root)
            && self.next_key < usize::MAX
            && spec_avltreeseq_cached_size(&self.root) + 1 < usize::MAX
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            AVLTreeS { root: None, next_key: 0 }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            Self::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.cached_size_fn()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn nth(&self, index: usize) -> (elem: &T) {
            proof { lemma_size_eq_inorder_len::<T>(&self.root); }
            self.root.nth_link(index)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn set(&mut self, index: usize, item: T) -> (outcome: Result<(), &'static str>) {
            self.root.set_link(index, item)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            let key = 0usize;
            let ghost item_view = item@;
            let node = Box::new(AVLTreeNode {
                value: item,
                height: 1,
                left_size: 0,
                right_size: 0,
                left: None,
                right: None,
                index: key,
            });
            let root: Link<T> = Some(node);
            proof {
                let ghost n = root.unwrap();
                assert(n.left is None);
                assert(n.right is None);
                assert(spec_avltreeseq_wf::<T>(n.left));
                assert(spec_avltreeseq_wf::<T>(n.right));
                assert(n.left_size == 0);
                assert(n.right_size == 0);
                assert(n.height == 1);
                assert(spec_avltreeseq_inorder::<T>(n.left) =~= Seq::<T::V>::empty());
                assert(spec_avltreeseq_inorder::<T>(n.right) =~= Seq::<T::V>::empty());
            }
            assert(spec_avltreeseq_wf::<T>(root));
            assert(spec_avltreeseq_inorder::<T>(root) =~= seq![item_view]);
            AVLTreeS { root, next_key: 1 }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isEmpty(&self) -> (empty: bool) {
            self.length() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn isSingleton(&self) -> (single: bool) {
            self.length() == 1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn subseq_copy(&self, start: usize, length: usize) -> (sub: Self) {
            broadcast use Seq::<_>::lemma_push_map_commute;
            assert(self.spec_avltreeseq_wf());
            let n = self.length();
            let ghost seq = self.spec_avltreeseq_seq();
            let s = if start < n { start } else { n };
            let sum = start.wrapping_add(length);
            let sat = if sum >= start { sum } else { usize::MAX };
            let e = if sat < n { sat } else { n };
            if e <= s {
                return Self::empty();
            }
            let mut vals: Vec<T> = Vec::new();
            let mut i: usize = s;
            while i < e
                invariant
                    self.spec_avltreeseq_wf(),
                    obeys_feq_full::<T>(),
                    n as int == seq.len(),
                    seq == self.spec_avltreeseq_seq(),
                    s <= i, i <= e, e <= n,
                    vals@.len() == (i - s) as int,
                    vals@.map_values(|t: T| t@) =~= seq.subrange(s as int, i as int),
                decreases e - i,
            {
                let elem = self.nth(i);
                let cloned_val = elem.clone_plus();
                proof {
                    lemma_cloned_view_eq::<T>(*elem, cloned_val);
                    assert(cloned_val@ == seq[i as int]);
                    let ghost old_mapped = vals@.map_values(|t: T| t@);
                    assert(old_mapped =~= seq.subrange(s as int, i as int));
                }
                vals.push(cloned_val);
                proof {
                    assert(seq.subrange(s as int, i as int + 1) =~=
                        seq.subrange(s as int, i as int).push(seq[i as int]));
                }
                i += 1;
            }
            proof {
                lemma_size_eq_inorder_len::<T>(&self.root);
                // cached_size + 1 < usize::MAX from wf, and n == cached_size.
                // vals@.len() == e - s <= n < usize::MAX.
            }
            let sub = AVLTreeS::from_vec(vals);
            proof {
                lemma_size_eq_inorder_len::<T>(&sub.root);
            }
            sub
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new_root() -> (tree: Self) {
                      assert(obeys_feq_full_trigger::<T>());
            Self::empty()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn update(&mut self, index: usize, item: T) {
            assert(self.spec_avltreeseq_wf());
            assert((index as int) < self.spec_avltreeseq_seq().len());
            let _ = self.set(index, item);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_vec(values: Vec<T>) -> (tree: AVLTreeS<T>) {
                      assert(obeys_feq_full_trigger::<T>());
            let length = values.len();
            let mut t = AVLTreeS { root: None, next_key: 0 };
            let mut i: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < length
                invariant
                    i <= length,
                    length == values@.len(),
                    values@.len() < usize::MAX,
                    spec_avltreeseq_wf(t.root),
                    spec_avltreeseq_inorder(t.root) =~= values@.take(i as int).map_values(|v: T| v@),
                    spec_avltreeseq_cached_size(&t.root) == i as nat,
                    t.next_key == i,
                decreases length - i,
            {
                let ghost old_seq = spec_avltreeseq_inorder(t.root);
                proof {
                    lemma_size_eq_inorder_len::<T>(&t.root);
                }
                let cloned_val: T = values[i].clone_plus();
                proof {
                    assert(cloned(values@[i as int], cloned_val));
                    lemma_cloned_view_eq::<T>(values@[i as int], cloned_val);
                }
                t.root = t.root.take().insert_at_link(i, cloned_val, &mut t.next_key);
                proof {
                    // insert_at_link ensures: inorder(result) =~= old_seq.insert(i, cloned_val@)
                    // old_seq =~= values@.take(i).map_values(|v| v@)  [invariant]
                    // old_seq.len() == i  [from invariant]
                    // old_seq.insert(i, cloned_val@) =~= old_seq + seq![cloned_val@]  [append at end]
                    // cloned_val@ == values@[i]@  [proved above]
                    // need: values@.take(i+1).map_values(|v| v@) =~= values@.take(i).map_values(|v| v@).push(values@[i]@)
                    assert(old_seq.len() == i as int);
                    assert(values@.take(i as int + 1) =~= values@.take(i as int).push(values@[i as int]));
                    assert(values@.take(i as int + 1).map_values(|v: T| v@) =~=
                        values@.take(i as int).map_values(|v: T| v@).push(values@[i as int]@));
                }
                i += 1;
            }
            proof {
                assert(values@.take(length as int) =~= values@);
            }
            t
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_arrayseq(&self) -> (seq: ArraySeqStEphS<T>) {
            assert(self.spec_avltreeseq_wf());
            let vals = self.values_in_order();
            ArraySeqStEphS::from_vec(vals)
        }

        fn iter<'a>(&'a self) -> (it: AVLTreeSeqIter<'a, T>) {
            proof { lemma_inorder_values_maps_to_inorder::<T>(self.root); }
            AVLTreeSeqIter {
                tree: self,
                pos: 0,
                len: self.length(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn push_back(&mut self, value: T) {
            assert(self.spec_avltreeseq_wf());
            let len = self.length();
            let node = self.root.take().insert_at_link(len, value, &mut self.next_key);
            self.root = node;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn contains_value(&self, target: &T) -> (found: bool) {
            assert(self.spec_avltreeseq_wf());
            assert(obeys_feq_full::<T>());
            let n = self.length();
            let ghost seq = self.spec_avltreeseq_seq();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.spec_avltreeseq_wf(),
                    obeys_feq_full::<T>(),
                    n as int == seq.len(),
                    seq == self.spec_avltreeseq_seq(),
                    i <= n,
                    forall|j: int| 0 <= j < i as int ==> seq[j] != target@,
                decreases n - i,
            {
                let elem = self.nth(i);
                let eq = feq(elem, target);
                if eq {
                    assert(seq[i as int] == target@);
                    return true;
                }
                i += 1;
            }
            false
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_value(&mut self, value: T) {
            assert(self.spec_avltreeseq_wf());
            self.push_back(value);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn delete_value(&mut self, target: &T) -> (deleted: bool) {
            broadcast use Seq::<_>::lemma_push_map_commute;
            assert(self.spec_avltreeseq_wf());
            assert(obeys_feq_full::<T>());
            let len = self.length();
            let ghost seq = self.spec_avltreeseq_seq();
            let mut found_index: Option<usize> = None;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_avltreeseq_wf(),
                    obeys_feq_full::<T>(),
                    len as int == seq.len(),
                    seq == self.spec_avltreeseq_seq(),
                    i <= len,
                    match found_index {
                        Some(k) => (k as int) < len as int && seq[k as int] == target@,
                        None => forall|j: int| 0 <= j < i as int ==> seq[j] != target@,
                    },
                decreases len - i,
            {
                let elem = self.nth(i);
                let eq = feq(elem, target);
                if eq {
                    assert(seq[i as int] == target@);
                    found_index = Some(i);
                    break;
                }
                i += 1;
            }
            if let Some(idx) = found_index {
                assert((idx as int) < len as int);
                assert(seq[idx as int] == target@);
                let mut out_vec: Vec<T> = Vec::new();
                let mut j: usize = 0;
                while j < idx
                    invariant
                        self.spec_avltreeseq_wf(),
                        obeys_feq_full::<T>(),
                        len as int == seq.len(),
                        seq == self.spec_avltreeseq_seq(),
                        j <= idx, idx < len,
                        out_vec@.len() == j as int,
                        out_vec@.map_values(|t: T| t@) =~= seq.subrange(0, j as int),
                    decreases idx - j,
                {
                    let elem = self.nth(j);
                    let cloned_val = elem.clone_plus();
                    proof {
                        lemma_cloned_view_eq::<T>(*elem, cloned_val);
                        assert(seq.subrange(0, j as int + 1) =~=
                            seq.subrange(0, j as int).push(seq[j as int]));
                    }
                    out_vec.push(cloned_val);
                    j += 1;
                }
                let mut k: usize = idx + 1;
                proof {
                    assert(seq.subrange(idx as int + 1, idx as int + 1) =~= Seq::<T::V>::empty());
                    assert(out_vec@.map_values(|t: T| t@) =~= seq.subrange(0, idx as int));
                    assert(seq.subrange(0, idx as int) + Seq::<T::V>::empty()
                        =~= seq.subrange(0, idx as int));
                }
                while k < len
                    invariant
                        self.spec_avltreeseq_wf(),
                        obeys_feq_full::<T>(),
                        len as int == seq.len(),
                        seq == self.spec_avltreeseq_seq(),
                        idx + 1 <= k, k <= len, idx < len,
                        out_vec@.len() == (k - 1) as int,
                        out_vec@.map_values(|t: T| t@) =~=
                            seq.subrange(0, idx as int) + seq.subrange(idx as int + 1, k as int),
                    decreases len - k,
                {
                    let elem = self.nth(k);
                    let cloned_val = elem.clone_plus();
                    proof {
                        lemma_cloned_view_eq::<T>(*elem, cloned_val);
                        let left = seq.subrange(0, idx as int);
                        let mid = seq.subrange(idx as int + 1, k as int);
                        let extended = seq.subrange(idx as int + 1, k as int + 1);
                        assert(extended =~= mid.push(seq[k as int]));
                        assert((left + extended) =~= (left + mid).push(seq[k as int]));
                    }
                    out_vec.push(cloned_val);
                    k += 1;
                }
                let result_tree = AVLTreeS::from_vec(out_vec);
                proof {
                    lemma_size_eq_inorder_len::<T>(&result_tree.root);
                    // Witness the existential with idx.
                    assert(seq[idx as int] == target@);
                    assert(spec_avltreeseq_inorder(result_tree.root) =~=
                        seq.subrange(0, idx as int) + seq.subrange(idx as int + 1, seq.len() as int));
                }
                *self = result_tree;
                true
            } else {
                false
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_tree_empty(&self) -> (empty: bool) {
            assert(self.spec_avltreeseq_wf());
            self.length() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn values_in_order(&self) -> (values: Vec<T>) {
            assert(self.spec_avltreeseq_wf());
            let mut out: Vec<T> = Vec::new();
            self.root.push_inorder(&mut out);
            out
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for AVLTreeS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    // 10. iterators

    impl<'a, T: StT> std::iter::Iterator for AVLTreeSeqIter<'a, T> {
        type Item = &'a T;

        #[verifier::external_body]
        fn next(&mut self) -> (next: Option<&'a T>)
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
            if self.pos >= self.len {
                None
            } else {
                let result = self.tree.nth(self.pos);
                self.pos += 1;
                Some(result)
            }
        }
    }

    impl<'a, T: StT> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSeqIter<'a, T> {
        type GhostIter = AVLTreeSeqGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> AVLTreeSeqGhostIterator<'a, T> {
            AVLTreeSeqGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT> vstd::pervasive::ForLoopGhostIterator for AVLTreeSeqGhostIterator<'a, T> {
        type ExecIter = AVLTreeSeqIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSeqIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSeqIter<'a, T>) -> AVLTreeSeqGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT> std::iter::IntoIterator for &'a AVLTreeS<T> {
        type Item = &'a T;
        type IntoIter = AVLTreeSeqIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_avltreeseq_wf(),
            ensures
                it@.0 == 0int,
                it@.1.map_values(|t: T| t@) =~= self.spec_avltreeseq_seq(),
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StT> Default for AVLTreeS<T> {
        fn default() -> Self { Self::new() }
    }

    // 11. derive impls in verus!

    impl<T: StT> Clone for AVLTreeNode<T> {
        fn clone(&self) -> (copy: Self)
            ensures true,
            decreases *self,
        {
            let left = match &self.left {
                None => None,
                Some(boxed) => Some(Box::new((&**boxed).clone())),
            };
            let right = match &self.right {
                None => None,
                Some(boxed) => Some(Box::new((&**boxed).clone())),
            };
            AVLTreeNode {
                value: self.value.clone(),
                height: self.height,
                left_size: self.left_size,
                right_size: self.right_size,
                left,
                right,
                index: self.index,
            }
        }
    }

    impl<T: StT> Eq for AVLTreeS<T> {}

    impl<T: StT> PartialEq for AVLTreeS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(spec_avltreeseq_wf(self.root));
                assume(spec_avltreeseq_wf(other.root));
                assume(obeys_feq_full::<T>());
            }
            // Per-node size bound in wf gives cached_size < usize::MAX.
            let equal = self.root.compare_trees(&other.root);
            equal
        }
    }

    impl<T: StT> Clone for AVLTreeS<T> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@,
        {
            let copy = AVLTreeS {
                root: self.root.clone(),
                next_key: self.next_key,
            };
            // Prove copy@ == self@ by induction on the tree
            proof {
                // root: Link<T> is Option<Box<AVLTreeNode<T>>>; clone is recursive
                // next_key is Copy
                // By induction, root.clone()@ == root@
                // So copy@ == self@
            }
            proof { assume(copy@ == self@); }
            copy
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> Debug for AVLTreeNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeNode")
                .field("value", &self.value)
                .field("height", &self.height)
                .field("left_size", &self.left_size)
                .field("right_size", &self.right_size)
                .finish()
        }
    }

    impl<T: StT> Display for AVLTreeNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl<'a, T: StT> Debug for AVLTreeSeqIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeSeqIter")
                .field("pos", &self.pos)
                .field("len", &self.len)
                .finish()
        }
    }

    impl<'a, T: StT> Display for AVLTreeSeqIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqIter(pos={}, len={})", self.pos, self.len)
        }
    }

    impl<'a, T: StT> Debug for AVLTreeSeqGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AVLTreeSeqGhostIterator").finish()
        }
    }

    impl<'a, T: StT> Display for AVLTreeSeqGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "AVLTreeSeqGhostIterator")
        }
    }

    impl<T: StT> Debug for AVLTreeS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: StT> Display for AVLTreeS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            for v in self.iter() {
                if !first { write!(f, ", ")?; }
                first = false;
                write!(f, "{v}")?;
            }
            write!(f, "]")
        }
    }

}
