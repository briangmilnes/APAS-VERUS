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
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
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
                match node.left {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(_) => {},
                }
                match node.right {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(_) => {},
                }
            },
        }
    }

    // 8. traits

    pub trait BSTRBStEphTrait<T: TotalOrder>: Sized {
        spec fn spec_root(self) -> BalBinTree<T>;
        spec fn spec_bstrbsteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures
                tree.spec_bstrbsteph_wf(),
                tree.spec_root().tree_is_bst(),
                forall|x: T| !tree.spec_root().tree_contains(x);
        fn size(&self) -> (n: usize)
            requires
                self.spec_bstrbsteph_wf(),
                self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstrbsteph_wf(),
            ensures b == (self.spec_root().spec_size() == 0);
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstrbsteph_wf(),
                self.spec_root().spec_height() <= usize::MAX,
            ensures h == self.spec_root().spec_height();
        fn insert(self, value: T) -> (inserted: Self)
            requires
                self.spec_bstrbsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                inserted.spec_bstrbsteph_wf(),
                inserted.spec_root().tree_is_bst(),
                inserted.spec_root().tree_contains(value),
                forall|x: T| (#[trigger] inserted.spec_root().tree_contains(x)) <==>
                    (self.spec_root().tree_contains(x) || x == value);
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bstrbsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures found == self.spec_root().tree_contains(*target);
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstrbsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                found.is_some() == self.spec_root().tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
    }

    // 9. impls

    /// Right rotation preserving BST ordering and containment.
    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
    fn rotate_right<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires
            tree.tree_is_bst(),
            !(tree is Leaf),
        ensures
            rotated.tree_is_bst(),
            forall|x: T| (#[trigger] rotated.tree_contains(x)) == tree.tree_contains(x),
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

                            assert forall|z: T| old_lr.tree_contains(z) implies
                                #[trigger] T::le(z, y_val) && z != y_val
                            by {
                                assert(old_left.tree_contains(z));
                            };

                            assert(old_left.tree_contains(x_val));
                            assert(x_val != y_val);

                            assert(right_sub.tree_is_bst());

                            assert forall|z: T| right_sub.tree_contains(z) implies
                                #[trigger] T::le(x_val, z) && z != x_val
                            by {
                                if old_lr.tree_contains(z) {
                                } else if z == y_val {
                                    assert(x_val != y_val);
                                } else if old_r.tree_contains(z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                }
                            };

                            assert forall|z: T| r.tree_contains(z) ==
                                tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) ==
                                    (x_val == z
                                    || old_ll.tree_contains(z)
                                    || right_sub.tree_contains(z)));
                                assert(right_sub.tree_contains(z) ==
                                    (y_val == z
                                    || old_lr.tree_contains(z)
                                    || old_r.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) ==
                                    (y_val == z
                                    || old_left.tree_contains(z)
                                    || old_r.tree_contains(z)));
                                assert(old_left.tree_contains(z) ==
                                    (x_val == z
                                    || old_ll.tree_contains(z)
                                    || old_lr.tree_contains(z)));
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
    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
    fn rotate_left<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires
            tree.tree_is_bst(),
            !(tree is Leaf),
        ensures
            rotated.tree_is_bst(),
            forall|x: T| (#[trigger] rotated.tree_contains(x)) == tree.tree_contains(x),
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

                            assert forall|z: T| old_rl.tree_contains(z) implies
                                #[trigger] T::le(x_val, z) && z != x_val
                            by {
                                assert(old_right.tree_contains(z));
                            };

                            assert(old_right.tree_contains(y_val));
                            assert(x_val != y_val);

                            assert(left_sub.tree_is_bst());
                            assert(old_rr.tree_is_bst());

                            assert forall|z: T| (#[trigger] old_rr.tree_contains(z)) implies
                                T::le(y_val, z) && z != y_val
                            by {
                            };

                            assert forall|z: T| left_sub.tree_contains(z) implies
                                #[trigger] T::le(z, y_val) && z != y_val
                            by {
                                if old_l.tree_contains(z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                } else if z == x_val {
                                    assert(x_val != y_val);
                                } else if old_rl.tree_contains(z) {
                                }
                            };

                            assert forall|z: T| r.tree_contains(z) ==
                                tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) ==
                                    (y_val == z
                                    || left_sub.tree_contains(z)
                                    || old_rr.tree_contains(z)));
                                assert(left_sub.tree_contains(z) ==
                                    (x_val == z
                                    || old_l.tree_contains(z)
                                    || old_rl.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) ==
                                    (x_val == z
                                    || old_l.tree_contains(z)
                                    || old_right.tree_contains(z)));
                                assert(old_right.tree_contains(z) ==
                                    (y_val == z
                                    || old_rl.tree_contains(z)
                                    || old_rr.tree_contains(z)));
                            };
                            assert(r.tree_is_bst());
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

    /// - APAS: Work O(h(T)), Span O(h(T))
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
        requires node.tree_is_bst(),
        ensures
            inserted.tree_is_bst(),
            inserted.tree_contains(value),
            forall|x: T| (#[trigger] inserted.tree_contains(x)) <==>
                (node.tree_contains(x) || x == value),
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
                            assert(new_left.tree_is_bst());
                            assert(old_right.tree_is_bst());

                            assert forall|x: T| new_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {
                                if old_left.tree_contains(x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| old_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {};

                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || new_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
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
                            assert(old_left.tree_is_bst());
                            assert(new_right.tree_is_bst());

                            assert forall|x: T| old_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {};

                            assert forall|x: T| new_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {
                                if old_right.tree_contains(x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || new_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
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
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                assert(value == node_val);
                            };
                        }
                        r
                    }
                }
            }
        }
    }

    /// - APAS: Work O(h(T)), Span O(h(T))
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
    fn contains_node<T: TotalOrder>(node: &BalBinTree<T>, target: &T) -> (found: bool)
        requires (*node).tree_is_bst(),
        ensures found == (*node).tree_contains(*target),
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
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = contains_node(&inner.right, target);
                        proof {
                            if inner.left.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    /// - APAS: Work O(h(T)), Span O(h(T))
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
    fn find_node<'a, T: TotalOrder>(node: &'a BalBinTree<T>, target: &T) -> (found: Option<&'a T>)
        requires (*node).tree_is_bst(),
        ensures
            found.is_some() == (*node).tree_contains(*target),
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
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = find_node(&inner.right, target);
                        proof {
                            if inner.left.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    /// - APAS: (no cost stated)
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- descends leftmost path.
    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (min: Option<&T>)
        requires (*node).tree_is_bst(),
        ensures
            node.spec_size() == 0 ==> min.is_none(),
            node.spec_size() > 0 ==> min.is_some(),
            min.is_some() ==> (*node).tree_contains(*min.unwrap()),
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

    /// - APAS: (no cost stated)
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- descends rightmost path.
    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (max: Option<&T>)
        requires (*node).tree_is_bst(),
        ensures
            node.spec_size() == 0 ==> max.is_none(),
            node.spec_size() > 0 ==> max.is_some(),
            max.is_some() ==> (*node).tree_contains(*max.unwrap()),
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
        open spec fn spec_bstrbsteph_wf(&self) -> bool { self.spec_root().tree_is_bst() }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn new() -> (tree: Self) {
            BSTRBStEph { root: BalBinTree::Leaf }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to BalBinTree::size.
        fn size(&self) -> (n: usize) {
            self.root.size()
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- leaf check.
        fn is_empty(&self) -> (b: bool) {
            self.root.is_leaf()
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to BalBinTree::height.
        fn height(&self) -> (h: usize) {
            self.root.height()
        }

        /// - APAS: Work O(h(T)), Span O(h(T))
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
        fn insert(self, value: T) -> (inserted: Self) {
            BSTRBStEph { root: insert_node(self.root, value) }
        }

        /// - APAS: Work O(h(T)), Span O(h(T))
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
        fn contains(&self, target: &T) -> (found: bool) {
            contains_node(&self.root, target)
        }

        /// - APAS: Work O(h(T)), Span O(h(T))
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
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
