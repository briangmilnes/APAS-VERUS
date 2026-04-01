//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! ClonePreservesView — a marker trait for types whose `clone()` preserves the View.
//!
//! In Verus, generic `Clone::clone` carries no postcondition. Concrete impls in
//! APAS-VERUS put `ensures result@ == self@` on clone bodies (with an assume bridge),
//! but generic code `V: Clone + View` cannot use that postcondition. This trait
//! provides `clone_view()` with `ensures result@ == self@` for use in generic contexts.
//!
//! For Copy types (usize, u64, etc.), `clone_view` is a simple copy — no assume needed.
//! For compound types (Edge<V>, Pair<K, V>), `clone_view` delegates to component
//! `clone_view` calls, which chain ensures without assumes.

#[cfg(verus_keep_ghost)]
pub mod clone_view {
    use vstd::prelude::*;

    verus! {

    /// Trait for types whose clone preserves the view.
    pub trait ClonePreservesView: Clone + View + Sized {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@;
    }

    impl ClonePreservesView for usize {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for u64 {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for i64 {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for u32 {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for i32 {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for u16 {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for i128 {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    impl ClonePreservesView for bool {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            *self
        }
    }

    /// Trait for types whose clone preserves well-formedness.
    /// Same category as the eq/clone workaround — Clone preserves structural
    /// invariants (sorted, balanced, no-dup) that Verus cannot derive generically.
    pub trait ClonePreservesWf: Clone + View + Sized {
        spec fn spec_wf(&self) -> bool;

        fn clone_wf(&self) -> (result: Self)
            requires self.spec_wf(),
            ensures result.spec_wf(), result@ == self@;
    }

    } // verus!
}

#[cfg(not(verus_keep_ghost))]
pub mod clone_view {
    /// Trait for types whose clone preserves the view.
    pub trait ClonePreservesView: Clone + Sized {
        fn clone_view(&self) -> Self;
    }

    impl<T: Clone> ClonePreservesView for T {
        fn clone_view(&self) -> Self { self.clone() }
    }

    /// Trait for types whose clone preserves well-formedness.
    pub trait ClonePreservesWf: Clone + Sized {
        fn clone_wf(&self) -> Self;
    }
}
