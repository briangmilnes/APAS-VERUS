//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Top-Level Coarse RwLock Standard: wrapping a verified struct in RwLock for Mt modules.
//!
//! Mt modules wrap a fully-verified St struct in an RwLock for thread-safe access.
//! The inner struct is verified with full specs (Layer 1). The locked wrapper (Layer 2)
//! acquires the lock, checks preconditions at runtime, and returns Result.
//!
//! This is optimistic locking: acquire the lock, check preconditions in exec code,
//! proceed on Ok, bail on Err. Another thread may have changed the inner value
//! between your last check and your lock acquisition — Err is not failure, it's
//! contention.
//!
//! Architecture:
//! - Layer 1 (inner struct): pub fields, View, trait with full specs, verified impl.
//!   No ghost fields — the struct is its own spec via View.
//! - Layer 2 (locked wrapper): pub(crate) fields (for type_invariant), RwLock<Inner, Inv>,
//!   ghost shadow of inner value, View via closed spec fn accessor, trait with Result
//!   return specs.
//!
//! Trust profile:
//! - Layer 1: zero trust, fully verified.
//! - Layer 2: accepts at lock boundary to link ghost to inner value.
//!   Three accept categories (use `accept()`, never bare `assume()`):
//!   1. Writer accept: ghost_value == inner@ (before exec check + mutation).
//!   2. Reader accept: return value == self@ (after reading through lock).
//!   3. Predicate accept: return predicate == spec predicate (after reading through lock).
//!
//! ONLY these three patterns may use `accept()` in a coarse RwLock Layer 2 impl.
//! Any other assume/accept is a proof obligation that must be discharged, not accepted.
//! Misclassifying an algorithmic assume as a lock-boundary accept defeats the proof.
//!
//! What we get for free (no accepts):
//! - spec_countdown_wf (Layer 1 well-formedness) is guaranteed by two things:
//!   1. new() constructs a valid inner value and the ghost shadow to match.
//!   2. The RwLockPredicate enforces spec_countdown_wf on every release_write,
//!      and guarantees it on every acquire_read/acquire_write.
//! - So spec_x_wf flows from construction + the lock predicate — no accept needed.
//!
//! Why the accepts are necessary:
//! - The RwLockPredicate is frozen at construction — it constrains shape but can't
//!   track a changing value.
//! - type_invariant can't see inside the RwLock.
//! - use_type_invariant is unsupported on &mut self.
//! - The ghost shadow is the only way to get value-level specs at Layer 2.
//!
//! Naming conventions:
//! - Inner struct: descriptive name (CountDown, BoundedCounter, etc.).
//! - Locked wrapper: Locked<InnerName> (LockedCountDown, LockedBoundedCounter).
//! - Predicate struct: <InnerName>Inv (CountDownInv, BoundedCounterInv).
//! - Well-formedness spec: spec_<typename>_wf (spec_countdown_wf, spec_lockedcountdown_wf).
//! - Ghost field: ghost_locked_<sibling> (ghost_locked_count for locked_count).
//! - Closed accessor: spec_ghost_locked_<sibling> for View indirection.
//!
//! The pub(crate) fields + type_invariant pattern:
//! - pub(crate) fields make the struct opaque (required for type_invariant).
//! - View uses a closed spec fn accessor to avoid opaque field access in open spec fn.
//! - type_invariant must live in an inherent impl block, not in a trait impl.
//! - type_invariant cannot be a trait function.
//! - use_type_invariant works only on &self, not &mut self.
//!
//! Mutating fields under type_invariant:
//! - Individual field assignment can violate the invariant at intermediate states.
//! - Use whole-struct replacement: `*self = LockedFoo { ... }` to satisfy the invariant
//!   atomically.
//!
//! Why RwLock, not Arc<RwLock>:
//! - APAS Mt modules do not share mutable state across threads. The pattern is:
//!   build (single thread, mutable), fork (read-only closures over immutable data or
//!   owned splits), join (combine results).
//! - The locked wrapper lives on one thread. Parallel workers get immutable slices or
//!   owned subproblems — they never mutate the same structure concurrently.
//! - Plain RwLock (no Arc) is sufficient for all APAS Mt modules.
//! - Arc<RwLock> (see hfscheduler_standard.rs) is only needed when multiple threads
//!   need shared ownership of mutable state (e.g., a concurrent data structure with
//!   multiple writers). APAS does not have this pattern.
//!
//! References:
//! - src/standards/hfscheduler_standard.rs (Arc<RwLock> + join for parallelism).
//! - src/standards/partial_eq_eq_clone_standard.rs (accept patterns for derive impls).

pub mod toplevel_coarse_rwlocks_for_mt_modules {

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::vstdplus::accept::accept;

    verus! {

    // Layer 1: Inner struct. Fully verified, no ghost, no lock.

    pub struct CountDown {
        pub count: i64,
    }

    impl View for CountDown {
        type V = int;
        open spec fn view(&self) -> int { self.count as int }
    }

    pub trait CountDownTrait: Sized + View<V = int> {
        spec fn spec_countdown_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            requires v <= i64::MAX as u64,
            ensures s@ == v as int, s.spec_countdown_wf();

        fn count_down(&mut self)
            requires old(self).spec_countdown_wf(),
                     old(self)@ > 0,
            ensures self.spec_countdown_wf(),
                    self@ == old(self)@ - 1;

        fn count(&self) -> (v: u64)
            requires self.spec_countdown_wf(),
            ensures v as int == self@;

        fn done(&self) -> (d: bool)
            requires self.spec_countdown_wf(),
            ensures d == (self@ == 0);
    }

    impl CountDownTrait for CountDown {
        open spec fn spec_countdown_wf(&self) -> bool { self@ >= 0 }

        fn new(v: u64) -> (s: Self) { CountDown { count: v as i64 } }

        fn count_down(&mut self) { self.count = self.count - 1; }

        fn count(&self) -> (v: u64) { self.count as u64 }

        fn done(&self) -> (d: bool) { self.count == 0 }
    }

    // Layer 2: Locked wrapper. RwLock + ghost shadow + type_invariant.

    // Predicate: real invariant on the inner value, not `true`.
    pub struct CountDownInv;

    impl RwLockPredicate<CountDown> for CountDownInv {
        open spec fn inv(self, v: CountDown) -> bool { v.spec_countdown_wf() }
    }

    pub struct LockedCountDown {
        pub(crate) locked_count: RwLock<CountDown, CountDownInv>,
        pub(crate) ghost_locked_count: Ghost<int>,
    }

    // type_invariant must be in an inherent impl, not a trait impl.
    impl LockedCountDown {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_count@ >= 0
        }

        // Closed accessor: lets View work without exposing opaque fields.
        pub closed spec fn spec_ghost_locked_count(self) -> int {
            self.ghost_locked_count@
        }
    }

    impl View for LockedCountDown {
        type V = int;
        open spec fn view(&self) -> int { self.spec_ghost_locked_count() }
    }

    // Trait: Result return, value specs via ghost shadow.
    pub trait LockedCountDownTrait: Sized + View<V = int> {
        spec fn spec_lockedcountdown_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            requires v <= i64::MAX as u64,
            ensures s.spec_lockedcountdown_wf(),
                    s@ == v as int;

        // Ok: precondition held, operation applied.
        // Err: precondition failed (contention or boundary), value unchanged.
        fn count_down(&mut self) -> (r: Result<(), ()>)
            requires old(self).spec_lockedcountdown_wf(),
            ensures self.spec_lockedcountdown_wf(),
                    match r {
                        Ok(_) => self@ == old(self)@ - 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn count(&self) -> (v: u64)
            requires self.spec_lockedcountdown_wf(),
            ensures v as int == self@;

        fn done(&self) -> (d: bool)
            requires self.spec_lockedcountdown_wf(),
            ensures d == (self@ == 0);
    }

    impl LockedCountDownTrait for LockedCountDown {
        open spec fn spec_lockedcountdown_wf(&self) -> bool { self@ >= 0 }

        fn new(v: u64) -> (s: Self) {
            let cd = CountDown::new(v);
            LockedCountDown {
                locked_count: RwLock::new(cd, Ghost(CountDownInv)),
                ghost_locked_count: Ghost(v as int),
            }
        }

        // Writer accept: ghost == inner before exec-check + mutation.
        fn count_down(&mut self) -> (r: Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_count.acquire_write();
            proof { accept(self.ghost_locked_count@ == locked_val@); }
            if locked_val.count > 0 {
                locked_val.count_down();
                let ghost new_val = locked_val@;
                self.ghost_locked_count = Ghost(new_val);
                write_handle.release_write(locked_val);
                Ok(())
            } else {
                write_handle.release_write(locked_val);
                Err(())
            }
        }

        // Reader accept: return value matches ghost.
        fn count(&self) -> (v: u64) {
            let read_handle = self.locked_count.acquire_read();
            let v = read_handle.borrow().count();
            proof { accept(v as int == self@); }
            read_handle.release_read();
            v
        }

        // Reader accept: return predicate matches spec predicate.
        fn done(&self) -> (d: bool) {
            let read_handle = self.locked_count.acquire_read();
            let d = read_handle.borrow().done();
            proof { accept(d == (self@ == 0)); }
            read_handle.release_read();
            d
        }
    }

    } // verus!
}
