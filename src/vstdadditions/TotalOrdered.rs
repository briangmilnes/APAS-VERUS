//! TotalOrdered trait from Verus guide
//! https://verus-lang.github.io/verus/guide/container_bst_generic.html#defining-a-total-order
//!
//! This trait connects executable comparison functions to spec-level total ordering.
//! Surprisingly, this is NOT in vstd - it's only in the tutorial examples.
pub mod TotalOrdered {
    use vstd::prelude::*;

    verus! {

pub enum Cmp {
    Less,
    Equal,
    Greater,
}

 pub trait TotalOrdered: Sized {
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

     fn compare(&self, other: &Self) -> (c: Cmp)
         ensures
             (match c {
                 Cmp::Less => self.le(*other) && self != other,
                 Cmp::Equal => self == other,
                 Cmp::Greater => other.le(*self) && self != other,
             }),
     ;
 }

 impl TotalOrdered for u8 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for u16 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for u32 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for u64 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for u128 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for usize {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for i8 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for i16 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for i32 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for i64 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for i128 {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

 impl TotalOrdered for isize {
     open spec fn le(self, other: Self) -> bool {
         self <= other
     }

     proof fn reflexive(x: Self) {}

     proof fn transitive(x: Self, y: Self, z: Self) {}

     proof fn antisymmetric(x: Self, y: Self) {}

     proof fn total(x: Self, y: Self) {}

     fn compare(&self, other: &Self) -> (c: Cmp) {
         if self < other {
             Cmp::Less
         } else if self == other {
             Cmp::Equal
         } else {
             Cmp::Greater
         }
     }
 }

} // verus!
}
