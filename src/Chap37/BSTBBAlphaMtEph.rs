//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Ephemeral weight-balanced (BB[α]) binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on BalBinTree) in sections 7/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//  1. module
//  2. imports
//  8. traits
//  9. impls
//  11. top level coarse locking
//  13. macros
//  14. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTBBAlphaMtEph {

    use core::marker::PhantomData;

    use vstd::prelude::*;
    use vstd::rwlock::{ReadHandle, RwLock, RwLockPredicate, WriteHandle};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;

    verus! {

    // 2. imports

    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    // 8. traits

    /// Exec BST operations on BalBinTree nodes (BB[α] Mt variant).
    pub trait BSTBBAlphaMtNodeFns<T: TotalOrder>: Sized + BSTSpecFns<T> + BalBinTreeTrait<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn insert_node(self, value: T) -> (inserted: Self)
            requires self.tree_is_bst(),
            ensures
                inserted.tree_is_bst(),
                inserted.tree_contains(value),
                forall|x: T| (#[trigger] inserted.tree_contains(x)) <==>
                    (self.tree_contains(x) || x == value),
                inserted.spec_size() <= self.spec_size() + 1,
                inserted.spec_height() <= self.spec_height() + 1,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains_node(&self, target: &T) -> (found: bool)
            requires (*self).tree_is_bst(),
            ensures found == (*self).tree_contains(*target),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find_node(&self, target: &T) -> (found: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                found.is_some() == (*self).tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn min_node(&self) -> (min: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> min.is_none(),
                (*self).spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> (*self).tree_contains(*min.unwrap()),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn max_node(&self) -> (max: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> max.is_none(),
                (*self).spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> (*self).tree_contains(*max.unwrap()),
            ;
        /// Remove and return the minimum element from a non-empty BST subtree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
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
                pair.0.spec_size() < self.spec_size(),
                pair.0.spec_height() <= self.spec_height(),
            ;
        /// Delete a key from the BST, returning the modified tree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn delete_node(self, target: &T) -> (deleted: Self)
            requires self.tree_is_bst(),
            ensures
                deleted.tree_is_bst(),
                !deleted.tree_contains(*target),
                forall|x: T| (#[trigger] deleted.tree_contains(x)) <==>
                    (self.tree_contains(x) && x != *target),
                deleted.spec_size() <= self.spec_size(),
                deleted.spec_height() <= self.spec_height(),
            ;
    }

    // 9. impls

    impl<T: TotalOrder> BSTBBAlphaMtNodeFns<T> for BalBinTree<T> {

    fn insert_node(self, value: T) -> (inserted: Self)
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
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
                        let new_left = left.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left, value: node_val, right: right,
                        }));
                        proof {
                            assert(new_left.tree_is_bst());
                            assert(old_right.tree_is_bst());
                            assert forall|x: T| new_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by { if old_left.tree_contains(x) {} else { assert(x == value); } };
                            assert forall|x: T| old_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val by {};
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) == (node_val == x
                                    || new_left.tree_contains(x) || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = right.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: new_right,
                        }));
                        proof {
                            assert(old_left.tree_is_bst());
                            assert(new_right.tree_is_bst());
                            assert forall|x: T| old_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val by {};
                            assert forall|x: T| new_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by { if old_right.tree_contains(x) {} else { assert(x == value); } };
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || new_right.tree_contains(x)));
                                assert(node.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                            };
                        }
                        r
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: right,
                        }));
                        proof {
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                                assert(value == node_val);
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
                    core::cmp::Ordering::Equal => true,
                    core::cmp::Ordering::Less => {
                        let r = inner.left.contains_node(target);
                        proof { if inner.right.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.contains_node(target);
                        proof { if inner.left.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
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
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => Some(&inner.value),
                    core::cmp::Ordering::Less => {
                        let r = inner.left.find_node(target);
                        proof { if inner.right.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.find_node(target);
                        proof { if inner.left.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
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
                if inner.left.is_leaf() { Some(&inner.value) }
                else { inner.left.min_node() }
            }
        }
    }

    fn max_node(&self) -> (max: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() { Some(&inner.value) }
                else { inner.right.max_node() }
            }
        }
    }

    // Verified BST delete (Layer 1).

    fn delete_min_node(self) -> (pair: (Self, T))
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
            BalBinTree::Leaf => {
                proof { assert(false); }
                vstd::pervasive::unreached()
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;
                if left.is_leaf() {
                    proof {
                        assert forall|x: T| right.tree_contains(x) implies
                            x != node_val
                        by {};

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
                    let (new_left, min_val) = left.delete_min_node();
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

                match TotalOrder::cmp(target, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = left.delete_node(target);
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
                        let new_right = right.delete_node(target);
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
                                    assert(node.tree_contains(x) ==
                                        (node_val == x
                                        || old_left.tree_contains(x)
                                        || old_right.tree_contains(x)));
                                };
                            }
                            right
                        } else if right.is_leaf() {
                            proof {
                                assert forall|x: T| old_left.tree_contains(x) ==
                                    (node.tree_contains(x) && x != *target)
                                by {
                                    assert(node.tree_contains(x) ==
                                        (node_val == x
                                        || old_left.tree_contains(x)
                                        || old_right.tree_contains(x)));
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

    } // impl BSTBBAlphaMtNodeFns

    // 11. top level coarse locking

    /// Lock predicate: the inner tree satisfies BST ordering and fits in usize.
    pub struct BSTBBAlphaMtEphInv<T> {
        _phantom: PhantomData<T>,
    }

    impl<T: TotalOrder> RwLockPredicate<BalBinTree<T>> for BSTBBAlphaMtEphInv<T> {
        open spec fn inv(self, tree: BalBinTree<T>) -> bool {
            tree.tree_is_bst()
                && tree.spec_size() <= usize::MAX
                && tree.spec_height() <= usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTBBAlphaMtEph<T: TotalOrder> {
        pub(crate) root: RwLock<BalBinTree<T>, BSTBBAlphaMtEphInv<T>>,
        pub(crate) ghost_root: Ghost<BalBinTree<T>>,
    }

    impl<T: TotalOrder> BSTBBAlphaMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_root@.tree_is_bst()
            && self.ghost_root@.spec_size() <= usize::MAX
            && self.ghost_root@.spec_height() <= usize::MAX
        }

        pub closed spec fn spec_ghost_root(self) -> BalBinTree<T> {
            self.ghost_root@
        }
    }

    impl<T: TotalOrder> View for BSTBBAlphaMtEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.spec_ghost_root() }
    }

    pub trait BSTBBAlphaMtEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_bstbbalphamteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_bstbbalphamteph_wf(),
                    tree@.spec_is_leaf(),
                    tree@.tree_is_bst(),
                    forall|x: T| !tree@.tree_contains(x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstbbalphamteph_wf(),
            ensures self.spec_bstbbalphamteph_wf(),
                    match r {
                        Ok(_) => self@.tree_contains(value)
                            && forall|x: T| (#[trigger] self@.tree_contains(x)) <==>
                                (old(self)@.tree_contains(x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn delete(&mut self, target: &T) -> (r: Result<(), ()>)
            requires old(self).spec_bstbbalphamteph_wf(),
            ensures self.spec_bstbbalphamteph_wf(),
                    match r {
                        Ok(_) => !self@.tree_contains(*target)
                            && forall|x: T| (#[trigger] self@.tree_contains(x)) <==>
                                (old(self)@.tree_contains(x) && x != *target),
                        Err(_) => self@ == old(self)@,
                    };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstbbalphamteph_wf(),
            ensures found == self@.tree_contains(*target);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize)
            requires self.spec_bstbbalphamteph_wf(),
            ensures n as nat == self@.spec_size();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstbbalphamteph_wf(),
            ensures b == (self@ is Leaf);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires self.spec_bstbbalphamteph_wf(),
            ensures h as nat == self@.spec_height();

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — matches APAS
        fn find(&self, target: &T) -> (found: Option<T>) where T: Clone + Eq
            requires self.spec_bstbbalphamteph_wf(),
            ensures
                found.is_some() == self@.tree_contains(*target),
                found.is_some() ==> found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>) where T: Clone + Eq
            requires self.spec_bstbbalphamteph_wf(),
            ensures
                self@.spec_size() == 0 ==> min.is_none(),
                self@.spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> self@.tree_contains(min.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>) where T: Clone + Eq
            requires self.spec_bstbbalphamteph_wf(),
            ensures
                self@.spec_size() == 0 ==> max.is_none(),
                self@.spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> self@.tree_contains(max.unwrap());
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>) where T: Clone + Eq
            requires self.spec_bstbbalphamteph_wf(), obeys_feq_clone::<T>(),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>) where T: Clone + Eq
            requires self.spec_bstbbalphamteph_wf(), obeys_feq_clone::<T>(),
            ensures true;
    }

    impl<T: TotalOrder> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {
        open spec fn spec_bstbbalphamteph_wf(&self) -> bool {
            self@.tree_is_bst()
            && self@.spec_size() <= usize::MAX
            && self@.spec_height() <= usize::MAX
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self) {
            BSTBBAlphaMtEph {
                root: RwLock::new(
                    BalBinTree::Leaf,
                    Ghost(BSTBBAlphaMtEphInv { _phantom: PhantomData }),
                ),
                ghost_root: Ghost(BalBinTree::Leaf),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (r: Result<(), ()>) {
            let (tree, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == tree); }
            let current_size = tree.size();
            let current_height = tree.height();
            if current_size < usize::MAX && current_height < usize::MAX {
                let new_tree = tree.insert_node(value);
                proof {
                    assert(new_tree.spec_size() <= usize::MAX);
                    assert(new_tree.spec_height() <= usize::MAX);
                }
                let ghost new_root = new_tree;
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(new_tree);
                Ok(())
            } else {
                write_handle.release_write(tree);
                Err(())
            }
        }

        // Writer: assume ghost == inner, delete always succeeds (no capacity check needed).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn delete(&mut self, target: &T) -> (r: Result<(), ()>) {
            let (tree, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == tree); }
            let new_tree = tree.delete_node(target);
            let ghost new_root = new_tree;
            self.ghost_root = Ghost(new_root);
            write_handle.release_write(new_tree);
            Ok(())
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn contains(&self, target: &T) -> (found: bool) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = tree_ref.contains_node(target);
            proof { assume(found == self@.tree_contains(*target)); }
            read_handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_size() <= usize::MAX);
            let n = tree_ref.size();
            proof { assume(n as nat == self@.spec_size()); }
            read_handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let b = tree_ref.is_leaf();
            proof { assume(b == (self@ is Leaf)); }
            read_handle.release_read();
            b
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_height() <= usize::MAX);
            let h = tree_ref.height();
            proof { assume(h as nat == self@.spec_height()); }
            read_handle.release_read();
            h
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<T>) where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = tree_ref.find_node(target).cloned();
            proof {
                assume(found.is_some() == self@.tree_contains(*target));
                accept(found.is_some() ==> found.unwrap() == *target);
            }
            read_handle.release_read();
            found
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn minimum(&self) -> (min: Option<T>) where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let min = tree_ref.min_node().cloned();
            proof {
                assume(self@.spec_size() == 0 ==> min.is_none());
                assume(self@.spec_size() > 0 ==> min.is_some());
                assume(min.is_some() ==> self@.tree_contains(min.unwrap()));
            }
            read_handle.release_read();
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn maximum(&self) -> (max: Option<T>) where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let max = tree_ref.max_node().cloned();
            proof {
                assume(self@.spec_size() == 0 ==> max.is_none());
                assume(self@.spec_size() > 0 ==> max.is_some());
                assume(max.is_some() ==> self@.tree_contains(max.unwrap()));
            }
            read_handle.release_read();
            max
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let out = tree_ref.in_order();
            read_handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> ArraySeqStPerS<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let out = tree_ref.pre_order();
            read_handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }
    }

    } // verus!

    // 13. macros

    #[macro_export]
    macro_rules! BSTBBAlphaMtEphLit {
        () => {
            < $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::BSTBBAlphaMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }

    // 14. derive impls outside verus!

    impl<T> std::fmt::Debug for BSTBBAlphaMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTBBAlphaMtEphInv").finish()
        }
    }

    impl<T> std::fmt::Display for BSTBBAlphaMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTBBAlphaMtEphInv")
        }
    }

    impl<T: TotalOrder> std::fmt::Debug for BSTBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTBBAlphaMtEph").finish()
        }
    }

    impl<T: TotalOrder> std::fmt::Display for BSTBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTBBAlphaMtEph(size={})", self.size())
        }
    }
} // mod
