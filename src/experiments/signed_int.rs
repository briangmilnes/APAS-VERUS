// Copyright (c) 2025 Brian G. Milnes
//! Trait for signed integer types (i8, i16, i32, i64, i128, isize).
//! No monotonicity guarantee for addition.

#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

broadcast use vstd::arithmetic::mul::group_mul_properties;

pub trait SignedInt: View<V = int> + Sized + Clone {
    spec fn spec_min() -> int;
    spec fn spec_max() -> int;
    open spec fn spec_zero() -> int { 0 }
    open spec fn spec_in_range(v: int) -> bool { Self::spec_min() <= v <= Self::spec_max() }
    open spec fn spec_add(a: int, b: int) -> int { a + b }
    open spec fn spec_sub(a: int, b: int) -> int { a - b }
    open spec fn spec_mul(a: int, b: int) -> int { a * b }
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

    proof fn lemma_sub_anti_commutative(a: int, b: int)
        ensures a - b == -1 * (b - a)
    {
        assert(a - b == -(b - a));
        assert(-1 * (b - a) == -(b - a)) by {
            vstd::arithmetic::mul::lemma_mul_basics(b - a);
        }
    }
}

} // verus!
