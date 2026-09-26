# r228 Alg Analysis Review: Chap49

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications

`prompts/Chap49.txt` (Dynamic Programming: Subset Sum, Minimum Edit
Distance) derives the costs from the dynamic-programming DAG: work is the
sum over the vertices, span is the longest path.

| # | Spec | Operation | Work | Span |
|---|---|---|---|---|
| 1 | Alg 49.2 (memoized SS) | subset_sum, subset_sum_rec | k\|S\| | \|S\| |
| 2 | Alg 49.5 (memoized MED) | min_edit_distance, min_edit_distance_rec | \|S\|\|T\| | \|S\| + \|T\| |

The chapter gives no cost for the constructors, accessors, setters,
`clear_memo`, `memo_size`, or the Arc memo helpers; those lines are "no
textbook cost". The legacy "(Ch49 ref)" APAS lines on the `_rec`
functions restate Alg 49.2/49.5 and are treated as the textbook cost.

Notation: k = target sum, S = multiset, S and T = source and target
sequences, n = memo entries. The memo is a `std::collections::HashMap`,
O(1) expected per operation; `HashMap::clear` is O(n). The multiset and
sequences are Vec-backed (`ArraySeqStEph/StPer` in St files, Chap18
`ArraySeqMtPerS` and Chap19 `ArraySeqMtEphS` in Mt files), so a clone
is linear. St files are sequential: Span = Work.

## 2. Reviewed functions

Each trait function has an annotation on the trait declaration (T) and
another on the impl (I); each got its own line. Rows merge T and I when
both verdicts agree. "old wrong" means "does not match old analysis".

### 2a. SubsetSumStEph.rs (16 lines) and SubsetSumStPer.rs (11 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 49 | SubsetSumStEph.rs | subset_sum (T,I) | k\|S\|, \|S\| | W k\|S\|, S k\|S\| | W k\|S\|, S k\|S\| | not textbook [1] |
| 2 | 49 | SubsetSumStEph.rs | subset_sum_rec | k\|S\|, \|S\| | W k\|S\|, S k\|S\| | W k\|S\|, S k\|S\| | not textbook [1] |
| 3 | 49 | SubsetSumStEph.rs | new, from_multiset (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 49 | SubsetSumStEph.rs | multiset, memo_size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 49 | SubsetSumStEph.rs | multiset_mut | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 49 | SubsetSumStEph.rs | set, clear_memo (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 7 | 49 | SubsetSumStPer.rs | subset_sum (T,I) | k\|S\|, \|S\| | W k\|S\|, S k\|S\| | W k\|S\|, S k\|S\| | not textbook [1] |
| 8 | 49 | SubsetSumStPer.rs | subset_sum_rec | k\|S\|, \|S\| | W k\|S\|, S k\|S\| | W k\|S\|, S k\|S\| | not textbook [1] |
| 9 | 49 | SubsetSumStPer.rs | new, from_multiset (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 10 | 49 | SubsetSumStPer.rs | multiset, memo_size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |

### 2b. SubsetSumMtEph.rs (18 lines) and SubsetSumMtPer.rs (13 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 49 | SubsetSumMtEph.rs | subset_sum (T,I) | k\|S\|, \|S\| | W k\|S\|, S \|S\| | W k\|S\|², S \|S\|² | not textbook; old wrong [2] |
| 2 | 49 | SubsetSumMtEph.rs | subset_sum_rec | k\|S\|, \|S\| | W k\|S\|, S \|S\| | W k\|S\|², S \|S\|² | not textbook; old wrong [2] |
| 3 | 49 | SubsetSumMtEph.rs | new, from_multiset (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 49 | SubsetSumMtEph.rs | multiset, memo_size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 49 | SubsetSumMtEph.rs | multiset_mut | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 49 | SubsetSumMtEph.rs | set, clear_memo (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 7 | 49 | SubsetSumMtEph.rs | new_arc_memo, clone_arc_memo | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 8 | 49 | SubsetSumMtPer.rs | subset_sum (T,I) | k\|S\|, \|S\| | W k\|S\|, S \|S\| | W k\|S\|², S \|S\|² | not textbook; old wrong [2] |
| 9 | 49 | SubsetSumMtPer.rs | subset_sum_rec | k\|S\|, \|S\| | W k\|S\|, S \|S\| | W k\|S\|², S \|S\|² | not textbook; old wrong [2] |
| 10 | 49 | SubsetSumMtPer.rs | new, from_multiset (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 11 | 49 | SubsetSumMtPer.rs | multiset, memo_size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 49 | SubsetSumMtPer.rs | new_arc_memo, clone_arc_memo | none | W 1, S 1 | W 1, S 1 | no textbook cost |

### 2c. MinEditDistStEph.rs (21 lines) and MinEditDistStPer.rs (13 lines)

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 49 | MinEditDistStEph.rs | min_edit_distance (T,I) | \|S\|\|T\|, \|S\|+\|T\| | W, S \|S\|\|T\| | W, S \|S\|\|T\| | not textbook [1] |
| 2 | 49 | MinEditDistStEph.rs | min_edit_distance_rec | \|S\|\|T\|, \|S\|+\|T\| | W, S \|S\|\|T\| | W, S \|S\|\|T\| | not textbook [1] |
| 3 | 49 | MinEditDistStEph.rs | new, from_sequences (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 49 | MinEditDistStEph.rs | source, target (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 49 | MinEditDistStEph.rs | memo_size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 49 | MinEditDistStEph.rs | source_mut, target_mut | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 7 | 49 | MinEditDistStEph.rs | set_source, set_target (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 49 | MinEditDistStEph.rs | clear_memo (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 9 | 49 | MinEditDistStPer.rs | min_edit_distance (T,I) | \|S\|\|T\|, \|S\|+\|T\| | W, S \|S\|\|T\| | W, S \|S\|\|T\| | not textbook [1] |
| 10 | 49 | MinEditDistStPer.rs | min_edit_distance_rec | \|S\|\|T\|, \|S\|+\|T\| | W, S \|S\|\|T\| | W, S \|S\|\|T\| | not textbook [1] |
| 11 | 49 | MinEditDistStPer.rs | new, from_sequences (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 12 | 49 | MinEditDistStPer.rs | source, target, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |

### 2d. MinEditDistMtEph.rs (23 lines) and MinEditDistMtPer.rs (15 lines)

Let L = \|S\| + \|T\|.

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 49 | MinEditDistMtEph.rs | min_edit_distance (T,I) | \|S\|\|T\|, L | W \|S\|\|T\|, S L | W \|S\|\|T\|L, S L² | not textbook; old wrong [3] |
| 2 | 49 | MinEditDistMtEph.rs | min_edit_distance_rec | \|S\|\|T\|, L | W \|S\|\|T\|, S L | W \|S\|\|T\|L, S L² | not textbook; old wrong [3] |
| 3 | 49 | MinEditDistMtEph.rs | new, from_sequences (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 4 | 49 | MinEditDistMtEph.rs | source, target (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 5 | 49 | MinEditDistMtEph.rs | memo_size (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 6 | 49 | MinEditDistMtEph.rs | source_mut, target_mut | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 7 | 49 | MinEditDistMtEph.rs | set_source, set_target (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 8 | 49 | MinEditDistMtEph.rs | clear_memo (T,I) | none | W n, S n | W n, S n | no textbook cost |
| 9 | 49 | MinEditDistMtEph.rs | new_arc_memo, clone_arc_memo | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 10 | 49 | MinEditDistMtPer.rs | min_edit_distance (T,I) | \|S\|\|T\|, L | W \|S\|\|T\|, S L | W \|S\|\|T\|L, S L² | not textbook; old wrong [3] |
| 11 | 49 | MinEditDistMtPer.rs | min_edit_distance_rec | \|S\|\|T\|, L | W \|S\|\|T\|, S L | W \|S\|\|T\|L, S L² | not textbook; old wrong [3] |
| 12 | 49 | MinEditDistMtPer.rs | new, from_sequences (T,I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 13 | 49 | MinEditDistMtPer.rs | source, target, memo_size | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 14 | 49 | MinEditDistMtPer.rs | new_arc_memo, clone_arc_memo | none | W 1, S 1 | W 1, S 1 | no textbook cost |

All new Mt costs are expected bounds (HashMap) and assume no memo race;
see footnote 4.

### Footnotes

1. The St files run the memoized recursion sequentially: `subset_sum_rec`
   evaluates the two recursive calls with a sequential `||`, and
   `min_edit_distance_rec` evaluates the delete and insert calls one after
   the other. Work matches the textbook (at most |S|(k + 1) or
   (|S| + 1)(|T| + 1) states, O(1) expected HashMap work each), but Span =
   Work. The old lines record this as "ACCEPTED DIFFERENCE"; they agree
   with the new review. The StPer `subset_sum` and `min_edit_distance`
   impls first clone the inputs into a fresh solver, O(|S|) or
   O(|S| + |T|), which the DP work dominates.
2. `subset_sum_rec` in the Mt files builds the two branches with `join`,
   but before the join it clones the O(|S|) Vec-backed multiset twice (one
   copy per closure). With O(k|S|) states this gives Work O(k|S|²); the
   depth is |S| and each level pays an O(|S|) clone on the critical path,
   so Span O(|S|²).
3. `min_edit_distance_rec` in the Mt files likewise clones both source and
   target twice per mismatch call, O(|S| + |T|) each, before the `join`.
   With O(|S||T|) states and depth |S| + |T|, Work is
   O(|S||T|(|S| + |T|)) and Span O((|S| + |T|)²).
4. In both Mt recursions the shared `Arc<RwLock<HashMap>>` memo is read
   before the `join` and written after it. Two concurrent branches that
   reach the same state both miss and both compute it. The bounds in
   footnotes 2 and 3 assume each state is computed once; under the race
   the worst-case work is exponential (O(2^|S| |S|) for subset sum).

## 3. Counts (per annotation site)

130 new lines were added, one per annotated site, in 8 files.

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 24 |
| 3 | does not match old analysis | 12 |
| 4 | no textbook cost | 106 |
| 5 | unannotated functions | 50 |
| 6 | malformed annotations | 0 |

Rows 1, 2, and 4 partition the 130 lines; row 3 overlaps them.

| # | Chap | File | Lines | Match | Not match | Old wrong | No cost |
|---|---|---|---|---|---|---|---|
| 1 | 49 | SubsetSumStEph.rs | 16 | 0 | 3 | 0 | 13 |
| 2 | 49 | SubsetSumStPer.rs | 11 | 0 | 3 | 0 | 8 |
| 3 | 49 | SubsetSumMtEph.rs | 18 | 0 | 3 | 3 | 15 |
| 4 | 49 | SubsetSumMtPer.rs | 13 | 0 | 3 | 3 | 10 |
| 5 | 49 | MinEditDistStEph.rs | 21 | 0 | 3 | 0 | 18 |
| 6 | 49 | MinEditDistStPer.rs | 13 | 0 | 3 | 0 | 10 |
| 7 | 49 | MinEditDistMtEph.rs | 23 | 0 | 3 | 3 | 20 |
| 8 | 49 | MinEditDistMtPer.rs | 15 | 0 | 3 | 3 | 12 |

## 4. Unannotated functions (50)

| # | Chap | File | Functions |
|---|---|---|---|
| 1 | 49 | SubsetSumStEph.rs | clone, multiset_mut (I), fmt x2, into_iter x3 |
| 2 | 49 | SubsetSumStPer.rs | clone, eq, fmt x2, into_iter x2 |
| 3 | 49 | SubsetSumMtEph.rs | clone, multiset_mut (I), eq, fmt x3 |
| 4 | 49 | SubsetSumMtPer.rs | clone, eq, fmt x3 |
| 5 | 49 | MinEditDistStEph.rs | clone, source_mut/target_mut (I), fmt x2 |
| 6 | 49 | MinEditDistStEph.rs | into_iter x3 |
| 7 | 49 | MinEditDistStPer.rs | clone, eq, fmt x2, into_iter x2 |
| 8 | 49 | MinEditDistMtEph.rs | clone, source_mut/target_mut (I), eq |
| 9 | 49 | MinEditDistMtEph.rs | fmt x3 |
| 10 | 49 | MinEditDistMtPer.rs | clone, eq, fmt x3 |

The `*_mut` impls sit outside `verus!` and return `&mut` references,
O(1). The trait declarations of these `*_mut` functions are annotated.
The remaining entries are Clone, PartialEq, Debug, Display, and
IntoIterator impls.

## 5. Malformed annotations (0)

No `/// - Alg Analysis:` block in the chapter is malformed. The St-file
old lines use `--` instead of `—` as the separator; this is a style
variation, not a defect.

## 6. Notable findings

1. **The Mt memoized recursions pay a linear clone per call.**
   `subset_sum_rec` and `min_edit_distance_rec` in all four Mt files clone
   the Vec-backed multiset (or both sequences) twice per branching call so
   that each `join` closure owns a copy. This multiplies work and span by
   |S| (or |S| + |T|): subset sum costs W O(k|S|²), S O(|S|²) against the
   textbook k|S| and |S|; MED costs W O(|S||T|(|S| + |T|)),
   S O((|S| + |T|)²) against |S||T| and |S| + |T|. The old lines claimed
   the textbook costs (12 sites). Sharing the input through an `Arc`
   would remove the clones.
2. **The Mt memo races.** The `RwLock` memo is consulted before the
   `join` and updated after it, so concurrent branches recompute states
   the other branch is computing. Nothing bounds the duplication; in the
   worst case the parallel recursion does the exponential work of the
   unmemoized algorithm.
3. **The St files are sequential.** All four St files match the textbook
   work but have Span = Work; the old lines already flagged this as an
   accepted difference.
4. **Setters clear the memo.** `set`, `set_source`, `set_target`, and
   `clear_memo` are O(n) in the number of memo entries (HashMap::clear),
   up to O(k|S|) or O(|S||T|), not O(1).
