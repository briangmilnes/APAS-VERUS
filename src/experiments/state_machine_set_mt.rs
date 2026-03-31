//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: state machine protocol for Mt set — zero assumes.
//!
//! Based on the SOSP tutorial "counting to 2" pattern.
//! Token lives INSIDE the RwLock. Predicate ties concrete to abstract.
//! After acquire, predicate PROVES equality. Zero assumes.

pub mod state_machine_set_mt {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use verus_state_machines_macros::tokenized_state_machine;

    // State machine OUTSIDE verus! (like rwlock_inc_token_inside_lock).
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

            #[invariant]
            pub fn the_invariant(&self) -> bool { true }

            #[inductive(initialize)]
            fn initialize_inductive(post: Self) { }

            #[inductive(tr_insert)]
            fn tr_insert_inductive(pre: Self, post: Self) { }
        }
    );

    verus! {

    // Inner StEph type.
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

    // Lock interior: concrete StEph + ghost token.
    pub struct SetLockInterior {
        pub steph: SetStEph,
        pub ghost_count: Tracked<SetSM::count>,
    }

    // Predicate ties concrete to abstract.
    pub ghost struct SetMtInv {
        pub instance: SetSM::Instance,
    }

    impl RwLockPredicate<SetLockInterior> for SetMtInv {
        open spec fn inv(self, interior: SetLockInterior) -> bool {
            interior.steph@.len() == interior.ghost_count@.value()
            && interior.ghost_count@.instance_id() == self.instance.id()
        }
    }

    // Mt type.
    pub struct SetMtEph {
        pub lock: RwLock<SetLockInterior, SetMtInv>,
        pub inst: Tracked<SetSM::Instance>,
    }

    impl SetMtEph {
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

            let steph = SetStEph::empty();
            let interior = SetLockInterior {
                steph,
                ghost_count: Tracked(count_token),
            };

            SetMtEph {
                lock: RwLock::new(interior, Ghost(SetMtInv { instance })),
                inst: Tracked(instance),
            }
        }

        pub fn mt_insert(&self, val: u64)
            requires self.wf(),
        {
            let (mut interior, write_handle) = self.lock.acquire_write();

            // Step state machine: count → count + 1.
            proof {
                self.inst.borrow().tr_insert(
                    &mut *interior.ghost_count.borrow_mut(),
                );
            }

            // Mutate concrete.
            interior.steph.insert(val);

            // Release — predicate checks len() == count.
            write_handle.release_write(interior);
        }

        pub fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let read_handle = self.lock.acquire_read();
            let interior = read_handle.borrow();

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
