//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.2: 3D Points using ArraySeqPer via flatten of nested tabulates.

pub mod Algorithm21_2 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Algorithm21_2Trait {
        /// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates
        /// APAS: Work Θ(n³), Span Θ(lg n)
        fn points3d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>>;
    }

    /// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.
    /// Comprehension form: 〈(x,y,z): 0 ≤ x ≤ n−1, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
    /// Implemented as: flatten ∘ (tabulate_x (flatten ∘ (tabulate_y (tabulate_z))))
    ///
    /// Generates all 3D points (x, y, z) where 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1.
    /// gpt-5-hard: Work: Θ(n³), Span: Θ(lg n)
    /// APAS: Work: Θ(n³), Span: Θ(lg n)
    pub fn points3d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, Pair<N, N>>> {
        if n == 0 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let outer: ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> = ArraySeqStPerS::tabulate(
            &|x| {
                let mid: ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> = ArraySeqStPerS::tabulate(
                    &|y| {
                        ArraySeqStPerS::tabulate(
                                &|z_idx| Pair(x, Pair(y + 1, z_idx + 2)),
                                n + 1 - 2 + 1, // z: 2..=n+1 has length n
                            )
                    },
                    n, // y: 1..=n has length n
                );
                ArraySeqStPerS::flatten(&mid)
            },
            n,
        );
        ArraySeqStPerS::flatten(&outer)
    }
}
