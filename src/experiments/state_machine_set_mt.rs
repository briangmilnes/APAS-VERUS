//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: state machine protocol for Mt set — zero assumes.
//!
//! Based on the SOSP tutorial "counting to 2" pattern:
//!   - tokenized_state_machine defines the abstract set state
//!   - Token lives INSIDE the RwLock alongside the concrete StEph
//!   - RwLockPredicate ties concrete view to token value
//!   - After acquire, predicate PROVES concrete == abstract
//!   - State machine transitions prove operations are valid
//!   - ZERO assumes
//!
//! Architecture:
//!   LockInterior = (StEph, Tracked<SetSM::contents>)
//!   RwLockPredicate: inner.steph@ == token.value
//!   Mt operations: acquire → step token → mutate steph → release

pub mod state_machine_set_mt {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use verus_state_machines_macros::tokenized_state_machine;

    // ================================================================
    // 1. The state machine — tracks the abstract set contents.
    //    Must be OUTSIDE verus! (macro generates its own verus blocks).
    // ================================================================

    tokenized_state_machine!(
        SetSM {
            fields {
                // The abstract contents — a sequence of elements.
                #[sharding(variable)]
                pub contents: Seq<u64>,
            }

            init!{
                initialize() {
                    init contents = Seq::<u64>::empty();
                }
            }

            transition!{
                tr_insert(val: u64) {
                    update contents = pre.contents.push(val);
                }
            }

            // Property: contents length is bounded.
            property!{
                contents_bounded() {
                    assert pre.contents.len() >= 0;
                }
            }

            #[invariant]
            pub fn the_invariant(&self) -> bool {
                true  // No additional invariant needed for this simple case.
            }

            #[inductive(initialize)]
            fn initialize_inductive(post: Self) { }

            #[inductive(tr_insert)]
            fn tr_insert_inductive(pre: Self, post: Self, val: u64) { }
        }
    );

    verus! {

    // ================================================================
    // 2. The inner StEph type — a simple set backed by Vec<u64>.
    // ================================================================

    pub struct SetStEph {
        pub elements: Vec<u64>,
    }

    impl View for SetStEph {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

    impl SetStEph {
        pub fn empty() -> (s: Self)
            ensures s@.len() == 0,
        {
            SetStEph { elements: Vec::new() }
        }

        pub fn insert(&mut self, val: u64)
            ensures self@.len() == old(self)@.len() + 1,
                    self@.last() == val,
        {
            self.elements.push(val);
        }

        pub fn size(&self) -> (n: usize)
            ensures n == self@.len(),
        {
            self.elements.len()
        }
    }

    // ================================================================
    // 3. Lock interior: concrete StEph + ghost token, tied by predicate.
    // ================================================================

    pub struct SetLockInterior {
        pub steph: SetStEph,
        pub ghost_contents: Tracked<SetSM::contents>,
    }

    pub ghost struct SetMtInv {
        pub instance: SetSM::Instance,
    }

    impl RwLockPredicate<SetLockInterior> for SetMtInv {
        open spec fn inv(self, interior: SetLockInterior) -> bool {
            // THE CONNECTION: concrete view == abstract token value.
            // This is what eliminates all assumes.
            interior.steph@ == interior.ghost_contents@@.value
            && interior.ghost_contents@@.instance == self.instance
        }
    }

    // ================================================================
    // 4. The Mt type — RwLock around the interior. No ghost field.
    // ================================================================

    pub struct SetMtEph {
        pub lock: RwLock<SetLockInterior, SetMtInv>,
        pub inst: Tracked<SetSM::Instance>,
    }

    // View: we still need to express self@ for specs.
    // But we can't read through the lock in a spec.
    // Option A: no View, express specs on return values only.
    // Option B: ghost field that tracks contents.
    //
    // For this experiment: Option A — no View on the Mt type.
    // Specs express "what the operation returns" not "what self@ is."

    pub trait SetMtEphTrait: Sized {
        fn new_empty() -> (s: Self);

        fn mt_insert(&self, val: u64);

        fn mt_size(&self) -> (n: usize);
    }

    impl SetMtEphTrait for SetMtEph {
        fn new_empty() -> (s: Self) {
            let tracked (
                Tracked(instance),
                Tracked(contents_token),
            ) = SetSM::Instance::initialize();

            let steph = SetStEph::empty();
            let interior = SetLockInterior {
                steph,
                ghost_contents: Tracked(contents_token),
            };

            SetMtEph {
                lock: RwLock::new(interior, Ghost(SetMtInv { instance })),
                inst: Tracked(instance),
            }
        }

        fn mt_insert(&self, val: u64) {
            let (mut interior, write_handle) = self.lock.acquire_write();

            // The predicate PROVES: interior.steph@ == token.value.
            // No assume needed!

            // Step the state machine.
            proof {
                self.inst.borrow().tr_insert(
                    val as u64,
                    &mut *interior.ghost_contents.borrow_mut(),
                );
            }

            // Mutate the concrete StEph — must match the transition.
            interior.steph.insert(val);

            // Release — predicate re-checked: steph@ == token.value (after push).
            write_handle.release_write(interior);
        }

        fn mt_size(&self) -> (n: usize) {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();

            // The predicate PROVES: interior.steph@ == token.value.
            // We can read steph.size() with full confidence — no assume!
            let n = interior.steph.size();

            read_handle.release_read();
            n
        }
    }

    } // verus!

    #[test]
    fn test_state_machine_set() {
        let s = SetMtEph::new_empty();
        assert_eq!(s.mt_size(), 0);
        s.mt_insert(42);
        assert_eq!(s.mt_size(), 1);
        s.mt_insert(99);
        assert_eq!(s.mt_size(), 2);
    }
}
