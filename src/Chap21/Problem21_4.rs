//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.4: Cartesian Product using different approaches.
//! Verusified.

pub mod Problem21_4 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    /// Problem 21.4 (Cartesian Product) - Imperative approach using explicit loops.
    /// - APAS: Work Θ(|a|·|b|), Span Θ(|a|·|b|)
    /// - Claude-Opus-4.6: Work Θ(|a|·|b|), Span Θ(|a|·|b|)
    pub fn cartesian_loops(
        a: &ArraySeqStPerS<N>,
        b: &ArraySeqStPerS<N>,
    ) -> (result: ArraySeqStPerS<Pair<N, N>>)
        requires
            a.seq@.len() as int * b.seq@.len() as int <= usize::MAX as int,
        ensures
            result.seq@.len() == a.seq@.len() as int * b.seq@.len() as int,
    {
        let alen = a.length();
        let blen = b.length();
        let cap = alen * blen;
        let mut v = Vec::<Pair<N, N>>::with_capacity(cap);
        let mut i: usize = 0;
        while i < alen
            invariant
                i <= alen,
                alen == a.seq@.len(),
                blen == b.seq@.len(),
                cap == alen * blen,
                v@.len() == i as int * blen as int,
                cap as int <= usize::MAX as int,
            decreases alen - i,
        {
            let mut j: usize = 0;
            assert((i as int + 1) * blen as int <= cap as int) by (nonlinear_arith)
                requires i < alen, cap == alen * blen;
            while j < blen
                invariant
                    i < alen, j <= blen,
                    alen == a.seq@.len(),
                    blen == b.seq@.len(),
                    cap == alen * blen,
                    v@.len() == i as int * blen as int + j as int,
                    (i as int + 1) * blen as int <= cap as int,
                    cap as int <= usize::MAX as int,
                decreases blen - j,
            {
                v.push(Pair(*a.nth(i), *b.nth(j)));
                j = j + 1;
            }
            assert((i as int + 1) * blen as int == i as int * blen as int + blen as int)
                by (nonlinear_arith);
            i = i + 1;
        }
        ArraySeqStPerS::from_vec(v)
    }

    /// Lemma: Seq::flatten of k sequences each of length m has length k * m.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
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

    /// Problem 21.4 (Cartesian Product) - Functional approach using tabulate + flatten.
    /// flatten(tabulate(λi. tabulate(λj. (a[i], b[j])) |b|) |a|)
    /// - APAS: Work Θ(|a|·|b|), Span Θ(lg |a|)
    /// - Claude-Opus-4.6: Work Θ(|a|·|b|), Span Θ(|a|·|b|) — sequential StPer tabulate + flatten.
    pub fn cartesian_tab_flat(
        a: &ArraySeqStPerS<N>,
        b: &ArraySeqStPerS<N>,
    ) -> (result: ArraySeqStPerS<Pair<N, N>>)
        requires
            a.seq@.len() as int * b.seq@.len() as int <= usize::MAX as int,
        ensures
            result.seq@.len() == a.seq@.len() as int * b.seq@.len() as int,
    {
        let alen = a.length();
        let blen = b.length();

        let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, N>>> = ArraySeqStPerS::tabulate(
            &(|i: usize| -> (row: ArraySeqStPerS<Pair<N, N>>)
                requires
                    i < alen,
                    alen == a.seq@.len(),
                    blen == b.seq@.len(),
                ensures
                    row.seq@.len() == blen,
            {
                let x = *a.nth(i);
                ArraySeqStPerS::tabulate(
                    &(|j: usize| -> (p: Pair<N, N>)
                        requires
                            j < blen,
                            blen == b.seq@.len(),
                    {
                        Pair(x, *b.nth(j))
                    }),
                    blen,
                )
            }),
            alen,
        );

        proof { assert(Pair_feq_trigger::<N, N>()); }
        let result = ArraySeqStPerS::flatten(&nested);
        proof {
            let ghost mapped = nested.seq@.map_values(
                |inner: ArraySeqStPerS<Pair<N, N>>| inner.seq@);
            assert forall|i: int| 0 <= i < mapped.len() implies
                (#[trigger] mapped[i]).len() == blen as int by {}
            lemma_flatten_uniform_len(mapped, blen as int);
        }
        result
    }

    } // verus!
}
