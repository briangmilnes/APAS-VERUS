# R143 Agent 1 — Parallel Partition for QuickSortMtEphSlice

## Summary

Replaced sequential O(n) three-way partition loop in all three slice-backed quicksort
variants with a parallel D&C partition (`partition_three_dc`) using `join()`. The
Vec-backed variants (QuickSortMtEph.rs) received ACCEPTED DIFFERENCE annotations.

## Changes

### QuickSortMtEphSlice.rs

| # | Chap | Change | Description |
|---|------|--------|-------------|
| 1 | 36 | New fn `append_vec` | Append one Vec onto another with multiset proof |
| 2 | 36 | New fn `partition_three_dc` | D&C three-way partition via join; O(n) work, O(lg n) span |
| 3 | 36 | Modified `quick_sort_first` | Replaced sequential partition loop with `partition_three_dc` call |
| 4 | 36 | Modified `quick_sort_median3` | Same replacement |
| 5 | 36 | Modified `quick_sort_random` | Same replacement |
| 6 | 36 | Updated 3 trait + 3 impl annotations | Removed DIFFERS, updated span analysis |

### QuickSortMtEph.rs

| # | Chap | Change | Description |
|---|------|--------|-------------|
| 7 | 36 | Updated 3 trait annotations | DIFFERS -> ACCEPTED DIFFERENCE: Vec-backed |

## DIFFERS Resolution

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 36 | QuickSortMtEphSlice.rs | DIFFERS (3 functions) | Resolved — parallel partition matches APAS |
| 2 | 36 | QuickSortMtEph.rs | DIFFERS (3 functions) | ACCEPTED DIFFERENCE: Vec-backed |

## Algorithm: `partition_three_dc`

D&C three-way partition matching APAS's parallel filter structure:
- **Base n=0**: three empty Vecs
- **Base n=1**: classify element via `TotalOrder::cmp`, place in appropriate Vec
- **Recursive n>=2**: split at midpoint, `join(partition(left_half), partition(right_half))`,
  concatenate corresponding Vecs (left++left, eq++eq, right++right)

Ensures: element classification (< / == / > pivot), multiset conservation, length conservation.

Key proof techniques:
- Index-based triggers (`#![trigger result.0@[j]]`) instead of `T::le(...)` triggers to avoid
  circular trigger matching
- Multiset conservation via `lemma_multiset_commutative` on split halves
- Pivot-in-equals proof via multiset counting (pivot not in left/right by !=, all copies in equals)

## Verification

- `scripts/validate.sh isolate Chap36`: 938 verified, 0 errors in QuickSortMtEphSlice.rs
  (1 pre-existing error in QuickSortMtEph.rs, confirmed pre-existing via git stash test)
- `scripts/rtt.sh`: 3690 passed, 0 failed
