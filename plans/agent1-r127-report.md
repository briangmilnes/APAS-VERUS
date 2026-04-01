# Agent 1 — R127 Report: Fix stale DIFFERS annotations + St parallel claims

## Task A: Updated Mt DIFFERS annotations (14 → 10 remaining)

4 annotations upgraded from DIFFERS to "matches APAS" (code already uses ParaPair!).
10 annotations updated with accurate DIFFERS reasons reflecting actual parallelism.

| # | Chap | File | Function | Old annotation | New annotation |
|---|------|------|----------|---------------|----------------|
| 1 | 35 | OrderStatSelectMtEph.rs | `select` | DIFFERS: partition uses sequential loops inside join() | DIFFERS: parallel recursion via join(), but sequential O(n) filter loops dominate span |
| 2 | 35 | OrderStatSelectMtPer.rs | `select` | DIFFERS: partition uses sequential loops inside join() | DIFFERS: parallel recursion via join(), but sequential O(n) filter loops dominate span |
| 3 | 36 | QuickSortMtEph.rs | `quick_sort_first` | DIFFERS: sequential partition loop, parallel recursion via ParaPair but partition dominates span | DIFFERS: parallel recursion via ParaPair!, but sequential O(n) partition dominates span |
| 4 | 36 | QuickSortMtEph.rs | `quick_sort_median3` | DIFFERS: sequential partition O(n) per level; parallel recursion via ParaPair gives geometric span | DIFFERS: parallel recursion via ParaPair!, but sequential O(n) partition dominates span |
| 5 | 36 | QuickSortMtEph.rs | `quick_sort_random` | DIFFERS: sequential partition O(n) per level; parallel recursion via ParaPair | DIFFERS: parallel recursion via ParaPair!, but sequential O(n) partition dominates span |
| 6 | 38 | BSTParaMtEph.rs | `union` | DIFFERS: sequential recursion, no parallel split | matches APAS; parallel recursion via ParaPair! |
| 7 | 38 | BSTParaMtEph.rs | `intersect` | DIFFERS: sequential recursion, no parallel split | matches APAS; parallel recursion via ParaPair! |
| 8 | 38 | BSTParaMtEph.rs | `difference` | DIFFERS: sequential recursion, no parallel split | matches APAS; parallel recursion via ParaPair! |
| 9 | 38 | BSTParaMtEph.rs | `filter` | DIFFERS: sequential tree traversal | DIFFERS: sequential recursion in filter_inner (spec_fn not Send) |
| 10 | 38 | BSTParaMtEph.rs | `reduce` | DIFFERS: sequential tree traversal | matches APAS; parallel recursion via ParaPair! |
| 11 | 66 | BoruvkaMtEph.rs | `boruvka_mst_mt` (trait) | DIFFERS: sequential loops despite Mt naming, no join/spawn | DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers |
| 12 | 66 | BoruvkaMtEph.rs | `boruvka_mst_mt_with_seed` (trait) | DIFFERS: sequential loops despite Mt naming, no join/spawn | DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers |
| 13 | 66 | BoruvkaMtEph.rs | `boruvka_mst_mt` (impl) | DIFFERS: sequential loops despite Mt naming, no join/spawn | DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers |
| 14 | 66 | BoruvkaMtEph.rs | `boruvka_mst_mt_with_seed` (impl) | DIFFERS: sequential loops despite Mt naming, no join/spawn | DIFFERS: sequential O(lg n) loop, each round O(lg m) span via ParaPair! helpers |

## Task B: St parallel claim fixes (67 → 0 remaining)

Removed `Parallelism O(...)` metadata, trailing periods, `Claude-Opus-4.6:` non-standard
format, and `Span = Work =` patterns from St file Code review annotations.

| # | Chap | File | Fixes |
|---|------|------|-------|
| 1 | 06 | LabDirGraphStEph.rs | 11 |
| 2 | 18 | ArraySeqStEph.rs | 1 |
| 3 | 18 | LinkedListStEph.rs | 5 |
| 4 | 18 | LinkedListStPer.rs | 5 |
| 5 | 19 | ArraySeqStEph.rs | 4 |
| 6 | 19 | ArraySeqStPer.rs | 3 |
| 7 | 23 | BalBinTreeStEph.rs | 3 |
| 8 | 23 | PrimTreeSeqStPer.rs | 5 |
| 9 | 35 | OrderStatSelectStEph.rs | 1 |
| 10 | 35 | OrderStatSelectStPer.rs | 1 |
| 11 | 47 | DoubleHashFlatHashTableStEph.rs | 4 |
| 12 | 47 | LinProbFlatHashTableStEph.rs | 4 |
| 13 | 47 | LinkedListChainedHashTableStEph.rs | 2 |
| 14 | 47 | VecChainedHashTableStEph.rs | 3 |
| 15 | 56 | PathWeightUtilsStEph.rs | 4 |
| 16 | 56 | PathWeightUtilsStPer.rs | 4 |
| 17 | 57 | StackStEph.rs | 6 |
| 18 | 64 | SpanTreeStEph.rs | 1 |
| **Total** | | | **67** |

## Summary

- 14 Mt DIFFERS annotations updated (4 promoted to "matches APAS", 10 corrected)
- 67 St parallel claims fixed (all resolved to zero)
- No code changes — annotations only
- 22 files modified across 11 chapters
