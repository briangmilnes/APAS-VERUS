//! clone_plus - Add postcondition to generic Clone::clone

pub mod clone_plus {
    use vstd::prelude::*;
    use core::clone::Clone;

    verus! {

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

    } // verus!
}
