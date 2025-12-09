//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 2 — Verified concurrent fork-join scheduler using vstd::rwlock.
//!
//! - execute: thread-safe, verified with RwLock
//! - join: single-caller, interleaves with concurrent execute()

pub mod ConcurrentScheduler {
    use vstd::prelude::*;
    use vstd::rwlock::{RwLock, RwLockPredicate, WriteHandle, ReadHandle};
    use crate::vstdplus::threads_plus::threads_plus::JoinHandlePlus;
    use crate::Chap02::SchedulerMtEph::SchedulerMtEph::{Task, box_closure, spawn_task};

verus! {

    /// Shared queue state
    #[verifier::reject_recursive_types(T)]
    pub struct SharedQueue<T> {
        pub tasks: Vec<Task<T>>,
        pub closed: bool,
    }

    /// Predicate for the RwLock - queue is always well-formed
    pub struct QueuePred;

    impl<T> RwLockPredicate<SharedQueue<T>> for QueuePred {
        open spec fn inv(self, q: SharedQueue<T>) -> bool {
            true  // Queue is always valid - no special invariants needed
        }
    }

    /// The concurrent scheduler using verified RwLock
    #[verifier::reject_recursive_types(T)]
    pub struct ConcurrentScheduler<T> {
        pub max_threads: usize,
        pub queue: RwLock<SharedQueue<T>, QueuePred>,
    }

    impl<T> ConcurrentScheduler<T> {
        pub open spec fn wf(&self) -> bool {
            self.max_threads > 0
        }
    }

    impl<T: Send + 'static> ConcurrentScheduler<T> {
        /// Create a new concurrent scheduler
        pub fn new(max_threads: usize) -> (sched: Self)
            requires max_threads > 0,
            ensures sched.wf(),
        {
            let queue = SharedQueue {
                tasks: Vec::new(),
                closed: false,
            };
            ConcurrentScheduler {
                max_threads,
                queue: RwLock::new(queue, Ghost(QueuePred)),
            }
        }

        /// Queue a task - thread-safe via RwLock
        pub fn execute<F: FnOnce() -> T + Send + 'static>(&self, f: F)
            requires 
                self.wf(),
                f.requires(()),
        {
            let task = box_closure(f);
            
            // Acquire write lock
            let (mut q, write_handle) = self.queue.acquire_write();
            
            if !q.closed {
                q.tasks.push(task);
            }
            // If closed, task is dropped
            
            // Release write lock
            write_handle.release_write(q);
        }

        /// Check if queue is closed (for callers to know if execute will work)
        pub fn is_closed(&self) -> (closed: bool)
            requires self.wf(),
        {
            let read_handle = self.queue.acquire_read();
            let closed = read_handle.borrow().closed;
            read_handle.release_read();
            closed
        }

        /// Get current pending count
        pub fn pending_count(&self) -> (count: usize)
            requires self.wf(),
        {
            let read_handle = self.queue.acquire_read();
            let count = read_handle.borrow().tasks.len();
            read_handle.release_read();
            count
        }

        /// Close the queue and return current tasks for joining
        /// After this, execute() will drop tasks
        pub fn close_and_take(&self) -> (tasks: Vec<Task<T>>)
            requires self.wf(),
        {
            let (mut q, write_handle) = self.queue.acquire_write();
            q.closed = true;
            
            let mut tasks: Vec<Task<T>> = Vec::new();
            std::mem::swap(&mut tasks, &mut q.tasks);
            
            write_handle.release_write(q);
            tasks
        }
    }

    /// Simple join function - takes tasks and runs them with bounded parallelism
    #[verifier::external_body]
    pub fn join_tasks<T: Send + 'static>(
        tasks: Vec<Task<T>>,
        max_threads: usize,
    ) -> (results: Vec<T>)
        requires max_threads > 0,
    {
        let mut pending = tasks;
        let mut running: Vec<JoinHandlePlus<T>> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        loop {
            // Harvest finished threads
            let mut i = 0;
            while i < running.len() {
                if running[i].is_finished() {
                    let handle = running.remove(i);
                    match handle.join() {
                        Result::Ok(val) => results.push(val),
                        Result::Err(_) => { }
                    }
                } else {
                    i += 1;
                }
            }

            // Spawn more if room
            while running.len() < max_threads && pending.len() > 0 {
                let task = pending.remove(0);
                let handle = spawn_task(task);
                running.push(handle);
            }

            // Done?
            if pending.len() == 0 && running.len() == 0 {
                break;
            }

            if running.len() > 0 {
                std::thread::yield_now();
            }
        }

        results
    }

    /// Full join: close queue and process all tasks
    pub fn join<T: Send + 'static>(sched: &ConcurrentScheduler<T>) -> (results: Vec<T>)
        requires sched.wf(),
    {
        let tasks = sched.close_and_take();
        join_tasks(tasks, sched.max_threads)
    }

} // verus!
} // mod
