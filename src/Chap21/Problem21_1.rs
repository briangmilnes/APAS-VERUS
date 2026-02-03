//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.1: Points in 2D using imperative loops.

pub mod Problem21_1 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Problem21_1Trait {
        /// Problem 21.1 (Points in 2D) - Imperative approach using nested loops
        /// APAS: Work Θ(n²), Span Θ(n²)
        fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>>;
    }

    /// Problem 21.1 (Points in 2D) - Imperative approach using nested loops.
    /// Construct the sequence of 2D points (x, y) with 0 ≤ x < n and 1 ≤ y < n,
    /// ordered by x major, then y.
    ///
    /// This is an educational example showing imperative style for comparison.
    /// Work: Θ(n²), Span: Θ(n²) (sequential due to imperative loops)
    pub fn points2d(n: N) -> ArraySeqStPerS<Pair<N, N>> {
        if n == 0 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let len = n * (n - 1);
        let mut v = Vec::<Pair<N, N>>::with_capacity(len);
        for x in 0..n {
            for y in 1..n {
                v.push(Pair(x, y));
            }
        }
        ArraySeqStPerS::from_vec(v)
    }
}
