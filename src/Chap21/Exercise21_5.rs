//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.5: All contiguous subsequences.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	7. proof fns/broadcast groups
//	9. impls

//		1. module

pub mod Exercise21_5 {

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq::seq::{spec_inner_lens_sum, lemma_flatten_len_is_inner_lens_sum};

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		7. proof fns/broadcast groups

    /// The sum of inner lengths for a descending pattern ss[i].len() == n-i
    /// equals the n-th triangular number: n*(n+1)/2.
    proof fn lemma_inner_lens_sum_triangular<A>(ss: Seq<Seq<A>>, n: int)
        requires
            ss.len() == n,
            n >= 0,
            forall|i: int| 0 <= i < n ==> (#[trigger] ss[i]).len() == n - i,
        ensures
            spec_inner_lens_sum(ss) * 2 == n * (n + 1),
        decreases n
    {
        if n > 0 {
            assert forall|i: int| 0 <= i < ss.drop_first().len() implies
                (#[trigger] ss.drop_first()[i]).len() == (n - 1) - i by {
                assert(ss.drop_first()[i] == ss[i + 1]);
            }
            lemma_inner_lens_sum_triangular(ss.drop_first(), n - 1);
            assert(spec_inner_lens_sum(ss) * 2 == n * (n + 1)) by (nonlinear_arith)
                requires
                    spec_inner_lens_sum(ss) == ss.first().len() + spec_inner_lens_sum::<A>(ss.drop_first()),
                    ss.first().len() == n,
                    spec_inner_lens_sum::<A>(ss.drop_first()) * 2 == (n - 1) * n;
        }
    }

    //		9. impls

    /// Exercise 21.5: Generate all contiguous subsequences using nested tabulate + flatten.
    /// - APAS: Work Θ(n²), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — sequential StPer; subseq_copy is O(k) not O(1).
    pub fn all_contiguous_subseqs(a: &ArraySeqStPerS<N>) -> (result: ArraySeqStPerS<ArraySeqStPerS<N>>)
         requires obeys_feq_clone::<ArraySeqStPerS<N>>()
         ensures
            a.spec_len() == 0 ==> result.spec_len() == 0,
            a.spec_len() > 0 ==> result.spec_len() * 2 == a.spec_len() * (a.spec_len() + 1),
    {
        let n = a.length();
        let nested: ArraySeqStPerS<ArraySeqStPerS<ArraySeqStPerS<N>>> =
            ArraySeqStPerS::tabulate(
                &(|i: usize| -> (row: ArraySeqStPerS<ArraySeqStPerS<N>>)
                    requires
                        i < n,
                        n == a.seq@.len(),
                    ensures
                        row.seq@.len() == n - i,
                {
                    ArraySeqStPerS::tabulate(
                        &(|j: usize| -> (sub: ArraySeqStPerS<N>)
                            requires
                                j < n - i,
                                i < n,
                                n == a.seq@.len(),
                        {
                            a.subseq_copy(i, j + 1)
                        }),
                        n - i,
                    )
                }),
                n,
            );
        let mid: ArraySeqStPerS<ArraySeqStPerS<N>> =
            ArraySeqStPerS::flatten(&nested);
        proof {
            let ghost mapped = nested.seq@.map_values(
                |inner: ArraySeqStPerS<ArraySeqStPerS<N>>| inner.seq@);
            assert forall|i: int| 0 <= i < mapped.len() implies
                (#[trigger] mapped[i]).len() == n as int - i by {}
            lemma_flatten_len_is_inner_lens_sum(mapped);
            lemma_inner_lens_sum_triangular(mapped, n as int);
        }
        mid
    }

    } // verus!
}
