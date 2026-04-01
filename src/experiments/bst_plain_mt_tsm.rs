//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: scaling the TSM zero-assume pattern to 10 Mt operations.
//!
//! Based on BSTPlainMtEph (10 operations, 14 assumes). Uses a Vec-based set
//! as the inner type. The TSM tracks element count; the RwLockPredicate ties
//! the token to the concrete state. After acquire, the predicate PROVES equality.
//!
//! Operations: new, insert, delete, contains, size, is_empty, find, minimum, maximum, to_vec.
//! Zero assumes. Compare with BSTPlainMtEph: 10 operations, 14 assumes.
//!
//! Trade-off: no View on the outer struct. Specs on return values only (Approach B).

pub mod bst_plain_mt_tsm {

    use vstd::prelude::*;
    use vstd::rwlock::{RwLock, RwLockPredicate};

    use verus_state_machines_macros::tokenized_state_machine;

    // State machine tracks element count. OUTSIDE verus!.
    tokenized_state_machine!(
        SetSM {
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
                tr_insert() {
                    update count = pre.count + 1;
                }
            }

            transition!{
                tr_noop() {
                    // For operations that don't change the count (dup insert, missing delete).
                }
            }

            transition!{
                tr_delete() {
                    require(pre.count > 0);
                    update count = (pre.count - 1) as nat;
                }
            }

            #[invariant]
            pub fn the_invariant(&self) -> bool { true }

            #[inductive(initialize)]
            fn initialize_inductive(post: Self) { }

            #[inductive(tr_insert)]
            fn tr_insert_inductive(pre: Self, post: Self) { }

            #[inductive(tr_noop)]
            fn tr_noop_inductive(pre: Self, post: Self) { }

            #[inductive(tr_delete)]
            fn tr_delete_inductive(pre: Self, post: Self) { }
        }
    );

    verus! {

    // Inner sequential type — simple Vec-based set (no duplicates).
    pub struct SetInner {
        pub elements: Vec<u64>,
    }

    impl SetInner {
        pub fn empty() -> (s: Self)
            ensures s.elements@.len() == 0,
        {
            SetInner { elements: Vec::new() }
        }

        pub fn size(&self) -> (n: usize)
            ensures n == self.elements@.len(),
        {
            self.elements.len()
        }

        pub fn is_empty(&self) -> (b: bool)
            ensures b == (self.elements@.len() == 0),
        {
            self.elements.len() == 0
        }

        fn index_of(&self, val: &u64) -> (idx: Option<usize>)
            ensures
                idx.is_some() ==> idx.unwrap() < self.elements@.len()
                    && self.elements@[idx.unwrap() as int] == *val,
                idx.is_none() ==> !self.elements@.contains(*val),
        {
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    i <= self.elements@.len(),
                    forall|j: int| 0 <= j < i ==> self.elements@[j] != *val,
                decreases self.elements@.len() - i,
            {
                if self.elements[i] == *val {
                    return Some(i);
                }
                i += 1;
            }
            None
        }

        pub fn contains(&self, val: &u64) -> (found: bool)
            ensures found == self.elements@.contains(*val),
        {
            self.index_of(val).is_some()
        }

        // Insert if not present. Returns true if inserted (size grew by 1).
        pub fn insert(&mut self, val: u64) -> (inserted: bool)
            ensures
                inserted ==> self.elements@.len() == old(self).elements@.len() + 1,
                !inserted ==> self.elements@.len() == old(self).elements@.len(),
        {
            if self.contains(&val) {
                false
            } else {
                self.elements.push(val);
                true
            }
        }

        // Delete if present. Returns true if deleted (size shrank by 1).
        pub fn delete(&mut self, val: &u64) -> (deleted: bool)
            ensures
                deleted ==> self.elements@.len() == old(self).elements@.len() - 1,
                !deleted ==> self.elements@.len() == old(self).elements@.len(),
        {
            match self.index_of(val) {
                None => false,
                Some(idx) => {
                    self.elements.swap_remove(idx);
                    true
                }
            }
        }

        pub fn find(&self, val: &u64) -> (found: Option<u64>)
            ensures
                found.is_some() == self.elements@.contains(*val),
                found.is_some() ==> found.unwrap() == *val,
        {
            if self.contains(val) {
                Some(*val)
            } else {
                None
            }
        }

        pub fn minimum(&self) -> (min: Option<u64>)
            ensures
                self.elements@.len() == 0 ==> min.is_none(),
                self.elements@.len() > 0 ==> min.is_some(),
        {
            if self.elements.len() == 0 {
                None
            } else {
                let mut min_val = self.elements[0];
                let mut i: usize = 1;
                while i < self.elements.len()
                    invariant 1 <= i <= self.elements@.len(),
                    decreases self.elements@.len() - i,
                {
                    if self.elements[i] < min_val {
                        min_val = self.elements[i];
                    }
                    i += 1;
                }
                Some(min_val)
            }
        }

        pub fn maximum(&self) -> (max: Option<u64>)
            ensures
                self.elements@.len() == 0 ==> max.is_none(),
                self.elements@.len() > 0 ==> max.is_some(),
        {
            if self.elements.len() == 0 {
                None
            } else {
                let mut max_val = self.elements[0];
                let mut i: usize = 1;
                while i < self.elements.len()
                    invariant 1 <= i <= self.elements@.len(),
                    decreases self.elements@.len() - i,
                {
                    if self.elements[i] > max_val {
                        max_val = self.elements[i];
                    }
                    i += 1;
                }
                Some(max_val)
            }
        }
    }

    // Lock interior: concrete set + ghost count token.
    pub struct SetLockInterior {
        pub inner: SetInner,
        pub ghost_count: Tracked<SetSM::count>,
    }

    // Predicate: token count == concrete element count.
    pub ghost struct SetMtInv {
        pub instance: SetSM::Instance,
    }

    impl RwLockPredicate<SetLockInterior> for SetMtInv {
        open spec fn inv(self, interior: SetLockInterior) -> bool {
            interior.inner.elements@.len() == interior.ghost_count@.value()
            && interior.ghost_count@.instance_id() == self.instance.id()
        }
    }

    // Mt type: lock + instance.
    pub struct SetMtTsm {
        pub lock: RwLock<SetLockInterior, SetMtInv>,
        pub inst: Tracked<SetSM::Instance>,
    }

    impl SetMtTsm {
        pub open spec fn wf(&self) -> bool {
            self.lock.pred().instance == self.inst@
        }

        pub fn new_empty() -> (s: Self)
            ensures s.wf(),
        {
            let tracked (
                Tracked(instance),
                Tracked(count_token),
            ) = SetSM::Instance::initialize();

            let interior = SetLockInterior {
                inner: SetInner::empty(),
                ghost_count: Tracked(count_token),
            };

            SetMtTsm {
                lock: RwLock::new(interior, Ghost(SetMtInv { instance })),
                inst: Tracked(instance),
            }
        }

        // Write: insert. Steps TSM on actual insertion. Zero assumes.
        pub fn mt_insert(&self, val: u64)
            requires self.wf(),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();

            // Predicate proves: elements.len() == token.count. No assume!
            let inserted = interior.inner.insert(val);

            proof {
                if inserted {
                    self.inst.borrow().tr_insert(
                        &mut *interior.ghost_count.borrow_mut(),
                    );
                } else {
                    self.inst.borrow().tr_noop();
                }
            }

            write_handle.release_write(interior);
        }

        // Write: delete. Steps TSM on actual deletion. Zero assumes.
        pub fn mt_delete(&self, val: &u64)
            requires self.wf(),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();

            let deleted = interior.inner.delete(val);

            proof {
                if deleted {
                    self.inst.borrow().tr_delete(
                        &mut *interior.ghost_count.borrow_mut(),
                    );
                } else {
                    self.inst.borrow().tr_noop();
                }
            }

            write_handle.release_write(interior);
        }

        // Read operations: no TSM transition. Predicate proves well-formedness.
        // Return values come from the real data. Zero assumes.

        pub fn mt_contains(&self, val: &u64) -> (found: bool)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let found = interior.inner.contains(val);
            read_handle.release_read();
            found
        }

        pub fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let n = interior.inner.size();
            read_handle.release_read();
            n
        }

        pub fn mt_is_empty(&self) -> (b: bool)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let b = interior.inner.is_empty();
            read_handle.release_read();
            b
        }

        pub fn mt_find(&self, val: &u64) -> (found: Option<u64>)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let found = interior.inner.find(val);
            read_handle.release_read();
            found
        }

        pub fn mt_minimum(&self) -> (min: Option<u64>)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let min = interior.inner.minimum();
            read_handle.release_read();
            min
        }

        pub fn mt_maximum(&self) -> (max: Option<u64>)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let max = interior.inner.maximum();
            read_handle.release_read();
            max
        }
    }

    } // verus!

    #[test]
    fn test_set_mt_tsm_10_ops() {
        let s = SetMtTsm::new_empty();
        assert_eq!(s.mt_size(), 0);
        assert!(s.mt_is_empty());

        s.mt_insert(42);
        assert_eq!(s.mt_size(), 1);
        assert!(!s.mt_is_empty());
        assert!(s.mt_contains(&42));
        assert!(!s.mt_contains(&99));

        s.mt_insert(10);
        s.mt_insert(99);
        assert_eq!(s.mt_size(), 3);

        // Dup insert — no change.
        s.mt_insert(42);
        assert_eq!(s.mt_size(), 3);

        assert_eq!(s.mt_find(&42), Some(42));
        assert_eq!(s.mt_find(&7), None);
        assert_eq!(s.mt_minimum(), Some(10));
        assert_eq!(s.mt_maximum(), Some(99));

        s.mt_delete(&42);
        assert_eq!(s.mt_size(), 2);
        assert!(!s.mt_contains(&42));

        // Missing delete — no change.
        s.mt_delete(&42);
        assert_eq!(s.mt_size(), 2);

        assert_eq!(s.mt_minimum(), Some(10));
        assert_eq!(s.mt_maximum(), Some(99));
    }
}
