// Copyright (c) 2025 Brian G. Milnes
//! TotalOrder trait connecting executable comparison to spec-level ordering.
pub mod total_order {
    use core::cmp::Ordering;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdIs;

    verus! {

 pub trait TotalOrder: Sized {
    spec fn le(self, other: Self) -> bool;

// Veracity: USED
    proof fn reflexive(x: Self)
        ensures
            Self::le(x, x),
    ;

// Veracity: USED
    proof fn transitive(x: Self, y: Self, z: Self)
        requires
            Self::le(x, y),
            Self::le(y, z),
        ensures
            Self::le(x, z),
    ;

// Veracity: USED
    proof fn antisymmetric(x: Self, y: Self)
        requires
            Self::le(x, y),
            Self::le(y, x),
        ensures
            x == y,
    ;

// Veracity: USED
    proof fn total(x: Self, y: Self)
        ensures
            Self::le(x, y) || Self::le(y, x),
    ;

    fn cmp(&self, other: &Self) -> (c: Ordering)
        ensures
            (match c {
                Ordering::Less => self.le(*other) && self != other,
                Ordering::Equal => self == other,
                Ordering::Greater => other.le(*self) && self != other,
            }),
    ;
}

// Note: A blanket impl for all T: Ord would be nice, but Verus has limitations
// with generic comparison operators in exec code. Individual impls work fine.

impl TotalOrder for u8 {
    open spec fn le(self, other: Self) -> bool {
// Veracity: USED
        self <= other
    }

    proof fn reflexive(x: Self) {
// Veracity: USED
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }
// Veracity: USED

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
// Veracity: USED

impl TotalOrder for u16 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }
// Veracity: USED

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
// Veracity: USED
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
// Veracity: USED
        }
    }
}

impl TotalOrder for u32 {
    open spec fn le(self, other: Self) -> bool {
// Veracity: USED
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

// Veracity: USED
    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
// Veracity: USED
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

// Veracity: USED
impl TotalOrder for u64 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }
// Veracity: USED

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
// Veracity: USED
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
// Veracity: USED
}

impl TotalOrder for u128 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
// Veracity: USED
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
// Veracity: USED
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
// Veracity: USED
        }
    }
}

impl TotalOrder for usize {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

// Veracity: USED
    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }
// Veracity: USED

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
// Veracity: USED
        } else {
            Ordering::Greater
        }
    }
}

impl TotalOrder for i8 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }
// Veracity: USED

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }
// Veracity: USED

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
// Veracity: USED
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl TotalOrder for i16 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
// Veracity: USED
    }

    proof fn reflexive(x: Self) {
    }
// Veracity: USED

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
// Veracity: USED
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl TotalOrder for i32 {
    open spec fn le(self, other: Self) -> bool {
// Veracity: USED
        self <= other
    }
// Veracity: USED

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }
// Veracity: USED

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl TotalOrder for i64 {
// Veracity: USED
// Veracity: USED
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
// Veracity: USED

// Veracity: USED
impl TotalOrder for i128 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }
// Veracity: USED

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
// Veracity: USED

impl TotalOrder for isize {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

// Veracity: USED
    proof fn total(x: Self, y: Self) {
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

/// Axiomatizes strict-less-than transitivity for vstd's `is_lt` spec fn.
///
/// For integer types, `is_lt` reduces to `<` and the solver proves transitivity
/// automatically (empty proof body). For generic code over ordered types, add
/// this as a trait bound to access the `is_lt_transitive` proof fn.
pub trait IsLtTransitive: PartialOrd + Sized {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self)
        requires a.is_lt(&b), b.is_lt(&c),
        ensures a.is_lt(&c);
}

impl IsLtTransitive for u8   { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for u16  { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for u32  { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for u64  { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for u128 { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for usize { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for i8   { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for i16  { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for i32  { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for i64  { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for i128 { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }
impl IsLtTransitive for isize { proof fn is_lt_transitive(a: Self, b: Self, c: Self) {} }

// String: partial_cmp_spec is opaque, so transitivity of is_lt must be assumed.
// accept hole
impl IsLtTransitive for String {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {
        assume(a.is_lt(&c));
    }
}

} // verus!
}
