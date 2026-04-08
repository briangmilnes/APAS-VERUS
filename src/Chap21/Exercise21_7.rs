//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 21 — Exercise 21.7: Comprehension with Conditionals - even elements paired with vowels.
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 9. impls

//		Section 1. module

pub mod Exercise21_7 {


    //		Section 2. imports

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq::seq::lemma_flatten_uniform_len;

    verus! 
{


    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    //		Section 6. spec fns


    /// Spec: x is even.
    pub open spec fn spec_is_even(x: int) -> bool { x % 2 == 0     }

    /// Spec: c is a vowel.
    pub open spec fn spec_is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }

    //		Section 9. impls


    /// Check if a number is even.
    /// - Alg Analysis: APAS (Ch21 Ex 21.7): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    // veracity: no_requires
    pub fn is_even(x: &usize) -> (r: bool)
        ensures r == spec_is_even(*x as int)
    { *x % 2 == 0 }

    /// Check if a character is a vowel (case-insensitive).
    /// - Alg Analysis: APAS (Ch21 Ex 21.7): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
    // veracity: no_requires
    pub fn is_vowel(c: &char) -> (r: bool)
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
    /// - Alg Analysis: APAS (Ch21 Ex 21.7): Work O(|a|·|b|), Span O(lg |a|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|·|b|), Span O(|a|·|b|) — sequential StPer filter + tabulate + flatten.
    pub fn pair_even_with_vowels(
        a: &ArraySeqStPerS<usize>,
        b: &ArraySeqStPerS<char>,
    ) -> (pairs: ArraySeqStPerS<Pair<usize, char>>)
       requires 
            obeys_feq_clone::<char>(),
            obeys_feq_clone::<Pair<usize, char>>(),
            a.seq@.len() as int * b.seq@.len() as int <= usize::MAX as int,
       ensures
            pairs.seq@.len() <= a.seq@.len() as int * b.seq@.len() as int,
    {
        let pred_even = |x: &usize| -> (r: bool) ensures r == spec_is_even(*x as int) { is_even(x) };
        let pred_vowel = |y: &char| -> (r: bool) ensures r == spec_is_vowel(*y) { is_vowel(y) };
        let ghost spec_even: spec_fn(usize) -> bool = |x: usize| spec_is_even(x as int);
        let ghost spec_vowel: spec_fn(char) -> bool = |c: char| spec_is_vowel(c);
        let filtered_a: ArraySeqStPerS<usize> = ArraySeqStPerS::filter(a, &pred_even, Ghost(spec_even));
        let filtered_b: ArraySeqStPerS<char> = ArraySeqStPerS::filter(b, &pred_vowel, Ghost(spec_vowel));

        let fa_len = filtered_a.length();
        let fb_len = filtered_b.length();

        let nested: ArraySeqStPerS<ArraySeqStPerS<Pair<usize, char>>> = ArraySeqStPerS::tabulate(
            &(|i: usize| -> (row: ArraySeqStPerS<Pair<usize, char>>)
                requires
                    i < fa_len,
                    fa_len == filtered_a.seq@.len(),
                    fb_len == filtered_b.seq@.len(),
                ensures
                    row.seq@.len() == fb_len,
            {
                let x = filtered_a.nth(i);
                ArraySeqStPerS::tabulate(
                    &(|j: usize| -> (p: Pair<usize, char>)
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
        let pairs = ArraySeqStPerS::flatten(&nested);
        // Veracity: NEEDED proof block
        proof {
            let ghost mapped = nested.seq@.map_values(
                |inner: ArraySeqStPerS<Pair<usize, char>>| inner.seq@);
            // Veracity: NEEDED assert
            assert forall|i: int| 0 <= i < mapped.len() implies
                (#[trigger] mapped[i]).len() == fb_len as int by {}
            lemma_flatten_uniform_len(mapped, fb_len as int);
            // Veracity: NEEDED assert
            assert(fa_len as int * fb_len as int <= a.seq@.len() as int * b.seq@.len() as int)
                by (nonlinear_arith)
                requires
                    fa_len as int >= 0, fb_len as int >= 0,
                    fa_len as int <= a.seq@.len() as int,
                    fb_len as int <= b.seq@.len() as int;
        }
        pairs
    }

    } // verus!
}
