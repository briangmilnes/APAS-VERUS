//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: eliminate ghost field entirely.
//!
//! Hypothesis: if we DON'T have a ghost field, and instead define View in terms
//! of the token state machine instance, we can avoid the ghost↔lock gap.
//!
//! Approach 1: Store a tracked token OUTSIDE the lock as a proof witness.
//! The token's value IS the view. When we acquire write, we get the lock's
//! copy of the token, step the state machine, update both tokens, release.
//! The struct's token always reflects the current state.
//!
//! Approach 2: Use split tokens. Half inside lock, half outside. The two
//! halves agree on value by construction (tokenized_state_machine sharding).
//!
//! Approach 3: No ghost field, no external token. View is simply not available
//! without acquiring the lock. Specs express "after acquiring and reading, the
//! result satisfies P" rather than "self@ has property P". This changes the
//! API contract but eliminates all assumes.

pub mod rwlock_no_ghost_field {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {

    // Inner type — simple counter.
    pub struct Counter { pub val: u64 }

    impl View for Counter {
        type V = u64;
        open spec fn view(&self) -> u64 { self.val as u64 }
    }

    // Lock predicate — the value is non-negative (trivial for u64).
    pub struct CounterInv;
    impl RwLockPredicate<Counter> for CounterInv {
        open spec fn inv(self, v: Counter) -> bool { true }
    }

    // ================================================================
    // Approach 3: No ghost field, no View on the outer struct.
    // Operations return results with ensures that reference the result only.
    // The "state" is opaque — you can only observe it through operations.
    // ================================================================

    pub struct LockedCounter {
        pub lock: Arc<RwLock<Counter, CounterInv>>,
    }

    // NO View impl for LockedCounter.
    // Specs talk about operation results, not self@.

    pub trait LockedCounterTrait: Sized {
        fn new(v: u64) -> (s: Self);

        fn increment(&self) -> (old_val: u64)
            ensures old_val < u64::MAX;
            // Returns old value. Caller knows it was < MAX (precondition).
            // But we can't say "new state = old_val + 1" without a view...

        fn read(&self) -> (v: u64);
            // Returns current value. No spec — concurrent reads are racy.

        // What if we combine read+operate atomically?
        fn read_and_check(&self) -> (v: u64);
            // ensures: v is whatever was in the lock at acquire time.
            // This is trivially true and useless.
    }

    // Approach 3 doesn't work — without View, we can't express useful specs.
    // The ensures can only talk about the return value, not the state change.

    // ================================================================
    // Approach 4: &mut self operations, no lock at all.
    // Use the inner type directly. &mut gives exclusive access.
    // The Mt wrapper uses external_body only at the thread spawn boundary.
    // ================================================================

    // This is what StEph already is. The question is: can we wrap StEph in
    // something that gives &mut access across threads, with full specs?

    // What if the "lock" returns &mut Self instead of the inner value?
    // Then the ensures on &mut self just work.

    // ================================================================
    // Approach 5: Tracked token as the ONLY state. No concrete lock at all.
    // Pure ghost protocol. The "operations" are proof functions that step
    // the token. Exec code wraps them with actual mutation.
    // ================================================================

    // Actually... what if we use a PCell with PointsTo?

    } // verus!

    // ================================================================
    // Approach 6: PCell + PointsTo — the permission IS the connection.
    // ================================================================

    use vstd::cell::*;
    use vstd::raw_ptr::MemContents;

    verus! {

    pub struct CellCounter {
        pub cell: PCell<u64>,
        pub perm: Tracked<PointsTo<u64>>,
    }

    impl View for CellCounter {
        type V = u64;
        // The view reads directly from the permission — no ghost field!
        open spec fn view(&self) -> u64 {
            match self.perm@.mem_contents() {
                MemContents::Init(v) => v,
                MemContents::Uninit => arbitrary(),
            }
        }
    }

    impl CellCounter {
        pub open spec fn wf(&self) -> bool {
            self.perm@.id() == self.cell.id()
            && self.perm@.is_init()
        }
    }

    pub trait CellCounterTrait: Sized + View<V = u64> {
        spec fn spec_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            ensures s@ == v, s.spec_wf();

        fn increment(&mut self)
            requires old(self).spec_wf(), old(self)@ < u64::MAX,
            ensures self.spec_wf(), self@ == old(self)@ + 1;

        fn read(&self) -> (v: u64)
            requires self.spec_wf(),
            ensures v == self@;
    }

    impl CellCounterTrait for CellCounter {
        open spec fn spec_wf(&self) -> bool { self.wf() }

        fn new(v: u64) -> (s: Self) {
            let (cell, Tracked(perm)) = PCell::empty();
            let tracked mut perm = perm;
            cell.put(Tracked(&mut perm), v);
            CellCounter { cell, perm: Tracked(perm) }
        }

        fn increment(&mut self) {
            let v = self.cell.take(Tracked(&mut self.perm.borrow_mut()));
            self.cell.put(Tracked(&mut self.perm.borrow_mut()), v + 1);
        }

        fn read(&self) -> (v: u64) {
            // PCell::borrow reads without taking.
            let v = *self.cell.borrow(Tracked(&self.perm.borrow()));
            v
        }
    }

    } // verus!

    // PCell approach: NO assumes needed. The Tracked<PointsTo> IS the proof
    // that the cell contains what we think it contains. View reads from the
    // permission, which is updated atomically with the cell.
    //
    // But PCell is single-threaded (requires &mut for write, & for read).
    // For Mt, we need concurrent access. Options:
    //   - Wrap PCell in an RwLock... and we're back to the same problem.
    //   - Use AtomicCell (vstd::atomic) for small values.
    //   - Use a concurrent protocol with split tokens.
    //
    // The REAL experiment: can we combine PCell's ghost-connection property
    // with RwLock's concurrency? What if the RwLock stores the PCell+PointsTo
    // pair, and the outer struct holds nothing?

    #[test]
    fn test_cell_counter() {
        let mut c = CellCounter::new(0);
        assert_eq!(c.read(), 0);
        c.increment();
        assert_eq!(c.read(), 1);
        c.increment();
        c.increment();
        assert_eq!(c.read(), 3);
    }

    // ================================================================
    // Approach 7: RwLock around StEph, NO Mt type at all.
    // The lock holds the inner StEph directly. Callers acquire the lock,
    // get &mut StEph, call trait methods with full specs, release.
    // A thin wrapper provides the concurrent API.
    //
    // The ghost_field↔lock gap disappears because there IS no ghost field.
    // The Mt "type" is just Arc<RwLock<StEph>>.
    // ================================================================

    // Simulate a StEph type with full specs (no assumes).
    verus! {

    pub struct SetStEph {
        pub elements: Vec<u64>,
    }

    impl View for SetStEph {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

    pub trait SetStEphTrait: Sized + View<V = Seq<u64>> {
        fn empty() -> (s: Self)
            ensures s@.len() == 0;

        fn insert(&mut self, val: u64)
            ensures self@.len() == old(self)@.len() + 1,
                    self@.last() == val;

        fn size(&self) -> (n: usize)
            ensures n == self@.len();
    }

    impl SetStEphTrait for SetStEph {
        fn empty() -> (s: Self) { SetStEph { elements: Vec::new() } }

        fn insert(&mut self, val: u64) {
            self.elements.push(val);
        }

        fn size(&self) -> (n: usize) { self.elements.len() }
    }

    // The Mt "wrapper" — just an RwLock predicate.
    pub struct SetMtInv;
    impl RwLockPredicate<SetStEph> for SetMtInv {
        open spec fn inv(self, v: SetStEph) -> bool { true }
    }

    // No Mt struct. Just a type alias.
    pub type SetMt = Arc<RwLock<SetStEph, SetMtInv>>;

    // Concurrent operations — thin wrappers with NO ghost fields, NO assumes.
    // These are free functions, not trait methods.
    pub fn mt_new() -> (mt: SetMt) {
        let s = SetStEph::empty();
        new_arc_rwlock::<SetStEph, SetMtInv>(s, Ghost(SetMtInv))
    }

    pub fn mt_insert(mt: &SetMt, val: u64) {
        let arc = clone_arc_rwlock(mt);
        let (mut inner, write_handle) = arc.acquire_write();
        inner.insert(val);
        write_handle.release_write(inner);
    }

    pub fn mt_size(mt: &SetMt) -> (n: usize) {
        let arc = clone_arc_rwlock(mt);
        let read_handle = arc.acquire_read();
        let n = read_handle.borrow().size();
        read_handle.release_read();
        n
    }

    } // verus!

    #[test]
    fn test_mt_set() {
        let mt = mt_new();
        assert_eq!(mt_size(&mt), 0);
        mt_insert(&mt, 42);
        assert_eq!(mt_size(&mt), 1);
        mt_insert(&mt, 99);
        assert_eq!(mt_size(&mt), 2);
    }
}
