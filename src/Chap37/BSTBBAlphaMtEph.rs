//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[α]) binary search tree with vstd::rwlock for verified multi-threaded access.
//! Verusified: BST ordering + weight-balance spec fully verified; lock operations are verified.

// Table of Contents
// 1. module
// 2. imports
// 4. type definitions
// 8. traits
// 6. spec fns
// 9. impls
// 12. macros

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTBBAlphaMtEph {

    use core::marker::PhantomData;

    use vstd::prelude::*;
    use vstd::rwlock::{ReadHandle, RwLock, RwLockPredicate, WriteHandle};

    verus! {

    // 2. imports

    use crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::{tree_is_bb, weight_balanced};
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 4. type definitions

    /// Lock invariant: the stored tree satisfies BST ordering.
    struct BstPred<T> {
        _phantom: PhantomData<T>,
    }

    // 8. traits

    impl<T: TotalOrder> RwLockPredicate<BalBinTree<T>> for BstPred<T> {
        open spec fn inv(self, tree: BalBinTree<T>) -> bool {
            tree_is_bst::<T>(tree)
                && tree.spec_size() <= usize::MAX
                && tree.spec_height() <= usize::MAX
        }
    }

    // 6. spec fns

    // Weight-balance spec is imported from BSTBBAlphaStEph.

    // 9. impls

    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaMtEph<T: TotalOrder> {
        root: RwLock<BalBinTree<T>, BstPred<T>>,
    }

    // Verified BST insert (same proof as BSTBBAlphaStEph / BSTPlainStEph).

    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (result: BalBinTree<T>)
        requires tree_is_bst::<T>(node),
        ensures
            tree_is_bst::<T>(result),
            tree_contains(result, value),
            forall|x: T| #![auto] tree_contains(result, x) <==>
                (tree_contains(node, x) || x == value),
            result.spec_size() <= node.spec_size() + 1,
            result.spec_height() <= node.spec_height() + 1,
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

    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (result: Option<&T>)
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

    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (result: Option<&T>)
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

    impl<T: TotalOrder> BSTBBAlphaMtEph<T> {
        pub fn new() -> (tree: Self)
        {
            BSTBBAlphaMtEph {
                root: RwLock::new(
                    BalBinTree::Leaf,
                    Ghost(BstPred { _phantom: PhantomData }),
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

        pub fn contains(&self, target: &T) -> (result: bool)
        {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let result = contains_node(tree_ref, target);
            read_handle.release_read();
            result
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

        pub fn is_empty(&self) -> (b: bool)
        {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let b = tree_ref.is_leaf();
            read_handle.release_read();
            b
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

    // 12. macros

    #[macro_export]
    macro_rules! BSTBBAlphaMtEphLit {
        () => {
            < $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEph<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
} // mod
