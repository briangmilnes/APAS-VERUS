# r228 Alg Analysis Review: Chap36

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap36.txt` is a task prompt that quotes Algorithm 36.1 (Generic
Quicksort) and the book's discussion of pivot selection. With parallel
filters (Span O(lg n) per level) and parallel recursion:

| # | Source | Pivot rule | Work | Span |
|---|---|---|---|---|
| 1 | Alg 36.1 | first element, worst (sorted input) | Θ(n²) | O(n lg n) [1] |
| 2 | Alg 36.1 | median of three, sorted input | Θ(n lg n) | Θ(lg² n) |
| 3 | Alg 36.1 | median of three, worst case | "no better than first": Θ(n²) | O(n lg n) [1] |
| 4 | Alg 36.1 | uniformly random | Θ(n lg n) expected | Θ(lg² n) w.h.p. |

[1] The prose gives the depth-n pivot tree and the Θ(n²) work; the span
O(n lg n) follows from n levels of O(lg n)-span filters and matches the
existing APAS line on `quick_sort_first`.

The APAS lines on `quick_sort_median3` state W O(n lg n), S O(lg² n), which
is the sorted-input case only; the review compares worst cases (row 3).

## 2. Reviewed functions

T = trait declaration, I = impl. NT = does not match textbook; Old = does
not match old analysis; NC = no textbook cost. m = the size of one
recursive call's input.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 36 | QuickSortStEph.rs | quick_sort_first (T) | n², n lg n | n², n² worst | W n², S n² worst | NT: sequential |
| 2 | 36 | QuickSortStEph.rs | quick_sort_first (I) | n², n lg n | n lg n exp | W n², S n² worst | NT; Old [2] |
| 3 | 36 | QuickSortStEph.rs | quick_sort_median3 (T,I) | n lg n, lg² n | n lg n, n lg n | W n², S n² worst | NT; Old [3] |
| 4 | 36 | QuickSortStEph.rs | quick_sort_random (T,I) | n lg n, lg² n | n lg n, n lg n exp | W n lg n, S n lg n exp | NT: sequential |
| 5 | 36 | QuickSortStEph.rs | median_of_three (T,I) | none | 1, 1 | W 1, S 1 | NC |
| 6 | 36 | QuickSortStEph.rs | median3_pivot_idx (T,I) | none | 1, 1 | W 1, S 1 | NC |
| 7 | 36 | QuickSortStEph.rs | concat_three (T,I) | none | n, n | W n, S n | NC |
| 8 | 36 | QuickSortMtEph.rs | quick_sort_first (T) | n², n lg n | n², n² worst | W n², S n² worst | NT [4] |
| 9 | 36 | QuickSortMtEph.rs | quick_sort_first (I) | n², n lg n | n², S n | W n², S n² worst | NT; Old [4] |
| 10 | 36 | QuickSortMtEph.rs | quick_sort_median3 (T,I) | n lg n, lg² n | n lg n, n | W n², S n² worst | NT; Old [3][4] |
| 11 | 36 | QuickSortMtEph.rs | quick_sort_random (T,I) | n lg n, lg² n | n lg n, n exp | W n lg n, S n exp | NT [4] |
| 12 | 36 | QuickSortMtEph.rs | median_of_three, median3_pivot_idx (T,I) | none | 1, 1 | W 1, S 1 | NC |
| 13 | 36 | QuickSortMtEph.rs | concat_three (T,I) | none | n, n | W n, S n | NC |
| 14 | 36 | QuickSortMtEphSlice.rs | quick_sort_first (T,I) | n², n lg n | n², n lg n | W n² lg n, S n² worst | NT; Old [5] |
| 15 | 36 | QuickSortMtEphSlice.rs | quick_sort_median3 (T,I) | n lg n, lg² n | n lg n, lg² n | W n² lg n, S n² worst | NT; Old [3][5] |
| 16 | 36 | QuickSortMtEphSlice.rs | quick_sort_random (T,I) | n lg n, lg² n | n lg n, lg² n exp | W n lg² n, S n exp | NT; Old [5] |
| 17 | 36 | QuickSortMtEphSlice.rs | median_of_three, median3_pivot_idx (T,I) | none | 1, 1 | W 1, S 1 | NC |
| 18 | 36 | QuickSortMtEphSlice.rs | concat_three_vecs (T,I) | none | n, n | W n, S n | NC |

### Footnotes

[2] The impl line said "Work O(n log n) expected, Span O(n log n)
expected" for the first-element pivot, which is deterministic: sorted input
gives a depth-n pivot tree and Θ(n²) work, as the trait line says.

[3] Median-of-three is deterministic; the textbook states that in the worst
case it is no better than the first-element rule. The old lines gave the
sorted-input bound (O(n lg n)) as if it were the general bound.

[4] `QuickSortMtEph.rs` forks the two recursive calls with `ParaPair!`,
but partitions with a sequential loop and concatenates with the sequential
`concat_three`, so each level costs O(m) span. The random-pivot version
has S(n) = S(larger part) + O(n) = O(n) expected; on a depth-n chain (first
or median-of-three worst case) the per-level spans add to O(n²). The old
`quick_sort_first` impl line claimed Span O(n).

[5] `QuickSortMtEphSlice.rs` partitions with `partition_three_dc`, which
splits its slice in O(1) and forks with `join`, but rejoins the halves'
result Vecs with `append_vec`, a sequential push loop: W(m) = 2W(m/2) + O(m)
= O(m lg m) and S(m) = O(m). Each level also copies the sorted halves with
`to_vec` and `concat_three_vecs` (sequential O(m)). Hence random pivot:
W O(n lg² n) expected (O(lg n) levels of O(n lg n) partitions), S O(n)
expected; first/median worst: W O(Σ m lg m) = O(n² lg n), S O(n²). The old
lines claimed the textbook's O(lg m)-span partition and S O(lg² n) or
O(n lg n).

## 3. Counts (per annotation site)

36 new lines were added, one per annotated function site (trait and impl
counted separately), in 3 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 18 |
| 3 | does not match old analysis | 12 |
| 4 | no textbook cost | 18 |
| 5 | unannotated functions | 2 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 36 lines; row 3 overlaps them.

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 36 | QuickSortStEph.rs | 12 | 0 | 6 | 3 | 6 |
| 2 | 36 | QuickSortMtEph.rs | 12 | 0 | 6 | 3 | 6 |
| 3 | 36 | QuickSortMtEphSlice.rs | 12 | 0 | 6 | 6 | 6 |

## 4. Unannotated functions (2)

| # | Chap | File | Function | Cost (for the record) |
|---|---|---|---|---|
| 1 | 36 | QuickSortMtEphSlice.rs | append_vec | W O(\|b\|), S O(\|b\|) |
| 2 | 36 | QuickSortMtEphSlice.rs | partition_three_dc | W O(n lg n), S O(n) [6] |

[6] `partition_three_dc` has a free-text doc line "Work O(n), Span O(lg n)
for the partition itself (plus O(n) rejoin)" but no `Alg Analysis` line,
so no review line was added. The rejoin dominates: at every level of the
recursion it is O(n) sequential, so the partition is W O(n lg n), S O(n).

## 5. Malformed annotations (0)

None.

## 6. Notable findings

1. **The Slice quicksort's "parallel partition" has linear span.** The
   D&C partition in `QuickSortMtEphSlice.rs` forks correctly but rejoins
   the three result Vecs with sequential pushes at every level, so it
   costs W O(n lg n), S O(n) rather than the O(n), O(lg n) of a parallel
   filter. Together with the sequential `to_vec` and `concat_three_vecs`
   copies, the random-pivot sort is W O(n lg² n), S O(n) expected, not the
   O(n lg n), O(lg² n) its six old lines claimed. The same rejoin defect is
   in `OrderStatSelectMt*.rs` (Chap35).
2. **Median-of-three was analyzed as if it were randomized.** All six old
   median-of-three lines give O(n lg n) work; the textbook says its worst
   case is no better than the first-element rule, Θ(n²).
3. `QuickSortMtEph.rs` forks the recursion but partitions and concatenates
   sequentially; its random-pivot span O(n) is stated correctly by the old
   lines, but the old `quick_sort_first` impl line claimed Span O(n) for a
   case whose span is O(n²) on sorted input.
4. No Chap36 file reaches the textbook's Span O(lg² n); every one would
   need an O(lg n)-span partition and an O(1)- or O(lg n)-span append.
