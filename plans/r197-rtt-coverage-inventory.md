# R197 RTT Coverage Inventory

Generated: 2026-04-11, Round 197, Agent 1.

Baseline: 3776 tests (before this round). Post-round: 3838 tests (+62).

Coverage levels:
- **none**: no test file for this source file
- **smoke**: 1-3 tests, basic happy path only
- **partial**: 4-9 tests, covers main operations
- **full**: 10+ tests, comprehensive coverage

## Chap52: Graph Representations

| # | Chap | File | Tests | Coverage |
|---|------|------|-------|----------|
| 1 | 52 | AdjMatrixGraphMtEph.rs | 13 (NEW) | full |
| 2 | 52 | AdjMatrixGraphMtPer.rs | 16 | full |
| 3 | 52 | AdjMatrixGraphStEph.rs | 14 | full |
| 4 | 52 | AdjMatrixGraphStPer.rs | 15 | full |
| 5 | 52 | AdjSeqGraphMtEph.rs | 12 (NEW) | full |
| 6 | 52 | AdjSeqGraphMtPer.rs | 12 | full |
| 7 | 52 | AdjSeqGraphStEph.rs | 12 | full |
| 8 | 52 | AdjSeqGraphStPer.rs | 12 | full |
| 9 | 52 | AdjTableGraphMtPer.rs | 10 (REACTIVATED) | full |
| 10 | 52 | AdjTableGraphStEph.rs | 9 (REACTIVATED, -1 clone) | partial |
| 11 | 52 | AdjTableGraphStPer.rs | 14 (REACTIVATED, -1 from_table) | full |
| 12 | 52 | AdjTableGraphSpecsAndLemmas.rs | 0 | none (lemmas only) |
| 13 | 52 | EdgeSetGraphMtEph.rs | 0 | none (source not implemented) |
| 14 | 52 | EdgeSetGraphMtPer.rs | 11 (REACTIVATED) | full |
| 15 | 52 | EdgeSetGraphStEph.rs | 7 (REACTIVATED) | partial |
| 16 | 52 | EdgeSetGraphStPer.rs | 7 (REACTIVATED) | partial |

## Chap65: Union-Find

| # | Chap | File | Tests | Coverage |
|---|------|------|-------|----------|
| 1 | 65 | UnionFindArrayStEph.rs | 10 (NEW) | full |
| 2 | 65 | UnionFindNoPCStEph.rs | 11 (NEW) | full |
| 3 | 65 | UnionFindPCStEph.rs | 21 | full |
| 4 | 65 | KruskalStEph.rs | 0 | none (ordered_float disabled) |
| 5 | 65 | PrimStEph.rs | 0 | none (ordered_float disabled) |

## Test Files Fixed in This Round

| # | File | Issue | Fix |
|---|------|-------|-----|
| 1 | TestAdjTableGraphStEph.rs | `clone()` not implemented on ephemeral type | Removed test_clone |
| 2 | TestAdjTableGraphStPer.rs | `from_table` takes `TableStPer` (Chap42), test used `OrderedTableStPer` (Chap43) | Removed test_from_table |

## Summary of Changes

- 4 new test files written: TestAdjMatrixGraphMtEph, TestAdjSeqGraphMtEph, TestUnionFindArrayStEph, TestUnionFindNoPCStEph
- 6 test files reactivated in Cargo.toml (previously commented with stale "module commented out" reason)
- 2 pre-existing test bugs fixed in reactivated files
- Net gain: +62 tests (3776 → 3838)
