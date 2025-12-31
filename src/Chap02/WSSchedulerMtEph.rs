//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Work-stealing scheduler with bounded parallelism.

pub mod WSSchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::{spawn_plus, JoinHandlePlus};
    use crate::Concurrency::diverge;
    use std::sync::{Arc, Mutex, Condvar};

verus! {

    #[verifier::external_body]
    pub struct Pool {
        budget: Arc<(Mutex<usize>, Condvar)>,
    }

    pub trait PoolTrait: Sized {
        spec fn spec_size(&self) -> nat;

        fn new(size: usize) -> (pool: Self)
            requires size > 0
            ensures pool.spec_size() == size as nat;

        fn try_acquire(&self) -> (acquired: bool);

        fn acquire(&self);

        fn release(&self);

        /// - Help-first fork-join: spawns fb in a new thread only if budget available.
        /// - If no budget, runs both closures sequentially (help-first strategy).
        /// - Prevents deadlock from nested joins holding slots while waiting.
        fn join<A, B, FA, FB>(&self, fa: FA, fb: FB) -> (result: (A, B))
        where
            FA: FnOnce() -> A + Send + 'static,
            FB: FnOnce() -> B + Send + 'static,
            A: Send + 'static,
            B: Send + 'static,
            requires
                fa.requires(()),
                fb.requires(()),
            ensures
                fa.ensures((), result.0),
                fb.ensures((), result.1);

        /// - Unconditional fork-join: always spawns fb in a new thread.
        /// - Runs fa in the current thread, waits for fb to complete, returns both results.
        /// - Use when parallelism is always desired regardless of budget.
        fn spawn_join<A, B, FA, FB>(fa: FA, fb: FB) -> (result: (A, B))
        where
            FA: FnOnce() -> A + Send + 'static,
            FB: FnOnce() -> B + Send + 'static,
            A: Send + 'static,
            B: Send + 'static,
            requires
                fa.requires(()),
                fb.requires(()),
            ensures
                fa.ensures((), result.0),
                fb.ensures((), result.1);
    }

    impl PoolTrait for Pool {
        uninterp spec fn spec_size(&self) -> nat;

        #[verifier::external_body]
        fn new(size: usize) -> (pool: Self) {
            Pool { budget: Arc::new((Mutex::new(size), Condvar::new())) }
        }

        #[verifier::external_body]
        fn try_acquire(&self) -> (acquired: bool) {
            let (lock, _cvar) = &*self.budget;
            let mut count = lock.lock().unwrap();
            if *count > 0 {
                *count -= 1;
                true
            } else {
                false
            }
        }

        #[verifier::external_body]
        fn acquire(&self) {
            let (lock, cvar) = &*self.budget;
            let mut count = lock.lock().unwrap();
            while *count == 0 {
                count = cvar.wait(count).unwrap();
            }
            *count -= 1;
        }

        #[verifier::external_body]
        fn release(&self) {
            let (lock, cvar) = &*self.budget;
            let mut count = lock.lock().unwrap();
            *count += 1;
            cvar.notify_one();
        }

        fn join<A, B, FA, FB>(&self, fa: FA, fb: FB) -> (result: (A, B))
        where
            FA: FnOnce() -> A + Send + 'static,
            FB: FnOnce() -> B + Send + 'static,
            A: Send + 'static,
            B: Send + 'static,
        {
            if self.try_acquire() {
                // Got a slot - spawn fb, run fa, wait for fb
                let result = Self::spawn_join(fa, fb);
                self.release();
                result
            } else {
                // No slot - help-first: run both sequentially
                (fa(), fb())
            }
        }

        fn spawn_join<A, B, FA, FB>(fa: FA, fb: FB) -> (result: (A, B))
        where
            FA: FnOnce() -> A + Send + 'static,
            FB: FnOnce() -> B + Send + 'static,
            A: Send + 'static,
            B: Send + 'static,
        {
            let handle: JoinHandlePlus<B> = spawn_plus(fb);
            let a = fa();
            let b = match handle.join() {
                Ok(val) => val,
                Err(_) => {
                    assume(false);
                    diverge()
                }
            };
            (a, b)
        }
    }

    impl Clone for Pool {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self) {
            Pool { budget: Arc::clone(&self.budget) }
        }
    }

} // verus!
} 
