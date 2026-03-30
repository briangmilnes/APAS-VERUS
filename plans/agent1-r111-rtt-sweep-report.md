# Agent 1 — R111 RTT Coverage Sweep Report

## Summary

Added 98 new runtime tests across 16 chapters. RTT count: 3099 → 3197.
All 3197 tests pass.

## Chapters Visited

| # | Chap | File Modified | Tests Before | Tests After | Delta |
|---|------|--------------|-------------|------------|-------|
| 1 | 03 | TestInsertionSortStEph.rs | 7 | 15 | +8 |
| 2 | 05 | TestSetMtEph.rs | 4 | 15 | +11 |
| 3 | 11 | TestFibonacciMt.rs | 5 | 11 | +6 |
| 4 | 17 | TestMathSeq.rs | 28 | 40 | +12 |
| 5 | 26 | TestMergeSortMtPer.rs | 5 | 10 | +5 |
| 6 | 35 | TestOrderStatSelectStEph.rs | 10 | 15 | +5 |
| 7 | 35 | TestOrderStatSelectStPer.rs | 10 | 14 | +4 |
| 8 | 35 | TestOrderStatSelectMtEph.rs | 10 | 15 | +5 |
| 9 | 35 | TestOrderStatSelectMtPer.rs | 10 | 14 | +4 |
| 10 | 36 | TestQuickSortStEph.rs | 2 | 9 | +7 |
| 11 | 36 | TestQuickSortMtEph.rs | 4 | 8 | +4 |
| 12 | 38 | TestBSTParaStEph.rs | 13 | 23 | +10 |
| 13 | 40 | TestBSTKeyValueStEph.rs | 6 | 14 | +8 |
| 14 | 41 | TestArraySetStEph.rs | 7 | 15 | +8 |
| 15 | 47 | TestVecChainedHashTable.rs | 9 | 16 | +7 |
| 16 | 47 | TestStructChainedHashTable.rs | 11 | 16 | +5 |
| 17 | 56 | TestSSSPResultStEphI64.rs | 5 | 9 | +4 |
| 18 | 62 | TestStarPartitionStEph.rs | 5 | 9 | +4 |
| 19 | 62 | TestStarContractionStEph.rs | 4 | 7 | +3 |
| 20 | 63 | TestConnectivityStEph.rs | 7 | 16 | +9 |
| 21 | 64 | TestSpanTreeStEph.rs | 5 | 13 | +8 |

### Tests Added to Commented-Out Files (Not Counted in RTT)

These test files are commented out in Cargo.toml and will count when re-enabled:

| # | Chap | File | Tests Added |
|---|------|------|------------|
| 1 | 61 | TestEdgeContractionStEph.rs | +6 |
| 2 | 61 | TestEdgeContractionMtEph.rs | +5 |
| 3 | 61 | TestVertexMatchingStEph.rs | +6 |
| 4 | 61 | TestVertexMatchingMtEph.rs | +3 |
| 5 | 62 | TestStarPartitionMtEph.rs | +3 |
| 6 | 62 | TestStarContractionMtEph.rs | +3 |
| 7 | 63 | TestConnectivityMtEph.rs | +4 |

Total in commented-out files: +30 tests (will become active when uncommented).

## Notable Edge Cases Tested

- **InsertionSort**: max u64 values, all-same elements, length preservation across sizes 0-19
- **Fibonacci**: recursive vs iterative agreement, recurrence property, monotonicity
- **OrderStatSelect**: out-of-bounds for sizes 0-9, i32::MIN/MAX values, median selection
- **QuickSort**: all 6 permutations of 3 elements, negative values, multiset preservation
- **BST Para**: self-intersection/difference/union, filter-none/all, delete-all, split at min/max
- **Hash Tables**: worst-case all-same-bucket collisions, delete-all, insert-after-delete
- **Graph Connectivity**: isolated vertices, complete graph K5, path graph, two-vertex connected/disconnected
- **Spanning Tree**: K4, path graph, star graph, disconnected pair, verify-too-many-edges

## Modules Skipped

| # | Chap | Module | Reason |
|---|------|--------|--------|
| 1 | 02 | HFSchedulerMtEph | Complex setup (global thread pool), already tested via TestParaPairs |
| 2 | 02 | FibonacciHFScheduler | TSM/scheduler integration, tested via TestFibonacciMt |
| 3 | 11 | FibonacciMt* (4 files) | `#[cfg(verus_keep_ghost)]` — Verus-only, not testable at runtime |
| 4 | 19 | ArraySeqMtEphSlice | Module commented out in Cargo.toml |
| 5 | 47 | ChainedHashTable.rs | Trait-only file, no concrete types |
| 6 | 47 | FlatHashTable.rs | Trait-only file, no concrete types |
| 7 | 52 | AdjMatrixGraphMtEph | Commented out in Cargo.toml |
| 8 | 52 | AdjSeqGraphMtEph | Commented out in Cargo.toml |
| 9 | 65 | All | Commented out of lib.rs |
