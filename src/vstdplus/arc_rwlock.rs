// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Arc bridge — generic external_body functions for Arc::clone and
//! Arc<RwLock> construction/clone that preserve specs through Arc.
//!
//! vstd has no ensures on Arc::clone. These functions bridge that gap
//! with tight ensures so callers get full spec propagation.
//!
//! Trust boundary: three external_body functions for the entire project.
//! - clone_arc: preserves View through Arc::clone.
//! - new_arc_rwlock: preserves pred() through Arc::new(RwLock::new(...)).
//! - clone_arc_rwlock: preserves pred() through Arc::clone on RwLock.


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

    /// Clone any Arc<T>, preserving the view in ensures.
    /// Replaces per-type clone_arc_* helpers throughout the codebase.
    #[verifier::external_body]
    pub fn clone_arc<T>(arc: &Arc<T>) -> (cloned: Arc<T>)
        ensures *cloned == *arc,
    {
        arc.clone()
    }

    } // verus!
}
