//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.7: Comprehension with Conditionals - even elements paired with vowels.
//! Verusified.

pub mod Exercise21_7 {

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
        crate::Types::Types::group_Pair_axioms,
    };

    /// Spec: x is even.
    pub open spec fn spec_is_even(x: int) -> bool { x % 2 == 0 }

    /// Check if a number is even. Work Θ(1), Span Θ(1).
    pub fn is_even(x: &N) -> (r: B)
        ensures r == spec_is_even(*x as int)
    { *x % 2 == 0 }

    /// Spec: c is a vowel.
    pub open spec fn spec_is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }

    /// Check if a character is a vowel (case-insensitive). Work Θ(1), Span Θ(1).
    pub fn is_vowel(c: &char) -> (r: B)
        ensures r == spec_is_vowel(*c)
    {
        match *c {
            | 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            | _ => false,
        }
    }

    /// Exercise 21.7: Comprehension with conditionals
    /// flatten 〈 〈 (x, y) : y ∈ b | isVowel y 〉 : x ∈ a | isEven x 〉
    ///
    /// Pairs even elements of a with vowels of b.
    /// APAS: Work Θ(|a|·|b|), Span Θ(lg |a|)
    pub fn pair_even_with_vowels(
        a: &ArraySeqStPerS<N>,
        b: &ArraySeqStPerS<char>,
    ) -> (result: ArraySeqStPerS<Pair<N, char>>)
       requires 
            obeys_feq_clone::<char>(),
            obeys_feq_clone::<Pair<N, char>>()
    {
        let pred_even = |x: &N| -> (r: B) ensures r == spec_is_even(*x as int) { is_even(x) };
        let pred_vowel = |y: &char| -> (r: B) ensures r == spec_is_vowel(*y) { is_vowel(y) };
        let ghost spec_even: spec_fn(N) -> bool = |x: N| spec_is_even(x as int);
        let ghost spec_vowel: spec_fn(char) -> bool = |c: char| spec_is_vowel(c);
        let filtered_a: ArraySeqStPerS<N> = ArraySeqStPerS::filter(a, &pred_even, Ghost(spec_even));
        let filtered_b: ArraySeqStPerS<char> = ArraySeqStPerS::filter(b, &pred_vowel, Ghost(spec_vowel));

        let fa_len = filtered_a.length();
        let fb_len = filtered_b.length();

        let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<N, char>>> = ArraySeqStPerS::tabulate(
            &(|i: usize| -> (row: ArraySeqStPerS<Pair<N, char>>)
                requires
                    i < fa_len,
                    fa_len == filtered_a.seq@.len(),
                    fb_len == filtered_b.seq@.len(),
            {
                let x = filtered_a.nth(i);
                ArraySeqStPerS::tabulate(
                    &(|j: usize| -> (p: Pair<N, char>)
                        requires
                            j < fb_len,
                            fb_len == filtered_b.seq@.len(),
                    {
                        Pair(*x, *filtered_b.nth(j))
                    }),
                    fb_len,
                )
            }),
            fa_len,
        );
        ArraySeqStPerS::flatten(&nested)
    }

    } // verus!
}
