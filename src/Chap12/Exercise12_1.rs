//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.1: spin-lock via fetch-and-add tickets.
//!
//! The external_body annotations here are permanent. AtomicUsize operations
//! (fetch_add, load) have no vstd specs, and spin-lock correctness depends on
//! hardware memory ordering guarantees that are outside Verus's model. Adding
//! meaningful requires/ensures would require a concurrency logic (e.g., TSM)
//! that is disproportionate for a ticket-lock exercise.

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

    pub trait SpinLockTrait: Sized {
        /// Spec: is the lock currently held?
        spec fn spec_locked(&self) -> bool;

        /// Create a new unlocked spin lock.
        /// - APAS: no cost spec.
        /// - Claude-Opus-4.6: O(1).
        fn new() -> (lock: Self)
            ensures !lock.spec_locked();

        /// Acquire the lock (spins until acquired).
        /// - APAS: no cost spec.
        /// - Claude-Opus-4.6: amortized O(1), worst-case unbounded (spin). Ticket lock guarantees FIFO fairness.
        fn lock(&self)
            ensures self.spec_locked();

        /// Release the lock.
        /// - APAS: no cost spec.
        /// - Claude-Opus-4.6: O(1) — single fetch_add.
        fn unlock(&self)
            requires self.spec_locked()
            ensures !self.spec_locked();

        /// Execute action while holding the lock.
        /// - APAS: no cost spec.
        /// - Claude-Opus-4.6: O(1) + cost of action.
        /// Note: requires/ensures omitted because Verus cannot express "result
        /// equals action()" for a generic FnOnce — the closure's spec is opaque.
        fn with_lock<T, F: FnOnce() -> T>(&self, action: F) -> T;
    }

    impl SpinLockTrait for SpinLock {
        uninterp spec fn spec_locked(&self) -> bool;

        #[verifier::external_body]
        fn new() -> (lock: Self) {
            SpinLock {
                ticket: AtomicUsize::new(0),
                turn: AtomicUsize::new(0),
            }
        }

        #[verifier::external_body]
        fn lock(&self) {
            let my_ticket = self.ticket.fetch_add(1, Ordering::Relaxed);
            while self.turn.load(Ordering::Acquire) != my_ticket {
                spin_loop();
            }
        }

        #[verifier::external_body]
        fn unlock(&self) {
            self.turn.fetch_add(1, Ordering::Release);
        }

        #[verifier::external_body]
        fn with_lock<T, F: FnOnce() -> T>(&self, action: F) -> T {
            self.lock();
            let result = action();
            self.unlock();
            result
        }
    }

    /// Run 4 threads, each incrementing a shared counter `iterations` times.
    /// - APAS: no cost spec.
    /// - Claude-Opus-4.6: Work Θ(iterations), Span Θ(iterations) — 4-way parallel, bounded by lock contention.
    #[verifier::external_body]
    pub fn parallel_increment(iterations: usize) -> (incremented: usize)
        ensures incremented == 4 * iterations
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

impl Default for SpinLock {
    fn default() -> Self { SpinLock::new() }
}

} // verus!

impl std::fmt::Debug for SpinLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpinLock").finish()
    }
}

} // mod
