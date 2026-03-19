# Plan: Iterative-vs-Recursive Naming Alignment

## Naming Convention

If the APAS textbook presents an algorithm recursively, the **default name** is the
recursive implementation. An iterative alternative gets the `_iter` suffix. If the
textbook presents it iteratively, the default is iterative and the recursive
alternative gets `_rec`.

## Current State

27 functions across 11 files are iterative where the textbook says recursive.
Zero existing `_iter` or `_rec` alternatives exist. All mismatches are in Chap41
(15 functions) and Chap43 (12 functions).

## Phase 1: Rename Current Iterative Impls (No New Proof Burden)

Rename the existing iterative implementations to `_iter`. The trait keeps the
default name and delegates to `_iter` via a one-line body. No new proofs needed —
we're just adding a suffix and a delegation wrapper.

### Chap41 — AVLTreeSetStEph.rs (8 functions)

| # | Chap | File | Current Name | Rename To | Notes |
|---|------|------|-------------|-----------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | insert | insert_iter | while loop, binary search for position |
| 2 | 41 | AVLTreeSetStEph.rs | delete | delete_iter | while loop, filter into result_vec |
| 3 | 41 | AVLTreeSetStEph.rs | find | find_iter | while loop, linear scan |
| 4 | 41 | AVLTreeSetStEph.rs | from_seq | from_seq_iter | while loop calling insert each time |
| 5 | 41 | AVLTreeSetStEph.rs | filter | filter_iter | while loop over sequence indices |
| 6 | 41 | AVLTreeSetStEph.rs | intersection | intersection_iter | while loop, calls find for each element |
| 7 | 41 | AVLTreeSetStEph.rs | union | union_iter | two consecutive while loops |
| 8 | 41 | AVLTreeSetStEph.rs | difference | difference_iter | while loop, calls find to exclude |

### Chap41 — AVLTreeSetStPer.rs (7 functions)

| # | Chap | File | Current Name | Rename To | Notes |
|---|------|------|-------------|-----------|-------|
| 9 | 41 | AVLTreeSetStPer.rs | insert | insert_iter | same pattern as StEph |
| 10 | 41 | AVLTreeSetStPer.rs | delete | delete_iter | same pattern as StEph |
| 11 | 41 | AVLTreeSetStPer.rs | find | find_iter | same pattern as StEph |
| 12 | 41 | AVLTreeSetStPer.rs | filter | filter_iter | same pattern as StEph |
| 13 | 41 | AVLTreeSetStPer.rs | intersection | intersection_iter | same pattern as StEph |
| 14 | 41 | AVLTreeSetStPer.rs | union | union_iter | same pattern as StEph |
| 15 | 41 | AVLTreeSetStPer.rs | difference | difference_iter | same pattern as StEph |

### Chap43 — OrderedSetStEph.rs (6 functions)

| # | Chap | File | Current Name | Rename To | Notes |
|---|------|------|-------------|-----------|-------|
| 16 | 43 | OrderedSetStEph.rs | first | first_iter | while loop linear scan |
| 17 | 43 | OrderedSetStEph.rs | last | last_iter | while loop linear scan |
| 18 | 43 | OrderedSetStEph.rs | previous | previous_iter | to_seq then scan backward |
| 19 | 43 | OrderedSetStEph.rs | next | next_iter | to_seq then scan forward |
| 20 | 43 | OrderedSetStEph.rs | rank | rank_iter | to_seq then count loop |
| 21 | 43 | OrderedSetStEph.rs | select | select_iter | to_seq then index |

### Chap43 — OrderedTableStEph.rs (6 functions)

| # | Chap | File | Current Name | Rename To | Notes |
|---|------|------|-------------|-----------|-------|
| 22 | 43 | OrderedTableStEph.rs | first_key | first_key_iter | while loop linear scan |
| 23 | 43 | OrderedTableStEph.rs | last_key | last_key_iter | while loop linear scan |
| 24 | 43 | OrderedTableStEph.rs | previous_key | previous_key_iter | collect then iterate sorted |
| 25 | 43 | OrderedTableStEph.rs | next_key | next_key_iter | collect then iterate sorted |
| 26 | 43 | OrderedTableStEph.rs | rank_key | rank_key_iter | collect then count via loop |
| 27 | 43 | OrderedTableStEph.rs | select_key | select_key_iter | collect then index |

### Execution Notes

- Rename the function body to `_iter`.
- The trait method keeps the default name and calls `_iter`.
- Same ensures on both — no spec duplication, just delegation.
- Callers are unaffected (they call the trait method).
- Run `scripts/validate.sh` after each file.

## Phase 2: Write Recursive Impls Under Default Names (Future)

Write the textbook-matching recursive implementations. The trait method then
delegates to the recursive impl (the new default), and callers who want the
iterative version call `_iter` explicitly.

### Priority

1. Chap41 `find` — O(n) linear scan → O(log n) recursive binary search.
2. Chap41 `insert`/`delete` — recursive tree descent with rebalancing.
3. Chap43 ordering ops — exploit tree structure for O(log n) operations.

### R39 Overlap

R39 Agents 1 and 2 are restructuring OrderedTableStEph/StPer to use
AVLTreeSetStEph/StPer as backing store. This restructure will rewrite
the 12 Chap43 OrderedTable functions (items 16-27) to delegate to tree
operations, which may naturally produce recursive implementations or
trivial delegations that don't need renaming. Wait for R39 results
before executing Chap43 renames.

## Scope Constraint

This plan adds NO new proof burden. Phase 1 is pure renaming + delegation.
Phase 2 adds new function bodies but those are new code with their own proofs —
existing proofs are untouched.
