//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: token inside lock — predicate ties real to abstract.
//! Same operations as rwlock_tsm_increment. Token travels with the data
//! inside the RwLock. Ghost field on struct mirrors token value for spec access.
//! This is a mish mash of a real lock and a TSM. Not clean.
//! RESULT: 1 trust point per op. 

pub mod rwlock_inc_token_inside_lock {

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

    // Predicate ties real u64 to token value and instance inside the lock.
    pub struct CounterInv {
        pub ghost inst: Counter::Instance,
    }

    impl RwLockPredicate<(u64, Tracked<Counter::value>)> for CounterInv {
        open spec fn inv(self, v: (u64, Tracked<Counter::value>)) -> bool {
            v.0 as int == v.1@.value()
            && v.1@.instance_id() == self.inst.id()
        }
    }

    pub struct LockedCounter {
        pub lock: Arc<RwLock<(u64, Tracked<Counter::value>), CounterInv>>,
        pub inst: Tracked<Counter::Instance>,
        pub ghost_val: Ghost<u64>,
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
        // Predicate's instance matches struct's instance.
        open spec fn wf(&self) -> bool {
            self.lock.pred().inst == self.inst@
        }

        open spec fn spec_value(&self) -> u64 {
            self.ghost_val@
        }

        fn new() -> (s: Self) {
            let tracked (Tracked(inst), Tracked(token)) = Counter::Instance::initialize();
            let ghost pred = CounterInv { inst: inst };
            let ghost_val: Ghost<u64> = Ghost(0u64);
            LockedCounter {
                lock: new_arc_rwlock::<(u64, Tracked<Counter::value>), CounterInv>(
                    (0u64, Tracked(token)), Ghost(pred)),
                inst: Tracked(inst),
                ghost_val,
            }
        }

        fn add_one(&mut self) {
            let (pair, write_handle) = self.lock.acquire_write();
            let val = pair.0;
            let tracked mut token;
            proof { token = pair.1.get(); }
            // Predicate gives: val as int == token.value().
            // Need: token.value() == ghost_val@ (maintained by protocol).
            proof {
                assume(self.ghost_val@ as int == token.value());
                self.inst.borrow().add_one(&mut token);
            }
            let ghost new_val = token.value();
            self.ghost_val = Ghost(new_val);
            write_handle.release_write((val + 1, Tracked(token)));
        }

        fn sub_one(&mut self) {
            let (pair, write_handle) = self.lock.acquire_write();
            let val = pair.0;
            let tracked mut token;
            proof { token = pair.1.get(); }
            proof {
                assume(self.ghost_val@ as int == token.value());
                self.inst.borrow().sub_one(&mut token);
            }
            let ghost new_val = token.value();
            self.ghost_val = Ghost(new_val);
            write_handle.release_write((val - 1, Tracked(token)));
        }

        fn zero_out(&mut self) {
            let (pair, write_handle) = self.lock.acquire_write();
            let tracked mut token;
            proof { token = pair.1.get(); }
            proof {
                assume(self.ghost_val@ as int == token.value());
                self.inst.borrow().zero_out(&mut token);
            }
            let ghost new_val = token.value();
            self.ghost_val = Ghost(new_val);
            write_handle.release_write((0u64, Tracked(token)));
        }

        #[verifier::external_body]
        fn read_value(&self) -> (v: u64) {
            let handle = self.lock.acquire_read();
            let v = handle.borrow().0;
            handle.release_read();
            v
        }
    }

    } // verus!
}
