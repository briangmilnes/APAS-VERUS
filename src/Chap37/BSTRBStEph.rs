//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Red-Black balanced binary search tree.
//! Verusified: functional-style RB with BST ordering invariant + rotation proofs.
//! Color invariant requires extending BalBinTree with a color field (future work).

// Table of Contents
// 1. module
// 2. imports
// 4. type definitions
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls
// 12. macros
// 13. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTRBStEph {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTAVLStEph::BSTAVLStEph::avl_balanced;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTRBStEph<T> {
        pub root: BalBinTree<T>,
    }

    // 6. spec fns

    // The RB color invariant cannot be expressed on BalBinTree since it lacks a color
    // field. The BST ordering invariant and rotation correctness are fully verified.
    // To model colors, BalBinTree would need a per-node color tag or a ghost color map.

    // 7. proof fns

    /// Decomposes tree_is_bst two levels deep. Reused from BSTAVLStEph pattern.
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

    // 8. traits

    pub trait BSTRBStEphTrait<T: TotalOrder>: Sized {
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
    }

    // 9. impls

    /// Right rotation preserving BST ordering and containment.
    fn rotate_right<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires
            tree_is_bst::<T>(tree),
            !(tree is Leaf),
        ensures
            tree_is_bst::<T>(rotated),
            forall|x: T| #![auto] tree_contains(rotated, x) == tree_contains(tree, x),
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
                            left: lr,
                            value: y_val,
                            right: r,
                        }));

                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: ll,
                            value: x_val,
                            right: right_sub,
                        }));

                        proof {
                            lemma_bst_deep::<T>(tree_ghost);

                            assert forall|z: T| tree_contains(old_lr, z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                assert(tree_contains(old_left, z));
                            };

                            assert(tree_contains(old_left, x_val));
                            assert(x_val != y_val);

                            assert(tree_is_bst::<T>(right_sub));

                            assert forall|z: T| tree_contains(right_sub, z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                if tree_contains(old_lr, z) {
                                } else if z == y_val {
                                    assert(x_val != y_val);
                                } else if tree_contains(old_r, z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                }
                            };

                            assert forall|z: T| tree_contains(r, z) ==
                                tree_contains(tree_ghost, z)
                            by {
                                assert(tree_contains(r, z) ==
                                    (x_val == z
                                    || tree_contains(old_ll, z)
                                    || tree_contains(right_sub, z)));
                                assert(tree_contains(right_sub, z) ==
                                    (y_val == z
                                    || tree_contains(old_lr, z)
                                    || tree_contains(old_r, z)));
                                assert(tree_contains(tree_ghost, z) ==
                                    (y_val == z
                                    || tree_contains(old_left, z)
                                    || tree_contains(old_r, z)));
                                assert(tree_contains(old_left, z) ==
                                    (x_val == z
                                    || tree_contains(old_ll, z)
                                    || tree_contains(old_lr, z)));
                            };
                        }
                        r
                    }
                    BalBinTree::Leaf => {
                        BalBinTree::Node(Box::new(BalBinNode {
                            left: BalBinTree::Leaf,
                            value: y_val,
                            right: r,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    /// Left rotation preserving BST ordering and containment.
    fn rotate_left<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires
            tree_is_bst::<T>(tree),
            !(tree is Leaf),
        ensures
            tree_is_bst::<T>(rotated),
            forall|x: T| #![auto] tree_contains(rotated, x) == tree_contains(tree, x),
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
                            left: l,
                            value: x_val,
                            right: rl,
                        }));

                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left_sub,
                            value: y_val,
                            right: rr,
                        }));

                        proof {
                            lemma_bst_deep::<T>(tree_ghost);

                            assert forall|z: T| tree_contains(old_rl, z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                assert(tree_contains(old_right, z));
                            };

                            assert(tree_contains(old_right, y_val));
                            assert(x_val != y_val);

                            assert(tree_is_bst::<T>(left_sub));

                            assert forall|z: T| tree_contains(left_sub, z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                if tree_contains(old_l, z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                } else if z == x_val {
                                    assert(x_val != y_val);
                                } else if tree_contains(old_rl, z) {
                                }
                            };

                            assert forall|z: T| tree_contains(r, z) ==
                                tree_contains(tree_ghost, z)
                            by {
                                assert(tree_contains(r, z) ==
                                    (y_val == z
                                    || tree_contains(left_sub, z)
                                    || tree_contains(old_rr, z)));
                                assert(tree_contains(left_sub, z) ==
                                    (x_val == z
                                    || tree_contains(old_l, z)
                                    || tree_contains(old_rl, z)));
                                assert(tree_contains(tree_ghost, z) ==
                                    (x_val == z
                                    || tree_contains(old_l, z)
                                    || tree_contains(old_right, z)));
                                assert(tree_contains(old_right, z) ==
                                    (y_val == z
                                    || tree_contains(old_rl, z)
                                    || tree_contains(old_rr, z)));
                            };
                        }
                        r
                    }
                    BalBinTree::Leaf => {
                        BalBinTree::Node(Box::new(BalBinNode {
                            left: l,
                            value: x_val,
                            right: BalBinTree::Leaf,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
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

    impl<T: TotalOrder> BSTRBStEphTrait<T> for BSTRBStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { self.root }

        fn new() -> (tree: Self) {
            BSTRBStEph { root: BalBinTree::Leaf }
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
            BSTRBStEph { root: insert_node(self.root, value) }
        }

        fn contains(&self, target: &T) -> (found: bool) {
            contains_node(&self.root, target)
        }

        fn find(&self, target: &T) -> (found: Option<&T>) {
            find_node(&self.root, target)
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! BSTRBStEphLit {
        () => { <$crate::Chap37::BSTRBStEph::BSTRBStEph::BSTRBStEph<_> as $crate::Chap37::BSTRBStEph::BSTRBStEph::BSTRBStEphTrait<_>>::new() };
        ($($val:expr),+ $(,)?) => {{
            use $crate::Chap37::BSTRBStEph::BSTRBStEph::BSTRBStEphTrait;
            let mut tree = $crate::Chap37::BSTRBStEph::BSTRBStEph::BSTRBStEph::new();
            $(tree = tree.insert($val);)+
            tree
        }};
    }

    // 13. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for BSTRBStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBStEph")
                .field("root", &self.root)
                .finish()
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for BSTRBStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBStEph({:?})", &self.root)
        }
    }
} // mod
