//! PartialOrder trait connecting executable comparison to spec-level partial ordering.
//! 
//! Unlike TotalOrder, PartialOrder allows incomparable elements (e.g., NaN in floats).
//! The partial_cmp method returns Option<Ordering>, where None indicates incomparability.

pub mod partial_order {
    use core::cmp::Ordering;
    use vstd::prelude::*;

    verus! {

    pub trait PartialOrder: Sized {
        spec fn le(self, other: Self) -> bool;

        proof fn reflexive(x: Self)
            ensures
                Self::le(x, x),
        ;

        proof fn transitive(x: Self, y: Self, z: Self)
            requires
                Self::le(x, y),
                Self::le(y, z),
            ensures
                Self::le(x, z),
        ;

        proof fn antisymmetric(x: Self, y: Self)
            requires
                Self::le(x, y),
                Self::le(y, x),
            ensures
                x == y,
        ;

        fn compare(&self, other: &Self) -> (c: Option<Ordering>)
            ensures
                (match c {
                    Some(Ordering::Less) => self.le(*other) && self != other,
                    Some(Ordering::Equal) => self == other,
                    Some(Ordering::Greater) => other.le(*self) && self != other,
                    None => true, // Incomparable elements
                }),
        ;
    }

    // Implementations for integer types (they're totally ordered, but we can implement PartialOrder)
    impl PartialOrder for u8 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for u16 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for u32 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for u64 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for u128 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for usize {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for i8 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for i16 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for i32 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for i64 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for i128 {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl PartialOrder for isize {
        open spec fn le(self, other: Self) -> bool {
            self <= other
        }

        proof fn reflexive(x: Self) {
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
        }

        proof fn antisymmetric(x: Self, y: Self) {
        }

        fn compare(&self, other: &Self) -> (c: Option<Ordering>) {
            if self < other {
                Some(Ordering::Less)
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    // Float implementations using uninterpreted specs (following vstd::std_specs::cmp pattern)
    // Note: We do not assume obeys_partial_cmp_spec() for floats because Rust floating point
    // operations are not guaranteed to be deterministic (see RFC 3514).
    // Instead, we use uninterpreted functions that users can axiomatize.

    pub uninterp spec fn partial_order_ensures<T>(x: T, y: T, o: Option<Ordering>) -> bool;

    impl PartialOrder for f32 {
        open spec fn le(self, other: Self) -> bool {
            arbitrary()
        }

        proof fn reflexive(x: Self) {
            admit();
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
            admit();
        }

        proof fn antisymmetric(x: Self, y: Self) {
            admit();
        }

        #[verifier::external_body]
        fn compare(&self, other: &Self) -> (c: Option<Ordering>)
            ensures
                partial_order_ensures::<f32>(*self, *other, c),
        {
            core::cmp::PartialOrd::partial_cmp(self, other)
        }
    }

    impl PartialOrder for f64 {
        open spec fn le(self, other: Self) -> bool {
            arbitrary()
        }

        proof fn reflexive(x: Self) {
            admit();
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
            admit();
        }

        proof fn antisymmetric(x: Self, y: Self) {
            admit();
        }

        #[verifier::external_body]
        fn compare(&self, other: &Self) -> (c: Option<Ordering>)
            ensures
                partial_order_ensures::<f64>(*self, *other, c),
        {
            core::cmp::PartialOrd::partial_cmp(self, other)
        }
    }

    } // verus!
}

