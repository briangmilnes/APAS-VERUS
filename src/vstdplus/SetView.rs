//! SetView trait - abstraction for types whose View is a Set
//!
//! This trait provides a common interface for executable types (like HashSetWithView, SetStEph)
//! that have a view type of Set<T::V>. This allows trait methods to use self@.len(),
//! self@.contains(), etc. without the compiler losing track of what the view type is.
//!
//! This is a gap in vstd that we're filling - there's no trait abstraction for "things with a Set view".

pub mod SetView {
    use vstd::prelude::*;

    verus! {

    /// Trait for types whose View is a Set
    /// This allows trait methods to use `self@.len()`, `self@.contains()`, etc.
    pub trait SetView<T: View>: Sized + View<V = Set<T::V>> {
        /// Create an empty set
        fn empty() -> (result: Self)
            ensures
                result@ == Set::<T::V>::empty();

        /// Get the size of the set (returns nat in spec, usize in exec)
        fn size(&self) -> (result: usize)
            ensures
                result == self@.len();

        /// Check if element is in the set
        fn mem(&self, x: &T) -> (result: bool)
            ensures
                result == self@.contains(x@);

        /// Insert an element
        fn insert(&mut self, x: T)
            ensures
                self@ == old(self)@.insert(x@);

        /// Remove an element
        fn remove(&mut self, x: T)
            ensures
                self@ == old(self)@.remove(x@);

        /// Union of two sets
        fn union(&self, other: &Self) -> (result: Self)
            ensures
                result@ == self@.union(other@);

        /// Intersection of two sets
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures
                result@ == self@.intersect(other@);
    }

    } // verus!
}

