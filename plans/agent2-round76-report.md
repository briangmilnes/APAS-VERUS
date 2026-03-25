# Agent 2 — Round 76 Report

## Objective

Rewrite `src/Chap37/BSTSetSplayMtEph.rs` to remove `std::collections::BTreeSet` dependency,
following the pattern Agent 5 used on BSTSetAVLMtEph in R75.

## Results

- **Verified**: 4794 → 4810 (+16)
- **RTT**: 2619 passed
- **PTT**: 157 passed
- **Errors/warnings**: 0

## Hole Summary

The raw hole count stayed at 13, but composition changed fundamentally:

| Metric | Before | After |
|--------|--------|-------|
| external_body | 13 | 5 |
| assume (obeys_feq_clone) | 0 | 8 |
| Total holes | 13 | 13 |
| Verified functions | 4794 | 4810 |

The 5 remaining external_body are structural (same as AVL):
- `union`, `intersection`, `difference` — ParaPair! macro (thread spawning)
- `filter`, `reduce` — FnMut closure (Verus limitation)

The 8 assumes are all `obeys_feq_clone::<T>()` — the established workaround pattern
used identically in the AVL variant.

## Changes Made

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 37 | BSTSetSplayMtEph.rs | imports | Removed `BTreeSet`, added `obeys_feq_clone` |
| 2 | 37 | BSTSetSplayMtEph.rs | values_vec | Removed external_body, while loop + nth() |
| 3 | 37 | BSTSetSplayMtEph.rs | rebuild_from_vec | Removed external_body, while loop + index |
| 4 | 37 | BSTSetSplayMtEph.rs | from_sorted_iter | Renamed to build_from_vec |
| 5 | 37 | BSTSetSplayMtEph.rs | delete | Removed external_body, while loop + nth() |
| 6 | 37 | BSTSetSplayMtEph.rs | split | Removed external_body, while loop + nth() |
| 7 | 37 | BSTSetSplayMtEph.rs | join_pair | Removed external_body + BTreeSet, two insert loops |
| 8 | 37 | BSTSetSplayMtEph.rs | join_m | Removed external_body + BTreeSet, two insert loops |
| 9 | 37 | BSTSetSplayMtEph.rs | filter | Rewrote body (kept external_body for FnMut) |
| 10 | 37 | BSTSetSplayMtEph.rs | reduce | Rewrote body (kept external_body for FnMut) |
| 11 | 37 | BSTSetSplayMtEph.rs | iter_in_order | Added obeys_feq_clone assume |
| 12 | 37 | BSTSetSplayMtEph.rs | iter | Removed external_body, uses values_vec |
| 13 | 37 | BSTSetSplayMtEph.rs | into_iter (consuming) | Removed external_body, uses BSTSetSplayMtEphIter |

## Differences from AVL Rewrite

- Splay has `+ 'static` bound on `T` in most signatures — preserved as-is.
- Splay had `from_sorted_iter` name (AVL used `build_from_vec`) — renamed for consistency.
- Splay's consuming `IntoIterator` used `std::vec::IntoIter<T>` — changed to
  `BSTSetSplayMtEphIter<T>` to match AVL pattern.

## Global Status

- 46 chapters, 42 clean, 4 holed, 76 holes, 244 modules
