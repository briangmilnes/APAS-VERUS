//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Disjoint parallelism: verified parallel execution of independent tasks.
//!

pub mod ParaPairs {
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
    pub fn para_pair_disjoint<A, B, F1, F2>(f1: F1, f2: F2) -> (result: Pair<A, B>)
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

    } // verus!
}

/// Unverified ParaPair macro for non-Verus code
#[macro_export]
macro_rules! ParaPair {
    ( $left:expr, $right:expr ) => {{
        use std::thread::spawn;
        let handle1 = spawn($left);
        let handle2 = spawn($right);
        let left_result = handle1.join().unwrap();
        let right_result = handle2.join().unwrap();
        $crate::Types::Types::Pair(left_result, right_result)
    }};
}

/// Verified disjoint ParaPair - calls disjoint_pair function
#[macro_export]
macro_rules! ParaPairDisjoint {
    ( $f1:expr, $f2:expr ) => {
        $crate::ParaPairs::ParaPairs::para_pair_disjoint($f1, $f2)
    };
}
