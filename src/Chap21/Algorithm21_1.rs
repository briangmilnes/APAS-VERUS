//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.1: 2D Points using ArraySeqPer via tabulate + flatten.
//! Verusified.

pub mod Algorithm21_1 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::{N, Pair};

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use vstd::std_specs::vec::group_vec_axioms;

    /// Spec function: sum of lengths of first k inner sequences.
    pub open spec fn sum_inner_lens<T>(ss: Seq<ArraySeqStPerS<T>>, k: int) -> int
        decreases k
    {
        if k <= 0 { 0 }
        else { sum_inner_lens(ss, k - 1) + ss[k - 1].seq@.len() as int }
    }

    /// Lemma: sum_inner_lens is monotonically increasing.
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
    proof fn lemma_sum_inner_lens_uniform<T>(ss: Seq<ArraySeqStPerS<T>>, k: int, m: int)
        requires
            0 <= k <= ss.len(),
            forall|i: int| 0 <= i < k ==> ss[i].seq@.len() == m,
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

    /// Helper: flatten a sequence of sequences into a single sequence.
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
            assert forall|i: int| 0 <= i < n implies inner.seq@[i].seq@.len() == n - 1 by {
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

    // Non-Verus implementation for cargo test compatibility.
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(not(verus_keep_ghost))]
    use crate::Types::Types::{N, Pair};

    #[cfg(not(verus_keep_ghost))]
    pub fn points2d_tab_flat(n: N) -> ArraySeqStPerS<Pair<N, N>> {
        if n == 0 {
            return ArraySeqStPerS { seq: Vec::new() };
        }
        let inner: ArraySeqStPerS<ArraySeqStPerS<Pair<N, N>>> =
            ArraySeqStPerS::tabulate(
                &|x| ArraySeqStPerS::tabulate(&|y| Pair(x, y + 1), n - 1),
                n,
            );
        flatten_inner_non_verus(&inner)
    }

    #[cfg(not(verus_keep_ghost))]
    fn flatten_inner_non_verus<T: Clone>(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {
        let mut result = Vec::new();
        for inner in ss.seq.iter() {
            result.extend(inner.seq.iter().cloned());
        }
        ArraySeqStPerS { seq: result }
    }
}
