//! Checked unsigned integer trait with overflow tracking.
//! Key property: if final sum fits, all partial sums fit (monotonicity).

#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

broadcast use vstd::arithmetic::mul::group_mul_properties;

pub trait CheckedUnsignedInt: View<V = int> + Sized + Clone {
    spec fn spec_max() -> nat;
    open spec fn spec_zero() -> int { 0 }
    open spec fn spec_in_range(v: int) -> bool { 0 <= v <= Self::spec_max() as int }

    fn is_normal(&self) -> (normal: bool)
        ensures normal == Self::spec_in_range(self@);

    fn is_overflow(&self) -> (overflow: bool)
        ensures overflow == (self@ > Self::spec_max() as int);

    fn from_value(v: u64) -> (checked: Self)
        requires v <= Self::spec_max()
        ensures checked@ == v as int;

    fn from_int(i: Ghost<int>) -> (checked: Self)
        ensures checked@ == i@;

    fn to_value(&self) -> (value: u64)
        requires Self::spec_in_range(self@)
        ensures value as int == self@;

    fn add_checked(&self, other: &Self) -> (sum: Self)
        ensures sum@ == self@ + other@;

    fn mul_checked(&self, other: &Self) -> (product: Self)
        ensures product@ == self@ * other@;

    fn clone_checked(&self) -> (clone: Self)
        ensures clone@ == self@;

    proof fn lemma_add_commutative_ghost(a: Self, b: Self)
        ensures a@ + b@ == b@ + a@
    {}

    proof fn lemma_mul_commutative_ghost(a: Self, b: Self)
        ensures a@ * b@ == b@ * a@
    {}

    proof fn lemma_add_associative_ghost(a: Self, b: Self, c: Self)
        ensures (a@ + b@) + c@ == a@ + (b@ + c@)
    {}

    proof fn lemma_mul_associative_ghost(a: Self, b: Self, c: Self)
        ensures (a@ * b@) * c@ == a@ * (b@ * c@)
    {}

    proof fn lemma_mul_distributes_over_add_ghost(a: Self, b: Self, c: Self)
        ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@
    {}

    proof fn lemma_add_monotonic(a: Self, b: Self)
        requires a@ >= 0, b@ >= 0
        ensures a@ + b@ >= a@, a@ + b@ >= b@
    {}

    proof fn lemma_partial_sums_fit(a: Self, b: Self)
        requires a@ >= 0, b@ >= 0, a@ + b@ <= Self::spec_max() as int
        ensures a@ <= Self::spec_max() as int, b@ <= Self::spec_max() as int
    {}
}

} // verus!
