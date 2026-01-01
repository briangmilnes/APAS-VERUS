//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Work-stealing scheduler with bounded parallelism.
//! Supports binary join and N-way spawn/wait patterns.

pub mod WSSchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::{spawn_plus, JoinHandlePlus};
    use crate::Concurrency::diverge;
    use std::sync::{Arc, Mutex, Condvar};

    // Implementation detail - outside verus! block
    enum PoolHandleInner<T> {
        Spawned { handle: JoinHandlePlus<T>, pool: Pool },
        Completed { result: Option<T> },
    }

    // Pool struct definition outside verus for PoolHandleInner to reference
    pub struct Pool {
        budget: Arc<(Mutex<usize>, Condvar)>,
    }
    
    impl Clone for Pool {
        fn clone(&self) -> Self {
            Pool { budget: Arc::clone(&self.budget) }
        }
    }
    
    /// Handle to a spawned task. Either holds a thread handle (spawned) or a result (help-first).
    pub struct PoolHandle<T> {
        inner: PoolHandleInner<T>,
    }

verus! {

    // Verus sees Pool as external_body
    #[verifier::external_type_specification]
    #[verifier::external_body]
    pub struct ExPool(Pool);

    // Verus sees PoolHandle as external_body
    #[verifier::external_type_specification]
    #[verifier::external_body]
    #[verifier::reject_recursive_types(T)]
    pub struct ExPoolHandle<T>(PoolHandle<T>);

    impl<T> PoolHandle<T> {
        pub uninterp spec fn predicate(&self, ret: T) -> bool;
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

        /// - Help-first spawn: spawns in new thread if budget available.
        /// - If no budget, runs locally (help-first) and returns completed handle.
        /// - Never blocks, never deadlocks.
        fn spawn<T, F>(&self, f: F) -> (handle: PoolHandle<T>)
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
            requires
                f.requires(()),
            ensures
                forall|ret: T| #[trigger] handle.predicate(ret) ==> f.ensures((), ret);

        /// - Wait for a spawned task to complete. Releases the budget slot.
        fn wait<T: Send + 'static>(handle: PoolHandle<T>) -> (result: T)
            ensures
                handle.predicate(result);
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

        #[verifier::external_body]
        fn spawn<T, F>(&self, f: F) -> (handle: PoolHandle<T>)
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            if self.try_acquire() {
                // Got a slot - spawn in new thread
                let inner_handle: JoinHandlePlus<T> = spawn_plus(f);
                PoolHandle { inner: PoolHandleInner::Spawned { handle: inner_handle, pool: self.clone() } }
            } else {
                // No slot - help-first: run locally, return completed handle
                let result = f();
                PoolHandle { inner: PoolHandleInner::Completed { result: Some(result) } }
            }
        }

        #[verifier::external_body]
        fn wait<T: Send + 'static>(handle: PoolHandle<T>) -> (result: T)
        {
            match handle.inner {
                PoolHandleInner::Spawned { handle: h, pool } => {
                    let result = match h.join() {
                        Ok(val) => val,
                        Err(_) => panic!("Thread panicked"),
                    };
                    pool.release();  // Release budget slot
                    result
                }
                PoolHandleInner::Completed { result } => {
                    result.expect("PoolHandle already consumed")
                }
            }
        }
    }

} // verus!
} 
