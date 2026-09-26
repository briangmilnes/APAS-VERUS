# r228 Alg Analysis Review: Chap17

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap17.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Def 17.1 sequence as a function | none |
| 2 | Syntax 17.2 indexing, subsequence | none |
| 3 | Syntax 17.3 pairs and strings | none |

Chapter 17 is the introduction to the sequence part. It announces array,
tree, and list cost specifications but defers them to later chapters
(Chap18/19). No function in `MathSeq.rs` has a textbook cost, and the file
has no APAS lines, so every verdict is "no textbook cost".

## 2. Reviewed functions

`MathSeqS` is a `Vec`-backed sequence. Each trait function has an
annotation on the trait declaration (T) and on the impl (I); both received a
line. W = Work, S = Span.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 17 | MathSeq.rs | new (T,I) | none | O(n) / W n, S n | W n, S n | no textbook cost |
| 2 | 17 | MathSeq.rs | set (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 3 | 17 | MathSeq.rs | length (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 17 | MathSeq.rs | nth (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 17 | MathSeq.rs | empty (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 17 | MathSeq.rs | singleton (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 7 | 17 | MathSeq.rs | add_last (T,I) | none | am. O(1) | W 1 am., S 1 am. | no textbook cost |
| 8 | 17 | MathSeq.rs | delete_last (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 9 | 17 | MathSeq.rs | is_empty (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 10 | 17 | MathSeq.rs | is_singleton (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 11 | 17 | MathSeq.rs | from_vec (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 17 | MathSeq.rs | with_len (T,I) | none | O(n) / W n, S n | W n, S n | no textbook cost |
| 13 | 17 | MathSeq.rs | subseq (T,I) | none | O(1) / W 1, S 1 | W 1, S 1 | no textbook cost |
| 14 | 17 | MathSeq.rs | subseq_copy (T,I) | none | O(len) / W k, S k | W len, S len | no textbook cost |
| 15 | 17 | MathSeq.rs | domain (T,I) | none | O(n) / W n, S n | W n, S n | no textbook cost |
| 16 | 17 | MathSeq.rs | range (T,I) | none | O(n) exp / W n, S n | W n exp, S n | no textbook cost |
| 17 | 17 | MathSeq.rs | multiset_range (T,I) | none | O(n) exp / W n, S n | W n exp, S n | no textbook cost |
| 18 | 17 | MathSeq.rs | iter (T) | none | O(1) | W 1, S 1 | no textbook cost |
| 19 | 17 | MathSeq.rs | iter_mut | none | O(1) | W 1, S 1 | no textbook cost |

All old analyses agree with the new ones. `new` and `with_len` build the
vector with `vec![init; n]`, a sequential clone fill, so span equals work;
`range` and `multiset_range` are single sequential passes with expected
O(1) hash operations.

## 3. Counts (per annotation site)

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 0 |
| 4 | no textbook cost | 36 |
| 5 | unannotated functions | 4 |
| 6 | malformed annotations | 19 |

Unannotated: `iter` (impl), `IntoIterator::into_iter` for `&mut MathSeqS`,
`Debug::fmt`, `Display::fmt`.

Malformed: the 18 trait-level old lines and the `iter_mut` line state a bare
`O(...)` with no `Work O(...), Span O(...)` form. They were left unchanged.

## 4. Notable findings

- No cost defects. The chapter has no textbook costs to compare against;
  the array-sequence cost specification that `MathSeqS` would be measured
  against is in Chap18.
- The trait-level annotations use a non-standard bare `O(...)` format.
