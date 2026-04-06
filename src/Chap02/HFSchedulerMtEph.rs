//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Help-first scheduler with bounded parallelism using a global pool.
//!
//! - Uses a help-first strategy: if no capacity available, runs sequentially.
//! - Prevents deadlock from nested joins by not blocking when capacity exhausted.
//! - Call `set_parallelism()` before first use to configure thread limit for a single
//! parallel pool.
//! Reviewed and is clean. briangmilnes@gmail.com 13 March 2026 

pub mod HFSchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::*;
    use crate::Concurrency::*;
    use std::sync::{Mutex, Condvar, LazyLock, RwLock};

    /// - We track the number of available tasks and have a condition variable to signal when task finishes.
    /// - Outside verus! because Condvar/Mutex and LazyLock closure are not Verus-friendly.
    struct PoolState { // accept hole
        available_tasks: Mutex<usize>,
        task_freed: Condvar,
    }

    /// - State of a spawned task: either running in a thread or already completed (help-first).
    pub enum TaskState<T> { // accept hole
        Spawned   { handle: JoinHandlePlus<T> },
        Completed { result: Option<T> },
    }

    /// - The configured parallelism level. None means use the number of CPUs minus one, minimum one.
    static PARALLELISM: RwLock<Option<usize>> = RwLock::new(None);

    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — reads config, creates mutex/condvar.
    fn init_pool() -> PoolState {
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
    }

    static POOL: LazyLock<PoolState> = LazyLock::new(init_pool);

    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — lock, check, unlock.
    fn try_acquire() -> bool {
        let mut available = POOL.available_tasks.lock().unwrap();
        if *available > 0 {
            *available -= 1;
            true
        } else {
            false
        }
    }

    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — waits on condvar.
    fn acquire() {
        let mut available = POOL.available_tasks.lock().unwrap();
        while *available == 0 {
            available = POOL.task_freed.wait(available).unwrap();
        }
        *available -= 1;
    }

    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — lock, increment, notify, unlock.
    fn release() {
        let mut available = POOL.available_tasks.lock().unwrap();
        *available += 1;
        POOL.task_freed.notify_one();
    }

    verus! {

    #[verifier::external_type_specification] // accept hole
    #[verifier::external_body] // accept hole
    #[verifier::reject_recursive_types(T)]
    pub struct ExTaskState<T>(TaskState<T>);

    impl<T> TaskState<T> {
        pub uninterp spec fn predicate(&self, ret: T) -> bool;
    }

    /// Set parallelism level. Must be called before any parallel operations.
    /// - Alg Analysis: APAS: N/A (scheduler config)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    #[verifier::external_body] // accept hole
    pub fn set_parallelism(n: usize) {
        *PARALLELISM.write().unwrap() = Some(n);
    }

    /// - Help-first fork-join: spawns fb in a new thread only if capacity available.
    /// - If no capacity, runs both closures sequentially (help-first strategy).
    /// - Prevents deadlock from nested joins.
    /// - Alg Analysis: APAS: N/A (scheduler primitive; cost = closure cost)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(W_fa + W_fb), Span O(max(S_fa, S_fb)) when parallel; else O(W_fa + W_fb)
    #[verifier::external_body] // accept hole
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
    /// - Alg Analysis: APAS: N/A (scheduler primitive; cost = closure cost)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(W_fa + W_fb), Span O(max(S_fa, S_fb))
    #[verifier::external_body] // accept hole
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
                proof { assume(false); } // accept hole: thread join error arm unreachable
                diverge()
            }
        };
        (a, b)
    }

    /// - Help-first spawn: spawns in new thread if capacity available.
    /// - If no capacity, runs locally (help-first) and returns completed state.
    /// - Never blocks, never deadlocks.
    /// - Alg Analysis: APAS: N/A (scheduler primitive; cost = closure cost)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(W_f), Span O(S_f)
    #[verifier::external_body] // accept hole
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
    /// - Alg Analysis: APAS: N/A (scheduler primitive)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(S_task) — blocks until task completes
    #[verifier::external_body] // accept hole
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

    //		14. derive impls outside verus!

    impl std::fmt::Debug for PoolState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PoolState").finish()
        }
    }

    impl std::fmt::Display for PoolState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PoolState")
        }
    }

    impl<T> std::fmt::Debug for ExTaskState<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ExTaskState")
        }
    }

    impl<T> std::fmt::Display for ExTaskState<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ExTaskState")
        }
    }
}
