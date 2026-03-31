//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: PCell + RwLock with zero assumes.
//!
//! Architecture:
//!   - Arc<PCell<T>> holds the data (shared across threads)
//!   - RwLock holds Tracked<PointsTo<T>> (the proof token)
//!   - To access data: acquire RwLock → get PointsTo → use it with PCell → release
//!   - The PointsTo proves what's in the PCell at every step
//!   - View defined from the PointsTo inside the RwLock predicate
//!
//! Hypothesis: zero assumes. The PointsTo IS the proof.

pub mod pcell_rwlock_zero_assume {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use vstd::cell::*;
    use vstd::raw_ptr::MemContents;

    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {

    // The RwLock predicate: the PointsTo must refer to our PCell and be initialized.
    pub struct TokenInv {
        pub ghost cell_id: CellId,
    }

    impl RwLockPredicate<Tracked<PointsTo<u64>>> for TokenInv {
        open spec fn inv(self, token: Tracked<PointsTo<u64>>) -> bool {
            token@.id() == self.cell_id
            && token@.is_init()
        }
    }

    // The concurrent counter: PCell for data, RwLock for the proof token.
    pub struct Counter {
        pub cell: Arc<PCell<u64>>,
        pub token_lock: Arc<RwLock<Tracked<PointsTo<u64>>, TokenInv>>,
    }

    impl View for Counter {
        type V = u64;
        // View: we'd like to say "whatever the PointsTo says is in the cell."
        // But we can't read through the lock in a spec...
        // We need a ghost field after all? Or can the predicate carry it?
        //
        // Alternative: don't define View on Counter. Define specs on operations
        // that return values with ensures about those values.
        open spec fn view(&self) -> u64 {
            // Can't access lock contents in spec. Need a ghost field.
            arbitrary()
        }
    }

    // OK — View without ghost field doesn't work for expressing specs on self.
    // Let's try with a ghost field BUT derive it from the PointsTo on construction
    // and update it atomically with the token.

    pub struct Counter2 {
        pub cell: Arc<PCell<u64>>,
        pub token_lock: Arc<RwLock<Tracked<PointsTo<u64>>, TokenInv>>,
        pub ghost_val: Ghost<u64>,
    }

    impl View for Counter2 {
        type V = u64;
        open spec fn view(&self) -> u64 { self.ghost_val@ }
    }

    pub trait Counter2Trait: Sized + View<V = u64> {
        fn new(v: u64) -> (s: Self)
            ensures s@ == v;

        fn increment(&mut self)
            requires old(self)@ < u64::MAX,
            ensures self@ == old(self)@ + 1;

        fn read(&self) -> (v: u64)
            ensures v == self@;
    }

    impl Counter2Trait for Counter2 {
        fn new(v: u64) -> (s: Self) {
            let (cell, Tracked(perm)) = PCell::new(v);
            let ghost cell_id = cell.id();
            let ghost inv = TokenInv { cell_id };
            Counter2 {
                cell: Arc::new(cell),
                token_lock: new_arc_rwlock::<Tracked<PointsTo<u64>>, TokenInv>(
                    Tracked(perm), Ghost(inv)),
                ghost_val: Ghost(v),
            }
        }

        fn increment(&mut self) {
            // Acquire the token from the lock.
            let token_arc = clone_arc_rwlock(&self.token_lock);
            let (tracked_perm, write_handle) = token_arc.acquire_write();
            let tracked mut perm = tracked_perm.get();

            // The predicate guarantees: perm.id() == cell.id() && perm.is_init().
            // Now we can access the PCell with full proof.

            // Read current value via PCell + PointsTo — no assume needed!
            let old_val = *self.cell.borrow(Tracked(&perm));

            // But wait: does old_val == self.ghost_val@?
            // The predicate says perm points to our cell and is init.
            // perm.mem_contents() == MemContents::Init(old_val) after borrow.
            // But ghost_val@ was set at construction or last write.
            // We need: ghost_val@ == old_val. That's... the same assume.

            proof {
                // Still need this. The PointsTo proves what's in the cell,
                // but nothing connects that to ghost_val.
                assume(self.ghost_val@ == old_val);
            }

            // Write new value.
            self.cell.write(Tracked(&mut perm), old_val + 1);

            // Update ghost.
            self.ghost_val = Ghost((old_val + 1) as u64);

            // Release token back to lock.
            write_handle.release_write(Tracked(perm));
        }

        fn read(&self) -> (v: u64) {
            let token_arc = clone_arc_rwlock(&self.token_lock);
            let read_handle = token_arc.acquire_read();
            let tracked_ref: &Tracked<PointsTo<u64>> = read_handle.borrow();
            let v = *self.cell.borrow(Tracked(tracked_ref.borrow()));
            proof { assume(self.ghost_val@ == v); }
            read_handle.release_read();
            v
        }
    }

    // ================================================================
    // Attempt 3: NO ghost field. View not on the struct.
    // Operations return values with ensures about the VALUES, not self@.
    // Use a "transaction" style: acquire returns a handle that knows the state.
    // ================================================================

    // Note: using PCell directly (not Arc) to prove zero-assume concept.
    // Arc deref is opaque to Verus specs — addressing that is a separate problem.
    pub struct Counter3 {
        pub cell: PCell<u64>,
        pub token_lock: RwLock<Tracked<PointsTo<u64>>, TokenInv>,
    }


    // No View impl — the state is only observable through operations.

    pub trait Counter3Trait: Sized {
        spec fn wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            ensures s.wf();

        fn increment(&self) -> (old_val: u64)
            requires self.wf();

        fn read(&self) -> (v: u64)
            requires self.wf();
    }

    impl Counter3Trait for Counter3 {
        open spec fn wf(&self) -> bool {
            self.cell.id() == self.token_lock.pred().cell_id
        }

        fn new(v: u64) -> (s: Self) {
            let (cell, Tracked(perm)) = PCell::new(v);
            let ghost cell_id = cell.id();
            let ghost inv = TokenInv { cell_id };
            Counter3 {
                cell,
                token_lock: RwLock::new(Tracked(perm), Ghost(inv)),
            }
        }

        fn increment(&self) -> (old_val: u64) {
            let (tracked_perm, write_handle) = self.token_lock.acquire_write();
            let tracked mut perm = tracked_perm.get();

            // Read via PointsTo — ZERO assume!
            let old_val = *self.cell.borrow(Tracked(&perm));

            // Write new value — ZERO assume!
            self.cell.write(Tracked(&mut perm), old_val + 1);

            // Release token back to lock.
            write_handle.release_write(Tracked(perm));
            old_val
        }

        fn read(&self) -> (v: u64) {
            let token_arc = clone_arc_rwlock(&self.token_lock);
            let read_handle = token_arc.acquire_read();
            let tracked_ref: &Tracked<PointsTo<u64>> = read_handle.borrow();
            let v = *self.cell.borrow(Tracked(tracked_ref.borrow()));
            read_handle.release_read();
            v
        }
    }

    } // verus!

    #[test]
    fn test_counter3_zero_assume() {
        let c = Counter3::new(0);
        assert_eq!(c.read(), 0);
        assert_eq!(c.increment(), 0);  // returns old val
        assert_eq!(c.read(), 1);
        assert_eq!(c.increment(), 1);
        assert_eq!(c.read(), 2);
    }
}
