//! Checked signed integer types that track overflow and underflow.
//! Weaker guarantees than unsigned: both overflow and underflow possible.

#[cfg(verus_keep_ghost)]
pub mod checked_int {

#[allow(unused_imports)]
use vstd::prelude::*;
#[allow(unused_imports)]
use vstd::view::View;
#[allow(unused_imports)]
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::mul::{lemma_mul_by_zero_is_zero, lemma_mul_inequality, lemma_mul_is_commutative};

verus! {

// Spec functions for folding operations on int
pub open spec fn spec_add(a: int, b: int) -> int { a + b }
pub open spec fn spec_mul(a: int, b: int) -> int { a * b }

// Spec predicates for algebraic properties (compatible with Set.fold's is_fun_commutative)
pub open spec fn spec_is_add_commutative() -> bool {
    forall |a1: int, a2: int, b: int| #[trigger] spec_add(spec_add(b, a2), a1) == spec_add(spec_add(b, a1), a2)
}

pub open spec fn spec_is_mul_commutative() -> bool {
    forall |a1: int, a2: int, b: int| #[trigger] spec_mul(spec_mul(b, a2), a1) == spec_mul(spec_mul(b, a1), a2)
}

pub open spec fn spec_is_add_associative() -> bool {
    forall |a: int, b: int, c: int| #[trigger] spec_add(spec_add(a, b), c) == spec_add(a, spec_add(b, c))
}

pub open spec fn spec_is_mul_associative() -> bool {
    forall |a: int, b: int, c: int| #[trigger] spec_mul(spec_mul(a, b), c) == spec_mul(a, spec_mul(b, c))
}

pub open spec fn spec_mul_distributes_over_add() -> bool {
    forall |a: int, b: int, c: int| #[trigger] spec_mul(a, spec_add(b, c)) == spec_add(spec_mul(a, b), spec_mul(a, c))
}

// Proofs that these properties hold
pub proof fn lemma_spec_add_commutative()
    ensures spec_is_add_commutative()
{}

pub proof fn lemma_spec_mul_commutative()
    ensures spec_is_mul_commutative()
{
    assert forall |a1: int, a2: int, b: int| #[trigger] spec_mul(spec_mul(b, a2), a1) == spec_mul(spec_mul(b, a1), a2) by {
        lemma_mul_is_commutative(b, a2);
        lemma_mul_is_commutative(b, a1);
        lemma_mul_is_commutative(b * a2, a1);
        lemma_mul_is_commutative(b * a1, a2);
        vstd::arithmetic::mul::lemma_mul_is_associative(b, a2, a1);
        vstd::arithmetic::mul::lemma_mul_is_associative(b, a1, a2);
    }
}

pub proof fn lemma_spec_add_associative()
    ensures spec_is_add_associative()
{}

pub proof fn lemma_spec_mul_associative()
    ensures spec_is_mul_associative()
{
    assert forall |a: int, b: int, c: int| #[trigger] spec_mul(spec_mul(a, b), c) == spec_mul(a, spec_mul(b, c)) by {
        vstd::arithmetic::mul::lemma_mul_is_associative(a, b, c);
    }
}

pub proof fn lemma_spec_mul_distributes_over_add()
    ensures spec_mul_distributes_over_add()
{
    assert forall |a: int, b: int, c: int| #[trigger] spec_mul(a, spec_add(b, c)) == spec_add(spec_mul(a, b), spec_mul(a, c)) by {
        vstd::arithmetic::mul::lemma_mul_is_distributive_add(a, b, c);
    }
}

/// Trait for checked signed integer types that track overflow/underflow.
/// Weaker guarantees: both overflow AND underflow possible.
/// Cannot guarantee intermediate values stay in range even if final result does.
pub trait CheckedIntTrait: View<V = int> + Sized + Clone {
    spec fn spec_min() -> int;
    spec fn spec_max() -> int;

    fn is_normal(&self) -> (normal: bool)
        ensures normal == (Self::spec_min() <= self@ <= Self::spec_max());

    fn is_out_of_range(&self) -> (out_of_range: bool)
        ensures out_of_range == !(Self::spec_min() <= self@ <= Self::spec_max());

    fn add_checked(&self, other: &Self) -> (sum: Self)
        ensures sum@ == self@ + other@;

    fn sub_checked(&self, other: &Self) -> (diff: Self)
        ensures diff@ == self@ - other@;

    fn mul_checked(&self, other: &Self) -> (product: Self)
        ensures product@ == self@ * other@;

    // Commutativity proofs (on ghost int)
    proof fn lemma_add_commutative_ghost(a: Self, b: Self)
        ensures a@ + b@ == b@ + a@;

    proof fn lemma_mul_commutative_ghost(a: Self, b: Self)
        ensures a@ * b@ == b@ * a@;

    proof fn lemma_sub_anticommutative_ghost(a: Self, b: Self)
        ensures a@ - b@ == -(b@ - a@);

    // Associativity proofs
    proof fn lemma_add_associative_ghost(a: Self, b: Self, c: Self)
        ensures (a@ + b@) + c@ == a@ + (b@ + c@);

    proof fn lemma_mul_associative_ghost(a: Self, b: Self, c: Self)
        ensures (a@ * b@) * c@ == a@ * (b@ * c@)
    {
        vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@);
    }

    // Distributivity proofs
    proof fn lemma_mul_distributes_over_add_ghost(a: Self, b: Self, c: Self)
        ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@;

    proof fn lemma_mul_distributes_over_sub_ghost(a: Self, b: Self, c: Self)
        ensures a@ * (b@ - c@) == a@ * b@ - a@ * c@;
}

} // verus!

macro_rules! checked_int_gen {
    ($ity:ty, $cty:ident, $min:expr, $max:expr) => {
        verus! {

            pub struct $cty {
                i: Ghost<int>,
                v: Option<$ity>,
            }

            impl View for $cty {
                type V = int;

                closed spec fn view(&self) -> int {
                    self.i@
                }
            }

            impl Clone for $cty {
                exec fn clone(&self) -> (clone: Self)
                    ensures clone@ == self@
                {
                    proof { use_type_invariant(self); }
                    Self { i: self.i, v: self.v }
                }
            }

            impl $cty {
                #[verifier::type_invariant]
                spec fn well_formed(self) -> bool {
                    match self.v {
                        Some(v) => self.i@ == v as int,
                        None => self.i@ < ($min as int) || self.i@ > ($max as int),
                    }
                }

                pub open spec fn spec_min() -> int { $min as int }
                pub open spec fn spec_max() -> int { $max as int }

                pub closed spec fn spec_new(v: $ity) -> $cty {
                    $cty { i: Ghost(v as int), v: Some(v) }
                }

                #[verifier::when_used_as_spec(spec_new)]
                pub exec fn new(v: $ity) -> (checked: Self)
                    ensures checked@ == v as int
                {
                    Self { i: Ghost(v as int), v: Some(v) }
                }

                pub exec fn new_out_of_range(Ghost(i): Ghost<int>) -> (checked: Self)
                    requires i < ($min as int) || i > ($max as int)
                    ensures checked@ == i
                {
                    Self { i: Ghost(i), v: None }
                }

                pub open spec fn spec_is_normal(&self) -> bool {
                    ($min as int) <= self@ <= ($max as int)
                }

                #[verifier::when_used_as_spec(spec_is_normal)]
                pub exec fn is_normal(&self) -> (normal: bool)
                    ensures normal == self.spec_is_normal()
                {
                    proof { use_type_invariant(self); }
                    self.v.is_some()
                }

                pub open spec fn spec_is_overflowed(&self) -> bool {
                    self@ > ($max as int)
                }

                pub open spec fn spec_is_underflowed(&self) -> bool {
                    self@ < ($min as int)
                }

                pub open spec fn spec_is_out_of_range(&self) -> bool {
                    !self.spec_is_normal()
                }

                #[verifier::when_used_as_spec(spec_is_out_of_range)]
                pub exec fn is_out_of_range(&self) -> (out_of_range: bool)
                    ensures out_of_range == self.spec_is_out_of_range()
                {
                    proof { use_type_invariant(self); }
                    self.v.is_none()
                }

                pub exec fn unwrap(&self) -> (value: $ity)
                    requires self.is_normal()
                    ensures value as int == self@
                {
                    proof { use_type_invariant(self); }
                    self.v.unwrap()
                }

                pub exec fn to_option(&self) -> (option: Option<$ity>)
                    ensures
                        option.is_some() == self.is_normal(),
                        option.is_some() ==> option.unwrap() as int == self@
                {
                    proof { use_type_invariant(self); }
                    self.v
                }

                #[inline]
                #[verifier::external_body]
                pub exec fn add_value(&self, v2: $ity) -> (sum: Self)
                    ensures sum@ == self@ + v2 as int
                {
                    let new_i: Ghost<int> = Ghost(self@ + v2 as int);
                    match self.v {
                        Some(v1) => Self { i: new_i, v: v1.checked_add(v2) },
                        None => Self { i: new_i, v: None },
                    }
                }

                #[inline]
                #[verifier::external_body]
                pub exec fn add_checked(&self, v2: &$cty) -> (sum: Self)
                    ensures sum@ == self@ + v2@
                {
                    let new_i: Ghost<int> = Ghost(self@ + v2@);
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                #[inline]
                #[verifier::external_body]
                pub exec fn sub_value(&self, v2: $ity) -> (diff: Self)
                    ensures diff@ == self@ - v2 as int
                {
                    let new_i: Ghost<int> = Ghost(self@ - v2 as int);
                    match self.v {
                        Some(v1) => Self { i: new_i, v: v1.checked_sub(v2) },
                        None => Self { i: new_i, v: None },
                    }
                }

                #[inline]
                #[verifier::external_body]
                pub exec fn sub_checked(&self, v2: &$cty) -> (diff: Self)
                    ensures diff@ == self@ - v2@
                {
                    let new_i: Ghost<int> = Ghost(self@ - v2@);
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_sub(*n2) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                #[inline]
                #[verifier::external_body]
                pub exec fn mul_value(&self, v2: $ity) -> (product: Self)
                    ensures product@ == self@ * v2 as int
                {
                    let new_i: Ghost<int> = Ghost(self@ * v2 as int);
                    match self.v {
                        Some(v1) => Self { i: new_i, v: v1.checked_mul(v2) },
                        None => {
                            if v2 == 0 {
                                Self { i: new_i, v: Some(0) }
                            } else {
                                Self { i: new_i, v: None }
                            }
                        }
                    }
                }

                #[inline]
                #[verifier::external_body]
                pub exec fn mul_checked(&self, v2: &$cty) -> (product: Self)
                    ensures product@ == self@ * v2@
                {
                    let new_i: Ghost<int> = Ghost(self@ * v2@);
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                        (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                        (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                // Ghost commutativity
                pub proof fn lemma_add_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ + b@ == b@ + a@
                {}

                pub proof fn lemma_mul_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ * b@ == b@ * a@
                {}

                pub proof fn lemma_sub_anticommutative_ghost(a: $cty, b: $cty)
                    ensures a@ - b@ == -(b@ - a@)
                {}

                // Normal commutativity (if result fits)
                pub proof fn lemma_add_commutative_normal(a: $cty, b: $cty)
                    requires ($min as int) <= a@ + b@ <= ($max as int)
                    ensures (a@ + b@) as $ity == (b@ + a@) as $ity
                {}

                pub proof fn lemma_mul_commutative_normal(a: $cty, b: $cty)
                    requires ($min as int) <= a@ * b@ <= ($max as int)
                    ensures (a@ * b@) as $ity == (b@ * a@) as $ity
                {}

                pub proof fn lemma_sub_anticommutative_normal(a: $cty, b: $cty)
                    requires ($min as int) <= a@ - b@ <= ($max as int)
                    ensures (a@ - b@) as $ity == (-(b@ - a@)) as $ity
                {}

                // Associativity
                pub proof fn lemma_add_associative_ghost(a: $cty, b: $cty, c: $cty)
                    ensures (a@ + b@) + c@ == a@ + (b@ + c@)
                {}

                pub proof fn lemma_mul_associative_ghost(a: $cty, b: $cty, c: $cty)
                    ensures (a@ * b@) * c@ == a@ * (b@ * c@)
                {
                    vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@);
                }

                pub proof fn lemma_add_associative_normal(a: $cty, b: $cty, c: $cty)
                    requires ($min as int) <= a@ + b@ + c@ <= ($max as int)
                    ensures ((a@ + b@) + c@) as $ity == (a@ + (b@ + c@)) as $ity
                {}

                pub proof fn lemma_mul_associative_normal(a: $cty, b: $cty, c: $cty)
                    requires
                        ($min as int) <= a@ * b@ <= ($max as int),
                        ($min as int) <= b@ * c@ <= ($max as int),
                        ($min as int) <= (a@ * b@) * c@ <= ($max as int),
                    ensures ((a@ * b@) * c@) as $ity == (a@ * (b@ * c@)) as $ity
                {
                    vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@);
                }

                // Distributivity
                pub proof fn lemma_mul_distributes_over_add_ghost(a: $cty, b: $cty, c: $cty)
                    ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@
                {
                    vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@, b@, c@);
                }

                pub proof fn lemma_mul_distributes_over_add_normal(a: $cty, b: $cty, c: $cty)
                    requires
                        ($min as int) <= b@ + c@ <= ($max as int),
                        ($min as int) <= a@ * b@ <= ($max as int),
                        ($min as int) <= a@ * c@ <= ($max as int),
                        ($min as int) <= a@ * b@ + a@ * c@ <= ($max as int),
                    ensures (a@ * (b@ + c@)) as $ity == (a@ * b@ + a@ * c@) as $ity
                {
                    vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@, b@, c@);
                }

                pub proof fn lemma_mul_distributes_over_sub_ghost(a: $cty, b: $cty, c: $cty)
                    ensures a@ * (b@ - c@) == a@ * b@ - a@ * c@
                {
                    vstd::arithmetic::mul::lemma_mul_is_distributive_sub(a@, b@, c@);
                }

                pub proof fn lemma_mul_distributes_over_sub_normal(a: $cty, b: $cty, c: $cty)
                    requires
                        ($min as int) <= b@ - c@ <= ($max as int),
                        ($min as int) <= a@ * b@ <= ($max as int),
                        ($min as int) <= a@ * c@ <= ($max as int),
                        ($min as int) <= a@ * b@ - a@ * c@ <= ($max as int),
                    ensures (a@ * (b@ - c@)) as $ity == (a@ * b@ - a@ * c@) as $ity
                {
                    vstd::arithmetic::mul::lemma_mul_is_distributive_sub(a@, b@, c@);
                }
            }

            impl CheckedIntTrait for $cty {
                open spec fn spec_min() -> int { $min as int }
                open spec fn spec_max() -> int { $max as int }

                fn is_normal(&self) -> (normal: bool)
                    ensures normal == (Self::spec_min() <= self@ <= Self::spec_max())
                {
                    proof { use_type_invariant(self); }
                    self.v.is_some()
                }

                fn is_out_of_range(&self) -> (out_of_range: bool)
                    ensures out_of_range == !(Self::spec_min() <= self@ <= Self::spec_max())
                {
                    proof { use_type_invariant(self); }
                    self.v.is_none()
                }

                #[verifier::external_body]
                fn add_checked(&self, other: &Self) -> (sum: Self)
                    ensures sum@ == self@ + other@
                {
                    let new_i: Ghost<int> = Ghost(self@ + other@);
                    match (&self.v, &other.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                #[verifier::external_body]
                fn sub_checked(&self, other: &Self) -> (diff: Self)
                    ensures diff@ == self@ - other@
                {
                    let new_i: Ghost<int> = Ghost(self@ - other@);
                    match (&self.v, &other.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_sub(*n2) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                #[verifier::external_body]
                fn mul_checked(&self, other: &Self) -> (product: Self)
                    ensures product@ == self@ * other@
                {
                    let new_i: Ghost<int> = Ghost(self@ * other@);
                    match (&self.v, &other.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                        (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                        (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                proof fn lemma_add_commutative_ghost(a: Self, b: Self)
                    ensures a@ + b@ == b@ + a@
                {}

                proof fn lemma_mul_commutative_ghost(a: Self, b: Self)
                    ensures a@ * b@ == b@ * a@
                {}

                proof fn lemma_sub_anticommutative_ghost(a: Self, b: Self)
                    ensures a@ - b@ == -(b@ - a@)
                {}

                proof fn lemma_add_associative_ghost(a: Self, b: Self, c: Self)
                    ensures (a@ + b@) + c@ == a@ + (b@ + c@)
                {}

                proof fn lemma_mul_associative_ghost(a: Self, b: Self, c: Self)
                    ensures (a@ * b@) * c@ == a@ * (b@ * c@)
                {
                    vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@);
                }

                proof fn lemma_mul_distributes_over_add_ghost(a: Self, b: Self, c: Self)
                    ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@
                {
                    vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@, b@, c@);
                }

                proof fn lemma_mul_distributes_over_sub_ghost(a: Self, b: Self, c: Self)
                    ensures a@ * (b@ - c@) == a@ * b@ - a@ * c@
                {
                    vstd::arithmetic::mul::lemma_mul_is_distributive_sub(a@, b@, c@);
                }
            }
        }
    };
}

// Signed types only
checked_int_gen!(i8, CheckedI8, i8::MIN, i8::MAX);
checked_int_gen!(i16, CheckedI16, i16::MIN, i16::MAX);
checked_int_gen!(i32, CheckedI32, i32::MIN, i32::MAX);
checked_int_gen!(i64, CheckedI64, i64::MIN, i64::MAX);
checked_int_gen!(i128, CheckedI128, i128::MIN, i128::MAX);
checked_int_gen!(isize, CheckedIsize, isize::MIN, isize::MAX);

}

/// Non-Verus version for cargo test
#[cfg(not(verus_keep_ghost))]
pub mod checked_int {

macro_rules! checked_int_gen {
    ($ity:ty, $cty:ident, $min:expr, $max:expr) => {
        #[derive(Clone)]
        pub struct $cty {
            v: Option<$ity>,
        }

        impl $cty {
            pub fn new(v: $ity) -> Self {
                Self { v: Some(v) }
            }

            pub fn is_normal(&self) -> bool {
                self.v.is_some()
            }

            pub fn is_out_of_range(&self) -> bool {
                self.v.is_none()
            }

            pub fn unwrap(&self) -> $ity {
                self.v.unwrap()
            }

            pub fn to_option(&self) -> Option<$ity> {
                self.v
            }

            pub fn add_value(&self, v2: $ity) -> Self {
                match self.v {
                    Some(v1) => Self { v: v1.checked_add(v2) },
                    None => Self { v: None },
                }
            }

            pub fn add_checked(&self, v2: &$cty) -> Self {
                match (&self.v, &v2.v) {
                    (Some(n1), Some(n2)) => Self { v: n1.checked_add(*n2) },
                    _ => Self { v: None },
                }
            }

            pub fn sub_value(&self, v2: $ity) -> Self {
                match self.v {
                    Some(v1) => Self { v: v1.checked_sub(v2) },
                    None => Self { v: None },
                }
            }

            pub fn sub_checked(&self, v2: &$cty) -> Self {
                match (&self.v, &v2.v) {
                    (Some(n1), Some(n2)) => Self { v: n1.checked_sub(*n2) },
                    _ => Self { v: None },
                }
            }

            pub fn mul_value(&self, v2: $ity) -> Self {
                match self.v {
                    Some(v1) => Self { v: v1.checked_mul(v2) },
                    None => {
                        if v2 == 0 {
                            Self { v: Some(0) }
                        } else {
                            Self { v: None }
                        }
                    }
                }
            }

            pub fn mul_checked(&self, v2: &$cty) -> Self {
                match (&self.v, &v2.v) {
                    (Some(n1), Some(n2)) => Self { v: n1.checked_mul(*n2) },
                    (Some(n1), None) if *n1 == 0 => Self { v: Some(0) },
                    (None, Some(n2)) if *n2 == 0 => Self { v: Some(0) },
                    _ => Self { v: None },
                }
            }
        }
    };
}

// Signed types only
checked_int_gen!(i8, CheckedI8, i8::MIN, i8::MAX);
checked_int_gen!(i16, CheckedI16, i16::MIN, i16::MAX);
checked_int_gen!(i32, CheckedI32, i32::MIN, i32::MAX);
checked_int_gen!(i64, CheckedI64, i64::MIN, i64::MAX);
checked_int_gen!(i128, CheckedI128, i128::MIN, i128::MAX);
checked_int_gen!(isize, CheckedIsize, isize::MIN, isize::MAX);

}
