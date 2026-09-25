// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Ephemeral Red-Black balanced binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on Link/Node) in sections 6/8/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 8d. traits
//	Section 9d. impls
//	Section 10d. iterators
//	Section 11c. top level coarse locking
//	Section 12d. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!

//		Section 1. module

pub mod BSTRBMtEph {


    //		Section 2. imports

    use std::sync::Arc;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power2::{pow2, lemma2_to64, lemma_pow2_adds, lemma_pow2_pos, lemma_pow2_strictly_increases,
        lemma_pow2_unfold};
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::mul::lemma_mul_upper_bound;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::slice::slice_subrange;

    verus! 
{

    //		Section 3. broadcast use

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4a. type definitions


    // (Arc kept for filter_parallel/reduce_parallel closure sharing.)


    #[derive(Clone, Copy, PartialEq, Eq, StructuralEq)]
    pub enum Color {
        Red,
        Black,
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: StTInMtT + Ord + TotalOrder> {
        pub key: T,
        pub color: Color,
        pub size: usize,
        pub left: Option<Box<Node<T>>>,
        pub right: Option<Box<Node<T>>>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    //		Section 6b. spec fns


    /// Structural node count for RB tree links.
    pub open spec fn link_spec_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_spec_size(node.left) + link_spec_size(node.right),
        }
    }

    /// Spec-level containment for RB tree links.
    pub open spec fn link_contains<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, target: T) -> bool
        decreases link,
    {
        match link {
            None => false,
            Some(node) => node.key == target
                || link_contains(node.left, target)
                || link_contains(node.right, target),
        }
    }

    /// Spec-level height for RB tree links.
    pub open spec fn link_height<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => {
                let lh = link_height(node.left);
                let rh = link_height(node.right);
                1 + if lh > rh { lh } else { rh }
            }
        }
    }

    /// BST ordering invariant for RB tree links.
    pub open spec fn spec_is_bst_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_is_bst_link(node.left)
                && spec_is_bst_link(node.right)
                && (forall|x: T| (#[trigger] link_contains(node.left, x)) ==>
                    TotalOrder::le(x, node.key) && x != node.key)
                && (forall|x: T| (#[trigger] link_contains(node.right, x)) ==>
                    TotalOrder::le(node.key, x) && x != node.key)
            }
        }
    }

    /// Convert Link<T> (concrete RB tree pointer) to BalBinTree<T> (abstract binary tree).
    /// Strips color and size, retaining structure and keys.
    pub open spec fn link_to_bbt<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> BalBinTree<T>
        decreases link,
    {
        match link {
            None => BalBinTree::Leaf,
            Some(node) => BalBinTree::Node(Box::new(BalBinNode {
                left: link_to_bbt(node.left),
                value: node.key,
                right: link_to_bbt(node.right),
            })),
        }
    }

    /// True iff the link is a node colored red. An empty link is black.
    pub open spec fn link_is_red<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(node) => node.color == Color::Red,
        }
    }

    /// Number of black nodes on the leftmost path from the link to an empty leaf.
    /// In a left-leaning red-black tree every path has this many black nodes.
    pub open spec fn link_black_height<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => link_black_height(node.left)
                + if node.color == Color::Black { 1nat } else { 0nat },
        }
    }

    /// Left-leaning red-black invariant (Sedgewick 2008, the 2-3 variant): no red
    /// right child, no red node with a red left child, and equal black height on
    /// both sides of every node. The root may be red.
    pub open spec fn spec_is_llrb_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_is_llrb_link(node.left)
                && spec_is_llrb_link(node.right)
                && !link_is_red(node.right)
                && !(node.color == Color::Red && link_is_red(node.left))
                && link_black_height(node.left) == link_black_height(node.right)
            }
        }
    }

    /// The state insert may leave below a red parent: a node whose subtrees are
    /// left-leaning red-black trees of equal black height and whose right child is
    /// black, but which may itself be red with a red left child.
    pub open spec fn spec_is_llrb_relaxed_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(node) => {
                spec_is_llrb_link(node.left)
                && spec_is_llrb_link(node.right)
                && !link_is_red(node.right)
                && link_black_height(node.left) == link_black_height(node.right)
            }
        }
    }

    /// The size field stored at the root of a link; 0 for an empty link.
    pub open spec fn link_size_field<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> usize {
        match link {
            None => 0usize,
            Some(node) => node.size,
        }
    }

    /// Every cached size field equals the structural node count below it.
    pub open spec fn spec_size_cached_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                node.size as nat == link_spec_size(link)
                && spec_size_cached_link(node.left)
                && spec_size_cached_link(node.right)
            }
        }
    }

    /// The node after `update`: its size field recomputed from its children's size
    /// fields when the sum fits in usize, unchanged otherwise.
    pub open spec fn spec_updated<T: StTInMtT + Ord + TotalOrder>(node: Node<T>) -> Node<T> {
        let ls = link_size_field(node.left);
        let rs = link_size_field(node.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            Node { key: node.key, color: node.color, size: (1 + ls + rs) as usize,
                   left: node.left, right: node.right }
        } else {
            node
        }
    }

    /// The link after `rotate_left`: the right child x of h becomes the root with h's
    /// color, h becomes x's red left child, and x's left subtree moves under h.
    pub open spec fn spec_rotate_left<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(h) => match h.right {
                None => link,
                Some(x) => {
                    let new_h = spec_updated(Node { key: h.key, color: Color::Red, size: h.size,
                                                    left: h.left, right: x.left });
                    Some(Box::new(spec_updated(Node { key: x.key, color: h.color, size: x.size,
                                                      left: Some(Box::new(new_h)), right: x.right })))
                },
            },
        }
    }

    /// The link after `rotate_right`: the left child x of h becomes the root with h's
    /// color, h becomes x's red right child, and x's right subtree moves under h.
    pub open spec fn spec_rotate_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(h) => match h.left {
                None => link,
                Some(x) => {
                    let new_h = spec_updated(Node { key: h.key, color: Color::Red, size: h.size,
                                                    left: x.right, right: h.right });
                    Some(Box::new(spec_updated(Node { key: x.key, color: h.color, size: x.size,
                                                      left: x.left, right: Some(Box::new(new_h)) })))
                },
            },
        }
    }

    /// The opposite color.
    pub open spec fn spec_toggle(c: Color) -> Color {
        match c {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }

    /// The link with its root's color toggled.
    pub open spec fn spec_toggle_root<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(n) => Some(Box::new(Node { key: n.key, color: spec_toggle(n.color), size: n.size,
                                            left: n.left, right: n.right })),
        }
    }

    /// The link after `flip_colors`: the root and both children change color.
    pub open spec fn spec_flip_colors<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(n) => Some(Box::new(Node { key: n.key, color: spec_toggle(n.color), size: n.size,
                                            left: spec_toggle_root(n.left),
                                            right: spec_toggle_root(n.right) })),
        }
    }

    /// `fix_up`'s test for a left rotation: red right child, black left child.
    pub open spec fn spec_needs_rotate_left<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(n) => link_is_red(n.right) && !link_is_red(n.left),
        }
    }

    /// `fix_up`'s test for a right rotation: red left child with a red left child.
    pub open spec fn spec_needs_rotate_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(n) => match n.left {
                None => false,
                Some(l) => l.color == Color::Red && link_is_red(l.left),
            },
        }
    }

    /// `fix_up`'s test for a color flip: both children red.
    pub open spec fn spec_needs_flip<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(n) => link_is_red(n.left) && link_is_red(n.right),
        }
    }

    /// The link with its root's size field recomputed by `update`.
    pub open spec fn spec_update_root<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(n) => Some(Box::new(spec_updated(*n))),
        }
    }

    /// The link after `fix_up`: rotate left, rotate right, flip colors, each when its
    /// test holds, then recompute the root's size.
    pub open spec fn spec_fix_up<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        let s1 = if spec_needs_rotate_left(link) { spec_rotate_left(link) } else { link };
        let s2 = if spec_needs_rotate_right(s1) { spec_rotate_right(s1) } else { s1 };
        let s3 = if spec_needs_flip(s2) { spec_flip_colors(s2) } else { s2 };
        spec_update_root(s3)
    }

    /// The link with its root colored black.
    pub open spec fn spec_blacken_root<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(n) => Some(Box::new(Node { key: n.key, color: Color::Black, size: n.size,
                                            left: n.left, right: n.right })),
        }
    }

    /// What `fix_up` may receive from `insert_link`: a node whose right subtree is a
    /// left-leaning red-black tree, whose left subtree is one or is relaxed and red,
    /// with equal black heights, and no two red links that fix_up cannot repair.
    pub open spec fn spec_fix_up_pre<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(h) => {
                spec_is_llrb_link(h.right)
                && (spec_is_llrb_link(h.left)
                    || (spec_is_llrb_relaxed_link(h.left) && link_is_red(h.left)))
                && link_black_height(h.left) == link_black_height(h.right)
                && (h.color == Color::Red ==> !(link_is_red(h.left) && link_is_red(h.right)))
                && (spec_needs_rotate_right(link) ==>
                    h.color == Color::Black && !link_is_red(h.right))
            }
        }
    }

    /// The link after `move_red_left`: flip colors; if the right child's left child is
    /// then red, rotate the right child right, rotate left, and flip again. Makes the
    /// left child, or its left child, red before delete descends left.
    pub open spec fn spec_move_red_left<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        let f = spec_flip_colors(link);
        match f {
            None => None,
            Some(fh) => {
                let rl_red = match fh.right { None => false, Some(r) => link_is_red(r.left) };
                if rl_red {
                    let g = Some(Box::new(Node { key: fh.key, color: fh.color, size: fh.size,
                                                 left: fh.left, right: spec_rotate_right(fh.right) }));
                    spec_flip_colors(spec_rotate_left(g))
                } else {
                    f
                }
            },
        }
    }

    /// The link after `move_red_right`: flip colors; if the left child's left child is
    /// then red, rotate right and flip again. Makes the right child, or its left child,
    /// red before delete descends right.
    pub open spec fn spec_move_red_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        let f = spec_flip_colors(link);
        match f {
            None => None,
            Some(fh) => {
                let ll_red = match fh.left { None => false, Some(l) => link_is_red(l.left) };
                if ll_red { spec_flip_colors(spec_rotate_right(f)) } else { f }
            },
        }
    }

    /// What `delete_min_link` requires: a left-leaning red-black tree whose root or
    /// whose root's left child is red, so the minimum is not alone in a 2-node.
    pub open spec fn spec_delete_min_pre<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(h) => spec_is_llrb_link(link) && (h.color == Color::Red || link_is_red(h.left)),
        }
    }

    /// A black node whose right child is red: the right-leaning 3-node that
    /// `move_red_right` hands to the right subtree.
    pub open spec fn spec_is_right_red_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        match link {
            None => false,
            Some(h) => {
                h.color == Color::Black
                && !link_is_red(h.left)
                && spec_is_llrb_link(h.left)
                && link_is_red(h.right)
                && spec_is_llrb_link(h.right)
                && link_black_height(h.left) == link_black_height(h.right)
            }
        }
    }

    /// What `delete_link` requires: a `delete_min_link` input, or a right-leaning
    /// 3-node whose key is at most the key being deleted.
    pub open spec fn spec_delete_pre<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, key: T) -> bool {
        spec_delete_min_pre(link)
        || (spec_is_right_red_link(link) && TotalOrder::le(link->Some_0.key, key))
    }

    /// Strictly increasing under TotalOrder: the order `from_sorted_slice` requires.
    pub open spec fn spec_sorted_strict<T: StTInMtT + Ord + TotalOrder>(s: Seq<T>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len() ==>
            TotalOrder::le(#[trigger] s[i], #[trigger] s[j]) && s[i] != s[j]
    }

    /// A well-formed red-black tree: BST order, the left-leaning red-black invariant,
    /// a black root, a correct size cache, and a size that fits in usize.
    pub open spec fn spec_is_rb_tree_link<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> bool {
        link_spec_size(link) <= usize::MAX
        && spec_is_bst_link(link)
        && spec_is_llrb_link(link)
        && !link_is_red(link)
        && spec_size_cached_link(link)
    }

    /// The link after delete's left-descent step: `move_red_left` when the left child
    /// and its left child are both black, otherwise unchanged.
    pub open spec fn spec_delete_borrow_left<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(h) => match h.left {
                None => link,
                Some(l) => if !(l.color == Color::Red) && !link_is_red(l.left) {
                    spec_move_red_left(link)
                } else {
                    link
                },
            },
        }
    }

    /// The link after delete's first right-descent step: rotate right when the left
    /// child is red, otherwise unchanged.
    pub open spec fn spec_delete_lean_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(h) => if link_is_red(h.left) { spec_rotate_right(link) } else { link },
        }
    }

    /// The link after delete's second right-descent step: `move_red_right` when the
    /// right child and its left child are both black, otherwise unchanged.
    pub open spec fn spec_delete_borrow_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>) -> Link<T> {
        match link {
            None => None,
            Some(h) => match h.right {
                None => link,
                Some(r) => if !(r.color == Color::Red) && !link_is_red(r.left) {
                    spec_move_red_right(link)
                } else {
                    link
                },
            },
        }
    }

    /// The facts delete needs at a node before it recurses into one child and calls
    /// `fix_up`: the keys, size, BST order, size cache, and black height of `before`
    /// are kept; both children are left-leaning red-black trees of equal black height
    /// after the recursion; a red node has two black children; and a black `before`
    /// keeps a black node whose children are not both red.
    pub open spec fn spec_delete_step<T: StTInMtT + Ord + TotalOrder>(before: Link<T>, after: Link<T>) -> bool {
        &&& after is Some
        &&& forall|z: T| #[trigger] link_contains(after, z) == link_contains(before, z)
        &&& link_spec_size(after) == link_spec_size(before)
        &&& spec_is_bst_link(after)
        &&& spec_size_cached_link(after)
        &&& link_black_height(after) == link_black_height(before)
        &&& link_black_height(after->Some_0.left) == link_black_height(after->Some_0.right)
        &&& (after->Some_0.color == Color::Red ==>
                !link_is_red(after->Some_0.left) && !link_is_red(after->Some_0.right))
        &&& (!link_is_red(before) ==>
                after->Some_0.color == Color::Black
                && !(link_is_red(after->Some_0.left) && link_is_red(after->Some_0.right)))
    }

    /// What delete needs at a node `c` reached from `before` on the right descent for
    /// `key`: `spec_delete_step`, a left-leaning red-black left child, a node key at
    /// most `key`, and a right child ready for `delete_min_link` when the node holds
    /// `key` or holding `key` and ready for `delete_link` otherwise.
    pub open spec fn spec_delete_right_ready<T: StTInMtT + Ord + TotalOrder>(
        before: Link<T>, c: Link<T>, key: T) -> bool {
        &&& spec_delete_step(before, c)
        &&& spec_is_llrb_link(c->Some_0.left)
        &&& TotalOrder::le(c->Some_0.key, key)
        &&& (c->Some_0.key == key ==> spec_delete_min_pre(c->Some_0.right))
        &&& (c->Some_0.key != key ==>
                spec_delete_pre(c->Some_0.right, key) && link_contains(c->Some_0.right, key))
    }

    //		Section 7b. proof fns/broadcast groups


    /// Bridge: link_spec_size == BalBinTree::spec_size after conversion.
    proof fn lemma_link_to_bbt_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_spec_size(link) == link_to_bbt(link).spec_size(),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_link_to_bbt_size::<T>(node.left);
                lemma_link_to_bbt_size::<T>(node.right);
            }
        }
    }

    /// Bridge: link_contains == BalBinTree::tree_contains after conversion.
    proof fn lemma_link_to_bbt_contains<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, target: T)
        ensures link_contains(link, target) == link_to_bbt(link).tree_contains(target),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_link_to_bbt_contains::<T>(node.left, target);
                lemma_link_to_bbt_contains::<T>(node.right, target);
            }
        }
    }

    /// Bridge: link_height == BalBinTree::spec_height after conversion.
    proof fn lemma_link_to_bbt_height<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_height(link) == link_to_bbt(link).spec_height(),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_link_to_bbt_height::<T>(node.left);
                lemma_link_to_bbt_height::<T>(node.right);
            }
        }
    }

    /// Bridge: spec_is_bst_link on Link implies tree_is_bst on BalBinTree conversion.
    proof fn lemma_link_to_bbt_is_bst<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires spec_is_bst_link(link),
        ensures link_to_bbt(link).tree_is_bst(),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                reveal_with_fuel(spec_is_bst_link, 2);
                lemma_link_to_bbt_is_bst::<T>(node.left);
                lemma_link_to_bbt_is_bst::<T>(node.right);
                // Veracity: NEEDED assert
                assert forall|x: T| (#[trigger] link_to_bbt(node.left).tree_contains(x))
                    implies (TotalOrder::le(x, node.key) && x != node.key)
                by {
                    lemma_link_to_bbt_contains::<T>(node.left, x);
                };
                // Veracity: NEEDED assert
                assert forall|x: T| (#[trigger] link_to_bbt(node.right).tree_contains(x))
                    implies (TotalOrder::le(node.key, x) && x != node.key)
                by {
                    lemma_link_to_bbt_contains::<T>(node.right, x);
                };
            }
        }
    }

    /// `spec_updated` changes only the size field.
    proof fn lemma_updated_fields<T: StTInMtT + Ord + TotalOrder>(node: Node<T>)
        ensures
            spec_updated(node).key == node.key,
            spec_updated(node).color == node.color,
            spec_updated(node).left == node.left,
            spec_updated(node).right == node.right,
            (spec_size_cached_link(node.left) && spec_size_cached_link(node.right)
                && link_spec_size(node.left) + link_spec_size(node.right) + 1 <= usize::MAX)
                ==> spec_updated(node).size as nat
                    == 1 + link_spec_size(node.left) + link_spec_size(node.right),
    {
        if spec_size_cached_link(node.left) && spec_size_cached_link(node.right) {
            reveal_with_fuel(spec_size_cached_link, 1);
        }
    }

    /// A cached link's size field is its structural size.
    proof fn lemma_size_field_cached<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires spec_size_cached_link(link),
        ensures link_size_field(link) as nat == link_spec_size(link),
    {
    }

    /// Rotating left preserves the keys, the size, the BST order, and the size cache.
    proof fn lemma_rotate_left_preserves<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            link is Some,
            link->Some_0.right is Some,
        ensures
            forall|z: T| #[trigger] link_contains(spec_rotate_left(link), z) == link_contains(link, z),
            link_spec_size(spec_rotate_left(link)) == link_spec_size(link),
            spec_is_bst_link(link) ==> spec_is_bst_link(spec_rotate_left(link)),
            (spec_size_cached_link(link->Some_0.left) && spec_size_cached_link(link->Some_0.right)
                && link_spec_size(link) <= usize::MAX)
                ==> spec_size_cached_link(spec_rotate_left(link)),
    {
        let h = link->Some_0;
        let x = h.right->Some_0;
        let nh0 = Node { key: h.key, color: Color::Red, size: h.size, left: h.left, right: x.left };
        let nh = spec_updated(nh0);
        let nx0 = Node { key: x.key, color: h.color, size: x.size,
                         left: Some(Box::new(nh)), right: x.right };
        let nx = spec_updated(nx0);
        let r = spec_rotate_left(link);
        lemma_updated_fields(nh0);
        lemma_updated_fields(nx0);
        assert(r == Some(Box::new(nx)));
        assert forall|z: T| #[trigger] link_contains(r, z) == link_contains(link, z) by {
            reveal_with_fuel(link_contains, 3);
        };
        assert(link_spec_size(r) == link_spec_size(link)) by {
            reveal_with_fuel(link_spec_size, 3);
        };
        if spec_is_bst_link(link) {
            reveal_with_fuel(spec_is_bst_link, 3);
            reveal_with_fuel(link_contains, 3);
            assert(link_contains(h.right, x.key));
            assert(TotalOrder::le(h.key, x.key) && x.key != h.key);
            assert forall|z: T| #[trigger] link_contains(x.left, z) implies
                TotalOrder::le(h.key, z) && z != h.key by {
                assert(link_contains(h.right, z));
            };
            assert forall|z: T| #[trigger] link_contains(Some(Box::new(nh)), z) implies
                TotalOrder::le(z, x.key) && z != x.key by {
                if z == h.key {
                } else if link_contains(h.left, z) {
                    T::transitive(z, h.key, x.key);
                    if z == x.key { T::antisymmetric(h.key, x.key); }
                } else {
                    assert(link_contains(x.left, z));
                    assert(spec_is_bst_link(h.right));
                }
            };
            assert(spec_is_bst_link(Some(Box::new(nh))));
        }
        if spec_size_cached_link(h.left) && spec_size_cached_link(h.right)
            && link_spec_size(link) <= usize::MAX {
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_spec_size, 3);
            assert(spec_size_cached_link(Some(Box::new(nh))));
        }
    }

    /// Rotating right preserves the keys, the size, the BST order, and the size cache.
    proof fn lemma_rotate_right_preserves<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            link is Some,
            link->Some_0.left is Some,
        ensures
            forall|z: T| #[trigger] link_contains(spec_rotate_right(link), z) == link_contains(link, z),
            link_spec_size(spec_rotate_right(link)) == link_spec_size(link),
            spec_is_bst_link(link) ==> spec_is_bst_link(spec_rotate_right(link)),
            (spec_size_cached_link(link->Some_0.left) && spec_size_cached_link(link->Some_0.right)
                && link_spec_size(link) <= usize::MAX)
                ==> spec_size_cached_link(spec_rotate_right(link)),
    {
        let h = link->Some_0;
        let x = h.left->Some_0;
        let nh0 = Node { key: h.key, color: Color::Red, size: h.size, left: x.right, right: h.right };
        let nh = spec_updated(nh0);
        let nx0 = Node { key: x.key, color: h.color, size: x.size,
                         left: x.left, right: Some(Box::new(nh)) };
        let nx = spec_updated(nx0);
        let r = spec_rotate_right(link);
        lemma_updated_fields(nh0);
        lemma_updated_fields(nx0);
        assert(r == Some(Box::new(nx)));
        assert forall|z: T| #[trigger] link_contains(r, z) == link_contains(link, z) by {
            reveal_with_fuel(link_contains, 3);
        };
        assert(link_spec_size(r) == link_spec_size(link)) by {
            reveal_with_fuel(link_spec_size, 3);
        };
        if spec_is_bst_link(link) {
            reveal_with_fuel(spec_is_bst_link, 3);
            reveal_with_fuel(link_contains, 3);
            assert(link_contains(h.left, x.key));
            assert(TotalOrder::le(x.key, h.key) && x.key != h.key);
            assert forall|z: T| #[trigger] link_contains(x.right, z) implies
                TotalOrder::le(z, h.key) && z != h.key by {
                assert(link_contains(h.left, z));
            };
            assert forall|z: T| #[trigger] link_contains(Some(Box::new(nh)), z) implies
                TotalOrder::le(x.key, z) && z != x.key by {
                if z == h.key {
                } else if link_contains(h.right, z) {
                    T::transitive(x.key, h.key, z);
                    if z == x.key { T::antisymmetric(x.key, h.key); }
                } else {
                    assert(link_contains(x.right, z));
                    assert(spec_is_bst_link(h.left));
                }
            };
            assert(spec_is_bst_link(Some(Box::new(nh))));
        }
        if spec_size_cached_link(h.left) && spec_size_cached_link(h.right)
            && link_spec_size(link) <= usize::MAX {
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_spec_size, 3);
            assert(spec_size_cached_link(Some(Box::new(nh))));
        }
    }

    /// Toggling a root's color preserves everything but the color.
    proof fn lemma_toggle_root_preserves<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures
            forall|z: T| #[trigger] link_contains(spec_toggle_root(link), z) == link_contains(link, z),
            link_spec_size(spec_toggle_root(link)) == link_spec_size(link),
            spec_is_bst_link(spec_toggle_root(link)) == spec_is_bst_link(link),
            spec_size_cached_link(spec_toggle_root(link)) == spec_size_cached_link(link),
            link_size_field(spec_toggle_root(link)) == link_size_field(link),
            link_is_red(spec_toggle_root(link)) == (link is Some && !link_is_red(link)),
            link is Some ==> link_black_height(spec_toggle_root(link))
                == link_black_height(link->Some_0.left)
                    + if link_is_red(link) { 1nat } else { 0nat },
            (spec_is_llrb_link(link) && link_is_red(link))
                ==> spec_is_llrb_link(spec_toggle_root(link)),
            (spec_is_llrb_link(link) && link is Some && !link_is_red(link)
                && !link_is_red(link->Some_0.left))
                ==> spec_is_llrb_link(spec_toggle_root(link)),
    {
        reveal_with_fuel(spec_is_llrb_link, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(link_spec_size, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(link_black_height, 2);
    }

    /// Flipping colors preserves the keys, the size, the BST order, and the size cache.
    proof fn lemma_flip_colors_preserves<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires link is Some,
        ensures
            forall|z: T| #[trigger] link_contains(spec_flip_colors(link), z) == link_contains(link, z),
            link_spec_size(spec_flip_colors(link)) == link_spec_size(link),
            spec_is_bst_link(link) ==> spec_is_bst_link(spec_flip_colors(link)),
            spec_size_cached_link(link) ==> spec_size_cached_link(spec_flip_colors(link)),
    {
        let n = link->Some_0;
        lemma_toggle_root_preserves(n.left);
        lemma_toggle_root_preserves(n.right);
        let r = spec_flip_colors(link);
        assert forall|z: T| #[trigger] link_contains(r, z) == link_contains(link, z) by {
            reveal_with_fuel(link_contains, 2);
        };
        reveal_with_fuel(link_spec_size, 2);
        if spec_is_bst_link(link) {
            reveal_with_fuel(spec_is_bst_link, 2);
            assert forall|z: T| #[trigger] link_contains(spec_toggle_root(n.left), z) implies
                TotalOrder::le(z, n.key) && z != n.key by {
                assert(link_contains(n.left, z));
            };
            assert forall|z: T| #[trigger] link_contains(spec_toggle_root(n.right), z) implies
                TotalOrder::le(n.key, z) && z != n.key by {
                assert(link_contains(n.right, z));
            };
        }
        if spec_size_cached_link(link) {
            reveal_with_fuel(spec_size_cached_link, 2);
        }
    }

    /// Recomputing the root's size preserves the keys, the size and the BST order,
    /// and makes the root's cache correct when the children's caches are.
    proof fn lemma_update_root_preserves<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires link is Some,
        ensures
            forall|z: T| #[trigger] link_contains(spec_update_root(link), z) == link_contains(link, z),
            link_spec_size(spec_update_root(link)) == link_spec_size(link),
            spec_is_bst_link(spec_update_root(link)) == spec_is_bst_link(link),
            spec_is_llrb_link(spec_update_root(link)) == spec_is_llrb_link(link),
            spec_is_llrb_relaxed_link(spec_update_root(link)) == spec_is_llrb_relaxed_link(link),
            link_is_red(spec_update_root(link)) == link_is_red(link),
            link_black_height(spec_update_root(link)) == link_black_height(link),
            (spec_size_cached_link(link->Some_0.left) && spec_size_cached_link(link->Some_0.right)
                && link_spec_size(link) <= usize::MAX)
                ==> spec_size_cached_link(spec_update_root(link)),
    {
        let n = link->Some_0;
        lemma_updated_fields(*n);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(link_spec_size, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(spec_is_llrb_link, 2);
        reveal_with_fuel(link_black_height, 2);
        reveal_with_fuel(spec_size_cached_link, 2);
    }

    /// Coloring the root black preserves the keys, the size, the BST order, the size
    /// cache, and the left-leaning red-black invariant.
    proof fn lemma_blacken_root_preserves<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures
            forall|z: T| #[trigger] link_contains(spec_blacken_root(link), z) == link_contains(link, z),
            link_spec_size(spec_blacken_root(link)) == link_spec_size(link),
            spec_is_bst_link(spec_blacken_root(link)) == spec_is_bst_link(link),
            spec_size_cached_link(spec_blacken_root(link)) == spec_size_cached_link(link),
            !link_is_red(spec_blacken_root(link)),
            spec_is_llrb_link(link) ==> spec_is_llrb_link(spec_blacken_root(link)),
            spec_is_llrb_relaxed_link(link) ==> spec_is_llrb_link(spec_blacken_root(link)),
    {
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(link_spec_size, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(spec_is_llrb_link, 2);
    }

    /// `fix_up` restores the left-leaning red-black invariant below a black parent
    /// and leaves a relaxed red node below a red parent, keeping the black height,
    /// the keys, the size, the BST order, and the size cache.
    proof fn lemma_fix_up<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            spec_fix_up_pre(link),
            spec_is_bst_link(link),
            spec_size_cached_link(link->Some_0.left),
            spec_size_cached_link(link->Some_0.right),
            link_spec_size(link) <= usize::MAX,
        ensures
            forall|z: T| #[trigger] link_contains(spec_fix_up(link), z) == link_contains(link, z),
            link_spec_size(spec_fix_up(link)) == link_spec_size(link),
            spec_is_bst_link(spec_fix_up(link)),
            spec_size_cached_link(spec_fix_up(link)),
            link_black_height(spec_fix_up(link)) == link_black_height(link),
            link->Some_0.color == Color::Black ==> spec_is_llrb_link(spec_fix_up(link)),
            link->Some_0.color == Color::Red ==>
                spec_is_llrb_relaxed_link(spec_fix_up(link)) && link_is_red(spec_fix_up(link)),
    {
        let h = link->Some_0;
        let l = h.left;
        let r = h.right;
        let s1 = if spec_needs_rotate_left(link) { spec_rotate_left(link) } else { link };
        let s2 = if spec_needs_rotate_right(s1) { spec_rotate_right(s1) } else { s1 };
        let s3 = if spec_needs_flip(s2) { spec_flip_colors(s2) } else { s2 };
        assert(spec_fix_up(link) == spec_update_root(s3));
        reveal_with_fuel(spec_is_llrb_link, 2);
        reveal_with_fuel(link_black_height, 2);
        if spec_needs_rotate_left(link) {
            // Red right child x, black left child: rotate x up.
            let x = r->Some_0;
            lemma_rotate_left_preserves(link);
            let nh0 = Node { key: h.key, color: Color::Red, size: h.size, left: l, right: x.left };
            let nx0 = Node { key: x.key, color: h.color, size: x.size,
                             left: Some(Box::new(spec_updated(nh0))), right: x.right };
            lemma_updated_fields(nh0);
            lemma_updated_fields(nx0);
            let nh = Some(Box::new(spec_updated(nh0)));
            assert(s1 == Some(Box::new(spec_updated(nx0))));
            assert(spec_is_llrb_link(l));
            assert(spec_is_llrb_link(nh));
            assert(link_is_red(nh));
            assert(link_black_height(nh) == link_black_height(l));
            assert(!spec_needs_rotate_right(s1));
            assert(!spec_needs_flip(s1));
            assert(s3 == s1);
            reveal_with_fuel(spec_size_cached_link, 2);
            lemma_update_root_preserves(s3);
        } else if spec_needs_rotate_right(link) {
            // Red left child with a red left child, black parent: rotate right, then flip.
            let x = l->Some_0;
            lemma_rotate_right_preserves(link);
            let nh0 = Node { key: h.key, color: Color::Red, size: h.size, left: x.right, right: r };
            let nx0 = Node { key: x.key, color: h.color, size: x.size,
                             left: x.left, right: Some(Box::new(spec_updated(nh0))) };
            lemma_updated_fields(nh0);
            lemma_updated_fields(nx0);
            let nh = Some(Box::new(spec_updated(nh0)));
            assert(s1 == link);
            assert(s2 == Some(Box::new(spec_updated(nx0))));
            assert(!spec_is_llrb_link(l));
            assert(spec_is_llrb_relaxed_link(l));
            assert(spec_is_llrb_link(nh));
            assert(link_is_red(nh));
            assert(spec_needs_flip(s2));
            lemma_flip_colors_preserves(s2);
            lemma_toggle_root_preserves(x.left);
            lemma_toggle_root_preserves(nh);
            reveal_with_fuel(spec_size_cached_link, 2);
            assert(spec_is_llrb_link(s3));
            lemma_update_root_preserves(s3);
        } else if spec_needs_flip(link) {
            // Both children red below a black parent: flip.
            assert(s1 == link);
            assert(s2 == link);
            assert(spec_is_llrb_link(l));
            lemma_flip_colors_preserves(link);
            lemma_toggle_root_preserves(l);
            lemma_toggle_root_preserves(r);
            reveal_with_fuel(spec_size_cached_link, 2);
            assert(spec_is_llrb_link(s3));
            lemma_update_root_preserves(s3);
        } else {
            assert(s3 == link);
            assert(spec_is_llrb_link(l));
            lemma_update_root_preserves(s3);
        }
    }

    /// `fix_up` after delete: over two left-leaning red-black subtrees of equal black
    /// height (both black when the node is red), it yields a left-leaning red-black
    /// tree of the same black height, black whenever the node is black and its two
    /// children are not both red.
    proof fn lemma_rebalance<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            link is Some,
            spec_is_llrb_link(link->Some_0.left),
            spec_is_llrb_link(link->Some_0.right),
            link_black_height(link->Some_0.left) == link_black_height(link->Some_0.right),
            link->Some_0.color == Color::Red ==>
                !link_is_red(link->Some_0.left) && !link_is_red(link->Some_0.right),
            spec_is_bst_link(link),
            spec_size_cached_link(link->Some_0.left),
            spec_size_cached_link(link->Some_0.right),
            link_spec_size(link) <= usize::MAX,
        ensures
            forall|z: T| #[trigger] link_contains(spec_fix_up(link), z) == link_contains(link, z),
            link_spec_size(spec_fix_up(link)) == link_spec_size(link),
            spec_is_bst_link(spec_fix_up(link)),
            spec_size_cached_link(spec_fix_up(link)),
            spec_is_llrb_link(spec_fix_up(link)),
            link_black_height(spec_fix_up(link)) == link_black_height(link),
            (link->Some_0.color == Color::Black
                && !(link_is_red(link->Some_0.left) && link_is_red(link->Some_0.right)))
                ==> !link_is_red(spec_fix_up(link)),
    {
        let h = link->Some_0;
        reveal_with_fuel(spec_is_llrb_link, 2);
        assert(!spec_needs_rotate_right(link));
        assert(spec_fix_up_pre(link));
        lemma_fix_up(link);
        if h.color == Color::Red {
            assert(!spec_needs_rotate_left(link));
            assert(!spec_needs_flip(link));
            lemma_update_root_preserves(link);
            assert(spec_fix_up(link) == spec_update_root(link));
        } else if !(link_is_red(h.left) && link_is_red(h.right)) {
            if spec_needs_rotate_left(link) {
                let x = h.right->Some_0;
                let nh0 = Node { key: h.key, color: Color::Red, size: h.size, left: h.left, right: x.left };
                let nx0 = Node { key: x.key, color: h.color, size: x.size,
                                 left: Some(Box::new(spec_updated(nh0))), right: x.right };
                lemma_updated_fields(nh0);
                lemma_updated_fields(nx0);
                let s1 = spec_rotate_left(link);
                assert(s1 == Some(Box::new(spec_updated(nx0))));
                assert(!spec_needs_rotate_right(s1));
                assert(!spec_needs_flip(s1));
                lemma_updated_fields(*s1->Some_0);
            } else {
                assert(!spec_needs_flip(link));
                lemma_updated_fields(*h);
            }
        }
    }

    /// `move_red_left` when the right child's left child is red: the borrow case, in
    /// which a key moves from the right sibling to the left through the root.
    proof fn lemma_move_red_left_borrow<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            link is Some,
            link->Some_0.color == Color::Red,
            spec_is_llrb_link(link),
            link->Some_0.left is Some,
            !link_is_red(link->Some_0.left->Some_0.left),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
            link->Some_0.right is Some,
            link_is_red(link->Some_0.right->Some_0.left),
        ensures
            spec_move_red_left(link) is Some,
            forall|z: T| #[trigger] link_contains(spec_move_red_left(link), z) == link_contains(link, z),
            link_spec_size(spec_move_red_left(link)) == link_spec_size(link),
            spec_is_bst_link(spec_move_red_left(link)),
            spec_size_cached_link(spec_move_red_left(link)),
            link_black_height(spec_move_red_left(link)) == link_black_height(link),
            spec_delete_min_pre(spec_move_red_left(link)->Some_0.left),
            spec_is_llrb_link(spec_move_red_left(link)->Some_0.right),
            link_black_height(spec_move_red_left(link)->Some_0.left)
                == link_black_height(spec_move_red_left(link)->Some_0.right),
            spec_move_red_left(link)->Some_0.color == Color::Red ==>
                !link_is_red(spec_move_red_left(link)->Some_0.left)
                && !link_is_red(spec_move_red_left(link)->Some_0.right),
            forall|z: T| #[trigger] link_contains(link->Some_0.left, z) ==>
                link_contains(spec_move_red_left(link)->Some_0.left, z),
    {
        let h = link->Some_0;
        let l = h.left;
        let r = h.right;
        reveal_with_fuel(spec_is_llrb_link, 3);
        reveal_with_fuel(link_black_height, 3);
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(link_spec_size, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        lemma_toggle_root_preserves(l);
        lemma_toggle_root_preserves(r);
        lemma_flip_colors_preserves(link);
        let f = spec_flip_colors(link);
        let fh = f->Some_0;
        assert(fh.left == spec_toggle_root(l));
        assert(fh.right == spec_toggle_root(r));
        let rn0 = r->Some_0;
        if link_is_red(rn0.left) {
            // Borrow from the right sibling: rotate its red left child up to the root.
            let rr = fh.right;
            let rrn = rr->Some_0;
            assert(rrn.left == rn0.left);
            assert(spec_is_bst_link(f));
            assert(spec_is_bst_link(rr));
            lemma_rotate_right_preserves(rr);
            let rot = spec_rotate_right(rr);
            let g = Some(Box::new(Node { key: fh.key, color: fh.color, size: fh.size,
                                         left: fh.left, right: rot }));
            assert(spec_is_bst_link(g)) by {
                assert forall|z: T| #[trigger] link_contains(rot, z) implies
                    TotalOrder::le(fh.key, z) && z != fh.key by {
                    assert(link_contains(rr, z));
                };
            };
            assert forall|z: T| #[trigger] link_contains(g, z) == link_contains(f, z) by {
                assert(link_contains(rot, z) == link_contains(rr, z));
            };
            assert(link_spec_size(g) == link_spec_size(f));
            // Shape of the rotated right child.
            let x = rn0.left->Some_0;
            let nh0 = Node { key: rrn.key, color: Color::Red, size: rrn.size, left: x.right, right: rrn.right };
            let nx0 = Node { key: x.key, color: rrn.color, size: x.size,
                             left: x.left, right: Some(Box::new(spec_updated(nh0))) };
            lemma_updated_fields(nh0);
            lemma_updated_fields(nx0);
            assert(rot == Some(Box::new(spec_updated(nx0))));
            assert(spec_size_cached_link(rr));
            assert(spec_size_cached_link(rot));
            assert(spec_size_cached_link(g));
            lemma_rotate_left_preserves(g);
            let s = spec_rotate_left(g);
            let mh0 = Node { key: fh.key, color: Color::Red, size: fh.size, left: fh.left, right: x.left };
            let mx0 = Node { key: x.key, color: fh.color, size: spec_updated(nx0).size,
                             left: Some(Box::new(spec_updated(mh0))), right: Some(Box::new(spec_updated(nh0))) };
            lemma_updated_fields(mh0);
            lemma_updated_fields(mx0);
            assert(s == Some(Box::new(spec_updated(mx0))));
            lemma_flip_colors_preserves(s);
            let m = Some(Box::new(spec_updated(mh0)));
            let n = Some(Box::new(spec_updated(nh0)));
            lemma_toggle_root_preserves(m);
            lemma_toggle_root_preserves(n);
            assert(spec_move_red_left(link) == spec_flip_colors(s));
            let out = spec_flip_colors(s)->Some_0;
            assert(out.left == spec_toggle_root(m));
            assert(out.right == spec_toggle_root(n));
            assert(spec_is_llrb_link(fh.left));
            assert(spec_is_llrb_link(out.left));
            assert(spec_is_llrb_link(out.right));
        }
    }

    /// `move_red_left` on a red node whose left child is a black 2-node: the new left
    /// child is a red left-leaning red-black tree below a black root, or a black one
    /// with a red left child below a red root; the right subtree is a left-leaning
    /// red-black tree of equal black height, black when the root is red; the black
    /// height, keys, size, BST order, and size cache are unchanged.
    proof fn lemma_move_red_left<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            link is Some,
            link->Some_0.color == Color::Red,
            spec_is_llrb_link(link),
            link->Some_0.left is Some,
            !link_is_red(link->Some_0.left->Some_0.left),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            spec_move_red_left(link) is Some,
            forall|z: T| #[trigger] link_contains(spec_move_red_left(link), z) == link_contains(link, z),
            link_spec_size(spec_move_red_left(link)) == link_spec_size(link),
            spec_is_bst_link(spec_move_red_left(link)),
            spec_size_cached_link(spec_move_red_left(link)),
            link_black_height(spec_move_red_left(link)) == link_black_height(link),
            spec_delete_min_pre(spec_move_red_left(link)->Some_0.left),
            spec_is_llrb_link(spec_move_red_left(link)->Some_0.right),
            link_black_height(spec_move_red_left(link)->Some_0.left)
                == link_black_height(spec_move_red_left(link)->Some_0.right),
            spec_move_red_left(link)->Some_0.color == Color::Red ==>
                !link_is_red(spec_move_red_left(link)->Some_0.left)
                && !link_is_red(spec_move_red_left(link)->Some_0.right),
            forall|z: T| #[trigger] link_contains(link->Some_0.left, z) ==>
                link_contains(spec_move_red_left(link)->Some_0.left, z),
    {
        let h = link->Some_0;
        let l = h.left;
        let r = h.right;
        reveal_with_fuel(spec_is_llrb_link, 2);
        reveal_with_fuel(link_black_height, 2);
        // h is red, so both children are black; they have equal black height, and the
        // left child is a node, so the right child is a node too.
        assert(r is Some);
        if link_is_red(r->Some_0.left) {
            lemma_move_red_left_borrow(link);
        } else {
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_contains, 2);
            reveal_with_fuel(spec_is_bst_link, 2);
            lemma_toggle_root_preserves(l);
            lemma_toggle_root_preserves(r);
            lemma_flip_colors_preserves(link);
            let f = spec_flip_colors(link);
            let fh = f->Some_0;
            assert(fh.left == spec_toggle_root(l));
            assert(fh.right == spec_toggle_root(r));
            assert(spec_move_red_left(link) == f);
            assert(spec_is_llrb_link(fh.left));
            assert(spec_is_llrb_link(fh.right));
        }
    }

    /// `move_red_right` on a red node whose right child is a black 2-node: below a
    /// black root the new right child is a red left-leaning red-black tree; below a red
    /// root it is a right-leaning 3-node keyed as the old root; the left subtree is a
    /// left-leaning red-black tree of equal black height, black when the root is red;
    /// the black height, keys, size, BST order, and size cache are unchanged.
    proof fn lemma_move_red_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            link is Some,
            link->Some_0.color == Color::Red,
            spec_is_llrb_link(link),
            link->Some_0.right is Some,
            !link_is_red(link->Some_0.right->Some_0.left),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            spec_move_red_right(link) is Some,
            forall|z: T| #[trigger] link_contains(spec_move_red_right(link), z) == link_contains(link, z),
            link_spec_size(spec_move_red_right(link)) == link_spec_size(link),
            spec_is_bst_link(spec_move_red_right(link)),
            spec_size_cached_link(spec_move_red_right(link)),
            link_black_height(spec_move_red_right(link)) == link_black_height(link),
            spec_is_llrb_link(spec_move_red_right(link)->Some_0.left),
            link_black_height(spec_move_red_right(link)->Some_0.left)
                == link_black_height(spec_move_red_right(link)->Some_0.right),
            spec_move_red_right(link)->Some_0.color == Color::Black ==>
                spec_move_red_right(link)->Some_0.key == link->Some_0.key
                && spec_is_llrb_link(spec_move_red_right(link)->Some_0.right)
                && link_is_red(spec_move_red_right(link)->Some_0.right),
            spec_move_red_right(link)->Some_0.color == Color::Red ==>
                !link_is_red(spec_move_red_right(link)->Some_0.left)
                && spec_is_right_red_link(spec_move_red_right(link)->Some_0.right)
                && spec_move_red_right(link)->Some_0.right->Some_0.key == link->Some_0.key
                && TotalOrder::le(spec_move_red_right(link)->Some_0.key, link->Some_0.key)
                && spec_move_red_right(link)->Some_0.key != link->Some_0.key,
    {
        let h = link->Some_0;
        let l = h.left;
        let r = h.right;
        reveal_with_fuel(spec_is_llrb_link, 3);
        reveal_with_fuel(link_black_height, 3);
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(link_spec_size, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        assert(l is Some);
        lemma_toggle_root_preserves(l);
        lemma_toggle_root_preserves(r);
        lemma_flip_colors_preserves(link);
        let f = spec_flip_colors(link);
        let fh = f->Some_0;
        assert(fh.left == spec_toggle_root(l));
        assert(fh.right == spec_toggle_root(r));
        let ln0 = l->Some_0;
        if link_is_red(ln0.left) {
            // Borrow from the left sibling: rotate right, then flip.
            let fl = fh.left->Some_0;
            assert(fl.left == ln0.left);
            lemma_rotate_right_preserves(f);
            let s = spec_rotate_right(f);
            let nh0 = Node { key: fh.key, color: Color::Red, size: fh.size, left: fl.right, right: fh.right };
            let nx0 = Node { key: fl.key, color: fh.color, size: fl.size,
                             left: fl.left, right: Some(Box::new(spec_updated(nh0))) };
            lemma_updated_fields(nh0);
            lemma_updated_fields(nx0);
            assert(s == Some(Box::new(spec_updated(nx0))));
            lemma_flip_colors_preserves(s);
            let n = Some(Box::new(spec_updated(nh0)));
            lemma_toggle_root_preserves(fl.left);
            lemma_toggle_root_preserves(n);
            assert(spec_move_red_right(link) == spec_flip_colors(s));
            let out = spec_flip_colors(s)->Some_0;
            assert(out.left == spec_toggle_root(fl.left));
            assert(out.right == spec_toggle_root(n));
            assert(spec_is_llrb_link(out.left));
            assert(spec_is_right_red_link(out.right));
            assert(link_contains(l, ln0.key));
        } else {
            assert(spec_move_red_right(link) == f);
            assert(spec_is_llrb_link(fh.left));
            assert(spec_is_llrb_link(fh.right));
        }
    }

    /// Delete's left descent: after `spec_delete_borrow_left` the left child is ready
    /// for `delete_min_link` (and still holds every key it held), the right child is a
    /// left-leaning red-black tree, and `spec_delete_step` holds.
    proof fn lemma_delete_descend_left<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            spec_delete_min_pre(link),
            link->Some_0.left is Some,
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            ({
                let m = spec_delete_borrow_left(link);
                &&& spec_delete_step(link, m)
                &&& spec_delete_min_pre(m->Some_0.left)
                &&& spec_is_llrb_link(m->Some_0.right)
                &&& forall|z: T| #[trigger] link_contains(link->Some_0.left, z) ==>
                        link_contains(m->Some_0.left, z)
            }),
            ({
                let l = link->Some_0.left->Some_0;
                (!(l.color == Color::Red) && !link_is_red(l.left)) ==>
                    link->Some_0.color == Color::Red && spec_is_llrb_link(link)
            }),
    {
        let h = link->Some_0;
        let l = h.left->Some_0;
        reveal_with_fuel(spec_is_llrb_link, 3);
        reveal_with_fuel(link_black_height, 2);
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        if !(l.color == Color::Red) && !link_is_red(l.left) {
            lemma_move_red_left(link);
        } else {
            assert(spec_delete_borrow_left(link) == link);
            assert(spec_is_llrb_link(h.left));
        }
    }

    /// Right descent, class (b): a black root with a red left child. Rotating right
    /// gives a right-leaning 3-node whose key is below `key`.
    proof fn lemma_descend_right_rotate<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, key: T)
        requires
            spec_delete_pre(link, key),
            link_is_red(link->Some_0.left),
            TotalOrder::le(link->Some_0.key, key),
            link_contains(link, key),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            spec_rotate_right(link) is Some,
            link_is_red(spec_rotate_right(link)->Some_0.right),
            spec_delete_right_ready(link, spec_rotate_right(link), key),
    {
        let h = link->Some_0;
        let x = h.left->Some_0;
        reveal_with_fuel(spec_is_llrb_link, 3);
        reveal_with_fuel(link_black_height, 3);
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(link_spec_size, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        assert(h.color == Color::Black);
        lemma_rotate_right_preserves(link);
        let a = spec_rotate_right(link);
        let nh0 = Node { key: h.key, color: Color::Red, size: h.size, left: x.right, right: h.right };
        let nx0 = Node { key: x.key, color: h.color, size: x.size,
                         left: x.left, right: Some(Box::new(spec_updated(nh0))) };
        lemma_updated_fields(nh0);
        lemma_updated_fields(nx0);
        let n = Some(Box::new(spec_updated(nh0)));
        assert(a == Some(Box::new(spec_updated(nx0))));
        assert(spec_is_llrb_link(n));
        assert(link_contains(h.left, x.key));
        T::transitive(x.key, h.key, key);
        assert(x.key != key) by {
            if x.key == key { T::antisymmetric(h.key, key); }
        };
        assert(link_contains(a, key));
        assert(link_contains(n, key)) by {
            if link_contains(x.left, key) {
                T::antisymmetric(x.key, key);
            }
        };
        assert(spec_delete_min_pre(n));
    }

    /// Right descent, class (a) with a borrow: a red left-leaning red-black root whose
    /// right child is a black 2-node; `move_red_right` makes the right child ready.
    proof fn lemma_descend_right_move<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, key: T)
        requires
            link is Some,
            link->Some_0.color == Color::Red,
            spec_is_llrb_link(link),
            link->Some_0.right is Some,
            !link_is_red(link->Some_0.right->Some_0.left),
            TotalOrder::le(link->Some_0.key, key),
            link_contains(link, key),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            spec_delete_right_ready(link, spec_move_red_right(link), key),
    {
        let h = link->Some_0;
        lemma_move_red_right(link);
        let c = spec_move_red_right(link);
        let ch = c->Some_0;
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(spec_is_llrb_link, 2);
        assert(link_contains(c, key));
        if ch.color == Color::Red {
            T::transitive(ch.key, h.key, key);
            assert(ch.key != key) by {
                if ch.key == key { T::antisymmetric(ch.key, h.key); }
            };
            assert(!link_contains(ch.left, key)) by {
                if link_contains(ch.left, key) { T::antisymmetric(ch.key, key); }
            };
        } else if h.key != key {
            assert(!link_contains(ch.left, key)) by {
                if link_contains(ch.left, key) { T::antisymmetric(h.key, key); }
            };
        }
    }

    /// Right descent with no restructuring: class (c), or class (a) whose right child
    /// or its left child is red.
    proof fn lemma_descend_right_stay<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, key: T)
        requires
            spec_delete_pre(link, key),
            !link_is_red(link->Some_0.left),
            link->Some_0.right is Some,
            link_is_red(link->Some_0.right) || link_is_red(link->Some_0.right->Some_0.left),
            TotalOrder::le(link->Some_0.key, key),
            link_contains(link, key),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            spec_delete_right_ready(link, link, key),
    {
        let h = link->Some_0;
        reveal_with_fuel(spec_is_llrb_link, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        assert(spec_delete_min_pre(h.right));
        if h.key != key {
            assert(!link_contains(h.left, key)) by {
                if link_contains(h.left, key) { T::antisymmetric(h.key, key); }
            };
        }
    }

    /// Delete's right descent at a node whose key is at most `key`: after
    /// `spec_delete_lean_right` either the node is a red leaf holding `key`, or, after
    /// `spec_delete_borrow_right`, `spec_delete_right_ready` holds.
    proof fn lemma_delete_descend_right<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, key: T)
        requires
            spec_delete_pre(link, key),
            TotalOrder::le(link->Some_0.key, key),
            link_contains(link, key),
            spec_is_bst_link(link),
            spec_size_cached_link(link),
            link_spec_size(link) <= usize::MAX,
        ensures
            ({
                let a = spec_delete_lean_right(link);
                &&& a is Some
                &&& forall|z: T| #[trigger] link_contains(a, z) == link_contains(link, z)
                &&& link_spec_size(a) == link_spec_size(link)
                &&& spec_is_bst_link(a)
                &&& spec_size_cached_link(a)
                &&& (a->Some_0.right is None ==>
                        a == link && a->Some_0.key == key && a->Some_0.left is None
                        && a->Some_0.right is None && a->Some_0.color == Color::Red)
                &&& (a->Some_0.right is Some ==> {
                        let r = a->Some_0.right->Some_0;
                        &&& ((!(r.color == Color::Red) && !link_is_red(r.left)) ==>
                                a->Some_0.color == Color::Red && spec_is_llrb_link(a))
                        &&& spec_delete_right_ready(link, spec_delete_borrow_right(a), key)
                    })
            }),
    {
        let h = link->Some_0;
        let a = spec_delete_lean_right(link);
        if link_is_red(h.left) {
            lemma_descend_right_rotate(link, key);
            lemma_rotate_right_preserves(link);
            assert(a == spec_rotate_right(link));
            assert(spec_delete_borrow_right(a) == a);
        } else {
            assert(a == link);
            reveal_with_fuel(spec_is_llrb_link, 2);
            reveal_with_fuel(link_black_height, 3);
            reveal_with_fuel(link_contains, 2);
            if h.right is None {
                // Class (c) has a red right child, so this is class (a): a red root
                // whose children have black height 0 and so are both empty.
                assert(h.color == Color::Red);
                assert(h.left is None);
            } else {
                let r = h.right->Some_0;
                if !(r.color == Color::Red) && !link_is_red(r.left) {
                    assert(h.color == Color::Red);
                    lemma_descend_right_move(link, key);
                } else {
                    lemma_descend_right_stay(link, key);
                    assert(spec_delete_borrow_right(a) == a);
                }
            }
        }
    }

    /// Removing `removed` from the left subtree removes it from the tree and keeps the
    /// BST order.
    proof fn lemma_rejoin_left<T: StTInMtT + Ord + TotalOrder>(s1: Link<T>, s2: Link<T>, removed: T)
        requires
            s1 is Some,
            s2 is Some,
            s2->Some_0.key == s1->Some_0.key,
            s2->Some_0.right == s1->Some_0.right,
            spec_is_bst_link(s1),
            spec_is_bst_link(s2->Some_0.left),
            link_contains(s1->Some_0.left, removed),
            forall|z: T| #[trigger] link_contains(s2->Some_0.left, z) <==>
                (link_contains(s1->Some_0.left, z) && z != removed),
            link_spec_size(s2->Some_0.left) + 1 == link_spec_size(s1->Some_0.left),
        ensures
            spec_is_bst_link(s2),
            forall|z: T| #[trigger] link_contains(s2, z) <==> (link_contains(s1, z) && z != removed),
            link_spec_size(s2) + 1 == link_spec_size(s1),
    {
        let h1 = s1->Some_0;
        let h2 = s2->Some_0;
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(link_spec_size, 2);
        assert forall|z: T| #[trigger] link_contains(h2.left, z) implies
            TotalOrder::le(z, h2.key) && z != h2.key by {
            assert(link_contains(h1.left, z));
        };
        assert(!link_contains(h1.right, removed)) by {
            if link_contains(h1.right, removed) { T::antisymmetric(removed, h1.key); }
        };
        assert forall|z: T| #[trigger] link_contains(s2, z) <==> (link_contains(s1, z) && z != removed) by {
            if link_contains(h2.left, z) { assert(link_contains(h1.left, z)); }
            if link_contains(h1.left, z) && z != removed { assert(link_contains(h2.left, z)); }
        };
    }

    /// Removing `removed` from the right subtree removes it from the tree and keeps
    /// the BST order.
    proof fn lemma_rejoin_right<T: StTInMtT + Ord + TotalOrder>(s1: Link<T>, s2: Link<T>, removed: T)
        requires
            s1 is Some,
            s2 is Some,
            s2->Some_0.key == s1->Some_0.key,
            s2->Some_0.left == s1->Some_0.left,
            spec_is_bst_link(s1),
            spec_is_bst_link(s2->Some_0.right),
            link_contains(s1->Some_0.right, removed),
            forall|z: T| #[trigger] link_contains(s2->Some_0.right, z) <==>
                (link_contains(s1->Some_0.right, z) && z != removed),
            link_spec_size(s2->Some_0.right) + 1 == link_spec_size(s1->Some_0.right),
        ensures
            spec_is_bst_link(s2),
            forall|z: T| #[trigger] link_contains(s2, z) <==> (link_contains(s1, z) && z != removed),
            link_spec_size(s2) + 1 == link_spec_size(s1),
    {
        let h1 = s1->Some_0;
        let h2 = s2->Some_0;
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(link_spec_size, 2);
        assert forall|z: T| #[trigger] link_contains(h2.right, z) implies
            TotalOrder::le(h2.key, z) && z != h2.key by {
            assert(link_contains(h1.right, z));
        };
        assert(!link_contains(h1.left, removed)) by {
            if link_contains(h1.left, removed) { T::antisymmetric(removed, h1.key); }
        };
        assert forall|z: T| #[trigger] link_contains(s2, z) <==> (link_contains(s1, z) && z != removed) by {
            if link_contains(h2.right, z) { assert(link_contains(h1.right, z)); }
            if link_contains(h1.right, z) && z != removed { assert(link_contains(h2.right, z)); }
        };
    }

    /// Replacing the root key by the minimum `min` of the right subtree and removing
    /// `min` from that subtree removes the old root key and keeps the BST order.
    proof fn lemma_rejoin_successor<T: StTInMtT + Ord + TotalOrder>(s1: Link<T>, s2: Link<T>, min: T)
        requires
            s1 is Some,
            s2 is Some,
            s2->Some_0.key == min,
            s2->Some_0.left == s1->Some_0.left,
            spec_is_bst_link(s1),
            spec_is_bst_link(s2->Some_0.right),
            link_contains(s1->Some_0.right, min),
            forall|z: T| #[trigger] link_contains(s1->Some_0.right, z) ==> TotalOrder::le(min, z),
            forall|z: T| #[trigger] link_contains(s2->Some_0.right, z) <==>
                (link_contains(s1->Some_0.right, z) && z != min),
            link_spec_size(s2->Some_0.right) + 1 == link_spec_size(s1->Some_0.right),
        ensures
            spec_is_bst_link(s2),
            forall|z: T| #[trigger] link_contains(s2, z) <==>
                (link_contains(s1, z) && z != s1->Some_0.key),
            link_spec_size(s2) + 1 == link_spec_size(s1),
    {
        let h1 = s1->Some_0;
        let h2 = s2->Some_0;
        let k = h1.key;
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(link_spec_size, 2);
        assert(TotalOrder::le(k, min) && min != k);
        assert forall|z: T| #[trigger] link_contains(h2.left, z) implies
            TotalOrder::le(z, min) && z != min by {
            T::transitive(z, k, min);
            if z == min { T::antisymmetric(k, min); }
        };
        assert forall|z: T| #[trigger] link_contains(h2.right, z) implies
            TotalOrder::le(min, z) && z != min by {
            assert(link_contains(h1.right, z));
        };
        assert forall|z: T| #[trigger] link_contains(s2, z) <==> (link_contains(s1, z) && z != k) by {
            if link_contains(h1.left, z) {
                assert(z != k);
            }
            if link_contains(h2.right, z) {
                assert(link_contains(h1.right, z));
            }
            if link_contains(h1.right, z) && z != min {
                assert(link_contains(h2.right, z));
            }
        };
    }

    /// After delete recurses into one child of `s1` and rejoins it as `s2`, `fix_up`
    /// yields a left-leaning red-black tree holding the keys of `before` less `removed`,
    /// with the black height of `before`, and black when `before` is black.
    proof fn lemma_delete_rejoin<T: StTInMtT + Ord + TotalOrder>(
        before: Link<T>, s1: Link<T>, s2: Link<T>, removed: T)
        requires
            spec_delete_step(before, s1),
            s2 is Some,
            s2->Some_0.color == s1->Some_0.color,
            spec_is_bst_link(s2),
            spec_size_cached_link(s2->Some_0.left),
            spec_size_cached_link(s2->Some_0.right),
            spec_is_llrb_link(s2->Some_0.left),
            spec_is_llrb_link(s2->Some_0.right),
            link_black_height(s2->Some_0.left) == link_black_height(s1->Some_0.left),
            link_black_height(s2->Some_0.right) == link_black_height(s1->Some_0.right),
            !link_is_red(s1->Some_0.left) ==> !link_is_red(s2->Some_0.left),
            !link_is_red(s1->Some_0.right) ==> !link_is_red(s2->Some_0.right),
            forall|z: T| #[trigger] link_contains(s2, z) <==> (link_contains(s1, z) && z != removed),
            link_spec_size(s2) + 1 == link_spec_size(s1),
            link_spec_size(s1) <= usize::MAX,
        ensures
            spec_is_bst_link(spec_fix_up(s2)),
            spec_size_cached_link(spec_fix_up(s2)),
            spec_is_llrb_link(spec_fix_up(s2)),
            link_black_height(spec_fix_up(s2)) == link_black_height(before),
            !link_is_red(before) ==> !link_is_red(spec_fix_up(s2)),
            forall|z: T| #[trigger] link_contains(spec_fix_up(s2), z) <==>
                (link_contains(before, z) && z != removed),
            link_spec_size(spec_fix_up(s2)) + 1 == link_spec_size(before),
    {
        reveal_with_fuel(link_black_height, 2);
        lemma_rebalance(s2);
        assert forall|z: T| #[trigger] link_contains(spec_fix_up(s2), z) <==>
            (link_contains(before, z) && z != removed) by {
            assert(link_contains(s1, z) == link_contains(before, z));
            assert(link_contains(spec_fix_up(s2), z) == link_contains(s2, z));
        };
    }

    /// Delete after recursing into the left child: rejoining and `fix_up` give a
    /// left-leaning red-black tree holding the keys of `s0` less `removed`.
    proof fn lemma_delete_tail_left<T: StTInMtT + Ord + TotalOrder>(
        s0: Link<T>, s1: Link<T>, s2: Link<T>, removed: T)
        requires
            spec_delete_step(s0, s1),
            spec_is_llrb_link(s1->Some_0.right),
            s2 is Some,
            s2->Some_0.key == s1->Some_0.key,
            s2->Some_0.right == s1->Some_0.right,
            s2->Some_0.color == s1->Some_0.color,
            spec_is_bst_link(s2->Some_0.left),
            spec_size_cached_link(s2->Some_0.left),
            spec_is_llrb_link(s2->Some_0.left),
            link_black_height(s2->Some_0.left) == link_black_height(s1->Some_0.left),
            !link_is_red(s1->Some_0.left) ==> !link_is_red(s2->Some_0.left),
            link_contains(s1->Some_0.left, removed),
            forall|z: T| #[trigger] link_contains(s2->Some_0.left, z) <==>
                (link_contains(s1->Some_0.left, z) && z != removed),
            link_spec_size(s2->Some_0.left) + 1 == link_spec_size(s1->Some_0.left),
            link_spec_size(s0) <= usize::MAX,
        ensures
            spec_is_bst_link(s2),
            spec_is_bst_link(spec_fix_up(s2)),
            spec_size_cached_link(spec_fix_up(s2)),
            spec_is_llrb_link(spec_fix_up(s2)),
            link_black_height(spec_fix_up(s2)) == link_black_height(s0),
            !link_is_red(s0) ==> !link_is_red(spec_fix_up(s2)),
            forall|z: T| #[trigger] link_contains(spec_fix_up(s2), z) <==>
                (link_contains(s0, z) && z != removed),
            link_spec_size(spec_fix_up(s2)) + 1 == link_spec_size(s0),
    {
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        lemma_rejoin_left(s1, s2, removed);
        lemma_delete_rejoin(s0, s1, s2, removed);
    }

    /// Delete after recursing into the right child: rejoining and `fix_up` give a
    /// left-leaning red-black tree holding the keys of `s0` less `removed`.
    proof fn lemma_delete_tail_right<T: StTInMtT + Ord + TotalOrder>(
        s0: Link<T>, s1: Link<T>, s2: Link<T>, removed: T)
        requires
            spec_delete_step(s0, s1),
            spec_is_llrb_link(s1->Some_0.left),
            s2 is Some,
            s2->Some_0.key == s1->Some_0.key,
            s2->Some_0.left == s1->Some_0.left,
            s2->Some_0.color == s1->Some_0.color,
            spec_is_bst_link(s2->Some_0.right),
            spec_size_cached_link(s2->Some_0.right),
            spec_is_llrb_link(s2->Some_0.right),
            link_black_height(s2->Some_0.right) == link_black_height(s1->Some_0.right),
            !link_is_red(s1->Some_0.right) ==> !link_is_red(s2->Some_0.right),
            link_contains(s1->Some_0.right, removed),
            forall|z: T| #[trigger] link_contains(s2->Some_0.right, z) <==>
                (link_contains(s1->Some_0.right, z) && z != removed),
            link_spec_size(s2->Some_0.right) + 1 == link_spec_size(s1->Some_0.right),
            link_spec_size(s0) <= usize::MAX,
        ensures
            spec_is_bst_link(s2),
            spec_is_bst_link(spec_fix_up(s2)),
            spec_size_cached_link(spec_fix_up(s2)),
            spec_is_llrb_link(spec_fix_up(s2)),
            link_black_height(spec_fix_up(s2)) == link_black_height(s0),
            !link_is_red(s0) ==> !link_is_red(spec_fix_up(s2)),
            forall|z: T| #[trigger] link_contains(spec_fix_up(s2), z) <==>
                (link_contains(s0, z) && z != removed),
            link_spec_size(spec_fix_up(s2)) + 1 == link_spec_size(s0),
    {
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        lemma_rejoin_right(s1, s2, removed);
        lemma_delete_rejoin(s0, s1, s2, removed);
    }

    /// Delete of the node key after removing its successor `min` from the right child:
    /// rejoining and `fix_up` give a left-leaning red-black tree holding the keys of
    /// `s0` less the old node key.
    proof fn lemma_delete_tail_successor<T: StTInMtT + Ord + TotalOrder>(
        s0: Link<T>, s1: Link<T>, s2: Link<T>, min: T)
        requires
            spec_delete_step(s0, s1),
            spec_is_llrb_link(s1->Some_0.left),
            s2 is Some,
            s2->Some_0.key == min,
            s2->Some_0.left == s1->Some_0.left,
            s2->Some_0.color == s1->Some_0.color,
            spec_is_bst_link(s2->Some_0.right),
            spec_size_cached_link(s2->Some_0.right),
            spec_is_llrb_link(s2->Some_0.right),
            link_black_height(s2->Some_0.right) == link_black_height(s1->Some_0.right),
            !link_is_red(s1->Some_0.right) ==> !link_is_red(s2->Some_0.right),
            link_contains(s1->Some_0.right, min),
            forall|z: T| #[trigger] link_contains(s1->Some_0.right, z) ==> TotalOrder::le(min, z),
            forall|z: T| #[trigger] link_contains(s2->Some_0.right, z) <==>
                (link_contains(s1->Some_0.right, z) && z != min),
            link_spec_size(s2->Some_0.right) + 1 == link_spec_size(s1->Some_0.right),
            link_spec_size(s0) <= usize::MAX,
        ensures
            spec_is_bst_link(s2),
            spec_is_bst_link(spec_fix_up(s2)),
            spec_size_cached_link(spec_fix_up(s2)),
            spec_is_llrb_link(spec_fix_up(s2)),
            link_black_height(spec_fix_up(s2)) == link_black_height(s0),
            !link_is_red(s0) ==> !link_is_red(spec_fix_up(s2)),
            forall|z: T| #[trigger] link_contains(spec_fix_up(s2), z) <==>
                (link_contains(s0, z) && z != s1->Some_0.key),
            link_spec_size(spec_fix_up(s2)) + 1 == link_spec_size(s0),
    {
        reveal_with_fuel(spec_size_cached_link, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        lemma_rejoin_successor(s1, s2, min);
        lemma_delete_rejoin(s0, s1, s2, s1->Some_0.key);
    }

    /// The minimum of the left child is the minimum of the tree.
    proof fn lemma_left_min_is_min<T: StTInMtT + Ord + TotalOrder>(link: Link<T>, min: T)
        requires
            link is Some,
            spec_is_bst_link(link),
            link_contains(link->Some_0.left, min),
            forall|z: T| #[trigger] link_contains(link->Some_0.left, z) ==> TotalOrder::le(min, z),
        ensures
            link_contains(link, min),
            forall|z: T| #[trigger] link_contains(link, z) ==> TotalOrder::le(min, z),
    {
        let h = link->Some_0;
        reveal_with_fuel(link_contains, 2);
        reveal_with_fuel(spec_is_bst_link, 2);
        assert(TotalOrder::le(min, h.key));
        assert forall|z: T| #[trigger] link_contains(link, z) implies TotalOrder::le(min, z) by {
            if link_contains(h.right, z) {
                T::transitive(min, h.key, z);
            } else if z == h.key {
            } else {
                assert(link_contains(h.left, z));
            }
        };
    }

    /// `fix_up` preserves the keys, the size, and the BST order of any BST.
    proof fn lemma_fix_up_preserves_bst<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires spec_is_bst_link(link),
        ensures
            forall|z: T| #[trigger] link_contains(spec_fix_up(link), z) == link_contains(link, z),
            link_spec_size(spec_fix_up(link)) == link_spec_size(link),
            spec_is_bst_link(spec_fix_up(link)),
    {
        let s1 = if spec_needs_rotate_left(link) { spec_rotate_left(link) } else { link };
        let s2 = if spec_needs_rotate_right(s1) { spec_rotate_right(s1) } else { s1 };
        let s3 = if spec_needs_flip(s2) { spec_flip_colors(s2) } else { s2 };
        if spec_needs_rotate_left(link) { lemma_rotate_left_preserves(link); }
        if spec_needs_rotate_right(s1) { lemma_rotate_right_preserves(s1); }
        if spec_needs_flip(s2) { lemma_flip_colors_preserves(s2); }
        if s3 is Some { lemma_update_root_preserves(s3); }
    }

    /// Splitting a sequence at `mid` partitions its elements into the prefix, the
    /// element at `mid`, and the suffix.
    proof fn lemma_split_contains<T>(s: Seq<T>, mid: int)
        requires 0 <= mid < s.len(),
        ensures
            forall|x: T| #[trigger] s.contains(x) <==>
                (s[mid] == x || s.subrange(0, mid).contains(x)
                    || s.subrange(mid + 1, s.len() as int).contains(x)),
    {
        let pre = s.subrange(0, mid);
        let suf = s.subrange(mid + 1, s.len() as int);
        assert forall|x: T| #[trigger] s.contains(x) implies
            (s[mid] == x || pre.contains(x) || suf.contains(x)) by {
            let i = choose|i: int| 0 <= i < s.len() && s[i] == x;
            if i < mid {
                assert(pre[i] == x);
            } else if i > mid {
                assert(suf[i - mid - 1] == x);
            }
        };
        assert forall|x: T| (s[mid] == x || pre.contains(x) || suf.contains(x))
            implies #[trigger] s.contains(x) by {
            if s[mid] == x {
                assert(s[mid] == x);
            } else if pre.contains(x) {
                let i = choose|i: int| 0 <= i < pre.len() && pre[i] == x;
                assert(s[i] == x);
            } else {
                let i = choose|i: int| 0 <= i < suf.len() && suf[i] == x;
                assert(s[mid + 1 + i] == x);
            }
        };
    }

    /// In a strictly sorted sequence, the prefix before `mid` is below `s[mid]`, the
    /// suffix after it is above, and both are strictly sorted.
    proof fn lemma_sorted_split<T: StTInMtT + Ord + TotalOrder>(s: Seq<T>, mid: int)
        requires
            spec_sorted_strict(s),
            0 <= mid < s.len(),
        ensures
            spec_sorted_strict(s.subrange(0, mid)),
            spec_sorted_strict(s.subrange(mid + 1, s.len() as int)),
            forall|x: T| #[trigger] s.subrange(0, mid).contains(x) ==>
                TotalOrder::le(x, s[mid]) && x != s[mid],
            forall|x: T| #[trigger] s.subrange(mid + 1, s.len() as int).contains(x) ==>
                TotalOrder::le(s[mid], x) && x != s[mid],
    {
        let pre = s.subrange(0, mid);
        let suf = s.subrange(mid + 1, s.len() as int);
        assert forall|i: int, j: int| 0 <= i < j < pre.len() implies
            TotalOrder::le(#[trigger] pre[i], #[trigger] pre[j]) && pre[i] != pre[j] by {
            assert(pre[i] == s[i] && pre[j] == s[j]);
        };
        assert forall|i: int, j: int| 0 <= i < j < suf.len() implies
            TotalOrder::le(#[trigger] suf[i], #[trigger] suf[j]) && suf[i] != suf[j] by {
            assert(suf[i] == s[mid + 1 + i] && suf[j] == s[mid + 1 + j]);
        };
        assert forall|x: T| #[trigger] pre.contains(x) implies
            TotalOrder::le(x, s[mid]) && x != s[mid] by {
            let i = choose|i: int| 0 <= i < pre.len() && pre[i] == x;
            assert(s[i] == x);
        };
        assert forall|x: T| #[trigger] suf.contains(x) implies
            TotalOrder::le(s[mid], x) && x != s[mid] by {
            let i = choose|i: int| 0 <= i < suf.len() && suf[i] == x;
            assert(s[mid + 1 + i] == x);
        };
    }

    /// A black or red node built from a split of a strictly sorted sequence, over
    /// subtrees holding exactly the prefix and the suffix, is a BST holding exactly
    /// the sequence.
    proof fn lemma_node_from_sorted<T: StTInMtT + Ord + TotalOrder>(
        s: Seq<T>, mid: int, node: Node<T>,
    )
        requires
            spec_sorted_strict(s),
            0 <= mid < s.len(),
            node.key == s[mid],
            spec_is_bst_link(node.left),
            spec_is_bst_link(node.right),
            forall|x: T| #[trigger] link_contains(node.left, x) <==> s.subrange(0, mid).contains(x),
            forall|x: T| #[trigger] link_contains(node.right, x)
                <==> s.subrange(mid + 1, s.len() as int).contains(x),
        ensures
            spec_is_bst_link(Some(Box::new(node))),
            forall|x: T| #[trigger] link_contains(Some(Box::new(node)), x) <==> s.contains(x),
    {
        lemma_split_contains(s, mid);
        lemma_sorted_split(s, mid);
        reveal_with_fuel(spec_is_bst_link, 2);
        reveal_with_fuel(link_contains, 2);
        assert forall|x: T| #[trigger] link_contains(Some(Box::new(node)), x) <==> s.contains(x) by {
            assert(link_contains(Some(Box::new(node)), x) == (node.key == x
                || link_contains(node.left, x) || link_contains(node.right, x)));
            assert(link_contains(node.left, x) <==> s.subrange(0, mid).contains(x));
            assert(link_contains(node.right, x) <==> s.subrange(mid + 1, s.len() as int).contains(x));
            assert(s.contains(x) <==> (s[mid] == x || s.subrange(0, mid).contains(x)
                || s.subrange(mid + 1, s.len() as int).contains(x)));
        };
    }

    /// A left-leaning red-black tree of black height b has at least 2^b - 1 keys.
    pub proof fn lemma_llrb_size_lower_bound<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires spec_is_llrb_link(link),
        ensures link_spec_size(link) + 1 >= pow2(link_black_height(link)),
        decreases link,
    {
        match link {
            None => {
                assert(pow2(0) == 1) by { lemma2_to64(); };
            },
            Some(n) => {
                lemma_llrb_size_lower_bound(n.left);
                lemma_llrb_size_lower_bound(n.right);
                if n.color == Color::Black {
                    lemma_pow2_unfold(link_black_height(link));
                }
            },
        }
    }

    /// A left-leaning red-black tree of black height b has height at most 2b, plus
    /// one if its root is red.
    pub proof fn lemma_llrb_height_upper_bound<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires spec_is_llrb_link(link),
        ensures
            link_height(link) <= 2 * link_black_height(link)
                + if link_is_red(link) { 1nat } else { 0nat },
        decreases link,
    {
        match link {
            None => {},
            Some(n) => {
                lemma_llrb_height_upper_bound(n.left);
                lemma_llrb_height_upper_bound(n.right);
            },
        }
    }

    /// Near balance (APAS Definition 37.6): a left-leaning red-black tree with a black
    /// root, n keys and height h satisfies 2^h <= (n + 1)^2, that is h <= 2 lg(n + 1).
    pub proof fn lemma_llrb_height_bound<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        requires
            spec_is_llrb_link(link),
            !link_is_red(link),
        ensures
            pow2(link_height(link)) <= (link_spec_size(link) + 1) * (link_spec_size(link) + 1),
    {
        let b = link_black_height(link);
        let h = link_height(link);
        let s = link_spec_size(link) + 1;
        lemma_llrb_size_lower_bound(link);
        lemma_llrb_height_upper_bound(link);
        lemma_pow2_adds(b, b);
        if h < 2 * b {
            lemma_pow2_strictly_increases(h, 2 * b);
        }
        lemma_mul_upper_bound(pow2(b) as int, s as int, pow2(b) as int, s as int);
    }

    /// Height is bounded by structural node count.
    proof fn lemma_height_le_size<T: StTInMtT + Ord + TotalOrder>(link: Link<T>)
        ensures link_height(link) <= link_spec_size(link),
        decreases link,
    {
        match link {
            None => {},
            Some(node) => {
                lemma_height_le_size::<T>(node.left);
                lemma_height_le_size::<T>(node.right);
            }
        }
    }

    //		Section 8b. traits


    /// Trait for RB tree node/link operations (Layer 1).
    pub trait BSTRBMtNodeFns<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bst(self) -> bool;
        spec fn spec_size(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;
        spec fn spec_height(self) -> nat;
        spec fn spec_is_empty(self) -> bool;
        spec fn spec_red(self) -> bool;
        spec fn spec_size_field(self) -> usize;
        spec fn spec_llrb(self) -> bool;
        spec fn spec_llrb_relaxed(self) -> bool;
        spec fn spec_black_height(self) -> nat;
        spec fn spec_size_cached(self) -> bool;
        spec fn spec_rotated_left(self) -> Self;
        spec fn spec_rotated_right(self) -> Self;
        spec fn spec_flipped(self) -> Self;
        spec fn spec_fixed_up(self) -> Self;
        spec fn spec_delete_min_ready(self) -> bool;
        spec fn spec_delete_ready(self, key: T) -> bool;

        // veracity: no_requires
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_red(&self) -> (red: bool)
            ensures
                self.spec_is_empty() ==> !red,
                red == self.spec_red();
        // veracity: no_requires
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size_link(&self) -> (size: usize)
            ensures
                self.spec_is_empty() ==> size == 0,
                size == self.spec_size_field();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size(),
                *self == old(self).spec_rotated_left();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size(),
                *self == old(self).spec_rotated_right();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn flip_colors(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size(),
                *self == old(self).spec_flipped();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn fix_up(&mut self)
            requires old(self).spec_bst(),
            ensures
                self.spec_bst(),
                forall|z: T| self.spec_contains(z) <==> old(self).spec_contains(z),
                self.spec_size() == old(self).spec_size(),
                *self == old(self).spec_fixed_up();
        /// Inserts `value` and rebalances on the way back up (left-leaning red-black
        /// insert). Below a black parent the result is a left-leaning red-black tree
        /// whose root may be red; below a red parent it is red and relaxed.
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn insert_link(&mut self, value: T)
            requires
                old(self).spec_bst(),
                old(self).spec_llrb(),
                old(self).spec_size_cached(),
                old(self).spec_size() < usize::MAX,
            ensures
                self.spec_bst(),
                self.spec_contains(value),
                forall|x: T| old(self).spec_contains(x) ==> self.spec_contains(x),
                forall|x: T| self.spec_contains(x) ==> (old(self).spec_contains(x) || x == value),
                self.spec_size() <= old(self).spec_size() + 1,
                self.spec_size_cached(),
                self.spec_black_height() == old(self).spec_black_height(),
                !old(self).spec_red() ==> self.spec_llrb(),
                old(self).spec_red() ==> self.spec_llrb_relaxed() && self.spec_red();
        /// Removes and returns the minimum key (left-leaning red-black delete-min).
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one leftmost path, O(1) per level.
        fn delete_min_link(&mut self) -> (min: T)
            requires
                old(self).spec_bst(),
                old(self).spec_size_cached(),
                old(self).spec_size() <= usize::MAX,
                old(self).spec_delete_min_ready(),
            ensures
                self.spec_bst(),
                self.spec_size_cached(),
                self.spec_llrb(),
                self.spec_black_height() == old(self).spec_black_height(),
                !old(self).spec_red() ==> !self.spec_red(),
                old(self).spec_contains(min),
                forall|x: T| #[trigger] old(self).spec_contains(x) ==> TotalOrder::le(min, x),
                forall|x: T| #[trigger] self.spec_contains(x) <==> (old(self).spec_contains(x) && x != min),
                self.spec_size() + 1 == old(self).spec_size(),
            decreases old(self).spec_size();
        /// Removes `key`, which the tree contains (left-leaning red-black delete).
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn delete_link(&mut self, key: &T)
            requires
                old(self).spec_bst(),
                old(self).spec_size_cached(),
                old(self).spec_size() <= usize::MAX,
                old(self).spec_contains(*key),
                old(self).spec_delete_ready(*key),
            ensures
                self.spec_bst(),
                self.spec_size_cached(),
                self.spec_llrb(),
                self.spec_black_height() == old(self).spec_black_height(),
                !old(self).spec_red() ==> !self.spec_red(),
                forall|x: T| #[trigger] self.spec_contains(x) <==> (old(self).spec_contains(x) && x != *key),
                self.spec_size() + 1 == old(self).spec_size(),
            decreases old(self).spec_size(), 2nat;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find_link(&self, target: &T) -> (found: Option<&T>)
            requires self.spec_bst(),
            ensures
                found.is_some() <==> self.spec_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn min_link(&self) -> (min: Option<&T>)
            requires self.spec_bst(),
            ensures
                !self.spec_is_empty() ==> min.is_some(),
                min.is_some() ==> self.spec_contains(*min.unwrap()),
                min.is_some() ==> forall|x: T| #[trigger] self.spec_contains(x) ==> TotalOrder::le(*min.unwrap(), x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn max_link(&self) -> (max: Option<&T>)
            requires self.spec_bst(),
            ensures
                !self.spec_is_empty() ==> max.is_some(),
                max.is_some() ==> self.spec_contains(*max.unwrap()),
                max.is_some() ==> forall|x: T| #[trigger] self.spec_contains(x) ==> TotalOrder::le(x, *max.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_collect(&self, out: &mut Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order_collect(&self, out: &mut Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order_parallel(&self) -> (elements: Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order_parallel(&self) -> (elements: Vec<T>)
            requires self.spec_size() <= usize::MAX as nat,
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_parallel<F>(&self, predicate: &Arc<F>) -> (filtered: Vec<T>)
            where
                F: Fn(&T) -> bool + Send + Sync,
            requires
                self.spec_size() <= usize::MAX as nat,
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce_parallel<F>(&self, op: &Arc<F>, identity: T) -> (reduced: T)
            where
                F: Fn(T, T) -> T + Send + Sync,
            requires
                self.spec_size() <= usize::MAX as nat,
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_rec(&self) -> (h: usize)
            requires self.spec_height() <= usize::MAX as nat,
            ensures h as nat == self.spec_height();
        /// Exec mirror of link_spec_size for runtime size guards.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn compute_link_spec_size(&self) -> (n: usize)
            requires self.spec_size() <= usize::MAX,
            ensures n as nat == self.spec_size();
    }

    //		Section 9b. impls


    // Free functions operating on Node<T> (not Link<T>).

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn new_node<T: StTInMtT + Ord + TotalOrder>(key: T) -> (node: Node<T>)
        requires link_spec_size::<T>(None) + 1 <= usize::MAX as nat,
        ensures
            node.key == key,
            node.color == Color::Red,
            node.size == 1,
            node.left is None,
            node.right is None,
    {
        Node {
            key,
            color: Color::Red,
            size: 1,
            left: None,
            right: None,
        }
    }

    // veracity: no_requires
    /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn update<T: StTInMtT + Ord + TotalOrder>(node: &mut Node<T>)
        ensures
            node.left == old(node).left,
            node.right == old(node).right,
            node.key == old(node).key,
            node.color == old(node).color,
            *node == spec_updated(*old(node)),
    {
        let ls = node.left.size_link();
        let rs = node.right.size_link();
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn toggle_color(c: Color) -> (toggled: Color)
        ensures toggled == spec_toggle(c),
    {
        match c {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }

    /// Before delete descends left from a red node whose left child is a black 2-node,
    /// makes the left child or its left child red (Sedgewick's moveRedLeft).
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    pub(crate) fn move_red_left_link<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>)
        requires
            *old(link) is Some,
            old(link)->Some_0.color == Color::Red,
            spec_is_llrb_link(*old(link)),
            old(link)->Some_0.left is Some,
            !link_is_red(old(link)->Some_0.left->Some_0.left),
            spec_is_bst_link(*old(link)),
            spec_size_cached_link(*old(link)),
            link_spec_size(*old(link)) <= usize::MAX,
        ensures *link == spec_move_red_left(*old(link)),
    {
        link.flip_colors();
        let ghost f = *link;
        let tmp = link.take();
        let rl_red = match &tmp {
            Some(n) => match &n.right {
                Some(r) => r.left.is_red(),
                None => false,
            },
            None => false,
        };
        if rl_red {
            match tmp {
                Some(mut n) => {
                    let ghost rr = n.right;
                    proof { reveal_with_fuel(spec_is_bst_link, 2); }
                    n.right.rotate_right();
                    *link = Some(n);
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        assert forall|z: T| #[trigger] link_contains(n.right, z) implies
                            TotalOrder::le(n.key, z) && z != n.key by {
                            assert(link_contains(rr, z));
                        };
                    }
                },
                None => {},
            }
            link.rotate_left();
            link.flip_colors();
        } else {
            *link = tmp;
        }
    }

    /// Before delete descends right from a red node whose right child is a black
    /// 2-node, makes the right child or its left child red (Sedgewick's moveRedRight).
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    pub(crate) fn move_red_right_link<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>)
        requires
            *old(link) is Some,
            old(link)->Some_0.color == Color::Red,
            spec_is_llrb_link(*old(link)),
            old(link)->Some_0.right is Some,
            !link_is_red(old(link)->Some_0.right->Some_0.left),
            spec_is_bst_link(*old(link)),
            spec_size_cached_link(*old(link)),
            link_spec_size(*old(link)) <= usize::MAX,
        ensures *link == spec_move_red_right(*old(link)),
    {
        link.flip_colors();
        let tmp = link.take();
        let ll_red = match &tmp {
            Some(n) => match &n.left {
                Some(l) => l.left.is_red(),
                None => false,
            },
            None => false,
        };
        *link = tmp;
        if ll_red {
            link.rotate_right();
            link.flip_colors();
        }
    }

    /// Delete of a key below the root key: descend left as `delete_min_link` does.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn delete_link_left<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, key: &T)
        requires
            old(link).spec_bst(),
            old(link).spec_size_cached(),
            old(link).spec_size() <= usize::MAX,
            old(link).spec_contains(*key),
            old(link).spec_delete_ready(*key),
            *old(link) is Some,
            TotalOrder::le(*key, old(link)->Some_0.key),
            *key != old(link)->Some_0.key,
        ensures
            link.spec_bst(),
            link.spec_size_cached(),
            link.spec_llrb(),
            link.spec_black_height() == old(link).spec_black_height(),
            !old(link).spec_red() ==> !link.spec_red(),
            forall|x: T| #[trigger] link.spec_contains(x) <==> (old(link).spec_contains(x) && x != *key),
            link.spec_size() + 1 == old(link).spec_size(),
        decreases old(link).spec_size(), 1nat,
    {
        let ghost s0 = *link;
        proof {
            let h = s0->Some_0;
            reveal_with_fuel(link_contains, 2);
            reveal_with_fuel(spec_is_bst_link, 2);
            if spec_is_right_red_link(s0) && TotalOrder::le(h.key, *key) {
                T::antisymmetric(h.key, *key);
            }
            assert(spec_delete_min_pre(s0));
            assert(!link_contains(h.right, *key)) by {
                if link_contains(h.right, *key) { T::antisymmetric(h.key, *key); }
            };
            assert(link_contains(h.left, *key));
            lemma_delete_descend_left(s0);
        }
        let needs_move = match link {
            Some(node) => match &node.left {
                Some(l) => !node.left.is_red() && !l.left.is_red(),
                None => false,
            },
            None => false,
        };
        if needs_move {
            move_red_left_link(link);
        }
        let ghost s1 = *link;
        proof { assert(s1 == spec_delete_borrow_left(s0)); }
        let cur = link.take();
        let mut node = match cur {
            Some(n) => n,
            None => { vstd::pervasive::unreached() },
        };
        let ghost old_left = node.left;
        proof {
            reveal_with_fuel(spec_is_bst_link, 2);
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_spec_size, 2);
        }
        node.left.delete_link(key);
        let ghost new_left = node.left;
        *link = Some(node);
        let ghost s2 = *link;
        proof {
            assert forall|z: T| #[trigger] link_contains(new_left, z) <==>
                (link_contains(old_left, z) && z != *key) by {
                assert(new_left.spec_contains(z) <==> (old_left.spec_contains(z) && z != *key));
            };
            lemma_delete_tail_left(s0, s1, s2, *key);
        }
        link.fix_up();
    }

    /// Delete below a node prepared by the right descent from `s0`: remove the node
    /// key through its successor, or delete `key` from the right child.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn delete_at_ready<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, key: &T, Ghost(s0): Ghost<Link<T>>)
        requires
            spec_delete_right_ready(s0, *old(link), *key),
            link_spec_size(s0) <= usize::MAX,
        ensures
            spec_is_bst_link(*link),
            spec_size_cached_link(*link),
            spec_is_llrb_link(*link),
            link_black_height(*link) == link_black_height(s0),
            !link_is_red(s0) ==> !link_is_red(*link),
            forall|x: T| #[trigger] link_contains(*link, x) <==> (link_contains(s0, x) && x != *key),
            link_spec_size(*link) + 1 == link_spec_size(s0),
        decreases link_spec_size(*old(link)), 0nat,
    {
        let ghost s1 = *link;
        let cur = link.take();
        let mut node = match cur {
            Some(n) => n,
            None => { vstd::pervasive::unreached() },
        };
        let ghost old_right = node.right;
        proof {
            reveal_with_fuel(spec_is_bst_link, 2);
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_spec_size, 2);
        }
        let eq = match TotalOrder::cmp(key, &node.key) {
            core::cmp::Ordering::Equal => true,
            _ => false,
        };
        if eq {
            // Replace the key by its successor, the minimum of the right subtree.
            let min = node.right.delete_min_link();
            let ghost new_right = node.right;
            node.key = min;
            *link = Some(node);
            let ghost s2 = *link;
            proof {
                assert(link_contains(old_right, min));
                assert forall|z: T| #[trigger] link_contains(old_right, z) implies TotalOrder::le(min, z) by {
                    assert(old_right.spec_contains(z));
                };
                assert forall|z: T| #[trigger] link_contains(new_right, z) <==>
                    (link_contains(old_right, z) && z != min) by {
                    assert(new_right.spec_contains(z) <==> (old_right.spec_contains(z) && z != min));
                };
                lemma_delete_tail_successor(s0, s1, s2, min);
            }
        } else {
            node.right.delete_link(key);
            let ghost new_right = node.right;
            *link = Some(node);
            let ghost s2 = *link;
            proof {
                assert forall|z: T| #[trigger] link_contains(new_right, z) <==>
                    (link_contains(old_right, z) && z != *key) by {
                    assert(new_right.spec_contains(z) <==> (old_right.spec_contains(z) && z != *key));
                };
                lemma_delete_tail_right(s0, s1, s2, *key);
            }
        }
        link.fix_up();
    }

    /// Delete of a key at or above the root key: lean right, borrow for the right
    /// child, then remove the root key through its successor or descend right.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn delete_link_right<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, key: &T)
        requires
            old(link).spec_bst(),
            old(link).spec_size_cached(),
            old(link).spec_size() <= usize::MAX,
            old(link).spec_contains(*key),
            old(link).spec_delete_ready(*key),
            *old(link) is Some,
            TotalOrder::le(old(link)->Some_0.key, *key),
        ensures
            link.spec_bst(),
            link.spec_size_cached(),
            link.spec_llrb(),
            link.spec_black_height() == old(link).spec_black_height(),
            !old(link).spec_red() ==> !link.spec_red(),
            forall|x: T| #[trigger] link.spec_contains(x) <==> (old(link).spec_contains(x) && x != *key),
            link.spec_size() + 1 == old(link).spec_size(),
        decreases old(link).spec_size(), 1nat,
    {
        let ghost s0 = *link;
        proof { lemma_delete_descend_right(s0, *key); }
        let left_red = match link {
            Some(node) => node.left.is_red(),
            None => false,
        };
        if left_red {
            link.rotate_right();
        }
        let ghost sa = *link;
        proof { assert(sa == spec_delete_lean_right(s0)); }
        let right_none = match link {
            Some(node) => node.right.is_none(),
            None => true,
        };
        if right_none {
            // A red leaf holding the key: remove it.
            *link = None;
            proof {
                reveal_with_fuel(link_contains, 2);
                reveal_with_fuel(link_spec_size, 2);
                reveal_with_fuel(link_black_height, 2);
            }
            return;
        }
        let needs_move = match link {
            Some(node) => match &node.right {
                Some(r) => !node.right.is_red() && !r.left.is_red(),
                None => false,
            },
            None => false,
        };
        if needs_move {
            move_red_right_link(link);
        }
        proof { assert(*link == spec_delete_borrow_right(sa)); }
        delete_at_ready(link, key, Ghost(s0));
    }

    // Verified RB tree algorithms (Layer 1) — trait impl on Link<T>.

    impl<T: StTInMtT + Ord + TotalOrder> BSTRBMtNodeFns<T> for Link<T> {

    open spec fn spec_bst(self) -> bool { spec_is_bst_link(self) }
    open spec fn spec_size(self) -> nat { link_spec_size(self) }
    open spec fn spec_contains(self, target: T) -> bool { link_contains(self, target) }
    open spec fn spec_height(self) -> nat { link_height(self) }
    open spec fn spec_is_empty(self) -> bool { self is None }
    open spec fn spec_red(self) -> bool { link_is_red(self) }
    open spec fn spec_size_field(self) -> usize { link_size_field(self) }
    open spec fn spec_llrb(self) -> bool { spec_is_llrb_link(self) }
    open spec fn spec_llrb_relaxed(self) -> bool { spec_is_llrb_relaxed_link(self) }
    open spec fn spec_black_height(self) -> nat { link_black_height(self) }
    open spec fn spec_size_cached(self) -> bool { spec_size_cached_link(self) }
    open spec fn spec_rotated_left(self) -> Self { spec_rotate_left(self) }
    open spec fn spec_rotated_right(self) -> Self { spec_rotate_right(self) }
    open spec fn spec_flipped(self) -> Self { spec_flip_colors(self) }
    open spec fn spec_fixed_up(self) -> Self { spec_fix_up(self) }
    open spec fn spec_delete_min_ready(self) -> bool { spec_delete_min_pre(self) }
    open spec fn spec_delete_ready(self, key: T) -> bool { spec_delete_pre(self, key) }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn is_red(&self) -> (red: bool)
    {
        match self {
            Some(node) => matches!(node.color, Color::Red),
            None => false,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn size_link(&self) -> (size: usize)
    {
        match self.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_left(&mut self)
    {
        let ghost old_link = *self;
        let cur = self.take();
        match cur {
            None => {},
            Some(mut h) => {
                let hr = h.right.take();
                match hr {
                    None => {
                        *self = Some(h);
                    },
                    Some(mut x) => {
                        let hc = h.color;
                        h.right = x.left.take();
                        h.color = Color::Red;
                        update(&mut h);
                        x.color = hc;
                        x.left = Some(h);
                        update(&mut x);
                        *self = Some(x);
                        proof { lemma_rotate_left_preserves(old_link); }
                    },
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn rotate_right(&mut self)
    {
        let ghost old_link = *self;
        let cur = self.take();
        match cur {
            None => {},
            Some(mut h) => {
                let hl = h.left.take();
                match hl {
                    None => {
                        *self = Some(h);
                    },
                    Some(mut x) => {
                        let hc = h.color;
                        h.left = x.right.take();
                        h.color = Color::Red;
                        update(&mut h);
                        x.color = hc;
                        x.right = Some(h);
                        update(&mut x);
                        *self = Some(x);
                        proof { lemma_rotate_right_preserves(old_link); }
                    },
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn flip_colors(&mut self)
    {
        let ghost old_link = *self;
        let cur = self.take();
        match cur {
            None => {},
            Some(mut node) => {
                node.color = toggle_color(node.color);
                if let Some(mut left) = node.left.take() {
                    left.color = toggle_color(left.color);
                    node.left = Some(left);
                }
                if let Some(mut right) = node.right.take() {
                    right.color = toggle_color(right.color);
                    node.right = Some(right);
                }
                *self = Some(node);
                proof { lemma_flip_colors_preserves(old_link); }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    fn fix_up(&mut self)
    {
        let ghost s0 = *self;

        let tmp = self.take();
        let rotate_left_needed = match &tmp {
            | Some(node) => node.right.is_red() && !node.left.is_red(),
            | None => false,
        };
        *self = tmp;
        if rotate_left_needed {
            self.rotate_left();
        }
        let ghost s1 = *self;

        let tmp = self.take();
        let rotate_right_needed = match &tmp {
            | Some(node) => {
                match &node.left {
                    | Some(left) => node.left.is_red() && left.left.is_red(),
                    | None => false,
                }
            }
            | None => false,
        };
        *self = tmp;
        if rotate_right_needed {
            self.rotate_right();
        }
        let ghost s2 = *self;

        let tmp = self.take();
        let flip_needed = match &tmp {
            | Some(node) => node.left.is_red() && node.right.is_red(),
            | None => false,
        };
        *self = tmp;
        if flip_needed {
            self.flip_colors();
        }
        let ghost s3 = *self;

        if let Some(mut node) = self.take() {
            update(&mut node);
            *self = Some(node);
        }
        proof {
            assert(s1 == if spec_needs_rotate_left(s0) { spec_rotate_left(s0) } else { s0 });
            assert(s2 == if spec_needs_rotate_right(s1) { spec_rotate_right(s1) } else { s1 });
            assert(s3 == if spec_needs_flip(s2) { spec_flip_colors(s2) } else { s2 });
            lemma_fix_up_preserves_bst(s0);
        }
    }

    /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn insert_link(&mut self, value: T)
        decreases *old(self),
    {
        let ghost old_link = *self;
        let cur = self.take();
        match cur {
            | None => {
                *self = Some(Box::new(new_node(value)));
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(link_contains, 2);
                    reveal_with_fuel(link_spec_size, 2);
                    reveal_with_fuel(spec_size_cached_link, 2);
                    reveal_with_fuel(spec_is_llrb_link, 2);
                    reveal_with_fuel(link_black_height, 2);
                }
                return;
            }
            | Some(mut node) => {
                let ghost old_left = node.left;
                let ghost old_right = node.right;
                let ghost node_key = node.key;
                proof {
                    reveal_with_fuel(spec_is_bst_link, 2);
                    reveal_with_fuel(spec_is_llrb_link, 2);
                    reveal_with_fuel(spec_size_cached_link, 2);
                    reveal_with_fuel(link_spec_size, 2);
                    reveal_with_fuel(link_black_height, 2);
                }
                match TotalOrder::cmp(&value, &node.key) {
                    core::cmp::Ordering::Less => {
                        node.left.insert_link(value);
                        update(&mut node);
                        *self = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            lemma_updated_fields(*old_link->Some_0);
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(node.left, x) implies
                                (TotalOrder::le(x, node.key) && x != node.key)
                            by {
                                if link_contains(old_left, x) {
                                } else {
                                    // Veracity: NEEDED assert
                                    assert(node.left.spec_contains(x) ==>
                                        (old_left.spec_contains(x) || x == value));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(old_link, x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(old_link, x) implies
                                link_contains(*self, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_left, x) {
                                    // Veracity: NEEDED assert
                                    assert(old_left.spec_contains(x) ==>
                                        node.left.spec_contains(x));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(*self, x) implies
                                (link_contains(old_link, x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                } else if link_contains(node.left, x) {
                                    // Veracity: NEEDED assert
                                    assert(node.left.spec_contains(x) ==>
                                        (old_left.spec_contains(x) || x == value));
                                    if link_contains(old_left, x) {
                                    }
                                }
                            };
                            reveal_with_fuel(spec_is_llrb_link, 2);
                            assert(spec_fix_up_pre(*self));
                        }
                    }
                    core::cmp::Ordering::Greater => {
                        node.right.insert_link(value);
                        update(&mut node);
                        *self = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            lemma_updated_fields(*old_link->Some_0);
                            // Veracity: NEEDED assert
                            assert forall|x: T| link_contains(node.right, x) implies
                                (TotalOrder::le(node.key, x) && x != node.key)
                            by {
                                if link_contains(old_right, x) {
                                } else {
                                    // Veracity: NEEDED assert
                                    assert(node.right.spec_contains(x) ==>
                                        (old_right.spec_contains(x) || x == value));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(old_link, x) implies
                                (node_key == x || link_contains(old_left, x) || link_contains(old_right, x))
                            by {
                                reveal_with_fuel(link_contains, 2);
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(old_link, x) implies
                                link_contains(*self, x)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node_key == x {
                                } else if link_contains(old_right, x) {
                                    // Veracity: NEEDED assert
                                    assert(old_right.spec_contains(x) ==>
                                        node.right.spec_contains(x));
                                }
                            };
                            // Veracity: NEEDED assert
                            assert forall|x: T| #[trigger] link_contains(*self, x) implies
                                (link_contains(old_link, x) || x == value)
                            by {
                                reveal_with_fuel(link_contains, 2);
                                if node.key == x {
                                } else if link_contains(node.right, x) {
                                    // Veracity: NEEDED assert
                                    assert(node.right.spec_contains(x) ==>
                                        (old_right.spec_contains(x) || x == value));
                                    if link_contains(old_right, x) {
                                    }
                                }
                            };
                            reveal_with_fuel(spec_is_llrb_link, 2);
                            assert(spec_fix_up_pre(*self));
                        }
                    }
                    core::cmp::Ordering::Equal => {
                        *self = Some(node);
                        proof {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                        }
                        return;
                    }
                }
            }
        }
        let ghost s0 = *self;
        self.fix_up();
        proof {
            lemma_fix_up(s0);
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one leftmost path, O(1) per level.
    fn delete_min_link(&mut self) -> (min: T)
        decreases old(self).spec_size(),
    {
        let ghost s0 = *self;
        let left_is_none = match self {
            Some(node) => node.left.is_none(),
            None => true,
        };
        if left_is_none {
            // A red leaf: remove it.
            let cur = self.take();
            let node = match cur {
                Some(n) => n,
                None => { vstd::pervasive::unreached() },
            };
            let n = *node;
            proof {
                reveal_with_fuel(spec_is_llrb_link, 2);
                reveal_with_fuel(link_black_height, 2);
                reveal_with_fuel(link_spec_size, 2);
                reveal_with_fuel(link_contains, 2);
                T::reflexive(n.key);
                assert(n.right is None);
            }
            return n.key;
        }
        let needs_move = match self {
            Some(node) => match &node.left {
                Some(l) => !node.left.is_red() && !l.left.is_red(),
                None => false,
            },
            None => false,
        };
        proof { lemma_delete_descend_left(s0); }
        if needs_move {
            move_red_left_link(self);
        }
        let ghost s1 = *self;
        proof { assert(s1 == spec_delete_borrow_left(s0)); }
        let cur = self.take();
        let mut node = match cur {
            Some(n) => n,
            None => { vstd::pervasive::unreached() },
        };
        let ghost old_left = node.left;
        proof {
            reveal_with_fuel(spec_is_bst_link, 2);
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_spec_size, 2);
        }
        let min = node.left.delete_min_link();
        let ghost new_left = node.left;
        *self = Some(node);
        let ghost s2 = *self;
        proof {
            assert(link_contains(old_left, min));
            assert forall|z: T| #[trigger] link_contains(old_left, z) implies TotalOrder::le(min, z) by {
                assert(old_left.spec_contains(z));
            };
            assert forall|z: T| #[trigger] link_contains(new_left, z) <==>
                (link_contains(old_left, z) && z != min) by {
                assert(new_left.spec_contains(z) <==> (old_left.spec_contains(z) && z != min));
            };
            lemma_delete_tail_left(s0, s1, s2, min);
            lemma_left_min_is_min(s1, min);
            assert forall|z: T| #[trigger] link_contains(s0, z) implies TotalOrder::le(min, z) by {
                assert(link_contains(s1, z));
            };
        }
        self.fix_up();
        min
    }

    /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn delete_link(&mut self, key: &T)
        decreases old(self).spec_size(), 2nat,
    {
        let go_left = match self {
            Some(node) => match TotalOrder::cmp(key, &node.key) {
                core::cmp::Ordering::Less => true,
                _ => false,
            },
            None => { vstd::pervasive::unreached() },
        };
        if go_left {
            delete_link_left(self, key);
        } else {
            proof { T::total(self->Some_0.key, *key); }
            delete_link_right(self, key);
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn find_link(&self, target: &T) -> (found: Option<&T>)
        decreases *self,
    {
        match self {
            // Veracity: NEEDED proof block
            | None => None,
            | Some(node) => {
                match TotalOrder::cmp(target, &node.key) {
                    core::cmp::Ordering::Equal => Some(&node.key),
                    core::cmp::Ordering::Less => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!link_contains(node.right, *target)) by {
                                if link_contains(node.right, *target) {
                                    T::antisymmetric(*target, node.key);
                                }
                            };
                        }
                        node.left.find_link(target)
                    }
                    // Veracity: NEEDED proof block
                    core::cmp::Ordering::Greater => {
                        proof {
                            // Veracity: NEEDED assert
                            assert(!link_contains(node.left, *target)) by {
                                if link_contains(node.left, *target) {
                                    T::antisymmetric(node.key, *target);
                                }
                            };
                        }
                        node.right.find_link(target)
                    }
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn min_link(&self) -> (min: Option<&T>)
        decreases *self,
    {
        match self {
            | None => None,
            | Some(node) => match node.left {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(node.key, x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                // Veracity: NEEDED proof block
                                T::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.right, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let min = node.left.min_link();
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        // Bridge: trait ensures uses spec_contains → link_contains.
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(*min.unwrap(), x) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.left, x) {
                                // Veracity: NEEDED assert
                                assert(node.left.spec_contains(x));
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.right, x));
                                T::transitive(*min.unwrap(), node.key, x);
                            }
                        };
                    }
                    min
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
    fn max_link(&self) -> (max: Option<&T>)
        decreases *self,
    {
        match self {
            | None => None,
            | Some(node) => match node.right {
                | None => {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(x, node.key) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if x == node.key {
                                T::reflexive(x);
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.left, x));
                            }
                        };
                    }
                    Some(&node.key)
                }
                | Some(_) => {
                    let max = node.right.max_link();
                    proof {
                        reveal_with_fuel(spec_is_bst_link, 2);
                        reveal_with_fuel(link_contains, 2);
                        // Bridge: trait ensures uses spec_contains → link_contains.
                        // Veracity: NEEDED assert
                        assert forall|x: T| #[trigger] link_contains(*self, x) implies TotalOrder::le(x, *max.unwrap()) by {
                            reveal_with_fuel(spec_is_bst_link, 2);
                            reveal_with_fuel(link_contains, 2);
                            if link_contains(node.right, x) {
                                // Veracity: NEEDED assert
                                assert(node.right.spec_contains(x));
                            } else if x == node.key {
                            } else {
                                // Veracity: NEEDED assert
                                assert(link_contains(node.left, x));
                                T::transitive(x, node.key, *max.unwrap());
                            }
                        };
                    }
                    max
                }
            },
        }
    // Veracity: NEEDED proof block
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(node) = self {
            node.left.in_order_collect(out);
            out.push(node.key.clone());
            node.right.in_order_collect(out);
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn pre_order_collect(&self, out: &mut Vec<T>)
        decreases *self,
    {
        if let Some(node) = self {
            out.push(node.key.clone());
            node.left.pre_order_collect(out);
            node.right.pre_order_collect(out);
        }
    }

    // Veracity: NEEDED proof block
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn in_order_parallel(&self) -> (elements: Vec<T>)
    {
        let mut out = Vec::new();
        self.in_order_collect(&mut out);
        out
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn pre_order_parallel(&self) -> (elements: Vec<T>)
    {
        let mut out = Vec::new();
        self.pre_order_collect(&mut out);
        out
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn filter_parallel<F>(&self, predicate: &Arc<F>) -> (filtered: Vec<T>)
        where
            // Veracity: NEEDED proof block
            F: Fn(&T) -> bool + Send + Sync,
        decreases *self,
    {
        match self {
            | None => Vec::new(),
            | Some(node) => {
                proof {
                    reveal_with_fuel(link_spec_size, 2);
                }
                let left_vals = node.left.filter_parallel(predicate);
                let mut right_vals = node.right.filter_parallel(predicate);
                let mut result = left_vals;
                if (**predicate)(&node.key) {
                    result.push(node.key.clone());
                }
                result.append(&mut right_vals);
                result
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn reduce_parallel<F>(&self, op: &Arc<F>, identity: T) -> (reduced: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        decreases *self,
    {
        match self {
            | None => identity,
            | Some(node) => {
                proof {
                    reveal_with_fuel(link_spec_size, 2);
                }
                let id_left = identity.clone();
                let left_acc = node.left.reduce_parallel(op, id_left);
                let right_acc = node.right.reduce_parallel(op, identity);
                let with_key = (**op)(left_acc, node.key.clone());
                (**op)(with_key, right_acc)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn height_rec(&self) -> (h: usize)
        decreases *self,
    // Veracity: NEEDED proof block
    {
        match self {
            | None => 0,
            | Some(node) => {
                proof {
                    // link_height = 1 + max(left, right), so children have height < usize::MAX.
                }
                1 + node.left.height_rec().max(node.right.height_rec())
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    fn compute_link_spec_size(&self) -> (n: usize)
        decreases *self,
    {
        match self {
            None => 0,
            Some(node) => {
                let l = node.left.compute_link_spec_size();
                let r = node.right.compute_link_spec_size();
                1 + l + r
            }
        }
    }

    } // impl BSTRBMtNodeFns for Link

    // Free function: builds balanced tree from sorted slice.

    /// Builds a node keyed `values[mid]` of the given color over subtrees built from
    /// `values[..mid]` and `values[mid + 1..]`, each of black height `b1`.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one node per key, O(1) per node.
    fn build_split<T: StTInMtT + Ord + TotalOrder>(
        values: &[T], mid: usize, red: bool, half: usize, Ghost(b1): Ghost<nat>,
    ) -> (node: Box<Node<T>>)
        requires
            obeys_feq_clone::<T>(),
            spec_sorted_strict(values@),
            mid < values@.len(),
            half as nat == pow2(b1),
            half as nat <= mid + 1,
            mid + 1 < 2 * (half as nat),
            half as nat <= values@.len() - mid,
            values@.len() - mid < 2 * (half as nat),
        ensures
            spec_is_bst_link(Some(node)),
            forall|x: T| #[trigger] link_contains(Some(node), x) <==> values@.contains(x),
            spec_is_llrb_link(node.left),
            spec_is_llrb_link(node.right),
            !link_is_red(node.left),
            !link_is_red(node.right),
            link_black_height(node.left) == b1,
            link_black_height(node.right) == b1,
            node.color == (if red { Color::Red } else { Color::Black }),
            spec_size_cached_link(Some(node)),
            link_spec_size(Some(node)) == values@.len(),
        decreases values@.len(), 0nat,
    {
        let n = values.len();
        let ghost s = values@;
        let left_slice = slice_subrange(values, 0, mid);
        let right_slice = slice_subrange(values, mid + 1, n);
        proof {
            lemma_sorted_split(s, mid as int);
            assert(left_slice@ =~= s.subrange(0, mid as int));
            assert(right_slice@ =~= s.subrange(mid as int + 1, n as int));
        }
        let left = build_balanced(left_slice, half, Ghost(b1));
        let right = build_balanced(right_slice, half, Ghost(b1));
        let key = values[mid].clone_plus();
        let mut node = Box::new(new_node(key));
        node.left = left;
        node.right = right;
        node.color = if red { Color::Red } else { Color::Black };
        let ghost pre = *node;
        update(&mut node);
        proof {
            lemma_updated_fields(pre);
            lemma_node_from_sorted(s, mid as int, *node);
            reveal_with_fuel(spec_size_cached_link, 2);
            reveal_with_fuel(link_spec_size, 2);
        }
        node
    }

    /// Builds a left-leaning red-black tree with a black root and black height `b`
    /// from a strictly sorted slice, where `cap == 2^b <= n + 1 < 2^(b+1)`. A black
    /// root splits the keys two ways, except when n == 2^(b+1) - 2, where a two-way
    /// split cannot give equal black heights: then the root's left child is red.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one node per key, O(1) per node.
    fn build_balanced<T: StTInMtT + Ord + TotalOrder>(values: &[T], cap: usize, Ghost(b): Ghost<nat>)
        -> (link: Link<T>)
        requires
            obeys_feq_clone::<T>(),
            spec_sorted_strict(values@),
            cap as nat == pow2(b),
            cap as nat <= values@.len() + 1,
            values@.len() + 1 < 2 * (cap as nat),
        ensures
            spec_is_bst_link(link),
            spec_is_llrb_link(link),
            !link_is_red(link),
            spec_size_cached_link(link),
            link_black_height(link) == b,
            link_spec_size(link) == values@.len(),
            forall|x: T| #[trigger] link_contains(link, x) <==> values@.contains(x),
        decreases values@.len(), 1nat,
    {
        let n = values.len();
        if n == 0 {
            proof {
                if b > 0 { lemma_pow2_unfold(b); lemma_pow2_pos((b - 1) as nat); }
            }
            return None;
        }
        proof {
            if b == 0 { lemma2_to64(); }
            lemma_pow2_unfold(b);
        }
        let ghost b1: nat = (b - 1) as nat;
        let half = cap / 2;
        if n / 2 + 1 < cap {
            // Two-way split: a black root over two subtrees of black height b - 1.
            let node = build_split(values, n / 2, false, half, Ghost(b1));
            proof {
                reveal_with_fuel(spec_is_llrb_link, 1);
                reveal_with_fuel(link_black_height, 1);
            }
            Some(node)
        } else {
            // n == 2^(b+1) - 2: a red node p over two subtrees of black height b - 1
            // is the left child of a black root whose right subtree has black height
            // b - 1.
            let a = half - 1;
            let q = cap - 1;
            let p_slice = slice_subrange(values, 0, q);
            let c_slice = slice_subrange(values, q + 1, n);
            let ghost s = values@;
            proof {
                lemma_sorted_split(s, q as int);
                assert(p_slice@ =~= s.subrange(0, q as int));
                assert(c_slice@ =~= s.subrange(q as int + 1, n as int));
            }
            let p = build_split(p_slice, a, true, half, Ghost(b1));
            let sub_c = build_balanced(c_slice, half, Ghost(b1));
            let key = values[q].clone_plus();
            let mut node = Box::new(new_node(key));
            node.left = Some(p);
            node.right = sub_c;
            node.color = Color::Black;
            let ghost pre = *node;
            update(&mut node);
            proof {
                lemma_updated_fields(pre);
                lemma_node_from_sorted(s, q as int, *node);
                reveal_with_fuel(spec_is_llrb_link, 2);
                reveal_with_fuel(link_black_height, 2);
                reveal_with_fuel(spec_size_cached_link, 2);
                reveal_with_fuel(link_spec_size, 2);
            }
            Some(node)
        }
    }

    //		Section 4c. type definitions


    /// Lock predicate: link size fits in usize.
    #[derive(Debug)]
    pub struct BSTRBMtEphInv;

    pub type BSTreeRB<T> = BSTRBMtEph<T>;

    //		Section 4d. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTRBMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub(crate) root: RwLock<Link<T>, BSTRBMtEphInv>,
        pub(crate) ghost_root: Ghost<Link<T>>,
    }

    //		Section 5d. view impls


    impl<T: StTInMtT + Ord + TotalOrder> View for BSTRBMtEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { link_to_bbt(self.spec_ghost_root()) }
    }

    //		Section 8d. traits


    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_bstrbmteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_bstrbmteph_wf(),
                    tree@ == BalBinTree::<T>::Leaf,
                    tree@.tree_is_bst(),
                    forall|x: T| !tree@.tree_contains(x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sorted_slice(values: &[T]) -> (tree: Self)
            requires
                obeys_feq_clone::<T>(),
                spec_sorted_strict(values@),
                values@.len() < usize::MAX,
            ensures
                tree.spec_bstrbmteph_wf(),
                tree@.spec_size() == values@.len(),
                forall|x: T| (#[trigger] tree@.tree_contains(x)) <==> values@.contains(x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>)
            requires old(self).spec_bstrbmteph_wf(),
            ensures self.spec_bstrbmteph_wf(),
                    match inserted {
                        Ok(_) => self@.tree_is_bst()
                            && self@.tree_contains(value)
                            && forall|x: T| (#[trigger] self@.tree_contains(x)) <==>
                                (old(self)@.tree_contains(x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstrbmteph_wf(),
            ensures found == self@.tree_contains(*target);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize)
            requires self.spec_bstrbmteph_wf(),
            ensures n as nat == self@.spec_size();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstrbmteph_wf(),
            ensures b == (self@.spec_size() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires self.spec_bstrbmteph_wf(),
            ensures
                h as nat == self@.spec_height(),
                pow2(h as nat) <= (self@.spec_size() + 1) * (self@.spec_size() + 1);

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>)
            requires self.spec_bstrbmteph_wf(),
            ensures
                found.is_some() == self@.tree_contains(*target),
                found.is_some() ==> found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F>(&self, predicate: F) -> (seq: ArraySeqStPerS<T>)
        where
            // Veracity: NEEDED proof block
            F: Fn(&T) -> bool + Send + Sync
            requires
                self.spec_bstrbmteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync
            requires
                self.spec_bstrbmteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn iter(&self) -> (it: std::vec::IntoIter<T>)
            requires self.spec_bstrbmteph_wf()
            ensures
                vstd::std_specs::vec::into_iter_elts(it) == IteratorSpec::remaining(&it),
                IteratorSpec::decrease(&it) is Some;
    }

    //		Section 9d. impls
// Veracity: NEEDED proof block (speed hint)


    impl<T: StTInMtT + Ord + TotalOrder> BSTRBMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            spec_is_rb_tree_link(self.ghost_root@)
        }

        pub closed spec fn spec_ghost_root(self) -> Link<T> {
            self.ghost_root@
        }
    }
// Veracity: NEEDED proof block

    impl<T: StTInMtT + Ord + TotalOrder> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {
        open spec fn spec_bstrbmteph_wf(&self) -> bool {
            spec_is_rb_tree_link(self.spec_ghost_root())
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> Self {
            BSTRBMtEph {
                root: RwLock::new(None, Ghost(BSTRBMtEphInv)),
                ghost_root: Ghost(None),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_sorted_slice(values: &[T]) -> Self {
            let n = values.len();
            // cap = 2^b, the largest power of two with cap <= n + 1.
            let mut cap: usize = 1;
            let ghost mut b: nat = 0;
            proof { lemma2_to64(); }
            while n - (cap - 1) >= cap
                invariant
                    cap as nat == pow2(b),
                    cap >= 1,
                    cap as nat <= n + 1,
                    n < usize::MAX,
                decreases n + 1 - cap,
            {
                proof { lemma_pow2_unfold(b + 1); }
                cap = cap * 2;
                proof { b = b + 1; }
            }
            let link = build_balanced(values, cap, Ghost(b));
            let ghost ghost_link = link;
            proof {
                lemma_link_to_bbt_size::<T>(ghost_link);
                assert forall|x: T| (#[trigger] link_to_bbt(ghost_link).tree_contains(x))
                    <==> values@.contains(x) by {
                    lemma_link_to_bbt_contains::<T>(ghost_link, x);
                };
            }
            BSTRBMtEph {
                root: RwLock::new(link, Ghost(BSTRBMtEphInv)),
                ghost_root: Ghost(ghost_link),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — O(1) size check from the cache, then one root-to-leaf insert.
        fn insert(&mut self, value: T) -> (inserted: Result<(), ()>) {
            let (mut current, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == current); }
            let sz = current.size_link();
            if sz < usize::MAX {
                // spec_is_bst_link(current) from lock predicate via acquire_write.
                current.insert_link(value);
                let ghost after_insert = current;
                let temp = current.take();
                if let Some(mut node) = temp {
                    node.color = Color::Black;
                    current = Some(node);
                }
                let ghost old_ghost = self.ghost_root@;
                let ghost new_root = current;
                proof {
                    // Color change (Black root) preserves structural properties.
                    reveal_with_fuel(link_spec_size, 2);
                    reveal_with_fuel(link_contains, 2);
                    // Veracity: NEEDED proof block (speed hint)
                    reveal_with_fuel(spec_is_bst_link, 2);
                    // Bridge from trait ensures (spec_contains) to link_contains.
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(after_insert, x) implies
                        (link_contains(old_ghost, x) || x == value)
                    by {
                        // Veracity: NEEDED assert
                        assert(after_insert.spec_contains(x) ==>
                            (old_ghost.spec_contains(x) || x == value));
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| (link_contains(old_ghost, x) || x == value) implies
                        // Veracity: NEEDED proof block
                        link_contains(after_insert, x)
                    by {
                        if link_contains(old_ghost, x) {
                            // Veracity: NEEDED assert
                            assert(old_ghost.spec_contains(x) ==>
                                after_insert.spec_contains(x));
                        }
                    };
                    // Veracity: NEEDED assert
                    assert forall|x: T| link_contains(new_root, x) <==>
                        // Veracity: NEEDED proof block
                        link_contains(after_insert, x)
                    by { reveal_with_fuel(link_contains, 2); };
                    // Size bound: insert adds at most 1, color change preserves size.
                    // Bridge to BalBinTree view.
                    lemma_link_to_bbt_size::<T>(new_root);
                    lemma_link_to_bbt_size::<T>(old_ghost);
                    lemma_link_to_bbt_contains::<T>(new_root, value);
                    // Veracity: NEEDED assert
                    assert forall|x: T| (#[trigger] link_to_bbt(new_root).tree_contains(x))
                        <==> (link_to_bbt(old_ghost).tree_contains(x) || x == value)
                    // Veracity: NEEDED proof block
                    by {
                        lemma_link_to_bbt_contains::<T>(new_root, x);
                        lemma_link_to_bbt_contains::<T>(old_ghost, x);
                    };
                    // Ghost-real bridge: spec_is_bst_link → tree_is_bst via link_to_bbt.
                    lemma_link_to_bbt_is_bst::<T>(new_root);
                    // The new root is black, so the tree is a left-leaning red-black tree.
                    lemma_blacken_root_preserves::<T>(after_insert);
                    assert(new_root == spec_blacken_root(after_insert));
                    assert(spec_is_rb_tree_link(new_root));
                // Veracity: NEEDED proof block
                }
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(current);
                Ok(())
            } else {
                write_handle.release_write(current);
                Err(())
            }
        }

        // Reader: spec_is_bst_link from lock predicate, assume return matches ghost.
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let found = data.find_link(target).is_some();
            proof { assume(found == self@.tree_contains(*target)); }
            handle.release_read();
            found
        }

        // Reader: link_spec_size from lock predicate, assume return matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (n: usize) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let n = data.size_link();
            proof { assume(n as nat == self@.spec_size()); }
            handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool) {
            let handle = self.root.acquire_read();
            let b = handle.borrow().is_none();
            proof { assume(b == (self@.spec_size() == 0)); }
            handle.release_read();
            b
        }

        // Reader: height bounded by size from lock predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            proof {
                // Lock predicate gives link_spec_size(*data) <= usize::MAX.
                // Lemma gives link_height <= link_spec_size.
                lemma_height_le_size::<T>(*data);
            }
            let h = data.height_rec();
            proof { assume(h as nat == self@.spec_height()); }
            proof {
                // Near balance (APAS Definition 37.6) from the red-black invariant.
                lemma_llrb_height_bound::<T>(self.spec_ghost_root());
                lemma_link_to_bbt_height::<T>(self.spec_ghost_root());
                lemma_link_to_bbt_size::<T>(self.spec_ghost_root());
            }
            handle.release_read();
            h
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>) {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let found = data.find_link(target).cloned();
            proof {
                assume(found.is_some() == self@.tree_contains(*target));
                accept(found.is_some() ==> found.unwrap() == *target);
            }
            handle.release_read();
            found
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let min = data.min_link().cloned();
            handle.release_read();
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // spec_is_bst_link(*data) from lock predicate via acquire_read.
            let max = data.max_link().cloned();
            handle.release_read();
            max
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let out = data.in_order_parallel();
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let out = data.pre_order_parallel();
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n h(T)), Span O(n h(T))
        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let predicate = Arc::new(predicate);
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let out = data.filter_parallel(&predicate);
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let op = Arc::new(op);
            let data = handle.borrow();
            // link_spec_size(*data) <= usize::MAX from lock predicate via acquire_read.
            let accumulated = data.reduce_parallel(&op, identity);
            handle.release_read();
            accumulated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn iter(&self) -> std::vec::IntoIter<T> {
            let seq = self.in_order();
            seq.seq.into_iter()
        }
    }

    //		Section 10d. iterators — BSTRBMtEph

    // r212 form C: this `IntoIterator` impl required `requires self.spec_bstrbmteph_wf()`, which
    // verus 0.2026.09.13 rejects on an external trait's impl and no exec check
    // can establish; use `iter()`, which keeps the requires
    // (src/experiments/intoiter_form_c_no_impl.rs).
    /*
    impl<'a, T: StTInMtT + Ord + TotalOrder> std::iter::IntoIterator for &'a BSTRBMtEph<T> {
        type Item = T;
        type IntoIter = BSTRBMtEphIter<T>;
        fn into_iter(self) -> (it: BSTRBMtEphIter<T>)
            requires self.spec_bstrbmteph_wf()
            ensures it@.0 == 0, iter_invariant_bstrbmteph(&it),
        {
            self.iter()
        }
    }
    */

    //		Section 11c. top level coarse locking


    impl<T: StTInMtT + Ord + TotalOrder> RwLockPredicate<Link<T>> for BSTRBMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            spec_is_rb_tree_link(v)
        }
    }

    //		Section 12d. derive impls in verus!


    impl<T: StTInMtT + Ord + TotalOrder> Default for BSTRBMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTRBMtEphLit {
        () => {
            < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> >::new()
        };
        ($($x:expr),* $(,)?) => {{
            let mut __tree = < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl std::fmt::Debug for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Color::Red => write!(f, "Red"),
                Color::Black => write!(f, "Black"),
            }
        }
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(self, f)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("color", &self.color)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    //		Section 14c. derive impls outside verus!


    impl std::fmt::Display for BSTRBMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBMtEphInv")
        }
    }

    //		Section 14d. derive impls outside verus!

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBMtEph(size={})", self.size())
        }
    }
}
