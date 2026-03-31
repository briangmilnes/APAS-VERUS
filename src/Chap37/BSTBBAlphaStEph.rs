//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[α]) binary search tree.
//! Verusified: functional-style BB[α] with BST ordering invariant.
//! Weight-balance (α = 3/4) modeled as a spec; rebuild omitted from verified core.

// Table of Contents
// 1. module
// 2. imports
// 4. type definitions
// 5. view impls
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
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{lemma_node_contains, lemma_bst_left, lemma_bst_right};
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaStEph<T> {
        pub root: BalBinTree<T>,
    }

    // 5. view impls

    impl<T> View for BSTBBAlphaStEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.root }
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
        tree.tree_is_bst() && weight_balanced(tree)
    }

    // 8. traits

    pub trait BSTBBAlphaStEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_root(self) -> BalBinTree<T>;
        spec fn spec_bstbbalphasteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures
                tree.spec_bstbbalphasteph_wf(),
                tree.spec_root().tree_is_bst(),
                forall|x: T| !tree.spec_root().tree_contains(x);
        fn size(&self) -> (n: usize)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstbbalphasteph_wf(),
            ensures b == (self.spec_root().spec_size() == 0);
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().spec_height() <= usize::MAX,
            ensures h == self.spec_root().spec_height();
        fn insert(self, value: T) -> (inserted: Self)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                inserted.spec_bstbbalphasteph_wf(),
                inserted.spec_root().tree_is_bst(),
                inserted.spec_root().tree_contains(value),
                forall|x: T| (#[trigger] inserted.spec_root().tree_contains(x)) <==>
                    (self.spec_root().tree_contains(x) || x == value);
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures found == self.spec_root().tree_contains(*target);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — matches APAS
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                found.is_some() == self.spec_root().tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
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
        fn minimum(&self) -> (min: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                self.spec_root().spec_size() == 0 ==> min.is_none(),
                self.spec_root().spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> self.spec_root().tree_contains(*min.unwrap());
        fn maximum(&self) -> (max: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                self.spec_root().spec_size() == 0 ==> max.is_none(),
                self.spec_root().spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> self.spec_root().tree_contains(*max.unwrap());
    }

    // 9. impls

    impl<T: TotalOrder> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { self.root }
        open spec fn spec_bstbbalphasteph_wf(&self) -> bool { self.spec_root().tree_is_bst() }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn new() -> (tree: Self) {
            BSTBBAlphaStEph { root: BalBinTree::Leaf }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- recursive tree traversal.
        fn size(&self) -> (n: usize) {
            self.root.size()
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- leaf check.
        fn is_empty(&self) -> (b: bool) {
            self.root.is_leaf()
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- recursive tree traversal.
        fn height(&self) -> (h: usize) {
            self.root.height()
        }

        /// - APAS: Work O(h(T)), Span O(h(T))
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
        fn insert(self, value: T) -> (inserted: Self) {
            BSTBBAlphaStEph { root: insert_node(self.root, value) }
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

        /// - APAS: Work O(h(T)), Span O(h(T))
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
        fn delete(self, target: &T) -> (deleted: Self) {
            BSTBBAlphaStEph { root: delete_node(self.root, target) }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- descends leftmost path.
        fn minimum(&self) -> (min: Option<&T>) {
            min_node(&self.root)
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- descends rightmost path.
        fn maximum(&self) -> (max: Option<&T>) {
            max_node(&self.root)
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
                let r = BalBinTree::Node(Box::new(BalBinNode {
                    left: BalBinTree::Leaf,
                    value: value,
                    right: BalBinTree::Leaf,
                }));
                assert(r.tree_is_bst());
                r
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

                            assert(r.tree_is_bst());

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

                            assert(r.tree_is_bst());

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

    /// Remove and return the minimum element from a non-empty BST subtree.
    /// - APAS: (no cost stated)
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- descends leftmost path.
    fn delete_min_node<T: TotalOrder>(node: BalBinTree<T>) -> (pair: (BalBinTree<T>, T))
        requires
            node.spec_size() > 0,
            node.tree_is_bst(),
        ensures
            pair.0.tree_is_bst(),
            node.tree_contains(pair.1),
            !pair.0.tree_contains(pair.1),
            forall|x: T| (#[trigger] pair.0.tree_contains(x)) <==>
                (node.tree_contains(x) && x != pair.1),
            forall|x: T| (#[trigger] node.tree_contains(x)) ==> T::le(pair.1, x),
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
                        assert forall|x: T| right.tree_contains(x) implies
                            x != node_val
                        by {
                            lemma_bst_right::<T>(old_left, node_val, old_right, x);
                        };

                        assert forall|x: T| node.tree_contains(x) implies
                            #[trigger] T::le(node_val, x)
                        by {
                            assert(node.tree_contains(x) ==
                                (node_val == x
                                || old_left.tree_contains(x)
                                || old_right.tree_contains(x)));
                            if x == node_val {
                                T::reflexive(node_val);
                            }
                        };

                        assert forall|x: T| old_right.tree_contains(x) ==
                            (node.tree_contains(x) && x != node_val)
                        by {
                            assert(node.tree_contains(x) ==
                                (node_val == x
                                || old_left.tree_contains(x)
                                || old_right.tree_contains(x)));
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
                        assert(new_left.tree_is_bst());
                        assert(old_right.tree_is_bst());

                        assert forall|x: T| new_left.tree_contains(x) implies
                            #[trigger] T::le(x, node_val) && x != node_val
                        by {
                            assert(old_left.tree_contains(x));
                        };

                        assert forall|x: T| old_right.tree_contains(x) implies
                            #[trigger] T::le(node_val, x) && x != node_val
                        by {};

                        assert(old_left.tree_contains(min_val));

                        assert forall|x: T| node.tree_contains(x) implies
                            #[trigger] T::le(min_val, x)
                        by {
                            if old_left.tree_contains(x) {
                            } else if x == node_val {
                                assert(T::le(min_val, node_val));
                            } else {
                                assert(old_right.tree_contains(x));
                                assert(T::le(min_val, node_val));
                                assert(T::le(node_val, x));
                                T::transitive(min_val, node_val, x);
                            }
                        };

                        assert forall|x: T| r.tree_contains(x) ==
                            (node.tree_contains(x) && x != min_val)
                        by {
                            assert(r.tree_contains(x) ==
                                (node_val == x
                                || new_left.tree_contains(x)
                                || old_right.tree_contains(x)));
                            assert(node.tree_contains(x) ==
                                (node_val == x
                                || old_left.tree_contains(x)
                                || old_right.tree_contains(x)));
                            if x == min_val {
                                if old_right.tree_contains(min_val) {
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
    /// - APAS: Work O(h(T)), Span O(h(T))
    /// - Claude-Opus-4.6: Work O(h(T)), Span O(h(T)) -- agrees with APAS.
    fn delete_node<T: TotalOrder>(node: BalBinTree<T>, target: &T) -> (deleted: BalBinTree<T>)
        requires node.tree_is_bst(),
        ensures
            deleted.tree_is_bst(),
            !deleted.tree_contains(*target),
            forall|x: T| (#[trigger] deleted.tree_contains(x)) <==>
                (node.tree_contains(x) && x != *target),
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
                            assert(new_left.tree_is_bst());
                            assert(old_right.tree_is_bst());

                            assert forall|x: T| new_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {
                                assert(old_left.tree_contains(x));
                            };

                            assert forall|x: T| old_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {};

                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) && x != *target)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || new_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                if x == *target && old_right.tree_contains(x) {
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
                            assert(old_left.tree_is_bst());
                            assert(new_right.tree_is_bst());

                            assert forall|x: T| old_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {};

                            assert forall|x: T| new_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {
                                assert(old_right.tree_contains(x));
                            };

                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) && x != *target)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || new_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                if x == *target && old_left.tree_contains(x) {
                                    T::antisymmetric(*target, node_val);
                                }
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        if left.is_leaf() {
                            proof {
                                assert forall|x: T| old_right.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                by {
                                    lemma_node_contains::<T>(old_left, node_val, old_right, x);
                                };
                            }
                            right
                        } else if right.is_leaf() {
                            proof {
                                assert forall|x: T| old_left.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                by {
                                    lemma_node_contains::<T>(old_left, node_val, old_right, x);
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
                                assert(old_left.tree_is_bst());
                                assert(new_right.tree_is_bst());
                                assert(old_right.tree_contains(successor));
                                assert(T::le(node_val, successor));
                                assert(successor != node_val);

                                assert forall|x: T| old_left.tree_contains(x) implies
                                    #[trigger] T::le(x, successor) && x != successor
                                by {
                                    assert(T::le(x, node_val));
                                    T::transitive(x, node_val, successor);
                                    if x == successor {
                                        T::antisymmetric(x, node_val);
                                    }
                                };

                                assert forall|x: T| new_right.tree_contains(x) implies
                                    #[trigger] T::le(successor, x) && x != successor
                                by {
                                    assert(old_right.tree_contains(x));
                                };

                                assert forall|x: T| r.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                by {
                                    assert(r.tree_contains(x) ==
                                        (successor == x
                                        || old_left.tree_contains(x)
                                        || new_right.tree_contains(x)));
                                    assert(node.tree_contains(x) ==
                                        (node_val == x
                                        || old_left.tree_contains(x)
                                        || old_right.tree_contains(x)));

                                    if successor == x {
                                        assert(old_right.tree_contains(successor));
                                    }

                                    if old_right.tree_contains(x) && x != *target && x != successor {
                                        assert(new_right.tree_contains(x));
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
