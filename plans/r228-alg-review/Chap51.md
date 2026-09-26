# r228 Alg Analysis Review: Chap51

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap51.txt` (Implementing Dynamic Programming) presents two
implementations of Minimum Edit Distance (MED). It states no numeric cost
table of its own; the MED DAG costs come from Chapter 49 ("Work and Span"
of Recursive MED), and the prose fixes the parallelism of each method.

| # | Source | Algorithm | Work | Span |
|---|---|---|---|---|
| 1 | Ch49 MED DAG | MED, any pebbling | \|S\|\|T\| | \|S\| + \|T\| |
| 2 | Alg 51.1 | Bottom-up MED, diagonal pebbling | \|S\|\|T\| | \|S\| + \|T\| |
| 3 | Alg 51.3 | memo (find, then update) | 1 per lookup (hash/tree) | same |
| 4 | Alg 51.4 | Memoized MED, threaded table | \|S\|\|T\| | \|S\|\|T\| [1] |
| 5 | Alg 51.4 prose | parallel memoization | \|S\|\|T\| | \|S\| + \|T\| [2] |

[1] The prose: "The top-down approach as described is inherently
sequential. By threading the memo table through the computation, we force a
total ordering on all calls to med." Span therefore equals work. The files'
APAS lines that give Alg 51.4 Span O(|S|+|T|) (TopDownDPStEph) overstate
the textbook; the new lines record the prose cost.

[2] The prose names hidden-state memo tables, concurrent hash tables, and
synchronization variables "so that no function is computed more than once"
as the way to parallelize top-down; it gives no cost, so the DAG cost (row
1) is the reference for the `*_parallel` functions.

Notation: |S|, |T| = string lengths; n = memo table capacity for the
memo-maintenance helpers. HashMap operations are expected O(1). St files
are sequential: Span = Work.

## 2. Reviewed functions

T = trait declaration, I = impl; both got a line. Rows merge T and I where
the verdicts agree.

### 2a. BottomUpDP* (4 files, 64 lines)

All four files share one body: `med_bottom_up` (St) and
`med_bottom_up_parallel` (Mt) fill the (|S|+1)×(|T|+1) table row by row
with nested `while` loops. The Mt files contain no `join` or other fork.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 51 | BottomUpDPStEph.rs | new, s/t_length (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 2 | 51 | BottomUpDPStEph.rs | is_empty, set_s, set_t (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 3 | 51 | BottomUpDPStEph.rs | med_bottom_up (T,I) | ST, S+T | ST, ST | ST, ST | not textbook [3] |
| 4 | 51 | BottomUpDPStEph.rs | initialize_base_cases (T,I) | — | ST, ST | ST, ST | no textbook cost |
| 5 | 51 | BottomUpDPStEph.rs | compute_cell_value (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 6 | 51 | BottomUpDPStPer.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 7 | 51 | BottomUpDPStPer.rs | med_bottom_up (T,I) | ST, S+T | ST, ST | ST, ST | not textbook [3] |
| 8 | 51 | BottomUpDPStPer.rs | initialize_base_cases (T,I) | — | ST, ST | ST, ST | no textbook cost |
| 9 | 51 | BottomUpDPStPer.rs | compute_cell_value (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 10 | 51 | BottomUpDPMtEph.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 11 | 51 | BottomUpDPMtEph.rs | set_s, set_t (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 12 | 51 | BottomUpDPMtEph.rs | med_bottom_up_parallel (T,I) | ST, S+T | ST, ST | ST, ST | not textbook [3][4] |
| 13 | 51 | BottomUpDPMtEph.rs | initialize_base_cases (T,I) | — | ST, ST | ST, ST | no textbook cost |
| 14 | 51 | BottomUpDPMtEph.rs | compute_cell_value (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 15 | 51 | BottomUpDPMtPer.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 16 | 51 | BottomUpDPMtPer.rs | med_bottom_up_parallel (T,I) | ST, S+T | ST, ST | ST, ST | not textbook [3][4] |
| 17 | 51 | BottomUpDPMtPer.rs | initialize_base_cases (T,I) | — | ST, ST | ST, ST | no textbook cost |
| 18 | 51 | BottomUpDPMtPer.rs | compute_cell_value (T,I) | — | 1, 1 | 1, 1 | no textbook cost |

ST = |S|·|T|, S+T = |S|+|T|.

[3] Row-by-row sequential fill; Alg 51.1 pebbles each anti-diagonal in
parallel for Span O(|S|+|T|).

[4] `med_bottom_up_parallel` has no parallelism despite its name; the old
lines already say so.

### 2b. TopDownDPStEph.rs (26 lines) and TopDownDPStPer.rs (22 lines)

Memo table: `std::collections::HashMap<Pair<usize,usize>, usize>`.
`med_recursive` looks up (i, j), recurses on at most two subproblems,
and inserts; each of the (|S|+1)(|T|+1) entries is computed once.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 51 | TopDownDPStEph.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 2 | 51 | TopDownDPStEph.rs | memo_size (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 3 | 51 | TopDownDPStEph.rs | is_memoized, get_memoized (T,I) | — | 1, 1 | 1, 1 exp. | no textbook cost |
| 4 | 51 | TopDownDPStEph.rs | insert_memo (T,I) | — | 1, 1 | 1, 1 am. exp. | no textbook cost |
| 5 | 51 | TopDownDPStEph.rs | clear_memo (T,I) | — | n, n | n, n | no textbook cost |
| 6 | 51 | TopDownDPStEph.rs | set_s, set_t (T) | — | 1, 1 | n, n | no cost; old wrong [5] |
| 7 | 51 | TopDownDPStEph.rs | set_s, set_t (I) | — | n, n | n, n | no textbook cost |
| 8 | 51 | TopDownDPStEph.rs | med_memoized (T,I) | ST, S+T | ST, ST | ST, ST exp. | matches textbook [1] |
| 9 | 51 | TopDownDPStEph.rs | med_recursive (T,I) | ST, S+T | ST, ST | ST, ST exp. | matches textbook [1] |
| 10 | 51 | TopDownDPStPer.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 11 | 51 | TopDownDPStPer.rs | memo_size (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 12 | 51 | TopDownDPStPer.rs | is_memoized, get_memoized (T,I) | — | 1, 1 | 1, 1 exp. | no textbook cost |
| 13 | 51 | TopDownDPStPer.rs | with_memo_table, clear_memo (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 14 | 51 | TopDownDPStPer.rs | med_memoized (T,I) | — | ST, ST | ST, ST exp. | matches textbook [1] |
| 15 | 51 | TopDownDPStPer.rs | med_recursive (T,I) | — | ST, ST | ST, ST exp. | matches textbook [1] |

[5] The trait lines say O(1) ("move sequence"), but the impls also call
`memo_table.clear()`, O(capacity).

### 2c. TopDownDPMtEph.rs (18 lines) and TopDownDPMtPer.rs (14 lines)

`med_memoized_concurrent` calls `med_recursive_sequential` with a local
HashMap; it is the sequential Alg 51.4. `med_memoized_parallel` calls
`med_recursive_parallel`, which shares an `Arc<RwLock<HashMap>>` and forks
the delete and insert subproblems with `join`.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 51 | TopDownDPMtEph.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 2 | 51 | TopDownDPMtEph.rs | set_s, set_t (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 3 | 51 | TopDownDPMtEph.rs | med_memoized_concurrent (T,I) | ST, ST | ST, ST | ST, ST exp. | matches textbook [6] |
| 4 | 51 | TopDownDPMtEph.rs | med_memoized_parallel (T,I) | ST, S+T | ST, S+T | see [7] | not textbook; old wrong [7] |
| 5 | 51 | TopDownDPMtEph.rs | med_recursive_sequential | ST, ST | ST, ST | ST, ST exp. | matches textbook |
| 6 | 51 | TopDownDPMtEph.rs | med_recursive_parallel | ST, S+T | ST, S+T | see [7] | not textbook; old wrong [7] |
| 7 | 51 | TopDownDPMtPer.rs | new, s/t_length, is_empty (T,I) | — | 1, 1 | 1, 1 | no textbook cost |
| 8 | 51 | TopDownDPMtPer.rs | med_memoized_concurrent (T,I) | — | ST, ST | ST, ST exp. | matches textbook [6] |
| 9 | 51 | TopDownDPMtPer.rs | med_memoized_parallel (T,I) | — | ST, S+T | see [7] | not textbook; old wrong [7] |
| 10 | 51 | TopDownDPMtPer.rs | med_recursive_sequential | — | ST, ST | ST, ST exp. | matches textbook |
| 11 | 51 | TopDownDPMtPer.rs | med_recursive_parallel | — | ST, S+T | see [7] | not textbook; old wrong [7] |

[6] Sequential; the name `med_memoized_concurrent` is misleading.

[7] New cost: Work O(|S|·|T|·(|S|+|T|)) when the two branches see each
other's memo entries, exponential in the worst case; Span
O((|S|+|T|)²). Two causes:

- Each mismatch call clones `seq_s` and `seq_t` twice for the closures.
  `ArraySeqMtEphS`/`ArraySeqMtPerS` wrap a `Vec`, so each clone is
  O(|S|+|T|) sequential work, and this lies on every level of the
  O(|S|+|T|)-deep recursion.
- A result is inserted into the memo only after it is computed. Two
  concurrent branches that both need (i-1, j-1) both miss and both
  recompute it. Under a fully parallel schedule every call misses, and the
  call tree is the full exponential MED recursion tree. The prose names
  synchronization variables as the required fix; none are present. With
  the bounded HFScheduler pool most joins run sequentially, which recovers
  sharing, but no bound better than exponential holds in general.

## 3. Counts (per annotation site)

144 new lines were added, one per annotated site (trait and impl counted
separately), in 8 files. `SeqSpecsAndLemmas.rs` has no annotations (spec
functions and lemmas only).

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 14 |
| 2 | does not match textbook | 14 |
| 3 | does not match old analysis | 8 |
| 4 | no textbook cost | 116 |
| 5 | unannotated functions | 44 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 144 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 51 | BottomUpDPStEph.rs | 18 | 0 | 2 | 0 | 16 |
| 2 | 51 | BottomUpDPStPer.rs | 14 | 0 | 2 | 0 | 12 |
| 3 | 51 | BottomUpDPMtEph.rs | 18 | 0 | 2 | 0 | 16 |
| 4 | 51 | BottomUpDPMtPer.rs | 14 | 0 | 2 | 0 | 12 |
| 5 | 51 | TopDownDPStEph.rs | 26 | 4 | 0 | 2 | 22 |
| 6 | 51 | TopDownDPStPer.rs | 22 | 4 | 0 | 0 | 18 |
| 7 | 51 | TopDownDPMtEph.rs | 18 | 3 | 3 | 3 | 12 |
| 8 | 51 | TopDownDPMtPer.rs | 14 | 3 | 3 | 3 | 8 |

## 4. Unannotated functions (44)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 51 | BottomUpDPStEph.rs | default, clone, eq, fmt x2 |
| 2 | 51 | BottomUpDPStPer.rs | default, clone, eq, fmt x2 |
| 3 | 51 | BottomUpDPMtEph.rs | default, clone, eq, fmt x2 |
| 4 | 51 | BottomUpDPMtPer.rs | default, clone, eq, fmt x2 |
| 5 | 51 | TopDownDPStEph.rs | default, clone, eq, fmt x2 |
| 6 | 51 | TopDownDPStPer.rs | default, clone, eq, fmt x2 |
| 7 | 51 | TopDownDPMtEph.rs | default, clone, eq, fmt x4 [8] |
| 8 | 51 | TopDownDPMtPer.rs | default, clone, eq, fmt x4 [8] |

[8] Includes Debug and Display for the `TopDownDPMt*Inv` lock predicate.
The count excludes spec functions, proof functions, and
`SeqSpecsAndLemmas.rs` (spec and proof only).

## 5. Malformed annotations (0)

No `/// - Alg Analysis:` line is malformed. In `TopDownDPStPer.rs` and
`TopDownDPMtPer.rs` a `// Veracity:` line sits between the doc block and
`fn med_memoized` / `fn med_memoized_parallel`; the new line was placed
directly after the last Alg Analysis line, above the Veracity comment.
`TopDownDPMtPer.rs` also has a glued comment
`// Veracity: UNNEEDED proof block             // Veracity: NEEDED proof block`
inside `med_memoized_parallel`; it is not an annotation and was left alone.

## 6. Notable findings

1. **No parallel bottom-up MED exists.** All four BottomUpDP files fill
   the table row by row; the Mt files' `med_bottom_up_parallel` has no
   fork. Span is O(|S|·|T|) against Alg 51.1's O(|S|+|T|) diagonal
   pebbling. The old lines admit this ("sequential row fill despite Mt
   name"), so the gap is a known regression against APAS, not a wrong old
   analysis.
2. **The parallel top-down MED is costlier than the old lines claim.**
   `med_recursive_parallel` (MtEph, MtPer) clones both Vec-backed strings
   twice per call, adding a factor |S|+|T| to work and making span
   quadratic, and its memo has no in-flight deduplication, so concurrent
   branches recompute shared subproblems (exponential worst case). The
   old "Work O(|S|·|T|), Span O(|S|+|T|)" is wrong on both counts. Sharing
   the strings through `Arc` (or passing `&` into the closures) would
   remove the clone factor; the dedup issue needs the synchronization
   variables the prose describes.
3. **Misleading APAS line and names.** `TopDownDPStEph.rs` cites Alg 51.4
   with Span O(|S|+|T|), but the prose says the threaded-memo method is
   inherently sequential; the code's Span O(|S|·|T|) matches the prose.
   `med_memoized_concurrent` is sequential, and `TopDownDPStEph`'s trait
   says `set_s`/`set_t` are O(1) while the impls clear the memo table.
