# r228 Alg Analysis Review: Chap26

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

From `prompts/Chap26.txt` (Divide and Conquer).

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Ex 26.2 | max element by D&C | Θ(n) | Θ(lg n) |
| 2 | Alg 26.2 | reduceDC, O(1) op | Θ(n) | Θ(lg n) |
| 3 | Alg 26.4 (assumed) | merge | Θ(n) | Θ(lg n) |
| 4 | Alg 26.4 | mergeSort | Θ(n lg n) | Θ(lg² n) |
| 5 | Alg 26.5 | scanDC | O(n lg n) | O(lg n) |
| 6 | Alg 26.7 | eTSP | O(n²) | O(lg² n) |
| 7 | Alg 26.7 | minVal over edge pairs | O(n²) [1] | O(lg n) [1] |

[1] Implied by the eTSP recurrences W(n) = 2W(n/2) + O(n²),
S(n) = S(n/2) + O(lg n). The prose gives no separate cost for the
split, for binary search, or for the distance function.

## 2. Reviewed functions

T = trait declaration, I = impl, F = free function. W = Work, S = Span.
NT = does not match textbook, Old = does not match old analysis,
NC = no textbook cost.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 26 | MergeSortStPer.rs | merge (T,I) | n, lg n | n, n | W n, S n | NT: seq two-finger |
| 2 | 26 | MergeSortStPer.rs | merge_sort (T,I) | n lg n, lg² n | n lg n, n lg n | W n lg n, S n lg n | NT: sequential |
| 3 | 26 | MergeSortMtPer.rs | merge_parallel (T) | n, lg n | n, n | W n lg n, S n lg n | NT; Old [2] |
| 4 | 26 | MergeSortMtPer.rs | merge_parallel (I) | n, lg n | n+m, lg²(n+m) | W n lg n, S n lg n | NT; Old [2] |
| 5 | 26 | MergeSortMtPer.rs | merge_sort_parallel (T,I) | n lg n, lg² n | n lg n, n | W n lg² n, S n lg n | NT; Old [3] |
| 6 | 26 | MergeSortMtPer.rs | binary_search_upper_bound (F) | none | lg n, lg n | W lg n, S lg n | NC |
| 7 | 26 | MergeSortMtPer.rs | merge_dc (F) | n, lg n | n, n | W n lg n, S n lg n | NT; Old [2] |
| 8 | 26 | DivConReduceStPer.rs | max_element (T,I) | n, lg n | n, n | W n, S n | NT: seq loop |
| 9 | 26 | DivConReduceStPer.rs | sum, product (T,I) | n, lg n | n, n | W n, S n | NT: seq fold |
| 10 | 26 | DivConReduceStPer.rs | any, all (T,I) | n, lg n | n, n | W n, S n | NT: seq fold |
| 11 | 26 | DivConReduceMtPer.rs | *_parallel x5 (T) | n, lg n | n, lg n | W n, S n | NT; Old [4] |
| 12 | 26 | DivConReduceMtPer.rs | *_parallel x5 (I) | n, lg n | n, lg n | W n, S n | NT; Old [4] |
| 13 | 26 | DivConReduceMtPer.rs | call_reduce_* x5 (F) | none | n, lg n | W n, S n | NC; Old [4] |
| 14 | 26 | ScanDCStPer.rs | scan_dc (T) | n lg n, lg n | n lg n, n lg n | W n lg n, S n lg n | NT: sequential |
| 15 | 26 | ScanDCStPer.rs | scan_dc (I) | n lg n, lg n | n, n | W n lg n, S n lg n | NT; Old [5] |
| 16 | 26 | ScanDCStPer.rs | prefix_sums_dc (T) | n lg n, lg n | n lg n, n lg n | W n lg n, S n lg n | NT: sequential |
| 17 | 26 | ScanDCStPer.rs | prefix_sums_dc (I) | n lg n, lg n | n, n | W n lg n, S n lg n | NT; Old [5] |
| 18 | 26 | ScanDCMtPer.rs | prefix_sums_dc_parallel (T) | n lg n, lg n | n lg n, n | W n lg n, S n | NT: seq Vec copies |
| 19 | 26 | ScanDCMtPer.rs | prefix_sums_dc_inner (F) | n lg n, lg n | n lg n, n | W n lg n, S n | NT: seq Vec copies |
| 20 | 26 | ScanDCMtPer.rs | prefix_sums_dc_parallel (I) | n lg n, lg n | n, lg² n | W n lg n, S n | NT; Old [6] |
| 21 | 26 | ETSPStEph.rs | etsp (T) | n², lg² n | n², n² | W n lg n, S n lg n | NT; Old [7] |
| 22 | 26 | ETSPStEph.rs | etsp_inner (F) | n², lg² n | n², n² | W n lg n, S n lg n | NT; Old [7] |
| 23 | 26 | ETSPStEph.rs | etsp (I) | n², lg² n | n² lg n, n² lg n | W n lg n, S n lg n | NT; Old [7] |
| 24 | 26 | ETSPStEph.rs | sort_and_split (F) | none [8] | n, n | W n, S n | NC |
| 25 | 26 | ETSPStEph.rs | find_best_swap (F) | n², lg n | 1, 1 | W 1, S 1 | NT: stub (0, 0) |
| 26 | 26 | ETSPMtEph.rs | distance (T,I) | none | 1, 1 | W 1, S 1 | NC |
| 27 | 26 | ETSPMtEph.rs | etsp_parallel (T) | n², lg² n | n², n² | W n lg n, S n | NT; Old [7] |
| 28 | 26 | ETSPMtEph.rs | etsp_parallel_inner (F) | n², lg² n | n², n² | W n lg n, S n | NT; Old [7] |
| 29 | 26 | ETSPMtEph.rs | etsp_parallel (I) | n², lg² n | n² lg n, n lg² n | W n lg n, S n | NT; Old [7] |
| 30 | 26 | ETSPMtEph.rs | sort_and_split (F) | none [8] | n, n | W n, S n | NC |
| 31 | 26 | ETSPMtEph.rs | find_best_swap (F) | n², lg n | 1, 1 | W 1, S 1 | NT: stub (0, 0) |
| 32 | 26 | ETSPMtEph.rs | point_distance (F) | none | 1, 1 | W 1, S 1 | NC |
| 33 | 26 | ETSPMtEph.rs | sort_and_split_impl (F) | none | n lg n, n lg n | W n lg n, S n lg n | NC |
| 34 | 26 | ETSPMtEph.rs | find_best_swap_impl (F) | n², lg n | nm, m lg n | W nm, S n + m | NT; Old [9] |
| 35 | 26 | ETSPMtEph.rs | find_best_swap_par (F) | n², lg n | nm, m lg n | W nm, S m + lg n | NT; Old [9] |

### Footnotes

[2] `merge_dc` picks the left median, binary-searches the right, and forks,
but builds the four sub-inputs with sequential `Vec` push loops and
concatenates the results with sequential push loops. Each recursion level
copies O(n) elements in total over lg n levels, so Work is O(n lg n). The
right input need not halve: on already-sorted input (all of right greater
than all of left) `pos` is 0 at every level, so the whole right input
travels down one branch and is copied at each of lg n levels, giving Span
O(n lg n). The old lines claimed Work O(n).

[3] W(n) = 2W(n/2) + O(n lg n) = O(n lg² n); S(n) = S(n/2) + O(n lg n)
= O(n lg n) with the worst-case `merge_dc` span. The split copies are also
sequential. The old lines claimed W O(n lg n), S O(n).

[4] `ArraySeqMtPerS::reduce` (Chap18) is a sequential left fold, not D&C.
Every "parallel" reduction in this file therefore has Span O(n); the old
lines said O(lg n) and "Agrees with APAS".

[5] The old impl lines said W O(n), S O(n); the recursion copies both
halves, adjusts the right prefixes, and concatenates at every level, so
W(n) = 2W(n/2) + O(n).

[6] The old impl line claimed W O(n), S O(lg² n). The inner recursion
forks correctly but the split into halves and the final concatenation are
sequential push loops, so S(n) = S(n/2) + O(n) = O(n) and
W(n) = O(n lg n).

[7] The verified `etsp_inner` / `etsp_parallel_inner` call the verified
`find_best_swap`, which is a stub returning `(0, 0)`, and `sort_and_split`,
which splits at the index midpoint instead of along the longest dimension.
No swap search runs, so W(n) = 2W(n/2) + O(n) = O(n lg n). The St version
is fully sequential (S n lg n); the Mt version forks the recursion but
splits and combines with sequential loops (S(n) = S(n/2) + O(n) = O(n)).
All old lines assumed the O(n²) search.

[8] The code carries an APAS line (W n, S n, "simplified from sort-based
split"), but the prose states no cost for the split, so it is counted as no
textbook cost.

[9] `find_best_swap_impl` (not called by the verified eTSP) clones both
tours sequentially (O(n + m) span) and calls `find_best_swap_par`. That
function forks over left-tour ranges down to leaves of at most 16 edges;
each leaf scans all m right edges sequentially. The m-scan is paid once per
root-to-leaf path, so its span is O(m + lg n), not O(m lg n).

## 3. Counts (per annotation site)

58 new lines were added, one per annotated function site (trait and impl
counted separately), in 8 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 46 |
| 3 | does not match old analysis | 31 |
| 4 | no textbook cost | 12 |
| 5 | unannotated functions | 12 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 58 lines; row 3 overlaps them.

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 26 | MergeSortStPer.rs | 4 | 0 | 4 | 0 | 0 |
| 2 | 26 | MergeSortMtPer.rs | 6 | 0 | 5 | 5 | 1 |
| 3 | 26 | DivConReduceStPer.rs | 10 | 0 | 10 | 0 | 0 |
| 4 | 26 | DivConReduceMtPer.rs | 15 | 0 | 10 | 15 | 5 |
| 5 | 26 | ScanDCStPer.rs | 4 | 0 | 4 | 2 | 0 |
| 6 | 26 | ScanDCMtPer.rs | 3 | 0 | 3 | 1 | 0 |
| 7 | 26 | ETSPStEph.rs | 5 | 0 | 4 | 3 | 1 |
| 8 | 26 | ETSPMtEph.rs | 11 | 0 | 6 | 5 | 5 |

## 4. Unannotated functions (12)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 26 | ETSPStEph.rs | ETSPPointTrait::distance (trait, impl) |
| 2 | 26 | ETSPStEph.rs | sort_and_split_impl, find_best_swap_impl |
| 3 | 26 | ETSPStEph.rs | fmt x4 (Point, Edge Debug/Display) |
| 4 | 26 | ETSPMtEph.rs | fmt x4 (Point, Edge Debug/Display) |

`ETSPStEph.rs` `sort_and_split_impl` and `find_best_swap_impl` are outside
`verus!` and unused by the verified `etsp`; for the record they cost
W O(n lg n), S O(n lg n) and W O(nm), S O(nm).

## 5. Malformed annotations (0)

None.

## 6. Notable findings

1. **The verified eTSP does not perform the eTSP swap search.** In both
   `ETSPStEph.rs` and `ETSPMtEph.rs`, the verified recursion calls
   `find_best_swap`, a stub that always returns `(0, 0)`, and splits at the
   index midpoint rather than along the longest dimension after sorting.
   The real f64 search (`find_best_swap_impl`) and split
   (`sort_and_split_impl`) exist but are not called. The result is a valid
   tour, but it is not Algorithm 26.7's heuristic, and its cost is
   O(n lg n) work rather than O(n²). All six old eTSP lines analyzed the
   O(n²) search that does not run.
2. **DivConReduceMtPer is sequential.** All five `*_parallel` functions go
   through `ArraySeqMtPerS::reduce`, which is a sequential left fold, so they
   have Span O(n). All 15 old lines claimed Span O(lg n) and "Agrees with
   APAS".
3. **The Mt merge is not linear work and has linear-or-worse span.**
   `merge_dc` in `MergeSortMtPer.rs` copies its sub-inputs and outputs with
   sequential Vec loops at every level: Work O(n lg n), Span O(n lg n) on
   sorted input, where the right half never splits. `merge_sort_parallel`
   is therefore Work O(n lg² n), Span O(n lg n), not the textbook
   O(n lg n), O(lg² n). The old lines claimed O(n) work for the merge.
4. `ScanDCMtPer.rs` forks the recursion and the combine map, but the
   split into halves and the concatenation are sequential, so its span is
   O(n), not O(lg n); the old impl line claimed Span O(lg² n) and Work O(n).
5. The St files (MergeSortStPer, DivConReduceStPer, ScanDCStPer) are
   sequential by design; their old lines stated this, except that the
   `ScanDCStPer.rs` impl lines claimed O(n) work for an O(n lg n) recursion.
