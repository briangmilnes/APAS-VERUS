//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 21 — Problem 21.4: Cartesian Product using different approaches.
//! Verusified.

//  Table of Contents
//	1. module
//	3. broadcast use
//	9. impls

//		1. module


pub mod Problem21_4 {

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq::seq::{lemma_flatten_uniform_len, lemma_flatten_all};

    verus! {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
    };


    //		9. impls

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
            forall|k: int| 0 <= k < result.seq@.len() ==> (
                a.seq@.contains((#[trigger] result.seq@[k]).0)
                && b.seq@.contains(result.seq@[k].1)
            ),
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
                    forall|j: int| 0 <= j < row.seq@.len() ==> (
                        a.seq@.contains((#[trigger] row.seq@[j]).0)
                        && b.seq@.contains(row.seq@[j].1)
                    ),
            {
                let x = *a.nth(i);
                let row = ArraySeqStPerS::tabulate(
                    &(|j: usize| -> (p: Pair<N, N>)
                        requires
                            j < blen,
                            blen == b.seq@.len(),
                            i < alen,
                            alen == a.seq@.len(),
                        ensures
                            a.seq@.contains(p.0),
                            b.seq@.contains(p.1),
                    {
                        let y = *b.nth(j);
                        proof {
                            assert(a.seq@[i as int] == x);
                            assert(b.seq@[j as int] == y);
                        }
                        Pair(x, y)
                    }),
                    blen,
                );
                row
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

            let ghost pred = |p: Pair<N, N>|
                a.seq@.contains(p.0) && b.seq@.contains(p.1);
            assert forall|i: int, j: int|
                0 <= i < mapped.len() && 0 <= j < mapped[i].len()
                implies #[trigger] pred(mapped[i][j]) by {
                assert(mapped[i][j] == nested.seq@[i].seq@[j]);
            }
            lemma_flatten_all(mapped, pred);
            assert forall|k: int| 0 <= k < result.seq@.len() implies (
                a.seq@.contains((#[trigger] result.seq@[k]).0)
                && b.seq@.contains(result.seq@[k].1)
            ) by {
                assert(result.seq@[k] == mapped.flatten()[k]);
                assert(pred(mapped.flatten()[k]));
            }
        }
        result
    }

    } // verus!
}
