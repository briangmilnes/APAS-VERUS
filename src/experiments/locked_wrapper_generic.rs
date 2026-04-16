// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: generic LockedWrapper<T> that provides View + concurrent access.
//!
//! Goal: ONE assume pattern in the wrapper, not 215 across 80 files.
//!
//! Approach: Mt operations follow a fixed protocol:
//!   1. Clone Arc, acquire lock
//!   2. assume(ghost == inner@)  ← the ONE trust point
//!   3. Call StEph method on inner
//!   4. Update ghost from inner@
//!   5. Release lock
//!
//! The LockedWrapper provides the struct, View, and new(). Each Mt trait impl
//! follows the protocol manually (Verus can't abstract over &mut closures).
//! The key insight: the assume is always the same — ghost_view@ == inner@.
//! A macro could generate it, or a veracity tool could verify each Mt file
//! follows the protocol correctly.
//!
//! RESULT: SUCCEEDS. 2 assumes total (1 write, 1 read) for all operations.

pub mod locked_wrapper_generic {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {

    // Generic lock predicate.
    pub struct LockedInv;
    impl<T> RwLockPredicate<T> for LockedInv {
        open spec fn inv(self, v: T) -> bool { true }
    }

    // The generic wrapper.
    pub struct LockedWrapper<T: View> {
        pub lock: Arc<RwLock<T, LockedInv>>,
        pub ghost_view: Ghost<T::V>,
    }

    // View reads from the ghost snapshot.
    impl<T: View> View for LockedWrapper<T> {
        type V = T::V;
        open spec fn view(&self) -> T::V { self.ghost_view@ }
    }

    impl<T: View> LockedWrapper<T> {
        // Construct from an inner value.
        pub fn new(inner: T) -> (wrapper: Self)
            ensures wrapper@ == inner@,
        {
            let ghost v = inner@;
            LockedWrapper {
                lock: new_arc_rwlock::<T, LockedInv>(inner, Ghost(LockedInv)),
                ghost_view: Ghost(v),
            }
        }
    }

    // ================================================================
    // Inner StEph type with full specs.
    // ================================================================

    pub struct SetStEph {
        pub elements: Vec<u64>,
    }

    impl View for SetStEph {
        type V = Seq<u64>;
        open spec fn view(&self) -> Seq<u64> { self.elements@ }
    }

    pub trait SetStEphTrait: Sized + View<V = Seq<u64>> {
        fn empty() -> (s: Self)
            ensures s@.len() == 0;

        fn insert(&mut self, val: u64)
            ensures self@.len() == old(self)@.len() + 1,
                    self@.last() == val;

        fn size(&self) -> (n: usize)
            ensures n == self@.len();
    }

    impl SetStEphTrait for SetStEph {
        fn empty() -> (s: Self) { SetStEph { elements: Vec::new() } }

        fn insert(&mut self, val: u64) {
            self.elements.push(val);
        }

        fn size(&self) -> (n: usize) { self.elements.len() }
    }

    // ================================================================
    // Mt type = LockedWrapper<StEph>. Mt trait with full specs.
    // Assumes are ONLY ghost_view@ == inner@ — always the same pattern.
    // ================================================================

    pub type SetMtEph = LockedWrapper<SetStEph>;

    pub trait SetMtEphTrait: Sized + View<V = Seq<u64>> {
        fn new_empty() -> (s: Self)
            ensures s@.len() == 0;

        fn mt_insert(&mut self, val: u64)
            ensures self@.len() == old(self)@.len() + 1,
                    self@.last() == val;

        fn mt_size(&self) -> (n: usize)
            ensures n == self@.len();
    }

    impl SetMtEphTrait for SetMtEph {
        fn new_empty() -> (s: Self) {
            LockedWrapper::new(SetStEph::empty())
        }

        fn mt_insert(&mut self, val: u64) {
            let arc = clone_arc_rwlock(&self.lock);
            let (mut inner, write_handle) = arc.acquire_write();

            // THE assume — same in every write operation.
            proof { assume(self.ghost_view@ == inner@); }

            // Delegate to StEph. Full specs, no per-method assumes.
            inner.insert(val);

            // Update ghost and release.
            let ghost new_view = inner@;
            write_handle.release_write(inner);
            self.ghost_view = Ghost(new_view);
        }

        fn mt_size(&self) -> (n: usize) {
            let arc = clone_arc_rwlock(&self.lock);
            let read_handle = arc.acquire_read();
            let inner = read_handle.borrow();

            // THE assume — same in every read operation.
            proof { assume(self.ghost_view@ == inner@); }

            // Delegate to StEph.
            let n = inner.size();

            read_handle.release_read();
            n
        }
    }

    } // verus!

    #[test]
    fn test_locked_wrapper() {
        let mut s = SetMtEph::new_empty();
        assert_eq!(s.mt_size(), 0);
        s.mt_insert(42);
        assert_eq!(s.mt_size(), 1);
        s.mt_insert(99);
        assert_eq!(s.mt_size(), 2);
    }
}
