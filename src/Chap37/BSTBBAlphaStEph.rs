//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Ephemeral weight-balanced (BB[α]) binary search tree.
//! Verusified: functional-style BB[α] with BST ordering invariant.
//! Weight-balance (α = 3/4) modeled as a spec; rebuild omitted from verified core.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTBBAlphaStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use vstd::pervasive::unreached;

    verus! 
{


    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{lemma_node_contains, lemma_bst_left, lemma_bst_right};
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTSpecsAndLemmas::BSTSpecsAndLemmas::{
        lemma_bst_insert_left, lemma_bst_insert_right,
        lemma_bst_delete_left, lemma_bst_delete_right};
    use crate::vstdplus::total_order::total_order::TotalOrder;

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaStEph<T> {
        pub root: BalBinTree<T>,
    }

    //		Section 5. view impls


    impl<T> View for BSTBBAlphaStEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.root }
    }

    //		Section 6. spec fns


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

    //		Section 8. traits


    pub trait BSTBBAlphaStEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_root(self) -> BalBinTree<T>;
        spec fn spec_bstbbalphasteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures
                tree.spec_bstbbalphasteph_wf(),
                tree.spec_root().tree_is_bst(),
                forall|x: T| !tree.spec_root().tree_contains(x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstbbalphasteph_wf(),
            ensures b == (self.spec_root().spec_size() == 0);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().spec_height() <= usize::MAX,
            ensures h == self.spec_root().spec_height();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures found == self.spec_root().tree_contains(*target);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                found.is_some() == self.spec_root().tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                self.spec_root().spec_size() == 0 ==> min.is_none(),
                self.spec_root().spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> self.spec_root().tree_contains(*min.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<&T>)
            requires
                self.spec_bstbbalphasteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                self.spec_root().spec_size() == 0 ==> max.is_none(),
                self.spec_root().spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> self.spec_root().tree_contains(*max.unwrap());
    }

    /// Exec BST operations on BalBinTree nodes (BB[α] variant).
    pub trait BSTBBAlphaNodeFns<T: TotalOrder>: Sized + BSTSpecFns<T> + BalBinTreeTrait<T> {
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn insert_node(self, value: T) -> (inserted: Self)
            requires self.tree_is_bst(),
            ensures
                inserted.tree_is_bst(),
                inserted.tree_contains(value),
                forall|x: T| (#[trigger] inserted.tree_contains(x)) <==>
                    (self.tree_contains(x) || x == value),
            ;
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn contains_node(&self, target: &T) -> (found: bool)
            requires (*self).tree_is_bst(),
            ensures found == (*self).tree_contains(*target),
            ;
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn find_node(&self, target: &T) -> (found: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                found.is_some() == (*self).tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends leftmost path.
        fn min_node(&self) -> (min: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> min.is_none(),
                (*self).spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> (*self).tree_contains(*min.unwrap()),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends rightmost path.
        fn max_node(&self) -> (max: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> max.is_none(),
                (*self).spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> (*self).tree_contains(*max.unwrap()),
            ;
        /// Remove and return the minimum element from a non-empty BST subtree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends leftmost path.
        fn delete_min_node(self) -> (pair: (Self, T))
            requires
                self.spec_size() > 0,
                self.tree_is_bst(),
            ensures
                pair.0.tree_is_bst(),
                self.tree_contains(pair.1),
                !pair.0.tree_contains(pair.1),
                forall|x: T| (#[trigger] pair.0.tree_contains(x)) <==>
                    (self.tree_contains(x) && x != pair.1),
                forall|x: T| (#[trigger] self.tree_contains(x)) ==> T::le(pair.1, x),
            ;
        /// Delete a key from the BST, returning the modified tree.
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn delete_node(self, target: &T) -> (deleted: Self)
            requires self.tree_is_bst(),
            ensures
                deleted.tree_is_bst(),
                !deleted.tree_contains(*target),
                forall|x: T| (#[trigger] deleted.tree_contains(x)) <==>
                    (self.tree_contains(x) && x != *target),
            ;
    }

    //		Section 9. impls


    impl<T: TotalOrder> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { self.root }
        open spec fn spec_bstbbalphasteph_wf(&self) -> bool { self.spec_root().tree_is_bst() }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn new() -> (tree: Self) {
            BSTBBAlphaStEph { root: BalBinTree::Leaf }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive tree traversal.
        fn size(&self) -> (n: usize) {
            self.root.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — leaf check.
        fn is_empty(&self) -> (b: bool) {
            self.root.is_leaf()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive tree traversal.
        fn height(&self) -> (h: usize) {
            self.root.height()
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn insert(self, value: T) -> (inserted: Self) {
            BSTBBAlphaStEph { root: self.root.insert_node(value) }
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn contains(&self, target: &T) -> (found: bool) {
            self.root.contains_node(target)
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn find(&self, target: &T) -> (found: Option<&T>) {
            self.root.find_node(target)
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — agrees with APAS.
        fn delete(self, target: &T) -> (deleted: Self) {
            BSTBBAlphaStEph { root: self.root.delete_node(target) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends leftmost path.
        fn minimum(&self) -> (min: Option<&T>) {
            self.root.min_node()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — descends rightmost path.
        fn maximum(&self) -> (max: Option<&T>) {
            self.root.max_node()
        }
    }

    impl<T: TotalOrder> BSTBBAlphaNodeFns<T> for BalBinTree<T> {

    fn insert_node(self, value: T) -> (inserted: Self)
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
            BalBinTree::Leaf => {
                let r = BalBinTree::Node(Box::new(BalBinNode {
                    left: BalBinTree::Leaf,
                    value: value,
                    right: BalBinTree::Leaf,
                }));
                r
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;

                match TotalOrder::cmp(&value, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = left.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: node_val,
                            right: right,
                        }));
                        // Veracity: NEEDED proof block
                        proof { lemma_bst_insert_left(node_val, old_left, old_right, node, new_left, r, value); }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = right.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: new_right,
                        // Veracity: NEEDED proof block
                        }));
                        proof { lemma_bst_insert_right(node_val, old_left, old_right, node, new_right, r, value); }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            // Veracity: NEEDED proof block
                            right: right,
                        }));
                        proof {
                            // Veracity: NEEDED assert
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                            };
                        }
                        r
                    }
                }
            }
        }
    }

    fn contains_node(&self, target: &T) -> (found: bool)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => false,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    // Veracity: NEEDED proof block
                    core::cmp::Ordering::Equal => true,
                    core::cmp::Ordering::Less => {
                        let r = inner.left.contains_node(target);
                        proof {
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        // Veracity: NEEDED proof block
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.contains_node(target);
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

    fn find_node(&self, target: &T) -> (found: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            // Veracity: NEEDED proof block
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => Some(&inner.value),
                    core::cmp::Ordering::Less => {
                        let r = inner.left.find_node(target);
                        proof {
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            // Veracity: NEEDED proof block
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.find_node(target);
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

    fn min_node(&self) -> (min: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.left.is_leaf() {
                    Some(&inner.value)
                } else {
                    inner.left.min_node()
                }
            }
        }
    }

    fn max_node(&self) -> (max: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() {
                    Some(&inner.value)
                } else {
                    inner.right.max_node()
                }
            }
        }
    }

    fn delete_min_node(self) -> (pair: (Self, T))
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
            BalBinTree::Leaf => {
                // Veracity: NEEDED proof block
                unreached()
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;
                if left.is_leaf() {
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|x: T| right.tree_contains(x) implies
                            x != node_val
                        by {
                            lemma_bst_right::<T>(old_left, node_val, old_right, x);
                        };

                        // Veracity: NEEDED assert
                        assert forall|x: T| node.tree_contains(x) implies
                            #[trigger] T::le(node_val, x)
                        by {
                            // Veracity: NEEDED assert
                            assert(node.tree_contains(x) ==
                                (node_val == x
                                || old_left.tree_contains(x)
                                || old_right.tree_contains(x)));
                            if x == node_val {
                                T::reflexive(node_val);
                            }
                        };

                        // Veracity: NEEDED assert
                        assert forall|x: T| old_right.tree_contains(x) ==
                            (node.tree_contains(x) && x != node_val)
                        by {
                            // Veracity: NEEDED assert
                            assert(node.tree_contains(x) ==
                                (node_val == x
                                || old_left.tree_contains(x)
                                || old_right.tree_contains(x)));
                        };
                    }
                    // Veracity: NEEDED proof block
                    (right, node_val)
                } else {
                    let (new_left, min_val) = left.delete_min_node();
                    let r = BalBinTree::Node(Box::new(BalBinNode {
                        left: new_left,
                        value: node_val,
                        right: right,
                    }));
                    proof {

                        // Veracity: NEEDED assert
                        assert forall|x: T| new_left.tree_contains(x) implies
                            #[trigger] T::le(x, node_val) && x != node_val
                        by {
                        };

                        // Veracity: NEEDED assert
                        assert forall|x: T| old_right.tree_contains(x) implies
                            #[trigger] T::le(node_val, x) && x != node_val
                        by {};


                        // Veracity: NEEDED assert
                        assert forall|x: T| node.tree_contains(x) implies
                            #[trigger] T::le(min_val, x)
                        by {
                            if old_left.tree_contains(x) {
                            } else if x == node_val {
                            } else {
                                T::transitive(min_val, node_val, x);
                            }
                        };

                        // Veracity: NEEDED assert
                        assert forall|x: T| r.tree_contains(x) ==
                            (node.tree_contains(x) && x != min_val)
                        by {
                            // Veracity: NEEDED assert
                            assert(r.tree_contains(x) ==
                                (node_val == x
                                || new_left.tree_contains(x)
                                || old_right.tree_contains(x)));
                            if x == min_val {
                                if old_right.tree_contains(min_val) {
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

    fn delete_node(self, target: &T) -> (deleted: Self)
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
            BalBinTree::Leaf => {
                BalBinTree::Leaf
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;
// Veracity: UNNEEDED proof block 
                match TotalOrder::cmp(target, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = left.delete_node(target);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: node_val,
                            right: right,
                        }));
                        proof { lemma_bst_delete_left(node_val, old_left, old_right, node, new_left, r, *target); }
                        // Veracity: NEEDED proof block
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = right.delete_node(target);
                        // Veracity: NEEDED proof block
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: new_right,
                        }));
                        proof { lemma_bst_delete_right(node_val, old_left, old_right, node, new_right, r, *target); }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        // Veracity: NEEDED proof block
                        if left.is_leaf() {
                            proof {
                                // Veracity: NEEDED assert
                                assert forall|x: T| old_right.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                by {
                                    lemma_node_contains::<T>(old_left, node_val, old_right, x);
                                };
                            }
                            right
                        } else if right.is_leaf() {
                            proof {
                                // Veracity: NEEDED assert
                                assert forall|x: T| old_left.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                // Veracity: NEEDED proof block
                                by {
                                    lemma_node_contains::<T>(old_left, node_val, old_right, x);
                                };
                            }
                            left
                        } else {
                            let (new_right, successor) = right.delete_min_node();
                            let r = BalBinTree::Node(Box::new(BalBinNode {
                                left: left,
                                value: successor,
                                right: new_right,
                            }));
                            proof {

                                // Veracity: NEEDED assert
                                assert forall|x: T| old_left.tree_contains(x) implies
                                    #[trigger] T::le(x, successor) && x != successor
                                by {
                                    T::transitive(x, node_val, successor);
                                    if x == successor {
                                        T::antisymmetric(x, node_val);
                                    }
                                };

                                // Veracity: NEEDED assert
                                assert forall|x: T| new_right.tree_contains(x) implies
                                    #[trigger] T::le(successor, x) && x != successor
                                by {
                                };

                                // Veracity: NEEDED assert
                                assert forall|x: T| r.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                by {
                                    // Veracity: NEEDED assert
                                    assert(r.tree_contains(x) ==
                                        (successor == x
                                        || old_left.tree_contains(x)
                                        || new_right.tree_contains(x)));

                                    if successor == x {
                                    }

                                    if old_right.tree_contains(x) && x != *target && x != successor {
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

    } // impl BSTBBAlphaNodeFns

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

    //		Section 14. derive impls outside verus!

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
