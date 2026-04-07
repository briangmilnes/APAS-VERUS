//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Spec Naming Convention Standard: well-formedness predicates and spec function names.
//!
//! Every well-formedness predicate follows the pattern `spec_<name>_wf`. Never use
//! bare `spec_wf`.
//!
//! Two levels of well-formedness:
//!
//! 1. Module-level wf: `spec_<module_no_underscores>_wf`.
//!    - The module name is lowercased with all underscores removed.
//!    - Examples: `spec_tablemteph_wf`, `spec_bfsmteph_wf`, `spec_orderedtablestper_wf`.
//!    - This is the top-level predicate that appears in requires/ensures across the API.
//!    - In Mt modules with RwLock, this is the Layer 2 predicate on the locked wrapper.
//!    - Can be either a trait method (`&self`) or a free function (algorithm precondition).
//!
//! 2. Datatype-level wf: `spec_<datatype_lowercase>_wf`.
//!    - Named after the inner struct, not the module.
//!    - Examples: `spec_countdown_wf`, `spec_boundedcounter_wf`.
//!    - In the two-layer RwLock pattern, this is the Layer 1 predicate on the inner struct.
//!    - The RwLockPredicate enforces it on every release_write and guarantees it on every
//!      acquire — so it flows from construction + predicate with no assume needed.
//!
//! Other spec function naming:
//! - Name spec functions after the operation: `spec_inject`, `spec_size`, `spec_contains`.
//! - No `_post` suffix. No generic names like `spec_result`.
//! - Closed accessors for opaque fields: `spec_ghost_locked_<field>`.
//!
//! Two forms of wf predicates:
//! - Trait method (`&self`): for datatype invariants. E.g., `self.spec_tablemteph_wf()`.
//! - Free function: for algorithm preconditions on external data. E.g.,
//!   `spec_bfsmteph_wf(graph)` checks that a graph's adjacency list is well-formed.
//!
//! Example below: BoundedCounter (inner, Layer 1) with `spec_boundedcounter_wf`,
//! LockedBoundedCounter (wrapper, Layer 2) with `spec_specnamingconvention_wf`.

pub mod spec_naming_convention {

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    // Layer 1: Inner struct with datatype-level wf.

    pub struct BoundedCounter {
        pub value: u64,
        pub bound: u64,
    }

    impl View for BoundedCounter {
        type V = (nat, nat);
        open spec fn view(&self) -> (nat, nat) {
            (self.value as nat, self.bound as nat)
        }
    }

    pub trait BoundedCounterTrait: Sized + View<V = (nat, nat)> {
        // Datatype-level wf: spec_<datatype>_wf.
        spec fn spec_boundedcounter_wf(&self) -> bool;

        fn new(bound: u64) -> (s: Self)
            requires bound > 0,
            ensures s.spec_boundedcounter_wf(),
                    s@ == (0nat, bound as nat);

        fn increment(&mut self)
            requires old(self).spec_boundedcounter_wf(),
                     old(self)@.0 < old(self)@.1,
            ensures self.spec_boundedcounter_wf(),
                    self@ == (old(self)@.0 + 1, old(self)@.1);

        fn value(&self) -> (v: u64)
            requires self.spec_boundedcounter_wf(),
            ensures v as nat == self@.0;

        fn full(&self) -> (f: bool)
            requires self.spec_boundedcounter_wf(),
            ensures f == (self@.0 == self@.1);
    }

    impl BoundedCounterTrait for BoundedCounter {
        open spec fn spec_boundedcounter_wf(&self) -> bool {
            self.value <= self.bound && self.bound > 0
        }

        fn new(bound: u64) -> (s: Self) {
            BoundedCounter { value: 0, bound }
        }

        fn increment(&mut self) {
            self.value = self.value + 1;
        }

        fn value(&self) -> (v: u64) { self.value }

        fn full(&self) -> (f: bool) { self.value == self.bound }
    }

    // Layer 2: Locked wrapper with module-level wf.

    pub struct BoundedCounterInv;

    impl RwLockPredicate<BoundedCounter> for BoundedCounterInv {
        // Links to Layer 1 wf — enforced on every lock release/acquire.
        open spec fn inv(self, v: BoundedCounter) -> bool {
            v.spec_boundedcounter_wf()
        }
    }

    pub struct LockedBoundedCounter {
        pub(crate) locked_counter: RwLock<BoundedCounter, BoundedCounterInv>,
        pub(crate) ghost_locked_counter: Ghost<(nat, nat)>,
    }

    impl LockedBoundedCounter {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_counter@.0 <= self.ghost_locked_counter@.1
            && self.ghost_locked_counter@.1 > 0
        }

        pub closed spec fn spec_ghost_locked_counter(self) -> (nat, nat) {
            self.ghost_locked_counter@
        }
    }

    impl View for LockedBoundedCounter {
        type V = (nat, nat);
        open spec fn view(&self) -> (nat, nat) { self.spec_ghost_locked_counter() }
    }

    pub trait LockedBoundedCounterTrait: Sized + View<V = (nat, nat)> {
        // Module-level wf: spec_<module_no_underscores>_wf.
        spec fn spec_specnamingconvention_wf(&self) -> bool;

        fn new(bound: u64) -> (s: Self)
            requires bound > 0,
            ensures s.spec_specnamingconvention_wf(),
                    s@ == (0nat, bound as nat);

        fn increment(&mut self) -> (r: Result<(), ()>)
            requires old(self).spec_specnamingconvention_wf(),
            ensures self.spec_specnamingconvention_wf(),
                    match r {
                        Ok(_) => self@ == (old(self)@.0 + 1, old(self)@.1),
                        Err(_) => self@ == old(self)@,
                    };

        fn value(&self) -> (v: u64)
            requires self.spec_specnamingconvention_wf(),
            ensures v as nat == self@.0;

        fn full(&self) -> (f: bool)
            requires self.spec_specnamingconvention_wf(),
            ensures f == (self@.0 == self@.1);
    }

    impl LockedBoundedCounterTrait for LockedBoundedCounter {
        // Module-level wf: named after the module, not the struct.
        open spec fn spec_specnamingconvention_wf(&self) -> bool {
            self@.0 <= self@.1 && self@.1 > 0
        }

        fn new(bound: u64) -> (s: Self) {
            let bc = BoundedCounter::new(bound);
            LockedBoundedCounter {
                locked_counter: RwLock::new(bc, Ghost(BoundedCounterInv)),
                ghost_locked_counter: Ghost((0nat, bound as nat)),
            }
        }

        fn increment(&mut self) -> (r: Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_counter.acquire_write();
            proof { assume(self.ghost_locked_counter@ == locked_val@); }
            if locked_val.value < locked_val.bound {
                locked_val.increment();
                let ghost new_val = locked_val@;
                self.ghost_locked_counter = Ghost(new_val);
                write_handle.release_write(locked_val);
                Ok(())
            } else {
                write_handle.release_write(locked_val);
                Err(())
            }
        }

        fn value(&self) -> (v: u64) {
            let read_handle = self.locked_counter.acquire_read();
            let v = read_handle.borrow().value();
            proof { assume(v as nat == self@.0); }
            read_handle.release_read();
            v
        }

        fn full(&self) -> (f: bool) {
            let read_handle = self.locked_counter.acquire_read();
            let f = read_handle.borrow().full();
            proof { assume(f == (self@.0 == self@.1)); }
            read_handle.release_read();
            f
        }
    }

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for BoundedCounter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "BoundedCounter({}/{})", self.value, self.bound)
        }
    }
    impl std::fmt::Display for BoundedCounter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "BoundedCounter({}/{})", self.value, self.bound)
        }
    }

    impl std::fmt::Debug for BoundedCounterInv {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "BoundedCounterInv")
        }
    }
    impl std::fmt::Display for BoundedCounterInv {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "BoundedCounterInv")
        }
    }

    impl std::fmt::Debug for LockedBoundedCounter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedBoundedCounter")
        }
    }
    impl std::fmt::Display for LockedBoundedCounter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedBoundedCounter")
        }
    }
}
