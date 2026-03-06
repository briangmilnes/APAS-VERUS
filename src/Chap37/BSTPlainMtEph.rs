//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral binary search tree with vstd::rwlock for verified multi-threaded access.
//! Verusified: BST ordering flows through the lock invariant — no external_body.

//  Table of Contents
//  1. module
//  2. imports
//  4. type definitions
//  8. traits
//  9. impls
//  12. macros
//  13. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTPlainMtEph {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use vstd::rwlock::{RwLock, RwLockPredicate, ReadHandle, WriteHandle};
    use core::marker::PhantomData;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 4. type definitions

    /// Lock invariant: the stored tree satisfies BST ordering.
    /// Because `inv` is `open` and ignores `self`, Verus can resolve
    /// `lock.inv(tree) == tree_is_bst(tree)` without knowing `lock.pred()`.
    pub struct BSTPlainMtEphInv<T> {
        _phantom: PhantomData<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTPlainMtEph<T: TotalOrder> {
        root: RwLock<BalBinTree<T>, BSTPlainMtEphInv<T>>,
    }

    // 8. traits

    pub trait BSTPlainMtEphTrait<T: TotalOrder>: Sized {
        fn new() -> (tree: Self)
            ensures true;
        fn insert(&self, value: T)
            ensures true;
        fn contains(&self, target: &T) -> (found: bool)
            ensures true;
        fn size(&self) -> (n: usize)
            ensures true;
        fn is_empty(&self) -> (b: bool)
            ensures true;
        fn height(&self) -> (h: usize)
            ensures true;
    }

    // 9. impls

    impl<T: TotalOrder> RwLockPredicate<BalBinTree<T>> for BSTPlainMtEphInv<T> {
        open spec fn inv(self, tree: BalBinTree<T>) -> bool {
            tree_is_bst::<T>(tree)
                && tree.spec_size() <= usize::MAX
                && tree.spec_height() <= usize::MAX
        }
    }

    // Verified BST operations (same proofs as BSTPlainStEph).

    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
        requires tree_is_bst::<T>(node),
        ensures
            tree_is_bst::<T>(inserted),
            tree_contains(inserted, value),
            forall|x: T| #![auto] tree_contains(inserted, x) <==>
                (tree_contains(node, x) || x == value),
            inserted.spec_size() <= node.spec_size() + 1,
            inserted.spec_height() <= node.spec_height() + 1,
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => {
                BalBinTree::Node(Box::new(BalBinNode {
                    left: BalBinTree::Leaf, value: value, right: BalBinTree::Leaf,
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
                            left: new_left, value: node_val, right: right,
                        }));
                        proof {
                            assert(tree_is_bst::<T>(new_left));
                            assert(tree_is_bst::<T>(old_right));
                            assert forall|x: T| tree_contains(new_left, x) implies
                                T::le(x, node_val) && x != node_val
                            by { if tree_contains(old_left, x) {} else { assert(x == value); } };
                            assert forall|x: T| tree_contains(old_right, x) implies
                                T::le(node_val, x) && x != node_val by {};
                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) == (node_val == x
                                    || tree_contains(new_left, x) || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) == (node_val == x
                                    || tree_contains(old_left, x) || tree_contains(old_right, x)));
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = insert_node(right, value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: new_right,
                        }));
                        proof {
                            assert(tree_is_bst::<T>(old_left));
                            assert(tree_is_bst::<T>(new_right));
                            assert forall|x: T| tree_contains(old_left, x) implies
                                T::le(x, node_val) && x != node_val by {};
                            assert forall|x: T| tree_contains(new_right, x) implies
                                T::le(node_val, x) && x != node_val
                            by { if tree_contains(old_right, x) {} else { assert(x == value); } };
                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) == (node_val == x
                                    || tree_contains(old_left, x) || tree_contains(new_right, x)));
                                assert(tree_contains(node, x) == (node_val == x
                                    || tree_contains(old_left, x) || tree_contains(old_right, x)));
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: right,
                        }));
                        proof {
                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) == (node_val == x
                                    || tree_contains(old_left, x) || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) == (node_val == x
                                    || tree_contains(old_left, x) || tree_contains(old_right, x)));
                                assert(value == node_val);
                            };
                        }
                        r
                    }
                }
            }
        }
    }

    fn contains_node<T: TotalOrder>(node: &BalBinTree<T>, target: &T) -> (found: bool)
        requires tree_is_bst::<T>(*node),
        ensures found == tree_contains(*node, *target),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => false,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => true,
                    core::cmp::Ordering::Less => {
                        let r = contains_node(&inner.left, target);
                        proof { if tree_contains(inner.right, *target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = contains_node(&inner.right, target);
                        proof { if tree_contains(inner.left, *target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                }
            }
        }
    }

    fn find_node<'a, T: TotalOrder>(node: &'a BalBinTree<T>, target: &T) -> (found: Option<&'a T>)
        requires tree_is_bst::<T>(*node),
        ensures
            found.is_some() == tree_contains(*node, *target),
            found.is_some() ==> *found.unwrap() == *target,
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => Some(&inner.value),
                    core::cmp::Ordering::Less => {
                        let r = find_node(&inner.left, target);
                        proof { if tree_contains(inner.right, *target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = find_node(&inner.right, target);
                        proof { if tree_contains(inner.left, *target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                }
            }
        }
    }

    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (min: Option<&T>)
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.left.is_leaf() { Some(&inner.value) }
                else { min_node(&inner.left) }
            }
        }
    }

    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (max: Option<&T>)
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() { Some(&inner.value) }
                else { max_node(&inner.right) }
            }
        }
    }

    // Public API: lock operations are fully verified through vstd::rwlock.

    impl<T: TotalOrder> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {
        pub fn new() -> (tree: Self)
        {
            BSTPlainMtEph {
                root: RwLock::new(
                    BalBinTree::Leaf,
                    Ghost(BSTPlainMtEphInv { _phantom: PhantomData }),
                ),
            }
        }

        pub fn insert(&self, value: T)
        {
            let (tree, write_handle) = self.root.acquire_write();
            let current_size = tree.size();
            let current_height = tree.height();
            if current_size < usize::MAX && current_height < usize::MAX {
                let new_tree = insert_node(tree, value);
                proof {
                    assert(tree.spec_size() <= usize::MAX);
                    assert(new_tree.spec_size() <= tree.spec_size() + 1);
                    assert(tree.spec_size() + 1 <= usize::MAX);
                    assert(new_tree.spec_size() <= usize::MAX);
                    assert(tree.spec_height() <= usize::MAX);
                    assert(new_tree.spec_height() <= tree.spec_height() + 1);
                    assert(tree.spec_height() + 1 <= usize::MAX);
                    assert(new_tree.spec_height() <= usize::MAX);
                }
                write_handle.release_write(new_tree);
            } else {
                write_handle.release_write(tree);
            }
        }

        pub fn contains(&self, target: &T) -> (found: bool)
        {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = contains_node(tree_ref, target);
            read_handle.release_read();
            found
        }

        pub fn is_empty(&self) -> (b: bool)
        {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let b = tree_ref.is_leaf();
            read_handle.release_read();
            b
        }

        pub fn size(&self) -> (n: usize)
        {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_size() <= usize::MAX);
            let n = tree_ref.size();
            read_handle.release_read();
            n
        }

        pub fn height(&self) -> (h: usize)
        {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_height() <= usize::MAX);
            let h = tree_ref.height();
            read_handle.release_read();
            h
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T> std::fmt::Debug for BSTPlainMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTPlainMtEphInv").finish()
        }
    }

    impl<T> std::fmt::Display for BSTPlainMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTPlainMtEphInv")
        }
    }

    impl<T: TotalOrder> std::fmt::Debug for BSTPlainMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTPlainMtEph").finish()
        }
    }

    impl<T: TotalOrder> std::fmt::Display for BSTPlainMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTPlainMtEph(size={})", self.size())
        }
    }

    // 12. macros

    #[macro_export]
    macro_rules! BSTPlainMtEphLit {
        () => { $crate::Chap37::BSTPlainMtEph::BSTPlainMtEph::BSTPlainMtEph::new() };
        ($($x:expr),+ $(,)?) => {{
            let __tree = $crate::Chap37::BSTPlainMtEph::BSTPlainMtEph::BSTPlainMtEph::new();
            __tree
        }};
    }
} // mod
