<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 56 Report

## Summary

This round validated and fixed the R55 sorted-method additions to `AVLTreeSetStEph.rs`.
Validation found 5 errors in the new `filter_sorted`, `intersection_sorted`,
`difference_sorted`, and `union_sorted` methods. All errors were fixed and the codebase
returns to 4487 verified, 0 errors.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:----:|:----:|:----:|
| 1 | 41 | AVLTreeSetStEph.rs | 1 | 1 | 0 |
| 2 | 41 | AVLTreeSetStPer.rs | 2 | 2 | 0 |

The 1 hole in `AVLTreeSetStEph.rs` is the `assume(combined@.len() + 1 < usize::MAX as nat)`
in the original `union` method (Task 1, blocked by `AVLTreeSetMtEph.rs` DO NOT TOUCH
constraint). The new `union_sorted` uses a proper requires clause instead.

## Errors Fixed

Validation found 5 errors in the newly added sorted methods:

| # | Chap | File | Error | Fix |
|---|:----:|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | Invariant `filter_sorted` loop | `lemma_empty_set_is_sorted` |
| 2 | 41 | AVLTreeSetStEph.rs | Invariant `intersection_sorted` loop | `lemma_empty_set_is_sorted` |
| 3 | 41 | AVLTreeSetStEph.rs | Invariant `difference_sorted` loop | `lemma_empty_set_is_sorted` |
| 4 | 41 | AVLTreeSetStEph.rs | Invariant `union_sorted` first loop | `lemma_empty_set_is_sorted` |
| 5 | 41 | AVLTreeSetStEph.rs | Capacity + precond in `union_sorted` | Strengthened second loop invariant |

## Techniques Used

**`lemma_empty_set_is_sorted`** (new proof fn in section 7): Proves that `Self::empty()`
satisfies `spec_elements_sorted()`. Proof chain:

1. `spec_avltreesetsteph_wf()` gives `no_duplicates()` on the backing sequence.
2. `unique_seq_to_set()` gives `elements@.len() == set@.len() == 0`.
3. `lemma_inorder_values_maps_to_views` shows `spec_inorder_values(root)` also has length 0.
4. `spec_seq_sorted(Seq::empty())` is vacuously true (no `(i,j)` pair to violate it).

**Strengthened `union_sorted` second loop invariant**: Added `self_len as nat == self@.len()`,
`other_len as nat == other@.len()`, and `self@.len() + other@.len() < usize::MAX as nat`.
These, combined with the loop invariant `combined@.len() <= self_len + j` and the loop
condition `j < other_len`, close the capacity assertion:
`combined@.len() + 1 <= self_len + other_len == self@.len() + other@.len() < usize::MAX`.

## What Was NOT Done

- **Task 1 (union capacity hole in `AVLTreeSetStEph.rs`)**: Still blocked. Adding a
  `self@.len() + other@.len() < usize::MAX` requires clause to `AVLTreeSetStEphTrait::union`
  would cascade into `AVLTreeSetMtEph.rs`, which is DO NOT TOUCH and currently clean.

## Verification Count

| Stage | Verified | Errors |
|---|:----:|:----:|
| Start of session | 4481 | 5 |
| After fixes | 4487 | 0 |
