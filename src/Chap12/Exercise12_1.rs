//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.1: spin-lock via fetch-and-add tickets.

pub mod Exercise12_1 {
    use vstd::prelude::*;
    use std::hint::spin_loop;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

verus! {

    #[verifier::external_body]
    pub struct SpinLock {
        ticket: AtomicUsize,
        turn: AtomicUsize,
    }

    impl SpinLock {
        pub uninterp spec fn spec_locked(&self) -> bool;

        #[verifier::external_body]
        pub fn new() -> (lock: Self)
            ensures !lock.spec_locked()
        {
            SpinLock {
                ticket: AtomicUsize::new(0),
                turn: AtomicUsize::new(0),
            }
        }

        #[verifier::external_body]
        pub fn lock(&self)
            ensures self.spec_locked()
        {
            let my_ticket = self.ticket.fetch_add(1, Ordering::Relaxed);
            while self.turn.load(Ordering::Acquire) != my_ticket {
                spin_loop();
            }
        }

        #[verifier::external_body]
        pub fn unlock(&self)
            requires self.spec_locked()
            ensures !self.spec_locked()
        {
            self.turn.fetch_add(1, Ordering::Release);
        }

        #[verifier::external_body]
        pub fn with_lock<T, F: FnOnce() -> T>(&self, action: F) -> T {
            self.lock();
            let result = action();
            self.unlock();
            result
        }
    }

    #[verifier::external_body]
    pub fn parallel_increment(iterations: usize) -> (result: usize)
        ensures result == 4 * iterations
    {
        let lock = Arc::new(SpinLock::new());
        let shared = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for _ in 0..4 {
            let lock_clone = Arc::clone(&lock);
            let shared_clone = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    lock_clone.lock();
                    shared_clone.fetch_add(1, Ordering::Relaxed);
                    lock_clone.unlock();
                }
            }));
        }

        for handle in handles {
            handle.join().expect("parallel_increment: worker panicked");
        }

        shared.load(Ordering::Relaxed)
    }

} // verus!

impl Default for SpinLock {
    fn default() -> Self { SpinLock::new() }
}

impl std::fmt::Debug for SpinLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpinLock").finish()
    }
}

} // mod
