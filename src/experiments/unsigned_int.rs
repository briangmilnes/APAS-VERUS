//! Trait for unsigned integer types (u8, u16, u32, u64, u128, usize).
//! Addition is monotonic (no underflow) so partial sums fit if final sum fits.

#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

broadcast use vstd::arithmetic::mul::group_mul_properties;

pub trait UnsignedInt: View<V = int> + Sized + Clone {
    spec fn spec_max() -> nat;
    open spec fn spec_zero() -> int { 0 }
    open spec fn spec_add(a: int, b: int) -> int { a + b }
    open spec fn spec_mul(a: int, b: int) -> int { a * b }
    open spec fn spec_in_range(v: int) -> bool { 0 <= v <= Self::spec_max() as int }

    proof fn lemma_add_commutative(a: int, b: int)
        ensures a + b == b + a
    {}

    proof fn lemma_mul_commutative(a: int, b: int)
        ensures a * b == b * a
    {}

    proof fn lemma_add_associative(a: int, b: int, c: int)
        ensures (a + b) + c == a + (b + c)
    {}

    proof fn lemma_mul_associative(a: int, b: int, c: int)
        ensures (a * b) * c == a * (b * c)
    {}

    proof fn lemma_mul_distributes_over_add(a: int, b: int, c: int)
        ensures a * (b + c) == a * b + a * c
    {}

    proof fn lemma_add_monotonic(a: int, b: int)
        requires a >= 0, b >= 0
        ensures a + b >= a, a + b >= b
    {}

    proof fn lemma_partial_sums_fit(a: int, b: int, max: nat)
        requires
            a >= 0, b >= 0,
            a + b <= max as int
        ensures
            a <= max as int,
            b <= max as int
    {}
}

} // verus!
