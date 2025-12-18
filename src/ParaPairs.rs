//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel pair abstractions for fork-join parallelism.

pub mod ParaPairs {
    use vstd::prelude::*;
    use vstd::thread::*;
    use crate::Concurrency::diverge;

    verus! {

    use crate::Types::Types::Pair;

    /// Verified parallel pair with closure spec propagation.
    ///
    /// The closure specs flow through spawn/join:
    /// - spawn requires f.requires(())
    /// - join ensures JoinHandle::predicate(ret) ==> f.ensures((), ret)
    pub fn para_pair<A, B, F1, F2>(f1: F1, f2: F2) -> (result: Pair<A, B>)
        where
            F1: FnOnce() -> A + Send + 'static,
            F2: FnOnce() -> B + Send + 'static,
            A: Send + 'static,
            B: Send + 'static,
        requires
            f1.requires(()),
            f2.requires(()),
        ensures
            f1.ensures((), result.0),
            f2.ensures((), result.1),
    {
        let h1 = spawn(f1);
        let h2 = spawn(f2);

        let a = match h1.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }
        };
        let b = match h2.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }
        };

        Pair(a, b)
    }

    /// Verified disjoint parallel pair for Set-viewing types.
    ///
    /// Works with any type where View = Set<T> (SetStEph, SetMtEph, etc).
    /// Caller must prove closures produce disjoint outputs.
    pub fn para_pair_disjoint<T, A, B, F1, F2>(f1: F1, f2: F2) -> (result: Pair<A, B>)
        where
            A: View<V = Set<T>> + Send + 'static,
            B: View<V = Set<T>> + Send + 'static,
            F1: FnOnce() -> A + Send + 'static,
            F2: FnOnce() -> B + Send + 'static,
        requires
            f1.requires(()),
            f2.requires(()),
            forall |a: A, b: B| f1.ensures((), a) && f2.ensures((), b) ==> a@.disjoint(b@),
        ensures
            f1.ensures((), result.0),
            f2.ensures((), result.1),
            result.0@.disjoint(result.1@),
    {
        let h1 = spawn(f1);
        let h2 = spawn(f2);

        let a = match h1.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }
        };
        let b = match h2.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }
        };

        Pair(a, b)
    }

    } // verus!
}

/// Verified ParaPair macro - calls para_pair function with spec propagation
#[macro_export]
macro_rules! ParaPair {
    ( $f1:expr, $f2:expr ) => {
        $crate::ParaPairs::ParaPairs::para_pair($f1, $f2)
    };
}

/// Verified disjoint ParaPair - calls para_pair_disjoint function
#[macro_export]
macro_rules! ParaPairDisjoint {
    ( $f1:expr, $f2:expr ) => {
        $crate::ParaPairs::ParaPairs::para_pair_disjoint($f1, $f2)
    };
}
