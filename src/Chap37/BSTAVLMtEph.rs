//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral AVL-balanced binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on BalBinTree) in sections 7/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//  1. module
//  2. imports
//  7. proof fns/broadcast groups
//  9. impls
//  11. top level coarse locking
//  13. macros
//  14. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTAVLMtEph {

    use core::marker::PhantomData;

    use vstd::prelude::*;
    use vstd::rwlock::{ReadHandle, RwLock, RwLockPredicate, WriteHandle};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;

    verus! {

    // 2. imports

    use crate::Chap37::BSTAVLStEph::BSTAVLStEph::{avl_balanced, tree_is_avl};
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 7. proof fns/broadcast groups

    proof fn lemma_bst_deep<T: TotalOrder>(tree: BalBinTree<T>)
        requires tree.tree_is_bst(),
        ensures
            match tree {
                BalBinTree::Leaf => true,
                BalBinTree::Node(node) =>
                    node.left.tree_is_bst()
                    && node.right.tree_is_bst()
                    && (forall|x: T| #![auto] node.left.tree_contains(x) ==>
                        T::le(x, node.value) && x != node.value)
                    && (forall|x: T| #![auto] node.right.tree_contains(x) ==>
                        T::le(node.value, x) && x != node.value)
                    && match node.left {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(lnode) =>
                            lnode.left.tree_is_bst()
                            && lnode.right.tree_is_bst()
                            && (forall|x: T| #![auto] lnode.left.tree_contains(x) ==>
                                T::le(x, lnode.value) && x != lnode.value)
                            && (forall|x: T| #![auto] lnode.right.tree_contains(x) ==>
                                T::le(lnode.value, x) && x != lnode.value)
                    }
                    && match node.right {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(rnode) =>
                            rnode.left.tree_is_bst()
                            && rnode.right.tree_is_bst()
                            && (forall|x: T| #![auto] rnode.left.tree_contains(x) ==>
                                T::le(x, rnode.value) && x != rnode.value)
                            && (forall|x: T| #![auto] rnode.right.tree_contains(x) ==>
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

    // 9. impls

    // Verified rotations (Layer 1 algorithms on BalBinTree).

    fn rotate_right<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires tree.tree_is_bst(), !(tree is Leaf),
        ensures
            rotated.tree_is_bst(),
            forall|x: T| #![auto] rotated.tree_contains(x) == tree.tree_contains(x),
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
                            assert forall|z: T| old_lr.tree_contains(z) implies
                                T::le(z, y_val) && z != y_val
                            by { assert(old_left.tree_contains(z)); };
                            assert(old_left.tree_contains(x_val));
                            assert(x_val != y_val);
                            assert(right_sub.tree_is_bst());
                            assert forall|z: T| right_sub.tree_contains(z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                if old_lr.tree_contains(z) {}
                                else if z == y_val { assert(x_val != y_val); }
                                else if old_r.tree_contains(z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val { T::antisymmetric(x_val, y_val); }
                                }
                            };
                            assert forall|z: T| r.tree_contains(z) == tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) == (x_val == z
                                    || old_ll.tree_contains(z) || right_sub.tree_contains(z)));
                                assert(right_sub.tree_contains(z) == (y_val == z
                                    || old_lr.tree_contains(z) || old_r.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) == (y_val == z
                                    || old_left.tree_contains(z) || old_r.tree_contains(z)));
                                assert(old_left.tree_contains(z) == (x_val == z
                                    || old_ll.tree_contains(z) || old_lr.tree_contains(z)));
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

    fn rotate_left<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires tree.tree_is_bst(), !(tree is Leaf),
        ensures
            rotated.tree_is_bst(),
            forall|x: T| #![auto] rotated.tree_contains(x) == tree.tree_contains(x),
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
                            assert forall|z: T| old_rl.tree_contains(z) implies
                                T::le(x_val, z) && z != x_val
                            by { assert(old_right.tree_contains(z)); };
                            assert(old_right.tree_contains(y_val));
                            assert(x_val != y_val);
                            assert(left_sub.tree_is_bst());
                            assert forall|z: T| left_sub.tree_contains(z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                if old_l.tree_contains(z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val { T::antisymmetric(x_val, y_val); }
                                } else if z == x_val { assert(x_val != y_val); }
                                else if old_rl.tree_contains(z) {}
                            };
                            assert forall|z: T| r.tree_contains(z) == tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) == (y_val == z
                                    || left_sub.tree_contains(z) || old_rr.tree_contains(z)));
                                assert(left_sub.tree_contains(z) == (x_val == z
                                    || old_l.tree_contains(z) || old_rl.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) == (x_val == z
                                    || old_l.tree_contains(z) || old_right.tree_contains(z)));
                                assert(old_right.tree_contains(z) == (y_val == z
                                    || old_rl.tree_contains(z) || old_rr.tree_contains(z)));
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

    // Verified BST insert (Layer 1).

    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
        requires node.tree_is_bst(),
        ensures
            inserted.tree_is_bst(),
            inserted.tree_contains(value),
            forall|x: T| #![auto] inserted.tree_contains(x) <==>
                (node.tree_contains(x) || x == value),
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
                            assert(new_left.tree_is_bst());
                            assert(old_right.tree_is_bst());
                            assert forall|x: T| new_left.tree_contains(x) implies
                                T::le(x, node_val) && x != node_val
                            by { if old_left.tree_contains(x) {} else { assert(x == value); } };
                            assert forall|x: T| old_right.tree_contains(x) implies
                                T::le(node_val, x) && x != node_val by {};
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
                        let new_right = insert_node(right, value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: new_right,
                        }));
                        proof {
                            assert(old_left.tree_is_bst());
                            assert(new_right.tree_is_bst());
                            assert forall|x: T| old_left.tree_contains(x) implies
                                T::le(x, node_val) && x != node_val by {};
                            assert forall|x: T| new_right.tree_contains(x) implies
                                T::le(node_val, x) && x != node_val
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
                        proof { if inner.right.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = contains_node(&inner.right, target);
                        proof { if inner.left.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                }
            }
        }
    }

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
                        proof { if inner.right.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = find_node(&inner.right, target);
                        proof { if inner.left.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                }
            }
        }
    }

    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (min: Option<&T>)
        requires true,
        ensures
            (node is Leaf) ==> min is None,
            (node is Node) ==> min is Some,
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
        requires true,
        ensures
            (node is Leaf) ==> max is None,
            (node is Node) ==> max is Some,
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

    // 11. top level coarse locking

    /// Lock predicate: the inner tree satisfies BST ordering and fits in usize.
    struct BSTAVLMtEphInv<T> {
        _phantom: PhantomData<T>,
    }

    impl<T: TotalOrder> RwLockPredicate<BalBinTree<T>> for BSTAVLMtEphInv<T> {
        open spec fn inv(self, tree: BalBinTree<T>) -> bool {
            tree.tree_is_bst()
                && tree.spec_size() <= usize::MAX
                && tree.spec_height() <= usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTAVLMtEph<T: TotalOrder> {
        pub(crate) root: RwLock<BalBinTree<T>, BSTAVLMtEphInv<T>>,
        pub(crate) ghost_root: Ghost<BalBinTree<T>>,
    }

    impl<T: TotalOrder> BSTAVLMtEph<T> {
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

    impl<T: TotalOrder> View for BSTAVLMtEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.spec_ghost_root() }
    }

    pub trait BSTAVLMtEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_bstavlmteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures tree.spec_bstavlmteph_wf(),
                    tree@.is_leaf();

        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstavlmteph_wf(),
            ensures self.spec_bstavlmteph_wf(),
                    match r {
                        Ok(_) => self@.tree_contains(value)
                            && forall|x: T| #![auto] self@.tree_contains(x) <==>
                                (old(self)@.tree_contains(x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstavlmteph_wf(),
            ensures found == self@.tree_contains(*target);

        fn size(&self) -> (n: usize)
            requires self.spec_bstavlmteph_wf(),
            ensures n as nat == self@.spec_size();

        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstavlmteph_wf(),
            ensures b == (self@ is Leaf);

        fn height(&self) -> (h: usize)
            requires self.spec_bstavlmteph_wf(),
            ensures h as nat == self@.spec_height();

        fn find(&self, target: &T) -> (found: Option<T>) where T: Clone + Eq
            ensures true;
        fn minimum(&self) -> (min: Option<T>) where T: Clone + Eq
            ensures true;
        fn maximum(&self) -> (max: Option<T>) where T: Clone + Eq
            ensures true;
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>) where T: Clone + Eq
            ensures true;
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>) where T: Clone + Eq
            ensures true;
    }

    impl<T: TotalOrder> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {
        open spec fn spec_bstavlmteph_wf(&self) -> bool {
            self@.tree_is_bst()
            && self@.spec_size() <= usize::MAX
            && self@.spec_height() <= usize::MAX
        }

        fn new() -> (tree: Self) {
            BSTAVLMtEph {
                root: RwLock::new(
                    BalBinTree::Leaf,
                    Ghost(BSTAVLMtEphInv { _phantom: PhantomData }),
                ),
                ghost_root: Ghost(BalBinTree::Leaf),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        fn insert(&mut self, value: T) -> (r: Result<(), ()>) {
            let (tree, write_handle) = self.root.acquire_write();
            proof { accept(self.ghost_root@ == tree); }
            let current_size = tree.size();
            let current_height = tree.height();
            if current_size < usize::MAX && current_height < usize::MAX {
                let new_tree = insert_node(tree, value);
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

        // Reader: assume return value matches ghost.
        fn contains(&self, target: &T) -> (found: bool) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = contains_node(tree_ref, target);
            proof { accept(found == self@.tree_contains(*target)); }
            read_handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        fn size(&self) -> (n: usize) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_size() <= usize::MAX);
            let n = tree_ref.size();
            proof { accept(n as nat == self@.spec_size()); }
            read_handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        fn is_empty(&self) -> (b: bool) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let b = tree_ref.is_leaf();
            proof { accept(b == (self@ is Leaf)); }
            read_handle.release_read();
            b
        }

        // Reader: assume return value matches ghost.
        fn height(&self) -> (h: usize) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_height() <= usize::MAX);
            let h = tree_ref.height();
            proof { accept(h as nat == self@.spec_height()); }
            read_handle.release_read();
            h
        }

        fn find(&self, target: &T) -> Option<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = find_node(tree_ref, target).cloned();
            read_handle.release_read();
            found
        }

        fn minimum(&self) -> Option<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let min = min_node(tree_ref).cloned();
            read_handle.release_read();
            min
        }

        fn maximum(&self) -> Option<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let max = max_node(tree_ref).cloned();
            read_handle.release_read();
            max
        }

        fn in_order(&self) -> ArraySeqStPerS<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let out = tree_ref.in_order();
            read_handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let out = tree_ref.pre_order();
            read_handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }
    }

    } // verus!

    // 14. derive impls outside verus!

    impl<T> std::fmt::Debug for BSTAVLMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTAVLMtEphInv").finish()
        }
    }

    impl<T> std::fmt::Display for BSTAVLMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTAVLMtEphInv")
        }
    }

    impl<T: TotalOrder> std::fmt::Debug for BSTAVLMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTAVLMtEph").finish()
        }
    }

    impl<T: TotalOrder> std::fmt::Display for BSTAVLMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTAVLMtEph(size={})", self.size())
        }
    }

    // 13. macros

    #[macro_export]
    macro_rules! BSTAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }
} // mod
