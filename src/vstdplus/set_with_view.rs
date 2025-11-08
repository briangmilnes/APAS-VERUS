//! SetWithView trait - standalone trait with View and specifications

pub mod SetWithView {
    use vstd::prelude::*;

    verus! {

    /// Standalone set trait with View and specifications.
    /// Does NOT extend Set - this is for verified code only.
    pub trait SetWithView<T: View>: Sized + View<V = vstd::set::Set<<T as View>::V>> {
        fn empty() -> (result: Self)
            requires vstd::std_specs::hash::obeys_key_model::<T>()
            ensures result@ == vstd::set::Set::<<T as View>::V>::empty();

        fn contains(&self, x: &T) -> (result: bool)
            ensures result == self@.contains(x@);

        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@);

        fn remove(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@);

        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@);

        fn intersect(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@);

        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@);

        fn len(&self) -> (result: usize)
            ensures result == self@.len();

        fn is_empty(&self) -> (result: bool)
            ensures result <==> self@ == vstd::set::Set::<<T as View>::V>::empty();
    }

    } // verus!
}
