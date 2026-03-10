//! Experiment: struct RwLock with function specs, Result return, handle-based layers.
//! Layer 1: Positive — fully verified struct with specs.
//! Layer 2: LockedPositive trait fns take the inner data from the lock, full specs.
//! Layer 3: Fns acquire the lock, check preconditions, call Layer 2, return Result.
//!          ghost_locked_pos tracks value for Layer 3 ensures.
//! RESULT: FAILS. Layer 2 is zero-trust, but Layer 3 requires assumes to link ghost to inner.

pub mod struct_rwlock_with_fn_specs_result_handles {

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    // Layer 1: Positive (fully verified).

    #[derive(Clone, Copy)]
    pub struct Positive { pub pos: i64, pub ghost_pos: Ghost<int>, }

    impl View for Positive {
        type V = int;
        open spec fn view(&self) -> int { self.ghost_pos@ }
    }

    pub trait PositiveTrait: Sized + View<V = int> {
        spec fn spec_positive_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s@ == 0, s.spec_positive_wf();

        fn add_one(&mut self)
            requires old(self).spec_positive_wf(),
                     old(self)@ < i64::MAX,
            ensures self.spec_positive_wf(),
                    self@ == old(self)@ + 1;

        fn sub_one(&mut self)
            requires old(self).spec_positive_wf(),
                     old(self)@ > 0,
            ensures self.spec_positive_wf(),
                    self@ == old(self)@ - 1;

        fn value_u64(&self) -> (v: u64)
            requires self.spec_positive_wf(),
            ensures v as int == self@;
    }

    impl PositiveTrait for Positive {
        open spec fn spec_positive_wf(&self) -> bool { self@ >= 0 && self.ghost_pos@ == self.pos as int }

        fn new() -> (s: Self) { Positive { pos: 0, ghost_pos: Ghost(0int) } }

        fn add_one(&mut self) {
            self.pos = self.pos + 1;
            self.ghost_pos = Ghost(self.pos as int);
        }

        fn sub_one(&mut self) {
            self.pos = self.pos - 1;
            self.ghost_pos = Ghost(self.pos as int);
        }

        fn value_u64(&self) -> (v: u64) { self.pos as u64 }
    }

    // Layer 2: Fns take the inner Positive (from the lock), call Layer 1.
    // Predicate guarantees locked_val.spec_positive_wf(). Full specs, zero trust.

    pub struct LockedPositiveInv;

    impl RwLockPredicate<Positive> for LockedPositiveInv {
        open spec fn inv(self, v: Positive) -> bool {
            v.spec_positive_wf()
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

    pub trait LockedPositiveTrait: Sized {
        spec fn spec_locked_positive_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s.spec_locked_positive_wf();

        fn add_one(&self, locked_val: &mut Positive)
            requires old(locked_val).spec_positive_wf(),
                     old(locked_val)@ < i64::MAX,
            ensures locked_val.spec_positive_wf(),
                    locked_val@ == old(locked_val)@ + 1;

        fn sub_one(&self, locked_val: &mut Positive)
            requires old(locked_val).spec_positive_wf(),
                     old(locked_val)@ > 0,
            ensures locked_val.spec_positive_wf(),
                    locked_val@ == old(locked_val)@ - 1;

        fn value_u64(&self, locked_val: &Positive) -> (v: u64)
            requires locked_val.spec_positive_wf(),
            ensures v as int == locked_val@;
    }

    impl LockedPositiveTrait for LockedPositive {
        open spec fn spec_locked_positive_wf(&self) -> bool { 
                        self.ghost_locked_pos >= 0 
                     && self.ghost_locked_pos <= u64::MAX
                   }

        fn new() -> (s: Self) {
            let pos = Positive::new();
            LockedPositive {
                locked_pos: RwLock::new(pos, Ghost(LockedPositiveInv)),
                ghost_locked_pos: pos.ghost_pos
            }
        }

        fn add_one(&self, locked_val: &mut Positive) { locked_val.add_one(); }
        fn sub_one(&self, locked_val: &mut Positive) { locked_val.sub_one(); }
        fn value_u64(&self, locked_val: &Positive) -> (v: u64) { locked_val.value_u64() }
    }

    // Layer 3: Acquires the lock, checks preconditions in exec code, calls Layer 2.
    // Returns Result. Value specs via ghost_locked_pos.

    pub trait LockedPositiveResultTrait: Sized + View<V = int> {
        spec fn spec_lockedpositiveresult_wf(&self) -> bool;

        fn add_one(&mut self) -> (r: Result<(), ()>)
            requires old(self).spec_lockedpositiveresult_wf(),
            ensures self.spec_lockedpositiveresult_wf(),
                    match r {
                        Ok(_) => self@ == old(self)@ + 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn sub_one(&mut self) -> (r: Result<(), ()>)
            requires old(self).spec_lockedpositiveresult_wf(),
            ensures self.spec_lockedpositiveresult_wf(),
                    match r {
                        Ok(_) => self@ == old(self)@ - 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn value_u64(&self) -> (v: u64)
            requires self.spec_lockedpositiveresult_wf(),
            ensures v as int == self@;
    }

    impl LockedPositiveResultTrait for LockedPositive {
        open spec fn spec_lockedpositiveresult_wf(&self) -> bool { 
                         self@ >= 0
                      && self@ <= u64::MAX
                   }

        fn add_one(&mut self) -> (r: Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_pos.acquire_write();
            proof { assume(self.ghost_locked_pos@ == locked_val@); }
            if locked_val.pos < i64::MAX {
                LockedPositiveTrait::add_one(self, &mut locked_val);
                let ghost new_val = locked_val@;
                self.ghost_locked_pos = Ghost(new_val);
                write_handle.release_write(locked_val);
                Ok(())
            } else {
                write_handle.release_write(locked_val);
                Err(())
            }
        }

        fn sub_one(&mut self) -> (r: Result<(), ()>) {
            let (mut locked_val, write_handle) = self.locked_pos.acquire_write();
            proof { assume(self.ghost_locked_pos@ == locked_val@); }
            if locked_val.pos > 0 {
                LockedPositiveTrait::sub_one(self, &mut locked_val);
                let ghost new_val = locked_val@;
                self.ghost_locked_pos = Ghost(new_val);
                write_handle.release_write(locked_val);
                Ok(())
            } else {
                write_handle.release_write(locked_val);
                Err(())
            }
        }

        fn value_u64(&self) -> (v: u64) {
            let read_handle = self.locked_pos.acquire_read();
            let v = LockedPositiveTrait::value_u64(self, read_handle.borrow());
            proof { assume(v as int == self@); }
            read_handle.release_read();
            v
        }
    }
    } // verus!
}
