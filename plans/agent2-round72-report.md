# R72 Agent 2: Iterator Standard Fixes

## Summary

Fixed all iterator standard compliance issues identified in the R71 review.
6 files changed across Chap19 and Chap37. No new holes introduced.

Validation: 4437 verified, 0 errors. RTT: 2528 passed. PTT: 145/145 passed.

## Changes

| # | Chap | File | Change | Effort |
|---|------|------|--------|--------|
| 1 | 19 | ArraySeqMtEphSlice.rs | Added full 10-component iterator standard (C1-C10) | Medium |
| 2 | 19 | ArraySeqMtEphSlice.rs | Moved `spec_backing_seq` from inherent impl to trait | Low |
| 3 | 37 | AVLTreeSeqMtPer.rs | Added full borrow iterator standard (C1-C10) | Medium |
| 4 | 37 | AVLTreeSeqMtPer.rs | Cleaned up duplicate TOC and section headers | Low |
| 5 | 37 | AVLTreeSeq.rs | Added `IntoIterator for &'a AVLTreeS<T>` (C10) | Low |
| 6 | 19 | ArraySeqStEph.rs | Removed duplicate section headers | Cosmetic |
| 7 | 19 | ArraySeqStPer.rs | Removed duplicate section headers | Cosmetic |
| 8 | 19 | ProveArraySeqMtEphSlice.rs (PTT) | Added proof hints for `spec_backing_seq` chain | Low |

## Detail

### Chap19/ArraySeqMtEphSlice.rs (1 → 10 components)

Previously returned raw `std::slice::Iter` with no custom iterator infrastructure.
Now has full standard:

- `ArraySeqMtEphSliceIter<'a, T>` wrapping `std::slice::Iter<'a, T>`
- View `(int, Seq<T>)` delegating to `self.inner@`
- `iter_invariant`: `0 <= it@.0 <= it@.1.len()`
- `Iterator::next` with standard two-arm ensures
- `ArraySeqMtEphSliceGhostIterator<'a, T>` with pos/elements/phantom
- `ForLoopGhostIteratorNew` and `ForLoopGhostIterator` (all 6 spec fns)
- `iter()` method on trait with ensures `it@.1 == self.spec_backing_seq()`
- `IntoIterator for &'a ArraySeqMtEphSliceS<T>`

Also moved `spec_backing_seq` from an inherent impl to the trait declaration,
since `iter()` is a trait method and needs `spec_backing_seq` visible in trait scope.

### Chap37/AVLTreeSeqMtPer.rs (1 → 10 components)

Previously had only a hand-rolled consuming iterator with `ensures true`.
Added full borrow iterator standard alongside the existing consuming iterator:

- `AVLTreeSeqMtPerBorrowIter<'a, T>` storing tree ref, pos, len
- View `(int, Seq<T>)` using `spec_inorder_values(self.tree.root)`
- `iter_invariant`: `0 <= it@.0 <= it@.1.len()`
- `Iterator::next` with `external_body` and standard two-arm ensures
- `AVLTreeSeqMtPerGhostIterator<'a, T>` with pos/elements/phantom
- `ForLoopGhostIteratorNew` and `ForLoopGhostIterator` (all 6 spec fns)
- `spec_inorder_values` spec fn (returns `Seq<T>` vs `spec_inorder`'s `Seq<T::V>`)
- `lemma_inorder_values_maps_to_inorder` bridging the two
- `iter()` on trait/impl
- `IntoIterator for &'a AVLTreeSeqMtPerS<T>`

Cleaned up duplicate TOC (removed second tab-indented copy) and removed all
duplicate section headers throughout the file.

### Chap37/AVLTreeSeq.rs (external_body justified)

Added `IntoIterator for &'a AVLTreeS<T>` delegating to `self.iter()`.

The `external_body` on `Iterator::next` was investigated and determined to be
structurally necessary: `Iterator::next()` has no `requires` clause (trait
constraint), so tree well-formedness cannot be established in the body. This
matches vstd's own pattern where `std::slice::Iter::next` uses
`assume_specification`. No change needed.

### PTT fix

`slice_iter_over_subslice` failed because the new `iter()` ensures uses
`self.spec_backing_seq()` instead of the old element-wise pattern. Added a
proof block before the for loop to connect `spec_backing_seq()[j]` through
`spec_index(j)` to the known element values via the `slice()` ensures chain.

## Not Changed

| # | Chap | File | Reason |
|---|------|------|--------|
| 1 | 18 | ArraySeqStPer.rs | IntoIterator outside verus! is a Verus AIR bug, not fixable |
| 2 | 23 | BalBinTreeStEph.rs | Consuming iterators justified for tree traversals |

## Hole Counts (unchanged)

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|------------|
| 1 | 19 | ArraySeqMtEphSlice.rs | 0 | 0 |
| 2 | 37 | AVLTreeSeqMtPer.rs | 0 | 0 |
| 3 | 37 | AVLTreeSeq.rs | 0 | 0 |
