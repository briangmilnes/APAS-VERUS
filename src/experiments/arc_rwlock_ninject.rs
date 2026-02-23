// Copyright (c) 2025 Brian G. Milnes
//! arc_rwlock_ninject — Experiment: Arc<RwLock<Vec<T>>> + spawn for parallel ninject.
//!
//! Hypothesis: Verus can verify threads contending for a single RwLock to produce
//! nondeterministic writes to a shared buffer, using RwLockPredicate and spawn.
//!
//! Result: PASSES with assumes for clone-view, Arc::clone pred, and ghost field opacity.
//!
//! ASSUMES: Several assume()s for clone-view, Arc::clone pred, and ghost field opacity.
//! The goal is to test the RwLock acquire/write/release + spawn machinery,
//! not to close every proof hole.

use vstd::prelude::*;
use vstd::rwlock::*;
use vstd::thread::*;
use std::sync::Arc;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

// 1. Lock predicate

/// Ghost state carried by the lock. The invariant says the buffer is
/// always a valid partial ninject result: every element is either the
/// original or came from some update.
pub struct NinjectPred<T> {
    pub ghost source: Seq<T>,
    pub ghost updates: Seq<(usize, T)>,
}

impl<T> RwLockPredicate<Vec<T>> for NinjectPred<T> {
    open spec fn inv(self, v: Vec<T>) -> bool {
        v@.len() == self.source.len()
        && forall|i: int| #![trigger v@[i]] 0 <= i < v@.len() ==> {
            v@[i] == self.source[i]
            || exists|j: int| #![trigger self.updates[j]] 0 <= j < self.updates.len()
                && self.updates[j].0 == i as usize && v@[i] == self.updates[j].1
        }
    }
}

// 2. Single-thread write helper

/// Acquire the lock, apply updates, release. Preserves the lock invariant.
fn apply_updates<T: Clone + Send + Sync + 'static>(
    lock: Arc<RwLock<Vec<T>, NinjectPred<T>>>,
    updates: Vec<(usize, T)>,
    Ghost(pred): Ghost<NinjectPred<T>>,
)
    requires
        lock.pred() == pred,
        forall|k: int| #![trigger updates@[k]] 0 <= k < updates@.len() ==> {
            0 <= updates@[k].0 < pred.source.len()
            && exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                && pred.updates[j] == updates@[k]
        },
{
    let (mut buf, write_handle) = lock.acquire_write();
    // acquire_write ensures lock.inv(buf) which is pred.inv(buf).
    let len = buf.len();
    let mut i: usize = 0;
    while i < updates.len()
        invariant
            i <= updates@.len(),
            len == buf@.len(),
            pred.inv(buf),
            forall|k: int| #![trigger updates@[k]] 0 <= k < updates@.len() ==> {
                0 <= updates@[k].0 < pred.source.len()
                && exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                    && pred.updates[j] == updates@[k]
            },
        decreases updates@.len() - i,
    {
        let pos = updates[i].0;
        if pos < len {
            let val = updates[i].1.clone();
            proof {
                assume(val == updates@[i as int].1); // clone preserves value
            }
            buf.set(pos, val);
            proof {
                // Find the full-list witness for this update.
                let witness = choose|j: int| #![trigger pred.updates[j]]
                    0 <= j < pred.updates.len()
                    && pred.updates[j] == updates@[i as int];
                // Prove the invariant still holds after the write.
                assert forall|p: int| #![trigger buf@[p]] 0 <= p < buf@.len() implies {
                    buf@[p] == pred.source[p]
                    || exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                        && pred.updates[j].0 == p as usize && buf@[p] == pred.updates[j].1
                } by {
                    if p == pos as int {
                        assert(pred.updates[witness].0 == pos as usize);
                        assert(buf@[p] == pred.updates[witness].1);
                    }
                }
            }
        }
        i += 1;
    }
    write_handle.release_write(buf);
}

// 3. Parallel ninject

fn ninject_par<T: Clone + Send + Sync + 'static>(
    source: &Vec<T>,
    updates: &Vec<(usize, T)>,
) -> (result: Vec<T>)
    requires
        forall|k: int| #![trigger updates@[k]] 0 <= k < updates@.len() ==>
            0 <= updates@[k].0 < source@.len(),
    ensures
        result@.len() == source@.len(),
        forall|i: int| #![trigger result@[i]] 0 <= i < result@.len() ==> {
            result@[i] == source@[i]
            || exists|j: int| #![trigger updates@[j]] 0 <= j < updates@.len()
                && updates@[j].0 == i as usize && result@[i] == updates@[j].1
        },
{
    let ghost pred = NinjectPred::<T> { source: source@, updates: updates@ };

    // Clone source into the result buffer.
    let buf = source.clone();
    proof { assume(buf@ =~= source@); } // clone preserves view

    let lock = Arc::new(RwLock::<Vec<T>, NinjectPred<T>>::new(buf, Ghost(pred)));
    proof { assume(lock.pred() == pred); } // Arc::new preserves pred

    // Split updates in half, cloning each entry.
    let mid = updates.len() / 2;
    let mut left: Vec<(usize, T)> = Vec::new();
    let mut right: Vec<(usize, T)> = Vec::new();
    let mut k: usize = 0;
    while k < updates.len()
        invariant
            k <= updates@.len(),
            mid == updates@.len() / 2,
            // left entries are a subset of updates@
            forall|p: int| #![trigger left@[p]] 0 <= p < left@.len() ==> {
                exists|j: int| #![trigger updates@[j]] 0 <= j < updates@.len()
                    && updates@[j] == left@[p]
            },
            // right entries are a subset of updates@
            forall|p: int| #![trigger right@[p]] 0 <= p < right@.len() ==> {
                exists|j: int| #![trigger updates@[j]] 0 <= j < updates@.len()
                    && updates@[j] == right@[p]
            },
        decreases updates@.len() - k,
    {
        let pos = updates[k].0;
        let val = updates[k].1.clone();
        proof { assume((pos, val) == updates@[k as int]); } // clone preserves value
        if k < mid {
            left.push((pos, val));
        } else {
            right.push((pos, val));
        }
        k += 1;
    }

    // Spawn two threads that race for the lock.
    let lock1 = lock.clone();
    proof { assume(lock1.pred() == pred); } // Arc::clone preserves pred
    let lock2 = lock.clone();
    proof { assume(lock2.pred() == pred); }

    // Snapshot ghost views of left/right for closure requires.
    let ghost lv = left@;
    let ghost rv = right@;

    proof {
        // Bridge: entries in left/right that are in updates@ are also in pred.updates.
        assert(pred.updates =~= updates@);
    }

    let handle1 = spawn(
        move || -> ()
            requires
                lock1.pred() == pred,
                forall|k: int| #![trigger lv[k]] 0 <= k < lv.len() ==> {
                    0 <= lv[k].0 < pred.source.len()
                    && exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                        && pred.updates[j] == lv[k]
                },
                left@ =~= lv,
        {
            apply_updates(lock1, left, Ghost(pred));
        }
    );

    let handle2 = spawn(
        move || -> ()
            requires
                lock2.pred() == pred,
                forall|k: int| #![trigger rv[k]] 0 <= k < rv.len() ==> {
                    0 <= rv[k].0 < pred.source.len()
                    && exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                        && pred.updates[j] == rv[k]
                },
                right@ =~= rv,
        {
            apply_updates(lock2, right, Ghost(pred));
        }
    );

    // Wait for both threads.
    let _ = handle1.join();
    let _ = handle2.join();

    // Extract the result. The lock invariant gives us the postcondition.
    let (result, write_handle) = lock.acquire_write();
    proof {
        assert(pred.inv(result));
        assert(pred.updates =~= updates@);
        assert(pred.source =~= source@);
    }
    let r = result.clone();
    proof { assume(r@ =~= result@); } // clone preserves view
    write_handle.release_write(result);
    r
}

} // verus!
