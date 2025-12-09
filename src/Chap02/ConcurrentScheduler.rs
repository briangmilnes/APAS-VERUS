//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 2 — Concurrent fork-join scheduler.
//!
//! Like SchedulerMtEph but with mutex-protected pending queue for concurrent execute().
//! - execute: thread-safe, can be called from multiple threads concurrently
//! - join: still single-caller, but interleaves with concurrent execute()
//!
//! Uses std::sync::Mutex directly (inside external_body). vstd::rwlock::RwLock would
//! be overkill since these functions aren't verified anyway.

pub mod ConcurrentScheduler {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::JoinHandlePlus;
    use crate::Chap02::SchedulerMtEph::SchedulerMtEph::{Task, box_closure, spawn_task};
    use std::sync::{Arc, Mutex};

verus! {

    /// Shared state protected by mutex
    #[verifier::reject_recursive_types(T)]
    pub struct SharedQueue<T> {
        pub tasks: Vec<Task<T>>,
        pub closed: bool,  // Set when join starts - no more execute() allowed
    }

    /// The concurrent scheduler - Arc<Mutex> for shared ownership
    #[verifier::reject_recursive_types(T)]
    #[verifier::external_body]
    pub struct ConcurrentScheduler<T> {
        max_threads: usize,
        queue: Arc<Mutex<SharedQueue<T>>>,
    }

    /// Handle for joining - only one thread should call join
    #[verifier::reject_recursive_types(T)]
    #[verifier::external_body]
    pub struct JoinHandle<T> {
        max_threads: usize,
        queue: Arc<Mutex<SharedQueue<T>>>,
        running: Vec<JoinHandlePlus<T>>,
        results: Vec<T>,
    }

    impl<T: Send + 'static> ConcurrentScheduler<T> {
        /// Create a new concurrent scheduler
        #[verifier::external_body]
        pub fn new(max_threads: usize) -> (sched: Self)
            requires max_threads > 0,
        {
            let queue = SharedQueue {
                tasks: Vec::new(),
                closed: false,
            };
            ConcurrentScheduler {
                max_threads,
                queue: Arc::new(Mutex::new(queue)),
            }
        }

        /// Queue a task - thread-safe, can be called concurrently
        #[verifier::external_body]
        pub fn execute<F: FnOnce() -> T + Send + 'static>(&self, f: F)
            requires f.requires(()),
        {
            let task = box_closure(f);
            let mut guard = self.queue.lock().unwrap();
            if !guard.closed {
                guard.tasks.push(task);
            }
            // If closed, task is dropped (execute after join started)
        }

        /// Start joining - returns handle, closes queue to new tasks
        #[verifier::external_body]
        pub fn start_join(self) -> (handle: JoinHandle<T>) {
            {
                let mut guard = self.queue.lock().unwrap();
                guard.closed = true;
            }
            JoinHandle {
                max_threads: self.max_threads,
                queue: self.queue,
                running: Vec::new(),
                results: Vec::new(),
            }
        }
    }

    impl<T: Send + 'static> JoinHandle<T> {
        /// Complete the join - blocks until all tasks finish
        #[verifier::external_body]
        pub fn finish(self) -> (results: Vec<T>) {
            let mut this = self;
            loop {
                // Try to harvest finished threads first
                let mut i = 0;
                while i < this.running.len() {
                    if this.running[i].is_finished() {
                        let handle = this.running.remove(i);
                        match handle.join() {
                            Result::Ok(val) => this.results.push(val),
                            Result::Err(_) => { /* thread panicked */ }
                        }
                        // Don't increment i, next element shifted down
                    } else {
                        i += 1;
                    }
                }

                // Spawn more if we have room
                while this.running.len() < this.max_threads {
                    let task_opt = {
                        let mut guard = this.queue.lock().unwrap();
                        if guard.tasks.len() > 0 {
                            Some(guard.tasks.remove(0))
                        } else {
                            None
                        }
                    };
                    
                    match task_opt {
                        Some(task) => {
                            let handle = spawn_task(task);
                            this.running.push(handle);
                        }
                        None => break,  // No more pending tasks
                    }
                }

                // Check if we're done
                let pending_count = {
                    let guard = this.queue.lock().unwrap();
                    guard.tasks.len()
                };
                
                if pending_count == 0 && this.running.len() == 0 {
                    break;  // All done!
                }

                // If we have running threads but nothing finished, yield
                if this.running.len() > 0 {
                    std::thread::yield_now();
                }
            }

            this.results
        }
    }

} // verus!
} // mod
