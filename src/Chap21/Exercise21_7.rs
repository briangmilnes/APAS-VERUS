//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.7: Comprehension with Conditionals - even elements paired with vowels.

pub mod Exercise21_7 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Exercise21_7Trait {
        /// Check if a number is even
        /// APAS: Work Θ(1), Span Θ(1)
        fn is_even(x: &N)                                                         -> B;

        /// Check if a character is a vowel (case-insensitive)
        /// APAS: Work Θ(1), Span Θ(1)
        fn is_vowel(c: &char)                                                     -> B;

        /// Exercise 21.7: Comprehension with conditionals
        /// APAS: Work Θ(|a|·|b|), Span Θ(lg |a|)
        fn pair_even_with_vowels(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<char>) -> ArraySeqStPerS<Pair<N, char>>;
    }

    /// Check if a number is even
    pub fn is_even(x: &N) -> B { *x % 2 == 0 }

    /// Check if a character is a vowel (case-insensitive)
    pub fn is_vowel(c: &char) -> B {
        match *c {
            | 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            | _ => false,
        }
    }

    /// Exercise 21.7: Comprehension with conditionals
    /// flatten 〈 〈 (x, y) : y ∈ b | isVowel y 〉 : x ∈ a | isEven x 〉
    ///
    /// Pairs even elements of sequence a with vowels of sequence b.
    /// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
    /// APAS: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
    pub fn pair_even_with_vowels(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<char>) -> ArraySeqStPerS<Pair<N, char>> {
        let filtered_a = ArraySeqStPerS::filter(a, &|x| is_even(x));
        let filtered_b = ArraySeqStPerS::filter(b, &|y| is_vowel(y));

        let nested = ArraySeqStPerS::tabulate(
            &|i| {
                let x = filtered_a.nth(i);
                ArraySeqStPerS::tabulate(
                    &|j| Pair(*x, *filtered_b.nth(j)),
                    filtered_b.length(),
                )
            },
            filtered_a.length(),
        );
        ArraySeqStPerS::flatten(&nested)
    }
}
