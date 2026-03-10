//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: RwLock with type_invariant linking ghost to inner value.
//! Hypothesis: A type_invariant on LockedCountDown can link its ghost to the
//! value inside the RwLock, eliminating assumes at the lock boundary but is optimistic:
//! it checks once it has the lock.
//! Failed, can't use type invariant on &mut. 

pub mod struct_rwlock_type_invariant {
    use vstd::prelude::*;
    use vstd::rwlock::*;
    verus! {
    pub struct CountDown { pub count: i64, }

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
        open spec fn spec_countdown_wf(&self) -> bool      { self@ >= 0 }
        fn new(v: u64)                        -> (s: Self) { CountDown { count: v as i64 } }
        fn count_down(&mut self)                           { self.count = self.count - 1; }
        fn count(&self)                       -> (v: u64)  { self.count as u64 }
        fn done(&self)                        -> (d: bool) { self.count == 0 }
    }

    pub struct LockedCountDownInv;

    impl RwLockPredicate<CountDown> for LockedCountDownInv {
        open spec fn inv(self, v: CountDown) -> bool { v.spec_countdown_wf() }
    }

    pub struct LockedCountDown {
        pub(crate) locked_count: RwLock<CountDown, LockedCountDownInv>,
        pub(crate) ghost_locked_count: Ghost<int>,
    }

    impl LockedCountDown {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_count@ >= 0
        }

        pub closed spec fn spec_ghost_locked_count(self) -> int { self.ghost_locked_count@ }
    }

    impl View for LockedCountDown {
        type V = int;
        open spec fn view(&self) -> int { self.spec_ghost_locked_count() }
    }

    pub trait LockedCountDownTrait: Sized + View<V = int> {
        spec fn spec_lockedcountdown_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            requires v <= i64::MAX as u64,
            ensures s.spec_lockedcountdown_wf(),
                    s@ == v as int;

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
                locked_count: RwLock::new(cd, Ghost(LockedCountDownInv)),
                ghost_locked_count: Ghost(v as int),
            }
        }

        fn count_down(&mut self) -> (r: Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_count.acquire_write();
            proof { assume(self.ghost_locked_count@ == locked_val@); }
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

        fn count(&self) -> (v: u64) {
            let read_handle = self.locked_count.acquire_read();
            let v = read_handle.borrow().count();
            proof { assume(v as int == self@); }
            read_handle.release_read();
            v
        }

        fn done(&self) -> (d: bool) {
            let read_handle = self.locked_count.acquire_read();
            let d = read_handle.borrow().done();
            proof { assume(d == (self@ == 0)); }
            read_handle.release_read();
            d
        }
    }

    } // verus!

    #[test]
    fn test_countdown() {
        let mut cd = LockedCountDown::new(3);

        // Count down from 3 to 0, checking each step.
        assert!(cd.count_down().is_ok());
        assert_eq!(cd.count(), 2);
        assert!(!cd.done());

        assert!(cd.count_down().is_ok());
        assert_eq!(cd.count(), 1);
        assert!(!cd.done());

        assert!(cd.count_down().is_ok());
        assert_eq!(cd.count(), 0);
        assert!(cd.done());

        // At zero, count_down fails — someone got there first.
        assert!(cd.count_down().is_err());
        assert_eq!(cd.count(), 0);
        assert!(cd.done());
    }
}
