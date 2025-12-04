//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Disjoint parallelism: verified parallel execution of independent tasks.
//!
//! KEY INSIGHT: For disjoint parallelism (no shared mutable state),
//! TSMs are NOT needed! The closure spec mechanism is sufficient:
//! - Closures carry specs via f.requires() and f.ensures()
//! - spawn requires f.requires(())
//! - join ensures JoinHandle::predicate(ret) ==> f.ensures((), ret)
//!
//! TSMs become necessary when threads share mutable state (counters, locks, etc).
//! See Verus counting-to-2 example for that pattern.

pub mod DisjointPar {
    use vstd::prelude::*;
    use vstd::thread::*;
    use crate::Concurrency::diverge;

    verus! {

    use crate::Types::Types::Pair;

    /// Disjoint parallel pair: spawn two independent computations, join both.
    ///
    /// The closure specs flow through spawn/join:
    /// - spawn requires f.requires(())
    /// - join ensures JoinHandle::predicate(ret) ==> f.ensures((), ret)
    pub fn disjoint_pair<A, B, F1, F2>(f1: F1, f2: F2) -> (result: Pair<A, B>)
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
            _ => { assume(false); diverge() }  // thread panicked - unreachable for verified closures
        };
        let b = match h2.join() {
            Result::Ok(v) => v,
            _ => { assume(false); diverge() }  // thread panicked - unreachable for verified closures
        };

        Pair(a, b)
    }

    } // verus!
}

/// Macro for disjoint parallel pair with closure specs.
/// Usage: `DisjointPair!(|| expr1, || expr2)` 
#[macro_export]
macro_rules! DisjointPair {
    ( $f1:expr, $f2:expr ) => {
        $crate::DisjointPar::DisjointPar::disjoint_pair($f1, $f2)
    };
}
