//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral weight-balanced (BB[α]) binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on BalBinTree) in sections 7/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//  1. module
//  2. imports
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

    use crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::{tree_is_bb, weight_balanced};
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 9. impls

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
    struct BSTBBAlphaMtEphInv<T> {
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

        fn new() -> (tree: Self)
            ensures tree.spec_bstbbalphamteph_wf(),
                    tree@.is_leaf();

        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstbbalphamteph_wf(),
            ensures self.spec_bstbbalphamteph_wf(),
                    match r {
                        Ok(_) => self@.tree_contains(value)
                            && forall|x: T| #![auto] self@.tree_contains(x) <==>
                                (old(self)@.tree_contains(x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstbbalphamteph_wf(),
            ensures found == self@.tree_contains(*target);

        fn size(&self) -> (n: usize)
            requires self.spec_bstbbalphamteph_wf(),
            ensures n as nat == self@.spec_size();

        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstbbalphamteph_wf(),
            ensures b == (self@ is Leaf);

        fn height(&self) -> (h: usize)
            requires self.spec_bstbbalphamteph_wf(),
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

    impl<T: TotalOrder> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {
        open spec fn spec_bstbbalphamteph_wf(&self) -> bool {
            self@.tree_is_bst()
            && self@.spec_size() <= usize::MAX
            && self@.spec_height() <= usize::MAX
        }

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
