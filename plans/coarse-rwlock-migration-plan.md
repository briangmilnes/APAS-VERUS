# Coarse RwLock Migration Plan for Mt Collection Modules

Standard: `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`

Pattern: Wrap a verified St struct in plain RwLock (no Arc). Layer 1 = verified inner
struct. Layer 2 = locked wrapper with ghost shadow, type_invariant, closed accessor,
Result-returning trait methods. Three assume categories at lock boundary only.

## Scope

Mt **collection** files only. Excludes:
- Pure fork-join algorithms (no shared mutable state, no RwLock needed).
- Memoization algorithms (Chap49/50/51) that genuinely share a memo table across
  threads via Arc<RwLock> — those follow the hfscheduler Arc<RwLock> standard instead.
- BSTParaMtEph / BSTParaTreapMtEph — concurrent BSTs with fine-grained per-node
  locking. These genuinely share mutable nodes across threads.
- HFSchedulerMtEph — utility, correctly uses Arc<RwLock>.

## Phase 1: Have RwLock, Need Ghost Shadow + type_invariant

Already use plain RwLock but lack the full standard (ghost shadow, type_invariant,
closed accessor, Result returns).

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 1 | 37 | BSTAVLMtEph.rs | BSTAVLStEph.rs | Plain RwLock, closest to standard |
| 2 | 37 | BSTBBAlphaMtEph.rs | BSTBBAlphaStEph.rs | Plain RwLock |
| 3 | 37 | BSTPlainMtEph.rs | BSTPlainStEph.rs | Plain RwLock |

## Phase 2: Have Arc<RwLock>, Should Be Plain RwLock

Use Arc<RwLock> but per the standard, APAS Mt collections don't share mutable state
across threads. Refactor to plain RwLock + add ghost shadow + type_invariant.

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 4 | 37 | BSTRBMtEph.rs | BSTRBStEph.rs | Arc<RwLock>, needs downgrade |
| 5 | 37 | BSTSplayMtEph.rs | BSTSplayStEph.rs | Arc<RwLock>, needs downgrade |
| 6 | 39 | BSTTreapMtEph.rs | BSTTreapStEph.rs | Arc<RwLock>, has external_body |
| 7 | 41 | AVLTreeSetMtEph.rs | AVLTreeSetStEph.rs | Arc<RwLock> |

## Phase 3: Pure external_body, No RwLock — Core Collections

All methods are external_body. Need Layer 1 verification of St counterpart confirmed,
then wrap in RwLock + ghost shadow + type_invariant.

### Sets (Chap05)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 8 | 05 | SetMtEph.rs | SetStEph.rs | Fundamental collection |

### Graphs (Chap06)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 9 | 06 | DirGraphMtEph.rs | DirGraphStEph.rs | |
| 10 | 06 | UnDirGraphMtEph.rs | UnDirGraphStEph.rs | |
| 11 | 06 | LabDirGraphMtEph.rs | LabDirGraphStEph.rs | |
| 12 | 06 | LabUnDirGraphMtEph.rs | LabUnDirGraphStEph.rs | |

### Sequences (Chap18/19)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 13 | 18 | ArraySeqMtEph.rs | ArraySeqStEph.rs | Complex ninject pattern |
| 14 | 18 | ArraySeqMtPer.rs | ArraySeqStPer.rs | |
| 15 | 19 | ArraySeqMtEph.rs | ArraySeqStEph.rs | Extended sequences |
| 16 | 19 | ArraySeqMtEphSlice.rs | ArraySeqStEphSlice.rs | Slice variant |

### BST Set Wrappers (Chap37)

These wrap BSTs in a Set interface. Each delegates to its BST counterpart.

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 17 | 37 | BSTSetAVLMtEph.rs | BSTSetAVLStEph.rs | Wraps BSTAVLMtEph |
| 18 | 37 | BSTSetBBAlphaMtEph.rs | BSTSetBBAlphaStEph.rs | Wraps BSTBBAlphaMtEph |
| 19 | 37 | BSTSetPlainMtEph.rs | BSTSetPlainStEph.rs | Wraps BSTPlainMtEph |
| 20 | 37 | BSTSetSplayMtEph.rs | BSTSetSplayStEph.rs | Wraps BSTSplayMtEph |
| 21 | 37 | BSTSetRBMtEph.rs | BSTSetRBStEph.rs | Wraps BSTRBMtEph |
| 22 | 37 | AVLTreeSeqMtPer.rs | AVLTreeSeqStPer.rs | Sequence via AVL tree |

### BST Set/Treap Wrappers (Chap39)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 23 | 39 | BSTSetTreapMtEph.rs | BSTSetTreapStEph.rs | Wraps BSTTreapMtEph |

### Sets (Chap41)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 24 | 41 | AVLTreeSetMtPer.rs | AVLTreeSetStPer.rs | |
| 25 | 41 | ArraySetEnumMtEph.rs | ArraySetEnumStEph.rs | |

### Tables (Chap42/43)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 26 | 42 | TableMtEph.rs | TableStEph.rs | Hash table |
| 27 | 43 | OrderedTableMtEph.rs | OrderedTableStEph.rs | Many external_body fns |
| 28 | 43 | OrderedTableMtPer.rs | OrderedTableStPer.rs | Many external_body fns |
| 29 | 43 | OrderedSetMtEph.rs | OrderedSetStEph.rs | |
| 30 | 43 | AugOrderedTableMtEph.rs | AugOrderedTableStEph.rs | Augmented table |

### Graph Representations (Chap52)

| # | Chap | File | St Counterpart | Notes |
|---|------|------|----------------|-------|
| 31 | 52 | AdjSeqGraphMtEph.rs | AdjSeqGraphStEph.rs | |
| 32 | 52 | AdjSeqGraphMtPer.rs | AdjSeqGraphStPer.rs | |
| 33 | 52 | AdjMatrixGraphMtEph.rs | AdjMatrixGraphStEph.rs | |
| 34 | 52 | AdjMatrixGraphMtPer.rs | AdjMatrixGraphStPer.rs | |
| 35 | 52 | AdjTableGraphMtPer.rs | AdjTableGraphStPer.rs | |
| 36 | 52 | EdgeSetGraphMtPer.rs | EdgeSetGraphStPer.rs | |

## 3-Agent Work Split

Split by chapter so agents never touch the same files. No merge conflicts possible.

### Agent 2: Chap37 (11 files)

All BST core files + BST Set wrappers + AVLTreeSeq. Highest complexity — Phase 1
and Phase 2 files require understanding existing RwLock/Arc<RwLock> patterns before
refactoring. BSTSet wrappers depend on the underlying BST being done first.

| # | Phase | File | Notes |
|---|-------|------|-------|
| 1 | 1 | BSTAVLMtEph.rs | Plain RwLock, add ghost+type_inv |
| 2 | 1 | BSTBBAlphaMtEph.rs | Plain RwLock, add ghost+type_inv |
| 3 | 1 | BSTPlainMtEph.rs | Plain RwLock, add ghost+type_inv |
| 4 | 2 | BSTRBMtEph.rs | Arc→plain RwLock + standard |
| 5 | 2 | BSTSplayMtEph.rs | Arc→plain RwLock + standard |
| 6 | 3 | BSTSetAVLMtEph.rs | After #1 done |
| 7 | 3 | BSTSetBBAlphaMtEph.rs | After #2 done |
| 8 | 3 | BSTSetPlainMtEph.rs | After #3 done |
| 9 | 3 | BSTSetSplayMtEph.rs | After #5 done |
| 10 | 3 | BSTSetRBMtEph.rs | After #4 done |
| 11 | 3 | AVLTreeSeqMtPer.rs | Standalone |

### Agent 3: Chap39/41/42/43 (12 files)

Trees, sets, and tables. BSTTreapMtEph and AVLTreeSetMtEph need Arc downgrade.
Chap43 tables have the most external_body methods (OrderedTableMtEph has 16+).

| # | Phase | File | Notes |
|---|-------|------|-------|
| 1 | 2 | BSTTreapMtEph.rs | Arc→plain RwLock + standard |
| 2 | 2 | AVLTreeSetMtEph.rs | Arc→plain RwLock + standard |
| 3 | 3 | BSTSetTreapMtEph.rs | After #1 done |
| 4 | 3 | AVLTreeSetMtPer.rs | Standalone |
| 5 | 3 | ArraySetEnumMtEph.rs | Standalone |
| 6 | 3 | TableMtEph.rs | Hash table |
| 7 | 3 | OrderedTableMtEph.rs | 16+ external_body fns |
| 8 | 3 | OrderedTableMtPer.rs | Many external_body fns |
| 9 | 3 | OrderedSetMtEph.rs | Standalone |
| 10 | 3 | AugOrderedTableMtEph.rs | Augmented table |

### Agent 4: Chap05/06/18/19/52 (13 files)

Foundation collections and graph representations. All Phase 3 (pure external_body).
High uniformity within each chapter group — once the first graph or sequence file
is done, the rest follow the same pattern.

| # | Phase | File | Notes |
|---|-------|------|-------|
| 1 | 3 | SetMtEph.rs | Chap05, fundamental |
| 2 | 3 | DirGraphMtEph.rs | Chap06, template for 3-5 |
| 3 | 3 | UnDirGraphMtEph.rs | Chap06, follows #2 |
| 4 | 3 | LabDirGraphMtEph.rs | Chap06, follows #2 |
| 5 | 3 | LabUnDirGraphMtEph.rs | Chap06, follows #2 |
| 6 | 3 | ArraySeqMtEph.rs | Chap18, complex ninject |
| 7 | 3 | ArraySeqMtPer.rs | Chap18 |
| 8 | 3 | ArraySeqMtEph.rs | Chap19, extended seqs |
| 9 | 3 | ArraySeqMtEphSlice.rs | Chap19, slice variant |
| 10 | 3 | AdjSeqGraphMtEph.rs | Chap52, template for 11-15 |
| 11 | 3 | AdjSeqGraphMtPer.rs | Chap52, follows #10 |
| 12 | 3 | AdjMatrixGraphMtEph.rs | Chap52 |
| 13 | 3 | AdjMatrixGraphMtPer.rs | Chap52 |
| 14 | 3 | AdjTableGraphMtPer.rs | Chap52 |
| 15 | 3 | EdgeSetGraphMtPer.rs | Chap52 |

### Merge Risk

**Zero conflict risk.** Each agent owns disjoint chapters. Mt files are standalone
(no cross-imports between Mt files per CLAUDE.md). No shared files (lib.rs, Types)
need modification — these files already exist and only their internal structure
changes.

Changes are **pervasive within each file** (struct, View, trait, impl all change),
but since no two agents touch the same file, merges are trivially clean.

### Load Balance

| Agent | Files | Phase 1 | Phase 2 | Phase 3 | Complexity |
|-------|-------|---------|---------|---------|------------|
| 2 | 11 | 3 | 2 | 6 | High (refactor existing RwLock) |
| 3 | 12 | 0 | 2 | 8 | High (Chap43 tables are large) |
| 4 | 15 | 0 | 0 | 15 | Medium (uniform patterns, batch) |

## Summary

| Phase | Files | Description |
|-------|-------|-------------|
| 1 | 3 | Add ghost shadow + type_invariant to existing plain RwLock |
| 2 | 4 | Downgrade Arc<RwLock> to plain RwLock + add standard |
| 3 | 29 | Add RwLock to pure external_body collections |
| **Total** | **36** | |

## Excluded (Not Collections)

Algorithm Mt files that use fork-join without shared mutable collections (no RwLock
needed): Chap11 Fibonacci, Chap26 DivConReduce/MergeSort/Scan/ETSP, Chap27
Reduce/ScanContract, Chap28 MaxContigSubSum, Chap35 OrderStatSelect, Chap36 QuickSort,
Chap53 GraphSearch, Chap54 BFS, Chap59 Johnson, Chap61 EdgeContraction/VertexMatching,
Chap62 StarPartition/StarContraction, Chap63 Connectivity, Chap64 SpanTree, Chap66
Boruvka.

Memoization Mt files that genuinely need Arc<RwLock> (concurrent writers to shared
memo): Chap49 SubsetSum/MinEditDist, Chap50 MatrixChain/OptBinSearchTree, Chap51
TopDownDP/BottomUpDP.

Concurrent BSTs with fine-grained per-node locking: Chap38 BSTParaMtEph, Chap39
BSTParaTreapMtEph.
