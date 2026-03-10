//! Experiment: struct RwLock with function specs, with ghost n@.
//! Hypothesis: I can spec out fns over RwLock over a struct.
//! RESULT: FAILS. Requires assumes and external_body to bridge the lock boundary.

pub mod struct_rwlock_with_fn_specs {

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

    impl LockedPositiveTrait for LockedPositive {
        open spec fn spec_wf(&self) -> bool { self@ >= 0 }

        fn new() -> (s: Self) {
            let pos = Positive::new();
            LockedPositive {
                locked_pos: RwLock::new(pos, Ghost(LockedPositiveInv)),
                ghost_locked_pos: Ghost(0int)
            }
        }

        fn add_one(&mut self) {
            let (mut inner, write_handle) = self.locked_pos.acquire_write();
            proof { assume(self.ghost_locked_pos@ == inner@); }
            inner.add_one();
            let ghost new_val = inner@;
            self.ghost_locked_pos = Ghost(new_val);
            write_handle.release_write(inner);
        }

        fn sub_one(&mut self) {
            let (mut inner, write_handle) = self.locked_pos.acquire_write();
            proof { assume(self.ghost_locked_pos@ == inner@); }
            inner.sub_one();
            let ghost new_val = inner@;
            self.ghost_locked_pos = Ghost(new_val);
            write_handle.release_write(inner);
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
