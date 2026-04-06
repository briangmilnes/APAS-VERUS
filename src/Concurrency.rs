//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Concurrency types and traits for multi-threaded code.
//! Based on Verus counting-to-2 example: https://verus-lang.github.io/verus/state_machines/examples/counting-to-2.html


//  Table of Contents
//	Section 2. imports
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 2. imports

use vstd::prelude::*;
use crate::Types::Types::*;

verus! 
{

    //		Section 9. impls


/// Diverges. Use with `assume(false)` or `crate::vstdplus::accept::accept(false)` in
/// unreachable error branches (e.g. `JoinHandle::join()` Err arm). The `accept` variant
/// is preferred for intentional holes — proof hole checkers treat it as an accepted idiom.
#[verifier::external_body] // accept hole
#[verifier::exec_allows_no_decreases_clause]
pub fn diverge<A>() -> A {
    loop { }
}

} // verus!

    //		Section 14. derive impls outside verus!

pub mod Concurrency {
    use vstd::prelude::*;
    use crate::Types::Types::*;

    verus! {

    /// Single-threaded friendly elements that can be shared across threads (StT + Send + Sync).
    pub trait StTInMtT: StT + Send + Sync + 'static {}

    /// Multi-threaded friendly elements: Sized + Send + Sync.
    pub trait MtT: Sized + Send + Sync {}

    /// Multi-threaded key type with ordering and static lifetime.
    pub trait MtKey: StTInMtT + Ord + 'static {}

    /// Multi-threaded value type with static lifetime.
    pub trait MtVal: StTInMtT + 'static {}

    /// Multi-threaded reducer function type: Fn(&V, &V) -> V + Clone + Send + Sync + 'static.
    pub trait MtReduceFn<V>: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}

    /// Predicate function (sequential): Fn(&T) -> bool + Send + Sync + 'static.
    /// For D&C filter via join(), use MtPred<T> which adds Clone.
    pub trait Pred<T>: Fn(&T) -> bool + Send + Sync + 'static {}

    /// Multi-threaded predicate: Fn(&T) -> bool + Clone + Send + Sync + 'static.
    /// Use for D&C filter where the predicate must be cloned into both join arms.
    pub trait MtPred<T>: Fn(&T) -> bool + Clone + Send + Sync + 'static {}

    /// Multi-threaded map function: Fn(&T) -> U + Clone + Send + Sync + 'static.
    pub trait MtMapFn<T, U>: Fn(&T) -> U + Clone + Send + Sync + 'static {}

    /// Multi-threaded tabulate function: Fn(usize) -> T + Clone + Send + Sync + 'static.
    pub trait MtTabulateFn<T>: Fn(usize) -> T + Clone + Send + Sync + 'static {}

    // Blanket implementations
    impl<T> StTInMtT for T where T: StT + Send + Sync + 'static {}
    impl<T> MtT for T where T: Sized + Send + Sync {}
    impl<T> MtKey for T where T: StTInMtT + Ord + 'static {}
    impl<T> MtVal for T where T: StTInMtT + 'static {}
    impl<T, V> MtReduceFn<V> for T where T: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}
    impl<F, T> Pred<T> for F where F: Fn(&T) -> bool + Send + Sync + 'static {}
    impl<F, T> MtPred<T> for F where F: Fn(&T) -> bool + Clone + Send + Sync + 'static {}
    impl<F, T, U> MtMapFn<T, U> for F where F: Fn(&T) -> U + Clone + Send + Sync + 'static {}
    impl<F, T> MtTabulateFn<T> for F where F: Fn(usize) -> T + Clone + Send + Sync + 'static {}

    } // verus!
}
