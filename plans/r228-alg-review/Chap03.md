# r228 Alg Analysis Review: Chap03

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap03.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Ex 3.1 insertion sort (insSort) | none in prose [1] |

1. The prose gives only the SPARC definition of `insSort`. The file's APAS
   line records the standard Work O(n^2), Span O(n^2); that is the reference
   used here.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 03 | InsertionSortStEph.rs | insertion_sort | W n^2, S n^2 | W n^2, S n^2 | W n^2, S n^2 | matches textbook |

The outer loop runs n - 1 times; the inner swap loop runs at most `up`
times, each iteration O(1). Best case (sorted input) is O(n) because the
inner loop exits after one comparison.

## 3. Counts

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 1 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 0 |
| 4 | no textbook cost | 0 |
| 5 | unannotated functions | 0 |
| 6 | malformed annotations | 0 |

## 4. Notable findings

None. The implementation is the in-place iterative variant of the textbook's
recursive `insSort`; the costs agree.
