//! Traits for executable set types that mirror vstd::set::Set<A> API

pub mod SetTrait {
    use vstd::prelude::*;

    verus! {

    /// Basic set operations trait without View dependency.
    /// Types implementing this trait should have a spec view function that returns Set<T::V>
    /// where T is the element type with View.
    pub trait SetTrait<T: View>: Sized {
        /// Specification view function - maps this executable set to a spec-level Set
        spec fn view(&self) -> Set<<T as View>::V>;
    }

    /// Extended trait that combines SetTrait with View as a supertrait.
    /// This allows using self@ syntax in code that uses this trait.
    pub trait SetTraitWithView<T: View>: SetTrait<T> + View<V = Set<<T as View>::V>> {
        /// Create an empty set
        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty();

        /// Check if element is in the set
        fn contains(&self, x: &T) -> (result: bool)
            ensures result == self@.contains(x@);

        /// Insert an element into the set
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@);

        /// Remove an element from the set
        fn remove(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@);

        /// Union of two sets
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@);

        /// Intersection of two sets
        fn intersect(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@);

        /// Set difference
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@);

        /// Size of the set (requires finite set)
        fn len(&self) -> (result: usize)
            requires self@.finite(),
            ensures result == self@.len();

        /// Check if set is empty
        fn is_empty(&self) -> (result: bool)
            ensures result <==> self@ == Set::<<T as View>::V>::empty();
    }

    } // verus!
}

