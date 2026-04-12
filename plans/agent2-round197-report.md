# Agent 2 — Round 197 Report

## Summary (continued from prior session)

R197 Part 2: Wrote all missing bench files for chapters with no Mt/new-St coverage,
registered 47 new `[[bench]]` entries in `Cargo.toml`, ran every chapter isolation
bench sequentially, fixed four import errors, confirmed all 17 chapters pass.

---

## New Bench Files Added (this session)

| # | Chap | File | Notes |
|---|------|------|-------|
| 1 | 05 | `BenchSetMtEph.rs` | New |
| 2 | 06 | `BenchDirGraphMtEph.rs` | New |
| 3 | 06 | `BenchLabDirGraphMtEph.rs` | New |
| 4 | 06 | `BenchLabDirGraphStEph.rs` | New |
| 5 | 06 | `BenchLabUnDirGraphMtEph.rs` | New |
| 6 | 06 | `BenchLabUnDirGraphStEph.rs` | New |
| 7 | 06 | `BenchUnDirGraphMtEph.rs` | New |
| 8 | 06 | `BenchUnDirGraphStEph.rs` | New |
| 9 | 18 | `BenchChap18ArraySeqMtEph.rs` | Renamed from `BenchArraySeqMtEph.rs` (name collision fix) |
| 10 | 18 | `BenchChap18ArraySeqMtEphSlice.rs` | Renamed |
| 11 | 18 | `BenchChap18ArraySeqMtPer.rs` | Renamed |
| 12 | 18 | `BenchChap18ArraySeqStEph.rs` | Renamed |
| 13 | 18 | `BenchChap18ArraySeqStPer.rs` | Renamed |
| 14 | 19 | `BenchArraySeqMtEph.rs` | New |
| 15 | 19 | `BenchArraySeqMtEphSlice.rs` | New |
| 16 | 19 | `BenchArraySeqStPer.rs` | New |
| 17 | 26 | `BenchDivConReduceMtPer.rs` | New |
| 18 | 26 | `BenchETSPMtEph.rs` | New |
| 19 | 26 | `BenchMergeSortMtPer.rs` | New |
| 20 | 26 | `BenchScanDCMtPer.rs` | New |
| 21 | 28 | `BenchMaxContigSubSumDivConOptMtEph.rs` | New |
| 22 | 28 | `BenchMaxContigSubSumDivConOptStEph.rs` | New |
| 23 | 28 | `BenchMaxContigSubSumReducedMcsseStEph.rs` | New |
| 24 | 35 | `BenchOrderStatSelectMtEph.rs` | New |
| 25 | 35 | `BenchOrderStatSelectMtPer.rs` | New |
| 26 | 35 | `BenchOrderStatSelectStPer.rs` | New |
| 27 | 36 | `BenchQuickSortMtEphSlice.rs` | New |
| 28 | 39 | `BenchBSTParaTreapMtEph.rs` | New |
| 29 | 40 | `BenchBSTReducedStEph.rs` | New |
| 30 | 49 | `BenchMinEditDistMtEph.rs` | New |
| 31 | 49 | `BenchMinEditDistMtPer.rs` | New |
| 32 | 49 | `BenchSubsetSumMtEph.rs` | New |
| 33 | 49 | `BenchSubsetSumMtPer.rs` | New |
| 34 | 50 | `BenchMatrixChainMtEph.rs` | New |
| 35 | 50 | `BenchMatrixChainMtPer.rs` | New |
| 36 | 50 | `BenchOptBinSearchTreeMtEph.rs` | New |
| 37 | 50 | `BenchOptBinSearchTreeMtPer.rs` | New |
| 38 | 53 | `BenchGraphSearchMtPer.rs` | New |
| 39 | 53 | `BenchGraphSearchStEph.rs` | New |
| 40 | 53 | `BenchGraphSearchStPer.rs` | New |
| 41 | 53 | `BenchPQMinStEph.rs` | New |
| 42 | 53 | `BenchPQMinStPer.rs` | New |
| 43 | 57 | `BenchDijkstraStEphF64.rs` | New |
| 44 | 58 | `BenchBellmanFordStEphF64.rs` | New |
| 45 | 59 | `BenchJohnsonMtEphF64.rs` | New |
| 46 | 59 | `BenchJohnsonStEphF64.rs` | New |
| 47 | 64 | `BenchTSPApproxStEph.rs` | New |

## Fixes Applied During Isolation Runs

| # | Chap | File | Fix |
|---|------|------|-----|
| 1 | 35 | `BenchOrderStatSelectMtEph.rs` | Added `ArraySeqMtEphTrait` to import |
| 2 | 36 | `BenchQuickSortMtEphSlice.rs` | Changed Chap18 import to Chap19 for `ArraySeqMtEphSliceS` |
| 3 | 49 | `BenchMinEditDistMtEph.rs` | Added `ArraySeqMtEphTrait` to import |
| 4 | 49 | `BenchSubsetSumMtEph.rs` | Added `ArraySeqMtEphTrait` to import |

## Isolation Run Results

All 17 chapters passed `scripts/bench.sh isolate ChapNN`:

| # | Chap | Result |
|---|------|--------|
| 1 | 05 | PASS |
| 2 | 06 | PASS |
| 3 | 18 | PASS |
| 4 | 19 | PASS |
| 5 | 26 | PASS |
| 6 | 28 | PASS |
| 7 | 35 | PASS |
| 8 | 36 | PASS |
| 9 | 39 | PASS |
| 10 | 40 | PASS |
| 11 | 49 | PASS |
| 12 | 50 | PASS |
| 13 | 53 | PASS |
| 14 | 57 | PASS |
| 15 | 58 | PASS |
| 16 | 59 | PASS |
| 17 | 64 | PASS |

Full bench run (`scripts/bench.sh all`) awaits user approval.

No `src/` files modified. No `assume`, `accept`, `admit`, or `external_body` added.
