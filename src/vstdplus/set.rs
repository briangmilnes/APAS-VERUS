//! Set trait - pure operations with no specifications

pub mod Set {
    use vstd::prelude::*;

    verus! {

    /// Basic set operations with no specifications.
    pub trait Set<T>: Sized {
        fn empty() -> Self;
        fn contains(&self, x: &T) -> bool;
        fn insert(&mut self, x: T);
        fn remove(&mut self, x: &T);
        fn union(&self, other: &Self) -> Self;
        fn intersect(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
    }

    } // verus!
}
