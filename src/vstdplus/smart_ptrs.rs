// Copyright (c) 2025 Brian G. Milnes
//! smart_ptrs — Specs for smart pointer operations not yet covered by vstd.
//! Extends `vstd::std_specs::smart_ptrs` with Arc::clone and Arc-deref helpers.

pub mod smart_ptrs {
    use std::sync::Arc;
    use vstd::prelude::*;

    verus! {

    /// Spec for Arc::clone: the result equals the original (Arc is transparent).
    /// Fills the gap in vstd::std_specs::smart_ptrs which has Arc::new but not Arc::clone.
    pub assume_specification<T: ?Sized, A: core::alloc::Allocator + Clone>[ <Arc<T, A> as Clone>::clone ](a: &Arc<T, A>) -> (res: Arc<T, A>)
        ensures res == *a,
    ;

    /// Call a binary function through an Arc reference.
    /// Arc<F> derefs to F, so (**f)(a, b) invokes F directly.
    /// This makes the requires/ensures explicit for Verus.
    pub fn call_f<T, F: Fn(&T, &T) -> T + Send + Sync + 'static>(
        f: &Arc<F>,
        a: &T,
        b: &T,
    ) -> (result: T)
        requires f.requires((a, b)),
        ensures f.ensures((a, b), result),
    {
        (**f)(a, b)
    }

    /// Deref an Arc to get a shared reference to the inner value.
    /// Verus cannot see through Arc's Deref impl in exec mode.
    #[verifier::external_body]
    pub fn arc_deref<T>(a: &Arc<T>) -> (r: &T)
        ensures *r == **a,
    {
        a.as_ref()
    }

    /// Get a subslice of the Vec inside an Arc. Combines Arc deref,
    /// Vec-to-slice coercion, and range indexing in one step.
    #[verifier::external_body]
    pub fn arc_vec_as_slice<'a, T>(a: &'a Arc<Vec<T>>, start: usize, len: usize) -> (r: &'a [T])
        requires start + len <= (*a)@.len(),
        ensures r@ == (*a)@.subrange(start as int, (start + len) as int),
    {
        &a[start..start + len]
    }

    } // verus!
}
