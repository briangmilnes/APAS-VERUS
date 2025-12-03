#[cfg(verus_keep_ghost)]
pub mod bounded_nat {

    #[allow(unused_imports)]
    use vstd::prelude::*;
    #[allow(unused_imports)]
    use vstd::view::View;
    #[allow(unused_imports)]
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::mul::{lemma_mul_by_zero_is_zero, lemma_mul_inequality, lemma_mul_is_commutative};

verus! {

    #[derive(Copy)]
    pub enum GhostNat { Normal(Ghost<nat>), Overflow(Ghost<nat>) }
    impl Clone for GhostNat { fn clone(&self) -> (clone: Self) ensures clone == *self { *self } }

    #[derive(Copy)]
    pub enum ExecNat<T: Copy + Clone> { Normal(T), Overflow }
    impl<T: Copy + Clone> Clone for ExecNat<T> { fn clone(&self) -> (clone: Self) ensures clone == *self { *self } }

    #[derive(Copy)]
    pub struct BoundedNat<T: Copy + Clone> { pub gn: GhostNat, pub en: ExecNat<T> }
    impl<T: Copy + Clone> Clone for BoundedNat<T> { fn clone(&self) -> (clone: Self) ensures clone@ == self@ { *self } }

    // BoundedNat is its own view.
    impl<T: Copy + Clone> View for BoundedNat<T> {
        type V = BoundedNat<T>;
        open spec fn view(&self) -> BoundedNat<T> { *self }
    }

    pub trait BoundedNatTrait<T: Copy + Clone>: Sized + Clone {
        type ViewType;
        spec fn spec_max() -> nat;
        spec fn spec_view(&self) -> Self::ViewType;
        spec fn spec_is_normal(&self) -> bool;
        spec fn spec_is_overflow(&self) -> bool;
        spec fn spec_add(&self, addend: &Self) -> bool;
        spec fn spec_mul(&self, factor: &Self) -> bool;

        fn is_normal  (&self) -> (normal: bool)            ensures normal == self.spec_is_normal();
        fn is_overflow(&self) -> (overflow: bool)          ensures overflow == self.spec_is_overflow();
        fn add_checked(&self, other: &Self) -> (sum: Self);
//            ensures sum.spec_value() == self.spec_value() + other.spec_value();
        fn mul_checked(&self, other: &Self) -> (product: Self);
//            ensures product.spec_value() == self.spec_value() * other.spec_value();

    }

    impl BoundedNatTrait<u32> for BoundedNat<u32> {
        type ViewType = BoundedNat<u32>;
        open spec fn spec_max()              -> nat { u32::MAX as nat}
        open spec fn spec_view(&self)        -> Self::ViewType { *self }

        open spec fn spec_is_normal(&self)   -> bool { match self.en { ExecNat::Normal(_) => true,  ExecNat::Overflow  => false} }
        open spec fn spec_is_overflow(&self) -> bool { match self.en { ExecNat::Normal(_) => false, ExecNat::Overflow  => true } }

        open spec fn spec_add(&self, addend: &BoundedNat<u32>) -> bool
        {
            match self.gn {
                GhostNat::Overflow(i) => 
                GhostNat::Normal(i) => 
            }
        }

        open spec fn spec_mul(&self, factor: &BoundedNat<u32>) -> bool {false}

        fn is_normal  (&self)           -> (normal: bool)
            ensures normal == self.spec_is_normal()
        { match self.en { ExecNat::Normal(_) => true, ExecNat::Overflow  => false} }

        fn is_overflow(&self)           -> (overflow: bool)
            ensures overflow == self.spec_is_overflow()
        { match self.en { ExecNat::Normal(_) => false, ExecNat::Overflow  => true } }

        fn add_checked(&self, other: &Self) -> (sum:     Self) { *self }
        // use self.add_checked()

        fn mul_checked(&self, other: &Self) -> (product: Self) { *self}
        // use self.mul_checked()
    }

 } // verus

}
