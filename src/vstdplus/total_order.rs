// Copyright (c) 2025 Brian G. Milnes
//! REVIEWED: NO
//! TotalOrder trait connecting executable comparison to spec-level ordering.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod total_order {

    //		Section 2. imports

    use core::cmp::Ordering;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialOrdIs;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;

    verus! 
{

    //		Section 8. traits


 pub trait TotalOrder: Sized {
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

    // Default bodies use assume — types with OrdSpecImpl (primitives) override
    // with empty bodies where Z3 proves it. User types without OrdSpecImpl
    // (blocked on Verus vir/ast_util.rs:734 panic) get the default.
    proof fn cmp_spec_less_implies_le(a: Self, b: Self) where Self: Ord
        requires a.cmp_spec(&b) == Ordering::Less
        ensures TotalOrder::le(a, b)
    { assume(TotalOrder::le(a, b)); }

    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) where Self: Ord
        requires a.cmp_spec(&b) == Ordering::Greater
        ensures TotalOrder::le(b, a)
    { assume(TotalOrder::le(b, a)); }

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

    proof fn is_lt_irreflexive(a: Self)
        ensures !a.is_lt(&a);

    proof fn is_lt_antisymmetric(a: Self, b: Self)
        requires !a.is_lt(&b), !b.is_lt(&a),
        ensures a == b;
}

    //		Section 9. impls


// Note: A blanket impl for all T: Ord would be nice, but Verus has limitations
// with generic comparison operators in exec code. Individual impls work fine.

impl TotalOrder for u8 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for u16 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for u32 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for u64 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for u128 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for usize {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for i8 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for i16 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for i32 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl TotalOrder for i64 {
    open spec fn le(self, other: Self) -> bool {
        self <= other
    }

    proof fn reflexive(x: Self) {
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
    }

    proof fn antisymmetric(x: Self, y: Self) {
    }

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

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

    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {}
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {}
}

impl IsLtTransitive for u8 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for u16 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for u32 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for u64 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for u128 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for usize {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for i8 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for i16 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for i32 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for i64 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for i128 {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}
impl IsLtTransitive for isize {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {}
    proof fn is_lt_irreflexive(a: Self) {}
    proof fn is_lt_antisymmetric(a: Self, b: Self) {}
}

// String: partial_cmp_spec is opaque, so ordering axioms must be assumed.
// accept hole
impl IsLtTransitive for String {
    proof fn is_lt_transitive(a: Self, b: Self, c: Self) {
        assume(a.is_lt(&c));
    }
    // accept hole
    proof fn is_lt_irreflexive(a: Self) {
        assume(!a.is_lt(&a));
    }
    // accept hole
    proof fn is_lt_antisymmetric(a: Self, b: Self) {
        assume(a == b);
    }
}

// String: comparison specs are opaque in vstd, so TotalOrder proof methods
// must assume their conclusions. The cmp body delegates to Ord::cmp.
// accept hole
impl TotalOrder for String {
    // Delegates to PartialOrd::is_le; opaque because vstd has no String ordering spec.
    open spec fn le(self, other: Self) -> bool {
        self.is_le(&other)
    }

    proof fn reflexive(x: Self) {
        assume(TotalOrder::le(x, x));
    }

    proof fn transitive(x: Self, y: Self, z: Self) {
        assume(TotalOrder::le(x, z));
    }

    proof fn antisymmetric(x: Self, y: Self) {
        assume(x == y);
    }

    proof fn total(x: Self, y: Self) {
        assume(TotalOrder::le(x, y) || TotalOrder::le(y, x));
    }

    fn cmp(&self, other: &Self) -> (c: Ordering) {
        if *self < *other {
            proof { assume(TotalOrder::le(*self, *other) && self != other); }
            Ordering::Less
        } else if *self == *other {
            proof { assume(*self == *other); }
            Ordering::Equal
        } else {
            proof { assume(TotalOrder::le(*other, *self) && self != other); }
            Ordering::Greater
        }
    }

    // accept hole
    proof fn cmp_spec_less_implies_le(a: Self, b: Self) {
        assume(TotalOrder::le(a, b));
    }
    // accept hole
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self) {
        assume(TotalOrder::le(b, a));
    }
}

// BYPASSED: TotalOrderBridge merged into TotalOrder — bridge lemmas now live
// directly in the TotalOrder trait with Ord as a supertrait.
// pub trait TotalOrderBridge: TotalOrder + Ord { ... }
// All 14 impls (u8..isize, String) removed — proof bodies identical to TotalOrder impls above.

} // verus!
}
