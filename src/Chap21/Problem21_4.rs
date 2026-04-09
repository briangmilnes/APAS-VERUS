//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

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
    /// - Alg Analysis: APAS (Ch21 Alg 21.3): Work O(|a|·|b|), Span O(|a|·|b|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|·|b|), Span O(|a|·|b|)
    pub fn cartesian_loops(
        a: &ArraySeqStPerS<usize>,
        b: &ArraySeqStPerS<usize>,
    ) -> (pairs: ArraySeqStPerS<Pair<usize, usize>>)
        requires
            a.seq@.len() as int * b.seq@.len() as int <= usize::MAX as int,
        ensures
            pairs.seq@.len() == a.seq@.len() as int * b.seq@.len() as int,
    {
        let alen = a.length();
        let blen = b.length();
        let cap = alen * blen;
        let mut v = Vec::<Pair<usize, usize>>::with_capacity(cap);
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
            // Veracity: NEEDED assert
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
            // Veracity: NEEDED assert
            assert((i as int + 1) * blen as int == i as int * blen as int + blen as int)
                by (nonlinear_arith);
            i = i + 1;
        }
        ArraySeqStPerS::from_vec(v)
    }

    /// Problem 21.4 (Cartesian Product) - Functional approach using tabulate + flatten.
    /// flatten(tabulate(λi. tabulate(λj. (a[i], b[j])) |b|) |a|)
    /// - Alg Analysis: APAS (Ch21 Alg 21.3): Work O(|a|·|b|), Span O(lg |a|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|·|b|), Span O(|a|·|b|) — sequential StPer tabulate + flatten.
    pub fn cartesian_tab_flat(
        a: &ArraySeqStPerS<usize>,
        b: &ArraySeqStPerS<usize>,
    ) -> (pairs: ArraySeqStPerS<Pair<usize, usize>>)
        requires
            a.seq@.len() as int * b.seq@.len() as int <= usize::MAX as int,
        ensures
            pairs.seq@.len() == a.seq@.len() as int * b.seq@.len() as int,
            forall|k: int| 0 <= k < pairs.seq@.len() ==> (
                a.seq@.contains((#[trigger] pairs.seq@[k]).0)
                && b.seq@.contains(pairs.seq@[k].1)
            ),
    {
        let alen = a.length();
        let blen = b.length();

        let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<usize, usize>>> = ArraySeqStPerS::tabulate(
            &(|i: usize| -> (row: ArraySeqStPerS<Pair<usize, usize>>)
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
                    &(|j: usize| -> (p: Pair<usize, usize>)
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
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        Pair(x, y)
                    }),
                    blen,
                );
                row
            }),
            alen,
        );

        // Veracity: NEEDED proof block
        proof { assert(Pair_feq_trigger::<usize, usize>()); }
        let pairs = ArraySeqStPerS::flatten(&nested);
        // Veracity: NEEDED proof block
        proof {
            let ghost mapped = nested.seq@.map_values(
                |inner: ArraySeqStPerS<Pair<usize, usize>>| inner.seq@);
            // Veracity: NEEDED assert
            assert forall|i: int| 0 <= i < mapped.len() implies
                (#[trigger] mapped[i]).len() == blen as int by {}
            lemma_flatten_uniform_len(mapped, blen as int);

            let ghost pred = |p: Pair<usize, usize>|
                a.seq@.contains(p.0) && b.seq@.contains(p.1);
            lemma_flatten_all(mapped, pred);
            // Veracity: NEEDED assert
            assert forall|k: int| 0 <= k < pairs.seq@.len() implies (
                a.seq@.contains((#[trigger] pairs.seq@[k]).0)
                && b.seq@.contains(pairs.seq@[k].1)
            ) by {
                // Veracity: NEEDED assert
                assert(pred(mapped.flatten()[k]));
            }
        }
        pairs
    }

    } // verus!
}
