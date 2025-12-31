// Copyright (c) 2025 Brian G. Milnes
//! Checked signed integer trait with overflow/underflow tracking.
//! No monotonicity guarantee for addition.

#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

broadcast use vstd::arithmetic::mul::group_mul_properties;

pub trait CheckedSignedInt: View<V = int> + Sized + Clone {
    spec fn spec_min() -> int;
    spec fn spec_max() -> int;
    open spec fn spec_zero() -> int { 0 }
    open spec fn spec_in_range(v: int) -> bool { Self::spec_min() <= v <= Self::spec_max() }

    fn is_normal(&self) -> (normal: bool)
        ensures normal == Self::spec_in_range(self@);

    fn is_overflow(&self) -> (overflow: bool)
        ensures overflow == (self@ > Self::spec_max());

    fn is_underflow(&self) -> (underflow: bool)
        ensures underflow == (self@ < Self::spec_min());

    fn from_value(v: i64) -> (checked: Self)
        requires Self::spec_min() <= v <= Self::spec_max()
        ensures checked@ == v as int;

    fn from_int(i: Ghost<int>) -> (checked: Self)
        ensures checked@ == i@;

    fn to_value(&self) -> (value: i64)
        requires Self::spec_in_range(self@)
        ensures value as int == self@;

    fn add_checked(&self, other: &Self) -> (sum: Self)
        ensures sum@ == self@ + other@;

    fn sub_checked(&self, other: &Self) -> (diff: Self)
        ensures diff@ == self@ - other@;

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

    proof fn lemma_sub_anti_commutative_ghost(a: Self, b: Self)
        ensures a@ - b@ == -1 * (b@ - a@)
    {
        assert(a@ - b@ == -(b@ - a@));
        assert(-1 * (b@ - a@) == -(b@ - a@)) by {
            vstd::arithmetic::mul::lemma_mul_basics(b@ - a@);
        }
    }
}

} // verus!
