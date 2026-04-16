// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: PCell-based Mt set — attempting View + zero assumes.
//!
//! Architecture:
//!   - PCell<SetInner> holds the actual data (field on the struct)
//!   - RwLock holds Tracked<PointsTo<SetInner>> (the proof token)
//!   - To access: acquire lock → get PointsTo → use with PCell → release
//!   - The PointsTo proves what's in the PCell at every step
//!
//! Approach A: No View, specs on return values only (Counter3 pattern scaled to 10 ops).
//!   Expected: zero assumes, clean, but callers can't write `self@`.
//!
//! Approach B: View via ghost_view field + predicate that ties ghost_view to PointsTo.
//!   Stores a Ghost copy INSIDE the lock that the predicate ties to the PointsTo.
//!   Stores a matching Ghost copy OUTSIDE for View.
//!   Question: can we prove the outside copy matches on acquire?
//!
//! 10 operations: new, insert, delete, contains, size, is_empty, find, min, max.

pub mod bst_plain_mt_pcell {

    use vstd::prelude::*;
    use vstd::rwlock::{RwLock, RwLockPredicate};
    use vstd::cell::*;

    verus! {

    // ================================================================
    // Inner sequential type (same as bst_plain_mt_tsm.rs)
    // ================================================================

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
        { self.elements.len() }

        pub fn is_empty(&self) -> (b: bool)
            ensures b == (self.elements@.len() == 0),
        { self.elements.len() == 0 }

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
            { if self.elements[i] == *val { return Some(i); } i += 1; }
            None
        }

        pub fn contains(&self, val: &u64) -> (found: bool)
            ensures found == self.elements@.contains(*val),
        { self.index_of(val).is_some() }

        pub fn insert(&mut self, val: u64) -> (inserted: bool)
            ensures
                inserted ==> self.elements@.len() == old(self).elements@.len() + 1,
                !inserted ==> self.elements@.len() == old(self).elements@.len(),
        {
            if self.contains(&val) { false }
            else { self.elements.push(val); true }
        }

        pub fn delete(&mut self, val: &u64) -> (deleted: bool)
            ensures
                deleted ==> self.elements@.len() == old(self).elements@.len() - 1,
                !deleted ==> self.elements@.len() == old(self).elements@.len(),
        {
            match self.index_of(val) {
                None => false,
                Some(idx) => { self.elements.swap_remove(idx); true }
            }
        }

        pub fn find(&self, val: &u64) -> (found: Option<u64>)
            ensures
                found.is_some() == self.elements@.contains(*val),
                found.is_some() ==> found.unwrap() == *val,
        { if self.contains(val) { Some(*val) } else { None } }

        pub fn minimum(&self) -> (min: Option<u64>)
            ensures
                self.elements@.len() == 0 ==> min.is_none(),
                self.elements@.len() > 0 ==> min.is_some(),
        {
            if self.elements.len() == 0 { None }
            else {
                let mut min_val = self.elements[0];
                let mut i: usize = 1;
                while i < self.elements.len()
                    invariant 1 <= i <= self.elements@.len(),
                    decreases self.elements@.len() - i,
                { if self.elements[i] < min_val { min_val = self.elements[i]; } i += 1; }
                Some(min_val)
            }
        }

        pub fn maximum(&self) -> (max: Option<u64>)
            ensures
                self.elements@.len() == 0 ==> max.is_none(),
                self.elements@.len() > 0 ==> max.is_some(),
        {
            if self.elements.len() == 0 { None }
            else {
                let mut max_val = self.elements[0];
                let mut i: usize = 1;
                while i < self.elements.len()
                    invariant 1 <= i <= self.elements@.len(),
                    decreases self.elements@.len() - i,
                { if self.elements[i] > max_val { max_val = self.elements[i]; } i += 1; }
                Some(max_val)
            }
        }
    }

    // ================================================================
    // APPROACH A: PCell + RwLock<PointsTo>, no View. Zero assumes.
    // ================================================================

    pub struct TokenInv {
        pub ghost cell_id: CellId,
    }

    impl RwLockPredicate<Tracked<PointsTo<SetInner>>> for TokenInv {
        open spec fn inv(self, token: Tracked<PointsTo<SetInner>>) -> bool {
            token@.id() == self.cell_id
            && token@.is_init()
        }
    }

    pub struct SetMtPcellA {
        pub cell: PCell<SetInner>,
        pub token_lock: RwLock<Tracked<PointsTo<SetInner>>, TokenInv>,
    }

    // No View — state observable only through operations.

    impl SetMtPcellA {
        pub open spec fn wf(&self) -> bool {
            self.cell.id() == self.token_lock.pred().cell_id
        }

        pub fn new_empty() -> (s: Self)
            ensures s.wf(),
        {
            let (cell, Tracked(perm)) = PCell::new(SetInner::empty());
            let ghost cell_id = cell.id();
            SetMtPcellA {
                cell,
                token_lock: RwLock::new(Tracked(perm), Ghost(TokenInv { cell_id })),
            }
        }

        // Write: acquire → get PointsTo → take from PCell → mutate → put back → release.
        // ZERO assumes.
        pub fn mt_insert(&self, val: u64)
            requires self.wf(),
        {
            let (tracked_perm, write_handle) = self.token_lock.acquire_write();
            let tracked mut perm = tracked_perm.get();

            // Take the value out of PCell — proved by PointsTo.
            let mut inner = self.cell.take(Tracked(&mut perm));

            // Mutate.
            inner.insert(val);

            // Put back — PointsTo updated to reflect new value.
            self.cell.put(Tracked(&mut perm), inner);

            // Release.
            write_handle.release_write(Tracked(perm));
        }

        pub fn mt_delete(&self, val: &u64)
            requires self.wf(),
        {
            let (tracked_perm, write_handle) = self.token_lock.acquire_write();
            let tracked mut perm = tracked_perm.get();
            let mut inner = self.cell.take(Tracked(&mut perm));
            inner.delete(val);
            self.cell.put(Tracked(&mut perm), inner);
            write_handle.release_write(Tracked(perm));
        }

        // Read: acquire_read → borrow PointsTo → borrow PCell → compute → release.
        // ZERO assumes.
        pub fn mt_contains(&self, val: &u64) -> (found: bool)
            requires self.wf(),
        {
            let read_handle = self.token_lock.acquire_read();
            let tracked_ref = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(tracked_ref.borrow()));
            let found = inner.contains(val);
            read_handle.release_read();
            found
        }

        pub fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let read_handle = self.token_lock.acquire_read();
            let tracked_ref = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(tracked_ref.borrow()));
            let n = inner.size();
            read_handle.release_read();
            n
        }

        pub fn mt_is_empty(&self) -> (b: bool)
            requires self.wf(),
        {
            let read_handle = self.token_lock.acquire_read();
            let tracked_ref = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(tracked_ref.borrow()));
            let b = inner.is_empty();
            read_handle.release_read();
            b
        }

        pub fn mt_find(&self, val: &u64) -> (found: Option<u64>)
            requires self.wf(),
        {
            let read_handle = self.token_lock.acquire_read();
            let tracked_ref = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(tracked_ref.borrow()));
            let found = inner.find(val);
            read_handle.release_read();
            found
        }

        pub fn mt_minimum(&self) -> (min: Option<u64>)
            requires self.wf(),
        {
            let read_handle = self.token_lock.acquire_read();
            let tracked_ref = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(tracked_ref.borrow()));
            let min = inner.minimum();
            read_handle.release_read();
            min
        }

        pub fn mt_maximum(&self) -> (max: Option<u64>)
            requires self.wf(),
        {
            let read_handle = self.token_lock.acquire_read();
            let tracked_ref = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(tracked_ref.borrow()));
            let max = inner.maximum();
            read_handle.release_read();
            max
        }
    }

    // ================================================================
    // APPROACH B: PCell + RwLock<PointsTo + Ghost>, WITH View.
    //
    // Idea: store a Ghost<nat> (the element count) INSIDE the lock alongside
    // the PointsTo. The predicate ties: ghost_count == inner.elements.len().
    // Also keep a Ghost<nat> OUTSIDE for View.
    //
    // On release_write: update both copies to match.
    // On acquire_write: predicate proves internal ghost == inner. But can we
    // prove external ghost == internal ghost?
    //
    // Answer: NO. The external ghost was set at the previous release. Between
    // release and the next acquire, nothing prevents another thread from acquiring
    // and changing the internal state. The external ghost is stale.
    //
    // For &self operations (which is what Mt provides — shared access), this is
    // fundamental: multiple threads share the struct, so the ghost_view can't
    // track the current state. Only single-owner (&mut self) could maintain
    // the invariant.
    //
    // CONCLUSION: PCell + RwLock gives zero assumes but no View.
    // View requires either:
    //   (a) Single ownership (&mut self for writes), OR
    //   (b) An atomic ghost field that's updated under the same lock (which is
    //       exactly what we have now — just with an assume bridge), OR
    //   (c) A fundamentally different abstraction (split permissions, etc.)
    //
    // For completeness, we show Approach B with &mut self writes. This DOES
    // give View + zero assumes, but changes the Mt API from &self to &mut self
    // for writes. Reads remain &self.
    // ================================================================

    pub struct LockInteriorB {
        pub perm: Tracked<PointsTo<SetInner>>,
        pub ghost_count: Ghost<nat>,
    }

    pub struct TokenInvB {
        pub ghost cell_id: CellId,
    }

    impl RwLockPredicate<LockInteriorB> for TokenInvB {
        open spec fn inv(self, interior: LockInteriorB) -> bool {
            interior.perm@.id() == self.cell_id
            && interior.perm@.is_init()
            && interior.ghost_count@ == interior.perm@.value().elements@.len()
        }
    }

    pub struct SetMtPcellB {
        pub cell: PCell<SetInner>,
        pub lock: RwLock<LockInteriorB, TokenInvB>,
        pub ghost_count: Ghost<nat>,  // External copy for View.
    }

    impl View for SetMtPcellB {
        type V = nat;
        // View returns the element count. We'd love this to be == the real count.
        open spec fn view(&self) -> nat { self.ghost_count@ }
    }

    impl SetMtPcellB {
        pub open spec fn wf(&self) -> bool {
            self.cell.id() == self.lock.pred().cell_id
        }

        pub fn new_empty() -> (s: Self)
            ensures s.wf(), s@ == 0,
        {
            let (cell, Tracked(perm)) = PCell::new(SetInner::empty());
            let ghost cell_id = cell.id();
            let interior = LockInteriorB {
                perm: Tracked(perm),
                ghost_count: Ghost(0nat),
            };
            SetMtPcellB {
                cell,
                lock: RwLock::new(interior, Ghost(TokenInvB { cell_id })),
                ghost_count: Ghost(0nat),
            }
        }

        // &mut self write: we own the struct exclusively.
        // After release, we set ghost_count = real count. Since we're &mut self,
        // nobody else can have changed it between release and the assertion.
        //
        // But wait: between acquire and release, another thread COULD have the lock
        // if we're using &self. With &mut self, we're exclusive — so the ghost_count
        // we set is still valid when we return.
        //
        // The ensures can reference self@ because we have &mut self.
        // NOTE: We CANNOT prove `self@ >= old(self)@` because old(self).ghost_count@
        // (the external ghost) has no proved relationship to the actual element count
        // inside the lock. That's the fundamental View gap:
        //   - The predicate proves: internal_ghost == perm.value().elements.len()
        //   - But nothing proves: external_ghost == internal_ghost
        // With &mut self we could maintain the invariant IF we were the only writer,
        // but between our last release and this acquire, the lock contents may have
        // changed (another thread acquired via Arc clone). So even &mut self doesn't
        // help unless we also own the only Arc reference.
        //
        // RESULT: View is available but specs can't reference old(self)@ meaningfully.
        // The ensures can only state what self@ IS, not how it relates to old(self)@.
        pub fn mt_insert(&mut self, val: u64)
            requires old(self).wf(),
            ensures self.wf(),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();

            // Predicate proves: perm points to cell, ghost_count == elements.len().
            // Take the value out.
            let tracked mut perm = interior.perm.get();
            let mut inner = self.cell.take(Tracked(&mut perm));

            let old_len = inner.elements.len();
            let inserted = inner.insert(val);
            let new_len = inner.elements.len();

            // Put back.
            self.cell.put(Tracked(&mut perm), inner);

            // Update ghost copies — both inside and outside.
            let ghost new_count = new_len as nat;
            let new_interior = LockInteriorB {
                perm: Tracked(perm),
                ghost_count: Ghost(new_count),
            };
            write_handle.release_write(new_interior);

            // Update external ghost. Since we have &mut self, this is safe —
            // nobody else can observe an inconsistent state.
            self.ghost_count = Ghost(new_count);
        }

        pub fn mt_delete(&mut self, val: &u64)
            requires old(self).wf(),
            ensures self.wf(),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();
            let tracked mut perm = interior.perm.get();
            let mut inner = self.cell.take(Tracked(&mut perm));

            inner.delete(val);
            let new_len = inner.elements.len();

            self.cell.put(Tracked(&mut perm), inner);
            let ghost new_count = new_len as nat;
            write_handle.release_write(LockInteriorB {
                perm: Tracked(perm),
                ghost_count: Ghost(new_count),
            });
            self.ghost_count = Ghost(new_count);
        }

        // Read via &self — still zero assumes.
        pub fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(interior.perm.borrow()));
            let n = inner.size();
            read_handle.release_read();
            n
        }

        pub fn mt_contains(&self, val: &u64) -> (found: bool)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(interior.perm.borrow()));
            let found = inner.contains(val);
            read_handle.release_read();
            found
        }

        pub fn mt_is_empty(&self) -> (b: bool)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();
            let inner = self.cell.borrow(Tracked(interior.perm.borrow()));
            let b = inner.is_empty();
            read_handle.release_read();
            b
        }
    }

    } // verus!

    #[test]
    fn test_approach_a_pcell_no_view() {
        let s = SetMtPcellA::new_empty();
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
        s.mt_insert(42);  // dup
        assert_eq!(s.mt_size(), 3);
        assert_eq!(s.mt_find(&42), Some(42));
        assert_eq!(s.mt_find(&7), None);
        assert_eq!(s.mt_minimum(), Some(10));
        assert_eq!(s.mt_maximum(), Some(99));
        s.mt_delete(&42);
        assert_eq!(s.mt_size(), 2);
        assert!(!s.mt_contains(&42));
    }

    #[test]
    fn test_approach_b_pcell_with_view() {
        let mut s = SetMtPcellB::new_empty();
        assert_eq!(s.mt_size(), 0);
        assert!(s.mt_is_empty());
        s.mt_insert(42);
        assert_eq!(s.mt_size(), 1);
        assert!(s.mt_contains(&42));
        s.mt_insert(10);
        s.mt_insert(99);
        assert_eq!(s.mt_size(), 3);
        s.mt_delete(&42);
        assert_eq!(s.mt_size(), 2);
    }
}
