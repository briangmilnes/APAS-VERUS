//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.2: 3D Points using ArraySeqPer via flatten of nested tabulates.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	9. impls

//		1. module

pub mod Algorithm21_2 {

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power::pow;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq::seq::lemma_flatten_uniform_len;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::arithmetic::power::group_pow_properties,
        vstd::arithmetic::power::lemma_square_is_pow2,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    //		9. impls

    /// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.
    /// - Comprehension form: 〈(x,y,z): 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
    /// - Implemented as: flatten (tabulate_x (flatten (tabulate_y (tabulate_z))))
    /// - APAS: Work Θ(n³), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — sequential StPer nested tabulate + flatten.
    pub fn points3d_tab_flat(n: N) -> (result: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
        requires
            n + 2 <= usize::MAX,
            pow(n as int, 2) <= usize::MAX as int,
            pow(n as int, 3) <= usize::MAX as int,
        ensures
            n == 0 ==> result.seq@.len() == 0,
            n > 0  ==> result.seq@.len() == pow(n as int, 3),
    {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }

        // Build the outer sequence: for each x in 0..n, produce a flattened n² block.
        let outer: ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> =
            ArraySeqStPerS::tabulate(
                &(|x: usize| -> (block: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
                    requires
                        x < n,
                        n > 0,
                        n + 2 <= usize::MAX,
                        pow(n as int, 2) <= usize::MAX as int,
                    ensures
                        block.seq@.len() == pow(n as int, 2),
                {
                    proof {
                        // Trigger Pair feq axioms for the nested Pair type.
                        assert(Pair_feq_trigger::<N, N>());
                        assert(Pair_feq_trigger::<N, Pair<N, N>>());
                    }
                    // For each y in 0..n, tabulate z values.
                    let mid: ArraySeqStPerS<ArraySeqStPerS<Pair<N, Pair<N, N>>>> =
                        ArraySeqStPerS::tabulate(
                            &(|y: usize| -> (row: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
                                requires
                                    y < n,
                                    x < n,
                                    n > 0,
                                    n + 2 <= usize::MAX,
                                ensures
                                    row.seq@.len() == n as int,
                            {
                                ArraySeqStPerS::tabulate(
                                    &(|z_idx: usize| -> (p: Pair<N, Pair<N, N>>)
                                        requires z_idx < n, x < n, y < n, n + 2 <= usize::MAX,
                                    {
                                        Pair(x, Pair(y + 1, z_idx + 2))
                                    }),
                                    n,
                                )
                            }),
                            n,
                        );

                    let flat_mid = ArraySeqStPerS::flatten(&mid);
                    proof {
                        let ghost mapped = mid.seq@.map_values(|inner: ArraySeqStPerS<Pair<N, Pair<N, N>>>| inner.seq@);
                        assert forall|i: int| 0 <= i < mapped.len() implies
                            (#[trigger] mapped[i]).len() == n as int by {}
                        lemma_flatten_uniform_len(mapped, n as int);
                        // n * n == pow(n, 2) via lemma_square_is_pow2
                    }
                    flat_mid
                }),
                n,
            );

        proof {
            assert(Pair_feq_trigger::<N, N>());
            assert(Pair_feq_trigger::<N, Pair<N, N>>());
        }
        let result = ArraySeqStPerS::flatten(&outer);
        proof {
            let ghost n2 = pow(n as int, 2);
            let ghost mapped = outer.seq@.map_values(|inner: ArraySeqStPerS<Pair<N, Pair<N, N>>>| inner.seq@);
            assert forall|i: int| 0 <= i < mapped.len() implies
                (#[trigger] mapped[i]).len() == n2 by {}
            lemma_flatten_uniform_len(mapped, n2);
            // n * pow(n, 2) == pow(n, 2+1) == pow(n, 3)
            vstd::arithmetic::power::lemma_pow_adds(n as int, 2, 1);
        }
        result
    }

    } // verus!
}
