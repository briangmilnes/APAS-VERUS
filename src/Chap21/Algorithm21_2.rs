//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.2: 3D Points using ArraySeqPer via flatten of nested tabulates.
//! Verusified.

pub mod Algorithm21_2 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerRedefinableTrait};

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::{N, Pair, Pair_feq_trigger};

    #[cfg(verus_keep_ghost)]
    verus! {

    use vstd::arithmetic::power::pow;

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::arithmetic::power::group_pow_properties,
        vstd::arithmetic::power::lemma_square_is_pow2,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    /// Lemma: Seq::flatten of n sequences each of length m has length n * m.
    proof fn lemma_flatten_uniform_len<A>(ss: Seq<Seq<A>>, m: int)
        requires
            forall|i: int| 0 <= i < ss.len() ==> (#[trigger] ss[i]).len() == m,
        ensures
            ss.flatten().len() == ss.len() * m,
        decreases ss.len()
    {
        if ss.len() == 0 {
            assert(ss.len() * m == 0) by (nonlinear_arith) requires ss.len() == 0;
        } else {
            assert forall|i: int| 0 <= i < ss.drop_first().len() implies
                (#[trigger] ss.drop_first()[i]).len() == m by {
                assert(ss.drop_first()[i] == ss[i + 1]);
            }
            lemma_flatten_uniform_len(ss.drop_first(), m);
            assert(ss.first().len() == m);
            assert(m + (ss.len() - 1) * m == ss.len() * m) by (nonlinear_arith)
                requires ss.len() > 0;
        }
    }

    /// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.
    /// - Comprehension form: 〈(x,y,z): 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
    /// - Implemented as: flatten (tabulate_x (flatten (tabulate_y (tabulate_z))))
    /// - APAS: Work Θ(n³), Span Θ(lg n)
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
