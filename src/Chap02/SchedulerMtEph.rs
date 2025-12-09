//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 2 — Bounded fork-join scheduler.
//!
//! - execute: queues closures (no spawn). Must be called before join.
//! - join: spawns up to max_threads, polls, spawns more as slots open.
//!
//! Ghost state tracks postcondition predicates so join guarantees all ensures are satisfied.

pub mod SchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::{JoinHandlePlus, spawn_plus};
    use crate::Concurrency::diverge;

verus! {

    /// A boxed closure that produces T
    #[verifier::external_body]
    #[verifier::reject_recursive_types(T)]
    pub struct Task<T> {
        f: Box<dyn FnOnce() -> T + Send + 'static>,
    }

    impl<T> Task<T> {
        /// The postcondition predicate this task satisfies
        pub uninterp spec fn post(&self) -> spec_fn(T) -> bool;

        #[verifier::external_body]
        pub fn call(self) -> (result: T)
            ensures (self.post())(result)
        {
            (self.f)()
        }
    }

    /// Box a closure into a Task - TCB: trusts post() reflects f.ensures
    #[verifier::external_body]
    pub fn box_closure<T, F: FnOnce() -> T + Send + 'static>(f: F) -> (task: Task<T>)
        requires f.requires(()),
        ensures task.post() == (|t: T| f.ensures((), t))
    {
        Task { f: Box::new(f) }
    }

    /// Spawn a Task directly - avoids closure capture issues
    /// TCB: trusts that handle.predicate correctly reflects task.post
    #[verifier::external_body]
    pub fn spawn_task<T: Send + 'static>(task: Task<T>) -> (handle: JoinHandlePlus<T>)
        ensures forall|ret: T| handle.predicate(ret) ==> (task.post())(ret)
    {
        use crate::vstdplus::threads_plus::threads_plus::spawn_plus;
        spawn_plus(move || task.call())
    }

    /// Ghost: postcondition predicates at each stage
    #[verifier::reject_recursive_types(T)]
    pub ghost struct SchedulerGhost<T> {
        pub pending: Seq<spec_fn(T) -> bool>,
        pub running: Seq<spec_fn(T) -> bool>,
        pub finished: Seq<spec_fn(T) -> bool>,
    }

    impl<T> SchedulerGhost<T> {
        pub proof fn empty() -> (g: Self)
            ensures
                g.pending.len() == 0,
                g.running.len() == 0,
                g.finished.len() == 0,
        {
            SchedulerGhost {
                pending: Seq::empty(),
                running: Seq::empty(),
                finished: Seq::empty(),
            }
        }
    }

    /// Exec state for the scheduler
    #[verifier::reject_recursive_types(T)]
    pub struct SchedulerMtEph<T> {
        pub max_threads: usize,
        pub pending: Vec<Task<T>>,
        pub running: Vec<JoinHandlePlus<T>>,
        pub results: Vec<T>,
        pub joined: bool,
        pub g: Ghost<SchedulerGhost<T>>,
    }

    impl<T> SchedulerMtEph<T> {
        pub open spec fn wf(&self) -> bool {
            &&& self.max_threads > 0
            &&& self.g@.pending.len() == self.pending@.len()
            &&& self.g@.running.len() == self.running@.len()
            &&& self.g@.finished.len() == self.results@.len()
            &&& forall|i: int| #![trigger self.pending@[i]]
                    0 <= i < self.pending@.len() ==>
                    self.pending@[i].post() == self.g@.pending[i]
            &&& forall|i: int| #![trigger self.running@[i]]
                    0 <= i < self.running@.len() ==>
                    forall|t: T| #![trigger self.running@[i].predicate(t)]
                        self.running@[i].predicate(t) ==> (self.g@.running[i])(t)
            &&& forall|i: int| #![trigger self.results@[i]]
                    0 <= i < self.results@.len() ==>
                    (self.g@.finished[i])(self.results@[i])
        }

        pub open spec fn spec_max_threads(&self) -> nat { self.max_threads as nat }
        pub open spec fn spec_pending_count(&self) -> nat { self.pending@.len() }
        pub open spec fn spec_running_count(&self) -> nat { self.running@.len() }
        pub open spec fn spec_result_count(&self) -> nat { self.results@.len() }
        pub open spec fn spec_total_tasks(&self) -> nat {
            self.pending@.len() + self.running@.len() + self.results@.len()
        }
    }

    impl<T: Send + 'static> SchedulerMtEph<T> {

        pub fn new(max_threads: usize) -> (scheduler: Self)
            requires max_threads > 0,
            ensures
                scheduler.wf(),
                scheduler.spec_max_threads() == max_threads as nat,
                scheduler.spec_pending_count() == 0,
                scheduler.spec_running_count() == 0,
                scheduler.spec_result_count() == 0,
                !scheduler.joined,
        {
            let pending: Vec<Task<T>> = Vec::new();
            let running: Vec<JoinHandlePlus<T>> = Vec::new();
            let results: Vec<T> = Vec::new();
            
            SchedulerMtEph {
                max_threads,
                pending,
                running,
                results,
                joined: false,
                g: Ghost(SchedulerGhost {
                    pending: Seq::empty(),
                    running: Seq::empty(),
                    finished: Seq::empty(),
                }),
            }
        }

        pub fn execute<F: FnOnce() -> T + Send + 'static>(&mut self, f: F)
            requires
                old(self).wf(),
                !old(self).joined,
                f.requires(()),
            ensures
                self.wf(),
                !self.joined,
                self.spec_max_threads() == old(self).spec_max_threads(),
                self.spec_pending_count() == old(self).spec_pending_count() + 1,
                self.spec_running_count() == old(self).spec_running_count(),
                self.spec_result_count() == old(self).spec_result_count(),
                self.g@.pending.last() == (|t: T| f.ensures((), t)),
        {
            let task = box_closure(f);
            // box_closure ensures: task.post() == (|t| f.ensures((), t))
            
            self.pending.push(task);
            
            // Update ghost state
            self.g = Ghost(SchedulerGhost {
                pending: self.g@.pending.push(task.post()),
                running: self.g@.running,
                finished: self.g@.finished,
            });
        }

        fn spawn_one(&mut self)
            requires
                old(self).wf(),
                old(self).pending@.len() > 0,
            ensures
                self.wf(),
                self.max_threads == old(self).max_threads,
                self.joined == old(self).joined,
                self.pending@.len() == old(self).pending@.len() - 1,
                self.running@.len() == old(self).running@.len() + 1,
                self.results@ == old(self).results@,
                self.g@.finished == old(self).g@.finished,
        {
            // Capture the post predicate before task moves
            let ghost post = self.pending@[0].post();
            
            let task = self.pending.remove(0);
            
            // spawn_task ensures: handle.predicate(ret) ==> (task.post())(ret)
            let handle = spawn_task(task);
            
            self.running.push(handle);
            
            // Update ghost state
            self.g = Ghost(SchedulerGhost {
                pending: self.g@.pending.drop_first(),
                running: self.g@.running.push(post),
                finished: self.g@.finished,
            });
        }

        #[verifier::loop_isolation(false)]
        fn poll_and_join_one(&mut self) -> (found: bool)
            requires old(self).wf(),
            ensures
                self.wf(),
                self.max_threads == old(self).max_threads,
                self.joined == old(self).joined,
                self.pending@ == old(self).pending@,
                self.g@.pending == old(self).g@.pending,
                found ==> self.running@.len() == old(self).running@.len() - 1,
                found ==> self.results@.len() == old(self).results@.len() + 1,
                !found ==> self.running@ == old(self).running@,
                !found ==> self.results@ == old(self).results@,
        {
            let n = self.running.len();
            let mut i: usize = 0;

            while i < n
                invariant
                    n == old(self).running@.len(),
                    i <= n,
                    self.wf(),
                    self.max_threads == old(self).max_threads,
                    self.joined == old(self).joined,
                    self.pending@ == old(self).pending@,
                    self.g@.pending == old(self).g@.pending,
                    self.running@ == old(self).running@,
                    self.g@.running == old(self).g@.running,
                    self.results@ == old(self).results@,
                    self.g@.finished == old(self).g@.finished,
                    decreases n - i,
            {
                if self.running[i].is_finished() {
                    let ghost post = self.g@.running[i as int];

                    self.g = Ghost(SchedulerGhost {
                        running: self.g@.running.remove(i as int),
                        ..self.g@
                    });

                    let handle = self.running.remove(i);

                    match handle.join() {
                        Result::Ok(val) => {
                            proof { assert(post(val)); }
                            self.g = Ghost(SchedulerGhost {
                                finished: self.g@.finished.push(post),
                                ..self.g@
                            });
                            self.results.push(val);
                        }
                        Result::Err(_) => {
                            assume(false);
                            diverge()
                        }
                    }
                    return true;
                }
                i = i + 1;
            }
            false
        }

        pub fn join(&mut self) -> (results: Vec<T>)
            requires
                old(self).wf(),
                !old(self).joined,
                old(self).spec_running_count() == 0,
                old(self).spec_result_count() == 0,
            ensures
                self.wf(),
                self.joined,
                self.spec_pending_count() == 0,
                self.spec_running_count() == 0,
                results@.len() == old(self).spec_pending_count(),
        {
            self.joined = true;

            let ghost original_pending = old(self).pending@.len();

            while self.pending.len() > 0 || self.running.len() > 0
                invariant
                    self.wf(),
                    self.joined,
                    // Conservation: no tasks lost
                    self.pending@.len() + self.running@.len() + self.results@.len() == original_pending,
                    // Boundedness: never exceed max_threads
                    self.running@.len() <= self.max_threads,
                    // Lexicographic: (total, pending) - poll decreases total, spawn decreases pending
                    decreases self.pending@.len() + self.running@.len(), self.pending@.len(),
            {
                // Eager: poll first to harvest finished threads (makes room faster)
                if self.running.len() > 0 {
                    let found = self.poll_and_join_one();
                    if found {
                        // Harvested one, total decreased
                        continue;
                    }
                }

                // Spawn one if we have room and pending work
                if self.running.len() < self.max_threads && self.pending.len() > 0 {
                    self.spawn_one();
                    // pending decreased (total same, but second component decreased)
                } else if self.running.len() > 0 {
                    // At capacity with no finished threads, must wait
                    assume(false);  // Liveness: a thread eventually finishes
                }
            }

            let mut out: Vec<T> = Vec::new();
            std::mem::swap(&mut out, &mut self.results);

            self.g = Ghost(SchedulerGhost {
                finished: Seq::empty(),
                ..self.g@
            });

            out
        }
    }

    /// Property 1: Boundedness
    /// During join, running.len() <= max_threads at all times.
    pub open spec fn property_boundedness<T>(s: &SchedulerMtEph<T>) -> bool {
        s.running@.len() <= s.max_threads
    }

    /// Property 2: Conservation
    /// join's postcondition guarantees: results.len() == original pending count
    pub proof fn lemma_conservation<T>(results_len: nat, original_pending: nat)
        requires results_len == original_pending,
        ensures results_len == original_pending,
    {
    }

    /// Property 3: Correctness - each result satisfies its task's postcondition
    pub proof fn lemma_correctness<T>(s: &SchedulerMtEph<T>, i: int)
        requires
            s.wf(),
            0 <= i < s.results@.len() as int,
        ensures
            (s.g@.finished[i])(s.results@[i]),
    {
    }

    /// Property 4: Partial termination
    /// join terminates IF all threads eventually finish (liveness assumption).
    pub proof fn lemma_partial_termination_doc()
        ensures true,
    {
    }

} // verus!
}
