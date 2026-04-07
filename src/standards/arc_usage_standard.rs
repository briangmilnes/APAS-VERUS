//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Arc Usage Standard: when Arc is needed and when it is not.
//!
//! Arc<T> provides shared ownership via reference counting. In APAS-VERUS, Arc appears
//! in four distinct roles. Three are necessary. One is an antipattern.
//!
//! Role 1: Fork-join closure sharing (NECESSARY).
//!
//!   join() closures require 'static + Send ownership. When the same value must go to
//!   both branches, Arc provides shared ownership without copying.
//!
//!   Two sub-cases:
//!   - Arc<F> for closures: filter, reduce, map functions shared across parallel halves.
//!   - Arc<ImmutableData> for read-only data: edge lists, key vectors, partition maps.
//!
//!   This is correct and unavoidable. References cannot satisfy the 'static bound.
//!
//! Role 2: Concurrent mutable state (NECESSARY).
//!
//!   Arc<RwLock<T, Inv>> when multiple threads genuinely read and write the same
//!   structure concurrently. The canonical example is parallel DP memoization: both
//!   branches of a fork-join check and update a shared memo table.
//!
//!   Use vstdplus::arc_rwlock::{new_arc_rwlock, clone_arc_rwlock} for these. They
//!   preserve pred() through Arc with tight ensures. See hfscheduler_standard.rs.
//!
//! Role 3: Persistent tree nodes (NECESSARY).
//!
//!   Link<T> = Option<Arc<Node<T>>> for persistent (path-copying) data structures.
//!   Multiple versions of the tree share subtrees via Arc. This is structural, not
//!   related to locking. Box cannot replace Arc here.
//!
//! Role 4: Arc<RwLock<Inner>> as the struct field in an Mt wrapper (ANTIPATTERN).
//!
//!   Some Mt modules store Arc<RwLock<Inner, Inv>> as their struct field instead of
//!   plain RwLock<Inner, Inv>. This is unnecessary. APAS Mt modules do not share
//!   mutable state across threads. The pattern is: build (single thread), fork
//!   (read-only closures or owned splits), join (combine). The locked wrapper lives
//!   on one thread. Plain RwLock suffices.
//!
//!   Arc<RwLock> in the struct field costs:
//!   - An extra allocation and indirection per operation.
//!   - An external_body or assume for Arc::clone pred() preservation.
//!   - Loss of &mut self: Arc is not DerefMut, so you cannot get &mut Inner.
//!   - No ghost shadow field: Arc's opacity prevents type_invariant.
//!   - Weaker specs: without ghost shadow, you cannot track value-level properties.
//!
//!   Plain RwLock in the struct field gives:
//!   - Direct field access, no Arc indirection.
//!   - Ghost shadow field (ghost_locked_X) for value-level specs.
//!   - type_invariant on the wrapper struct.
//!   - &mut self works: the struct owns the lock directly.
//!   - Stronger specs: View through ghost shadow, Result-returning trait methods.
//!
//! Summary: use Arc when you need shared ownership (roles 1-3). Do not use Arc when
//! the struct owns the lock outright (role 4 — use plain RwLock instead).
//!
//! References:
//! - toplevel_coarse_rwlocks_for_mt_modules.rs (correct pattern: plain RwLock).
//! - hfscheduler_standard.rs (correct pattern: Arc<RwLock> for join()).
//! - vstdplus/arc_rwlock.rs (generic new_arc_rwlock / clone_arc_rwlock bridges).
//! - vstdplus/smart_ptrs.rs (Arc::clone spec, arc_deref, call_f).

pub mod arc_usage_standard {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    // 1. CORRECT: Plain RwLock as struct field.
    //
    // The Mt wrapper owns the lock. Ghost shadow tracks the value.
    // type_invariant constrains the ghost. View goes through the ghost.
    // This is the toplevel coarse RwLock standard.

    pub struct Widget {
        pub value: u64,
    }

    impl View for Widget {
        type V = u64;
        open spec fn view(&self) -> u64 { self.value as u64 }
    }

    pub trait WidgetTrait: Sized + View<V = u64> {
        spec fn spec_widget_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            ensures s.spec_widget_wf(), s@ == v;

        fn get(&self) -> (v: u64)
            requires self.spec_widget_wf(),
            ensures v == self@;
    }

    impl WidgetTrait for Widget {
        open spec fn spec_widget_wf(&self) -> bool { true }

        fn new(v: u64) -> (s: Self) { Widget { value: v } }

        fn get(&self) -> (v: u64) { self.value }
    }

    pub struct WidgetInv;

    impl RwLockPredicate<Widget> for WidgetInv {
        open spec fn inv(self, v: Widget) -> bool { v.spec_widget_wf() }
    }

    // CORRECT: plain RwLock, ghost shadow, type_invariant.
    pub struct LockedWidget {
        pub(crate) locked_widget: RwLock<Widget, WidgetInv>,
        pub(crate) ghost_locked_widget: Ghost<u64>,
    }

    impl LockedWidget {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool { true }

        pub closed spec fn spec_ghost_locked_widget(self) -> u64 {
            self.ghost_locked_widget@
        }
    }

    impl View for LockedWidget {
        type V = u64;
        open spec fn view(&self) -> u64 { self.spec_ghost_locked_widget() }
    }

    pub trait LockedWidgetTrait: Sized + View<V = u64> {
        spec fn spec_lockedwidget_wf(&self) -> bool;

        fn new(v: u64) -> (s: Self)
            ensures s.spec_lockedwidget_wf(), s@ == v;

        fn get(&self) -> (v: u64)
            requires self.spec_lockedwidget_wf(),
            ensures v == self@;
    }

    impl LockedWidgetTrait for LockedWidget {
        open spec fn spec_lockedwidget_wf(&self) -> bool { true }

        fn new(v: u64) -> (s: Self) {
            let w = Widget::new(v);
            LockedWidget {
                locked_widget: RwLock::new(w, Ghost(WidgetInv)),
                ghost_locked_widget: Ghost(v),
            }
        }

        fn get(&self) -> (v: u64) {
            let handle = self.locked_widget.acquire_read();
            let v = handle.borrow().get();
            proof { assume(v == self@); }
            handle.release_read();
            v
        }
    }

    // 2. ANTIPATTERN: Arc<RwLock> as struct field.
    //
    // Do NOT do this for Mt wrappers. Arc is unnecessary when the struct
    // owns the lock and no other thread holds a clone.
    //
    // Problems:
    // - No ghost shadow (Arc's opacity blocks type_invariant).
    // - No value-level specs on the wrapper (View cannot see through Arc).
    // - Requires external_body bridges for pred() preservation.
    // - Every operation pays Arc indirection cost.
    //
    // If you see this pattern in existing code, it is technical debt, not
    // a design choice. New Mt modules must use plain RwLock (pattern 1).

    // pub struct BadLockedWidget {
    //     pub inner: Arc<RwLock<Widget, WidgetInv>>,  // WRONG
    // }
    //
    // Why it's wrong:
    // - Cannot add ghost_locked_widget field (Arc is opaque).
    // - Cannot add type_invariant (Arc blocks it).
    // - View would need to read through the lock, defeating the point.
    // - new() needs new_arc_rwlock (external_body) instead of RwLock::new.
    // - clone() needs clone_arc_rwlock (external_body) for pred().
    // - get() cannot relate return value to self@ without a ghost shadow.

    // 3. CORRECT: Arc<RwLock> for concurrent shared state (not struct field).
    //
    // When two threads must read/write the SAME lock (e.g., DP memoization),
    // Arc<RwLock> is necessary. The Arc lives as a local variable, cloned into
    // each join() closure. It is NOT a struct field.
    //
    // See hfscheduler_standard.rs for the full pattern with new_arc_rwlock,
    // clone_arc_rwlock, and join().

    // 4. CORRECT: Arc<F> for fork-join closure sharing.
    //
    // When a closure F must go to both branches of join(), wrap in Arc.
    // The Arc is a local, not a struct field.

    fn example_arc_closure_sharing() {
        let data: Vec<u64> = Vec::new();
        let arc_data = Arc::new(data);
        let arc1 = arc_data.clone();
        let arc2 = arc_data.clone();

        // Both closures can read arc_data without copying.
        // This is the correct use of Arc: shared read-only ownership.
        let f1 = move || -> (r: usize)
            ensures r == (*arc1)@.len(),
        {
            arc1.len()
        };

        let f2 = move || -> (r: usize)
            ensures r == (*arc2)@.len(),
        {
            arc2.len()
        };

        // let (a, b) = join(f1, f2);
        // Omitted: this standard doesn't import HFScheduler.
    }

    // 5. CORRECT: Arc<Node<T>> for persistent data structures.
    //
    // Persistent trees use Arc for structural sharing between versions.
    // This is unrelated to locking. Box cannot replace Arc because
    // multiple tree versions share the same subtree nodes.
    //
    // type Link<T> = Option<Arc<Node<T>>>;

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for Widget {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Widget({})", self.value)
        }
    }
    impl std::fmt::Display for Widget {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Widget({})", self.value)
        }
    }

    impl std::fmt::Debug for WidgetInv {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "WidgetInv")
        }
    }
    impl std::fmt::Display for WidgetInv {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "WidgetInv")
        }
    }

    impl std::fmt::Debug for LockedWidget {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedWidget")
        }
    }
    impl std::fmt::Display for LockedWidget {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedWidget")
        }
    }
}
