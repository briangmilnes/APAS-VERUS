//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[α]) binary search tree.
//! Verusified: functional-style BB[α] with BST ordering invariant.
//! Weight-balance (α = 3/4) modeled as a spec; rebuild omitted from verified core.

// Table of Contents
// 1. module
// 2. imports
// 4. type definitions
// 6. spec fns
// 8. traits
// 9. impls
// 12. macros
// 13. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTBBAlphaStEph {

    use vstd::prelude::*;
    use vstd::pervasive::unreached;

    verus! {

    // 2. imports

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaStEph<T> {
        pub root: BalBinTree<T>,
    }

    // 6. spec fns

    /// Weight-balance at every node: neither child exceeds 3/4 of total size.
    /// Models ALPHA = 0.75 with integer arithmetic to avoid f64.
    pub open spec fn weight_balanced<T>(tree: BalBinTree<T>) -> bool
        decreases tree.spec_size(),
    {
        match tree {
            BalBinTree::Leaf => true,
            BalBinTree::Node(node) => {
                let total = 1 + node.left.spec_size() + node.right.spec_size();
                weight_balanced(node.left)
                && weight_balanced(node.right)
                && 4 * node.left.spec_size() <= 3 * total
                && 4 * node.right.spec_size() <= 3 * total
            }
        }
    }

    /// Combined BB[α] tree invariant: BST ordering + weight balance.
    pub open spec fn tree_is_bb<T: TotalOrder>(tree: BalBinTree<T>) -> bool {
        tree_is_bst(tree) && weight_balanced(tree)
    }

    // 8. traits

    pub trait BSTBBAlphaStEphTrait<T: TotalOrder>: Sized {
        spec fn spec_root(self) -> BalBinTree<T>;

        fn new() -> (tree: Self)
            ensures
                tree_is_bst::<T>(tree.spec_root()),
                forall|x: T| !tree_contains(tree.spec_root(), x);
        fn size(&self) -> (n: usize)
            requires self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        fn is_empty(&self) -> (b: bool)
            ensures b == (self.spec_root().spec_size() == 0);
        fn height(&self) -> (h: usize)
            requires self.spec_root().spec_height() <= usize::MAX,
            ensures h == self.spec_root().spec_height();
        fn insert(self, value: T) -> (inserted: Self)
            requires tree_is_bst::<T>(self.spec_root()),
            ensures
                tree_is_bst::<T>(inserted.spec_root()),
                tree_contains(inserted.spec_root(), value),
                forall|x: T| #![auto] tree_contains(inserted.spec_root(), x) <==>
                    (tree_contains(self.spec_root(), x) || x == value);
        fn contains(&self, target: &T) -> (found: bool)
            requires tree_is_bst::<T>(self.spec_root()),
            ensures found == tree_contains(self.spec_root(), *target);
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires tree_is_bst::<T>(self.spec_root()),
            ensures
                found.is_some() == tree_contains(self.spec_root(), *target),
                found.is_some() ==> *found.unwrap() == *target;
        fn delete(self, target: &T) -> (deleted: Self)
            requires tree_is_bst::<T>(self.spec_root()),
            ensures
                tree_is_bst::<T>(deleted.spec_root()),
                !tree_contains(deleted.spec_root(), *target),
                forall|x: T| #![auto] tree_contains(deleted.spec_root(), x) <==>
                    (tree_contains(self.spec_root(), x) && x != *target);
        fn minimum(&self) -> (min: Option<&T>)
            requires tree_is_bst::<T>(self.spec_root()),
            ensures
                self.spec_root().spec_size() == 0 ==> min.is_none(),
                self.spec_root().spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> tree_contains(self.spec_root(), *min.unwrap());
        fn maximum(&self) -> (max: Option<&T>)
            requires tree_is_bst::<T>(self.spec_root()),
            ensures
                self.spec_root().spec_size() == 0 ==> max.is_none(),
                self.spec_root().spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> tree_contains(self.spec_root(), *max.unwrap());
    }

    // 9. impls

    impl<T: TotalOrder> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { self.root }

        fn new() -> (tree: Self) {
            BSTBBAlphaStEph { root: BalBinTree::Leaf }
        }

        fn size(&self) -> (n: usize) {
            self.root.size()
        }

        fn is_empty(&self) -> (b: bool) {
            self.root.is_leaf()
        }

        fn height(&self) -> (h: usize) {
            self.root.height()
        }

        fn insert(self, value: T) -> (inserted: Self) {
            BSTBBAlphaStEph { root: insert_node(self.root, value) }
        }

        fn contains(&self, target: &T) -> (found: bool) {
            contains_node(&self.root, target)
        }

        fn find(&self, target: &T) -> (found: Option<&T>) {
            find_node(&self.root, target)
        }

        fn delete(self, target: &T) -> (deleted: Self) {
            BSTBBAlphaStEph { root: delete_node(self.root, target) }
        }

        fn minimum(&self) -> (min: Option<&T>) {
            min_node(&self.root)
        }

        fn maximum(&self) -> (max: Option<&T>) {
            max_node(&self.root)
        }
    }

    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
        requires tree_is_bst::<T>(node),
        ensures
            tree_is_bst::<T>(inserted),
            tree_contains(inserted, value),
            forall|x: T| #![auto] tree_contains(inserted, x) <==>
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

    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (min: Option<&T>)
        requires tree_is_bst::<T>(*node),
        ensures
            node.spec_size() == 0 ==> min.is_none(),
            node.spec_size() > 0 ==> min.is_some(),
            min.is_some() ==> tree_contains(*node, *min.unwrap()),
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

    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (max: Option<&T>)
        requires tree_is_bst::<T>(*node),
        ensures
            node.spec_size() == 0 ==> max.is_none(),
            node.spec_size() > 0 ==> max.is_some(),
            max.is_some() ==> tree_contains(*node, *max.unwrap()),
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

    /// Remove and return the minimum element from a non-empty BST subtree.
    fn delete_min_node<T: TotalOrder>(node: BalBinTree<T>) -> (pair: (BalBinTree<T>, T))
        requires
            node.spec_size() > 0,
            tree_is_bst::<T>(node),
        ensures
            tree_is_bst::<T>(pair.0),
            tree_contains(node, pair.1),
            !tree_contains(pair.0, pair.1),
            forall|x: T| #![auto] tree_contains(pair.0, x) <==>
                (tree_contains(node, x) && x != pair.1),
            forall|x: T| #![auto] tree_contains(node, x) ==> T::le(pair.1, x),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => {
                proof { assert(false); }
                unreached()
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;
                if left.is_leaf() {
                    proof {
                        assert forall|x: T| tree_contains(right, x) implies
                            x != node_val
                        by {};

                        assert forall|x: T| tree_contains(node, x) implies
                            T::le(node_val, x)
                        by {
                            assert(tree_contains(node, x) ==
                                (node_val == x
                                || tree_contains(old_left, x)
                                || tree_contains(old_right, x)));
                            if x == node_val {
                                T::reflexive(node_val);
                            }
                        };

                        assert forall|x: T| tree_contains(old_right, x) ==
                            (tree_contains(node, x) && x != node_val)
                        by {
                            assert(tree_contains(node, x) ==
                                (node_val == x
                                || tree_contains(old_left, x)
                                || tree_contains(old_right, x)));
                        };
                    }
                    (right, node_val)
                } else {
                    let (new_left, min_val) = delete_min_node(left);
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
                            assert(tree_contains(old_left, x));
                        };

                        assert forall|x: T| tree_contains(old_right, x) implies
                            T::le(node_val, x) && x != node_val
                        by {};

                        assert(tree_contains(old_left, min_val));

                        assert forall|x: T| tree_contains(node, x) implies
                            T::le(min_val, x)
                        by {
                            if tree_contains(old_left, x) {
                            } else if x == node_val {
                                assert(T::le(min_val, node_val));
                            } else {
                                assert(tree_contains(old_right, x));
                                assert(T::le(min_val, node_val));
                                assert(T::le(node_val, x));
                                T::transitive(min_val, node_val, x);
                            }
                        };

                        assert forall|x: T| tree_contains(r, x) ==
                            (tree_contains(node, x) && x != min_val)
                        by {
                            assert(tree_contains(r, x) ==
                                (node_val == x
                                || tree_contains(new_left, x)
                                || tree_contains(old_right, x)));
                            assert(tree_contains(node, x) ==
                                (node_val == x
                                || tree_contains(old_left, x)
                                || tree_contains(old_right, x)));
                            if x == min_val {
                                if tree_contains(old_right, min_val) {
                                    assert(T::le(min_val, node_val));
                                    assert(T::le(node_val, min_val));
                                    T::antisymmetric(min_val, node_val);
                                }
                            }
                        };
                    }
                    (r, min_val)
                }
            }
        }
    }

    /// Delete a key from the BST, returning the modified tree.
    fn delete_node<T: TotalOrder>(node: BalBinTree<T>, target: &T) -> (deleted: BalBinTree<T>)
        requires tree_is_bst::<T>(node),
        ensures
            tree_is_bst::<T>(deleted),
            !tree_contains(deleted, *target),
            forall|x: T| #![auto] tree_contains(deleted, x) <==>
                (tree_contains(node, x) && x != *target),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => {
                BalBinTree::Leaf
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;

                match TotalOrder::cmp(target, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = delete_node(left, target);
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
                                assert(tree_contains(old_left, x));
                            };

                            assert forall|x: T| tree_contains(old_right, x) implies
                                T::le(node_val, x) && x != node_val
                            by {};

                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) && x != *target)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(new_left, x)
                                    || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                                if x == *target && tree_contains(old_right, x) {
                                    T::antisymmetric(*target, node_val);
                                }
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = delete_node(right, target);
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
                                assert(tree_contains(old_right, x));
                            };

                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) && x != *target)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(new_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                                if x == *target && tree_contains(old_left, x) {
                                    T::antisymmetric(*target, node_val);
                                }
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        if left.is_leaf() {
                            proof {
                                assert forall|x: T| tree_contains(old_right, x) ==
                                    (tree_contains(node, x) && x != *target)
                                by {
                                    assert(tree_contains(node, x) ==
                                        (node_val == x
                                        || tree_contains(old_left, x)
                                        || tree_contains(old_right, x)));
                                };
                            }
                            right
                        } else if right.is_leaf() {
                            proof {
                                assert forall|x: T| tree_contains(old_left, x) ==
                                    (tree_contains(node, x) && x != *target)
                                by {
                                    assert(tree_contains(node, x) ==
                                        (node_val == x
                                        || tree_contains(old_left, x)
                                        || tree_contains(old_right, x)));
                                };
                            }
                            left
                        } else {
                            let (new_right, successor) = delete_min_node(right);
                            let r = BalBinTree::Node(Box::new(BalBinNode {
                                left: left,
                                value: successor,
                                right: new_right,
                            }));
                            proof {
                                assert(tree_is_bst::<T>(old_left));
                                assert(tree_is_bst::<T>(new_right));
                                assert(tree_contains(old_right, successor));
                                assert(T::le(node_val, successor));
                                assert(successor != node_val);

                                assert forall|x: T| tree_contains(old_left, x) implies
                                    T::le(x, successor) && x != successor
                                by {
                                    assert(T::le(x, node_val));
                                    T::transitive(x, node_val, successor);
                                    if x == successor {
                                        T::antisymmetric(x, node_val);
                                    }
                                };

                                assert forall|x: T| tree_contains(new_right, x) implies
                                    T::le(successor, x) && x != successor
                                by {
                                    assert(tree_contains(old_right, x));
                                };

                                assert forall|x: T| tree_contains(r, x) ==
                                    (tree_contains(node, x) && x != *target)
                                by {
                                    assert(tree_contains(r, x) ==
                                        (successor == x
                                        || tree_contains(old_left, x)
                                        || tree_contains(new_right, x)));
                                    assert(tree_contains(node, x) ==
                                        (node_val == x
                                        || tree_contains(old_left, x)
                                        || tree_contains(old_right, x)));

                                    if successor == x {
                                        assert(tree_contains(old_right, successor));
                                    }

                                    if tree_contains(old_right, x) && x != *target && x != successor {
                                        assert(tree_contains(new_right, x));
                                    }
                                };
                            }
                            r
                        }
                    }
                }
            }
        }
    }

    } // verus!

    // 12. macros

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
// 13. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for BSTBBAlphaStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTBBAlphaStEph")
                .field("root", &self.root)
                .finish()
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for BSTBBAlphaStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTBBAlphaStEph({:?})", &self.root)
        }
    }
} // mod
