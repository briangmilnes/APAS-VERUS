# R142 Agent 2 — AVL to_seq and from_seq Analysis Report

## Summary

Investigated the 4 reported DIFFERS in Chap41 AVLTreeSetMtEph and AVLTreeSetMtPer
for `to_seq` and `from_seq`. Found that from_seq was already parallelized in a prior
round. Updated to_seq annotations to ACCEPTED DIFFERENCE — parallelization requires
tree-based sequence concat that Chap37 sequence types do not provide.

## Findings

### from_seq — Already Parallel (both files, no changes)

Both `AVLTreeSetMtEph::from_seq` and `AVLTreeSetMtPer::from_seq` are already
implemented with parallel D&C via `from_vec_dc` / `from_vec_dc_per`:

- Split Vec at midpoint, recurse on halves via `join()`, union results
- Work O(n lg n), Span O(lg^2 n) — matches APAS Example 41.3 parallel variant
- Annotations already correct

### to_seq — ACCEPTED DIFFERENCE (both files, annotation update)

APAS CS 41.4 specifies `toSeq`: Work O(|a|), Span O(lg |a|).

The O(lg n) span requires a tree-based sequence output type with O(lg n)
concatenation (join). The approach would be:

1. Expose BST root via expose() in O(1)
2. In parallel via join(): to_seq(left), to_seq(right)
3. Concatenate: left_seq ++ singleton(key) ++ right_seq in O(lg n)

Step 3 is the blocker: neither `AVLTreeSeqStEphS` nor `AVLTreeSeqMtPerS` has a
concat/join operation. Without it, any materialization approach (collect to Vec,
build from_vec) is inherently O(n) span.

Updated both trait and impl annotations from implicit "sequential" to explicit
"ACCEPTED DIFFERENCE" with the reason.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | AVLTreeSetMtEph.rs | Updated to_seq trait + impl annotations to ACCEPTED DIFFERENCE |
| 2 | 41 | AVLTreeSetMtPer.rs | Updated to_seq trait + impl annotations to ACCEPTED DIFFERENCE |

## Validation

- `scripts/validate.sh isolate Chap41`: 2200 verified, 0 errors
- `scripts/rtt.sh`: 3690 tests passed

## What Would Unblock to_seq Parallelization

Adding `fn concat(left: Self, right: Self) -> Self` to AVLTreeSeqStEphS and
AVLTreeSeqMtPerS with O(lg n) work/span (AVL tree join). This is standard for
tree-based sequences but is a Chap37 infrastructure addition, outside this round's
scope.
