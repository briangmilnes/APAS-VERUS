//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Work-stealing scheduler with bounded parallelism using a global pool.
//!
//! - Uses a help-first strategy: if no capacity available, runs sequentially.
//! - Prevents deadlock from nested joins by not blocking when capacity exhausted.
//! - Call `set_parallelism()` before first use to configure thread limit for a single
//! parallel pool.

pub mod WSSchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::*;
    use crate::Concurrency::*;
    use std::sync::{Mutex, Condvar, LazyLock, RwLock};

    /// - We track the number of available tasks and have a condition variable to signal when task finishes.
    /// - This is outside of the verus! macro, and thus outside of proof, as we need a Condvar to signal
    /// that at task has finished, and it Rust it must be protected by a Mutex.
    struct PoolState {
        available_tasks: Mutex<usize>,
        task_freed: Condvar,
    }

    /// - State of a spawned task: either running in a thread or already completed (help-first).
    pub enum TaskState<T> {
        Spawned   { handle: JoinHandlePlus<T> },
        Completed { result: Option<T> },
    }

    /// - The configured parallelism level. None means use the number of CPUs minus one, minimum one.
    static PARALLELISM: RwLock<Option<usize>> = RwLock::new(None);

    static POOL: LazyLock<PoolState> = LazyLock::new(|| {
        let n = PARALLELISM.read().unwrap();
        let threads = n.unwrap_or_else(|| {
            let cpus = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(2);
            (cpus - 1).max(1)
        });
        PoolState {
            available_tasks: Mutex::new(threads),
            task_freed: Condvar::new(),
        }
    });

    fn try_acquire() -> bool {
        let mut available = POOL.available_tasks.lock().unwrap();
        if *available > 0 {
            *available -= 1;
            true
        } else {
            false
        }
    }

    fn acquire() {
        let mut available = POOL.available_tasks.lock().unwrap();
        while *available == 0 {
            available = POOL.task_freed.wait(available).unwrap();
        }
        *available -= 1;
    }

    fn release() {
        let mut available = POOL.available_tasks.lock().unwrap();
        *available += 1;
        POOL.task_freed.notify_one();
    }

verus! {

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

    /// - Help-first fork-join: spawns fb in a new thread only if capacity available.
    /// - If no capacity, runs both closures sequentially (help-first strategy).
    /// - Prevents deadlock from nested joins.
    #[verifier::external_body]
    pub fn join<A, B, FA, FB>(fa: FA, fb: FB) -> (joined_pair: (A, B))
    where
        FA: FnOnce() -> A + Send + 'static,
        FB: FnOnce() -> B + Send + 'static,
        A: Send + 'static,
        B: Send + 'static,
        requires
            fa.requires(()),
            fb.requires(()),
        ensures
            fa.ensures((), joined_pair.0),
            fb.ensures((), joined_pair.1),
    {
        if try_acquire() {
            let joined_pair = spawn_join(fa, fb);
            release();
            joined_pair
        } else {
            (fa(), fb())
        }
    }

    /// - Unconditional fork-join: always spawns fb in a new thread.
    /// - Runs fa in the current thread, waits for fb to complete, returns both results.
    #[verifier::external_body]
    pub fn spawn_join<A, B, FA, FB>(fa: FA, fb: FB) -> (joined_pair: (A, B))
    where
        FA: FnOnce() -> A + Send + 'static,
        FB: FnOnce() -> B + Send + 'static,
        A: Send + 'static,
        B: Send + 'static,
        requires
            fa.requires(()),
            fb.requires(()),
        ensures
            fa.ensures((), joined_pair.0),
            fb.ensures((), joined_pair.1),
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

    /// - Help-first spawn: spawns in new thread if capacity available.
    /// - If no capacity, runs locally (help-first) and returns completed state.
    /// - Never blocks, never deadlocks.
    #[verifier::external_body]
    pub fn spawn<T, F>(f: F) -> (task: TaskState<T>)
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
        requires
            f.requires(()),
        ensures
            forall|ret: T| #[trigger] task.predicate(ret) ==> f.ensures((), ret),
    {
        if try_acquire() {
            let thread_handle: JoinHandlePlus<T> = spawn_plus(f);
            TaskState::Spawned { handle: thread_handle }
        } else {
            let joined_pair = f();
            TaskState::Completed { result: Some(joined_pair) }
        }
    }

    /// Wait for a spawned task to complete. Releases capacity.
    #[verifier::external_body]
    pub fn wait<T: Send + 'static>(task: TaskState<T>) -> (task_result: T)
        ensures
            task.predicate(task_result),
    {
        match task {
            TaskState::Spawned { handle: h } => {
                let task_result = match h.join() {
                    Ok(val) => val,
                    Err(_) => panic!("Thread panicked"),
                };
                release();
                task_result
            }
            TaskState::Completed { result } => {
                result.expect("TaskState already consumed")
            }
        }
    }

} // verus!
} 
