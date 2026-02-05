//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.1: 2D Points using ArraySeqPer via tabulate + flatten.

pub mod Algorithm21_1 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use vstd::prelude::*;

    verus! {

    pub type T = N;

    /// Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten.
    /// Functional form: points2D n = flatten (tabulate (\x. tabulate (\y. (x, y+1)) (n-1)) n)
    /// Generates all 2D points (x, y) where 0 <= x < n and 1 <= y < n.
    /// Work: Θ(n²), Span: Θ(lg n)
    pub fn points2d_tab_flat(n: N) -> (result: ArraySeqStPerS<Pair<N, N>>)
        requires
            n as int * (n as int - 1) <= usize::MAX as int,
        ensures
            result.seq@.len() == n as int * (n as int - 1),
    {
        if n == 0 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }

        // outer = tabulate (\x. tabulate (\y. (x, y+1)) (n-1)) n
        let outer: ArraySeqStPerS<ArraySeqStPerS<Pair<N, N>>> =
            ArraySeqStPerS::<ArraySeqStPerS<Pair<N, N>>>::tabulate(
                &(|x: usize| -> (row: ArraySeqStPerS<Pair<N, N>>)
                    requires
                        x < n,
                        n > 0,
                    ensures
                        row.seq@.len() == n - 1,
                {
                    // Inner tabulate: tabulate (\y. (x, y+1)) (n-1)
                    ArraySeqStPerS::<Pair<N, N>>::tabulate(
                        &(|y: usize| -> (p: Pair<N, N>)
                            requires y < n - 1
                        {
                            Pair(x, y + 1)
                        }),
                        n - 1,
                    )
                }),
                n,
            );

        proof {
            // The outer tabulate's ensures gives us: f.ensures((i,), outer.seq@[i])
            // where f's ensures is: row.seq@.len() == n - 1
            // So outer.seq@[i].seq@.len() == n - 1 for all i < n.
            assert forall|i: int| #![auto] 0 <= i < n as int implies outer.seq@[i].seq@.len() == n - 1 by {
                // Trigger the tabulate postcondition
            }
            lemma_sum_lens_uniform(outer.seq@, n as int, (n - 1) as int);
        }

        <ArraySeqStPerS<Pair<N, N>> as ArraySeqStPerTrait<Pair<N, N>>>::flatten(&outer)
    }

    // Proves that if all inner sequences have the same length m, then sum_lens equals k * m.
    proof fn lemma_sum_lens_uniform<T>(ss: Seq<ArraySeqStPerS<T>>, k: int, m: int)
        requires
            k >= 0,
            k <= ss.len(),
            forall|i: int| #![auto] 0 <= i < k ==> ss[i].seq@.len() == m,
        ensures
            sum_lens(ss, k) == k * m,
        decreases k,
    {
        if k == 0 {
        } else {
            lemma_sum_lens_uniform(ss, k - 1, m);
            // Direct calls needed for performance (broadcast group causes rlimit issues).
            vstd::arithmetic::mul::lemma_mul_is_distributive_add_other_way(m, k - 1, 1);
            vstd::arithmetic::mul::lemma_mul_basics(m);
        }
    }

    } // verus!
}
