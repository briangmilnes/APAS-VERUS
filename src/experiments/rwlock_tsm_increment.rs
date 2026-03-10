//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: TSM tracking abstract value + vstd RwLock for real synchronization.
//! Three operations (add_one, sub_one, zero_out) with provable specs.
//! Hypothesis: coarse-grained lock caller can see specs via ghost TSM token.
//!
//! Result: TSM proves abstract state transitions correct. One assume per method
//! bridges real lock value to abstract token value. read_value needs external_body.

pub mod rwlock_tsm_increment {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use verus_state_machines_macros::tokenized_state_machine;

    tokenized_state_machine!(Counter {
        fields {
            #[sharding(variable)]
            pub value: u64,
        }

        init!{
            initialize() {
                init value = 0;
            }
        }

        #[inductive(initialize)]
        fn initialize_inductive(post: Self) { }

        transition!{
            add_one() {
                require(pre.value < u64::MAX);
                update value = (pre.value + 1) as u64;
            }
        }

        transition!{
            sub_one() {
                require(pre.value > 0);
                update value = (pre.value - 1) as u64;
            }
        }

        transition!{
            zero_out() {
                update value = 0u64;
            }
        }

        #[inductive(add_one)]
        fn add_one_inductive(pre: Self, post: Self) { }

        #[inductive(sub_one)]
        fn sub_one_inductive(pre: Self, post: Self) { }

        #[inductive(zero_out)]
        fn zero_out_inductive(pre: Self, post: Self) { }
    });

    verus! {

    pub struct CounterInv;

    impl RwLockPredicate<u64> for CounterInv {
        open spec fn inv(self, v: u64) -> bool {
            v <= u64::MAX
        }
    }

    pub struct LockedCounter {
        pub lock: Arc<RwLock<u64, CounterInv>>,
        pub inst: Tracked<Counter::Instance>,
        pub token: Tracked<Counter::value>,
    }

    pub trait LockedCounterTrait: Sized {
        spec fn wf(&self) -> bool;

        spec fn spec_value(&self) -> u64;

        fn new() -> (s: Self)
            ensures s.spec_value() == 0, s.wf();

        fn add_one(&mut self)
            requires old(self).spec_value() < u64::MAX, old(self).wf(),
            ensures self.spec_value() == old(self).spec_value() + 1, self.wf();

        fn sub_one(&mut self)
            requires old(self).spec_value() > 0, old(self).wf(),
            ensures self.spec_value() == old(self).spec_value() - 1, self.wf();

        fn zero_out(&mut self)
            requires old(self).wf(),
            ensures self.spec_value() == 0, self.wf();

        fn read_value(&self) -> (v: u64)
            requires self.wf(),
            ensures v == self.spec_value();
    }

    impl LockedCounterTrait for LockedCounter {
        // Instance and token come from the same initialization.
        open spec fn wf(&self) -> bool {
            self.token@.instance_id() == self.inst@.id()
        }

        open spec fn spec_value(&self) -> u64 {
            self.token@.value()
        }

        fn new() -> (s: Self) {
            let tracked (Tracked(inst), Tracked(token)) = Counter::Instance::initialize();
            LockedCounter {
                lock: new_arc_rwlock::<u64, CounterInv>(0u64, Ghost(CounterInv)),
                inst: Tracked(inst),
                token: Tracked(token),
            }
        }

        fn add_one(&mut self) {
            let (val, write_handle) = self.lock.acquire_write();
            proof {
                // Trust: real lock value matches abstract token value.
                assume(val as int == self.token@.value());
                self.inst.borrow().add_one(self.token.borrow_mut());
            }
            write_handle.release_write(val + 1);
        }

        fn sub_one(&mut self) {
            let (val, write_handle) = self.lock.acquire_write();
            proof {
                assume(val as int == self.token@.value());
                self.inst.borrow().sub_one(self.token.borrow_mut());
            }
            write_handle.release_write(val - 1);
        }

        fn zero_out(&mut self) {
            let (_val, write_handle) = self.lock.acquire_write();
            proof {
                self.inst.borrow().zero_out(self.token.borrow_mut());
            }
            write_handle.release_write(0u64);
        }

        #[verifier::external_body]
        fn read_value(&self) -> (v: u64) {
            let handle = self.lock.acquire_read();
            let v = *handle.borrow();
            handle.release_read();
            v
        }
    }

    } // verus!
}
