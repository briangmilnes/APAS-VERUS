//! clone_plus - Add postconditions to Clone::clone for data and closures

#[cfg(verus_keep_ghost)]
pub mod clone_plus {
    use vstd::prelude::*;
    use core::clone::Clone;

    verus! {

    // Data cloning with cloned() postcondition
    pub trait ClonePlus: Clone + Sized {
        fn clone_plus(&self) -> (res: Self)
            ensures cloned(*self, res);
    }

    impl<T: Clone> ClonePlus for T {
        #[verifier::external_body]
        fn clone_plus(&self) -> (res: Self) {
            self.clone()
        }
    }

    // Closure cloning with spec preservation

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
pub mod clone_plus {
    pub trait ClonePlus: Clone + Sized {
        fn clone_plus(&self) -> Self;
    }

    impl<T: Clone> ClonePlus for T {
        fn clone_plus(&self) -> Self { self.clone() }
    }

    pub fn clone_fn<T, U, F: Fn(&T) -> U + Clone>(f: &F) -> F { f.clone() }
    pub fn clone_fn2<T, F: Fn(&T, &T) -> T + Clone>(f: &F) -> F { f.clone() }
    pub fn clone_pred<T, F: Fn(&T) -> bool + Clone>(f: &F) -> F { f.clone() }
}
