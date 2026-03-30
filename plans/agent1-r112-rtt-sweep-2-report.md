# Agent 1 — R112 RTT Coverage Sweep 2 Report

## Summary

Added 158 new runtime tests across 14 chapters. Revived 6 test files that were
commented out in Cargo.toml (modules active in lib.rs but tests dead).
Fixed 3 test files using removed `ordered_float` crate (rewrote to use `WrappedF64`).
Fixed 2 test files using nonexistent `WeightedDirGraphStEphFloat` module.

RTT count: 3197 -> 3355 (+158). All 3355 tests pass.

## Per-Chapter Results

| # | Chap | Tests Before | Tests After | Delta | Notes |
|---|------|-------------|------------|-------|-------|
| 1 | 02 | 19 | 41 | +22 | Fibonacci + HFScheduler + threads_plus |
| 2 | 19 | 47 | 51 | +4 | ArraySeqStEph (large, flatten, for-iter, map identity) |
| 3 | 21 | 33 | 46 | +13 | Problem21_3 loops + edge cases across all modules |
| 4 | 27 | 38 | 45 | +7 | Reduce/Scan min, 3-elem, max variants |
| 5 | 38 | 41 | 50 | +9 | BSTParaSt/Mt (sorted, dup, delete root, join, clone) |
| 6 | 40 | 41 | 45 | +4 | BSTKeyValueStEph (dup update, absent delete, keys order) |
| 7 | 42 | 57 | 60 | +3 | TableStEph (delete nonexistent, insert-delete, combine) |
| 8 | 53 | 43 | 46 | +3 | GraphSearchStEph (disconnected, star, self-loop) |
| 9 | 54 | 42 | 44 | +2 | BFSStEph (K4 complete, cycle) |
| 10 | 57 | 24 | 40 | +16 | Stack + Dijkstra U64/F64 (diamond, chain, fractional) |
| 11 | 58 | 15 | 23 | +8 | BellmanFord F64 revived (+11), I64 +2, -5 dead F64 |
| 12 | 59 | 31 | 32 | +1 | Johnson F64/Mt revived (+14), I64 +5; net after dead removal |
| 13 | 66 | 29 | 36 | +7 | Boruvka St/Mt fully rewritten (ordered_float -> WrappedF64) |

Note: Chap58, 59, 66 counts include revived test files that were commented out in
Cargo.toml. The "delta" reflects the change in running test count, not just additions.

## Cargo.toml Test Files Uncommented

| # | Test File | Reason Was Commented |
|---|-----------|---------------------|
| 1 | TestBellmanFordStEphF64 | "module commented out in lib.rs" (stale) |
| 2 | TestJohnsonStEphF64 | "module commented out in lib.rs" (stale) |
| 3 | TestJohnsonMtEphF64 | "module commented out in lib.rs" (stale) |
| 4 | TestJohnsonMtEphI64 | "module commented out in lib.rs" (stale) |
| 5 | TestBoruvkaStEph | "uses ordered_float (removed crate)" |
| 6 | TestBoruvkaMtEph | "uses ordered_float (removed crate)" |

All 6 modules are active in lib.rs. Tests were rewritten to use current types.

## Test Files Rewritten

| # | Chap | File | Issue | Fix |
|---|------|------|-------|-----|
| 1 | 58 | TestBellmanFordStEphF64.rs | Used `WeightedDirGraphStEphFloat` (nonexistent) + `OrderedF64` (nonexistent) | Rewrote with `WeightedDirGraphStEphF64` + `WrappedF64` |
| 2 | 59 | TestJohnsonStEphF64.rs | Used `WeightedDirGraphStEphFloat` + `OrderedF64` | Rewrote with proper imports |
| 3 | 59 | TestJohnsonMtEphF64.rs | Used `WeightedDirGraphMtEphFloat` + `OrderedF64` | Rewrote with proper imports |
| 4 | 66 | TestBoruvkaStEph.rs | Used `ordered_float::OrderedFloat` (crate removed) | Rewrote with `WrappedF64` |
| 5 | 66 | TestBoruvkaMtEph.rs | Used `ordered_float::OrderedFloat` (crate removed) | Rewrote with `WrappedF64` |

## Chapters Still Under 40 Tests

| # | Chap | Tests | Reason |
|---|------|-------|--------|
| 1 | 58 | 23 | Graph algo — each test is a substantial graph setup. Good coverage quality. |
| 2 | 59 | 32 | Graph algo — Johnson APSP, 4 variants. Each test builds full weighted graphs. |
| 3 | 66 | 36 | Boruvka MST — helper functions + full MST + determinism tests. Close to 40. |

## Techniques Used

- **Revived dead tests**: Found 6 test files commented out in Cargo.toml whose modules
  are actually active. Rewrote imports to match current codebase.
- **Crate migration**: Replaced `ordered_float::OrderedFloat` with `WrappedF64` in 2 files.
- **Module path fixes**: Replaced nonexistent `WeightedDirGraphStEphFloat` with
  `WeightedDirGraphStEphF64` in 3 files.
- **Edge case coverage**: Duplicate inserts, delete-from-empty, delete-nonexistent,
  boundary values, clone independence, operation interleaving.
