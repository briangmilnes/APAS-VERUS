//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.5: Brute Force Solution to the Primes Problem.

pub mod Algorithm21_5 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap21::Exercise21_8::Exercise21_8::*;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Algorithm21_5Trait {
        /// Algorithm 21.5 (Brute Force Solution to the Primes Problem)
        /// APAS: Work Θ(n^{3/2}), Span Θ(lg n)
        fn primes_bf(n: N) -> ArraySeqStPerS<N>;
    }

    /// Algorithm 21.5 (Brute Force Solution to the Primes Problem)
    /// primesBF n = { i in 2..n : isPrime(i) }
    ///
    /// Finds all prime numbers less than n using brute force primality testing.
    ///
    /// APAS: Work: Θ(n^{3/2}), Span: Θ(lg n)
    /// gpt-5-hard: Work: Θ(n^{3/2}), Span: Θ(lg n)
    pub fn primes_bf(n: N) -> ArraySeqStPerS<N> {
        if n <= 2 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let all: ArraySeqStPerS<N> =
            ArraySeqStPerS::tabulate(&|i| i + 2, n - 2);
        let filtered: ArraySeqStPerS<N> =
            ArraySeqStPerS::filter(&all, &|x| is_prime(*x));
        filtered
    }
}
