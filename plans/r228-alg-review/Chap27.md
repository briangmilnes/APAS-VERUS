# r228 Alg Analysis Review: Chap27

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

From `prompts/Chap27.txt` (Contraction). All costs assume f has O(1)
work and span.

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Ex 27.1 | max element by contraction | Θ(n) | Θ(lg n) |
| 2 | Alg 27.2 | reduceContract | O(n) | O(lg n) |
| 3 | Alg 27.2 | contraction step (tabulate) | O(n) | O(1) |
| 4 | Alg 27.3 | scan by contraction | O(n) | O(lg n) |
| 5 | Alg 27.3 | expansion step (tabulate) | O(n) | O(1) |
| 6 | §3 | scan by iterate | O(n) | O(n) (sequential) |

## 2. Reviewed functions

T = trait declaration, I = impl, F = free function. NT = does not match
textbook; Old = does not match old analysis.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 27 | ReduceContractStEph.rs | reduce_contract (T,I) | n, lg n | n, n | W n, S n | NT: seq contraction |
| 2 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel (T) | n, lg n | n, n | W n, S n | NT [1] |
| 3 | 27 | ReduceContractMtEph.rs | contract_parallel (F) | n, 1 [2] | n, n/2 | W n, S n | NT [1] |
| 4 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel (I) | n, lg n | n, lg n | W n, S n | NT; Old [1] |
| 5 | 27 | ScanContractStEph.rs | scan_contract (T,I) | n, lg n | n, n | W n, S n | NT: seq loops |
| 6 | 27 | ScanContractStEph.rs | expand_scan (T,I) | n, 1 | n, n | W n, S n | NT: seq loop |
| 7 | 27 | ScanContractMtEph.rs | scan_contract_parallel (T) | n, lg n | n, n | W n, S n | NT [1][3] |
| 8 | 27 | ScanContractMtEph.rs | scan_contract_parallel (I) | n, lg n | n, lg² n | W n, S n | NT; Old [3] |
| 9 | 27 | ScanContractMtEph.rs | expand_scan_parallel (T) | n, 1 | n, n | W n, S n | NT: seq loop |
| 10 | 27 | ScanContractMtEph.rs | expand_scan_parallel (I) | n, 1 | n, lg n | W n, S n | NT; Old [3] |

### Footnotes

[1] `contract_parallel` clones the input sequence (sequential O(n)), forks
exactly once, fills each quarter-size half with a sequential push loop, and
joins the halves with `Vec::append`. Its span is O(n), so the contraction
recursion has S(n) = S(n/2) + O(n) = O(n), not O(lg n). The old impl line
for `reduce_contract_parallel` claimed Span O(lg n).

[2] The function has no APAS line; the textbook states the contraction
step's cost (parallel tabulate, O(n) work, O(1) span), so it is compared
against that.

[3] `expand_scan_parallel` is a sequential `while` loop that pushes two
values per pair; it does not use tabulate. The old impl lines claimed
Span O(lg n) for it and Span O(lg² n) for `scan_contract_parallel`.

## 3. Counts (per annotation site)

13 new lines were added, one per annotated function site (trait and impl
counted separately), in 4 files. `ContractSpecsAndLemmas.rs` has only
proof functions and no annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 13 |
| 3 | does not match old analysis | 3 |
| 4 | no textbook cost | 0 |
| 5 | unannotated functions | 0 |
| 6 | malformed annotations | 0 |

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 27 | ReduceContractStEph.rs | 2 | 0 | 2 | 0 | 0 |
| 2 | 27 | ReduceContractMtEph.rs | 3 | 0 | 3 | 1 | 0 |
| 3 | 27 | ScanContractStEph.rs | 4 | 0 | 4 | 0 | 0 |
| 4 | 27 | ScanContractMtEph.rs | 4 | 0 | 4 | 2 | 0 |

## 4. Unannotated functions (0)

None among exec functions in Chap27. (`call_f`, used by the Mt files, is
defined outside Chap27.)

## 5. Malformed annotations (0)

None.

## 6. Notable findings

1. **The Mt contraction files have linear span.** Both
   `ReduceContractMtEph.rs` and `ScanContractMtEph.rs` contract through
   `contract_parallel`, which clones the whole input and forks only once
   into two sequential loops. Each recursion level therefore costs O(n)
   span, and the whole algorithm is Span O(n) rather than the textbook
   O(lg n). A parallel tabulate (or recursive fork down to a grain) over the
   pairs would restore O(lg n).
2. **The Mt scan's expansion is sequential, and its old lines said
   otherwise.** `expand_scan_parallel` is a plain `while` loop; the old impl
   line described it as "parallel expand via tabulate" with Span O(lg n),
   and the old `scan_contract_parallel` impl line claimed Span O(lg² n).
3. The work of every function matches the textbook O(n); the only
   difference in the chapter is span, in the categories "St sequential" and
   "Mt sequential loops".
