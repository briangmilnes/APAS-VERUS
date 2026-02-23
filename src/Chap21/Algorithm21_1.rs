//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.1: 2D Points using ArraySeqPer via tabulate + flatten.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	6. spec fns
//	7. proof fns/broadcast groups
//	9. impls

//		1. module

pub mod Algorithm21_1 {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		6. spec fns

    /// Spec function: sum of lengths of first k inner sequences.
    pub open spec fn sum_inner_lens<T>(ss: Seq<ArraySeqStPerS<T>>, k: int) -> int
        decreases k
    {
        if k <= 0 { 0 }
        else { sum_inner_lens(ss, k - 1) + ss[k - 1].seq@.len() as int }
    }

    //		7. proof fns/broadcast groups

    /// Lemma: sum_inner_lens is monotonically increasing.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
    proof fn lemma_sum_inner_lens_mono<T>(ss: Seq<ArraySeqStPerS<T>>, a: int, b: int)
        requires 0 <= a <= b <= ss.len()
        ensures sum_inner_lens(ss, a) <= sum_inner_lens(ss, b)
        decreases b - a
    {
        if a < b {
            lemma_sum_inner_lens_mono(ss, a, b - 1);
        }
    }

    /// Lemma: if all inner sequences have the same length m, then sum = k * m.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
    proof fn lemma_sum_inner_lens_uniform<T>(ss: Seq<ArraySeqStPerS<T>>, k: int, m: int)
        requires
            0 <= k <= ss.len(),
            forall|i: int| 0 <= i < k ==> (#[trigger] ss[i]).seq@.len() == m,
        ensures
            sum_inner_lens(ss, k) == k * m
        decreases k
    {
        if k > 0 {
            lemma_sum_inner_lens_uniform(ss, k - 1, m);
            assert(sum_inner_lens(ss, k - 1) == (k - 1) * m);
            assert(ss[k - 1].seq@.len() == m);
            assert(sum_inner_lens(ss, k) == sum_inner_lens(ss, k - 1) + ss[k - 1].seq@.len() as int);
            assert((k - 1) * m + m == k * m) by (nonlinear_arith)
                requires k > 0;
        } else {
            assert(sum_inner_lens(ss, k) == 0);
            assert(k * m == 0) by (nonlinear_arith)
                requires k == 0;
        }
    }

    //		9. impls

    /// Flatten a sequence of sequences into a single sequence.
    /// - APAS: Work Θ(m), Span Θ(lg k) where m = total elements, k = number of inner sequences.
    /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — sequential two-pass implementation.
    fn flatten_inner<T: View + Clone>(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (result: ArraySeqStPerS<T>)
        requires
            sum_inner_lens(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int,
        ensures
            result.seq@.len() == sum_inner_lens(ss.seq@, ss.seq@.len() as int),
    {
        let ss_len = ss.seq.len();
        // First pass: compute total length.
        let mut total_len: usize = 0;
        let mut i: usize = 0;
        proof { lemma_sum_inner_lens_mono(ss.seq@, 0, ss.seq@.len() as int); }
        while i < ss_len
            invariant
                i <= ss_len,
                ss_len == ss.seq@.len(),
                total_len as int == sum_inner_lens(ss.seq@, i as int),
                sum_inner_lens(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int,
                sum_inner_lens(ss.seq@, i as int) <= sum_inner_lens(ss.seq@, ss.seq@.len() as int),
            decreases ss_len - i
        {
            proof { lemma_sum_inner_lens_mono(ss.seq@, (i + 1) as int, ss.seq@.len() as int); }
            total_len = total_len + ss.seq[i].seq.len();
            i = i + 1;
        }

        // Second pass: copy all elements.
        let mut result: Vec<T> = Vec::with_capacity(total_len);
        let mut j: usize = 0;
        while j < ss_len
            invariant
                j <= ss_len,
                ss_len == ss.seq@.len(),
                result@.len() == sum_inner_lens(ss.seq@, j as int),
            decreases ss_len - j
        {
            let inner = &ss.seq[j];
            let inner_len = inner.seq.len();
            let mut k: usize = 0;
            while k < inner_len
                invariant
                    k <= inner_len,
                    inner_len == inner.seq@.len(),
                    j < ss_len,
                    ss_len == ss.seq@.len(),
                    result@.len() == sum_inner_lens(ss.seq@, j as int) + k as int,
                decreases inner_len - k
            {
                result.push(inner.seq[k].clone());
                k = k + 1;
            }
            j = j + 1;
        }
        ArraySeqStPerS { seq: result }
    }

    /// Algorithm 21.1 (2D Points) using ArraySeqPer: points2D via tabulate + flatten.
    /// - Functional form: points2D n = flatten (tabulate (\x. tabulate (\y. (x, y+1)) (n-1)) n)
    /// - Generates all 2D points (x, y) where 0 ≤ x < n and 1 ≤ y < n.
    /// - APAS: Work Θ(n²), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential StPer tabulate + flatten.
    pub fn points2d_tab_flat(n: N) -> (result: ArraySeqStPerS<Pair<N, N>>)
        requires
            n <= usize::MAX,
            n as int * (n as int - 1) <= usize::MAX as int,
        ensures
            n == 0 ==> result.seq@.len() == 0,
            n > 0 ==> result.seq@.len() == n as int * (n as int - 1),
    {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }

        // Build the outer sequence: for each x in 0..n, create inner sequence of pairs.
        let inner: ArraySeqStPerS<ArraySeqStPerS<Pair<N, N>>> =
            ArraySeqStPerS::tabulate(
                &(|x: usize| -> (row: ArraySeqStPerS<Pair<N, N>>)
                    requires
                        x < n,
                        n > 0,
                    ensures
                        row.seq@.len() == n - 1,
                {
                    ArraySeqStPerS::tabulate(
                        &(|y: usize| -> (p: Pair<N, N>)
                            requires
                                y < n - 1,
                                x < n,
                            ensures
                                p.0 == x,
                                p.1 == y + 1,
                        {
                            Pair(x, y + 1)
                        }),
                        n - 1,
                    )
                }),
                n,
            );

        proof {
            // Each inner sequence has length n-1.
            assert forall|i: int| 0 <= i < n implies (#[trigger] inner.seq@[i]).seq@.len() == n - 1 by {
                // From tabulate's ensures: result.seq@.len() == length
                // and the closure ensures row.seq@.len() == n - 1
            }
            // Therefore sum_inner_lens(inner, n) == n * (n-1).
            lemma_sum_inner_lens_uniform(inner.seq@, n as int, (n - 1) as int);
            assert(sum_inner_lens(inner.seq@, n as int) == n as int * (n as int - 1));
        }

        flatten_inner(&inner)
    }

    } // verus!
}
