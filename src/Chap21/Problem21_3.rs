//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.3: Points in 3D using imperative triple loop.

pub mod Problem21_3 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Problem21_3Trait {
        /// Problem 21.3 (Points in 3D) using ArraySeqPer — imperative triple loop
        /// APAS: Work Θ(n³), Span Θ(n³)
        fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>>;
    }

    /// Problem 21.3 (Points in 3D) using ArraySeqPer — imperative triple loop.
    /// Generate points (x, y, z) with 0 ≤ x ≤ n−1, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1 in x-major, then y, then z order.
    ///
    /// This is an educational example showing imperative style for comparison with functional approaches.
    /// The triple loop is intentionally inefficient (sequential) to contrast with parallel functional methods.
    /// gpt-5-hard: Work: Θ(n³), Span: Θ(n³) (sequential due to imperative loops)
    /// APAS: Work: Θ(n³), Span: Θ(n³)
    pub fn points3d_loops(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {
        if n == 0 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let len = n * n * n;
        let mut v = Vec::<Pair<N, Pair<N, N>>>::with_capacity(len);
        for x in 0..n {
            for y in 1..=n {
                for z in 2..=n + 1 {
                    v.push(Pair(x, Pair(y, z)));
                }
            }
        }
        ArraySeqStPerS::from_vec(v)
    }
}
