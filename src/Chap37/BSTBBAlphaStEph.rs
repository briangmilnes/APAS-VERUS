// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Ephemeral weight-balanced (BB[α]) binary search tree (APAS Chap37).
//! Rotation-based weight balance (Nievergelt and Reingold, in Adams' formulation)
//! with the integer parameters (Δ, Γ) = (3, 2) of Hirai and Yamamoto, "Balancing
//! weight-balanced trees" (JFP 2011). The weight of a subtree is its size plus one;
//! at every node each child's weight is at most 3 times its sibling's, so each child
//! has at least a quarter of the node's weight (α = 1/4 ≤ 1 − 1/√2). Layer 1
//! (sections 4a-9a) verifies insert and delete on size-cached nodes; Layer 2
//! (sections 4b-10b) wraps a link whose view is the BalBinTree without sizes.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 6a. spec fns
//	Section 7a. proof fns/broadcast groups
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 10b. iterators
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTBBAlphaStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power::pow;

    verus!
{

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct Node<T: TotalOrder> {
        pub key: T,
        pub size: usize,
        pub left: Option<Box<Node<T>>>,
        pub right: Option<Box<Node<T>>>,
    }

    pub type Link<T> = Option<Box<Node<T>>>;

    //		Section 6a. spec fns


    /// Structural node count for weight-balanced tree links.
    pub open spec fn link_spec_size<T: TotalOrder>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_spec_size(node.left) + link_spec_size(node.right),
        }
    }

    /// Spec-level containment for weight-balanced tree links.
    pub open spec fn link_contains<T: TotalOrder>(link: Link<T>, target: T) -> bool
        decreases link,
    {
        match link {
            None => false,
            Some(node) => node.key == target
                || link_contains(node.left, target)
                || link_contains(node.right, target),
        }
    }

    /// Spec-level height for weight-balanced tree links.
    pub open spec fn link_height<T: TotalOrder>(link: Link<T>) -> nat
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

    /// BST ordering invariant for weight-balanced tree links.
    pub open spec fn spec_is_bst_link<T: TotalOrder>(link: Link<T>) -> bool
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

    /// Convert Link<T> (concrete tree pointer) to BalBinTree<T> (abstract binary tree).
    /// Strips the size fields, retaining structure and keys.
    pub open spec fn link_to_bbt<T: TotalOrder>(link: Link<T>) -> BalBinTree<T>
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

    /// The size field stored at the root of a link; 0 for an empty link.
    pub open spec fn link_size_field<T: TotalOrder>(link: Link<T>) -> usize {
        match link {
            None => 0usize,
            Some(node) => node.size,
        }
    }

    /// Every cached size field equals the structural node count below it.
    pub open spec fn spec_size_cached_link<T: TotalOrder>(link: Link<T>) -> bool
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


    /// The weight of a link: its size plus one, the number of its empty leaves.
    pub open spec fn link_weight<T: TotalOrder>(link: Link<T>) -> nat {
        link_spec_size(link) + 1
    }

    /// The weight-balance condition (Δ = 3) between two sibling weights.
    pub open spec fn spec_weights_balanced(a: nat, b: nat) -> bool {
        3 * a >= b && 3 * b >= a
    }

    /// Weight balance at every node.
    pub open spec fn spec_is_wb_link<T: TotalOrder>(link: Link<T>) -> bool
        decreases link,
    {
        match link {
            None => true,
            Some(node) => {
                spec_is_wb_link(node.left)
                && spec_is_wb_link(node.right)
                && spec_weights_balanced(link_weight(node.left), link_weight(node.right))
            }
        }
    }

    /// Every key of `link` is below `k`.
    pub open spec fn spec_all_lt<T: TotalOrder>(link: Link<T>, k: T) -> bool {
        forall|x: T| #[trigger] link_contains(link, x) ==> TotalOrder::le(x, k) && x != k
    }

    /// Every key of `link` is above `k`.
    pub open spec fn spec_all_gt<T: TotalOrder>(link: Link<T>, k: T) -> bool {
        forall|x: T| #[trigger] link_contains(link, x) ==> TotalOrder::le(k, x) && x != k
    }

    /// The node over `l`, `k`, `r` whose size field is computed from the children.
    pub open spec fn spec_mk_node<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> Link<T> {
        Some(Box::new(Node {
            key: k,
            size: (link_spec_size(l) + link_spec_size(r) + 1) as usize,
            left: l,
            right: r,
        }))
    }

    /// Single left rotation of the node (l, k, r): r's key becomes the root.
    pub open spec fn spec_single_left<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> Link<T> {
        match r {
            None => spec_mk_node(l, k, r),
            Some(rn) => spec_mk_node(spec_mk_node(l, k, rn.left), rn.key, rn.right),
        }
    }

    /// Double left rotation of the node (l, k, r): r's left child's key becomes the root.
    pub open spec fn spec_double_left<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> Link<T> {
        match r {
            None => spec_mk_node(l, k, r),
            Some(rn) => match rn.left {
                None => spec_mk_node(l, k, r),
                Some(rl) => spec_mk_node(spec_mk_node(l, k, rl.left), rl.key,
                                         spec_mk_node(rl.right, rn.key, rn.right)),
            },
        }
    }

    /// Single right rotation of the node (l, k, r): l's key becomes the root.
    pub open spec fn spec_single_right<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> Link<T> {
        match l {
            None => spec_mk_node(l, k, r),
            Some(ln) => spec_mk_node(ln.left, ln.key, spec_mk_node(ln.right, k, r)),
        }
    }

    /// Double right rotation of the node (l, k, r): l's right child's key becomes the root.
    pub open spec fn spec_double_right<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> Link<T> {
        match l {
            None => spec_mk_node(l, k, r),
            Some(ln) => match ln.right {
                None => spec_mk_node(l, k, r),
                Some(lr) => spec_mk_node(spec_mk_node(ln.left, ln.key, lr.left), lr.key,
                                         spec_mk_node(lr.right, k, r)),
            },
        }
    }

    /// The node (l, k, r) after `balance`: when one side outweighs the other more
    /// than 3 to 1, rotate toward the lighter side, singly when the heavy child's
    /// outer grandchild outweighs half its inner one (Γ = 2), doubly otherwise.
    pub open spec fn spec_balance<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> Link<T> {
        if link_weight(r) > 3 * link_weight(l) {
            match r {
                None => spec_mk_node(l, k, r),
                Some(rn) => if link_weight(rn.left) < 2 * link_weight(rn.right) {
                    spec_single_left(l, k, r)
                } else {
                    spec_double_left(l, k, r)
                },
            }
        } else if link_weight(l) > 3 * link_weight(r) {
            match l {
                None => spec_mk_node(l, k, r),
                Some(ln) => if link_weight(ln.right) < 2 * link_weight(ln.left) {
                    spec_single_right(l, k, r)
                } else {
                    spec_double_right(l, k, r)
                },
            }
        } else {
            spec_mk_node(l, k, r)
        }
    }

    /// What `balance` needs of the two sides: each weight at most three times the
    /// other plus three, as after one insert into or delete from a balanced node.
    pub open spec fn spec_balance_pre<T: TotalOrder>(l: Link<T>, r: Link<T>) -> bool {
        link_weight(r) <= 3 * link_weight(l) + 3 && link_weight(l) <= 3 * link_weight(r) + 3
    }

    /// A well-formed weight-balanced tree: BST order, weight balance, a correct size
    /// cache, and a size that fits in usize.
    pub open spec fn spec_is_wb_tree_link<T: TotalOrder>(link: Link<T>) -> bool {
        link_spec_size(link) <= usize::MAX
        && spec_is_bst_link(link)
        && spec_is_wb_link(link)
        && spec_size_cached_link(link)
    }

    //		Section 7a. proof fns/broadcast groups

    /// Bridge: link_spec_size == BalBinTree::spec_size after conversion.
    proof fn lemma_link_to_bbt_size<T: TotalOrder>(link: Link<T>)
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
    proof fn lemma_link_to_bbt_contains<T: TotalOrder>(link: Link<T>, target: T)
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
    proof fn lemma_link_to_bbt_height<T: TotalOrder>(link: Link<T>)
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
    proof fn lemma_link_to_bbt_is_bst<T: TotalOrder>(link: Link<T>)
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

    /// A cached link's size field is its structural size.
    proof fn lemma_size_field_cached<T: TotalOrder>(link: Link<T>)
        requires spec_size_cached_link(link),
        ensures link_size_field(link) as nat == link_spec_size(link),
    {
    }

    /// Height is bounded by structural node count.
    proof fn lemma_height_le_size<T: TotalOrder>(link: Link<T>)
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

    /// Keys below `x` are below any `y` above `x`.
    proof fn lemma_all_lt_trans<T: TotalOrder>(link: Link<T>, x: T, y: T)
        requires spec_all_lt(link, x), TotalOrder::le(x, y),
        ensures spec_all_lt(link, y),
    {
        assert forall|z: T| #[trigger] link_contains(link, z) implies TotalOrder::le(z, y) && z != y by {
            T::transitive(z, x, y);
            if z == y { T::antisymmetric(x, y); }
        };
    }

    /// Keys above `x` are above any `y` below `x`.
    proof fn lemma_all_gt_trans<T: TotalOrder>(link: Link<T>, x: T, y: T)
        requires spec_all_gt(link, x), TotalOrder::le(y, x),
        ensures spec_all_gt(link, y),
    {
        assert forall|z: T| #[trigger] link_contains(link, z) implies TotalOrder::le(y, z) && z != y by {
            T::transitive(y, x, z);
            if z == y { T::antisymmetric(x, y); }
        };
    }

    /// A node over BST children ordered around its key: its keys, BST order, size,
    /// weight, size cache, and weight balance.
    proof fn lemma_mk_node<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>)
        requires
            spec_is_bst_link(l),
            spec_is_bst_link(r),
            spec_all_lt(l, k),
            spec_all_gt(r, k),
        ensures
            spec_is_bst_link(spec_mk_node(l, k, r)),
            forall|z: T| #[trigger] link_contains(spec_mk_node(l, k, r), z)
                == (z == k || link_contains(l, z) || link_contains(r, z)),
            link_spec_size(spec_mk_node(l, k, r)) == link_spec_size(l) + link_spec_size(r) + 1,
            link_weight(spec_mk_node(l, k, r)) == link_weight(l) + link_weight(r),
            (spec_size_cached_link(l) && spec_size_cached_link(r)
                && link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX)
                ==> spec_size_cached_link(spec_mk_node(l, k, r)),
            (spec_is_wb_link(l) && spec_is_wb_link(r)
                && spec_weights_balanced(link_weight(l), link_weight(r)))
                ==> spec_is_wb_link(spec_mk_node(l, k, r)),
    {
    }

    /// The parts of a BST node: BST children ordered around its key.
    proof fn lemma_bst_node_parts<T: TotalOrder>(link: Link<T>)
        requires spec_is_bst_link(link), link is Some,
        ensures
            spec_is_bst_link(link->Some_0.left),
            spec_is_bst_link(link->Some_0.right),
            spec_all_lt(link->Some_0.left, link->Some_0.key),
            spec_all_gt(link->Some_0.right, link->Some_0.key),
    {
    }

    /// Single left rotation when the right side outweighs the left more than 3 to 1
    /// (by at most 3) and the right child's inner grandchild is light: the result is
    /// a weight-balanced BST holding the same keys.
    proof fn lemma_single_left<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>)
        requires
            r is Some,
            spec_is_bst_link(l), spec_is_bst_link(r),
            spec_all_lt(l, k), spec_all_gt(r, k),
            spec_size_cached_link(l), spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
            spec_is_wb_link(l), spec_is_wb_link(r),
            link_weight(r) > 3 * link_weight(l),
            link_weight(r) <= 3 * link_weight(l) + 3,
            link_weight(r->Some_0.left) < 2 * link_weight(r->Some_0.right),
        ensures
            spec_is_bst_link(spec_single_left(l, k, r)),
            forall|z: T| #[trigger] link_contains(spec_single_left(l, k, r), z)
                == (z == k || link_contains(l, z) || link_contains(r, z)),
            link_spec_size(spec_single_left(l, k, r)) == link_spec_size(l) + link_spec_size(r) + 1,
            spec_size_cached_link(spec_single_left(l, k, r)),
            spec_is_wb_link(spec_single_left(l, k, r)),
    {
        let rn = r->Some_0;
        lemma_bst_node_parts(r);
        assert(link_contains(r, rn.key));
        assert(spec_all_gt(rn.left, k)) by {
            assert forall|z: T| #[trigger] link_contains(rn.left, z) implies TotalOrder::le(k, z) && z != k by {
                assert(link_contains(r, z));
            };
        };
        lemma_mk_node(l, k, rn.left);
        let inner = spec_mk_node(l, k, rn.left);
        lemma_all_lt_trans(l, k, rn.key);
        assert(spec_all_lt(inner, rn.key));
        lemma_mk_node(inner, rn.key, rn.right);
        assert forall|z: T| #[trigger] link_contains(spec_single_left(l, k, r), z)
            == (z == k || link_contains(l, z) || link_contains(r, z)) by {
            assert(link_contains(inner, z) == (z == k || link_contains(l, z) || link_contains(rn.left, z)));
        };
    }


    /// The weight arithmetic of a double rotation (Hirai and Yamamoto, (Δ, Γ) = (3, 2)):
    /// with sibling weights a and b1 + b2 + c, a heavy side outweighing the light
    /// side more than 3 to 1 by at most 3, and an inner grandchild b1 + b2 at least
    /// twice its sibling c, the three rebuilt nodes are balanced.
    proof fn lemma_double_weights(a: nat, b1: nat, b2: nat, c: nat)
        requires
            a >= 1, b1 >= 1, b2 >= 1, c >= 1,
            spec_weights_balanced(b1, b2),
            spec_weights_balanced(b1 + b2, c),
            b1 + b2 + c > 3 * a,
            b1 + b2 + c <= 3 * a + 3,
            b1 + b2 >= 2 * c,
        ensures
            spec_weights_balanced(a, b1),
            spec_weights_balanced(b2, c),
            spec_weights_balanced(a + b1, b2 + c),
    {
    }
    /// Double left rotation when the right side outweighs the left more than 3 to 1
    /// (by at most 3) and the right child's inner grandchild is heavy.
    proof fn lemma_double_left<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>)
        requires
            r is Some,
            spec_is_bst_link(l), spec_is_bst_link(r),
            spec_all_lt(l, k), spec_all_gt(r, k),
            spec_size_cached_link(l), spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
            spec_is_wb_link(l), spec_is_wb_link(r),
            link_weight(r) > 3 * link_weight(l),
            link_weight(r) <= 3 * link_weight(l) + 3,
            link_weight(r->Some_0.left) >= 2 * link_weight(r->Some_0.right),
        ensures
            r->Some_0.left is Some,
            spec_is_bst_link(spec_double_left(l, k, r)),
            forall|z: T| #[trigger] link_contains(spec_double_left(l, k, r), z)
                == (z == k || link_contains(l, z) || link_contains(r, z)),
            link_spec_size(spec_double_left(l, k, r)) == link_spec_size(l) + link_spec_size(r) + 1,
            spec_size_cached_link(spec_double_left(l, k, r)),
            spec_is_wb_link(spec_double_left(l, k, r)),
    {
        let rn = r->Some_0;
        lemma_bst_node_parts(r);
        let rl = rn.left;
        assert(rl is Some);
        let rln = rl->Some_0;
        lemma_bst_node_parts(rl);
        // The four weights of the double rotation, and the balance and cache facts below r.
        assert(link_weight(r) == link_weight(rl) + link_weight(rn.right));
        assert(link_weight(rl) == link_weight(rln.left) + link_weight(rln.right));
        assert(spec_is_wb_link(rl) && spec_size_cached_link(rl));
        assert(spec_weights_balanced(link_weight(rl), link_weight(rn.right)));
        assert(spec_weights_balanced(link_weight(rln.left), link_weight(rln.right)));
        assert(spec_is_wb_link(rln.left) && spec_is_wb_link(rln.right) && spec_is_wb_link(rn.right));
        assert(spec_size_cached_link(rln.left) && spec_size_cached_link(rln.right)
            && spec_size_cached_link(rn.right));
        lemma_double_weights(link_weight(l), link_weight(rln.left), link_weight(rln.right), link_weight(rn.right));
        assert(link_contains(r, rn.key));
        assert(link_contains(rl, rln.key));
        assert(link_contains(r, rln.key));
        // Left part: (l, k, rl.left).
        assert(spec_all_gt(rln.left, k)) by {
            assert forall|z: T| #[trigger] link_contains(rln.left, z) implies TotalOrder::le(k, z) && z != k by {
                assert(link_contains(rl, z));
                assert(link_contains(r, z));
            };
        };
        lemma_mk_node(l, k, rln.left);
        let a = spec_mk_node(l, k, rln.left);
        // Right part: (rl.right, r.key, r.right).
        assert(spec_all_lt(rln.right, rn.key)) by {
            assert forall|z: T| #[trigger] link_contains(rln.right, z) implies TotalOrder::le(z, rn.key) && z != rn.key by {
                assert(link_contains(rl, z));
            };
        };
        lemma_mk_node(rln.right, rn.key, rn.right);
        let b = spec_mk_node(rln.right, rn.key, rn.right);
        // Root: (a, rl.key, b).
        lemma_all_lt_trans(l, k, rln.key);
        assert(spec_all_lt(a, rln.key));
        assert(TotalOrder::le(rln.key, rn.key));
        lemma_all_gt_trans(rn.right, rn.key, rln.key);
        assert(spec_all_gt(b, rln.key));
        lemma_mk_node(a, rln.key, b);
        assert forall|z: T| #[trigger] link_contains(spec_double_left(l, k, r), z)
            == (z == k || link_contains(l, z) || link_contains(r, z)) by {
            assert(link_contains(a, z) == (z == k || link_contains(l, z) || link_contains(rln.left, z)));
            assert(link_contains(b, z) == (z == rn.key || link_contains(rln.right, z) || link_contains(rn.right, z)));
            assert(link_contains(rl, z) == (z == rln.key || link_contains(rln.left, z) || link_contains(rln.right, z)));
        };
    }

    /// Single right rotation, the mirror of `lemma_single_left`.
    proof fn lemma_single_right<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>)
        requires
            l is Some,
            spec_is_bst_link(l), spec_is_bst_link(r),
            spec_all_lt(l, k), spec_all_gt(r, k),
            spec_size_cached_link(l), spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
            spec_is_wb_link(l), spec_is_wb_link(r),
            link_weight(l) > 3 * link_weight(r),
            link_weight(l) <= 3 * link_weight(r) + 3,
            link_weight(l->Some_0.right) < 2 * link_weight(l->Some_0.left),
        ensures
            spec_is_bst_link(spec_single_right(l, k, r)),
            forall|z: T| #[trigger] link_contains(spec_single_right(l, k, r), z)
                == (z == k || link_contains(l, z) || link_contains(r, z)),
            link_spec_size(spec_single_right(l, k, r)) == link_spec_size(l) + link_spec_size(r) + 1,
            spec_size_cached_link(spec_single_right(l, k, r)),
            spec_is_wb_link(spec_single_right(l, k, r)),
    {
        let ln = l->Some_0;
        lemma_bst_node_parts(l);
        assert(link_contains(l, ln.key));
        assert(spec_all_lt(ln.right, k)) by {
            assert forall|z: T| #[trigger] link_contains(ln.right, z) implies TotalOrder::le(z, k) && z != k by {
                assert(link_contains(l, z));
            };
        };
        lemma_mk_node(ln.right, k, r);
        let inner = spec_mk_node(ln.right, k, r);
        lemma_all_gt_trans(r, k, ln.key);
        assert(spec_all_gt(inner, ln.key));
        lemma_mk_node(ln.left, ln.key, inner);
        assert forall|z: T| #[trigger] link_contains(spec_single_right(l, k, r), z)
            == (z == k || link_contains(l, z) || link_contains(r, z)) by {
            assert(link_contains(inner, z) == (z == k || link_contains(ln.right, z) || link_contains(r, z)));
        };
    }

    /// Double right rotation, the mirror of `lemma_double_left`.
    proof fn lemma_double_right<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>)
        requires
            l is Some,
            spec_is_bst_link(l), spec_is_bst_link(r),
            spec_all_lt(l, k), spec_all_gt(r, k),
            spec_size_cached_link(l), spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
            spec_is_wb_link(l), spec_is_wb_link(r),
            link_weight(l) > 3 * link_weight(r),
            link_weight(l) <= 3 * link_weight(r) + 3,
            link_weight(l->Some_0.right) >= 2 * link_weight(l->Some_0.left),
        ensures
            l->Some_0.right is Some,
            spec_is_bst_link(spec_double_right(l, k, r)),
            forall|z: T| #[trigger] link_contains(spec_double_right(l, k, r), z)
                == (z == k || link_contains(l, z) || link_contains(r, z)),
            link_spec_size(spec_double_right(l, k, r)) == link_spec_size(l) + link_spec_size(r) + 1,
            spec_size_cached_link(spec_double_right(l, k, r)),
            spec_is_wb_link(spec_double_right(l, k, r)),
    {
        let ln = l->Some_0;
        lemma_bst_node_parts(l);
        let lr = ln.right;
        assert(lr is Some);
        let lrn = lr->Some_0;
        lemma_bst_node_parts(lr);
        // The four weights of the double rotation, and the balance and cache facts below l.
        assert(link_weight(l) == link_weight(ln.left) + link_weight(lr));
        assert(link_weight(lr) == link_weight(lrn.left) + link_weight(lrn.right));
        assert(spec_is_wb_link(lr) && spec_size_cached_link(lr));
        assert(spec_weights_balanced(link_weight(ln.left), link_weight(lr)));
        assert(spec_weights_balanced(link_weight(lrn.left), link_weight(lrn.right)));
        assert(spec_is_wb_link(lrn.left) && spec_is_wb_link(lrn.right) && spec_is_wb_link(ln.left));
        assert(spec_size_cached_link(lrn.left) && spec_size_cached_link(lrn.right)
            && spec_size_cached_link(ln.left));
        lemma_double_weights(link_weight(r), link_weight(lrn.right), link_weight(lrn.left), link_weight(ln.left));
        assert(link_contains(l, ln.key));
        assert(link_contains(lr, lrn.key));
        assert(link_contains(l, lrn.key));
        // Right part: (l.right.right, k, r).
        assert(spec_all_lt(lrn.right, k)) by {
            assert forall|z: T| #[trigger] link_contains(lrn.right, z) implies TotalOrder::le(z, k) && z != k by {
                assert(link_contains(lr, z));
                assert(link_contains(l, z));
            };
        };
        lemma_mk_node(lrn.right, k, r);
        let b = spec_mk_node(lrn.right, k, r);
        // Left part: (l.left, l.key, l.right.left).
        assert(spec_all_gt(lrn.left, ln.key)) by {
            assert forall|z: T| #[trigger] link_contains(lrn.left, z) implies TotalOrder::le(ln.key, z) && z != ln.key by {
                assert(link_contains(lr, z));
            };
        };
        lemma_mk_node(ln.left, ln.key, lrn.left);
        let a = spec_mk_node(ln.left, ln.key, lrn.left);
        // Root: (a, l.right.key, b).
        lemma_all_gt_trans(r, k, lrn.key);
        assert(spec_all_gt(b, lrn.key));
        assert(TotalOrder::le(ln.key, lrn.key));
        lemma_all_lt_trans(ln.left, ln.key, lrn.key);
        assert(spec_all_lt(a, lrn.key));
        lemma_mk_node(a, lrn.key, b);
        assert forall|z: T| #[trigger] link_contains(spec_double_right(l, k, r), z)
            == (z == k || link_contains(l, z) || link_contains(r, z)) by {
            assert(link_contains(a, z) == (z == ln.key || link_contains(ln.left, z) || link_contains(lrn.left, z)));
            assert(link_contains(b, z) == (z == k || link_contains(lrn.right, z) || link_contains(r, z)));
            assert(link_contains(lr, z) == (z == lrn.key || link_contains(lrn.left, z) || link_contains(lrn.right, z)));
        };
    }

    /// `balance` on weight-balanced BST sides ordered around `k`, each weight at most
    /// three times the other plus three, gives a weight-balanced BST holding `k` and
    /// the keys of both sides.
    proof fn lemma_balance<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>)
        requires
            spec_is_bst_link(l), spec_is_bst_link(r),
            spec_all_lt(l, k), spec_all_gt(r, k),
            spec_size_cached_link(l), spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
            spec_is_wb_link(l), spec_is_wb_link(r),
            spec_balance_pre(l, r),
        ensures
            spec_is_bst_link(spec_balance(l, k, r)),
            forall|z: T| #[trigger] link_contains(spec_balance(l, k, r), z)
                == (z == k || link_contains(l, z) || link_contains(r, z)),
            link_spec_size(spec_balance(l, k, r)) == link_spec_size(l) + link_spec_size(r) + 1,
            spec_size_cached_link(spec_balance(l, k, r)),
            spec_is_wb_link(spec_balance(l, k, r)),
    {
        if link_weight(r) > 3 * link_weight(l) {
            let rn = r->Some_0;
            if link_weight(rn.left) < 2 * link_weight(rn.right) {
                lemma_single_left(l, k, r);
            } else {
                lemma_double_left(l, k, r);
            }
        } else if link_weight(l) > 3 * link_weight(r) {
            let ln = l->Some_0;
            if link_weight(ln.right) < 2 * link_weight(ln.left) {
                lemma_single_right(l, k, r);
            } else {
                lemma_double_right(l, k, r);
            }
        } else {
            lemma_mk_node(l, k, r);
        }
    }

    /// The minimum of the left child is the minimum of the tree, and it is not the
    /// root key and not in the right child.
    proof fn lemma_left_min_is_min<T: TotalOrder>(link: Link<T>, min: T)
        requires
            link is Some,
            spec_is_bst_link(link),
            link_contains(link->Some_0.left, min),
            forall|z: T| #[trigger] link_contains(link->Some_0.left, z) ==> TotalOrder::le(min, z),
        ensures
            link_contains(link, min),
            min != link->Some_0.key,
            !link_contains(link->Some_0.right, min),
            forall|z: T| #[trigger] link_contains(link, z) ==> TotalOrder::le(min, z),
    {
        let h = link->Some_0;
        assert(TotalOrder::le(min, h.key));
        assert(!link_contains(h.right, min)) by {
            if link_contains(h.right, min) { T::antisymmetric(min, h.key); }
        };
        assert forall|z: T| #[trigger] link_contains(link, z) implies TotalOrder::le(min, z) by {
            if link_contains(h.right, z) {
                T::transitive(min, h.key, z);
            } else if z == h.key {
            } else {
                assert(link_contains(h.left, z));
            }
        };
    }


    /// Insert below the left child: `balance` over the grown left child keeps a
    /// weight-balanced BST holding the old keys and `v`.
    proof fn lemma_insert_left_step<T: TotalOrder>(s0: Link<T>, nl: Link<T>, v: T)
        requires
            s0 is Some,
            spec_is_bst_link(s0), spec_is_wb_link(s0), spec_size_cached_link(s0),
            link_spec_size(s0) < usize::MAX,
            TotalOrder::le(v, s0->Some_0.key), v != s0->Some_0.key,
            spec_is_bst_link(nl), spec_is_wb_link(nl), spec_size_cached_link(nl),
            forall|x: T| #[trigger] link_contains(nl, x) == (link_contains(s0->Some_0.left, x) || x == v),
            link_spec_size(s0->Some_0.left) <= link_spec_size(nl) <= link_spec_size(s0->Some_0.left) + 1,
        ensures
            link_spec_size(nl) + link_spec_size(s0->Some_0.right) + 1 <= usize::MAX,
            spec_is_bst_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            spec_is_wb_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            spec_size_cached_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            forall|x: T| #[trigger] link_contains(spec_balance(nl, s0->Some_0.key, s0->Some_0.right), x)
                == (link_contains(s0, x) || x == v),
            link_spec_size(s0) <= link_spec_size(spec_balance(nl, s0->Some_0.key, s0->Some_0.right))
                <= link_spec_size(s0) + 1,
    {
        let h = s0->Some_0;
        lemma_bst_node_parts(s0);
        assert(spec_all_lt(nl, h.key));
        lemma_balance(nl, h.key, h.right);
    }

    /// Insert below the right child, the mirror of `lemma_insert_left_step`.
    proof fn lemma_insert_right_step<T: TotalOrder>(s0: Link<T>, nr: Link<T>, v: T)
        requires
            s0 is Some,
            spec_is_bst_link(s0), spec_is_wb_link(s0), spec_size_cached_link(s0),
            link_spec_size(s0) < usize::MAX,
            TotalOrder::le(s0->Some_0.key, v), v != s0->Some_0.key,
            spec_is_bst_link(nr), spec_is_wb_link(nr), spec_size_cached_link(nr),
            forall|x: T| #[trigger] link_contains(nr, x) == (link_contains(s0->Some_0.right, x) || x == v),
            link_spec_size(s0->Some_0.right) <= link_spec_size(nr) <= link_spec_size(s0->Some_0.right) + 1,
        ensures
            link_spec_size(s0->Some_0.left) + link_spec_size(nr) + 1 <= usize::MAX,
            spec_is_bst_link(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)),
            spec_is_wb_link(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)),
            spec_size_cached_link(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)),
            forall|x: T| #[trigger] link_contains(spec_balance(s0->Some_0.left, s0->Some_0.key, nr), x)
                == (link_contains(s0, x) || x == v),
            link_spec_size(s0) <= link_spec_size(spec_balance(s0->Some_0.left, s0->Some_0.key, nr))
                <= link_spec_size(s0) + 1,
    {
        let h = s0->Some_0;
        lemma_bst_node_parts(s0);
        assert(spec_all_gt(nr, h.key));
        lemma_balance(h.left, h.key, nr);
    }

    /// Delete `t` below the left child: `balance`
    /// over the shrunk left child keeps a weight-balanced BST without `t`.
    proof fn lemma_delete_left_step<T: TotalOrder>(s0: Link<T>, nl: Link<T>, t: T)
        requires
            s0 is Some,
            spec_is_bst_link(s0), spec_is_wb_link(s0), spec_size_cached_link(s0),
            link_spec_size(s0) <= usize::MAX,
            TotalOrder::le(t, s0->Some_0.key), t != s0->Some_0.key,
            spec_is_bst_link(nl), spec_is_wb_link(nl), spec_size_cached_link(nl),
            forall|x: T| #[trigger] link_contains(nl, x) == (link_contains(s0->Some_0.left, x) && x != t),
            link_spec_size(nl) <= link_spec_size(s0->Some_0.left) <= link_spec_size(nl) + 1,
        ensures
            link_spec_size(nl) + link_spec_size(s0->Some_0.right) + 1 <= usize::MAX,
            spec_is_bst_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            spec_is_wb_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            spec_size_cached_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            forall|x: T| #[trigger] link_contains(spec_balance(nl, s0->Some_0.key, s0->Some_0.right), x)
                == (link_contains(s0, x) && x != t),
            link_spec_size(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)) <= link_spec_size(s0)
                <= link_spec_size(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)) + 1,
    {
        let h = s0->Some_0;
        lemma_bst_node_parts(s0);
        assert(spec_all_lt(nl, h.key));
        lemma_balance(nl, h.key, h.right);
        assert forall|x: T| #[trigger] link_contains(spec_balance(nl, h.key, h.right), x)
            == (link_contains(s0, x) && x != t) by {
            if x == t && link_contains(h.right, x) { T::antisymmetric(t, h.key); }
        };
    }

    /// Delete `t` below the right child, the mirror of `lemma_delete_left_step`.
    proof fn lemma_delete_right_step<T: TotalOrder>(s0: Link<T>, nr: Link<T>, t: T)
        requires
            s0 is Some,
            spec_is_bst_link(s0), spec_is_wb_link(s0), spec_size_cached_link(s0),
            link_spec_size(s0) <= usize::MAX,
            TotalOrder::le(s0->Some_0.key, t), t != s0->Some_0.key,
            spec_is_bst_link(nr), spec_is_wb_link(nr), spec_size_cached_link(nr),
            forall|x: T| #[trigger] link_contains(nr, x) == (link_contains(s0->Some_0.right, x) && x != t),
            link_spec_size(nr) <= link_spec_size(s0->Some_0.right) <= link_spec_size(nr) + 1,
        ensures
            link_spec_size(s0->Some_0.left) + link_spec_size(nr) + 1 <= usize::MAX,
            spec_is_bst_link(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)),
            spec_is_wb_link(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)),
            spec_size_cached_link(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)),
            forall|x: T| #[trigger] link_contains(spec_balance(s0->Some_0.left, s0->Some_0.key, nr), x)
                == (link_contains(s0, x) && x != t),
            link_spec_size(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)) <= link_spec_size(s0)
                <= link_spec_size(spec_balance(s0->Some_0.left, s0->Some_0.key, nr)) + 1,
    {
        let h = s0->Some_0;
        lemma_bst_node_parts(s0);
        assert(spec_all_gt(nr, h.key));
        lemma_balance(h.left, h.key, nr);
        assert forall|x: T| #[trigger] link_contains(spec_balance(h.left, h.key, nr), x)
            == (link_contains(s0, x) && x != t) by {
            if x == t && link_contains(h.left, x) { T::antisymmetric(t, h.key); }
        };
    }

    /// Delete-min below the left child: `balance` over the shrunk left child keeps a
    /// weight-balanced BST without `min`, and `min` was the tree's minimum.
    proof fn lemma_delete_min_step<T: TotalOrder>(s0: Link<T>, nl: Link<T>, min: T)
        requires
            s0 is Some,
            spec_is_bst_link(s0), spec_is_wb_link(s0), spec_size_cached_link(s0),
            link_spec_size(s0) <= usize::MAX,
            link_contains(s0->Some_0.left, min),
            forall|x: T| #[trigger] link_contains(s0->Some_0.left, x) ==> TotalOrder::le(min, x),
            spec_is_bst_link(nl), spec_is_wb_link(nl), spec_size_cached_link(nl),
            forall|x: T| #[trigger] link_contains(nl, x) == (link_contains(s0->Some_0.left, x) && x != min),
            link_spec_size(nl) + 1 == link_spec_size(s0->Some_0.left),
        ensures
            link_spec_size(nl) + link_spec_size(s0->Some_0.right) + 1 <= usize::MAX,
            spec_is_bst_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            spec_is_wb_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            spec_size_cached_link(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)),
            forall|x: T| #[trigger] link_contains(spec_balance(nl, s0->Some_0.key, s0->Some_0.right), x)
                == (link_contains(s0, x) && x != min),
            link_spec_size(spec_balance(nl, s0->Some_0.key, s0->Some_0.right)) + 1 == link_spec_size(s0),
            link_contains(s0, min),
            forall|x: T| #[trigger] link_contains(s0, x) ==> TotalOrder::le(min, x),
    {
        let h = s0->Some_0;
        lemma_bst_node_parts(s0);
        lemma_left_min_is_min(s0, min);
        assert(spec_all_lt(nl, h.key));
        lemma_balance(nl, h.key, h.right);
    }

    /// Delete of the root key through its successor `min`, the minimum of the right
    /// child: `balance` over the left child and the shrunk right child keeps a
    /// weight-balanced BST without the old root key.
    proof fn lemma_delete_root_step<T: TotalOrder>(s0: Link<T>, nr: Link<T>, min: T)
        requires
            s0 is Some,
            spec_is_bst_link(s0), spec_is_wb_link(s0), spec_size_cached_link(s0),
            link_spec_size(s0) <= usize::MAX,
            link_contains(s0->Some_0.right, min),
            forall|x: T| #[trigger] link_contains(s0->Some_0.right, x) ==> TotalOrder::le(min, x),
            spec_is_bst_link(nr), spec_is_wb_link(nr), spec_size_cached_link(nr),
            forall|x: T| #[trigger] link_contains(nr, x) == (link_contains(s0->Some_0.right, x) && x != min),
            link_spec_size(nr) + 1 == link_spec_size(s0->Some_0.right),
        ensures
            link_spec_size(s0->Some_0.left) + link_spec_size(nr) + 1 <= usize::MAX,
            spec_is_bst_link(spec_balance(s0->Some_0.left, min, nr)),
            spec_is_wb_link(spec_balance(s0->Some_0.left, min, nr)),
            spec_size_cached_link(spec_balance(s0->Some_0.left, min, nr)),
            forall|x: T| #[trigger] link_contains(spec_balance(s0->Some_0.left, min, nr), x)
                == (link_contains(s0, x) && x != s0->Some_0.key),
            link_spec_size(spec_balance(s0->Some_0.left, min, nr)) + 1 == link_spec_size(s0),
    {
        let h = s0->Some_0;
        lemma_bst_node_parts(s0);
        assert(TotalOrder::le(h.key, min) && min != h.key);
        lemma_all_lt_trans(h.left, h.key, min);
        assert(spec_all_gt(nr, min)) by {
            assert forall|x: T| #[trigger] link_contains(nr, x) implies TotalOrder::le(min, x) && x != min by {
                assert(link_contains(h.right, x));
            };
        };
        lemma_balance(h.left, min, nr);
        assert forall|x: T| #[trigger] link_contains(spec_balance(h.left, min, nr), x)
            == (link_contains(s0, x) && x != h.key) by {
            if link_contains(h.right, x) && x != min { assert(link_contains(nr, x)); }
        };
    }

    /// Near balance (APAS Definition 37.6): a weight-balanced tree of height h and
    /// weight w satisfies 4^h ≤ 3^h · w, that is h ≤ lg(n + 1) / lg(4/3), about
    /// 2.41 lg(n + 1). The heavier child of a node of weight w weighs at most 3w/4.
    pub proof fn lemma_wb_height_bound<T: TotalOrder>(link: Link<T>)
        requires spec_is_wb_link(link),
        ensures pow(4, link_height(link)) <= pow(3, link_height(link)) * link_weight(link),
        decreases link,
    {
        reveal(pow);
        match link {
            None => {},
            Some(node) => {
                lemma_wb_height_bound::<T>(node.left);
                lemma_wb_height_bound::<T>(node.right);
                let hl = link_height(node.left);
                let hr = link_height(node.right);
                let wl = link_weight(node.left) as int;
                let wr = link_weight(node.right) as int;
                let h = link_height(link);
                let w = link_weight(link) as int;
                assert(w == wl + wr);
                // The taller child c, of height h - 1 and weight wc with 4·wc ≤ 3·w.
                let (hc, wc) = if hl >= hr { (hl, wl) } else { (hr, wr) };
                assert(h == hc + 1);
                assert(4 * wc <= 3 * w);
                let p4 = pow(4, hc);
                let p3 = pow(3, hc);
                assert(p4 <= p3 * wc);
                vstd::arithmetic::power::lemma_pow_positive(3, hc);
                assert(pow(4, h) == 4 * p4);
                assert(pow(3, h) == 3 * p3);
                assert(4 * p4 <= 3 * p3 * w) by (nonlinear_arith)
                    requires p4 <= p3 * wc, 4 * wc <= 3 * w, p3 > 0;
            },
        }
    }

    //		Section 8a. traits


    /// Operations on weight-balanced links (Layer 1).
    pub trait BSTBBAlphaNodeFns<T: TotalOrder>: Sized {
        spec fn spec_bst(self) -> bool;
        spec fn spec_size(self) -> nat;
        spec fn spec_contains(self, target: T) -> bool;
        spec fn spec_height(self) -> nat;
        spec fn spec_is_empty(self) -> bool;
        spec fn spec_wb(self) -> bool;
        spec fn spec_size_cached(self) -> bool;
        spec fn spec_in_order_seq(self) -> Seq<T>;
        spec fn spec_pre_order_seq(self) -> Seq<T>;

        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1) — reads the cached size.
        fn size_link(&self) -> (size: usize)
            requires self.spec_size_cached(),
            ensures size as nat == self.spec_size();
        /// Inserts `value` and rebalances each node on the way back up.
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn insert_link(self, value: T) -> (inserted: Self)
            requires
                self.spec_bst(),
                self.spec_wb(),
                self.spec_size_cached(),
                self.spec_size() < usize::MAX,
            ensures
                inserted.spec_bst(),
                inserted.spec_wb(),
                inserted.spec_size_cached(),
                forall|x: T| #[trigger] inserted.spec_contains(x) <==> (self.spec_contains(x) || x == value),
                self.spec_size() <= inserted.spec_size() <= self.spec_size() + 1,
            decreases self.spec_size();
        /// Removes and returns the minimum key, rebalancing on the way back up.
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one leftmost path, O(1) per level.
        fn delete_min_link(self) -> (pair: (Self, T))
            requires
                !self.spec_is_empty(),
                self.spec_bst(),
                self.spec_wb(),
                self.spec_size_cached(),
            ensures
                pair.0.spec_bst(),
                pair.0.spec_wb(),
                pair.0.spec_size_cached(),
                self.spec_contains(pair.1),
                forall|x: T| #[trigger] self.spec_contains(x) ==> TotalOrder::le(pair.1, x),
                forall|x: T| #[trigger] pair.0.spec_contains(x) <==> (self.spec_contains(x) && x != pair.1),
                pair.0.spec_size() + 1 == self.spec_size(),
            decreases self.spec_size();
        /// Removes `key` if present, rebalancing on the way back up.
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn delete_link(self, key: &T) -> (deleted: Self)
            requires
                self.spec_bst(),
                self.spec_wb(),
                self.spec_size_cached(),
            ensures
                deleted.spec_bst(),
                deleted.spec_wb(),
                deleted.spec_size_cached(),
                forall|x: T| #[trigger] deleted.spec_contains(x) <==> (self.spec_contains(x) && x != *key),
                deleted.spec_size() <= self.spec_size() <= deleted.spec_size() + 1,
            decreases self.spec_size();
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
        /// Appends the keys in in-order (left, key, right) to `out`.
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one clone and one push per key.
        fn in_order_into(&self, out: &mut Vec<T>)
            where T: Clone + Eq
            requires obeys_feq_clone::<T>(),
            ensures out@ == old(out)@ + self.spec_in_order_seq();
        /// Appends the keys in pre-order (key, left, right) to `out`.
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one clone and one push per key.
        fn pre_order_into(&self, out: &mut Vec<T>)
            where T: Clone + Eq
            requires obeys_feq_clone::<T>(),
            ensures out@ == old(out)@ + self.spec_pre_order_seq();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height_rec(&self) -> (h: usize)
            requires self.spec_height() <= usize::MAX as nat,
            ensures h as nat == self.spec_height();
    }


    //		Section 9a. impls


    /// The size of a link from its cached size field.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn link_size<T: TotalOrder>(link: &Link<T>) -> (size: usize)
        requires spec_size_cached_link(*link),
        ensures size as nat == link_spec_size(*link),
    {
        match link {
            None => 0,
            Some(node) => node.size,
        }
    }

    /// Builds the node (l, k, r) with its size computed from the children's sizes.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn mk_node<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> (node: Link<T>)
        requires
            spec_size_cached_link(l),
            spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
        ensures node == spec_mk_node(l, k, r),
    {
        let size = link_size(&l) + link_size(&r) + 1;
        Some(Box::new(Node { key: k, size, left: l, right: r }))
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn single_left<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> (rotated: Link<T>)
        requires
            r is Some,
            spec_size_cached_link(l),
            spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
        ensures rotated == spec_single_left(l, k, r),
    {
        match r {
            None => { vstd::pervasive::unreached() },
            Some(rn) => {
                let Node { key: rk, size: _, left: rl, right: rr } = *rn;
                let inner = mk_node(l, k, rl);
                mk_node(inner, rk, rr)
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn double_left<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> (rotated: Link<T>)
        requires
            r is Some,
            r->Some_0.left is Some,
            spec_size_cached_link(l),
            spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
        ensures rotated == spec_double_left(l, k, r),
    {
        proof { reveal_with_fuel(spec_size_cached_link, 2); reveal_with_fuel(link_spec_size, 2); }
        match r {
            None => { vstd::pervasive::unreached() },
            Some(rn) => {
                let Node { key: rk, size: _, left: rl, right: rr } = *rn;
                match rl {
                    None => { vstd::pervasive::unreached() },
                    Some(rln) => {
                        let Node { key: rlk, size: _, left: rll, right: rlr } = *rln;
                        let a = mk_node(l, k, rll);
                        let b = mk_node(rlr, rk, rr);
                        mk_node(a, rlk, b)
                    },
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn single_right<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> (rotated: Link<T>)
        requires
            l is Some,
            spec_size_cached_link(l),
            spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
        ensures rotated == spec_single_right(l, k, r),
    {
        match l {
            None => { vstd::pervasive::unreached() },
            Some(ln) => {
                let Node { key: lk, size: _, left: ll, right: lr } = *ln;
                let inner = mk_node(lr, k, r);
                mk_node(ll, lk, inner)
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn double_right<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> (rotated: Link<T>)
        requires
            l is Some,
            l->Some_0.right is Some,
            spec_size_cached_link(l),
            spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
        ensures rotated == spec_double_right(l, k, r),
    {
        proof { reveal_with_fuel(spec_size_cached_link, 2); reveal_with_fuel(link_spec_size, 2); }
        match l {
            None => { vstd::pervasive::unreached() },
            Some(ln) => {
                let Node { key: lk, size: _, left: ll, right: lr } = *ln;
                match lr {
                    None => { vstd::pervasive::unreached() },
                    Some(lrn) => {
                        let Node { key: lrk, size: _, left: lrl, right: lrr } = *lrn;
                        let a = mk_node(ll, lk, lrl);
                        let b = mk_node(lrr, k, r);
                        mk_node(a, lrk, b)
                    },
                }
            },
        }
    }

    /// Rebuilds the node (l, k, r), rotating toward the lighter side when one side
    /// outweighs the other more than 3 to 1.
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1)
    fn balance<T: TotalOrder>(l: Link<T>, k: T, r: Link<T>) -> (balanced: Link<T>)
        requires
            spec_size_cached_link(l),
            spec_size_cached_link(r),
            link_spec_size(l) + link_spec_size(r) + 1 <= usize::MAX,
        ensures balanced == spec_balance(l, k, r),
    {
        let wl = link_size(&l) as u128 + 1;
        let wr = link_size(&r) as u128 + 1;
        if wr > 3 * wl {
            // 0: no right child; 1: single rotation; 2: double rotation.
            let kind: u8 = match &r {
                None => 0,
                Some(rn) => {
                    let wrl = link_size(&rn.left) as u128 + 1;
                    let wrr = link_size(&rn.right) as u128 + 1;
                    if wrl < 2 * wrr { 1 } else { 2 }
                },
            };
            if kind == 0 {
                mk_node(l, k, r)
            } else if kind == 1 {
                single_left(l, k, r)
            } else {
                proof { reveal_with_fuel(link_spec_size, 2); }
                double_left(l, k, r)
            }
        } else if wl > 3 * wr {
            let kind: u8 = match &l {
                None => 0,
                Some(ln) => {
                    let wll = link_size(&ln.left) as u128 + 1;
                    let wlr = link_size(&ln.right) as u128 + 1;
                    if wlr < 2 * wll { 1 } else { 2 }
                },
            };
            if kind == 0 {
                mk_node(l, k, r)
            } else if kind == 1 {
                single_right(l, k, r)
            } else {
                proof { reveal_with_fuel(link_spec_size, 2); }
                double_right(l, k, r)
            }
        } else {
            mk_node(l, k, r)
        }
    }

    impl<T: TotalOrder> BSTBBAlphaNodeFns<T> for Link<T> {

    open spec fn spec_bst(self) -> bool { spec_is_bst_link(self) }
    open spec fn spec_size(self) -> nat { link_spec_size(self) }
    open spec fn spec_contains(self, target: T) -> bool { link_contains(self, target) }
    open spec fn spec_height(self) -> nat { link_height(self) }
    open spec fn spec_is_empty(self) -> bool { self is None }
    open spec fn spec_wb(self) -> bool { spec_is_wb_link(self) }
    open spec fn spec_size_cached(self) -> bool { spec_size_cached_link(self) }
    open spec fn spec_in_order_seq(self) -> Seq<T> { link_to_bbt(self).spec_in_order() }
    open spec fn spec_pre_order_seq(self) -> Seq<T> { link_to_bbt(self).spec_pre_order() }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1) — reads the cached size.
    fn size_link(&self) -> (size: usize) {
        link_size(self)
    }

    /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn insert_link(self, value: T) -> (inserted: Self)
        decreases self.spec_size(),
    {
        let ghost s0 = self;
        match self {
            None => {
                proof { lemma_mk_node::<T>(None, value, None); }
                mk_node(None, value, None)
            },
            Some(node) => {
                let Node { key, size: _, left, right } = *node;
                proof { lemma_bst_node_parts::<T>(s0); }
                match TotalOrder::cmp(&value, &key) {
                    core::cmp::Ordering::Less => {
                        let new_left = left.insert_link(value);
                        proof {
                            assert forall|x: T| #[trigger] link_contains(new_left, x)
                                == (link_contains(left, x) || x == value) by {
                                assert(new_left.spec_contains(x) <==> (left.spec_contains(x) || x == value));
                            };
                            lemma_insert_left_step::<T>(s0, new_left, value);
                        }
                        balance(new_left, key, right)
                    },
                    core::cmp::Ordering::Greater => {
                        let new_right = right.insert_link(value);
                        proof {
                            assert forall|x: T| #[trigger] link_contains(new_right, x)
                                == (link_contains(right, x) || x == value) by {
                                assert(new_right.spec_contains(x) <==> (right.spec_contains(x) || x == value));
                            };
                            lemma_insert_right_step::<T>(s0, new_right, value);
                        }
                        balance(left, key, new_right)
                    },
                    core::cmp::Ordering::Equal => {
                        proof { lemma_mk_node::<T>(left, key, right); }
                        mk_node(left, key, right)
                    },
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one leftmost path, O(1) per level.
    fn delete_min_link(self) -> (pair: (Self, T))
        decreases self.spec_size(),
    {
        let ghost s0 = self;
        match self {
            None => { vstd::pervasive::unreached() },
            Some(node) => {
                let Node { key, size: _, left, right } = *node;
                proof { lemma_bst_node_parts::<T>(s0); }
                if left.is_none() {
                    proof {
                        assert forall|x: T| #[trigger] link_contains(s0, x) implies TotalOrder::le(key, x) by {
                            assert(link_contains(s0, x) == (x == key || link_contains(left, x) || link_contains(right, x)));
                            if x == key {
                                T::reflexive(key);
                            }
                        };
                    }
                    (right, key)
                } else {
                    let (new_left, min) = left.delete_min_link();
                    proof {
                        assert(link_contains(left, min));
                        assert forall|x: T| #[trigger] link_contains(left, x) implies TotalOrder::le(min, x) by {
                            assert(left.spec_contains(x));
                        };
                        assert forall|x: T| #[trigger] link_contains(new_left, x)
                            == (link_contains(left, x) && x != min) by {
                            assert(new_left.spec_contains(x) <==> (left.spec_contains(x) && x != min));
                        };
                        lemma_delete_min_step::<T>(s0, new_left, min);
                    }
                    (balance(new_left, key, right), min)
                }
            },
        }
    }

    /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
    fn delete_link(self, target: &T) -> (deleted: Self)
        decreases self.spec_size(),
    {
        let ghost s0 = self;
        match self {
            None => None,
            Some(node) => {
                let Node { key, size: _, left, right } = *node;
                proof { lemma_bst_node_parts::<T>(s0); }
                match TotalOrder::cmp(target, &key) {
                    core::cmp::Ordering::Less => {
                        let new_left = left.delete_link(target);
                        proof {
                            assert forall|x: T| #[trigger] link_contains(new_left, x)
                                == (link_contains(left, x) && x != *target) by {
                                assert(new_left.spec_contains(x) <==> (left.spec_contains(x) && x != *target));
                            };
                            lemma_delete_left_step::<T>(s0, new_left, *target);
                        }
                        balance(new_left, key, right)
                    },
                    core::cmp::Ordering::Greater => {
                        let new_right = right.delete_link(target);
                        proof {
                            assert forall|x: T| #[trigger] link_contains(new_right, x)
                                == (link_contains(right, x) && x != *target) by {
                                assert(new_right.spec_contains(x) <==> (right.spec_contains(x) && x != *target));
                            };
                            lemma_delete_right_step::<T>(s0, new_right, *target);
                        }
                        balance(left, key, new_right)
                    },
                    core::cmp::Ordering::Equal => {
                        if left.is_none() {
                            right
                        } else if right.is_none() {
                            left
                        } else {
                            let (new_right, min) = right.delete_min_link();
                            proof {
                                assert(link_contains(right, min));
                                assert forall|x: T| #[trigger] link_contains(right, x) implies TotalOrder::le(min, x) by {
                                    assert(right.spec_contains(x));
                                };
                                assert forall|x: T| #[trigger] link_contains(new_right, x)
                                    == (link_contains(right, x) && x != min) by {
                                    assert(new_right.spec_contains(x) <==> (right.spec_contains(x) && x != min));
                                };
                                lemma_delete_root_step::<T>(s0, new_right, min);
                            }
                            balance(left, min, new_right)
                        }
                    },
                }
            },
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


    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one clone and one push per key.
    fn in_order_into(&self, out: &mut Vec<T>)
        where T: Clone + Eq
        decreases *self,
    {
        match self {
            None => {
                proof { assert(out@ =~= old(out)@ + self.spec_in_order_seq()); }
            },
            Some(node) => {
                node.left.in_order_into(out);
                let key = node.key.clone_plus();
                out.push(key);
                node.right.in_order_into(out);
                proof {
                    assert(out@ =~= old(out)@ + self.spec_in_order_seq());
                }
            },
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one clone and one push per key.
    fn pre_order_into(&self, out: &mut Vec<T>)
        where T: Clone + Eq
        decreases *self,
    {
        match self {
            None => {
                proof { assert(out@ =~= old(out)@ + self.spec_pre_order_seq()); }
            },
            Some(node) => {
                let key = node.key.clone_plus();
                out.push(key);
                node.left.pre_order_into(out);
                node.right.pre_order_into(out);
                proof {
                    assert(out@ =~= old(out)@ + self.spec_pre_order_seq());
                }
            },
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


    } // impl BSTBBAlphaNodeFns for Link

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaStEph<T: TotalOrder> {
        pub root: Link<T>,
    }

    //		Section 5b. view impls


    impl<T: TotalOrder> View for BSTBBAlphaStEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { link_to_bbt(self.root) }
    }

    //		Section 8b. traits


    pub trait BSTBBAlphaStEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_root(self) -> BalBinTree<T>;
        spec fn spec_bstbbalphasteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures
                tree.spec_bstbbalphasteph_wf(),
                tree.spec_root().tree_is_bst(),
                forall|x: T| !tree.spec_root().tree_contains(x);
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1) — reads the root's size field.
        fn size(&self) -> (n: usize)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstbbalphasteph_wf(),
            ensures b == (self.spec_root().spec_size() == 0);
        /// Near balance (APAS Definition 37.6): 4^h ≤ 3^h · (n + 1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().spec_height() <= usize::MAX,
            ensures
                h == self.spec_root().spec_height(),
                pow(4, h as nat) <= pow(3, h as nat) * (self.spec_root().spec_size() + 1);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn insert(self, value: T) -> (inserted: Self)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
                self.spec_root().spec_size() < usize::MAX,
            ensures
                inserted.spec_bstbbalphasteph_wf(),
                inserted.spec_root().tree_is_bst(),
                inserted.spec_root().tree_contains(value),
                forall|x: T| (#[trigger] inserted.spec_root().tree_contains(x)) <==>
                    (self.spec_root().tree_contains(x) || x == value);
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path.
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures found == self.spec_root().tree_contains(*target);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path.
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                found.is_some() == self.spec_root().tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn delete(self, target: &T) -> (deleted: Self)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                deleted.spec_bstbbalphasteph_wf(),
                deleted.spec_root().tree_is_bst(),
                !deleted.spec_root().tree_contains(*target),
                forall|x: T| (#[trigger] deleted.spec_root().tree_contains(x)) <==>
                    (self.spec_root().tree_contains(x) && x != *target);
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — descends leftmost path.
        fn minimum(&self) -> (min: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                self.spec_root().spec_size() == 0 ==> min.is_none(),
                self.spec_root().spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> self.spec_root().tree_contains(*min.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — descends rightmost path.
        fn maximum(&self) -> (max: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                self.spec_root().spec_size() == 0 ==> max.is_none(),
                self.spec_root().spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> self.spec_root().tree_contains(*max.unwrap());
        /// The keys in in-order (ascending key order).
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n)
        fn in_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires
                self.spec_bstbbalphasteph_wf(),
                obeys_feq_clone::<T>(),
            ensures traversal@ == self.spec_root().spec_in_order();
        /// The keys in pre-order (key, left, right); determines the tree's shape.
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n)
        fn pre_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
            requires
                self.spec_bstbbalphasteph_wf(),
                obeys_feq_clone::<T>(),
            ensures traversal@ == self.spec_root().spec_pre_order();
    }

    //		Section 9b. impls


    impl<T: TotalOrder> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { link_to_bbt(self.root) }
        open spec fn spec_bstbbalphasteph_wf(&self) -> bool { spec_is_wb_tree_link(self.root) }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn new() -> (tree: Self) {
            BSTBBAlphaStEph { root: None }
        }

        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(1), Span O(1) — reads the root's size field.
        fn size(&self) -> (n: usize) {
            proof { lemma_link_to_bbt_size::<T>(self.root); }
            self.root.size_link()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — leaf check.
        fn is_empty(&self) -> (b: bool) {
            proof { lemma_link_to_bbt_size::<T>(self.root); }
            self.root.is_none()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive height.
        fn height(&self) -> (h: usize) {
            proof {
                lemma_link_to_bbt_height::<T>(self.root);
                lemma_link_to_bbt_size::<T>(self.root);
                lemma_wb_height_bound::<T>(self.root);
            }
            self.root.height_rec()
        }

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn insert(self, value: T) -> (inserted: Self) {
            let ghost old_root = self.root;
            proof { lemma_link_to_bbt_size::<T>(old_root); }
            let root = self.root.insert_link(value);
            proof {
                lemma_link_to_bbt_is_bst::<T>(root);
                assert forall|x: T| (#[trigger] link_to_bbt(root).tree_contains(x)) <==>
                    (link_to_bbt(old_root).tree_contains(x) || x == value) by {
                    lemma_link_to_bbt_contains::<T>(root, x);
                    lemma_link_to_bbt_contains::<T>(old_root, x);
                    assert(root.spec_contains(x) <==> (old_root.spec_contains(x) || x == value));
                };
            }
            BSTBBAlphaStEph { root }
        }

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path.
        fn contains(&self, target: &T) -> (found: bool) {
            proof { lemma_link_to_bbt_contains::<T>(self.root, *target); }
            self.root.find_link(target).is_some()
        }

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path.
        fn find(&self, target: &T) -> (found: Option<&T>) {
            proof { lemma_link_to_bbt_contains::<T>(self.root, *target); }
            self.root.find_link(target)
        }

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — one root-to-leaf path, O(1) per level.
        fn delete(self, target: &T) -> (deleted: Self) {
            let ghost old_root = self.root;
            let root = self.root.delete_link(target);
            proof {
                lemma_link_to_bbt_is_bst::<T>(root);
                assert forall|x: T| (#[trigger] link_to_bbt(root).tree_contains(x)) <==>
                    (link_to_bbt(old_root).tree_contains(x) && x != *target) by {
                    lemma_link_to_bbt_contains::<T>(root, x);
                    lemma_link_to_bbt_contains::<T>(old_root, x);
                    assert(root.spec_contains(x) <==> (old_root.spec_contains(x) && x != *target));
                };
                lemma_link_to_bbt_contains::<T>(root, *target);
                assert(root.spec_contains(*target) <==> (old_root.spec_contains(*target) && *target != *target));
            }
            BSTBBAlphaStEph { root }
        }

        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — descends leftmost path.
        fn minimum(&self) -> (min: Option<&T>) {
            let min = self.root.min_link();
            proof {
                lemma_link_to_bbt_size::<T>(self.root);
                if min.is_some() { lemma_link_to_bbt_contains::<T>(self.root, *min.unwrap()); }
            }
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(lg n), Span O(lg n) — descends rightmost path.
        fn maximum(&self) -> (max: Option<&T>) {
            let max = self.root.max_link();
            proof {
                lemma_link_to_bbt_size::<T>(self.root);
                if max.is_some() { lemma_link_to_bbt_contains::<T>(self.root, *max.unwrap()); }
            }
            max
        }

        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n)
        fn in_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
        {
            let mut out = Vec::new();
            self.root.in_order_into(&mut out);
            proof { assert(out@ =~= self.spec_root().spec_in_order()); }
            out
        }

        /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n)
        fn pre_order(&self) -> (traversal: Vec<T>)
            where T: Clone + Eq
        {
            let mut out = Vec::new();
            self.root.pre_order_into(&mut out);
            proof { assert(out@ =~= self.spec_root().spec_pre_order()); }
            out
        }
    }

    //		Section 10b. iterators


    impl<T: TotalOrder> BSTBBAlphaStEph<T> {
        /// Returns a snapshot iterator over the tree elements in ascending key order.
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(n), Span O(n) — in-order traversal.
        pub fn iter(&self) -> (it: std::vec::IntoIter<T>) where T: Clone + Eq
            requires
                self.spec_bstbbalphasteph_wf(),
                obeys_feq_clone::<T>(),
            ensures
                IteratorSpec::remaining(&it) == self.spec_root().spec_in_order(),
                vstd::std_specs::vec::into_iter_elts(it) == self.spec_root().spec_in_order(),
                IteratorSpec::decrease(&it) is Some,
        {
            let traversal = self.in_order();
            traversal.into_iter()
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! BSTBBAlphaStEphLit {
        () => {{
            use $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEphTrait;
            $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEph::new()
        }};
        ($($val:expr),+ $(,)?) => {{
            use $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEphTrait;
            let tree = $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEph::new();
            $(let tree = tree.insert($val);)+
            tree
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl<T: TotalOrder + std::fmt::Debug> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: TotalOrder + std::fmt::Debug> std::fmt::Debug for BSTBBAlphaStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTBBAlphaStEph")
                .field("root", &self.root)
                .finish()
        }
    }

    impl<T: TotalOrder + std::fmt::Debug + std::fmt::Display> std::fmt::Display for BSTBBAlphaStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTBBAlphaStEph({:?})", &self.root)
        }
    }
} // mod
