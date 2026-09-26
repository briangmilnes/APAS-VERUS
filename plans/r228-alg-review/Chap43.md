# r228 Alg Analysis Review: Chap43

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

Chapter 43 (Ordered Sets and Tables) gives one cost specification for the
ordered operations and defers every other operation to the Chapter 41 set
and Chapter 42 table specifications. The augmented-table section (Def 43.3,
Example 43.2) fixes the cost of `reduceVal`. `prompts/Chap43.txt` lacks the
augmented section; it was read from the book text.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | CS 43.2 | first, last, previous, next | lg n | lg n |
| 2 | CS 43.2 | split, getRange, rank, select, splitRank | lg n | lg n |
| 3 | CS 43.2 | join | lg(n + m) | lg(n + m) |
| 4 | CS 41.4 (sets) | size, singleton | 1 | 1 |
| 5 | CS 41.4 | toSeq | \|a\| | lg \|a\| |
| 6 | CS 41.4 | filter | Σ W(f) | lg n + max S(f) |
| 7 | CS 41.4 | intersection, union, difference | m lg(1 + n/m) | lg n |
| 8 | CS 41.4 | find, insert, delete | lg n | lg n |
| 9 | Ex 41.3 | fromSeq | n lg n | lg² n |
| 10 | CS 42.5 (tables) | size, singleton | 1 | 1 |
| 11 | CS 42.5 | domain | \|a\| | lg \|a\| |
| 12 | CS 42.5 | filter, map | Σ W(f) | lg \|a\| + max S(f) |
| 13 | CS 42.5 | find, insert, delete | lg \|a\| | lg \|a\| |
| 14 | CS 42.5 | inter/union/diff/restrict/subtract | m lg(1 + n/m) | lg(n + m) |
| 15 | Def 43.3 | reduceVal | 1 | 1 |
| 16 | Example 43.2 | reduceVal(getRange) | lg n | lg n |

CS 42.5 has no row for `tabulate`, `reduce`, or `collect`; those lines were
compared against the file's own APAS line ("APAS line O(...)"). Helper
functions the book does not name (`from_st`, `from_sorted_elements`,
`from_sorted_entries`, `calculate_reduction`, `iter`, `tree_max_key`,
`tree_select`) are "no textbook cost".

Notation: n = |self|, m = |other| or |keys|, h = height of the backing
ParamBST (unbounded; h = n for a path). St files are sequential: Span =
Work. The Mt files in this chapter are sequential too (see finding 4).

## 2. The cost base

Every Chap43 structure stores its data in the Chap38 parametric BST, either
directly (OrderedSet* via AVLTreeSet*) or through the Chap41 `OrdKeyMap`
(OrderedTable*, AugOrderedTable*). The batch 7 and batch 9 reviews set
these callee costs, used here instead of the old annotations:

- ParamBST `expose` deep-copies both subtrees, so one expose is Θ(subtree
  size), and `join_mid` never rebalances. find, insert, delete, split,
  min_key, max_key, and in_order cost O(n h); union, intersect, and
  difference cost O(n h²); filter costs O(n h² + Σ W(f)).
- AVLTreeSetStEph/StPer: find/insert/delete O(n h) (StPer adds an O(n)
  clone); set operations O(n h²).
- OrdKeyMap: point and ordered operations O(n h); union/union_with
  O((n + m)² h + Σ W); intersect_with O(n m h + n h + Σ W); difference
  O(n m h + n²); map_values O(n² + Σ W); domain O(n²); tabulate
  O(n² h + Σ W); restrict and subtract O(n² + n m). The bulk operations
  insert in key order into a fresh tree, which builds a path.

Where a Chap43 function only forwards to one of these, its verdict says
"callee defect only".

## 3. Reviewed functions

Every trait function has an annotation on the trait declaration (T) and
another on the impl (I); each got its own line. Rows merge T and I when
both verdicts agree. "old wrong" means "does not match old analysis".

### 3a. OrderedSetStEph.rs (62 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedSetStEph.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | OrderedSetStEph.rs | find, insert, delete | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 3 | 43 | OrderedSetStEph.rs | filter (T,I) | ΣW, lg n+maxS | W n | W n h²+ΣW | not textbook; old wrong |
| 4 | 43 | OrderedSetStEph.rs | inter/union/diff | m lg(1+n/m) | truncated [1] | W n h², S n h² | not textbook; old wrong |
| 5 | 43 | OrderedSetStEph.rs | to_seq | \|a\|, lg \|a\| | W n | W n h, S n h | not textbook; old wrong |
| 6 | 43 | OrderedSetStEph.rs | from_seq | n lg n, lg² n | W n lg n | W n² h, S n² h | not textbook; old wrong |
| 7 | 43 | OrderedSetStEph.rs | first/last/prev/next | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 8 | 43 | OrderedSetStEph.rs | split, get_range | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 9 | 43 | OrderedSetStEph.rs | rank, select, split_rank | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 10 | 43 | OrderedSetStEph.rs | join | lg(n + m) | W m lg(n/m+1) | W n h², S n h² | not textbook; old wrong [2] |
| 11 | 43 | OrderedSetStEph.rs | *_iter variants | as base op | W lg n | as base op | not textbook; old wrong [3] |
| 12 | 43 | OrderedSetStEph.rs | tree_max_key, tree_select | none | W lg n | W n h, S n h | no cost [4] |
| 13 | 43 | OrderedSetStEph.rs | from_sorted_elements | none | W n | W n², S n² | no cost [4] |

### 3b. OrderedSetStPer.rs (63 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedSetStPer.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | OrderedSetStPer.rs | find | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 3 | 43 | OrderedSetStPer.rs | insert, delete | lg n | W lg n | W n h (+ O(n) clone) | not textbook; old wrong [5] |
| 4 | 43 | OrderedSetStPer.rs | filter | ΣW, lg n+maxS | W n | W n h²+ΣW | not textbook; old wrong |
| 5 | 43 | OrderedSetStPer.rs | inter/union/diff | m lg(1+n/m) | truncated [1] | W n h², S n h² | not textbook; old wrong |
| 6 | 43 | OrderedSetStPer.rs | to_seq, from_seq | \|a\|; n lg n | W n; n lg n | W n h; n² h | not textbook; old wrong |
| 7 | 43 | OrderedSetStPer.rs | ordered ops (9 fns) | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 8 | 43 | OrderedSetStPer.rs | join | lg(n + m) | W m lg(n/m+1) | W n h², S n h² | not textbook; old wrong [2] |
| 9 | 43 | OrderedSetStPer.rs | *_iter variants | as base op | in_order+scan | as base op | not textbook; old wrong [3] |
| 10 | 43 | OrderedSetStPer.rs | 3 helpers | none | W lg n / n | W n h / n² | no cost [4] |

### 3c. OrderedSetMtEph.rs (45 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedSetMtEph.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | OrderedSetMtEph.rs | find, insert, delete | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 3 | 43 | OrderedSetMtEph.rs | filter | ΣW, lg n+maxS | W n | W n h²+ΣW (seq) | not textbook; old wrong [6] |
| 4 | 43 | OrderedSetMtEph.rs | inter/union/diff | m lg(1+n/m) | truncated [1] | W n h², S n h² | not textbook; old wrong [6] |
| 5 | 43 | OrderedSetMtEph.rs | to_seq (T), from_seq | \|a\|; n lg n | W n; n lg n | W n h; n² h | not textbook; old wrong |
| 6 | 43 | OrderedSetMtEph.rs | ordered ops (9 fns) | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 7 | 43 | OrderedSetMtEph.rs | join | lg(n + m) | W m lg(n/m+1) | W n h², S n h² | not textbook; old wrong [2] |
| 8 | 43 | OrderedSetMtEph.rs | from_st | none | W 1 | W 1, S 1 | no textbook cost |
| 9 | 43 | OrderedSetMtEph.rs | iter (T,I) | none | W n | W n h, S n h | no cost; old wrong |

The impl `to_seq` annotation is glued onto a `// Veracity:` line and got no
new line (section 6).

### 3d. OrderedTableStEph.rs (81 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedTableStEph.rs | size/empty/singleton/is_empty | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | OrderedTableStEph.rs | find, lookup | lg n | W lg n / Θ(n) | W n h, S n h | not textbook; old wrong |
| 3 | 43 | OrderedTableStEph.rs | insert, delete | lg n | W n | W n h, S n h | not textbook; old wrong |
| 4 | 43 | OrderedTableStEph.rs | domain | \|a\|, lg \|a\| | W n | W n², S n² | not textbook; old wrong [7] |
| 5 | 43 | OrderedTableStEph.rs | tabulate | APAS n lg n | W n lg n | W n² h + ΣW | not textbook; old wrong |
| 6 | 43 | OrderedTableStEph.rs | map | ΣW, lg n+maxS | W n lg n | W n² + ΣW | not textbook; old wrong [8] |
| 7 | 43 | OrderedTableStEph.rs | filter | ΣW, lg n+maxS | W n lg n | W n h² + ΣW | not textbook; old wrong |
| 8 | 43 | OrderedTableStEph.rs | reduce, collect | APAS n; n lg n | W n | W n h (+ΣW) | not textbook; old wrong |
| 9 | 43 | OrderedTableStEph.rs | intersection | m lg(1+n/m) | truncated [1] | W n m h + ΣW | not textbook; old wrong [8] |
| 10 | 43 | OrderedTableStEph.rs | union | m lg(1+n/m) | truncated [1] | W (n+m)² h + ΣW | not textbook; old wrong [8] |
| 11 | 43 | OrderedTableStEph.rs | difference | m lg(1+n/m) | truncated [1] | W n m h + n² | not textbook; old wrong [8] |
| 12 | 43 | OrderedTableStEph.rs | restrict, subtract | m lg(1+n/m) | truncated [1] | W n² + n m | not textbook; old wrong [9] |
| 13 | 43 | OrderedTableStEph.rs | ordered ops (9 fns) | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 14 | 43 | OrderedTableStEph.rs | join_key | lg(n + m) | truncated [1] | W (n+m)² h | not textbook; old wrong [2] |
| 15 | 43 | OrderedTableStEph.rs | *_iter variants | as base op | W lg n | as base op | not textbook; old wrong [3] |
| 16 | 43 | OrderedTableStEph.rs | from_sorted_entries | none | W n | W n², S n² | no cost [4] |

### 3e. OrderedTableStPer.rs (78 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedTableStPer.rs | size, empty, singleton | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | OrderedTableStPer.rs | find | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 3 | 43 | OrderedTableStPer.rs | insert, delete | lg n | W lg n | W n h (+ O(n) clone) | not textbook; old wrong [5] |
| 4 | 43 | OrderedTableStPer.rs | domain | \|a\|, lg \|a\| | W n | W n², S n² | not textbook; old wrong [7] |
| 5 | 43 | OrderedTableStPer.rs | tabulate | APAS n lg n | W n lg n | W n² h + ΣW | not textbook; old wrong |
| 6 | 43 | OrderedTableStPer.rs | map (local) | ΣW, lg n+maxS | W n | W n² + ΣW | not textbook; old wrong [9] |
| 7 | 43 | OrderedTableStPer.rs | filter | ΣW, lg n+maxS | W n | W n h² + ΣW | not textbook; old wrong |
| 8 | 43 | OrderedTableStPer.rs | collect, reduce | APAS n lg n; n | W n | W n h (+ΣW) | not textbook; old wrong |
| 9 | 43 | OrderedTableStPer.rs | inter/union/diff | m lg(1+n/m) | truncated [1] | as 3d rows 9–11 | not textbook; old wrong [8] |
| 10 | 43 | OrderedTableStPer.rs | restrict, subtract (local) | m lg(1+n/m) | truncated [1] | W n² + n m | not textbook; old wrong [9] |
| 11 | 43 | OrderedTableStPer.rs | ordered ops (9 fns) | lg n | W lg n | W n h, S n h | not textbook; old wrong |
| 12 | 43 | OrderedTableStPer.rs | join_key | lg(n + m) | W m lg(n/m+1) | W (n+m)² h | not textbook; old wrong [2] |
| 13 | 43 | OrderedTableStPer.rs | *_iter variants | as base op | in_order + scan | as base op | not textbook; old wrong [3] |
| 14 | 43 | OrderedTableStPer.rs | from_sorted_entries | none | W n | W n², S n² | no cost [4] |

### 3f. OrderedTableMtEph.rs (60 lines) and OrderedTableMtPer.rs (45 lines)

Both are RwLock wrappers that take the lock and call the St counterpart
(StEph for MtEph, StPer for MtPer), so each operation costs the same as the
St operation in 3d/3e, and Span = Work.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedTableMtEph.rs | size/empty/singleton/is_empty | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | OrderedTableMtEph.rs | all other ops | as 3d | Θ(n) / n lg n | as 3d | not textbook; old wrong [6] |
| 3 | 43 | OrderedTableMtEph.rs | from_st | none | W 1 | W 1, S 1 | no textbook cost |
| 4 | 43 | OrderedTableMtEph.rs | from_sorted_entries | none | W n | W n², S n² | no cost [4] |
| 5 | 43 | OrderedTableMtPer.rs | size, empty, singleton | 1, 1 | Θ(1) | W 1, S 1 | matches textbook |
| 6 | 43 | OrderedTableMtPer.rs | find, insert(_wf), delete(_wf) | lg n | Θ(n) linear scan | W n h, S n h | not textbook; old wrong [5] |
| 7 | 43 | OrderedTableMtPer.rs | domain (I builds set) | \|a\|, lg \|a\| | Θ(n) / n lg n | W n², S n² | not textbook; old wrong [10] |
| 8 | 43 | OrderedTableMtPer.rs | map | ΣW, lg n+maxS | Θ(n) / n lg n | W n² + ΣW | not textbook; old wrong |
| 9 | 43 | OrderedTableMtPer.rs | filter (I builds table) | ΣW, lg n+maxS | Θ(n) / n² | W n² + ΣW | not textbook [10] |
| 10 | 43 | OrderedTableMtPer.rs | ordered ops (8 fns) | lg n | Θ(n lg n) collect | W n h, S n h | not textbook; old wrong |
| 11 | 43 | OrderedTableMtPer.rs | join_key | lg(n + m) | Θ(n + m) | W (n+m)² h | not textbook; old wrong [2] |
| 12 | 43 | OrderedTableMtPer.rs | from_st_table | none | W 1 | W 1, S 1 | no textbook cost |
| 13 | 43 | OrderedTableMtPer.rs | iter (T,I) | none | W n lg n | W n h, S n h | no cost; old wrong |

The MtPer filter impl's old line (O(n²)) matches the new bound up to the
Σ W(f) term, so it was not marked old wrong; the MtPer trait filter line
(Θ(n)) was.

### 3g. AugOrderedTableStEph.rs (63 lines), AugOrderedTableStPer.rs (57 lines), AugOrderedTableMtEph.rs (66 lines)

The three augmented tables wrap an ordered table (StEph, StPer, MtEph
respectively) and add one `cached_reduction` field. The rows apply to all
three files unless the File cell says otherwise.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 43 | AugOrderedTable*.rs | size/empty/singleton/is_empty | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 2 | 43 | AugOrderedTable*.rs | reduce_val (T,I) | Def 43.3: 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 3 | 43 | AugOrderedTable*.rs | find, lookup | lg n | Θ(n) linear scan | W n h, S n h | not textbook; old wrong |
| 4 | 43 | AugOrderedTable*.rs | insert, delete | lg n | W n | W n h, S n h | not textbook; old wrong [11] |
| 5 | 43 | AugOrderedTable*.rs | domain | \|a\|, lg \|a\| | W n | W n², S n² | not textbook; old wrong [7] |
| 6 | 43 | AugOrderedTable*.rs | tabulate, map, filter | as 3d | W n / n lg n | as 3d + recalc | not textbook; old wrong [11] |
| 7 | 43 | AugOrderedTable*.rs | inter/union/diff/restr/subtr | m lg(1+n/m) | truncated [1] | as 3d + recalc | not textbook; old wrong [11] |
| 8 | 43 | AugOrderedTable*.rs | collect, reduce | APAS n / n lg n | W n / n lg n | W n h (+ΣW) | not textbook; old wrong |
| 9 | 43 | AugOrderedTable*.rs | first/last/prev/next | lg n | W n lg n collect | W n h, S n h | not textbook; old wrong |
| 10 | 43 | AugOrderedTable*.rs | rank, select | lg n | W n lg n | W n h, S n h | not textbook; old wrong |
| 11 | 43 | AugOrderedTable*.rs | split, split_rank, range | lg n | W n lg n | W n h + recalc | not textbook; old wrong [11] |
| 12 | 43 | AugOrderedTable*.rs | join_key | lg(n + m) | W n + m / n·m | W (n+m)² h | not textbook; old wrong [2][12] |
| 13 | 43 | AugOrderedTable*.rs | reduce_range | Ex 43.2: lg n | W n lg n | W n h, S n h | not textbook; old wrong [13] |
| 14 | 43 | AugOrderedTableMtEph.rs | reduce_range_parallel | Ex 43.2: lg n | W n lg n | W n h, S n h | not textbook; old wrong [14] |
| 15 | 43 | AugOrderedTable*.rs | calculate_reduction | none | W n | W n h + n W(r) | no cost; old wrong |
| 16 | 43 | AugOrderedTableMtEph.rs | recalculate_reduction | none | W n | W n h + n W(r) | no cost; old wrong |

"recalc" = one or two `calculate_reduction` folds, O(n h) each.

### Footnotes

1. The Opus 4.6 line is cut off after "Work O(m log(n/m + 1)" (or
   "Work O(log n + m)" for get_key_range) with no Span; see section 6. The
   comparison used the file's Θ line that follows it.
2. `join` / `join_key` calls union (ParamBST union or OrdKeyMap union_with)
   instead of a BST join, so it costs a full union instead of O(lg(n + m)).
3. The `*_iter` functions are not iterative: each calls its base
   operation, so it costs the same. Old StPer trait lines describe
   "in_order + scan", which the impls no longer do.
4. Helpers the book does not name. `from_sorted_elements` and
   `from_sorted_entries` insert sorted input one at a time into a
   never-rebalanced tree, which builds a path in O(n²). The old lines
   (O(n), O(lg n)) were wrong.
5. Persistence is implemented by an O(n) clone of the whole tree before
   the O(n h) OrdKeyMap or ParamBST operation.
6. The Mt files are sequential. Every operation takes the RwLock and calls
   the St version; no fork-join exists, so Span = Work.
7. OrdKeyMap `domain` inserts each key into an `ArraySetStEph`, whose
   insert is an O(i) linear find plus copy: O(n²).
8. OrdKeyMap bulk operations; costs from the Chap41 review (section 2).
9. Local implementations: `in_order` O(n h), an O(m) `ArraySetStEph`
   membership test per entry (restrict/subtract), then key-order inserts
   into a fresh ParamBST, O(i) each, which builds a path.
10. The MtPer impls do not call StPer domain/filter: they collect O(n h)
    and then insert in key order, n times, into an `OrderedSetMtEph`
    (domain) or a persistent StPer table (filter: O(i) clone per insert).
11. The Aug files keep one cached reduction for the whole table, not a
    reduction per BST node. After every mutation or split,
    `calculate_reduction` refolds the whole result: `collect` O(n h), then
    n `AVLTreeSeqStPer::nth` calls, O(lg n) each. Def 43.3 updates
    per-node values along one root-to-leaf path in O(lg n).
12. The StEph and StPer `join_key` combine the two cached reductions with
    one reducer call, O(1). The MtEph `join_key` refolds the joined table
    with `calculate_reduction` instead.
13. `reduce_range` copies the range with `get_key_range` and refolds it
    (O(n h)); Example 43.2 combines O(lg n) cached subtree values.
14. `reduce_range_parallel` does three `get_key_range` copies with their
    refolds, plus `select_key`, `next_key`, and `find`, all sequential and
    O(n h). Its `ParaPair!` forks two `reduce_val` calls, each O(1), so the
    fork adds no useful parallelism.

## 4. Counts (per annotation site)

620 new lines were added, one per annotated site (trait and impl counted
separately), in 10 files. `Example43_1.rs` (Example file, skipped) and
`OrderedSpecsAndLemmas.rs` have no annotations.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 74 |
| 2 | does not match textbook | 526 |
| 3 | does not match old analysis | 542 |
| 4 | no textbook cost | 20 |
| 5 | unannotated functions | 59 |
| 6 | malformed annotations | 49 |

Rows 1, 2, and 4 partition the 620 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedSetStEph.rs | 62 | 6 | 53 | 56 | 3 |
| 2 | 43 | OrderedSetStPer.rs | 63 | 6 | 54 | 57 | 3 |
| 3 | 43 | OrderedSetMtEph.rs | 45 | 6 | 36 | 38 | 3 |
| 4 | 43 | OrderedTableStEph.rs | 81 | 8 | 72 | 73 | 1 |
| 5 | 43 | OrderedTableStPer.rs | 78 | 6 | 71 | 72 | 1 |
| 6 | 43 | OrderedTableMtEph.rs | 60 | 8 | 50 | 51 | 2 |
| 7 | 43 | OrderedTableMtPer.rs | 45 | 6 | 36 | 37 | 3 |
| 8 | 43 | AugOrderedTableStEph.rs | 63 | 10 | 52 | 53 | 1 |
| 9 | 43 | AugOrderedTableStPer.rs | 57 | 8 | 48 | 49 | 1 |
| 10 | 43 | AugOrderedTableMtEph.rs | 66 | 10 | 54 | 56 | 2 |

## 5. Unannotated functions (59)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 43 | OrderedSetStEph.rs | next (I), iter, clone, default |
| 2 | 43 | OrderedSetStEph.rs | eq, fmt x2 |
| 3 | 43 | OrderedSetStPer.rs | iter, clone, default, eq, fmt x2 |
| 4 | 43 | OrderedSetMtEph.rs | next (I), fmt x3 |
| 5 | 43 | OrderedTableStEph.rs | iter, clone, eq, fmt x2 |
| 6 | 43 | OrderedTableStPer.rs | iter, eq, clone, fmt x2 |
| 7 | 43 | OrderedTableMtEph.rs | iter (T,I), clone, eq, default |
| 8 | 43 | OrderedTableMtEph.rs | fmt x3 |
| 9 | 43 | OrderedTableMtPer.rs | next, into_iter, clone, default |
| 10 | 43 | OrderedTableMtPer.rs | fmt x4 |
| 11 | 43 | AugOrderedTableStEph.rs | iter, clone, eq, fmt x2 |
| 12 | 43 | AugOrderedTableStPer.rs | iter, eq, clone, fmt x2 |
| 13 | 43 | AugOrderedTableMtEph.rs | iter (T,I), clone, eq, fmt x2 |

The commented-out `IntoIterator::into_iter` blocks (form C, r213) were not
counted. The two sites with glued annotations (section 6) are counted as
malformed, not unannotated.

## 6. Malformed annotations (49)

| # | Chap | File | Site | Problem |
|---|---|---|---|---|
| 1 | 43 | OrderedSetMtEph.rs | to_seq (I) | glued onto `// Veracity:` line |
| 2 | 43 | OrderedTableStPer.rs | next_key_iter (I) | glued onto `// Veracity:` line |
| 3 | 43 | OrderedSetStEph.rs | inter/union/diff (T) | 3 truncated lines [a] |
| 4 | 43 | OrderedSetStPer.rs | inter/union/diff (T) | 3 truncated lines [a] |
| 5 | 43 | OrderedSetMtEph.rs | inter/union/diff (T) | 3 truncated lines [a] |
| 6 | 43 | OrderedTableStEph.rs | 5 set ops, join, range (T) | 7 truncated lines [a] |
| 7 | 43 | OrderedTableStPer.rs | 5 set ops, join (T) | 6 truncated lines [a] |
| 8 | 43 | OrderedTableMtEph.rs | 5 set ops, join (T) | 6 truncated lines [a] |
| 9 | 43 | OrderedTableMtPer.rs | join_key (T) | 1 truncated line [a] |
| 10 | 43 | AugOrderedTableStEph.rs | 5 set ops, join (T) | 6 truncated lines [a] |
| 11 | 43 | AugOrderedTableStPer.rs | 5 set ops, join (T) | 6 truncated lines [a] |
| 12 | 43 | AugOrderedTableMtEph.rs | 5 set ops, join (T) | 6 truncated lines [a] |

Rows 1–2: the text `/// - Alg Analysis: Code review ...` sits on the same
line as `// Veracity: UNNEEDED proof block`, so it is not a doc line. No
review line was added. The reviews would be: `to_seq` O(n h), O(n h);
`next_key_iter` O(n h), O(n h). Both are "does not match textbook".

[a] A `/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m
+ 1)` line (or `Work O(log n + m)` on OrderedTableStEph `get_key_range`)
ends with the parenthesis open and has no Span. Each sits inside a block
with other lines, so the block got its review line after its last line.
The truncated lines were left unchanged.

## 7. Notable findings

1. **No Chap43 operation meets its O(lg n) bound, because of the callee
   defects.** OrderedSet* sits on AVLTreeSet* and OrderedTable* on
   OrdKeyMap, both on the Chap38 ParamBST, whose `expose` deep-copies both
   subtrees and whose `join_mid` never rebalances. Every CS 43.2 ordered
   operation (first, last, previous, next, split, getRange, rank, select,
   splitRank) and every find/insert/delete costs O(n h), which is O(n²)
   on a path. Bulk operations cost O(n h²) (sets) or up to O((n + m)² h)
   (OrdKeyMap union). 542 of the 620 old lines claimed lower costs,
   usually the textbook's own. Fixing expose and join_mid in Chap38 would
   repair most of the chapter at once.
2. **The augmented tables are not augmented BSTs.** Def 43.3 keeps a
   reduced value in every tree node, so insert, delete, split, and
   reduceVal(getRange) stay O(lg n). All three AugOrderedTable files keep
   one `cached_reduction` for the whole table and refold everything with
   `calculate_reduction` (O(n h)) after each insert, delete, bulk
   operation, split, and range query. Only `reduce_val` meets the textbook
   (O(1)); `reduce_range` is O(n h) against Example 43.2's O(lg n). This is
   an algorithm deviation, not only a callee defect. The MtEph
   `reduce_range_parallel` forks two O(1) reads after three sequential
   O(n h) range copies, so its parallelism does no useful work.
3. **join is union, and the Mt files are sequential.** Every `join` /
   `join_key` calls union instead of a BST join, costing a full union
   (O(n h²) to O((n + m)² h)) instead of O(lg(n + m)). The Mt files
   (OrderedSetMtEph, OrderedTableMtEph, OrderedTableMtPer,
   AugOrderedTableMtEph) take one RwLock and call the St code
   sequentially, so no operation reaches a textbook span.
4. Several constructors build paths: `from_seq`, `from_sorted_elements`,
   `from_sorted_entries`, the local map/restrict/subtract in
   OrderedTableStPer, and the MtPer domain/filter all insert sorted keys
   one at a time into a never-rebalanced tree, which costs O(n²) and
   leaves every later operation on the result at O(n²).
5. The `*_iter` variants only delegate to the base operations. Many old
   descriptions ("linear scan", "collect + sort", "treap", "TableStPer",
   "external_body") describe code that is no longer there, and the APAS
   lines misquote the book (filter as O(n), set-operation span as
   m log(n/m + 1), join as m log(n/m + 1)).
