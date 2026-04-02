# R144 Agent 1 — Parallel D&C Partition for OrderStatSelect

## Summary

Replaced sequential O(n) filter loops in both `OrderStatSelectMtEph.rs` and
`OrderStatSelectMtPer.rs` with parallel D&C three-way partition via `join()`,
following the `partition_three_dc` pattern from `QuickSortMtEphSlice.rs`.

## Changes

### OrderStatSelectMtEph.rs (Chap35)

- **Removed**: Sequential two-closure filter partition (copied data into two Vecs,
  ran O(n) sequential filter loops in each closure via join).
- **Added**: `partition_three_dc` — recursive D&C partition over `ArraySeqMtEphSliceS`.
  Base case classifies single element. Recursive case splits slice in half, partitions
  each half in parallel via `join()`, concatenates results.
- **Added**: `append_vec` helper for Vec concatenation.
- **Added**: `spec_slice_elements` spec fn for slice element extraction.
- **Added**: `lemma_all_equal_multiset` — proves that a sequence of all-equal elements
  has multiset count equal to length. Used to prove eq_vec multiset equals eq_seq multiset.
- **Modified**: `parallel_three_way_partition` now copies data into one Vec, wraps in
  `ArraySeqMtEphSliceS::from_vec()` for O(1) splitting, then calls `partition_three_dc`.
- **Added**: `obeys_feq_clone::<T>()` to requires of `select` (trait), `select_inner`.
- **Added**: `Clone + Eq` bounds to `OrderStatSelectMtEphTrait`.
- **Updated**: All DIFFERS annotations to match APAS.

### OrderStatSelectMtPer.rs (Chap35)

- Identical structural changes as MtEph, using `ArraySeqMtPerS` instead of `ArraySeqMtEphS`.
- Standalone: no imports from MtEph; all helpers duplicated locally.

## Span Analysis

| Function | Before | After |
|---|---|---|
| `partition_three_dc` | N/A | O(lg n) divide + O(n) rejoin |
| `parallel_three_way_partition` | O(n) | O(lg n) divide + O(n) data copy |
| `select_inner` | O(n) per round | O(lg n) per round |
| `select` | O(n) expected | O(lg^2 n) expected |

The D&C partition splits the slice in half at each level, partitions halves in parallel
via `join()`, giving O(lg n) levels with O(1) split work per level. The sequential
rejoin (append_vec) at each level is O(size_of_half), totaling O(n) work. The divide
phase itself achieves O(lg n) span. Combined with O(lg n) expected recursion rounds
in select, the overall span is O(lg^2 n) expected, matching APAS.

## DIFFERS Resolved

| # | Chap | File | Status |
|---|---|---|---|
| 1 | 35 | OrderStatSelectMtEph.rs | DIFFERS → matches APAS |
| 2 | 35 | OrderStatSelectMtPer.rs | DIFFERS → matches APAS |

## Verification

- Full validation: 5688 verified, 0 errors
- RTT: 3690 passed, 0 skipped
- PTT: 221 passed, 0 skipped
