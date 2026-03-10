//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: struct RwLock Arc with function specs.
//! Hypothesis: I can spec out fns over Arc over RwLock over a struct.
//! RESULT: I can't prove the upper layer over the RwLock without assumes/external_body.

pub mod struct_rwlock_arc_with_fn_specs {

    use std::sync::Arc;

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
        open spec fn inv(self, v: Positive) -> bool {
            v.ghost_pos@ == v.pos as int && v@ >= 0
        }
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
            LockedPositive {
                locked_pos: RwLock::new(Positive::new(), Ghost(LockedPositiveInv)),
                ghost_locked_pos: Ghost(0int),
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

    pub struct ArcLockedPositive {
        pub inner: Arc<LockedPositive>,
    }

    impl View for ArcLockedPositive {
        type V = int;
        #[verifier::external_body]
        open spec fn view(&self) -> int { 0 }
    }

    pub trait ArcLockedPositiveTrait: Sized + View<V = int> {
        spec fn spec_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s@ == 0, s.spec_wf();

        fn add_one(&mut self)
            requires old(self)@ < i64::MAX, old(self).spec_wf(),
            ensures self@ == old(self)@ + 1, self.spec_wf();

        fn sub_one(&mut self)
            requires old(self)@ > 0, old(self).spec_wf(),
            ensures self@ == old(self)@ - 1, self.spec_wf();

        fn value_i64(&self) -> (v: i64)
            requires self.spec_wf(),
            ensures v as int == self@;

        fn value_u64(&self) -> (v: u64)
            requires self.spec_wf(),
            ensures v as int == self@;
    }

    impl ArcLockedPositiveTrait for ArcLockedPositive {
        open spec fn spec_wf(&self) -> bool { self@ >= 0 }

        #[verifier::external_body]
        fn new() -> (s: Self) {
            ArcLockedPositive {
                inner: Arc::new(LockedPositive::new()),
            }
        }

        #[verifier::external_body]
        fn add_one(&mut self) {
            Arc::get_mut(&mut self.inner).unwrap().add_one();
        }

        #[verifier::external_body]
        fn sub_one(&mut self) {
            Arc::get_mut(&mut self.inner).unwrap().sub_one();
        }

        #[verifier::external_body]
        fn value_i64(&self) -> (v: i64) {
            self.inner.value_i64()
        }

        #[verifier::external_body]
        fn value_u64(&self) -> (v: u64) {
            self.inner.value_u64()
        }
    }

    } // verus!
}
