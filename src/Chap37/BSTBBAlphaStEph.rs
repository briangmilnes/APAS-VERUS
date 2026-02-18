//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[alpha]) binary search tree.
//! Verusified: functional-style BST with BST invariant specs.
//! Weight-balance property is specified but rebalancing is not included
//! (it preserves elements and BST ordering, which is what the specs prove).

// Table of Contents
// 1. module
// 2. imports
// 6. spec fns
// 9. impls
// 12. macros

// 1. module

pub mod BSTBBAlphaStEph {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 9. impls

    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaStEph<T> {
        pub root: BalBinTree<T>,
    }

    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (result: BalBinTree<T>)
        requires tree_is_bst::<T>(node),
        ensures
            tree_is_bst::<T>(result),
            tree_contains(result, value),
            forall|x: T| #![auto] tree_contains(result, x) <==>
                (tree_contains(node, x) || x == value),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => {
                BalBinTree::Node(Box::new(BalBinNode {
                    left: BalBinTree::Leaf,
                    value: value,
                    right: BalBinTree::Leaf,
                }))
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;

                match TotalOrder::cmp(&value, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = insert_node(left, value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: node_val,
                            right: right,
                        }));
                        proof {
                            assert(tree_is_bst::<T>(new_left));
                            assert(tree_is_bst::<T>(old_right));

                            assert forall|x: T| tree_contains(new_left, x) implies
                                T::le(x, node_val) && x != node_val
                            by {
                                if tree_contains(old_left, x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| tree_contains(old_right, x) implies
                                T::le(node_val, x) && x != node_val
                            by {};

                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(new_left, x)
                                    || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = insert_node(right, value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: new_right,
                        }));
                        proof {
                            assert(tree_is_bst::<T>(old_left));
                            assert(tree_is_bst::<T>(new_right));

                            assert forall|x: T| tree_contains(old_left, x) implies
                                T::le(x, node_val) && x != node_val
                            by {};

                            assert forall|x: T| tree_contains(new_right, x) implies
                                T::le(node_val, x) && x != node_val
                            by {
                                if tree_contains(old_right, x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(new_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: right,
                        }));
                        proof {
                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                                assert(value == node_val);
                            };
                        }
                        r
                    }
                }
            }
        }
    }

    fn contains_node<T: TotalOrder>(node: &BalBinTree<T>, target: &T) -> (result: bool)
        requires tree_is_bst::<T>(*node),
        ensures result == tree_contains(*node, *target),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => false,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => true,
                    core::cmp::Ordering::Less => {
                        let r = contains_node(&inner.left, target);
                        proof {
                            if tree_contains(inner.right, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = contains_node(&inner.right, target);
                        proof {
                            if tree_contains(inner.left, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    fn find_node<'a, T: TotalOrder>(node: &'a BalBinTree<T>, target: &T) -> (result: Option<&'a T>)
        requires tree_is_bst::<T>(*node),
        ensures
            result.is_some() == tree_contains(*node, *target),
            result.is_some() ==> *result.unwrap() == *target,
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => Some(&inner.value),
                    core::cmp::Ordering::Less => {
                        let r = find_node(&inner.left, target);
                        proof {
                            if tree_contains(inner.right, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = find_node(&inner.right, target);
                        proof {
                            if tree_contains(inner.left, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (result: Option<&T>)
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.left.is_leaf() {
                    Some(&inner.value)
                } else {
                    min_node(&inner.left)
                }
            }
        }
    }

    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (result: Option<&T>)
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() {
                    Some(&inner.value)
                } else {
                    max_node(&inner.right)
                }
            }
        }
    }

    pub fn bb_new<T: TotalOrder>() -> (tree: BSTBBAlphaStEph<T>)
        ensures
            tree_is_bst::<T>(tree.root),
            forall|x: T| !tree_contains(tree.root, x),
    {
        BSTBBAlphaStEph { root: BalBinTree::Leaf }
    }

    pub fn bb_size<T: TotalOrder>(tree: &BSTBBAlphaStEph<T>) -> (n: usize)
        requires tree.root.spec_size() <= usize::MAX,
        ensures n == tree.root.spec_size(),
    {
        tree.root.size()
    }

    pub fn bb_is_empty<T: TotalOrder>(tree: &BSTBBAlphaStEph<T>) -> (b: bool)
        ensures b == (tree.root.spec_size() == 0),
    {
        tree.root.is_leaf()
    }

    pub fn bb_height<T: TotalOrder>(tree: &BSTBBAlphaStEph<T>) -> (h: usize)
        requires tree.root.spec_height() <= usize::MAX,
        ensures h == tree.root.spec_height(),
    {
        tree.root.height()
    }

    pub fn bb_insert<T: TotalOrder>(tree: BSTBBAlphaStEph<T>, value: T) -> (result: BSTBBAlphaStEph<T>)
        requires tree_is_bst::<T>(tree.root),
        ensures
            tree_is_bst::<T>(result.root),
            tree_contains(result.root, value),
            forall|x: T| #![auto] tree_contains(result.root, x) <==>
                (tree_contains(tree.root, x) || x == value),
    {
        BSTBBAlphaStEph { root: insert_node(tree.root, value) }
    }

    pub fn bb_contains<T: TotalOrder>(tree: &BSTBBAlphaStEph<T>, target: &T) -> (result: bool)
        requires tree_is_bst::<T>(tree.root),
        ensures result == tree_contains(tree.root, *target),
    {
        contains_node(&tree.root, target)
    }

    pub fn bb_find<'a, T: TotalOrder>(tree: &'a BSTBBAlphaStEph<T>, target: &T) -> (result: Option<&'a T>)
        requires tree_is_bst::<T>(tree.root),
        ensures
            result.is_some() == tree_contains(tree.root, *target),
            result.is_some() ==> *result.unwrap() == *target,
    {
        find_node(&tree.root, target)
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! BSTBBAlphaStEphLit {
        () => { $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::bb_new() };
    }
} // mod
