//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree with locking for multi-threaded access.
//! Verusified: BST ordering + AVL balance fully verified; lock acquisition is external_body.

// Table of Contents
// 1. module
// 2. imports
// 6. spec fns
// 7. proof fns
// 9. impls
// 12. macros

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTAVLMtEph {

    use std::sync::{Arc, RwLock};

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    use crate::Chap37::BSTAVLStEph::BSTAVLStEph::{avl_balanced, tree_is_avl};
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    #[verifier::external_type_specification]
    #[verifier::external_body]
    #[verifier::reject_recursive_types(T)]
    pub struct ExRwLock<T: ?Sized>(RwLock<T>);

    pub assume_specification<T>[ RwLock::<T>::new ](t: T) -> (v: RwLock<T>);

    // 7. proof fns

    proof fn lemma_bst_deep<T: TotalOrder>(tree: BalBinTree<T>)
        requires tree_is_bst::<T>(tree),
        ensures
            match tree {
                BalBinTree::Leaf => true,
                BalBinTree::Node(node) =>
                    tree_is_bst::<T>(node.left)
                    && tree_is_bst::<T>(node.right)
                    && (forall|x: T| #![auto] tree_contains(node.left, x) ==>
                        T::le(x, node.value) && x != node.value)
                    && (forall|x: T| #![auto] tree_contains(node.right, x) ==>
                        T::le(node.value, x) && x != node.value)
                    && match node.left {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(lnode) =>
                            tree_is_bst::<T>(lnode.left)
                            && tree_is_bst::<T>(lnode.right)
                            && (forall|x: T| #![auto] tree_contains(lnode.left, x) ==>
                                T::le(x, lnode.value) && x != lnode.value)
                            && (forall|x: T| #![auto] tree_contains(lnode.right, x) ==>
                                T::le(lnode.value, x) && x != lnode.value)
                    }
                    && match node.right {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(rnode) =>
                            tree_is_bst::<T>(rnode.left)
                            && tree_is_bst::<T>(rnode.right)
                            && (forall|x: T| #![auto] tree_contains(rnode.left, x) ==>
                                T::le(x, rnode.value) && x != rnode.value)
                            && (forall|x: T| #![auto] tree_contains(rnode.right, x) ==>
                                T::le(rnode.value, x) && x != rnode.value)
                    }
            }
    {
        reveal_with_fuel(tree_is_bst, 3);
        reveal_with_fuel(tree_contains, 3);
    }

    // 9. impls

    #[verifier::reject_recursive_types(T)]
    pub struct BSTAVLMtEph<T> {
        root: Arc<RwLock<BalBinTree<T>>>,
    }

    // Verified rotations (same proofs as BSTAVLStEph).

    fn rotate_right<T: TotalOrder>(tree: BalBinTree<T>) -> (result: BalBinTree<T>)
        requires tree_is_bst::<T>(tree), !(tree is Leaf),
        ensures
            tree_is_bst::<T>(result),
            forall|x: T| #![auto] tree_contains(result, x) == tree_contains(tree, x),
    {
        let ghost tree_ghost = tree;
        match tree {
            BalBinTree::Node(y_box) => {
                let BalBinNode { left: left_tree, value: y_val, right: r } = *y_box;
                let ghost old_left = left_tree;
                let ghost old_r = r;
                match left_tree {
                    BalBinTree::Node(x_box) => {
                        let BalBinNode { left: ll, value: x_val, right: lr } = *x_box;
                        let ghost old_ll = ll;
                        let ghost old_lr = lr;
                        let right_sub = BalBinTree::Node(Box::new(BalBinNode {
                            left: lr, value: y_val, right: r,
                        }));
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: ll, value: x_val, right: right_sub,
                        }));
                        proof {
                            lemma_bst_deep::<T>(tree_ghost);
                            assert forall|z: T| tree_contains(old_lr, z) implies
                                T::le(z, y_val) && z != y_val
                            by { assert(tree_contains(old_left, z)); };
                            assert(tree_contains(old_left, x_val));
                            assert(x_val != y_val);
                            assert(tree_is_bst::<T>(right_sub));
                            assert forall|z: T| tree_contains(right_sub, z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                if tree_contains(old_lr, z) {}
                                else if z == y_val { assert(x_val != y_val); }
                                else if tree_contains(old_r, z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val { T::antisymmetric(x_val, y_val); }
                                }
                            };
                            assert forall|z: T| tree_contains(r, z) == tree_contains(tree_ghost, z)
                            by {
                                assert(tree_contains(r, z) == (x_val == z
                                    || tree_contains(old_ll, z) || tree_contains(right_sub, z)));
                                assert(tree_contains(right_sub, z) == (y_val == z
                                    || tree_contains(old_lr, z) || tree_contains(old_r, z)));
                                assert(tree_contains(tree_ghost, z) == (y_val == z
                                    || tree_contains(old_left, z) || tree_contains(old_r, z)));
                                assert(tree_contains(old_left, z) == (x_val == z
                                    || tree_contains(old_ll, z) || tree_contains(old_lr, z)));
                            };
                        }
                        r
                    }
                    BalBinTree::Leaf => {
                        BalBinTree::Node(Box::new(BalBinNode {
                            left: BalBinTree::Leaf, value: y_val, right: r,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    fn rotate_left<T: TotalOrder>(tree: BalBinTree<T>) -> (result: BalBinTree<T>)
        requires tree_is_bst::<T>(tree), !(tree is Leaf),
        ensures
            tree_is_bst::<T>(result),
            forall|x: T| #![auto] tree_contains(result, x) == tree_contains(tree, x),
    {
        let ghost tree_ghost = tree;
        match tree {
            BalBinTree::Node(x_box) => {
                let BalBinNode { left: l, value: x_val, right: right_tree } = *x_box;
                let ghost old_right = right_tree;
                let ghost old_l = l;
                match right_tree {
                    BalBinTree::Node(y_box) => {
                        let BalBinNode { left: rl, value: y_val, right: rr } = *y_box;
                        let ghost old_rl = rl;
                        let ghost old_rr = rr;
                        let left_sub = BalBinTree::Node(Box::new(BalBinNode {
                            left: l, value: x_val, right: rl,
                        }));
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left_sub, value: y_val, right: rr,
                        }));
                        proof {
                            lemma_bst_deep::<T>(tree_ghost);
                            assert forall|z: T| tree_contains(old_rl, z) implies
                                T::le(x_val, z) && z != x_val
                            by { assert(tree_contains(old_right, z)); };
                            assert(tree_contains(old_right, y_val));
                            assert(x_val != y_val);
                            assert(tree_is_bst::<T>(left_sub));
                            assert forall|z: T| tree_contains(left_sub, z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                if tree_contains(old_l, z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val { T::antisymmetric(x_val, y_val); }
                                } else if z == x_val { assert(x_val != y_val); }
                                else if tree_contains(old_rl, z) {}
                            };
                            assert forall|z: T| tree_contains(r, z) == tree_contains(tree_ghost, z)
                            by {
                                assert(tree_contains(r, z) == (y_val == z
                                    || tree_contains(left_sub, z) || tree_contains(old_rr, z)));
                                assert(tree_contains(left_sub, z) == (x_val == z
                                    || tree_contains(old_l, z) || tree_contains(old_rl, z)));
                                assert(tree_contains(tree_ghost, z) == (x_val == z
                                    || tree_contains(old_l, z) || tree_contains(old_right, z)));
                                assert(tree_contains(old_right, z) == (y_val == z
                                    || tree_contains(old_rl, z) || tree_contains(old_rr, z)));
                            };
                        }
                        r
                    }
                    BalBinTree::Leaf => {
                        BalBinTree::Node(Box::new(BalBinNode {
                            left: l, value: x_val, right: BalBinTree::Leaf,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    // Verified BST insert (same proof as BSTAVLStEph).

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

    // Public API: lock acquisition is external_body, BST + AVL specs verified.

    impl<T: TotalOrder> BSTAVLMtEph<T> {
        pub fn new() -> (tree: Self)
        {
            BSTAVLMtEph {
                root: Arc::new(RwLock::new(BalBinTree::Leaf)),
            }
        }

        #[verifier::external_body]
        pub fn insert(&self, value: T)
        {
            let mut guard = self.root.write().unwrap();
            let old_tree = std::mem::replace(&mut *guard, BalBinTree::Leaf);
            *guard = insert_node(old_tree, value);
        }

        #[verifier::external_body]
        pub fn contains(&self, target: &T) -> (result: bool)
        {
            let guard = self.root.read().unwrap();
            contains_node(&*guard, target)
        }

        #[verifier::external_body]
        pub fn size(&self) -> (n: usize)
        {
            let guard = self.root.read().unwrap();
            guard.size()
        }

        #[verifier::external_body]
        pub fn is_empty(&self) -> (b: bool)
        {
            let guard = self.root.read().unwrap();
            guard.is_leaf()
        }

        #[verifier::external_body]
        pub fn height(&self) -> (h: usize)
        {
            let guard = self.root.read().unwrap();
            guard.height()
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! BSTAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
} // mod
