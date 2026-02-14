//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.5: Brute Force Solution to the Primes Problem.
//! Verusified.

pub mod Algorithm21_5 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};

    #[cfg(verus_keep_ghost)]
    use crate::Chap21::Exercise21_8::Exercise21_8::{is_prime, spec_is_prime};

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::N;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    pub type T = N;

    /// Algorithm 21.5 (Brute Force Solution to the Primes Problem)
    /// primesBF n = { i in 2..n : isPrime(i) }
    ///
    /// Finds all prime numbers less than n using brute force primality testing.
    ///
    /// APAS: Work Θ(n^{3/2}), Span Θ(lg n)
    pub fn primes_bf(n: N) -> (result: ArraySeqStPerS<N>)
        ensures
            n <= 2 ==> result.spec_len() == 0,
            n > 2  ==> result.spec_len() <= n - 2,
            forall|i: int| 0 <= i < result.spec_len()
                ==> spec_is_prime(#[trigger] result.spec_index(i) as int),
    {
        if n <= 2 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let all: ArraySeqStPerS<N> = ArraySeqStPerS::tabulate(
            &(|i: usize| -> (v: N)
                requires i < n - 2,
                ensures v == i + 2,
            { i + 2 }),
            n - 2,
        );
        let pred = |x: &N| -> (keep: bool)
            ensures keep == spec_is_prime(*x as int),
        { is_prime(*x) };
        let ghost spec_pred: spec_fn(N) -> bool = |x: N| spec_is_prime(x as int);
        proof {
            assume(forall|v: N, ret: bool| pred.ensures((&v,), ret) <==> spec_pred(v) == ret);
        }
        let filtered: ArraySeqStPerS<N> = ArraySeqStPerS::filter(
            &all,
            &pred,
            Ghost(spec_pred),
        );
        filtered
    }

    } // verus!
}
