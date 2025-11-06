//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Common types used across the crate.
//!

pub mod Types {

    use std::fmt::{Formatter, Debug, Display};
    use std::hash::Hash;
    use std::ops::Add;
    use std::sync::Mutex;

    pub type N = usize;

    /// Data Type 18.1 (Boolean) type used by APAS.
    /// Using Rust's built-in bool with normal true/false literals
    pub type B = bool;

    /// Data Type 18.1 (Ordering) relationships used by APAS, using Rust's as it matches.
    /// Enumerated values in `std::cmp::Ordering` are named: Less, Equal, Greater.
    pub use std::cmp::Ordering as O;

    // Note: bool already implements Display, Debug, Not, etc.
    // No custom implementations needed when B = bool

    // Newtype wrapper for key-value pairs that implements Display
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pair<K, V>(pub K, pub V);

    // Triple wrapper for three-element tuples
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Triple<A, B, C>(pub A, pub B, pub C);

    // Quadruple wrapper for four-element tuples
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Quadruple<A, B, C, D>(pub A, pub B, pub C, pub D);

    // Key-value struct with named fields
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct KeyVal<K, V> {
        pub key: K,
        pub val: V,
    }

    // Type bounds shorthands
    // StT: single-threaded friendly elements: Eq + Clone + Display + Debug + Sized
    pub trait StT: Eq + Clone + Display + Debug + Sized {}
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}

    // StTInMtT: St-friendly elements that can be shared across threads (StT + Send + Sync)
    pub trait StTInMtT: StT + Send + Sync {}
    impl<T> StTInMtT for T where T: StT + Send + Sync {}

    // MtT: multi-threaded friendly elements; minimal so it can include Mutex<..>
    // Keep only thread-safety and size requirements.
    pub trait MtT: Sized + Send + Sync {
        type Inner: StT;
        fn clone_mt(&self)            -> Self;
        fn new_mt(inner: Self::Inner) -> Self;
    }

    // HashFunClone: DEPRECATED - Used by old Chap47 (now in attic)
    // New Chap47 uses HashFunGen pattern instead
    // pub trait HashFunClone<K>: crate::Chap47::HashFunctionTraits::HashFunctionTraits::HashFunction<K> + Clone {}
    // impl<K, T> HashFunClone<K> for T where T: crate::Chap47::HashFunctionTraits::HashFunctionTraits::HashFunction<K> + Clone {}

    // MtKey: Multi-threaded key type with ordering and static lifetime
    // Common pattern: StTInMtT + Ord + 'static (appears 15+ times)
    pub trait MtKey: StTInMtT + Ord + 'static {}
    impl<T> MtKey for T where T: StTInMtT + Ord + 'static {}

    // MtVal: Multi-threaded value type with static lifetime
    // Common pattern: StTInMtT + 'static (appears 15+ times)
    pub trait MtVal: StTInMtT + 'static {}
    impl<T> MtVal for T where T: StTInMtT + 'static {}

    // MtFn: Multi-threaded function type with common bounds
    // Common pattern: Fn(...) + Send + Sync + 'static (appears 30+ times)
    pub trait MtFn<Args, Output>: Fn(Args) -> Output + Send + Sync + 'static {}
    impl<T, Args, Output> MtFn<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + 'static {}

    // MtFnClone: Multi-threaded function type with Clone
    // Common pattern: Fn(...) + Send + Sync + Clone + 'static (appears 20+ times)
    pub trait MtFnClone<Args, Output>: Fn(Args) -> Output + Send + Sync + Clone + 'static {}
    impl<T, Args, Output> MtFnClone<Args, Output> for T where T: Fn(Args) -> Output + Send + Sync + Clone + 'static {}

    // MtReduceFn: Multi-threaded reducer function type
    // Common pattern: Fn(&V, &V) -> V + Clone + Send + Sync + 'static (appears 8+ times)
    pub trait MtReduceFn<V>: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}
    impl<T, V> MtReduceFn<V> for T where T: Fn(&V, &V) -> V + Clone + Send + Sync + 'static {}

    // PredSt: Single-threaded predicate function (boolean function)
    // Common pattern: Fn(&T) -> B (for St/Eph code without Send/Sync)
    pub trait PredSt<T>: Fn(&T) -> B {}
    impl<F, T> PredSt<T> for F where F: Fn(&T) -> B {}

    // PredMt: Multi-threaded predicate function (boolean function)
    // Common pattern: Fn(&T) -> B + Send + Sync + 'static (appears 10+ times)
    pub trait PredMt<T>: Fn(&T) -> B + Send + Sync + 'static {}
    impl<F, T> PredMt<T> for F where F: Fn(&T) -> B + Send + Sync + 'static {}

    // Backward compatibility alias (many existing uses)
    pub use PredMt as Pred;

    // PredVal: Multi-threaded predicate function taking values by value
    // Common pattern: Fn(T) -> B + Send + Sync + 'static (for Copy types like N)
    pub trait PredVal<T>: Fn(T) -> B + Send + Sync + 'static {}
    impl<F, T> PredVal<T> for F where F: Fn(T) -> B + Send + Sync + 'static {}

    // Note: StT + Send + Sync is already covered by existing StTInMtT trait
    // StTInMtT + 'static pattern can be expressed as StTInMtT + 'static inline

    // HashOrd: Type that can be hashed and ordered (for graph vertices)
    // Common pattern: StT + MtT + Hash + Ord (appears in graph modules)
    pub trait HashOrd: StT + Hash + Ord {}
    impl<T> HashOrd for T where T: StT + Hash + Ord {}

    // ArithmeticT: Type supporting arithmetic operations (for reductions)
    // Common pattern: StT + Add<Output = T> + Default + Copy
    pub trait ArithmeticT: StT + Add<Output = Self> + Default + Copy {}
    impl<T> ArithmeticT for T where T: StT + Add<Output = T> + Default + Copy {}

    impl<T: StT + Send> MtT for Mutex<T> {
        type Inner = T;
        fn clone_mt(&self) -> Self {
            let inner = self.lock().unwrap().clone();
            Mutex::new(inner)
        }
        fn new_mt(inner: Self::Inner) -> Self { Mutex::new(inner) }
    }

    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {
        type Inner = Pair<A, B>;
        fn clone_mt(&self) -> Self { self.clone() }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // Ad-hoc implementations for specific primitive types to avoid conflicts
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

    // Special case: ad-hoc implementation for String
    impl MtT for String {
        type Inner = String;
        fn clone_mt(&self) -> Self { self.clone() }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // String slice implementation
    impl<'a> MtT for &'a str {
        type Inner = &'a str;
        fn clone_mt(&self) -> Self { self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // Note: bool already has MtT implementation above (line ~112)
    // No custom implementation needed when B = bool

    /// Edge wrapper to enable Display/Debug for pairs (V,V) under baseline bounds.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Edge<V: StT>(pub V, pub V);

    impl<V: StT> Display for Edge<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.0, self.1) }
    }

    impl<V: StT> From<(V, V)> for Edge<V> {
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }
    }

    impl<V: StT> From<Edge<V>> for (V, V) {
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }
    }

    /// Labeled Edge wrapper to enable edges with labels.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LabEdge<V: StT, L: StT + Hash>(pub V, pub V, pub L);

    impl<V: StT, L: StT + Hash> Display for LabEdge<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.0, self.1, self.2) }
    }

    impl<V: StT, L: StT + Hash> From<(V, V, L)> for LabEdge<V, L> {
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }
    }

    impl<V: StT, L: StT + Hash> From<LabEdge<V, L>> for (V, V, L) {
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }
    }

    // Import OrderedFloat from the ordered-float crate
    // COMMENTED OUT: Not needed for Chapter 5 verified wrappers
    // pub use ordered_float::OrderedFloat;

    // Convenience type aliases for common float types
    // COMMENTED OUT: Not needed for Chapter 5 verified wrappers
    // pub type OrderedF32 = OrderedFloat<f32>;
    // pub type OrderedF64 = OrderedFloat<f64>;

    impl<A, B> From<(A, B)> for Pair<A, B> {
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }
    }

    impl<A, B> From<Pair<A, B>> for (A, B) {
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }
    }

    impl<K: Display, V: Display> Display for Pair<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({} -> {})", self.0, self.1) }
    }

    impl<A: Display, B: Display, C: Display> Display for Triple<A, B, C> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.0, self.1, self.2) }
    }
    impl<A: Display, B: Display, C: Display, D: Display> Display for Quadruple<A, B, C, D> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }
    impl<K: Display, V: Display> Display for KeyVal<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{key: {}, val: {}}}", self.key, self.val)
        }
    }

    // ARCHITECTURE NOTE: Thread Pool-Based Parallelism
    // ================================================
    // Implements the APAS textbook's || (parallel pair) operator.
    // Previous implementation spawned unbounded threads, causing exponential growth:
    // - 16 elements → ~16 threads
    // - 32 elements → ~32+ threads
    // - Resulted in SIGABRT crashes and thread exhaustion
    //
    // Current implementation uses rayon's work-stealing thread pool:
    // - Default pool size (num_cpus threads, typically 8-16)
    // - Work-stealing prevents deadlock during nested parallelism
    // - Allows parallel recursion without thread explosion
    // COMMENTED OUT: Not needed for Chapter 5 verified wrappers
    // #[macro_export]
    // macro_rules! ParaPair {
    //     ( $left:expr, $right:expr ) => {{
    //         let (left_result, right_result) = rayon::join($left, $right);
    //         $crate::Types::Types::Pair(left_result, right_result)
    //     }};
    // }

    #[macro_export]
    macro_rules! EdgeLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Edge($a, $b)
        };
    }

    #[macro_export]
    macro_rules! PairLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Pair($a, $b)
        };
    }

    #[macro_export]
    macro_rules! EdgeList {
        () => {
            Vec::new()
        };
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {
            vec![ $( $crate::EdgeLit!($a, $b) ),* ]
        };
    }

    #[macro_export]
    macro_rules! PairList {
        () => {
            Vec::new()
        };
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {
            vec![ $( $crate::PairLit!($a, $b) ),* ]
        };
    }
}

// Verus View implementation for Pair
#[cfg(verus_keep_ghost)]
pub mod verus_pair_view {
    use super::Types::*;
    use vstd::prelude::*;

    verus! {

    impl<K: View, V: View> View for Pair<K, V> {
        type V = Pair<K::V, V::V>;

        closed spec fn view(&self) -> Pair<K::V, V::V> {
            Pair(self.0@, self.1@)
        }
    }

    } // verus!
}
