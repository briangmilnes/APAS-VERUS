# Agent 1 Round 77 Report: Register Missing RTTs

## Summary

- **Tests registered**: 72 new active `[[test]]` entries (193 → 221 active, after commenting out 44 that cannot compile)
- **Tests that compile and pass**: 221 active entries, 2774 tests pass (up from 2619, delta +155)
- **Tests that needed fixes**: 1 (TestBSTBBAlphaMtEph: removed incorrect height assertion)
- **Tests that remain broken**: 44 (commented out with reasons)
- **Commented-out `#[test]` functions re-enabled**: 0 (all 13 have real bugs)

## Baseline

| Metric | Before | After |
|--------|--------|-------|
| Active `[[test]]` entries | 193 | 221 |
| RTT passed | 2619 | 2774 |
| Delta | | +155 |

## What was done

1. **Uncommented 62 test entries** that were commented out in Cargo.toml with `path = "disabled"` or correct paths but marked as disabled. Set correct paths for all.

2. **Added 10 new Chap06 entries** for Weighed graph test files that had no Cargo.toml entry at all (TestWeighedDirGraph*, TestWeighedUnDirGraph*, TestLabDirGraphMtEph, TestLabUnDirGraphMtEph).

3. **Commented out 44 entries** that cannot compile, with documented reasons:

### Tests commented out: module commented out in lib.rs (30)

| # | Chap | File | Reason |
|---|------|------|--------|
| 1 | 26 | TestETSPStEph | ETSPStEph module commented out |
| 2 | 26 | TestETSPMtEph | ETSPMtEph module commented out |
| 3 | 28 | TestMaxContigSubSumOptMtEph | module commented out |
| 4 | 28 | TestMaxContigSubSumOptStEph | module commented out |
| 5 | 43 | TestExample43_1 | Example43_1 module commented out |
| 6 | 43 | TestOrderedSetStPer | module commented out |
| 7 | 43 | TestOrderedSetMtEph | module commented out |
| 8 | 43 | TestOrderedTableMtPer | depends on OrderedSetMtEph |
| 9 | 44 | TestDocumentIndex | DocumentIndex module commented out |
| 10 | 44 | TestExample44_1 | depends on DocumentIndex |
| 11 | 52 | TestEdgeSetGraphStPer | module commented out (broken) |
| 12 | 52 | TestEdgeSetGraphStEph | module commented out |
| 13 | 52 | TestAdjTableGraphStEph | module commented out |
| 14 | 52 | TestAdjTableGraphStPer | module commented out |
| 15 | 53 | TestGraphSearchMtPer | module commented out |
| 16 | 55 | TestCycleDetectStEph | module commented out |
| 17 | 55 | TestCycleDetectStPer | module commented out |
| 18 | 55 | TestDFSStEph | module commented out |
| 19 | 55 | TestDFSStPer | module commented out |
| 20 | 55 | TestSCCStEph | module commented out |
| 21 | 55 | TestSCCStPer | module commented out |
| 22 | 55 | TestTopoSortStEph | module commented out |
| 23 | 55 | TestTopoSortStPer | module commented out |
| 24 | 59 | TestJohnsonMtEphI64 | module commented out |
| 25 | 61 | TestEdgeContractionMtEph | depends on VertexMatchingMtEph |
| 26 | 61 | TestEdgeContractionStEph | depends on VertexMatchingStEph |
| 27 | 61 | TestVertexMatchingMtEph | module commented out |
| 28 | 61 | TestVertexMatchingStEph | module commented out |
| 29 | 62 | TestStarContractionMtEph | depends on StarPartitionMtEph |
| 30 | 62 | TestStarPartitionMtEph | module commented out (uses rand) |

### Tests commented out: uses old apas_ai crate, no source module (8)

| # | Chap | File |
|---|------|------|
| 1 | 06 | TestWeighedDirGraphMtEphFloat |
| 2 | 06 | TestWeighedDirGraphMtEphInt |
| 3 | 06 | TestWeighedDirGraphStEphFloat |
| 4 | 06 | TestWeighedDirGraphStEphInt |
| 5 | 06 | TestWeighedUnDirGraphMtEphFloat |
| 6 | 06 | TestWeighedUnDirGraphMtEphInt |
| 7 | 06 | TestWeighedUnDirGraphStEphFloat |
| 8 | 06 | TestWeighedUnDirGraphStEphInt |

### Tests commented out: API changed (2)

| # | Chap | File | Reason |
|---|------|------|--------|
| 1 | 06 | TestLabDirGraphMtEph | out_neighbors/in_neighbors renamed to n_plus/n_minus (27 errors) |
| 2 | 06 | TestLabUnDirGraphMtEph | neighbors method removed (4 errors) |

### Tests commented out: missing type (4)

| # | Chap | File | Reason |
|---|------|------|--------|
| 1 | 56 | TestSSSPResultStEphF64 | F64Dist type does not exist |
| 2 | 56 | TestSSSPResultStPerF64 | F64Dist type does not exist |
| 3 | 56 | TestAllPairsResultStEphF64 | F64Dist type does not exist |
| 4 | 56 | TestAllPairsResultStPerF64 | F64Dist type does not exist |

## Test fix: TestBSTBBAlphaMtEph

Removed incorrect `assert!(height <= 4)` in `test_insert_sequential`. BB-alpha trees do not guarantee height <= 4 for 7 sequential inserts. The size assertion (`assert_eq!(tree.size(), 7)`) is the meaningful check.

## Commented-out #[test] functions (13 examined, 0 re-enabled)

| # | Chap | File | Count | Reason |
|---|------|------|-------|--------|
| 1 | 37 | TestAVLTreeSeq | 2 | set() panics on out-of-bounds instead of returning Err |
| 2 | 41 | TestAVLTreeSetMtPer | 10 | Thread explosion via recursive ParaPair! calls |
| 3 | 06 | TestLabUnDirGraphStEph | 1 | normalize_edge method was removed from trait |

All have documented, real bugs. None can be re-enabled without source changes.

## Build warnings

28 tests emit `unexpected cfg condition value: all_chapters` — pre-existing feature gate issue, not introduced by this change.
