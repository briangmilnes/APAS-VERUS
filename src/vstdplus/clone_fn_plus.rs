//! clone_fn_plus - Closure cloning with spec preservation
//! 
//! Verus can express that f.clone() preserves f.requires/f.ensures.
//! These are axioms (external_body) that we trust based on Rust semantics.

#[cfg(verus_keep_ghost)]
pub mod clone_fn_plus {
    use vstd::prelude::*;

    verus! {

    /// Clone a unary function preserving requires and ensures
    #[verifier::external_body]
    pub fn clone_fn<T, U, F: Fn(&T) -> U + Clone>(f: &F) -> (res: F)
        ensures
            forall|x: &T| f.requires((x,)) == res.requires((x,)),
            forall|x: &T, r: U| f.ensures((x,), r) == res.ensures((x,), r),
    {
        f.clone()
    }

    /// Clone a binary function preserving requires and ensures
    #[verifier::external_body]
    pub fn clone_fn2<T, F: Fn(&T, &T) -> T + Clone>(f: &F) -> (res: F)
        ensures
            forall|x: &T, y: &T| f.requires((x, y)) == res.requires((x, y)),
            forall|x: &T, y: &T, r: T| f.ensures((x, y), r) == res.ensures((x, y), r),
    {
        f.clone()
    }

    /// Clone a predicate preserving requires and ensures
    #[verifier::external_body]
    pub fn clone_pred<T, F: Fn(&T) -> bool + Clone>(f: &F) -> (res: F)
        ensures
            forall|x: &T| f.requires((x,)) == res.requires((x,)),
            forall|x: &T, r: bool| f.ensures((x,), r) == res.ensures((x,), r),
    {
        f.clone()
    }

    } // verus!
}

#[cfg(not(verus_keep_ghost))]
pub mod clone_fn_plus {
    pub fn clone_fn<T, U, F: Fn(&T) -> U + Clone>(f: &F) -> F { f.clone() }
    pub fn clone_fn2<T, F: Fn(&T, &T) -> T + Clone>(f: &F) -> F { f.clone() }
    pub fn clone_pred<T, F: Fn(&T) -> bool + Clone>(f: &F) -> F { f.clone() }
}
