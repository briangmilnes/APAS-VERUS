# Agent 4 Work Plan — Tables + Priority Queues (Round 3)

## Current Baseline (2026-03-11)

| # | Chap | Holes | Breakdown | Notes |
|---|------|-------|-----------|-------|
| 1 | 45 | 17 | 13 ext_body, 4 external | BalancedTreePQ wraps BTreeSet (permanent) |
| 2 | 47 | 39 | 39 ext_body | Hash table algorithms |
| 3 | 43 | 127 | 3 assume, 124 ext_body | Largest chapter, Mt wrappers |
| 4 | 42 | 14 | 14 ext_body | TableMtEph needs coarse RwLock |
| 5 | 49 | 4 | 4 ext_body | Memoized DP (permanent, Arc+RwLock) |
| 6 | 51 | 8 | 8 ext_body | DP algorithms |
| 7 | 38 | 32 | 7 assume, 25 ext_body | Per-node locking (hardest) |
| **Total** | | **241** | | |

## Execution Order

### Wave 1: Quick Wins (assumes and missing specs)

1. Chap43/OrderedSetStEph.rs — 1 assume (clone/view) → accept or prove
2. Chap43/AugOrderedTableStPer.rs — 2 assumes (reducer.requires) → investigate
3. Chap47/ParaHashTableStEph.rs — fn_missing_requires → add requires

### Wave 2: Coarse RwLock Migration (biggest hole reduction)

4. Chap42/TableMtEph.rs — 13 ext_body → accepts (-13 holes)
5. Chap43/OrderedSetMtEph.rs — 23 ext_body → accepts (-23 holes)
6. Chap43/OrderedTableMtEph.rs — 17 ext_body → accepts (-17 holes)
7. Chap43/OrderedTableMtPer.rs — 21 ext_body → accepts (-21 holes)
8. Chap43/AugOrderedTableMtEph.rs — 5 ext_body → accepts (-5 holes)

### Wave 3: Spec Quality

9. Chap45/BinaryHeapPQ — real spec_wf (heap ordering + capacity bounds)
10. Chap51/TopDownDPMtEph/MtPer — real RwLockPredicate (not true)

### Wave 4: Hard Proofs (if time permits)

11. Chap38/BSTParaStEph.rs — 7 assumes on cmp ordering lemmas
12. Chap47 hash table algorithms — 38 ext_body on algorithmic code

### Permanent Holes (cannot fix)

- Chap45/BalancedTreePQ.rs — 15 holes (BTreeSet not verified by Verus)
- Chap49 Mt files — 4 ext_body (Arc+RwLock + recursion, by design)
- Chap38/BSTParaMtEph.rs — 19 ext_body (fine-grained concurrent BST)

## Expected Impact

Wave 1: -3 holes (assumes → accepts/proves)
Wave 2: -79 holes (ext_body → accepts via coarse RwLock)
Wave 3: 0 holes (quality improvement)
Wave 4: up to -45 holes (stretch goal)

Best case after waves 1-2: 241 → 159 holes.
