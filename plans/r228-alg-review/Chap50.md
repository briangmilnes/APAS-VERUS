# r228 Alg Analysis Review: Chap50

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap50.txt` (Dynamic Programming: Optimal Binary Search Trees,
Matrix Chain) derives the OBST costs from its DAG and states that matrix
chain "has the same cost bounds".

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 50.2 / 50.3 (memoized OBST) | optimal_cost, obst_rec | n³ | n lg n |
| 2 | Matrix chain (same bounds as OBST) | optimal_cost, matrix_chain_rec | n³ | n lg n |

The chapter gives no cost for constructors, accessors, setters,
`clear_memo`, `memo_size`, `multiply_cost`, or the split-reduction
helpers; those lines are "no textbook cost". The legacy "APAS (Ch50 ref)"
lines on `into_iter` and `fmt` impls are treated as the textbook cost.

Notation: n = number of keys (OBST) or matrices (matrix chain);
l = hi − lo. The memo is a `std::collections::HashMap`, O(1) expected per
operation, and holds up to O(n²) entries, so `HashMap::clear` is O(n²).
St files are sequential: Span = Work.

## 2. Reviewed functions

Each trait function has an annotation on the trait declaration (T) and
another on the impl (I); each got its own line. Rows merge T and I when
both verdicts agree. "old wrong" means "does not match old analysis".

### 2a. OptBinSearchTreeStEph.rs (26 lines) and OptBinSearchTreeStPer.rs (19 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 50 | OptBinSearchTreeStEph.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 2 | 50 | OptBinSearchTreeStEph.rs | obst_rec_st_eph | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 3 | 50 | OptBinSearchTreeStEph.rs | set_key_prob (T,I) | none | W 1, S 1 | W n², S n² | no cost; old wrong [2] |
| 4 | 50 | OptBinSearchTreeStEph.rs | update_prob (T,I) | none | W 1, S 1 | W n², S n² | no cost; old wrong [2] |
| 5 | 50 | OptBinSearchTreeStEph.rs | clear_memo (T,I) | none | W 1, S 1 | W n², S n² | no cost; old wrong [2] |
| 6 | 50 | OptBinSearchTreeStEph.rs | new, from_key_probs (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 7 | 50 | OptBinSearchTreeStEph.rs | from_keys_probs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 50 | OptBinSearchTreeStEph.rs | keys, num_keys, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 9 | 50 | OptBinSearchTreeStEph.rs | into_iter x3, fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 10 | 50 | OptBinSearchTreeStPer.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 11 | 50 | OptBinSearchTreeStPer.rs | obst_rec_st_per | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 12 | 50 | OptBinSearchTreeStPer.rs | new, from_key_probs (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 13 | 50 | OptBinSearchTreeStPer.rs | from_keys_probs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 14 | 50 | OptBinSearchTreeStPer.rs | keys, num_keys, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 15 | 50 | OptBinSearchTreeStPer.rs | into_iter x2, fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### 2b. OptBinSearchTreeMtEph.rs (27 lines) and OptBinSearchTreeMtPer.rs (20 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 50 | OptBinSearchTreeMtEph.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n lg n | W n³, S n lg n exp | not textbook [3] |
| 2 | 50 | OptBinSearchTreeMtEph.rs | obst_rec | n³, n lg n | W n³, S n lg n | W n³, S n lg n exp | not textbook [3] |
| 3 | 50 | OptBinSearchTreeMtEph.rs | parallel_min_split_cost | none | W l, S lg l | W l, S lg l [4] | no textbook cost |
| 4 | 50 | OptBinSearchTreeMtEph.rs | from_key_probs (T) | none | W n, S n | W 1, S 1 | no cost; old wrong [5] |
| 5 | 50 | OptBinSearchTreeMtEph.rs | from_key_probs (I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 50 | OptBinSearchTreeMtEph.rs | clear_memo (T) | none | W 1, S 1 | W n², S n² | no cost; old wrong [2] |
| 7 | 50 | OptBinSearchTreeMtEph.rs | clear_memo (I) | none | W m, S m | W n², S n² | no textbook cost |
| 8 | 50 | OptBinSearchTreeMtEph.rs | keys, set_key_prob (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 9 | 50 | OptBinSearchTreeMtEph.rs | update_prob (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 10 | 50 | OptBinSearchTreeMtEph.rs | new, num_keys, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 11 | 50 | OptBinSearchTreeMtEph.rs | from_keys_probs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 12 | 50 | OptBinSearchTreeMtEph.rs | into_iter x3 | n, n | W n, S n | W n, S n | matches textbook |
| 13 | 50 | OptBinSearchTreeMtEph.rs | fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 50 | OptBinSearchTreeMtPer.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n lg n | W n³, S n lg n exp | not textbook [3] |
| 15 | 50 | OptBinSearchTreeMtPer.rs | obst_rec | n³, n lg n | W n³, S n lg n | W n³, S n lg n exp | not textbook [3] |
| 16 | 50 | OptBinSearchTreeMtPer.rs | parallel_min_split_cost | none | W l, S lg l | W l, S lg l [4] | no textbook cost |
| 17 | 50 | OptBinSearchTreeMtPer.rs | new, from_key_probs (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 18 | 50 | OptBinSearchTreeMtPer.rs | from_keys_probs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 19 | 50 | OptBinSearchTreeMtPer.rs | keys, num_keys, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 20 | 50 | OptBinSearchTreeMtPer.rs | into_iter x2 | 1 / n | W 1 / W n | W 1 / W n | matches textbook |
| 21 | 50 | OptBinSearchTreeMtPer.rs | fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### 2c. MatrixChainStEph.rs (29 lines) and MatrixChainStPer.rs (22 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 50 | MatrixChainStEph.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 2 | 50 | MatrixChainStEph.rs | matrix_chain_rec (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 3 | 50 | MatrixChainStEph.rs | set_dimension (T,I) | none | W n, S n | W n², S n² | no cost; old wrong [2] |
| 4 | 50 | MatrixChainStEph.rs | update_dimension (T,I) | none | W n, S n | W n², S n² | no cost; old wrong [2] |
| 5 | 50 | MatrixChainStEph.rs | clear_memo (T,I) | none | W n, S n | W n², S n² | no cost; old wrong [2] |
| 6 | 50 | MatrixChainStEph.rs | new, from_dimensions (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 7 | 50 | MatrixChainStEph.rs | from_dim_pairs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 50 | MatrixChainStEph.rs | dimensions, num_matrices | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 9 | 50 | MatrixChainStEph.rs | memo_size, multiply_cost | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 10 | 50 | MatrixChainStEph.rs | into_iter x3, fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 11 | 50 | MatrixChainStPer.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 12 | 50 | MatrixChainStPer.rs | matrix_chain_rec (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [1] |
| 13 | 50 | MatrixChainStPer.rs | new, from_dimensions (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 14 | 50 | MatrixChainStPer.rs | from_dim_pairs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 15 | 50 | MatrixChainStPer.rs | dimensions, num_matrices | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 16 | 50 | MatrixChainStPer.rs | memo_size, multiply_cost | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 17 | 50 | MatrixChainStPer.rs | into_iter x2, fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

### 2d. MatrixChainMtEph.rs (31 lines) and MatrixChainMtPer.rs (24 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 50 | MatrixChainMtEph.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [6] |
| 2 | 50 | MatrixChainMtEph.rs | matrix_chain_rec (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [6] |
| 3 | 50 | MatrixChainMtEph.rs | parallel_min_reduction (T,I) | none | W n, S n | W \|costs\|, S \|costs\| | no textbook cost [6] |
| 4 | 50 | MatrixChainMtEph.rs | from_dimensions (T,I) | none | W n, S n | W 1, S 1 | no cost; old wrong [5] |
| 5 | 50 | MatrixChainMtEph.rs | from_dim_pairs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 6 | 50 | MatrixChainMtEph.rs | dimensions (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 7 | 50 | MatrixChainMtEph.rs | set_dimension (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 50 | MatrixChainMtEph.rs | update_dimension (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 9 | 50 | MatrixChainMtEph.rs | clear_memo (T,I) | none | W 1, S 1 | W 1, S 1 [7] | no textbook cost |
| 10 | 50 | MatrixChainMtEph.rs | new, num_matrices (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 11 | 50 | MatrixChainMtEph.rs | memo_size, multiply_cost | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 50 | MatrixChainMtEph.rs | into_iter x3 | n, n | W n, S n | W n, S n | matches textbook |
| 13 | 50 | MatrixChainMtEph.rs | fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |
| 14 | 50 | MatrixChainMtPer.rs | optimal_cost (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [6] |
| 15 | 50 | MatrixChainMtPer.rs | matrix_chain_rec (T,I) | n³, n lg n | W n³, S n³ | W n³, S n³ | not textbook [6] |
| 16 | 50 | MatrixChainMtPer.rs | parallel_min_reduction (T,I) | none | W n, S n | W \|costs\|, S \|costs\| | no textbook cost [6] |
| 17 | 50 | MatrixChainMtPer.rs | from_dimensions (T,I) | none | W n, S n | W 1, S 1 | no cost; old wrong [5] |
| 18 | 50 | MatrixChainMtPer.rs | from_dim_pairs (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 19 | 50 | MatrixChainMtPer.rs | new, dimensions (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 20 | 50 | MatrixChainMtPer.rs | num_matrices, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 21 | 50 | MatrixChainMtPer.rs | multiply_cost (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 22 | 50 | MatrixChainMtPer.rs | into_iter x2 | 1 / n | W 1 / W n | W 1 / W n | matches textbook |
| 23 | 50 | MatrixChainMtPer.rs | fmt x2 | 1, 1 | W 1, S 1 | W 1, S 1 | matches textbook |

All DP costs are expected bounds because of the HashMap memo.

### Footnotes

1. The St files run the memoized recursion and the split loop
   sequentially: O(n²) memo states, each with an O(l) split loop (and, in
   OBST, an O(l) probability sum), so Work O(n³) and Span = Work. The
   OBST old lines mark this "ACCEPTED DIFFERENCE"; the new lines agree on
   the costs. The StPer `optimal_cost` impls first clone the input into a
   fresh solver, O(n), which the DP dominates.
2. The St `set_key_prob`, `update_prob`, `clear_memo` (OBST) and
   `set_dimension`, `update_dimension`, `clear_memo` (matrix chain) call
   `HashMap::clear`, which is linear in the O(n²) memo entries. The old
   lines say O(1) (OBST) or O(n) (matrix chain). The same holds for the
   OBST MtEph trait `clear_memo`, whose impl line already says O(m) for m
   memo entries.
3. OBST Mt `obst_rec` has the textbook shape: O(n²) states, an O(1)
   prefix-sum lookup, and a split reduction of O(l) work and O(lg l)
   span, giving S(l) ≤ lg l + max_k (S(k) + S(l − k − 1)), i.e.
   O(n lg n). But the shared memo is read before the reduction and
   written after it, so concurrent branches that reach the same (i, l)
   both compute it. The O(n³) work holds only when lookups hit; the
   duplication is unbounded, exponential in the worst case. The verdict
   is "does not match textbook" on that ground, although the nominal
   bounds equal the textbook's.
4. `parallel_min_split_cost` splits the range with `join` and reduces by
   min: O(l) work and O(lg l) span, excluding the `obst_rec` calls. At
   each leaf the two `obst_rec` calls (left and right subtree) run one
   after the other.
5. `from_key_probs` (OBST MtEph trait line) and `from_dimensions` (matrix
   chain MtEph and MtPer, both lines) move the given Vec into an
   `Arc<RwLock<..>>` or `Arc` without traversing it: O(1), not O(n).
6. The matrix chain Mt files contain no `join`. `matrix_chain_rec` is a
   sequential memoized recursion with a sequential split loop (the MtEph
   version takes a read lock per `multiply_cost` call), so Span = Work =
   O(n³) against the textbook n lg n. `parallel_min_reduction` is a
   sequential linear scan, O(|costs|), and no function in either file
   calls it.
7. MtEph `clear_memo` replaces the memo `Arc` with a fresh empty one, O(1);
   the old memo is dropped when its last reference goes, which costs
   O(n²) at that point.

## 3. Counts (per annotation site)

198 new lines were added, one per annotated site, in 8 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 36 |
| 2 | does not match textbook | 28 |
| 3 | does not match old analysis | 18 |
| 4 | no textbook cost | 134 |
| 5 | unannotated functions | 62 |
| 6 | malformed annotations | 1 |

Rows 1, 2, and 4 partition the 198 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 50 | OptBinSearchTreeStEph.rs | 26 | 5 | 3 | 6 | 18 |
| 2 | 50 | OptBinSearchTreeStPer.rs | 19 | 4 | 3 | 0 | 12 |
| 3 | 50 | OptBinSearchTreeMtEph.rs | 27 | 5 | 3 | 2 | 19 |
| 4 | 50 | OptBinSearchTreeMtPer.rs | 20 | 4 | 3 | 0 | 13 |
| 5 | 50 | MatrixChainStEph.rs | 29 | 5 | 4 | 6 | 20 |
| 6 | 50 | MatrixChainStPer.rs | 22 | 4 | 4 | 0 | 14 |
| 7 | 50 | MatrixChainMtEph.rs | 31 | 5 | 4 | 2 | 22 |
| 8 | 50 | MatrixChainMtPer.rs | 24 | 4 | 4 | 2 | 16 |

## 4. Unannotated functions (62)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 50 | OptBinSearchTreeStEph.rs | clone x2, eq x2, fmt x4 |
| 2 | 50 | OptBinSearchTreeStPer.rs | clone x2, eq x2, fmt x4 |
| 3 | 50 | OptBinSearchTreeMtEph.rs | clone x2, eq x2, fmt x7 |
| 4 | 50 | OptBinSearchTreeMtPer.rs | clone x2, eq x2, fmt x5 |
| 5 | 50 | MatrixChainStEph.rs | clone, eq, fmt x3 |
| 6 | 50 | MatrixChainStPer.rs | clone, eq, fmt x3 |
| 7 | 50 | MatrixChainMtEph.rs | clone, eq, fmt x7 |
| 8 | 50 | MatrixChainMtPer.rs | clone, eq, fmt x5 |

All are Clone, PartialEq, Debug, and Display impls (for the solver
structs, their View structs, `KeyProb`, and `MatrixDim`), plus the Debug
and Display impls of the RwLock predicate structs in the Mt files. No
algorithmic function is unannotated.

## 5. Malformed annotations (1)

| # | Chap | File | Function | Defect |
|---|---|---|---|---|
| 1 | 50 | OptBinSearchTreeStEph.rs | from_key_probs (I) | Veracity comment glued to line [8] |

8. Line 203 reads
   `// Veracity: UNNEEDED proof block         /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)`.
   The old annotation follows a `//` comment on the same line, so it is a
   plain comment, not a doc line. The new `///` line was added on the next
   line; the glued line was left as is.

## 6. Notable findings

1. **Matrix chain Mt files are sequential.** `MatrixChainMtEph.rs` and
   `MatrixChainMtPer.rs` contain no `join`; the memoized recursion and the
   split loop run on one thread, so Span is O(n³) against the textbook's
   O(n lg n). The helper `parallel_min_reduction` is a sequential scan
   that nothing calls. The old lines recorded Span O(n³) without flagging
   the gap.
2. **The OBST Mt memo races.** `obst_rec` has the right parallel
   structure (a `join`-based min reduction over split points, O(n lg n)
   span), but the shared `RwLock` memo is read before and written after
   the reduction. Concurrent branches recompute shared subproblems, so the
   O(n³) work bound holds only if lookups hit; the worst case is
   exponential. Both Per variants also mutate a shared memo through an
   `Arc<RwLock>`, so they are not persistent in the memo.
3. **Old lines misstate memo-clear and constructor costs (18 sites).**
   Setters and `clear_memo` in the St files, and the OBST MtEph trait
   `clear_memo`, call `HashMap::clear` on up to O(n²) entries: O(n²), not
   O(1) or O(n). Conversely `from_key_probs` and the Mt `from_dimensions`
   only move a Vec into a lock or an `Arc`: O(1), not O(n).
4. **All four St files are sequential.** Work O(n³) matches, Span = Work.
   The OBST St old lines already accept the difference.
5. **The "(Ch50 ref)" APAS lines on `into_iter` and `fmt`** are not
   textbook costs; they restate the code-review line. They account for
   all 36 "matches textbook" lines.
