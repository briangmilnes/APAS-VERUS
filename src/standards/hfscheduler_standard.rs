//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! HFScheduler Standard: Arc<RwLock<T, Inv>> + HFScheduler join() for verified
//! fork-join parallelism with shared mutable state.
//!
//! Demonstrates:
//! - RwLockPredicate with ghost field for construction-time context.
//! - external_body helpers for Arc::new(RwLock::new(...)) and Arc::clone.
//! - HFScheduler join() with named closures for spec propagation.
//! - acquire_write/release_write and acquire_read/release_read through Arc.
//! - ParaPair! macro alternative.
//!
//! The gap this fills: vstd::rwlock, std::sync::Arc, and HFScheduler each
//! work independently but Verus can't prove pred() preservation through Arc.
//! Small external_body helpers bridge that gap with tight ensures.

pub mod hfscheduler_standard {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
    use crate::Types::Types::*;
    use crate::ParaPair;

    verus! {

    // Pattern A: Parallel reads from shared state, returning computed results.
    //
    // Two tasks read a shared bounded counter and return derived values.
    // Demonstrates: new_arc, clone_arc, acquire_read, join with return values.

    pub struct BoundedCounterInv {
        pub ghost max_val: nat,
    }

    impl RwLockPredicate<u64> for BoundedCounterInv {
        open spec fn inv(self, v: u64) -> bool {
            v as nat <= self.max_val
        }
    }

    // Bridge: Arc::new(RwLock::new(...)) with pred() in ensures.
    #[verifier::external_body]
    fn new_arc_counter(init: u64, Ghost(pred): Ghost<BoundedCounterInv>)
        -> (arc: Arc<RwLock<u64, BoundedCounterInv>>)
        requires pred.inv(init),
        ensures arc.pred() == pred,
    {
        Arc::new(RwLock::new(init, Ghost(pred)))
    }

    // Bridge: Arc::clone with pred() preservation.
    #[verifier::external_body]
    fn clone_arc_counter(arc: &Arc<RwLock<u64, BoundedCounterInv>>)
        -> (cloned: Arc<RwLock<u64, BoundedCounterInv>>)
        ensures cloned.pred() == arc.pred(),
    {
        arc.clone()
    }

    fn parallel_reads() {
        let ghost pred = BoundedCounterInv { max_val: 100 };
        let arc = new_arc_counter(42, Ghost(pred));
        let arc1 = clone_arc_counter(&arc);
        let arc2 = clone_arc_counter(&arc);

        // Named closures: required for spec propagation through join().
        // Inline closures lose ensures; bind to a variable first.
        let f1 = move || -> (r: u64)
            requires arc1.pred() == pred
            ensures r as nat <= pred.max_val
        {
            let handle = arc1.acquire_read();
            let v = *handle.borrow();
            handle.release_read();
            v
        };

        let f2 = move || -> (r: u64)
            requires arc2.pred() == pred
            ensures r as nat <= pred.max_val
        {
            let handle = arc2.acquire_read();
            let v = *handle.borrow();
            handle.release_read();
            v
        };

        let (a, b) = join(f1, f2);
        assert(a as nat <= 100);
        assert(b as nat <= 100);
    }


    // Pattern B: Parallel writes to shared state (memoization / accumulation).
    //
    // Two tasks each increment a shared counter through the lock.
    // Demonstrates: acquire_write, release_write, unit-returning closures.

    // Pass a concrete cap so exec code can guard the increment.
    // The ghost max_val lives in the predicate; cap bridges ghost to exec.
    fn increment(arc: Arc<RwLock<u64, BoundedCounterInv>>, cap: u64)
        requires arc.pred().max_val == cap as nat,
    {
        let (val, write_handle) = arc.acquire_write();
        if val < cap {
            write_handle.release_write(val + 1);
        } else {
            write_handle.release_write(val);
        }
    }

    fn parallel_writes() {
        let ghost pred = BoundedCounterInv { max_val: 100 };
        let arc = new_arc_counter(0, Ghost(pred));
        let arc1 = clone_arc_counter(&arc);
        let arc2 = clone_arc_counter(&arc);

        let f1 = move || -> (r: ())
            requires arc1.pred() == pred, pred.max_val == 100,
        { increment(arc1, 100); };

        let f2 = move || -> (r: ())
            requires arc2.pred() == pred, pred.max_val == 100,
        { increment(arc2, 100); };

        let ((), ()) = join(f1, f2);

        // After join, original arc still valid — read the result.
        let handle = arc.acquire_read();
        let final_val = *handle.borrow();
        handle.release_read();
        assert(final_val as nat <= 100);
    }


    // Pattern C: ParaPair! macro alternative.
    //
    // Same as Pattern A but using ParaPair! which returns Pair(a, b).

    fn parallel_reads_parapair() {
        let ghost pred = BoundedCounterInv { max_val: 100 };
        let arc = new_arc_counter(42, Ghost(pred));
        let arc1 = clone_arc_counter(&arc);
        let arc2 = clone_arc_counter(&arc);

        let f1 = move || -> (r: u64)
            requires arc1.pred() == pred
            ensures r as nat <= pred.max_val
        {
            let handle = arc1.acquire_read();
            let v = *handle.borrow();
            handle.release_read();
            v
        };

        let f2 = move || -> (r: u64)
            requires arc2.pred() == pred
            ensures r as nat <= pred.max_val
        {
            let handle = arc2.acquire_read();
            let v = *handle.borrow();
            handle.release_read();
            v
        };

        let Pair(a, b) = ParaPair!(f1, f2);
        assert(a as nat <= 100);
        assert(b as nat <= 100);
    }

    } // verus!
}
