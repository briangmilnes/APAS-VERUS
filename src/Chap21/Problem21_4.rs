//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Problem 21.4: Cartesian Product using different approaches.

pub mod Problem21_4 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqStPerS<N>;

    pub trait Problem21_4Trait {
        /// Problem 21.4 (Cartesian Product) - Imperative approach using explicit loops
        /// APAS: Work Θ(|a|·|b|), Span Θ(|a|·|b|)
        fn cartesian_loops(
            a: &ArraySeqStPerS<N>,
            b: &ArraySeqStPerS<&'static str>,
        ) -> ArraySeqStPerS<Pair<N, &'static str>>;

        /// Cartesian product using functional approach with tabulate + flatten
        /// APAS: Work Θ(|a|·|b|), Span Θ(lg |a|)
        fn cartesian_functional(
            a: &ArraySeqStPerS<N>,
            b: &ArraySeqStPerS<&'static str>,
        ) -> ArraySeqStPerS<Pair<N, &'static str>>;
    }

    /// Problem 21.4 (Cartesian Product) - Imperative approach using explicit loops.
    /// Cartesian product by explicit loops (x-major then y).
    ///
    /// This is an educational example showing imperative style for comparison.
    /// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(|a|·|b|) (sequential due to imperative loops)
    /// APAS: Work: Θ(|a|·|b|), Span: Θ(|a|·|b|)
    pub fn cartesian_loops(
        a: &ArraySeqStPerS<N>,
        b: &ArraySeqStPerS<&'static str>,
    ) -> ArraySeqStPerS<Pair<N, &'static str>> {
        let mut v = Vec::<Pair<N, &'static str>>::with_capacity(a.length() * b.length());
        for i in 0..a.length() {
            for j in 0..b.length() {
                v.push(Pair(*a.nth(i), *b.nth(j)));
            }
        }
        ArraySeqStPerS::from_vec(v)
    }

    /// Problem 21.4 (Cartesian Product) - Functional approach using tabulate + flatten.
    /// Cartesian product using map + flatten: flatten(map(\x. map(\y. (x,y)) b) a)
    ///
    /// This shows the functional parallel approach for comparison with the imperative version.
    /// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
    /// APAS: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
    pub fn cartesian_tab_flat(
        a: &ArraySeqStPerS<N>,
        b: &ArraySeqStPerS<&'static str>,
    ) -> ArraySeqStPerS<Pair<N, &'static str>> {
        let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, &'static str>>> = ArraySeqStPerS::tabulate(
            &|i| {
                let x = *a.nth(i);
                ArraySeqStPerS::tabulate(
                        &|j| Pair(x, *b.nth(j)),
                        b.length(),
                    )
            },
            a.length(),
        );
        ArraySeqStPerS::flatten(&nested)
    }
}
