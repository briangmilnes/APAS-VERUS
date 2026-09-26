# r228 Alg Analysis Review: Chap28

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

From `prompts/Chap28.txt` (Maximum Contiguous Subsequence Sum), with array
sequences.

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | §2 | reduction MCSS to MCS | O(n) | O(lg n) |
| 2 | Alg 28.6 | MCS brute force | Θ(n³) | Θ(lg n) |
| 3 | Alg 28.7/28.8 | MCSS brute force (strengthened) | Θ(n³) | Θ(lg n) |
| 4 | Alg 28.11 | MCSSS optimal (scan + reduce) | Θ(n − i) | Θ(lg(n − i)) |
| 5 | Alg 28.12 | MCSSE optimal (scan + reduce) | Θ(j) | Θ(lg j) |
| 6 | Alg 28.13 | MCSS reduced force (to MCSSS) | Θ(n²) | Θ(lg n) |
| 7 | Alg 28.14 | MCSS by reduction to MCSSE | O(n²) | O(lg n) |
| 8 | Alg 28.15 | MCSS with iteration (Kadane) | O(n) | O(n) |
| 9 | Alg 28.16 | MCSS work optimal, low span | Θ(n) | Θ(lg n) |
| 10 | Alg 28.17 | simple D&C MCSS | Θ(n lg n) | Θ(lg² n) |
| 11 | Alg 28.18 | bestAcross (MCSSS + MCSSE) | Θ(n) | Θ(lg n) |
| 12 | Alg 28.19 | linear-work D&C MCSS | O(n) | O(lg² n) |

Alg 28.19's span recurrence is S(n) = S(n/2) + O(lg n) because of
`splitMid`; the prose states "the span is the same as before", i.e.
O(lg² n).

## 2. Reviewed functions

T = trait declaration, I = impl, F = free function. NT = does not match
textbook; Old = does not match old analysis; NC = no textbook cost.
`max_with_neginf` (F) appears in eight files; every copy is W 1, S 1, NC,
and matches its old line. Those eight rows are merged into row 1.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 28 | (8 files) | max_with_neginf (F) | none [1] | 1, 1 | W 1, S 1 | NC |
| 2 | 28 | ...BruteStEph.rs | max_contig_sub_sum_brute (T) | 28.6: n³, lg n | n³, n³ | W n², S n² | NT; Old [2] |
| 3 | 28 | ...BruteStEph.rs | max_contig_sub_sum_brute (I) | 28.6: n³, lg n | n², n² | W n², S n² | NT [2] |
| 4 | 28 | ...ReducedStEph.rs | max_contig_sub_sum_reduced (T,I) | 28.13 [3] | n, n | W n², S n² | NT; Old [3] |
| 5 | 28 | ...ReducedMcsseStEph.rs | ..._reduced_mcsse (T) | 28.14: n², lg n | n², n² | W n², S n² | NT: sequential |
| 6 | 28 | ...ReducedMcsseStEph.rs | ..._reduced_mcsse (I) | 28.14: n², lg n | n, n | W n², S n² | NT; Old [4] |
| 7 | 28 | ...IterStEph.rs | max_contig_sub_sum_iter (T,I) | 28.15: n, n | n, n | W n, S n | matches textbook |
| 8 | 28 | ...OptStEph.rs | max_contig_sub_sum_opt (T,I) | 28.16: n, lg n | n, n | W n, S n | NT: seq loops |
| 9 | 28 | ...OptMtEph.rs | ..._opt_mt (T) | 28.16: n, lg n | n, lg n | W n, S n | NT; Old [5] |
| 10 | 28 | ...OptMtEph.rs | ..._opt_mt (I) | 28.16: n, lg n | n, n | W n, S n | NT: seq loops |
| 11 | 28 | ...DivConStEph.rs | ..._divcon (T,I) | 28.17: n lg n, lg² n | n lg n, n lg n | W n lg n, S n lg n | NT: sequential |
| 12 | 28 | ...DivConStEph.rs | max_suffix_sum, max_prefix_sum (F) | 28.18: n, lg n | n, n | W n, S n | NT: seq loop |
| 13 | 28 | ...DivConMtEph.rs | ..._divcon_mt (T) | 28.17: n lg n, lg² n | n lg n, lg² n | W n lg n, S n lg n | NT; Old [6] |
| 14 | 28 | ...DivConMtEph.rs | ..._divcon_mt (I) | 28.17: n lg n, lg² n | n lg n, n | W n lg n, S n lg n | NT; Old [6] |
| 15 | 28 | ...DivConMtEph.rs | max_suffix_sum, max_prefix_sum (F) | 28.18: n, lg n | n, n | W n, S n | NT: seq loop |
| 16 | 28 | ...DivConOptStEph.rs | ..._divcon_opt (T,I) | 28.19: n, lg² n | n, n | W n lg n, S n lg n | NT; Old [7] |
| 17 | 28 | ...DivConOptStEph.rs | max_contig_sub_sum_aux (F) | 28.19: n, lg² n | n lg n, n lg n | W n lg n, S n lg n | NT [7] |
| 18 | 28 | ...DivConOptMtEph.rs | ..._divcon_opt_mt (T,I) | 28.19 [8] | n, lg n | W n lg n, S n lg n | NT; Old [7][8] |
| 19 | 28 | ...DivConOptMtEph.rs | max_contig_sub_sum_aux (F) | 28.19 [8] | n lg n, n | W n lg n, S n lg n | NT; Old [7] |

`...` abbreviates `MaxContigSubSum` in file names and
`max_contig_sub_sum` in function names.

### Footnotes

[1] Each copy carries an APAS line citing the file's algorithm with
W O(1), S O(1); the prose gives no cost for this comparison helper.

[2] Documented as Algorithm 28.8 but implemented with two nested loops that
keep a running sum for each start position, so no per-subsequence reduce
runs: Work O(n²), not O(n³). This is the work of Alg 28.13, not 28.6/28.8.
All loops are sequential. The old trait line claimed "triple-nested
sequential loops" and O(n³).

[3] The trait documents Algorithm 28.13 (reduced force, W Θ(n²),
S Θ(lg n)), but its APAS line cites Algorithm 28.16 (W O(n), S O(lg n)).
The body is two nested sequential loops (one MCSSS per start, computed by a
running sum), so W O(n²), S O(n²). The old lines on both trait and impl
claimed a single pass, W O(n).

[4] For each end position j the body reruns a prefix-sum/min-prefix loop
over [0, j), so Work is Σ j = O(n²). The old impl line claimed a single
pass, W O(n).

[5] The Mt file has no `join`; the three phases (prefix sums, running
minimum, max difference) are sequential loops. The impl's old line says so;
the trait's old line claimed Span O(lg n).

[6] `MaxContigSubSumDivConMtEph.rs` contains no `join`: the two recursive
calls run one after the other, the halves are copied with the sequential
`subseq_copy`, and the suffix/prefix helpers are sequential loops. Span
equals Work, O(n lg n). The old lines claimed S O(lg² n) (trait) and
S O(n) "via join" (impl).

[7] Both linear-work D&C files split with `subseq_copy`, which copies O(n)
elements per level, so W(n) = 2W(n/2) + O(n) = O(n lg n) instead of the
textbook O(n) (which assumes an O(lg n) split). The recursion is sequential
in both files (the Mt file has no `join`), so Span is O(n lg n). The old
lines claimed W O(n) on the top-level functions, and S O(lg n) or O(n) on
the Mt file.

[8] The Mt file's APAS lines say S O(lg n); the textbook gives S O(lg² n)
for Alg 28.19 (the St file's APAS line has it right). The new line is
compared against O(lg² n).

## 3. Counts (per annotation site)

34 new lines were added, one per annotated function site (trait and impl
counted separately), in 10 files. `MCSSSpec.rs` has only spec and proof
functions and no annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 2 |
| 2 | does not match textbook | 24 |
| 3 | does not match old analysis | 12 |
| 4 | no textbook cost | 8 |
| 5 | unannotated functions | 0 |
| 6 | malformed annotations | 3 |

Rows 1, 2, and 4 partition the 34 lines; row 3 overlaps them.

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 28 | MaxContigSubSumBruteStEph.rs | 3 | 0 | 2 | 1 | 1 |
| 2 | 28 | MaxContigSubSumReducedStEph.rs | 3 | 0 | 2 | 2 | 1 |
| 3 | 28 | MaxContigSubSumReducedMcsseStEph.rs | 3 | 0 | 2 | 1 | 1 |
| 4 | 28 | MaxContigSubSumIterStEph.rs | 3 | 2 | 0 | 0 | 1 |
| 5 | 28 | MaxContigSubSumOptStEph.rs | 2 | 0 | 2 | 0 | 0 |
| 6 | 28 | MaxContigSubSumOptMtEph.rs | 2 | 0 | 2 | 1 | 0 |
| 7 | 28 | MaxContigSubSumDivConStEph.rs | 5 | 0 | 4 | 0 | 1 |
| 8 | 28 | MaxContigSubSumDivConMtEph.rs | 5 | 0 | 4 | 2 | 1 |
| 9 | 28 | MaxContigSubSumDivConOptStEph.rs | 4 | 0 | 3 | 2 | 1 |
| 10 | 28 | MaxContigSubSumDivConOptMtEph.rs | 4 | 0 | 3 | 3 | 1 |

## 4. Unannotated functions (0)

Every exec function in Chap28 carries an annotation.

## 5. Malformed annotations (3)

Three trait doc blocks carry an extra, non-standard analysis line after the
two `Alg Analysis` lines. The new review line was placed after the last
`Alg Analysis` line, before the extra line; the extra lines were left
unchanged.

| # | Chap | File | Function | Extra line |
|---|---|---|---|---|
| 1 | 28 | MaxContigSubSumOptMtEph.rs | ..._opt_mt (T) | `Claude-Opus-4.6 (verified): W Θ(n), S Θ(n)` |
| 2 | 28 | MaxContigSubSumDivConMtEph.rs | ..._divcon_mt (T) | `... (verified): W Θ(n lg n), S Θ(n lg n)` |
| 3 | 28 | MaxContigSubSumDivConOptMtEph.rs | ..._divcon_opt_mt (T) | `... (verified): W Θ(n lg n), S Θ(n)` |

The first two agree with the new review; the third has the right work but
S Θ(n) where the sequential recursion gives S Θ(n lg n).

## 6. Notable findings

1. **None of the three Mt files is parallel.** `MaxContigSubSumOptMtEph.rs`,
   `MaxContigSubSumDivConMtEph.rs`, and `MaxContigSubSumDivConOptMtEph.rs`
   contain no `join`; their bodies are the St algorithms over
   `ArraySeqMtEphS`. The old lines on the two D&C files claimed
   "parallel halves via join" with Span O(n), O(lg n), or O(lg² n).
2. **Old lines understated the work of three algorithms by a factor of n.**
   `MaxContigSubSumReducedStEph.rs` and `MaxContigSubSumReducedMcsseStEph.rs`
   were described as single-pass O(n); both are O(n²) nested loops, as the
   textbook's Algs 28.13 and 28.14 are. The brute-force file is the
   reverse: it is O(n²) (running sums), not the O(n³) of Alg 28.6/28.8 that
   its trait line claimed.
3. **The linear-work D&C (Alg 28.19) is O(n lg n) work here.** Both
   `MaxContigSubSumDivConOpt*` files split with `subseq_copy`, an O(n)
   copy per level, so the strengthening that makes Alg 28.19 linear is
   cancelled by the split. The old top-level lines claimed O(n).
4. Only `MaxContigSubSumIterStEph.rs` (Kadane, Alg 28.15) matches the
   textbook, since the textbook's algorithm is itself sequential.
5. APAS-line errors: `MaxContigSubSumReducedStEph.rs` cites Alg 28.16 for
   an Alg 28.13 implementation, and `MaxContigSubSumDivConOptMtEph.rs`
   gives Alg 28.19 a span of O(lg n) instead of O(lg² n).
6. The textbook's MCSSS and MCSSE optimal algorithms (28.11, 28.12) have no
   standalone implementation; they appear only inlined as sequential loops
   (`max_suffix_sum`, `max_prefix_sum`, and the inner loop of the MCSSE
   reduction).
