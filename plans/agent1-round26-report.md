# Agent 1 — Round 26 Report

## Summary

R26 focused on Chap37 spec infrastructure and cleanup:
- Duplicated `spec_is_bst_link` into BSTSplayMtEph.rs and BSTRBMtEph.rs
- Added `TotalOrder` bound to all type parameters in 4 MtEph files
- Strengthened `height_rec` ensures in both BST MtEph files
- Removed all 12 `requires true` instances across AVLTreeSeq files
- Assessed 3 AVLTreeSeq external_body holes (all irreducible)

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 37 | AVLTreeSeq.rs | 1 | 1 | 0 | Iterator next: value-view gap |
| 2 | 37 | AVLTreeSeqMtPer.rs | 2 | 2 | 0 | Thread boundaries (build_balanced, subseq_copy) |
| 3 | 37 | AVLTreeSeqStEph.rs | 0 | 0 | 0 | Clean |
| 4 | 37 | AVLTreeSeqStPer.rs | 0 | 0 | 0 | Clean |
| 5 | 37 | BSTSplayMtEph.rs | 0 | 0 | 0 | Added spec_is_bst_link + TotalOrder |
| 6 | 37 | BSTRBMtEph.rs | 0 | 0 | 0 | Added spec_is_bst_link + TotalOrder |
| 7 | 37 | BSTSetSplayMtEph.rs | 0 | 0 | 0 | Added TotalOrder bound |
| 8 | 37 | BSTSetRBMtEph.rs | 0 | 0 | 0 | Added TotalOrder bound |

**Chap37 total: 3 holes (unchanged)**
**Project total: 217 holes, 34 clean chapters (unchanged)**

## Verification Counts

- validate: 4103 verified, 0 errors
- RTT: 2613 passed
- PTT: 147 passed

## What Was Done

### Part 1 & 2: BSTSplayMtEph.rs + BSTRBMtEph.rs — spec_is_bst_link

**Completed:**
- Added `use crate::vstdplus::total_order::total_order::TotalOrder` import
- Changed all `StTInMtT + Ord` bounds to `StTInMtT + Ord + TotalOrder` in 4 files (BSTSplayMtEph, BSTRBMtEph, BSTSetSplayMtEph, BSTSetRBMtEph)
- Duplicated `spec_is_bst_link` from BSTSplayStEph.rs into both MtEph files, adapted for by-value `Link<T>` and `link_contains` (vs `spec_contains_link`)
- Strengthened `height_rec` in both files: added `requires link_height(*link) < usize::MAX as nat` and `ensures h as nat == link_height(*link)`

**Not completed:**
- Did NOT add `spec_is_bst_link` as requires/ensures to helper functions (splay, bst_insert, insert_link, find_link, min_link, max_link). Adding requires to these helpers cascades to the trait impl callers (insert, find, etc.), which would require `spec_bstsplaymteph_wf` to include BST ordering. That breaks `from_sorted_slice` and `insert` without substantial proof code to establish BST preservation through rotations and insertions. The spec_is_bst_link is defined and available for future proof work.

### Part 3: AVLTreeSeq external_body holes — assessed, all irreducible

1. **AVLTreeSeq.rs `next`** (line 1116): Iterator::next trait has no `requires` clause in Rust. The ensures needs `result matches Some(v) ==> v@ == self@[old_cursor]` but bridging from `T: Clone` physical equality to `@`-equality requires `obeys_feq_full` which can't be assumed in Iterator::next.

2. **AVLTreeSeqMtPer.rs `build_balanced_from_slice`** (line 506): Thread boundary — uses `para_pair` which requires `'static` closures; `&[T]` slices can't be captured. Plus clone-view bridging for Arc nodes.

3. **AVLTreeSeqMtPer.rs `subseq_copy`** (line 621): Thread boundary — uses spawn/wait with Mutex for parallel subrange copying. Same `'static` closure constraint.

### Part 4: requires true removal — all 12 removed

| # | Chap | File | Function | Line |
|---|------|------|----------|------|
| 1 | 37 | AVLTreeSeq.rs | cached_height | 346 |
| 2 | 37 | AVLTreeSeqStEph.rs | h_fn | 328 |
| 3 | 37 | AVLTreeSeqStEph.rs | clone_link | 675 |
| 4 | 37 | AVLTreeSeqStEph.rs | push_left_iter | 1041 |
| 5 | 37 | AVLTreeSeqStPer.rs | height_fn | 231 |
| 6 | 37 | AVLTreeSeqStPer.rs | size_fn | 241 |
| 7 | 37 | AVLTreeSeqStPer.rs | inorder_collect | 487 |
| 8 | 37 | AVLTreeSeqStPer.rs | build_balanced_from_slice | 499 |
| 9 | 37 | AVLTreeSeqStPer.rs | push_left_iter_stper | 732 |
| 10 | 37 | AVLTreeSeqMtPer.rs | height_fn | 260 |
| 11 | 37 | AVLTreeSeqMtPer.rs | size_fn | 270 |
| 12 | 37 | AVLTreeSeqMtPer.rs | inorder_collect | 498 |

These functions genuinely need no preconditions — they operate on raw links/nodes with built-in decreases clauses.

## Techniques Used

- **TotalOrder bound propagation**: Added spec-level ordering capability to MtEph files by threading `TotalOrder` through all generic bounds, enabling `spec_is_bst_link` to use `T::le`.
- **Spec duplication for Mt standalone**: Defined `spec_is_bst_link` directly in MtEph files (not imported from St), following the Mt standalone rule.
- **Irreducibility assessment**: Traced each external_body hole through Verus's type system constraints (Iterator trait limitations, `'static` closure requirements, value-view equality gap).

## Remaining Holes in Chap37

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 37 | AVLTreeSeq.rs | next | Iterator::next can't have requires; value-view equality gap |
| 2 | 37 | AVLTreeSeqMtPer.rs | build_balanced_from_slice | Thread boundary: `'static` closure + clone-view bridging |
| 3 | 37 | AVLTreeSeqMtPer.rs | subseq_copy | Thread boundary: spawn/wait + Mutex, `'static` closure |
