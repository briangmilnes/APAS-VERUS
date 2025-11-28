//! Checked integer types that track overflow and underflow.
//! Works for both signed and unsigned integer types.
//! Based on vstd::arithmetic::overflow but extended for signed types.

#[cfg(verus_keep_ghost)]
pub mod checked_int {

#[allow(unused_imports)]
use vstd::prelude::*;
#[allow(unused_imports)]
use vstd::view::View;
#[allow(unused_imports)]
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::mul::{lemma_mul_by_zero_is_zero, lemma_mul_inequality, lemma_mul_is_commutative};

/// Macro to generate checked integer types for both signed and unsigned integers.
/// 
/// Parameters:
/// - $ity: The primitive integer type (e.g., i32, u64)
/// - $cty: The checked type name (e.g., CheckedI32, CheckedU64)
/// - $min: The minimum value expression (e.g., i32::MIN, 0)
/// - $max: The maximum value expression (e.g., i32::MAX, u64::MAX)
macro_rules! checked_int_gen {
    ($ity:ty, $cty:ident, $min:expr, $max:expr) => {
        verus! {

            /// A checked integer that tracks the true mathematical value.
            /// - `i`: Ghost value representing the true unbounded integer
            /// - `v`: Some(x) if x is in range, None if overflowed or underflowed
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
                exec fn clone(&self) -> (result: Self)
                    ensures result@ == self@
                {
                    proof { use_type_invariant(self); }
                    Self { i: self.i, v: self.v }
                }
            }

            impl $cty {
                /// Type invariant: v matches i when in range, v is None when out of range
                #[verifier::type_invariant]
                spec fn well_formed(self) -> bool {
                    match self.v {
                        Some(v) => self.i@ == v as int,
                        None => self.i@ < ($min as int) || self.i@ > ($max as int),
                    }
                }

                /// Spec: minimum value for this type
                pub open spec fn spec_min() -> int { $min as int }

                /// Spec: maximum value for this type
                pub open spec fn spec_max() -> int { $max as int }

                /// Create a new checked integer from a primitive value
                pub closed spec fn spec_new(v: $ity) -> $cty {
                    $cty { i: Ghost(v as int), v: Some(v) }
                }

                #[verifier::when_used_as_spec(spec_new)]
                pub exec fn new(v: $ity) -> (result: Self)
                    ensures result@ == v as int
                {
                    Self { i: Ghost(v as int), v: Some(v) }
                }

                /// Create a checked integer that is out of range (overflow or underflow)
                pub exec fn new_out_of_range(Ghost(i): Ghost<int>) -> (result: Self)
                    requires i < ($min as int) || i > ($max as int)
                    ensures result@ == i
                {
                    Self { i: Ghost(i), v: None }
                }

                /// True if the value is in the normal range (not overflowed or underflowed)
                pub open spec fn spec_is_normal(&self) -> bool {
                    ($min as int) <= self@ <= ($max as int)
                }

                #[verifier::when_used_as_spec(spec_is_normal)]
                pub exec fn is_normal(&self) -> (result: bool)
                    ensures result == self.spec_is_normal()
                {
                    proof { use_type_invariant(self); }
                    self.v.is_some()
                }

                /// True if the value overflowed (exceeded max)
                pub open spec fn spec_is_overflowed(&self) -> bool {
                    self@ > ($max as int)
                }

                /// True if the value underflowed (went below min)
                pub open spec fn spec_is_underflowed(&self) -> bool {
                    self@ < ($min as int)
                }

                /// True if out of range (overflowed or underflowed)
                pub open spec fn spec_is_out_of_range(&self) -> bool {
                    !self.spec_is_normal()
                }

                #[verifier::when_used_as_spec(spec_is_out_of_range)]
                pub exec fn is_out_of_range(&self) -> (result: bool)
                    ensures result == self.spec_is_out_of_range()
                {
                    proof { use_type_invariant(self); }
                    self.v.is_none()
                }

                /// Unwrap the value, requires it to be normal
                pub exec fn unwrap(&self) -> (result: $ity)
                    requires self.is_normal()
                    ensures result as int == self@
                {
                    proof { use_type_invariant(self); }
                    self.v.unwrap()
                }

                /// Convert to Option
                pub exec fn to_option(&self) -> (result: Option<$ity>)
                    ensures
                        result.is_some() == self.is_normal(),
                        result.is_some() ==> result.unwrap() as int == self@
                {
                    proof { use_type_invariant(self); }
                    self.v
                }

                /// Add a primitive value
                #[inline]
                #[verifier::external_body]
                pub exec fn add_value(&self, v2: $ity) -> (result: Self)
                    ensures result@ == self@ + v2 as int
                {
                    let new_i: Ghost<int> = Ghost(self@ + v2 as int);
                    match self.v {
                        Some(v1) => Self { i: new_i, v: v1.checked_add(v2) },
                        None => Self { i: new_i, v: None },
                    }
                }
                

                /// Add another checked integer
                #[inline]
                #[verifier::external_body]
                pub exec fn add_checked(&self, v2: &$cty) -> (result: Self)
                    ensures result@ == self@ + v2@
                {
                    let new_i: Ghost<int> = Ghost(self@ + v2@);
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                /// Subtract a primitive value
                #[inline]
                #[verifier::external_body]
                pub exec fn sub_value(&self, v2: $ity) -> (result: Self)
                    ensures result@ == self@ - v2 as int
                {
                    let new_i: Ghost<int> = Ghost(self@ - v2 as int);
                    match self.v {
                        Some(v1) => Self { i: new_i, v: v1.checked_sub(v2) },
                        None => Self { i: new_i, v: None },
                    }
                }

                /// Subtract another checked integer
                #[inline]
                #[verifier::external_body]
                pub exec fn sub_checked(&self, v2: &$cty) -> (result: Self)
                    ensures result@ == self@ - v2@
                {
                    let new_i: Ghost<int> = Ghost(self@ - v2@);
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_sub(*n2) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                /// Multiply by a primitive value
                #[inline]
                #[verifier::external_body]
                pub exec fn mul_value(&self, v2: $ity) -> (result: Self)
                    ensures result@ == self@ * v2 as int
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

                /// Multiply by another checked integer
                #[inline]
                #[verifier::external_body]
                pub exec fn mul_checked(&self, v2: &$cty) -> (result: Self)
                    ensures result@ == self@ * v2@
                {
                    let new_i: Ghost<int> = Ghost(self@ * v2@);
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                        (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                        (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                        _ => Self { i: new_i, v: None },
                    }
                }

                // Commutativity proofs for addition
                pub proof fn lemma_add_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ + b@ == b@ + a@
                {}

                pub proof fn lemma_add_commutative_normal(a: $cty, b: $cty)
                    requires ($min as int) <= a@ + b@ <= ($max as int)
                    ensures (a@ + b@) as $ity == (b@ + a@) as $ity
                {}

                // Commutativity proofs for multiplication
                pub proof fn lemma_mul_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ * b@ == b@ * a@
                {}

                pub proof fn lemma_mul_commutative_normal(a: $cty, b: $cty)
                    requires ($min as int) <= a@ * b@ <= ($max as int)
                    ensures (a@ * b@) as $ity == (b@ * a@) as $ity
                {}

                // Anti-commutativity for subtraction
                pub proof fn lemma_sub_anticommutative_ghost(a: $cty, b: $cty)
                    ensures a@ - b@ == -(b@ - a@)
                {}

                pub proof fn lemma_sub_anticommutative_normal(a: $cty, b: $cty)
                    requires ($min as int) <= a@ - b@ <= ($max as int)
                    ensures (a@ - b@) as $ity == (-(b@ - a@)) as $ity
                {}
            }
        }
    };
}

// Unsigned types
checked_int_gen!(u8, CheckedU8, 0u8, u8::MAX);
checked_int_gen!(u16, CheckedU16, 0u16, u16::MAX);
checked_int_gen!(u32, CheckedU32, 0u32, u32::MAX);
checked_int_gen!(u64, CheckedU64, 0u64, u64::MAX);
checked_int_gen!(u128, CheckedU128, 0u128, u128::MAX);
checked_int_gen!(usize, CheckedUsize, 0usize, usize::MAX);

// Signed types
checked_int_gen!(i8, CheckedI8, i8::MIN, i8::MAX);
checked_int_gen!(i16, CheckedI16, i16::MIN, i16::MAX);
checked_int_gen!(i32, CheckedI32, i32::MIN, i32::MAX);
checked_int_gen!(i64, CheckedI64, i64::MIN, i64::MAX);
checked_int_gen!(i128, CheckedI128, i128::MIN, i128::MAX);
checked_int_gen!(isize, CheckedIsize, isize::MIN, isize::MAX);

}

/// Non-Verus stub for cargo test
#[cfg(not(verus_keep_ghost))]
pub mod checked_int {

/// Macro to generate checked integer stubs for non-Verus builds
macro_rules! checked_int_stub {
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

// Unsigned types
checked_int_stub!(u8, CheckedU8, 0u8, u8::MAX);
checked_int_stub!(u16, CheckedU16, 0u16, u16::MAX);
checked_int_stub!(u32, CheckedU32, 0u32, u32::MAX);
checked_int_stub!(u64, CheckedU64, 0u64, u64::MAX);
checked_int_stub!(u128, CheckedU128, 0u128, u128::MAX);
checked_int_stub!(usize, CheckedUsize, 0usize, usize::MAX);

// Signed types
checked_int_stub!(i8, CheckedI8, i8::MIN, i8::MAX);
checked_int_stub!(i16, CheckedI16, i16::MIN, i16::MAX);
checked_int_stub!(i32, CheckedI32, i32::MIN, i32::MAX);
checked_int_stub!(i64, CheckedI64, i64::MIN, i64::MAX);
checked_int_stub!(i128, CheckedI128, i128::MIN, i128::MAX);
checked_int_stub!(isize, CheckedIsize, isize::MIN, isize::MAX);

}
