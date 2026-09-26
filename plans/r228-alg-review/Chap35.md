# r228 Alg Analysis Review: Chap35

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

From `prompts/Chap35.txt` (Order Statistics).

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | §1 | select by reduction to sorting | O(n lg n) | — |
| 2 | Alg 35.2 | contraction-based select | O(n) expected | O(lg² n) w.h.p. |
| 3 | Alg 35.2 | one round (two filters) | O(n) | O(lg n) |

Row 3 is the per-round cost used in the dart-game analysis ("at each
recursive call work is linear and span is logarithmic").

## 2. Reviewed functions

T = trait declaration, I = impl, F = free function. All costs for select
are expected over the random pivots. m = the size of the input to one
round. NT = does not match textbook; Old = does not match old analysis.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 35 | OrderStatSelectStEph.rs | select (T,I) | n, lg² n | n, n | W n, S n | NT: seq partition |
| 2 | 35 | OrderStatSelectStEph.rs | select_inner (F) | n, lg² n | n, n | W n, S n | NT: seq partition |
| 3 | 35 | OrderStatSelectStPer.rs | select (T,I) | n, lg² n | n, n | W n, S n | NT: seq partition |
| 4 | 35 | OrderStatSelectStPer.rs | select_inner (F) | n, lg² n | n, n | W n, S n | NT: seq partition |
| 5 | 35 | OrderStatSelectMtEph.rs | select (T,I) | n, lg² n | n, lg² n | W n lg n, S n | NT; Old [1] |
| 6 | 35 | OrderStatSelectMtEph.rs | partition_three_dc (F) | m, lg m | m, lg m | W m lg m, S m | NT; Old [2] |
| 7 | 35 | OrderStatSelectMtEph.rs | parallel_three_way_partition (F) | m, lg m | m, lg m | W m lg m, S m | NT; Old [3] |
| 8 | 35 | OrderStatSelectMtEph.rs | select_inner (F) | n, lg² n | n, lg² n | W n lg n, S n | NT; Old [1] |
| 9 | 35 | OrderStatSelectMtPer.rs | select (T,I) | n, lg² n | n, lg² n | W n lg n, S n | NT; Old [1] |
| 10 | 35 | OrderStatSelectMtPer.rs | partition_three_dc (F) | m, lg m | m, lg m | W m lg m, S m | NT; Old [2] |
| 11 | 35 | OrderStatSelectMtPer.rs | parallel_three_way_partition (F) | m, lg m | m, lg m | W m lg m, S m | NT; Old [3] |
| 12 | 35 | OrderStatSelectMtPer.rs | select_inner (F) | n, lg² n | n, lg² n | W n lg n, S n | NT; Old [1] |

### Footnotes

[1] Each round costs W O(m lg m), S O(m) (rows 6-7). With the expected
geometric shrinkage of Alg 35.2 (E[m_i] ≤ 0.875^i n), the expected totals
are W ≤ lg n · Σ E[m_i] = O(n lg n) and S = Σ E[m_i] = O(n). The old lines
claimed the textbook's O(n), O(lg² n).

[2] `partition_three_dc` splits its slice in O(1) (`ArraySeqMtEphSliceS::slice`)
and forks the two halves with `join`, which is correct, but then merges
the right half's three result Vecs into the left half's with `append_vec`,
a sequential push loop. That gives W(m) = 2W(m/2) + O(m) = O(m lg m) and
S(m) = S(m/2) + O(m) = O(m). The old lines claimed W O(m), S O(lg m)
("base case O(1), O(lg n) levels"), ignoring the rejoin, although the
doc comment itself mentions "plus O(n) sequential rejoin".

[3] Copies the whole input into a fresh Vec with a sequential `nth` loop
(O(m) span) before calling `partition_three_dc`.

## 3. Counts (per annotation site)

16 new lines were added, one per annotated function site (trait and impl
counted separately), in 4 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 16 |
| 3 | does not match old analysis | 10 |
| 4 | no textbook cost | 0 |
| 5 | unannotated functions | 2 |
| 6 | malformed annotations | 0 |

Per file:

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 35 | OrderStatSelectStEph.rs | 3 | 0 | 3 | 0 | 0 |
| 2 | 35 | OrderStatSelectStPer.rs | 3 | 0 | 3 | 0 | 0 |
| 3 | 35 | OrderStatSelectMtEph.rs | 5 | 0 | 5 | 5 | 0 |
| 4 | 35 | OrderStatSelectMtPer.rs | 5 | 0 | 5 | 5 | 0 |

## 4. Unannotated functions (2)

| # | Chap | File | Function | Cost (for the record) |
|---|---|---|---|---|
| 1 | 35 | OrderStatSelectMtEph.rs | append_vec | W O(\|b\|), S O(\|b\|) |
| 2 | 35 | OrderStatSelectMtPer.rs | append_vec | W O(\|b\|), S O(\|b\|) |

## 5. Malformed annotations (0)

None. The `select_inner` Code-review lines in both Mt files continue onto a
second `///   ...` line; the new line was placed after that continuation.

## 6. Notable findings

1. **The Mt partition has linear span and n lg n work.** The fork-join
   structure of `partition_three_dc` is right (O(1) slice split, `join` on
   the halves), but the three result Vecs are rejoined with sequential
   pushes at every level. That turns the textbook's O(n)-work,
   O(lg n)-span filter into O(n lg n) work and O(n) span, and makes
   `select` O(n lg n) expected work and O(n) expected span, no better in
   span than the St version and worse in work. All ten old Mt lines
   claimed the textbook bounds.
2. `parallel_three_way_partition` also copies the input sequentially into
   a new Vec before partitioning, an O(n)-span step per round that would
   remain even with a parallel rejoin.
3. The St versions match the textbook work (O(n) expected) and are
   sequential by design (Span O(n)); their old lines are right.
4. `OrderStatSelectMtEph.rs` and `OrderStatSelectMtPer.rs` are identical
   except for the imported sequence module (Chap19 vs Chap18).
