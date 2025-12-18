//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Concurrency types and traits for multi-threaded code.
//! Based on Verus counting-to-2 example: https://verus-lang.github.io/verus/state_machines/examples/counting-to-2.html

use vstd::prelude::*;
use std::sync::Mutex;
use crate::Types::Types::{StT, Pair, B};

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
    use crate::Types::Types::{StT, Pair, B};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

    verus! {

    // StTInMtT: St-friendly elements that can be shared across threads (StT + Send + Sync)
    pub trait StTInMtT: StT + Send + Sync {}

    // MtT: multi-threaded friendly elements; minimal so it can include Mutex<..>
    // Keep only thread-safety and size requirements.
    //
    // NOTE: clone_mt() has no Verus specification. For verified code, use clone_plus()
    // from ClonePlus trait instead, which has `ensures cloned(*self, res)` postcondition.
    // All types implementing MtT also implement Clone, so clone_plus() is always available.
    pub trait MtT: Sized + Send + Sync {
        type Inner: StT;
        /// Deprecated for verified code - use clone_plus() which has specification.
        fn clone_mt(&self) -> Self;
        fn new_mt(inner: Self::Inner) -> Self;
    }

    // MtKey: Multi-threaded key type with ordering and static lifetime
    pub trait MtKey: StTInMtT + Ord + 'static {}

    // MtVal: Multi-threaded value type with static lifetime
    pub trait MtVal: StTInMtT + 'static {}

    // MtFn: Multi-threaded function type with common bounds
    pub trait MtFn<Args, Output>: Fn(Args) -> Output + Send + Sync + 'static {}

    // MtFnClone: Multi-threaded function type with Clone
    pub trait MtFnClone<Args, Output>: Fn(Args) -> Output + Send + Sync + Clone + 'static {}

    // MtReduceFn: Multi-threaded reducer function type
    pub trait MtReduceFn<V>: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}

    // PredMt: Multi-threaded predicate function (boolean function)
    pub trait PredMt<T>: Fn(&T) -> B + Send + Sync + 'static {}

    // PredVal: Multi-threaded predicate function taking values by value
    pub trait PredVal<T>: Fn(T) -> B + Send + Sync + 'static {}

    // Backward compatibility alias
    pub use PredMt as Pred;

    // Blanket impls
    impl<T> StTInMtT for T where T: StT + Send + Sync {}
    impl<T> MtKey for T where T: StTInMtT + Ord + 'static {}
    impl<T> MtVal for T where T: StTInMtT + 'static {}
    impl<T, Args, Output> MtFn<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + 'static {}
    impl<T, Args, Output> MtFnClone<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + 'static + Clone {}
    impl<T, V> MtReduceFn<V> for T where T: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}
    impl<F, T> PredMt<T> for F where F: Fn(&T) -> B + Send + Sync + 'static {}
    impl<F, T> PredVal<T> for F where F: Fn(T) -> B + Send + Sync + 'static {}

    // MtT for Pair when components are Send+Sync+Clone
    impl<A: StT + Send + Sync + Clone, B: StT + Send + Sync + Clone> MtT for Pair<A, B> {
        type Inner = Pair<A, B>;
        fn clone_mt(&self) -> Self { Pair(self.0.clone_plus(), self.1.clone_plus()) }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // MtT implementations for primitive types
    impl MtT for usize {
        type Inner = usize;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for isize {
        type Inner = isize;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for i32 {
        type Inner = i32;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for u32 {
        type Inner = u32;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for i64 {
        type Inner = i64;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for u64 {
        type Inner = u64;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for bool {
        type Inner = bool;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for char {
        type Inner = char;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for String {
        type Inner = String;
        fn clone_mt(&self) -> Self { self.clone_plus() }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl<'a> MtT for &'a str {
        type Inner = &'a str;
        fn clone_mt(&self) -> Self { self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    } // verus!

    // MtT implementations for Mutex - outside verus! because Mutex is not specified
    impl<T: StT + Send> MtT for Mutex<T> {
        type Inner = T;
        fn clone_mt(&self) -> Self {
            let inner = self.lock().unwrap().clone();
            Mutex::new(inner)
        }
        fn new_mt(inner: Self::Inner) -> Self { Mutex::new(inner) }
    }
}
