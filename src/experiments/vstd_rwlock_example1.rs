//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: vstd RwLock example 1 from vstd/rwlock.rs doc comments.
//! Uses spec_fn predicate: v == 5 || v == 13.

pub mod vstd_rwlock_example1 {

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    fn example1() {
        // We can create a lock with an invariant: `v == 5 || v == 13`.
        // Thus only 5 or 13 can be stored in the lock.
        let lock = RwLock::<u64, spec_fn(u64) -> bool>::new(5, Ghost(|v| v == 5 || v == 13));

        let (val, write_handle) = lock.acquire_write();
        assert(val == 5 || val == 13);
        write_handle.release_write(13);

        let read_handle1 = lock.acquire_read();
        let read_handle2 = lock.acquire_read();

        // We can take multiple read handles at the same time:

        let val1 = read_handle1.borrow();
        let val2 = read_handle2.borrow();

        // RwLock has a lemma that both read handles have the same value:

        proof { ReadHandle::lemma_readers_match(&read_handle1, &read_handle2); }
        assert(*val1 == *val2);

        read_handle1.release_read();
        read_handle2.release_read();
    }

    } // verus!
}
