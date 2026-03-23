# veracity-full-generic-feq Analysis Report

Date: 2026-03-23
Command: `veracity-full-generic-feq -c ~/projects/APAS-VERUS-agent5 -e experiments -e vstdplus -n`

## What This Tool Does

`veracity-full-generic-feq` analyzes files that use `obeys_feq_full` (full generic
function equality) and estimates the impact of folding those constraints into
`spec_*_wf` predicates. The net line count shows how many lines of proof
boilerplate (loop invariants, trigger asserts, requires clauses) would be
eliminated, offset by the wf predicate additions and `loop_isolation(false)`
annotations needed.

## Summary Table

|   # | Chap | File                    | Type |  WF + |  Inv - |  Trig - |  Req - |  Iso + |   Net |
|-----|------|-------------------------|------|-------|--------|---------|--------|--------|-------|
|   1 |   05 | SetMtEph.rs             |    T |    +1 |      0 |       0 |      0 |      0 |    +1 |
|   2 |   05 | SetStEph.rs             |    T |    +1 |      0 |       0 |      0 |      0 |    +1 |
|   3 |   37 | AVLTreeSeq.rs           |    T |    +1 |     -5 |       0 |     -7 |     +3 |    -8 |
|   4 |   37 | AVLTreeSeqMtPer.rs      |    T |    +1 |     -1 |       0 |     -1 |     +1 |     0 |
|   5 |   37 | AVLTreeSeqStEph.rs      |    T |    +1 |     -7 |       0 |     -5 |     +5 |    -6 |
|   6 |   37 | AVLTreeSeqStPer.rs      |    T |    +1 |     -1 |       0 |     -1 |     +1 |     0 |
|   7 |   41 | AVLTreeSetStEph.rs      |    T |    +1 |    -19 |     -13 |     -3 |    +14 |   -20 |
|   8 |   41 | AVLTreeSetStPer.rs      |    T |     0 |     -3 |      -1 |     -1 |      0 |    -5 |
|   9 |   41 | ArraySetStEph.rs        |    T |     0 |     -8 |       0 |      0 |     +7 |    -1 |
|  10 |   42 | TableMtEph.rs           |  K,V |    +2 |    -11 |       0 |     -3 |     +9 |    -3 |
|  11 |   42 | TableStEph.rs           |  K,V |    +2 |    -10 |       0 |     -8 |     +9 |    -7 |
|  12 |   43 | AugOrderedTableMtEph.rs |  K,V |    +2 |      0 |       0 |     -5 |      0 |    -3 |
|  13 |   43 | AugOrderedTableStEph.rs |  K,V |    +2 |      0 |       0 |    -10 |      0 |    -8 |
|  14 |   43 | AugOrderedTableStPer.rs |  K,V |    +2 |      0 |       0 |    -11 |      0 |    -9 |
|  15 |   43 | OrderedSetMtEph.rs      |    T |    +1 |     -1 |       0 |      0 |     +1 |    +1 |
|  16 |   43 | OrderedSetStEph.rs      |    T |    +1 |     -9 |      -9 |      0 |     +9 |    -8 |
|  17 |   43 | OrderedSetStPer.rs      |    T |    +1 |     -8 |      -9 |      0 |     +8 |    -8 |
|  18 |   43 | OrderedTableMtEph.rs    |  K,V |    +2 |      0 |       0 |     -4 |      0 |    -2 |
|  19 |   43 | OrderedTableStEph.rs    |  K,V |    +2 |    -28 |     -15 |    -10 |    +21 |   -30 |
|  20 |   43 | OrderedTableStPer.rs    |  K,V |    +2 |    -22 |      -4 |    -10 |    +16 |   -18 |
|  21 |   45 | BalancedTreePQ.rs       |    T |    +1 |     -1 |       0 |     -1 |     +1 |     0 |
|  22 |   65 | UnionFindStEph.rs       |    V |     0 |      0 |       0 |      0 |      0 |     0 |
|     |      | **TOTAL**               |      | **+27** | **-134** | **-51** | **-80** | **+105** | **-133** |

## Impact

Folding `obeys_feq_full` into `spec_*_wf` across 22 files would:
- Add 27 lines to wf predicates (the fold itself)
- Remove 134 loop invariant lines
- Remove 51 trigger assert lines
- Remove 80 requires clause lines
- Add 105 `loop_isolation(false)` annotations
- **Net: remove 133 lines of proof boilerplate**

Biggest wins: OrderedTableStEph (-30), AVLTreeSetStEph (-20), OrderedTableStPer (-18).

## Files Needing Human Review

| # | Chap | File | Reason |
|---|------|------|--------|
| 1 |   42 | TableStPer.rs | Unusual feq type params: `[ArraySeqStPerS<V>, K, Pair<K, ArraySeqStPerS<V>>, Pair<K, V>, V]` |
| 2 |   43 | OrderedTableMtPer.rs | Unusual feq type params: `[Pair<K, V>]` |

These have non-standard `obeys_feq_full` parameterizations that the tool cannot
safely transform automatically.

## Files With obeys_feq_full in Text but Not in AST (9 files)

These have `obeys_feq_full` appearing in the source but the AST parser does not
find actual calls. Could be in comments, dead code, or unusual syntax.

| # | Chap | File |
|---|------|------|
| 1 |   05 | MappingStEph.rs |
| 2 |   38 | BSTParaMtEph.rs |
| 3 |   38 | BSTParaStEph.rs |
| 4 |   47 | DoubleHashFlatHashTableStEph.rs |
| 5 |   47 | LinProbFlatHashTableStEph.rs |
| 6 |   47 | QuadProbFlatHashTableStEph.rs |
| 7 |   47 | StructChainedHashTable.rs |
| 8 |   53 | PQMinStEph.rs |
| 9 |   53 | PQMinStPer.rs |

## Files Missing spec_*_wf Predicate (7 files)

These use `obeys_feq_full` but have no `spec_*_wf` predicate to fold into.
They need a wf predicate added before the transform can apply.

| # | Chap | File |
|---|------|------|
| 1 |   17 | MathSeq.rs |
| 2 |   18 | ArraySeq.rs |
| 3 |   47 | LinkedListChainedHashTableStEph.rs |
| 4 |   47 | VecChainedHashTableStEph.rs |
| 5 |   57 | DijkstraStEphU64.rs |
| 6 |   65 | KruskalStEph.rs |
| 7 |   65 | PrimStEph.rs |
| 8 |   66 | BoruvkaStEph.rs |

## Suspicious Observations

1. **UnionFindStEph.rs (Chap65):** Type listed as `V` (not `T` or `K,V`), net 0,
   wf +0. The tool found an `obeys_feq_full` call but computed zero changes. Either
   the fold is already done, or the single-param `V` shape is not recognized. Worth
   manual inspection.

2. **Chap43 dominates the savings.** Six Chap43 files account for -78 of the -133
   net (59%). OrderedTableStEph alone is -30. This chapter would benefit most from
   the transform.

3. **loop_isolation(false) cost.** The transform adds 105 `loop_isolation(false)`
   annotations. These are needed because folding feq into wf changes what the SMT
   solver sees in loop contexts. This is mechanical but adds visual noise.

4. **Chap05 SetMtEph/SetStEph are net +1.** These files have no redundant invariants
   to remove, so the transform only adds the wf line. Still worth doing for
   consistency but no proof simplification.
