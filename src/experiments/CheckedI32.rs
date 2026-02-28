// Copyright (c) 2025 Brian G. Milnes
//! Hypothesis: Can Verus verify a checked integer tracking true mathematical value?
#[allow(unused_imports)]
use vstd::prelude::*;
#[allow(unused_imports)]
use vstd::view::View;
#[allow(unused_imports)]
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::mul::{lemma_mul_by_zero_is_zero, lemma_mul_inequality, lemma_mul_is_commutative};

verus! {

    /// A checked integer that tracks the true mathematical value.
    /// - `i`: Ghost value representing the true unbounded integer
    /// - `v`: Some(x) if x is in range, None if overflowed or underflowed
    pub struct CheckedI32 {
        pub i: Ghost<int>,
        pub v: Option<i32>,
    }

    impl View for CheckedI32 {
        type V = int;

        open spec fn view(&self) -> int {
            self.i@
        }
    }

    impl Clone for CheckedI32 {
        exec fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            proof { use_type_invariant(self); }
            Self { i: self.i, v: self.v }
        }
    }

    impl CheckedI32 {
        /// Type invariant: v matches i when in range, v is None when out of range
        #[verifier::type_invariant]
        spec fn well_formed(self) -> bool {
            match self.v {
                Some(v) => self.i@ == v as int,
                None => self.i@ < (i32::MIN as int) || self.i@ > (i32::MAX as int),
            }
        }

        /// Spec: minimum value for this type
        pub open spec fn spec_min() -> int { i32::MIN as int }

        /// Spec: maximum value for this type
        pub open spec fn spec_max() -> int { i32::MAX as int }

        /// Create a new checked integer from a primitive value
        pub open spec fn spec_new(v: i32) -> CheckedI32 {
           CheckedI32 { i: Ghost(v as int), v: Some(v) }
        }

        #[verifier::when_used_as_spec(spec_new)]
        pub exec fn new(v: i32) -> (result: Self)
            ensures result@ == v as int
        {
            Self { i: Ghost(v as int), v: Some(v) }
        }

        /// Create a checked integer that is out of range (overflow or underflow)
        pub exec fn new_out_of_range(Ghost(i): Ghost<int>) -> (result: Self)
            requires i < (i32::MIN as int) || i > (i32::MAX as int)
            ensures result@ == i
        {
            Self { i: Ghost(i), v: None }
        }

        /// True if the value is in the normal range (not overflowed or underflowed)
        pub open spec fn spec_is_normal(&self) -> bool {
            (i32::MIN as int) <= self@ <= (i32::MAX as int)
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
            self@ > (i32::MAX as int)
        }

        /// True if the value underflowed (went below min)
        pub open spec fn spec_is_underflowed(&self) -> bool {
            self@ < (i32::MIN as int)
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
        pub exec fn unwrap(&self) -> (result: i32)
            requires self.is_normal()
            ensures result as int == self@
        {
            proof { use_type_invariant(self); }
            self.v.unwrap()
        }

        /// Convert to Option
        pub exec fn to_option(&self) -> (result: Option<i32>)
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
        pub exec fn add_value(&self, v2: i32) -> (result: Self)
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
        pub exec fn add_checked(&self, v2: &CheckedI32) -> (result: Self)
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
        pub exec fn sub_value(&self, v2: i32) -> (result: Self)
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
        pub exec fn sub_checked(&self, v2: &CheckedI32) -> (result: Self)
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
        pub exec fn mul_value(&self, v2: i32) -> (result: Self)
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
        pub exec fn mul_checked(&self, v2: &CheckedI32) -> (result: Self)
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
    }
}
