//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Work-stealing scheduler with bounded parallelism.
//!
//! - Uses a help-first strategy: if no thread budget available, runs sequentially.
//! - Prevents deadlock from nested joins holding slots while waiting.
//! - A global default pool is available via `default_pool()` for convenient fork-join.
//! - Supports binary join and N-way spawn/wait patterns.

pub mod WSSchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::{spawn_plus, JoinHandlePlus};
    use crate::Concurrency::diverge;
    use std::sync::{Arc, Mutex, Condvar, LazyLock, RwLock};

    /// Shared state for the pool: tracks available capacity.
    struct PoolState {
        available_tasks: Mutex<usize>,
        task_freed: Condvar,
    }

    pub struct Pool {
        state: Arc<PoolState>,
    }
    
    impl Clone for Pool {
        fn clone(&self) -> Self {
            Pool { state: Arc::clone(&self.state) }
        }
    }
    
    /// State of a spawned task: either running in a thread or already completed (help-first).
    pub enum TaskState<T> {
        Spawned   { handle: JoinHandlePlus<T>, pool: Pool },
        Completed { result: Option<T> },
    }

    /// The configured parallelism level. None means use the number of CPUs minus one, minimum one.
    static PARALLELISM: RwLock<Option<usize>> = RwLock::new(None);

    static DEFAULT_POOL: LazyLock<Pool> = LazyLock::new(|| {
        let n = PARALLELISM.read().unwrap();
        let threads = n.unwrap_or_else(|| {
            let cpus = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(2);
            (cpus - 1).max(1)
        });
        Pool::new(threads)
    });

verus! {

    #[verifier::external_type_specification]
    #[verifier::external_body]
    pub struct ExPool(Pool);

    #[verifier::external_type_specification]
    #[verifier::external_body]
    #[verifier::reject_recursive_types(T)]
    pub struct ExTaskState<T>(TaskState<T>);

    impl<T> TaskState<T> {
        pub uninterp spec fn predicate(&self, ret: T) -> bool;
    }

    /// Set parallelism level. Must be called before any parallel operations.
    #[verifier::external_body]
    pub fn set_parallelism(n: usize) {
        *PARALLELISM.write().unwrap() = Some(n);
    }

    /// Get the default pool. Initializes on first call.
    #[verifier::external_body]
    pub fn default_pool() -> (pool: &'static Pool)
        ensures pool.spec_size() > 0
    {
        &*DEFAULT_POOL
    }

    pub trait PoolTrait: Sized {
        spec fn spec_size(&self) -> nat;

        fn new(size: usize) -> (pool: Self)
            requires size > 0
            ensures pool.spec_size() == size as nat;

        fn try_acquire(&self) -> (acquired: bool);

        fn acquire(&self);

        fn release(&self);

        /// - Help-first fork-join: spawns fb in a new thread only if capacity available.
        /// - If no capacity, runs both closures sequentially (help-first strategy).
        /// - Prevents deadlock from nested joins holding capacity while waiting.
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
        /// - Use when parallelism is always desired regardless of capacity.
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

        /// - Help-first spawn: spawns in new thread if capacity available.
        /// - If no capacity, runs locally (help-first) and returns completed state.
        /// - Never blocks, never deadlocks.
        fn spawn<T, F>(&self, f: F) -> (task: TaskState<T>)
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
            requires
                f.requires(()),
            ensures
                forall|ret: T| #[trigger] task.predicate(ret) ==> f.ensures((), ret);

        /// Wait for a spawned task to complete. Releases capacity.
        fn wait<T: Send + 'static>(task: TaskState<T>) -> (result: T)
            ensures
                task.predicate(result);
    }

    impl PoolTrait for Pool {
        uninterp spec fn spec_size(&self) -> nat;

        #[verifier::external_body]
        fn new(size: usize) -> (pool: Self) {
            Pool {
                state: Arc::new(PoolState {
                    available_tasks: Mutex::new(size),
                    task_freed: Condvar::new(),
                })
            }
        }

        #[verifier::external_body]
        fn try_acquire(&self) -> (acquired: bool) {
            let mut available = self.state.available_tasks.lock().unwrap();
            if *available > 0 {
                *available -= 1;
                true
            } else {
                false
            }
        }

        #[verifier::external_body]
        fn acquire(&self) {
            let mut available = self.state.available_tasks.lock().unwrap();
            while *available == 0 {
                available = self.state.task_freed.wait(available).unwrap();
            }
            *available -= 1;
        }

        #[verifier::external_body]
        fn release(&self) {
            let mut available = self.state.available_tasks.lock().unwrap();
            *available += 1;
            self.state.task_freed.notify_one();
        }

        fn join<A, B, FA, FB>(&self, fa: FA, fb: FB) -> (result: (A, B))
        where
            FA: FnOnce() -> A + Send + 'static,
            FB: FnOnce() -> B + Send + 'static,
            A: Send + 'static,
            B: Send + 'static,
        {
            if self.try_acquire() {
                let result = Self::spawn_join(fa, fb);
                self.release();
                result
            } else {
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
        fn spawn<T, F>(&self, f: F) -> (task: TaskState<T>)
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            if self.try_acquire() {
                let thread_handle: JoinHandlePlus<T> = spawn_plus(f);
                TaskState::Spawned { handle: thread_handle, pool: self.clone() }
            } else {
                let result = f();
                TaskState::Completed { result: Some(result) }
            }
        }

        #[verifier::external_body]
        fn wait<T: Send + 'static>(task: TaskState<T>) -> (result: T)
        {
            match task {
                TaskState::Spawned { handle: h, pool } => {
                    let result = match h.join() {
                        Ok(val) => val,
                        Err(_) => panic!("Thread panicked"),
                    };
                    pool.release();
                    result
                }
                TaskState::Completed { result } => {
                    result.expect("TaskState already consumed")
                }
            }
        }
    }

} // verus!
} 
