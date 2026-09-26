# r228 Alg Analysis Review: Chap21

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap21.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Prob 21.1 / Alg 21.1 points2D (flatten of tabulates) | Ex 21.2 asks; implied W n², S lg n |
| 2 | Prob 21.3 / Alg 21.2 points3D | none stated; implied W n³, S lg n |
| 3 | Prob 21.4 / Alg 21.3 Cartesian product | none stated; implied W \|a\|·\|b\|, S lg \|a\| |
| 4 | Ex 21.5 / Ex 21.6 all contiguous subsequences | W O(\|a\|²), S O(lg \|a\|) |
| 5 | Ex 21.7 comprehension with conditionals | none stated |
| 6 | Alg 21.4 isPrime (brute force) | W O(√n), S O(lg n) |
| 7 | Alg 21.5 primesBF | W O(n^(3/2)), S O(lg n) |
| 8 | Ex 21.9 composites via multiples up to √n | proof exercise, no cost |
| 9 | Alg 21.6 primeSieve | W O(n lg n), S O(lg n) |

The existing APAS lines for Problem 21.1, 21.3, and the loop form of 21.4
state Span equal to Work ("sequential due to imperative loops"). These spans
are not from the prose; the prose's own algorithms (21.1, 21.2, 21.3) have
logarithmic span. Following the plan, those sites are compared against their
APAS line.

## 2. Reviewed functions

W = Work, S = Span. Proof functions have "N/A" old lines; the new line records
Work O(0), Span O(0) (erased before execution).

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 21 | Algorithm21_1.rs | lemma_sum_inner_lens_mono | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 2 | 21 | Algorithm21_1.rs | lemma_sum_inner_lens_uniform | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 3 | 21 | Algorithm21_1.rs | flatten_inner | W m, S lg k | W m, S m | W k+m, S k+m | not tb: sequential |
| 4 | 21 | Algorithm21_1.rs | points2d_tab_flat | W n², S lg n | W n², S n² | W n², S n² | not tb: sequential |
| 5 | 21 | Algorithm21_2.rs | points3d_tab_flat | W n³, S lg n | W n³, S n³ | W n³, S n³ | not tb: sequential |
| 6 | 21 | Algorithm21_5.rs | primes_bf | W n^1.5, S lg n | W n^1.5, S n^1.5 | W n^1.5, S n^1.5 | not tb: sequential |
| 7 | 21 | Algorithm21_6.rs | prime_sieve | W n lg n, S lg n | W n lg n, S n lg n | W n lg n, S n lg n | not tb: sequential [1] |
| 8 | 21 | Exercise21_5.rs | all_contiguous_subseqs | W n², S lg n | W n³, S n³ | W n³, S n³ | not tb: copying subseq [2] |
| 9 | 21 | Exercise21_7.rs | is_even | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 10 | 21 | Exercise21_7.rs | is_vowel | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 11 | 21 | Exercise21_7.rs | pair_even_with_vowels | W ab, S lg a | W ab, S ab | W a+b+ab, S same | not tb: sequential |
| 12 | 21 | Exercise21_8.rs | lemma_zero_count_means_no_divisors | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 13 | 21 | Exercise21_8.rs | lemma_no_divisors_means_zero_count | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 14 | 21 | Exercise21_8.rs | lemma_divisor_count_nonneg | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 15 | 21 | Exercise21_8.rs | is_divisible | W 1, S 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 16 | 21 | Exercise21_8.rs | is_prime | W √n, S lg n | W √n, S √n | W √n, S √n | not tb: sequential |
| 17 | 21 | Exercise21_9.rs | lemma_composite_has_small_divisor | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 18 | 21 | Exercise21_9.rs | lemma_composites_covered_by_small_multiples | none | N/A (proof) | W 0, S 0 | no textbook cost |
| 19 | 21 | Problem21_1.rs | points2d | W n², S n² [3] | W n², S n² | W n², S n² | matches textbook |
| 20 | 21 | Problem21_3.rs | points3d_loops | W n³, S n³ [3] | W n³, S n³ | W n³, S n³ | matches textbook |
| 21 | 21 | Problem21_4.rs | cartesian_loops | W ab, S ab [3] | W ab, S ab | W ab, S ab | matches textbook |
| 22 | 21 | Problem21_4.rs | cartesian_tab_flat | W ab, S lg a | W ab, S ab | W ab, S ab | not tb: sequential |

Footnotes:

1. `prime_sieve` generates the m = O(n lg n) composites with a nested Chap19
   StPer tabulate, flattens them, marks the sieve in a sequential loop (the
   textbook's `ninject`), and collects primes in a sequential loop. Work
   matches the textbook; every phase is sequential, so Span = Work.
2. `all_contiguous_subseqs` calls `subseq_copy(i, j + 1)`, which copies j + 1
   elements, for each of the n(n+1)/2 pairs, and the Chap19 flatten then
   clones each subsequence (a Vec clone). Both give Θ(n³) work. The textbook
   solution (Ex 21.6) relies on O(1) `subseq` for Work O(n²).
3. APAS line with Span = Work for the imperative loop form; the prose gives
   no cost for these loop forms (see section 1).

## 3. Counts (per annotation site = lines added)

| # | Measure | Count |
|---|---|---|
| 1 | lines added | 22 |
| 2 | matches textbook | 6 |
| 3 | does not match textbook | 9 |
| 4 | does not match old analysis | 0 |
| 5 | no textbook cost | 7 |
| 6 | unannotated functions | 0 exec (5 proof lemmas) |
| 7 | malformed annotations | 0 |

Unannotated functions: no exec function lacks an annotation. Five proof
lemmas carry none, which is consistent with their zero runtime cost:

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 21 | Algorithm21_6.rs | lemma_product_not_prime |
| 2 | 21 | Exercise21_5.rs | lemma_inner_lens_sum_triangular |
| 3 | 21 | Exercise21_8.rs | lemma_filter_len_eq_divisor_count, lemma_divisor_count_split_last |
| 4 | 21 | Exercise21_9.rs | lemma_div_exact |

`Exercise21_6.rs` has no code (analysis exercise).

Malformed: none. Placement notes: in `Exercise21_7.rs` (`is_even`,
`is_vowel`) and `Exercise21_8.rs` (`is_prime`), a `// veracity: no_requires`
line sits between the doc block and the `fn`; the new line was placed inside
the doc block after the last `Alg Analysis` line.

## 4. Notable findings

- Every sequence-based algorithm in the chapter (points2D/3D, Cartesian
  product, primesBF, isPrime, primeSieve, Ex 21.7) is built on the St
  (sequential) array sequences of Chap18/Chap19, so Span equals Work
  throughout. The textbook's point in this chapter is the logarithmic span of
  tabulate/flatten/filter; none of the implementations shows it. An Mt
  variant would be needed to match.
- Cost regression against APAS: Ex 21.5 is Θ(n³), not O(n²), because
  `subseq_copy` copies and the flatten clones each subsequence (footnote 2).
  With an O(1) slice (as in `Chap19/ArraySeqMtEphSlice.rs`) the work would
  match.
- `prime_sieve` replaces `ninject` with a sequential marking loop; work still
  matches O(n lg n).
- The APAS lines on the three imperative "Problem" functions state Span =
  Work; that is the reviewer's own statement, not the textbook's.
- All old Code-review lines agree with the new analysis.
