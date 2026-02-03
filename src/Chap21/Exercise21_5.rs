//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.5: All contiguous subsequences.

pub mod Exercise21_5 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T<S> = ArraySeqStPerS<S>;

    pub trait Exercise21_5Trait {
        /// Exercise 21.5: Generate all contiguous subsequences using nested tabulate + flatten
        /// APAS: Work Θ(n²), Span Θ(lg n)
        fn all_contiguous_subseqs<T: StT>(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<ArraySeqStPerS<T>>;
    }

    /// Exercise 21.5: Generate all contiguous subsequences using nested tabulate + flatten.
    ///
    /// gpt-5-hard: Work: Θ(n²), Span: Θ(lg n)
    /// APAS: Work: Θ(n²), Span: Θ(lg n)
    pub fn all_contiguous_subseqs<T: StT>(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<ArraySeqStPerS<T>> {
        let n = a.length();
        let nested: ArraySeqStPerS<ArraySeqStPerS<ArraySeqStPerS<T>>> =
            ArraySeqStPerS::tabulate(
                &|i| {
                    ArraySeqStPerS::tabulate(
                        &|j| a.subseq_copy(i, j + 1),
                        n - i,
                    )
                },
                n,
            );
        // flatten twice
        let mid: ArraySeqStPerS<ArraySeqStPerS<T>> =
            ArraySeqStPerS::flatten(&nested);
        mid
    }
}
