//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Concurrency types and traits for multi-threaded code.
//! Based on Verus counting-to-2 example: https://verus-lang.github.io/verus/state_machines/examples/counting-to-2.html

use vstd::prelude::*;

verus! {

/// Diverges. Use with `assume(false)` in unreachable error branches.
#[verifier::exec_allows_no_decreases_clause]
pub fn diverge<A>() -> A {
    loop { }
}

} // verus!

pub mod Concurrency {
    use std::sync::Mutex;
    use crate::Types::Types::{StT, Pair, B};

    // StTInMtT: St-friendly elements that can be shared across threads (StT + Send + Sync)
    pub trait StTInMtT: StT + Send + Sync {}
    impl<T> StTInMtT for T where T: StT + Send + Sync {}

    // MtT: multi-threaded friendly elements; minimal so it can include Mutex<..>
    // Keep only thread-safety and size requirements.
    pub trait MtT: Sized + Send + Sync {
        type Inner: StT;
        fn clone_mt(&self) -> Self;
        fn new_mt(inner: Self::Inner) -> Self;
    }

    // MtKey: Multi-threaded key type with ordering and static lifetime
    pub trait MtKey: StTInMtT + Ord + 'static {}
    impl<T> MtKey for T where T: StTInMtT + Ord + 'static {}

    // MtVal: Multi-threaded value type with static lifetime
    pub trait MtVal: StTInMtT + 'static {}
    impl<T> MtVal for T where T: StTInMtT + 'static {}

    // MtFn: Multi-threaded function type with common bounds
    pub trait MtFn<Args, Output>: Fn(Args) -> Output + Send + Sync + 'static {}
    impl<T, Args, Output> MtFn<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + 'static {}

    // MtFnClone: Multi-threaded function type with Clone
    pub trait MtFnClone<Args, Output>: Fn(Args) -> Output + Send + Sync + Clone + 'static {}
    impl<T, Args, Output> MtFnClone<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + Clone + 'static {}

    // MtReduceFn: Multi-threaded reducer function type
    pub trait MtReduceFn<V>: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}
    impl<T, V> MtReduceFn<V> for T where T: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}

    // PredMt: Multi-threaded predicate function (boolean function)
    pub trait PredMt<T>: Fn(&T) -> B + Send + Sync + 'static {}
    impl<F, T> PredMt<T> for F where F: Fn(&T) -> B + Send + Sync + 'static {}

    // PredVal: Multi-threaded predicate function taking values by value
    pub trait PredVal<T>: Fn(T) -> B + Send + Sync + 'static {}
    impl<F, T> PredVal<T> for F where F: Fn(T) -> B + Send + Sync + 'static {}

    // Backward compatibility alias
    pub use PredMt as Pred;

    // MtT implementations for Mutex
    impl<T: StT + Send> MtT for Mutex<T> {
        type Inner = T;
        fn clone_mt(&self) -> Self {
            let inner = self.lock().unwrap().clone();
            Mutex::new(inner)
        }
        fn new_mt(inner: Self::Inner) -> Self { Mutex::new(inner) }
    }

    // MtT for Pair when components are Send+Sync
    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {
        type Inner = Pair<A, B>;
        fn clone_mt(&self) -> Self { self.clone() }
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
        fn clone_mt(&self) -> Self { self.clone() }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl<'a> MtT for &'a str {
        type Inner = &'a str;
        fn clone_mt(&self) -> Self { self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

}

