//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 21 — Algorithm 21.2: 3D Points using ArraySeqPer via flatten of nested tabulates.
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 9. impls

//		Section 1. module

pub mod Algorithm21_2 {


    //		Section 2. imports

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::power::pow;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq::seq::lemma_flatten_uniform_len;

    verus! 
{


    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::arithmetic::power::group_pow_properties,
        vstd::arithmetic::power::lemma_square_is_pow2,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    //		Section 9. impls


    /// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.
    /// - Comprehension form: 〈(x,y,z): 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
    /// - Implemented as: flatten (tabulate_x (flatten (tabulate_y (tabulate_z))))
    /// - Alg Analysis: APAS (Ch21 Alg 21.2): Work O(n³), Span O(lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n³), Span O(n³) — sequential StPer nested tabulate + flatten.
    pub fn points3d_tab_flat(n: usize) -> (points: ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>)
        requires
            n + 2 <= usize::MAX,
            pow(n as int, 2) <= usize::MAX as int,
            pow(n as int, 3) <= usize::MAX as int,
        ensures
            n == 0 ==> points.seq@.len() == 0,
            n > 0  ==> points.seq@.len() == pow(n as int, 3),
    {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }

        // Build the outer sequence: for each x in 0..n, produce a flattened n² block.
        let outer: ArraySeqStPerS<ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>> =
            ArraySeqStPerS::tabulate(
                &(|x: usize| -> (block: ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>)
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
                        assert(Pair_feq_trigger::<usize, usize>());
                        assert(Pair_feq_trigger::<usize, Pair<usize, usize>>());
                    }
                    // For each y in 0..n, tabulate z values.
                    let mid: ArraySeqStPerS<ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>> =
                        ArraySeqStPerS::tabulate(
                            &(|y: usize| -> (row: ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>)
                                requires
                                    y < n,
                                    x < n,
                                    n > 0,
                                    n + 2 <= usize::MAX,
                                ensures
                                    row.seq@.len() == n as int,
                            {
                                ArraySeqStPerS::tabulate(
                                    &(|z_idx: usize| -> (p: Pair<usize, Pair<usize, usize>>)
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
                        let ghost mapped = mid.seq@.map_values(|inner: ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>| inner.seq@);
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
            assert(Pair_feq_trigger::<usize, usize>());
            assert(Pair_feq_trigger::<usize, Pair<usize, usize>>());
        }
        let flattened = ArraySeqStPerS::flatten(&outer);
        proof {
            let ghost n2 = pow(n as int, 2);
            let ghost mapped = outer.seq@.map_values(|inner: ArraySeqStPerS<Pair<usize, Pair<usize, usize>>>| inner.seq@);
            assert forall|i: int| 0 <= i < mapped.len() implies
                (#[trigger] mapped[i]).len() == n2 by {}
            lemma_flatten_uniform_len(mapped, n2);
            // n * pow(n, 2) == pow(n, 2+1) == pow(n, 3)
            vstd::arithmetic::power::lemma_pow_adds(n as int, 2, 1);
        }
        flattened
    }

    } // verus!
}
