# R37 Plan

## State After R36

- 4282 verified, 0 errors
- **75 actionable holes** (41 SFPs now info-only, not counted)
- 37 clean chapters, 9 holed
- Chap37 and Chap45 are effectively clean (0 real actionable; SFPs/Example only)

## Key Findings Affecting Assignments

1. **Chap41 Mt functions are parallel algorithms** (fork-join), not simple
   RwLock delegations. Proving them requires verified closure specs for
   join(). Deferred to later round.
2. **OrderedTableMtEph wraps TableMtEph directly** (no RwLock). Its 6
   ordering ops use the same linear-scan algorithm as OrderedTableStEph
   (proved by agent2 in R36). Can reuse proof technique.
3. **Hash table lookup is proved** in all 3 open-addressing implementations.
   Insert/delete use identical probe sequences — lookup proofs are templates.

## Four Non-Conflicting Paths

| # | Agent | Target | Files | Real Holes | Expected |
|---|-------|--------|-------|-----------|----------|
| 1 | 1 | Chap43 OrderedTableMtEph ordering + Chap41 StEph assumes | MtEph.rs, StEph.rs | 8 | -5 to -8 |
| 2 | 2 | Chap43 OrderedTable St remaining ops | StEph.rs, StPer.rs | 9 | -4 to -7 |
| 3 | 3 | Chap43 OrderedSet St + AugOrderedTableMtEph | StEph.rs, StPer.rs, AugMtEph.rs | 10 | -4 to -8 |
| 4 | 4 | Chap47 hash tables + Chap57 Dijkstra | 5 Chap47 files, Dijkstra | 11 | -5 to -9 |

**Expected total: -18 to -32 holes (75 → 43-57 remaining)**

## File Assignments (no overlap)

### Agent 1
- src/Chap43/OrderedTableMtEph.rs (6 ext_body: ordering ops)
- src/Chap41/AVLTreeSetStEph.rs (2 assume: vec length bounds)

### Agent 2
- src/Chap43/OrderedTableStEph.rs (5 remaining: collect, filter, split_key, rank_key, select_key)
- src/Chap43/OrderedTableStPer.rs (4 remaining: collect, split_key, rank_key, select_key)

### Agent 3
- src/Chap43/OrderedSetStEph.rs (4 ext_body: split, rank, select, Iterator::next)
- src/Chap43/OrderedSetStPer.rs (4 ext_body: split, rank, select, Iterator::next)
- src/Chap43/AugOrderedTableMtEph.rs (2 ext_body: calculate_reduction, reduce_range_parallel)

### Agent 4
- src/Chap47/DoubleHashFlatHashTableStEph.rs (3: insert, lookup assume, delete)
- src/Chap47/LinProbFlatHashTableStEph.rs (2: insert, delete)
- src/Chap47/QuadProbFlatHashTableStEph.rs (2: insert, delete)
- src/Chap47/StructChainedHashTable.rs (1: resize)
- src/Chap47/ParaHashTableStEph.rs (2: call_hash_fn, compute_second_hash)
- src/Chap57/DijkstraStEphU64.rs (2 assume: heap property, budget)

## Deferred (Future Rounds)

- Chap41 AVLTreeSetMtPer (8 holes): parallel fork-join algorithms
- Chap41 AVLTreeSetMtEph (6 holes): parallel fork-join algorithms
- Chap38 BSTParaMtEph (9 holes): parallel recursive outside verus!
- Chap39 BSTParaTreapMtEph (10 holes): parallel recursive outside verus!
- Chap43 OrderedSetMtEph (1 real: filter ext_body)
