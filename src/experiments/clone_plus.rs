// Copyright (c) 2025 Brian G. Milnes
//! Test clone_plus preserves views with and without feq

pub mod clone_plus_test {
    use vstd::prelude::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::*;

    verus! {

    // Test 1: Does clone_plus give us cloned?
    fn test_clone_gives_cloned<T: Clone>(x: &T) {
        let x_clone = x.clone_plus();
        assert(cloned(*x, x_clone));  // Should prove from postcondition
    }

    // Test 2: Without feq, can we get view equality from clone_plus?
    #[verifier::external_body]
    fn test_clone_view_no_feq<T: Clone + View>(x: &T) {
        let x_clone = x.clone_plus();
        assert(cloned(*x, x_clone));
        assert(x@ == x_clone@);  // Fails without feq
    }

    // Test 3: With feq, can we get view equality from clone_plus?
    fn test_clone_view_with_feq<T: Clone + View + Eq>(x: &T)
        requires obeys_feq_full::<T>()
    {
        let x_clone = x.clone_plus();
        assert(x@ == x_clone@);  // Proves.
    }

    // Test 4: With feq, can we get exec equality from clone_plus?
    fn test_clone_eq_with_feq<T: Clone + View + Eq>(x: &T) -> (feq_works: bool)
        requires obeys_feq_full::<T>(),
        ensures feq_works,
    {
        let x_clone = x.clone_plus();
//        assert(cloned(*x, x_clone));
        *x == x_clone  // Proves.
    }

    } // verus!
}
