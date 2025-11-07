//! Trait for executable set types that mirror vstd::set::Set<A> API

pub mod SetTrait {
    use vstd::prelude::*;

    verus! {

    /// Trait for executable set types that provide the same operations as vstd::set::Set<A>
    /// at the specification level.
    ///
    /// Types implementing this trait should have a spec view function that returns Set<T::V>
    /// where T is the element type with View.
    pub trait SetTrait<T: View>: Sized {
        /// Create an empty set
        fn empty() -> Self
            ensures Self::view(&result) == Set::<<T as View>::V>::empty();

        /// Check if element is in the set
        fn contains(&self, x: &T) -> (result: bool)
            ensures result == Self::view(self).contains(x@);

        /// Insert an element into the set
        fn insert(&mut self, x: T)
            ensures Self::view(self) == old(Self::view(self)).insert(x@);

        /// Remove an element from the set
        fn remove(&mut self, x: &T)
            ensures Self::view(self) == old(Self::view(self)).remove(x@);

        /// Union of two sets
        fn union(&self, other: &Self) -> Self
            ensures Self::view(&result) == Self::view(self).union(Self::view(other));

        /// Intersection of two sets
        fn intersect(&self, other: &Self) -> Self
            ensures Self::view(&result) == Self::view(self).intersect(Self::view(other));

        /// Set difference
        fn difference(&self, other: &Self) -> Self
            ensures Self::view(&result) == Self::view(self).difference(Self::view(other));

        /// Size of the set (requires finite set)
        fn len(&self) -> (result: usize)
            requires Self::view(self).finite(),
            ensures result == Self::view(self).len();

        /// Check if set is empty
        fn is_empty(&self) -> (result: bool)
            ensures result <==> Self::view(self) == Set::<<T as View>::V>::empty();

        /// Specification view function - maps this executable set to a spec-level Set
        spec fn view(&self) -> Set<<T as View>::V>;
    }

    } // verus!
}

