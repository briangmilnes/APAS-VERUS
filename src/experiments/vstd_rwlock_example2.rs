// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: vstd RwLock example 2 from vstd/rwlock.rs doc comments.
//! Uses custom FixedParity struct as RwLockPredicate.

pub mod vstd_rwlock_example2 {

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    pub struct FixedParity {
        pub parity: int,
    }

    impl RwLockPredicate<u64> for FixedParity {
        open spec fn inv(self, v: u64) -> bool {
            v % 2 == self.parity
        }
    }

    fn example2() {
        // Create a lock that can only store even integers.
        let lock_even = RwLock::<u64, FixedParity>::new(20, Ghost(FixedParity { parity: 0 }));

        // Create a lock that can only store odd integers.
        let lock_odd = RwLock::<u64, FixedParity>::new(23, Ghost(FixedParity { parity: 1 }));

        let read_handle_even = lock_even.acquire_read();
        let val_even = *read_handle_even.borrow();
        assert(val_even % 2 == 0);

        let read_handle_odd = lock_odd.acquire_read();
        let val_odd = *read_handle_odd.borrow();
        assert(val_odd % 2 == 1);

        read_handle_even.release_read();
        read_handle_odd.release_read();
    }

    } // verus!
}
