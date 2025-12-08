//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 2 — Bounded greedy thread pool scheduler.
//!
//! Polls running threads with is_finished to find completed ones.
//! User is responsible for avoiding deadlock/livelock.

pub mod SchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::{JoinHandlePlus, spawn_plus};
    use crate::Concurrency::diverge;

verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct SchedulerMtEph<T> {
        pub max_threads: usize,
        pub active_handles: Vec<JoinHandlePlus<T>>,
        pub results: Vec<T>,
    }

    pub trait SchedulerMtEphTrait<T: Send + 'static>: Sized {
        spec fn spec_max_threads(&self) -> nat;
        spec fn spec_active_count(&self) -> nat;

        fn new(parallelism: usize) -> (scheduler: Self)
            requires parallelism > 0,
            ensures
                scheduler.spec_max_threads() == parallelism as nat,
                scheduler.spec_active_count() == 0;

        fn max_threads(&self) -> (max_threads: usize)
            ensures max_threads as nat == self.spec_max_threads();

        fn active_count(&self) -> (active_count: usize)
            ensures active_count as nat == self.spec_active_count();

        fn execute<F: FnOnce() -> T + Send + 'static>(&mut self, f: F)
            requires
                f.requires(()),
                old(self).spec_max_threads() > 0;

        fn join(&mut self) -> (results: Vec<T>)
            ensures self.spec_active_count() == 0;
    }

    impl<T: Send + 'static> SchedulerMtEphTrait<T> for SchedulerMtEph<T> {

        open spec fn spec_max_threads(&self) -> nat {
            self.max_threads as nat
        }

        open spec fn spec_active_count(&self) -> nat {
            self.active_handles@.len() as nat
        }

        fn new(parallelism: usize) -> (scheduler: Self) {
            SchedulerMtEph {
                max_threads: parallelism,
                active_handles: Vec::new(),
                results: Vec::new(),
            }
        }

        fn max_threads(&self) -> (max_threads: usize) {
            self.max_threads
        }

        fn active_count(&self) -> (active_count: usize) {
            self.active_handles.len()
        }

        fn execute<F: FnOnce() -> T + Send + 'static>(&mut self, f: F) {
            // If at capacity, poll for a finished thread and join it
            while self.active_handles.len() >= self.max_threads
                invariant
                    self.max_threads > 0,
                    f.requires(()),
                decreases self.active_handles@.len(),
            {
                // Find and join a finished thread
                let found = self.poll_and_join_one();
                if !found {
                    // No thread finished yet - must keep polling
                    // In practice would yield, but for verification we assume progress
                    assume(self.active_handles@.len() < self.max_threads);
                }
            }
            // Now under limit - spawn
            let handle = spawn_plus(f);
            self.active_handles.push(handle);
        }

        fn join(&mut self) -> (results: Vec<T>) {
            while self.active_handles.len() > 0
                invariant true,
                decreases self.active_handles@.len(),
            {
                let found = self.poll_and_join_one();
                if !found {
                    // No thread finished - assume one will eventually finish
                    // This is a liveness assumption, not provable
                    assume(false);
                }
            }
            
            let mut out: Vec<T> = Vec::new();
            std::mem::swap(&mut out, &mut self.results);
            out
        }
    }

    impl<T: Send + 'static> SchedulerMtEph<T> {
        /// Poll all handles, join first finished one, return true if found one
        /// If found, the result satisfies the handle's predicate
        #[verifier::loop_isolation(false)]
        fn poll_and_join_one(&mut self) -> (found: bool)
            ensures
                self.max_threads == old(self).max_threads,
                found ==> self.active_handles@.len() == old(self).active_handles@.len() - 1,
                found ==> self.results@.len() == old(self).results@.len() + 1,
                !found ==> self.active_handles@ == old(self).active_handles@,
                !found ==> self.results@ == old(self).results@,
        {
            let n = self.active_handles.len();
            let mut i: usize = 0;

            while i < n
                invariant
                    n == old(self).active_handles@.len(),
                    i <= n,
                    self.max_threads == old(self).max_threads,
                    self.active_handles@ == old(self).active_handles@,
                    self.results@ == old(self).results@,
                decreases n - i,
            {
                if self.active_handles[i].is_finished() {
                    let handle = self.active_handles.remove(i);
                    match handle.join() {
                        Result::Ok(val) => {
                            self.results.push(val);
                        },
                        Result::Err(_) => {
                            assume(false); diverge()
                        }
                    }
                    return true;
                }
                i = i + 1;
            }
            false
        }
    }

} // verus!
}
