//! Experiment: struct RwLock with function specs, Result return.
//! Hypothesis: Moving value preconditions to runtime checks with Result return.
//! RESULT: FAILS. Requires assumes to link ghost_locked_pos to inner value at lock boundary.

pub mod struct_rwlock_with_fn_specs_result {

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    pub struct Positive { pub pos: i64, pub ghost_pos: Ghost<int>, }

    impl View for Positive {
        type V = int;
        open spec fn view(&self) -> int { self.ghost_pos@ }
    }

    pub trait PositiveTrait: Sized + View<V = int> {
        spec fn spec_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s@ == 0, s.spec_wf();

        fn add_one(&mut self)
            requires old(self).spec_wf(),
                     old(self)@ < i64::MAX,
            ensures self.spec_wf(),
                    self@ == old(self)@ + 1;

        fn sub_one(&mut self)
            requires old(self).spec_wf(),
                     old(self)@ > 0,
            ensures self.spec_wf(),
                    self@ == old(self)@ - 1;

        fn value_i64(&self) -> (v: i64)
            requires self.spec_wf(),
            ensures v as int == self@;

        fn value_u64(&self) -> (v: u64)
            requires self.spec_wf(),
            ensures v as int == self@;
    }

    impl PositiveTrait for Positive {
        open spec fn spec_wf(&self) -> bool { self@ >= 0 && self.ghost_pos@ == self.pos as int }

        fn new() -> (s: Self) { Positive { pos: 0, ghost_pos: Ghost(0int) } }

        fn add_one(&mut self) {
            self.pos = self.pos + 1;
            self.ghost_pos = Ghost(self.pos as int);
        }

        fn sub_one(&mut self) {
            self.pos = self.pos - 1;
            self.ghost_pos = Ghost(self.pos as int);
        }

        fn value_i64(&self) -> (v: i64) { self.pos }

        fn value_u64(&self) -> (v: u64) { self.pos as u64 }
    }

    pub struct LockedPositiveInv;

    impl RwLockPredicate<Positive> for LockedPositiveInv {
        open spec fn inv(self, v: Positive) -> bool { v.spec_wf() }
    }

    pub struct LockedPositive {
        pub locked_pos: RwLock<Positive, LockedPositiveInv>,
        pub ghost_locked_pos: Ghost<int>,
    }

    impl View for LockedPositive {
        type V = int;
        open spec fn view(&self) -> int { self.ghost_locked_pos@ }
    }

    pub trait LockedPositiveTrait: Sized + View<V = int> {
        spec fn spec_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s.spec_wf(),
                    s@ == 0;

        fn add_one(&mut self) -> (r: Result<(), ()>)
            requires old(self).spec_wf(),
            ensures self.spec_wf(),
                    match r {
                        Ok(_) => self@ == old(self)@ + 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn sub_one(&mut self) -> (r: Result<(), ()>)
            requires old(self).spec_wf(),
            ensures self.spec_wf(),
                    match r {
                        Ok(_) => self@ == old(self)@ - 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn value_i64(&self) -> (v: i64)
            requires self.spec_wf(),
            ensures v as int == self@;

        fn value_u64(&self) -> (v: u64)
            requires self.spec_wf(),
            ensures v as int == self@;
    }

    impl LockedPositiveTrait for LockedPositive {
        open spec fn spec_wf(&self) -> bool { self@ >= 0 }

        fn new() -> (s: Self) {
            let pos = Positive::new();
            LockedPositive {
                locked_pos: RwLock::new(pos, Ghost(LockedPositiveInv)),
                ghost_locked_pos: Ghost(0int),
            }
        }

        fn add_one(&mut self) -> (r: Result<(), ()>) {
            let (mut inner, write_handle) = self.locked_pos.acquire_write();
            proof { assume(self.ghost_locked_pos@ == inner@); }
            if inner.pos < i64::MAX {
                inner.add_one();
                let ghost new_val = inner@;
                self.ghost_locked_pos = Ghost(new_val);
                write_handle.release_write(inner);
                Ok(())
            } else {
                write_handle.release_write(inner);
                Err(())
            }
        }

        fn sub_one(&mut self) -> (r: Result<(), ()>) {
            let (mut inner, write_handle) = self.locked_pos.acquire_write();
            proof { assume(self.ghost_locked_pos@ == inner@); }
            if inner.pos > 0 {
                inner.sub_one();
                let ghost new_val = inner@;
                self.ghost_locked_pos = Ghost(new_val);
                write_handle.release_write(inner);
                Ok(())
            } else {
                write_handle.release_write(inner);
                Err(())
            }
        }

        #[verifier::external_body]
        fn value_i64(&self) -> (v: i64) {
            let handle = self.locked_pos.acquire_read();
            let v = handle.borrow().value_i64();
            handle.release_read();
            v
        }

        #[verifier::external_body]
        fn value_u64(&self) -> (v: u64) {
            let handle = self.locked_pos.acquire_read();
            let v = handle.borrow().value_u64();
            handle.release_read();
            v
        }
    }

    } // verus!
}
