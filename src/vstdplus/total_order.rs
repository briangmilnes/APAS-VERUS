//! TotalOrder trait connecting executable comparison to spec-level ordering.
pub mod total_order {
    use core::cmp::Ordering;
    use vstd::prelude::*;

    verus! {

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
}

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
}

} // verus!
}
