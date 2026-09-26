# r228 Alg Analysis Review: Chap30

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap30.txt` (Probability Spaces, with the start of Chapter 31,
Random Variables) is mathematical background: sample spaces, events,
probability measures, the union bound, conditional probability, the law of
total probability, independence, random variables, and PMFs. It states no
algorithm and no cost specification.

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Ch30 | (none) | — | — |

## 2. Reviewed functions

`Probability.rs` defines `Probability`, a newtype over `f64`, and its
operator and trait impls. Every function is a single f64 operation,
constructor, or formatter.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 30 | Probability.rs | new (T,I) | none | 1, 1 | W 1, S 1 | no textbook cost |
| 2 | 30 | Probability.rs | value (T,I) | none | 1, 1 | W 1, S 1 | no textbook cost |
| 3 | 30 | Probability.rs | infinity (T,I) | none | 1, 1 | W 1, S 1 | no textbook cost |
| 4 | 30 | Probability.rs | zero (T,I) | none | 1, 1 | W 1, S 1 | no textbook cost |
| 5 | 30 | Probability.rs | partial_cmp, cmp | none | 1, 1 | W 1, S 1 | no textbook cost |
| 6 | 30 | Probability.rs | from x2 | none | 1, 1 | W 1, S 1 | no textbook cost |
| 7 | 30 | Probability.rs | add, sub, mul, div | none | 1, 1 | W 1, S 1 | no textbook cost |
| 8 | 30 | Probability.rs | default, eq, hash | none | 1, 1 | W 1, S 1 | no textbook cost |
| 9 | 30 | Probability.rs | fmt x2 (Debug, Display) | none | 1, 1 | W 1, S 1 | no textbook cost |

## 3. Counts (per annotation site)

21 new lines were added, one per annotated function site, in 1 file.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 0 |
| 4 | no textbook cost | 21 |
| 5 | unannotated functions | 0 |
| 6 | malformed annotations | 0 |

## 4. Unannotated functions (0)

None. (`Eq` is an empty marker impl; the `AddSpecImpl`/`SubSpecImpl`
members are spec functions.)

## 5. Malformed annotations (0)

None. For the record, the doc blocks of `cmp`, `eq`, and `hash` are
followed by a blank line before the `#[verifier::external_body]`
attribute; the doc comment still attaches to the function. The new line
was placed directly after the existing Alg Analysis line, before the blank
line.

## 6. Notable findings

1. The chapter has no algorithmic content and no textbook costs; all 21 old
   lines (O(1)) are correct.
2. Ten of the functions are `external_body` with `// accept hole` (f64
   arithmetic, comparison, conversion, hashing); this does not affect cost.
