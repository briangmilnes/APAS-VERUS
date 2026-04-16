// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! RwLock + TSM Standard: zero-assume locking for Mt modules.
//!
//! This standard replaces `toplevel_coarse_rwlocks_for_mt_modules.rs` for modules
//! migrated to the TSM pattern. The old standard uses ghost fields + accepts at
//! the lock boundary. This standard uses a tokenized state machine (TSM) inside
//! the lock, eliminating all ghost-lock bridge assumes.
//!
//! Architecture:
//! - Layer 1 (inner struct): fully verified, no ghost, no lock. Same as before.
//! - Layer 2 (locked wrapper): RwLock<(Inner, TSM::token), Inv>. The TSM token
//!   lives inside the lock alongside the data. The RwLockPredicate ties them:
//!   `token.value() == data.abstract_state()`. After acquire, the predicate is
//!   PROVED — no accept needed for the ghost-lock bridge.
//!
//! Trust profile:
//! - Layer 1: zero trust, fully verified. Same as before.
//! - Layer 2: 2 accepts per file for the View bridge (ghost View ↔ TSM token).
//!   No ghost-lock bridge assumes. No reader result accepts. No return value
//!   accepts. The TSM proves all of these.
//!
//! The 2 View bridge accepts:
//! - Writer accept: after release_write, the external ghost View must equal the
//!   new token value. The caller can't verify this because it just released the lock.
//! - Reader accept: after release_read, the return value must correspond to the
//!   View the caller had before the read.
//! These are correct by induction on &mut self ownership (sole access during write,
//! ghost set at previous release). See architecture doc section 3.4.
//!
//! What TSM eliminates vs the old standard:
//! - Ghost-lock bridge assumes (ghost_field@ == locked_data@): GONE
//! - Reader result assumes (found == spec_contains(target)): GONE
//! - Return value assumes (size == spec_size()): GONE
//! - Extrema assumes (min.is_some() ==> contains(min)): GONE
//!
//! What stays (Verus limitations, same as old standard):
//! - Clone bridge assume in Clone::clone body
//! - PartialEq bridge assume in PartialEq::eq body
//! - Iterator invariant assume in Iterator::next body
//! - assume(false); diverge() in unreachable thread-join error arms
//!
//! TSM definition lives OUTSIDE verus! (tokenized_state_machine! is a proc macro).
//! Everything else lives INSIDE verus!.
//!
//! Naming conventions:
//! - TSM: <TypeName>SM (CdStateMachine, BSTPlainSM)
//! - Lock interior: <TypeName>LockInterior
//! - Predicate: <TypeName>TSMInv (CdStateMachineInv) — NOT the old pattern <Name>Inv
//! - TSM instance field: inst: Tracked<SM::Instance>
//! - TSM token inside lock: ghost_token: Tracked<SM::field_name>
//!
//! References:
//! - src/experiments/bst_plain_mt_tsm.rs — 10-operation example, zero assumes
//! - docs/architecture-coarse-lock-parallel-mt.md — full architecture discussion
//! - src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs — old standard (ghost+accepts)

pub mod rwlock_tsm_standard {

    use vstd::prelude::*;
    use vstd::rwlock::{RwLock, RwLockPredicate};

    use verus_state_machines_macros::tokenized_state_machine;

    // TSM definition: OUTSIDE verus!.
    // One field per tracked abstract value. One transition per mutating operation.
    // Read-only operations need no transition — the predicate proves the result.
    tokenized_state_machine!(
        CdStateMachine {
            fields {
                #[sharding(variable)]
                pub count: nat,
            }

            init!{
                initialize(initial_count: nat) {
                    init count = initial_count;
                }
            }

            transition!{
                tr_count_down() {
                    require(pre.count > 0);
                    update count = (pre.count - 1) as nat;
                }
            }

            #[invariant]
            pub fn the_invariant(&self) -> bool { true }

            #[inductive(initialize)]
            fn initialize_inductive(post: Self, initial_count: nat) { }

            #[inductive(tr_count_down)]
            fn tr_count_down_inductive(pre: Self, post: Self) { }
        }
    );

    verus! {

    // Layer 1: Inner struct. Identical to the old standard.

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

    // Layer 2: RwLock + TSM. Zero ghost-lock assumes.

    // Lock interior: inner data + TSM token. Both travel together inside the lock.
    pub struct CountDownLockInterior {
        pub inner: CountDown,
        pub ghost_token: Tracked<CdStateMachine::count>,
    }

    // Predicate: token value == concrete value. Proved on every acquire.
    pub ghost struct CdStateMachineInv {
        pub instance: CdStateMachine::Instance,
    }

    impl RwLockPredicate<CountDownLockInterior> for CdStateMachineInv {
        open spec fn inv(self, v: CountDownLockInterior) -> bool {
            // The TSM token tracks the concrete count.
            v.inner@ == v.ghost_token@.value()
            // The token belongs to our instance.
            && v.ghost_token@.instance_id() == self.instance.id()
            // The inner value is well-formed.
            && v.inner.spec_countdown_wf()
        }
    }

    // Locked wrapper: lock + TSM instance.
    // No ghost field — the TSM token inside the lock IS the ghost state.
    pub struct LockedCdStateMachine {
        pub lock: RwLock<CountDownLockInterior, CdStateMachineInv>,
        pub inst: Tracked<CdStateMachine::Instance>,
    }

    impl LockedCdStateMachine {
        pub open spec fn wf(&self) -> bool {
            self.lock.pred().instance == self.inst@
        }
    }

    pub trait LockedCdStateMachineTrait: Sized {
        spec fn spec_lockedcountdowntsm_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            requires v <= i64::MAX as u64,
            ensures s.spec_lockedcountdowntsm_wf();

        fn count_down(&self) -> (r: Result<(), ()>)
            requires self.spec_lockedcountdowntsm_wf();

        fn count(&self) -> (v: u64)
            requires self.spec_lockedcountdowntsm_wf();

        fn done(&self) -> (d: bool)
            requires self.spec_lockedcountdowntsm_wf();
    }

    impl LockedCdStateMachineTrait for LockedCdStateMachine {
        open spec fn spec_lockedcountdowntsm_wf(&self) -> bool { self.wf() }

        fn new(v: u64) -> (s: Self) {
            let tracked (
                Tracked(instance),
                Tracked(count_token),
            ) = CdStateMachine::Instance::initialize(v as nat);

            let interior = CountDownLockInterior {
                inner: CountDown::new(v),
                ghost_token: Tracked(count_token),
            };

            LockedCdStateMachine {
                lock: RwLock::new(interior, Ghost(CdStateMachineInv { instance })),
                inst: Tracked(instance),
            }
        }

        // Writer: acquire, check precondition, mutate, step TSM, release.
        // ZERO assumes. The predicate proves token == inner on acquire.
        // The TSM transition proves the new token == new inner on release.
        fn count_down(&self) -> (r: Result<(), ()>) {
            let (mut interior, write_handle) = self.lock.acquire_write();

            // Predicate proves: interior.inner@ == interior.ghost_token@.value()
            // AND interior.inner.spec_countdown_wf(). NO ASSUME.

            if interior.inner.count > 0 {
                interior.inner.count_down();

                // Step the TSM token. This proves the new token value
                // matches the new inner value.
                proof {
                    self.inst.borrow().tr_count_down(
                        &mut *interior.ghost_token.borrow_mut(),
                    );
                }

                write_handle.release_write(interior);
                Ok(())
            } else {
                write_handle.release_write(interior);
                Err(())
            }
        }

        // Reader: acquire, read from real data, release.
        // ZERO assumes. The predicate guarantees wf. The inner fn's
        // ensures chain through the concrete data — no ghost needed.
        fn count(&self) -> (v: u64) {
            let read_handle = self.lock.acquire_read();
            let v = read_handle.borrow().inner.count();
            // NO accept needed. v == inner@ is proved by CountDown::count's ensures.
            // inner.spec_countdown_wf() is guaranteed by the predicate.
            read_handle.release_read();
            v
        }

        fn done(&self) -> (d: bool) {
            let read_handle = self.lock.acquire_read();
            let d = read_handle.borrow().inner.done();
            // NO accept needed. d == (inner@ == 0) from CountDown::done's ensures.
            read_handle.release_read();
            d
        }
    }

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for CountDown {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CountDown({})", self.count)
        }
    }
    impl std::fmt::Display for CountDown {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CountDown({})", self.count)
        }
    }

    impl std::fmt::Debug for CountDownLockInterior {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CountDownLockInterior")
        }
    }
    impl std::fmt::Display for CountDownLockInterior {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CountDownLockInterior")
        }
    }

    impl std::fmt::Debug for LockedCdStateMachine {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedCdStateMachine")
        }
    }
    impl std::fmt::Display for LockedCdStateMachine {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedCdStateMachine")
        }
    }
}
