//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: three-layer Mt architecture — coarse lock + TSM + fork-join inside.
//!
//! Combines:
//!   Layer 1: Top-level RwLock for thread safety
//!   Layer 2: TSM token inside lock for zero assumes
//!   Layer 3: Fork-join parallelism inside lock on owned data
//!
//! Uses a simple tree type (not BalBinTree) to keep the experiment focused on
//! the architecture, not on BST proof obligations.
//!
//! Operations: new, insert_left, insert_right, size, sum, parallel_map.
//! Zero assumes. Parallel map via join() inside acquire_write.
//!
//! The key demonstration: acquire_write returns owned data. Destructure it,
//! fork-join on the parts, reassemble, step TSM, release. Parallelism inside
//! the lock with zero assumes and no nested locks.

pub mod coarse_lock_parallel_tsm {

    use vstd::prelude::*;
    use vstd::rwlock::{RwLock, RwLockPredicate};

    use verus_state_machines_macros::tokenized_state_machine;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;

    // ================================================================
    // State machine: tracks element count. Outside verus!.
    // ================================================================

    tokenized_state_machine!(
        TreeSM {
            fields {
                #[sharding(variable)]
                pub count: nat,
            }

            init!{
                initialize() {
                    init count = 0;
                }
            }

            transition!{
                tr_grow(new_count: nat) {
                    update count = new_count;
                }
            }

            // Read operations: no transition needed.

            #[invariant]
            pub fn the_invariant(&self) -> bool { true }

            #[inductive(initialize)]
            fn initialize_inductive(post: Self) { }

            #[inductive(tr_grow)]
            fn tr_grow_inductive(pre: Self, post: Self, new_count: nat) { }
        }
    );

    verus! {

    // ================================================================
    // Inner type: simple binary tree of u64. No balancing, no BST order.
    // This is the St layer — fully verified, no locks.
    // ================================================================

    pub enum Tree {
        Leaf,
        Node { left: Box<Tree>, val: u64, right: Box<Tree> },
    }

    impl Tree {
        pub open spec fn spec_size(&self) -> nat
            decreases *self,
        {
            match *self {
                Tree::Leaf => 0,
                Tree::Node { left, val: _, right } => 1 + left.spec_size() + right.spec_size(),
            }
        }

        pub open spec fn spec_sum(&self) -> int
            decreases *self,
        {
            match *self {
                Tree::Leaf => 0,
                Tree::Node { left, val, right } => left.spec_sum() + val as int + right.spec_sum(),
            }
        }

        pub fn size(&self) -> (n: usize)
            requires self.spec_size() <= usize::MAX,
            ensures n as nat == self.spec_size(),
            decreases *self,
        {
            match self {
                Tree::Leaf => 0,
                Tree::Node { left, val: _, right } => {
                    1 + left.size() + right.size()
                }
            }
        }

        pub fn sum(&self) -> (s: u64)
            decreases *self,
        {
            match self {
                Tree::Leaf => 0,
                Tree::Node { left, val, right } => {
                    left.sum().wrapping_add(*val).wrapping_add(right.sum())
                }
            }
        }

        // Map: apply f to every value. Returns a new tree with same shape.
        pub fn map_tree<F: Fn(u64) -> u64>(self, f: &F) -> (result: Tree)
            requires forall|x: &u64| #[trigger] f.requires((x,)),
            ensures result.spec_size() == self.spec_size(),
            decreases self.spec_size(),
        {
            match self {
                Tree::Leaf => Tree::Leaf,
                Tree::Node { left, val, right } => {
                    let new_val = f(&val);
                    let new_left = (*left).map_tree(f);
                    let new_right = (*right).map_tree(f);
                    Tree::Node {
                        left: Box::new(new_left),
                        val: new_val,
                        right: Box::new(new_right),
                    }
                }
            }
        }
    }

    // ================================================================
    // Lock interior: data + TSM token.
    // ================================================================

    pub struct TreeLockInterior {
        pub tree: Tree,
        pub ghost_count: Tracked<TreeSM::count>,
    }

    pub ghost struct TreeMtInv {
        pub instance: TreeSM::Instance,
    }

    impl RwLockPredicate<TreeLockInterior> for TreeMtInv {
        open spec fn inv(self, interior: TreeLockInterior) -> bool {
            interior.tree.spec_size() == interior.ghost_count@.value()
            && interior.ghost_count@.instance_id() == self.instance.id()
            && interior.tree.spec_size() <= usize::MAX
        }
    }

    // ================================================================
    // Mt type: coarse lock + TSM instance. No ghost field on the struct.
    // ================================================================

    pub struct TreeMt {
        pub lock: RwLock<TreeLockInterior, TreeMtInv>,
        pub inst: Tracked<TreeSM::Instance>,
    }

    impl TreeMt {
        pub open spec fn wf(&self) -> bool {
            self.lock.pred().instance == self.inst@
        }

        pub fn new_empty() -> (s: Self)
            ensures s.wf(),
        {
            let tracked (
                Tracked(instance),
                Tracked(count_token),
            ) = TreeSM::Instance::initialize();

            let interior = TreeLockInterior {
                tree: Tree::Leaf,
                ghost_count: Tracked(count_token),
            };

            TreeMt {
                lock: RwLock::new(interior, Ghost(TreeMtInv { instance })),
                inst: Tracked(instance),
            }
        }

        // ============================================================
        // Read operations: acquire_read, compute, release. No TSM step.
        // Zero assumes.
        // ============================================================

        pub fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let n = interior.tree.size();
            read_handle.release_read();
            n
        }

        pub fn mt_sum(&self) -> (s: u64)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let s = interior.tree.sum();
            read_handle.release_read();
            s
        }

        // ============================================================
        // Write operation (simple): acquire_write, mutate, step TSM, release.
        // Zero assumes.
        // ============================================================

        pub fn mt_insert_left(&self, val: u64) -> (r: Result<(), ()>)
            requires self.wf(),
        {
            let (interior, write_handle) = self.lock.acquire_write();

            // Destructure — we own everything.
            let TreeLockInterior { tree, ghost_count } = interior;
            let tracked mut token = ghost_count.get();

            // Capacity check (exec-time guard).
            let sz = tree.size();
            if sz >= usize::MAX - 1 {
                // Reassemble and bail.
                proof { self.inst.borrow().tr_grow(sz as nat, &mut token); }
                write_handle.release_write(TreeLockInterior {
                    tree, ghost_count: Tracked(token),
                });
                return Err(());
            }

            // Build new tree: insert val as new root, old tree becomes left child.
            let new_tree = Tree::Node {
                left: Box::new(tree),
                val,
                right: Box::new(Tree::Leaf),
            };

            // Step TSM.
            let new_sz = new_tree.size();
            proof {
                self.inst.borrow().tr_grow(new_sz as nat, &mut token);
            }

            // Release with new data + updated token.
            write_handle.release_write(TreeLockInterior {
                tree: new_tree,
                ghost_count: Tracked(token),
            });
            Ok(())
        }

        // ============================================================
        // LAYER 3: Parallel write operation.
        //
        // acquire_write → own the data → destructure tree →
        // join(map_left, map_right) → reassemble → step TSM → release.
        //
        // Fork-join INSIDE the lock. No nested locks. No unsafe.
        // Zero assumes.
        // ============================================================

        pub fn mt_parallel_map(&self, f: &(impl Fn(u64) -> u64 + Send + Sync + Clone + 'static))
            -> (r: Result<(), ()>)
            requires
                self.wf(),
                forall|x: &u64| #[trigger] f.requires((x,)),
        {
            let (interior, write_handle) = self.lock.acquire_write();

            // Destructure — we own everything.
            let TreeLockInterior { tree, ghost_count } = interior;
            let tracked mut token = ghost_count.get();

            match tree {
                Tree::Leaf => {
                    // Nothing to map. Release unchanged.
                    write_handle.release_write(TreeLockInterior {
                        tree: Tree::Leaf,
                        ghost_count: Tracked(token),
                    });
                    Ok(())
                }
                Tree::Node { left, val, right } => {
                    // Apply f to root.
                    let new_val = f(&val);

                    // Fork-join on owned subtrees. Each closure owns its subtree.
                    let f1 = f.clone();
                    let f2 = f.clone();

                    let ghost left_size = left.spec_size();
                    let ghost right_size = right.spec_size();

                    let (mapped_left, mapped_right) = join(
                        move || -> (result: Tree)
                            ensures result.spec_size() == left_size
                        { (*left).map_tree(&f1) },
                        move || -> (result: Tree)
                            ensures result.spec_size() == right_size
                        { (*right).map_tree(&f2) },
                    );

                    // Reassemble.
                    let new_tree = Tree::Node {
                        left: Box::new(mapped_left),
                        val: new_val,
                        right: Box::new(mapped_right),
                    };

                    // Step TSM — size unchanged by map.
                    let new_sz = new_tree.spec_size();
                    proof {
                        assert(new_sz == 1 + left_size + right_size);
                        self.inst.borrow().tr_grow(new_sz, &mut token);
                    }

                    // Release with mapped data + updated token.
                    write_handle.release_write(TreeLockInterior {
                        tree: new_tree,
                        ghost_count: Tracked(token),
                    });
                    Ok(())
                }
            }
        }
    }

    } // verus!

    #[test]
    fn test_coarse_lock_parallel_tsm() {
        let t = TreeMt::new_empty();
        assert_eq!(t.mt_size(), 0);
        assert_eq!(t.mt_sum(), 0);

        // Build a small tree: insert_left chains leftward.
        t.mt_insert_left(10).unwrap();
        t.mt_insert_left(20).unwrap();
        t.mt_insert_left(30).unwrap();
        assert_eq!(t.mt_size(), 3);
        assert_eq!(t.mt_sum(), 60);

        // Parallel map: double every value.
        t.mt_parallel_map(&|x: u64| x * 2).unwrap();
        assert_eq!(t.mt_size(), 3);  // size unchanged
        assert_eq!(t.mt_sum(), 120); // sum doubled
    }
}
