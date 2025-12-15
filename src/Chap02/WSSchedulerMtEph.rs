//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Work-stealing scheduler with bounded parallelism.

pub mod WSSchedulerMtEph {
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::threads_plus::threads_plus::{spawn_plus, JoinHandlePlus};
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::Concurrency::diverge;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(verus_keep_ghost)]
verus! {

    #[verifier::external_body]
    pub struct Pool {
        budget: Arc<AtomicUsize>,
    }

    impl Clone for Pool {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            Pool { budget: self.budget.clone() }
        }
    }

    impl Pool {
        /// Clone with spec: cloned(*self, result)
        pub fn clone_plus(&self) -> (result: Self)
            ensures cloned(*self, result),
        {
            self.clone()
        }
    }

    impl Pool {
        pub uninterp spec fn spec_size(&self) -> nat;

        #[verifier::external_body]
        pub fn new(size: usize) -> (pool: Self)
            requires size > 0,
            ensures pool.spec_size() == size as nat,
        {
            Pool { budget: Arc::new(AtomicUsize::new(size)) }
        }

        #[verifier::external_body]
        fn try_acquire(&self) -> bool {
            let old = self.budget.fetch_sub(1, Ordering::SeqCst);
            if old > 0 {
                true
            } else {
                self.budget.fetch_add(1, Ordering::SeqCst);
                false
            }
        }

        #[verifier::external_body]
        fn release(&self) {
            self.budget.fetch_add(1, Ordering::SeqCst);
        }

        pub fn join<A, B, FA, FB>(&self, fa: FA, fb: FB) -> (result: (A, B))
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
                fb.ensures((), result.1),
        {
            if self.try_acquire() {
                let result = join(fa, fb);
                self.release();
                result
            } else {
                (fa(), fb())
            }
        }
    }

    pub fn join<A, B, FA, FB>(fa: FA, fb: FB) -> (result: (A, B))
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
            fb.ensures((), result.1),
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

} // verus!

// Non-Verus stub for Pool
#[cfg(not(verus_keep_ghost))]
pub struct Pool {
    budget: Arc<AtomicUsize>,
}

#[cfg(not(verus_keep_ghost))]
impl Clone for Pool {
    fn clone(&self) -> Self {
        Pool { budget: self.budget.clone() }
    }
}

#[cfg(not(verus_keep_ghost))]
impl Pool {
    pub fn new(size: usize) -> Self {
        Pool { budget: Arc::new(AtomicUsize::new(size)) }
    }
    
    fn try_acquire(&self) -> bool {
        let old = self.budget.fetch_sub(1, Ordering::SeqCst);
        if old > 0 { true } else {
            self.budget.fetch_add(1, Ordering::SeqCst);
            false
        }
    }
    
    fn release(&self) {
        self.budget.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn join<A, B, FA, FB>(&self, fa: FA, fb: FB) -> (A, B)
    where
        FA: FnOnce() -> A + Send + 'static,
        FB: FnOnce() -> B + Send + 'static,
        A: Send + 'static,
        B: Send + 'static,
    {
        if self.try_acquire() {
            let result = join(fa, fb);
            self.release();
            result
        } else {
            (fa(), fb())
        }
    }
}

#[cfg(not(verus_keep_ghost))]
pub fn join<A, B, FA, FB>(fa: FA, fb: FB) -> (A, B)
where
    FA: FnOnce() -> A + Send + 'static,
    FB: FnOnce() -> B + Send + 'static,
    A: Send + 'static,
    B: Send + 'static,
{
    use std::thread;
    let handle = thread::spawn(fb);
    let a = fa();
    let b = handle.join().unwrap();
    (a, b)
}
} // mod
