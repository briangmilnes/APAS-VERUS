//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Arc<RwLock> bridge — two generic external_body functions that preserve
//! pred() through Arc::new and Arc::clone.
//!
//! vstd's Arc spec preserves View (arc@ == (*arc)@) but RwLock's pred()
//! isn't part of its View. These two functions bridge that gap with tight
//! ensures so callers get full spec propagation.
//!
//! Trust boundary: two external_body functions for the entire project.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 9. impls

//		Section 1. module

pub mod arc_rwlock {

    //		Section 2. imports

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! 
{

    //		Section 9. impls


    /// Wrap RwLock::new in Arc, preserving pred() in ensures.
    #[verifier::external_body]
    pub fn new_arc_rwlock<V, Pred: RwLockPredicate<V>>(
        val: V,
        Ghost(pred): Ghost<Pred>,
    ) -> (arc: Arc<RwLock<V, Pred>>)
        requires pred.inv(val),
        ensures arc.pred() == pred,
    {
        Arc::new(RwLock::new(val, Ghost(pred)))
    }

    /// Clone an Arc<RwLock>, preserving pred() in ensures.
    #[verifier::external_body]
    pub fn clone_arc_rwlock<V, Pred: RwLockPredicate<V>>(
        arc: &Arc<RwLock<V, Pred>>,
    ) -> (cloned: Arc<RwLock<V, Pred>>)
        ensures cloned.pred() == arc.pred(),
    {
        arc.clone()
    }

    } // verus!
}
