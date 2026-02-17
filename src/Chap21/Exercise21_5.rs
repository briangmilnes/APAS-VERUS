//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.5: All contiguous subsequences.
//! Verusified.

pub mod Exercise21_5 {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    /// Exercise 21.5: Generate all contiguous subsequences using nested tabulate + flatten.
    /// APAS: Work Θ(n²), Span Θ(lg n)
    pub fn all_contiguous_subseqs(a: &ArraySeqStPerS<N>) -> (result: ArraySeqStPerS<ArraySeqStPerS<N>>)
         requires obeys_feq_clone::<ArraySeqStPerS<N>>()
    {
        let n = a.length();
        let nested: ArraySeqStPerS<ArraySeqStPerS<ArraySeqStPerS<N>>> =
            ArraySeqStPerS::tabulate(
                &(|i: usize| -> (row: ArraySeqStPerS<ArraySeqStPerS<N>>)
                    requires
                        i < n,
                        n == a.seq@.len(),
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
        mid
    }

    } // verus!
}
