//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared proof lemmas for the BST modules in Chap37.
//! All lemmas operate on `BalBinTree<T>` where `T: TotalOrder`.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 7. proof fns

//		Section 1. module

pub mod BSTSpecsAndLemmas {

    //		Section 2. imports

    use vstd::prelude::*;

    verus! {

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    //		Section 7. proof fns


    /// Decomposes tree_contains through the two-level BalBinTree/BalBinNode trait dispatch.
    pub proof fn lemma_node_contains<T: TotalOrder>(
        left: BalBinTree<T>, val: T, right: BalBinTree<T>, x: T,
    )
        ensures
            BalBinTree::<T>::Node(Box::new(BalBinNode { left: left, value: val, right: right }))
                .tree_contains(x)
                == (val == x || left.tree_contains(x) || right.tree_contains(x)),
    {}

    /// BST ordering: left child element is less than and not equal to the root.
    pub proof fn lemma_bst_left<T: TotalOrder>(
        left: BalBinTree<T>, val: T, right: BalBinTree<T>, x: T,
    )
        requires
            BalBinTree::<T>::Node(Box::new(BalBinNode { left: left, value: val, right: right }))
                .tree_is_bst(),
            left.tree_contains(x),
        ensures
            T::le(x, val),
            x != val,
    {}

    /// BST ordering: right child element is greater than and not equal to the root.
    pub proof fn lemma_bst_right<T: TotalOrder>(
        left: BalBinTree<T>, val: T, right: BalBinTree<T>, x: T,
    )
        requires
            BalBinTree::<T>::Node(Box::new(BalBinNode { left: left, value: val, right: right }))
                .tree_is_bst(),
            right.tree_contains(x),
        ensures
            T::le(val, x),
            x != val,
    {}

    /// When a left subtree is modified (insert/delete), BST ordering wrt the node value
    /// is preserved if every new element satisfies the ordering.
    pub proof fn lemma_modified_left_preserves_bst<T: TotalOrder>(
        old_left: BalBinTree<T>,
        new_left: BalBinTree<T>,
        node_val: T,
    )
        requires
            forall|x: T| (#[trigger] old_left.tree_contains(x)) ==>
                T::le(x, node_val) && x != node_val,
            forall|x: T| (#[trigger] new_left.tree_contains(x)) ==>
                (old_left.tree_contains(x) || (T::le(x, node_val) && x != node_val)),
        ensures
            forall|x: T| (#[trigger] new_left.tree_contains(x)) ==>
                T::le(x, node_val) && x != node_val,
    {}

    /// Mirror of lemma_modified_left_preserves_bst for right subtree modifications.
    pub proof fn lemma_modified_right_preserves_bst<T: TotalOrder>(
        old_right: BalBinTree<T>,
        new_right: BalBinTree<T>,
        node_val: T,
    )
        requires
            forall|x: T| (#[trigger] old_right.tree_contains(x)) ==>
                T::le(node_val, x) && x != node_val,
            forall|x: T| (#[trigger] new_right.tree_contains(x)) ==>
                (old_right.tree_contains(x) || (T::le(node_val, x) && x != node_val)),
        ensures
            forall|x: T| (#[trigger] new_right.tree_contains(x)) ==>
                T::le(node_val, x) && x != node_val,
    {}

    /// Decomposes tree_is_bst two levels deep, exposing children and grandchildren BST
    /// facts plus all ordering quantifiers. Used by rotation proofs in AVL and RB trees.
    pub proof fn lemma_bst_deep<T: TotalOrder>(tree: BalBinTree<T>)
        requires tree.tree_is_bst(),
        ensures
            match tree {
                BalBinTree::Leaf => true,
                BalBinTree::Node(node) =>
                    node.left.tree_is_bst()
                    && node.right.tree_is_bst()
                    && (forall|x: T| (#[trigger] node.left.tree_contains(x)) ==>
                        T::le(x, node.value) && x != node.value)
                    && (forall|x: T| (#[trigger] node.right.tree_contains(x)) ==>
                        T::le(node.value, x) && x != node.value)
                    && match node.left {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(lnode) =>
                            lnode.left.tree_is_bst()
                            && lnode.right.tree_is_bst()
                            && (forall|x: T| (#[trigger] lnode.left.tree_contains(x)) ==>
                                T::le(x, lnode.value) && x != lnode.value)
                            && (forall|x: T| (#[trigger] lnode.right.tree_contains(x)) ==>
                                T::le(lnode.value, x) && x != lnode.value)
                    }
                    && match node.right {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(rnode) =>
                            rnode.left.tree_is_bst()
                            && rnode.right.tree_is_bst()
                            && (forall|x: T| (#[trigger] rnode.left.tree_contains(x)) ==>
                                T::le(x, rnode.value) && x != rnode.value)
                            && (forall|x: T| (#[trigger] rnode.right.tree_contains(x)) ==>
                                T::le(rnode.value, x) && x != rnode.value)
                    }
            }
    {
        match tree {
            BalBinTree::Leaf => {},
            BalBinTree::Node(node) => {
                // Veracity: NEEDED assert
                assert(node.left.tree_is_bst());
                // Veracity: NEEDED assert
                assert(node.right.tree_is_bst());
                match node.left {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(lnode) => {
                    },
                }
                match node.right {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(rnode) => {
                    },
                }
            },
        }
    }

    /// max(a+1, b) <= max(a, b) + 1 for natural numbers.
    pub proof fn lemma_max_plus_one(a: nat, b: nat)
        ensures
            (if a >= b { a + 1 } else { b }) <= (if a >= b { a } else { b }) + 1,
    {
    }

    } // verus!
} // mod BSTSpecsAndLemmas
