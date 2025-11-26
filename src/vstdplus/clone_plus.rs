//! clone_plus - Add postcondition to generic Clone::clone

#[cfg(verus_keep_ghost)]
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

#[cfg(not(verus_keep_ghost))]
pub mod clone_plus {
    /// ClonePlus trait for non-Verus builds - just delegates to clone()
    pub trait ClonePlus: Clone + Sized {
        fn clone_plus(&self) -> Self;
    }

    impl<T: Clone> ClonePlus for T {
        fn clone_plus(&self) -> Self {
            self.clone()
        }
    }
}
