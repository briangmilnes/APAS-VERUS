# PROVE BIG OR GO HOME — Proof Hole Reduction Plan

Baseline: 552 holes (101 assume, 430 external_body, 4 external, 17 trivial wf).
3632 verified, 0 errors. 18 clean chapters, 28 holed.

## What Just Happened

Converted 85 lock-boundary assumes and 37 eq/clone assumes to `accept()`.
New standards: `partial_eq_eq_clone_standard.rs`, updated `toplevel_coarse_rwlocks_for_mt_modules.rs`.
Chap05 is now 0 holes. Chap06 dropped from 53 to 2.

## Strategy

**Section 4.2 drives everything.** Only files whose dependencies are already clean can
be worked on. Fixing a file may unblock downstream files. The critical path runs bottom-up
through foundation chapters.

Three work categories, in priority order:

1. **Accept what must be accepted** — Verus limitations (lock boundary, eq/clone, external
   container ops). Convert assume→accept. Zero proof effort, high hole reduction.
2. **Write real specs** — trivial `spec_wf { true }` predicates, missing requires/ensures.
   Moderate effort, unblocks downstream consumers.
3. **Prove** — algorithmic assumes (loop invariants, termination, correctness). Hard work,
   highest value.

## Phase 1: Quick Wins (accept + trivial specs)

Low effort, high hole reduction. Unblocks Phase 2.

### 1a. Remaining lock-boundary + eq/clone assumes

Already done for Chap05/06/37. Remaining Mt files in the coarse RwLock migration plan
(Chap39/41/42/43/18/19/52) will gain these accepts as they get migrated. No separate
work item — the migration plan handles it.

### 1b. Trivial spec_wf { true } (17 holes)

These are placeholder wf predicates returning `true`. Write real invariants.

| # | File | What to fix |
|---|------|-------------|
| 1 | Chap45/BinaryHeapPQ.rs | Heap ordering, capacity |
| 2 | Others TBD from hole log | Scan for `trivial spec*wf` |

### 1c. Missing requires (from error list)

| # | File | What to fix |
|---|------|-------------|
| 1 | Chap03/InsertionSortStEph.rs | Add requires to insertion_sort |

## Phase 2: Foundation Files (clean deps, actionable now)

These files appear in section 4.2 — their dependencies are already clean.

### Tier A: Chap50 — Dynamic Programming (61 holes, 8 files)

Biggest single chapter. All external_body in Mt files wrapping memoization.

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | OptBinSearchTreeMtEph.rs | 15 | external_body | Coarse RwLock migration or accept |
| 2 | MatrixChainMtEph.rs | 15 | external_body | Coarse RwLock migration or accept |
| 3 | OptBinSearchTreeMtPer.rs | 12 | external_body | Coarse RwLock migration or accept |
| 4 | MatrixChainMtPer.rs | 8 | external_body | Coarse RwLock migration or accept |
| 5 | OptBinSearchTreeStEph.rs | 6 | external_body | Real proof work (St algorithms) |
| 6 | OptBinSearchTreeStPer.rs | 5 | external_body | Real proof work (St algorithms) |

Note: Chap50 Mt files use Arc<RwLock> for shared memo tables (genuine concurrent
writers). They follow the HFScheduler pattern, NOT the coarse RwLock pattern. The
external_body methods wrap thread-unsafe memo lookups. Strategy: verify the St
counterparts first, then assess whether the Mt wrappers can be tightened.

### Tier B: Chap65 — Union-Find (10 holes, 1 file)

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | UnionFindStEph.rs | 10 | assume | Real proof: loop invariants, termination, path compression |

This is genuine proof engineering. Assumes cover:
- `spec_unionfindsteph_wf()` maintenance through mutations
- `steps < elem_count` termination bounds
- `root == old(self).roots[v]` path compression correctness
High difficulty, high value — UnionFind is a foundation for Chap65/66 graph algorithms.

### Tier C: Small Files (1-3 holes each, clean deps)

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | Chap47/ParaHashTableStEph.rs | 3 | mixed | Audit assumes |
| 2 | Chap66/BoruvkaStEph.rs | 3 | external_body | Prove or accept |
| 3 | Chap45/BinaryHeapPQ.rs | 3 | trivial wf | Write real spec_wf |
| 4 | Chap40/BSTSizeStEph.rs | 2 | assume | Prove |
| 5 | Chap06/DirGraphStEph.rs | 2 | mixed | Prove |
| 6 | Chap26/ETSPStEph.rs | 2 | external_body | Prove or accept |
| 7 | Chap26/ETSPMtEph.rs | 2 | external_body | Accept (Mt wrapper) |
| 8 | Chap21/Exercise21_7.rs | 2 | mixed | Prove |
| 9 | Chap23/BalBinTreeStEph.rs | 2 | mixed | Prove |
| 10 | Chap18/ArraySeqMtEph.rs | 2 | mixed | Accept (Mt wrapper) |

### Tier D: Single-Hole Files (1 hole each, clean deps)

| # | File | Action |
|---|------|--------|
| 1 | Chap18/LinkedListStEph.rs | Prove or accept |
| 2 | Chap18/ArraySeqMtPer.rs | Accept (Mt wrapper) |
| 3 | Chap18/ArraySeqStEph.rs | Prove |
| 4 | Chap18/ArraySeqStPer.rs | Prove |
| 5 | Chap18/ArraySeq.rs | Prove |
| 6 | Chap18/LinkedListStPer.rs | Prove or accept |
| 7 | Chap19/ArraySeqStPer.rs | Prove |
| 8 | Chap19/ArraySeqStEph.rs | Prove |
| 9 | Chap19/ArraySeqMtEph.rs | Accept (Mt wrapper) |
| 10 | Chap21/Algorithm21_5.rs | Prove |
| 11 | Chap21/Exercise21_8.rs | Prove |
| 12 | Chap23/PrimTreeSeqStPer.rs | Prove or accept |
| 13 | Chap12/Exercise12_5.rs | Prove |
| 14 | Chap03/InsertionSortStEph.rs | Add missing requires |

## Phase 3: Blocked Chapters (need Phase 2 first)

These chapters have holes but depend on holed modules. They become actionable as
Phase 2 cleans their dependencies.

| # | Chap | Holes | Blocked by | Action when unblocked |
|---|------|-------|------------|----------------------|
| 1 | 43 | 144 | Chap42/43 internal | Coarse RwLock migration + prove St |
| 2 | 37 | 128 | Chap37 internal | Most already migrated, finish BSTSet wrappers |
| 3 | 41 | 97 | Chap41 internal | Coarse RwLock migration + prove St |
| 4 | 47 | 45 | Chap47 internal | Prove hash table algorithms |
| 5 | 39 | 43 | Chap39 internal | Coarse RwLock migration |
| 6 | 38 | 33 | Chap38 internal | Per-node locking, genuinely hard |
| 7 | 45 | 27 | Chap45 internal | Priority queue proofs |
| 8 | 53 | 23 | Chap53 internal | Graph search algorithms |
| 9 | 42 | 22 | Chap42 internal | Hash table Mt wrappers |
| 10 | 28 | 8 | Chap18/19 sequences | Prove after sequences clean |
| 11 | 49 | 8 | Chap49 internal | Memoization DP |
| 12 | 56 | 7 | Chap19 sequences | SSSP after sequences clean |
| 13 | 55 | 4 | Chap55 internal | Prove |

## 4-Agent Work Split

Split by dependency chains. Agents work bottom-up within their assignment.

### Agent 1: Foundation Sequences + Downstream (Chap18/19/28/56)

Fix the sequence holes first (Chap18: 8, Chap19: 3). This unblocks Chap28 (8)
and Chap56 (7). Total: 26 holes across 4 chapters.

### Agent 2: BST + Collections (Chap37/39/40/41/42)

Continue coarse RwLock migration for remaining Mt files. Prove BSTSizeStEph.
Total: 296 holes but most are external_body that become accepts via migration.

### Agent 3: DP + Graph Algorithms (Chap50/53/65/66)

UnionFindStEph (10 real proof holes) is the flagship. Chap50 needs St verification
before Mt can be tightened. Total: 97 holes.

### Agent 4: Small Chapters + Tables (Chap03/06/12/21/23/26/43/45/47)

Many small wins. BinaryHeapPQ trivial wf, InsertionSort missing requires, exercises.
Chap43/47 are large but mostly external_body Mt wrappers. Total: 133 holes.

## Scoreboard

| Metric | Before | After Accept | Target |
|--------|--------|--------------|--------|
| Total holes | 637 | 552 | <200 |
| assume() | 186 | 101 | <30 |
| external_body | 430 | 430 | <300 |
| trivial wf | 17 | 17 | 0 |
| Clean chapters | 18 | 20 | 30+ |
| Verified fns | 3632 | 3632 | 3700+ |
