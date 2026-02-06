//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Concurrency types and traits for multi-threaded code.
//! Based on Verus counting-to-2 example: https://verus-lang.github.io/verus/state_machines/examples/counting-to-2.html

use vstd::prelude::*;
use std::sync::Mutex;
use crate::Types::Types::*;

verus! {

/// Diverges. Use with `assume(false)` in unreachable error branches.
#[verifier::exec_allows_no_decreases_clause]
pub fn diverge<A>() -> A {
    loop { }
}

} // verus!

pub mod Concurrency {
    use vstd::prelude::*;
    use std::sync::Mutex;
    use crate::Types::Types::*;

    verus! {

    /// Single-threaded friendly elements that can be shared across threads (StT + Send + Sync).
    pub trait StTInMtT: StT + Send + Sync {}

    /// Multi-threaded friendly elements: Sized + Send + Sync.
    pub trait MtT: Sized + Send + Sync {}

    /// Multi-threaded key type with ordering and static lifetime.
    pub trait MtKey: StTInMtT + Ord + 'static {}

    /// Multi-threaded value type with static lifetime.
    pub trait MtVal: StTInMtT + 'static {}

    /// Multi-threaded function type with common bounds.
    pub trait MtFn<Args, Output>: Fn(Args) -> Output + Send + Sync + 'static {}

    /// Multi-threaded function type with Clone.
    pub trait MtFnClone<Args, Output>: Fn(Args) -> Output + Send + Sync + Clone + 'static {}

    /// Multi-threaded reducer function type.
    pub trait MtReduceFn<V>: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}

    /// Multi-threaded predicate function (boolean function).
    pub trait PredMt<T>: Fn(&T) -> B + Send + Sync + 'static {}

    /// Multi-threaded predicate function taking values by value.
    pub trait PredVal<T>: Fn(T) -> B + Send + Sync + 'static {}

    /// Backward compatibility alias.
    pub use PredMt as Pred;

    // Blanket implementations
    impl<T> StTInMtT for T where T: StT + Send + Sync {}
    impl<T> MtT for T where T: Sized + Send + Sync {}
    impl<T> MtKey for T where T: StTInMtT + Ord + 'static {}
    impl<T> MtVal for T where T: StTInMtT + 'static {}
    impl<T, Args, Output> MtFn<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + 'static {}
    impl<T, Args, Output> MtFnClone<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + 'static + Clone {}
    impl<T, V> MtReduceFn<V> for T where T: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}
    impl<F, T> PredMt<T> for F where F: Fn(&T) -> B + Send + Sync + 'static {}
    impl<F, T> PredVal<T> for F where F: Fn(T) -> B + Send + Sync + 'static {}

    } // verus!
}
