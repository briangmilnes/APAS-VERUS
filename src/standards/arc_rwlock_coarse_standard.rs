//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Arc<RwLock> Coarse Standard: multi-struct BST behind a single coarse lock
//! with parallel readers and single writer.
//!
//! Demonstrates:
//! - Multi-struct spec style (Leaf, Interior, Node, Tree) with per-type traits.
//! - Recursive specs via bottom-up trait dispatch (no free spec fns).
//! - RwLockPredicate carrying a real BST ordering invariant.
//! - new_arc_rwlock / clone_arc_rwlock from vstdplus for pred() preservation.
//! - Parallel reads via HFScheduler join() with named closures.
//! - Write-then-parallel-read pattern.

// Table of Contents
// 1. module
// 2. imports
// 4. type definitions
// 7. proof fns
// 8. traits
// 9. impls
// 13. derive impls outside verus!

// 1. module

pub mod arc_rwlock_coarse_standard {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {

    // 4. type definitions

    pub struct Leaf {
        pub key: u64,
    }

    pub struct Interior {
        pub key: u64,
        pub left: Option<Box<Node>>,
        pub right: Option<Box<Node>>,
    }

    pub enum Node {
        LeafNode(Leaf),
        InteriorNode(Interior),
    }

    pub struct Tree {
        pub root: Option<Box<Node>>,
    }

    pub struct ArcRwLockCoarseStandardInv {
        pub ghost lo: int,
        pub ghost hi: int,
    }

    // 7. proof fns

    /// Every key in a well-formed subtree lies within [lo, hi).
    proof fn lemma_node_wf_bounds(n: &Node, lo: int, hi: int, k: u64)
        requires
            n.spec_wf(lo, hi),
            n.spec_contains(k),
        ensures
            lo <= k as int && (k as int) < hi,
        decreases *n,
    {
        match n {
            Node::LeafNode(_) => {},
            Node::InteriorNode(i) => {
                if k == i.key {
                } else if match &i.left {
                    None => false,
                    Some(n) => NodeTrait::spec_contains(&**n, k),
                } {
                    if let Some(n) = &i.left {
                        lemma_node_wf_bounds(&**n, lo, i.key as int, k);
                    }
                } else {
                    if let Some(n) = &i.right {
                        lemma_node_wf_bounds(&**n, i.key as int + 1, hi, k);
                    }
                }
            },
        }
    }

    // 8. traits

    pub trait LeafTrait: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_contains(&self, key: u64) -> bool;
        spec fn spec_wf(&self, lo: int, hi: int) -> bool;
    }

    pub trait InteriorTrait: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_contains(&self, key: u64) -> bool;
        spec fn spec_wf(&self, lo: int, hi: int) -> bool;
    }

    pub trait NodeTrait: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_contains(&self, key: u64) -> bool;
        spec fn spec_wf(&self, lo: int, hi: int) -> bool;

        fn insert(self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (out: Self)
            requires
                self.spec_wf(lo@, hi@),
                lo@ <= key as int,
                (key as int) < hi@,
            ensures
                out.spec_wf(lo@, hi@),
                out.spec_contains(key);

        fn search(&self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (found: bool)
            requires self.spec_wf(lo@, hi@),
            ensures found == self.spec_contains(key);
    }

    pub trait TreeTrait: Sized {
        spec fn spec_size(&self) -> nat;
        spec fn spec_contains(&self, key: u64) -> bool;
        spec fn spec_wf(&self, lo: int, hi: int) -> bool;

        fn insert(self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (out: Self)
            requires
                self.spec_wf(lo@, hi@),
                lo@ <= key as int,
                (key as int) < hi@,
            ensures
                out.spec_wf(lo@, hi@),
                out.spec_contains(key);

        fn search(&self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (found: bool)
            requires self.spec_wf(lo@, hi@),
            ensures found == self.spec_contains(key);
    }

    // 9. impls

    impl RwLockPredicate<Tree> for ArcRwLockCoarseStandardInv {
        open spec fn inv(self, t: Tree) -> bool {
            t.spec_wf(self.lo, self.hi)
        }
    }

    impl LeafTrait for Leaf {
        open spec fn spec_size(&self) -> nat { 1 }

        open spec fn spec_contains(&self, key: u64) -> bool {
            self.key == key
        }

        open spec fn spec_wf(&self, lo: int, hi: int) -> bool {
            lo <= self.key as int && (self.key as int) < hi
        }
    }

    impl InteriorTrait for Interior {
        open spec fn spec_size(&self) -> nat
            decreases self,
        {
            let l = match self.left  { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) };
            let r = match self.right { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) };
            1 + l + r
        }

        open spec fn spec_contains(&self, key: u64) -> bool
            decreases self,
        {
            self.key == key
            || match self.left  { None => false, Some(n) => NodeTrait::spec_contains(&*n, key) }
            || match self.right { None => false, Some(n) => NodeTrait::spec_contains(&*n, key) }
        }

        open spec fn spec_wf(&self, lo: int, hi: int) -> bool
            decreases self,
        {
            lo <= self.key as int && (self.key as int) < hi
            && match self.left  { None => true, Some(n) => NodeTrait::spec_wf(&*n, lo, self.key as int) }
            && match self.right { None => true, Some(n) => NodeTrait::spec_wf(&*n, self.key as int + 1, hi) }
        }
    }

    impl NodeTrait for Node {
        open spec fn spec_size(&self) -> nat
            decreases *self,
        {
            match *self {
                Node::LeafNode(l)     => LeafTrait::spec_size(&l),
                Node::InteriorNode(i) => InteriorTrait::spec_size(&i),
            }
        }

        open spec fn spec_contains(&self, key: u64) -> bool
            decreases *self,
        {
            match *self {
                Node::LeafNode(l)     => LeafTrait::spec_contains(&l, key),
                Node::InteriorNode(i) => InteriorTrait::spec_contains(&i, key),
            }
        }

        open spec fn spec_wf(&self, lo: int, hi: int) -> bool
            decreases *self,
        {
            match *self {
                Node::LeafNode(l)     => LeafTrait::spec_wf(&l, lo, hi),
                Node::InteriorNode(i) => InteriorTrait::spec_wf(&i, lo, hi),
            }
        }

        fn insert(self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (out: Self)
            decreases self,
        {
            match self {
                Node::LeafNode(l) => {
                    if key == l.key {
                        let out = Node::LeafNode(l);
                        assert(LeafTrait::spec_wf(&l, lo@, hi@));
                        assert(LeafTrait::spec_contains(&l, key));
                        out
                    } else if key < l.key {
                        let interior = Interior {
                            key: l.key,
                            left: Some(Box::new(Node::LeafNode(Leaf { key }))),
                            right: None,
                        };
                        assert(InteriorTrait::spec_wf(&interior, lo@, hi@));
                        assert(InteriorTrait::spec_contains(&interior, key));
                        Node::InteriorNode(interior)
                    } else {
                        let interior = Interior {
                            key: l.key,
                            left: None,
                            right: Some(Box::new(Node::LeafNode(Leaf { key }))),
                        };
                        assert(InteriorTrait::spec_wf(&interior, lo@, hi@));
                        assert(InteriorTrait::spec_contains(&interior, key));
                        Node::InteriorNode(interior)
                    }
                },
                Node::InteriorNode(i) => {
                    let Interior { key: node_key, left, right } = i;
                    if key == node_key {
                        let interior = Interior { key: node_key, left, right };
                        assert(InteriorTrait::spec_wf(&interior, lo@, hi@));
                        assert(InteriorTrait::spec_contains(&interior, key));
                        Node::InteriorNode(interior)
                    } else if key < node_key {
                        let new_left = match left {
                            None => Some(Box::new(Node::LeafNode(Leaf { key }))),
                            Some(n) => Some(Box::new(
                                (*n).insert(key, Ghost(lo@), Ghost(node_key as int)))),
                        };
                        let interior = Interior {
                            key: node_key, left: new_left, right,
                        };
                        assert(InteriorTrait::spec_wf(&interior, lo@, hi@));
                        assert(InteriorTrait::spec_contains(&interior, key));
                        Node::InteriorNode(interior)
                    } else {
                        let new_right = match right {
                            None => Some(Box::new(Node::LeafNode(Leaf { key }))),
                            Some(n) => Some(Box::new(
                                (*n).insert(key, Ghost(node_key as int + 1), Ghost(hi@)))),
                        };
                        let interior = Interior {
                            key: node_key, left, right: new_right,
                        };
                        assert(InteriorTrait::spec_wf(&interior, lo@, hi@));
                        assert(InteriorTrait::spec_contains(&interior, key));
                        Node::InteriorNode(interior)
                    }
                },
            }
        }

        fn search(&self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (found: bool)
            decreases *self,
        {
            match self {
                Node::LeafNode(l) => l.key == key,
                Node::InteriorNode(i) => {
                    if key == i.key {
                        true
                    } else if key < i.key {
                        let found = match &i.left {
                            None => false,
                            Some(n) => (**n).search(key,
                                Ghost(lo@), Ghost(i.key as int)),
                        };
                        proof {
                            if match &i.right {
                                None => false,
                                Some(n) => NodeTrait::spec_contains(&**n, key),
                            } {
                                if let Some(n) = &i.right {
                                    lemma_node_wf_bounds(&**n, i.key as int + 1, hi@, key);
                                }
                            }
                        }
                        found
                    } else {
                        let found = match &i.right {
                            None => false,
                            Some(n) => (**n).search(key,
                                Ghost(i.key as int + 1), Ghost(hi@)),
                        };
                        proof {
                            if match &i.left {
                                None => false,
                                Some(n) => NodeTrait::spec_contains(&**n, key),
                            } {
                                if let Some(n) = &i.left {
                                    lemma_node_wf_bounds(&**n, lo@, i.key as int, key);
                                }
                            }
                        }
                        found
                    }
                },
            }
        }
    }

    impl TreeTrait for Tree {
        open spec fn spec_size(&self) -> nat {
            match self.root { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) }
        }

        open spec fn spec_contains(&self, key: u64) -> bool {
            match self.root { None => false, Some(n) => NodeTrait::spec_contains(&*n, key) }
        }

        open spec fn spec_wf(&self, lo: int, hi: int) -> bool {
            match self.root { None => true, Some(n) => NodeTrait::spec_wf(&*n, lo, hi) }
        }

        fn insert(self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (out: Self) {
            let new_root = match self.root {
                None => Some(Box::new(Node::LeafNode(Leaf { key }))),
                Some(n) => Some(Box::new((*n).insert(key, lo, hi))),
            };
            Tree { root: new_root }
        }

        fn search(&self, key: u64, lo: Ghost<int>, hi: Ghost<int>) -> (found: bool) {
            match &self.root {
                None => false,
                Some(n) => (**n).search(key, lo, hi),
            }
        }
    }

    // Demonstration: parallel contains on a shared BST behind Arc<RwLock>.

    fn parallel_contains() {
        let ghost lo: int = i64::MIN as int;
        let ghost hi: int = i64::MAX as int;
        let tree = Tree { root: None };
        let tree = tree.insert(50, Ghost(lo), Ghost(hi));
        let tree = tree.insert(30, Ghost(lo), Ghost(hi));
        let tree = tree.insert(70, Ghost(lo), Ghost(hi));
        let ghost pred = ArcRwLockCoarseStandardInv { lo, hi };
        let arc = new_arc_rwlock::<Tree, ArcRwLockCoarseStandardInv>(tree, Ghost(pred));
        let arc1 = clone_arc_rwlock(&arc);
        let arc2 = clone_arc_rwlock(&arc);

        let f1 = move || -> (r: bool)
            requires arc1.pred() == pred
        {
            let handle = arc1.acquire_read();
            let tree = handle.borrow();
            let found = tree.search(30, Ghost(lo), Ghost(hi));
            handle.release_read();
            found
        };

        let f2 = move || -> (r: bool)
            requires arc2.pred() == pred
        {
            let handle = arc2.acquire_read();
            let tree = handle.borrow();
            let found = tree.search(70, Ghost(lo), Ghost(hi));
            handle.release_read();
            found
        };

        let (a, b) = join(f1, f2);
    }

    // Demonstration: write then parallel read.

    fn write_then_parallel_read() {
        let ghost lo: int = i64::MIN as int;
        let ghost hi: int = i64::MAX as int;
        let tree = Tree { root: None };
        let ghost pred = ArcRwLockCoarseStandardInv { lo, hi };
        let arc = new_arc_rwlock::<Tree, ArcRwLockCoarseStandardInv>(tree, Ghost(pred));

        // Single writer inserts a key.
        let (tree, write_handle) = arc.acquire_write();
        let tree = tree.insert(42, Ghost(lo), Ghost(hi));
        write_handle.release_write(tree);

        // Two parallel readers search for the key.
        let arc1 = clone_arc_rwlock(&arc);
        let arc2 = clone_arc_rwlock(&arc);

        let f1 = move || -> (r: bool)
            requires arc1.pred() == pred
        {
            let handle = arc1.acquire_read();
            let tree = handle.borrow();
            let found = tree.search(42, Ghost(lo), Ghost(hi));
            handle.release_read();
            found
        };

        let f2 = move || -> (r: bool)
            requires arc2.pred() == pred
        {
            let handle = arc2.acquire_read();
            let tree = handle.borrow();
            let found = tree.search(99, Ghost(lo), Ghost(hi));
            handle.release_read();
            found
        };

        let (a, b) = join(f1, f2);
    }

    } // verus!

// 13. derive impls outside verus!

    impl std::fmt::Debug for Leaf {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Leaf({})", self.key)
        }
    }

    impl std::fmt::Display for Leaf {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    impl std::fmt::Debug for Interior {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Interior")
                .field("key", &self.key)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl std::fmt::Display for Interior {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Interior({})", self.key)
        }
    }

    impl std::fmt::Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node::LeafNode(l) => write!(f, "{:?}", l),
                Node::InteriorNode(i) => write!(f, "{:?}", i),
            }
        }
    }

    impl std::fmt::Display for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node::LeafNode(l) => write!(f, "{}", l),
                Node::InteriorNode(i) => write!(f, "{}", i),
            }
        }
    }

    impl std::fmt::Debug for Tree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Tree").field("root", &self.root).finish()
        }
    }

    impl std::fmt::Display for Tree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Tree({:?})", self.root)
        }
    }

    impl std::fmt::Debug for ArcRwLockCoarseStandardInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArcRwLockCoarseStandardInv")
        }
    }

    impl std::fmt::Display for ArcRwLockCoarseStandardInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArcRwLockCoarseStandardInv")
        }
    }
}
