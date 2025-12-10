//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Work-stealing scheduler with bounded parallelism.

pub mod WSSchedulerMtEph {
    use vstd::prelude::*;
    use crate::vstdplus::threads_plus::threads_plus::{spawn_plus, JoinHandlePlus};
    use crate::Concurrency::diverge;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
} // mod
