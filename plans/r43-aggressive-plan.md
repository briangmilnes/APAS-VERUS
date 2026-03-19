# R43 Plan: Aggressive Hole Filling (139 → target ~90)

## Baseline

- 4362 verified, 0 errors, 0 trigger warnings
- 2613 RTT, 143 PTT (4 pre-existing OrderedTableStPer iterator failures)
- 139 holes across 16 chapters
- 34 clean chapters

## Current Hole Inventory

| # | Chap | Holes | Key Files | Hole Types |
|---|------|-------|-----------|------------|
| 1 | 02 | 1 | HFSchedulerMtEph | assume(false) thread-join |
| 2 | 11 | 6 | 3 Fibonacci Mt files | assume(false) thread-join |
| 3 | 26 | 4 | ETSPMtEph | external_body (float dist) |
| 4 | 38 | 28 | BSTParaMtEph | 4 assume + 16 external_body + 8 RwLock |
| 5 | 39 | 27 | BSTParaTreapMtEph | 4 assume + 17 external_body + 6 misc |
| 6 | 41 | 6 | MtEph (0 real), MtPer (0 real), StEph (2) | RWLOCK_GHOST + 2 StEph assumes |
| 7 | 43 | 6 | AugOT MtEph (1), StPer (2), OSet misc (3) | Mixed |
| 8 | 45 | 1 | Example45_2 | Skip per CLAUDE.md |
| 9 | 47 | 5 | 3 flat hash (3 diverge), ParaHash (2) | assume(false) + ext_body |
| 10 | 59 | 5 | JohnsonMtEph (4), JohnsonStEph (1) | external_body + assume |
| 11 | 61 | 8 | EdgeContraction St/Mt (4), VertexMatching St/Mt (4) | external_body |
| 12 | 62 | 9 | StarContraction St/Mt (7), StarPartition St/Mt (2) | external_body |
| 13 | 63 | 12 | ConnectivityStEph (5), ConnectivityMtEph (7) | external_body |
| 14 | 64 | 8 | SpanTree St/Mt (3), TSPApprox (5) | external_body |
| 15 | 65 | 2 | KruskalStEph (1), PrimStEph (1) | external_body |
| 16 | 66 | 11 | BoruvkaMtEph | external_body |

## Structural Floor (can't close without Verus changes)

- 7 thread-join assume(false): Chap02 (1), Chap11 (6)
- 1 Example45_2 (skip per CLAUDE.md)
- ~40+ RWLOCK_GHOST assumes: Chap41 MtEph/MtPer, Chap43 OrderedSetMtEph
- 1 closure clone totality: Chap43 AugOrderedTableStPer
- ~10 eq/clone workaround assumes (not counted as holes by veracity)
- 4 float distance: Chap26 ETSPMtEph

**Structural floor: ~53 holes that won't close.**
**Achievable target: ~85-90 total (from 139) = close ~50.**

## R43 Agent Assignments (4 agents)

### Agent 1: Graph Cluster A — Chap61 + Chap62 (17 holes)

All external_body on graph algorithms. No proof-technique challenges — these are
implementations using SetStEph, UnDirGraphStEph, etc.

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 61 | EdgeContractionStEph.rs | 2 |
| 2 | 61 | EdgeContractionMtEph.rs | 2 |
| 3 | 61 | VertexMatchingStEph.rs | 2 |
| 4 | 61 | VertexMatchingMtEph.rs | 2 |
| 5 | 62 | StarContractionStEph.rs | 3 |
| 6 | 62 | StarContractionMtEph.rs | 4 |
| 7 | 62 | StarPartitionStEph.rs | 1 |
| 8 | 62 | StarPartitionMtEph.rs | 1 |

Strategy: These are all cfg-gated exec functions that we've already moved into
verus! with external_body. The functions use SetStEph, HashMap, random coin flips.
Prove by writing real loop bodies with ghost state tracking. For Mt variants,
keep the parallelism (ParaPair!/HFScheduler) and wrap only spawn boundaries.

### Agent 2: Graph Cluster B — Chap63 + Chap64 (20 holes)

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 63 | ConnectivityStEph.rs | 5 |
| 2 | 63 | ConnectivityMtEph.rs | 7 |
| 3 | 64 | SpanTreeStEph.rs | 2 |
| 4 | 64 | SpanTreeMtEph.rs | 1 |
| 5 | 64 | TSPApproxStEph.rs | 5 |

Strategy: Same approach as Agent 1. ConnectivityStEph/MtEph use iterative star
contraction. TSPApprox uses Euler tour + shortcutting. SpanTree delegates to
star contraction. All external_body, all implementations with HashSets and loops.

### Agent 3: Graph Cluster C + Chap47 — Chap65 + Chap66 + Chap47 (18 holes)

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 65 | KruskalStEph.rs | 1 |
| 2 | 65 | PrimStEph.rs | 1 |
| 3 | 66 | BoruvkaMtEph.rs | 11 |
| 4 | 47 | QuadProbFlatHashTable.rs | 1 |
| 5 | 47 | LinProbFlatHashTable.rs | 1 |
| 6 | 47 | DoubleHashFlatHashTable.rs | 1 |
| 7 | 47 | ParaHashTableStEph.rs | 2 |

Strategy: Kruskal uses UnionFind + sort. Prim uses priority queue (BinaryHeapPQ).
Boruvka uses star contraction phases with edge finding. The Chap47 flat hash
assume(false) holes need diverge() after the assume — simple fix. ParaHash has
clone_elem assume + external_body on compute_second_hash.

### Agent 4: Chap38 + Chap39 Parallel BST (55 holes)

The big one. 28 + 27 = 55 holes across parallel BST implementations.

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 38 | BSTParaMtEph.rs | 28 |
| 2 | 39 | BSTParaTreapMtEph.rs | 27 |

Strategy: These are RwLock-wrapped parallel BST operations. Many holes are
external_body on tree manipulation (insert, delete, join, split, find, union,
intersection, difference). The StEph counterparts (BSTParaStEph.rs,
BSTParaTreapStEph.rs) are clean — use those as proof templates.

Focus on the tree ops first (join, split, find) since they're foundations.
The set operations (union, intersection, difference) delegate to join/split.
RwLock ghost assumes at thread boundaries are structural — count but don't
try to close.

Realistically Agent 4 can close 10-20 of 55 in one round. Prioritize:
1. The 16 external_body in BSTParaMtEph that are pure delegations to StEph
2. The 4 algorithmic assumes in BSTParaMtEph (join/empty/singleton set specs)
3. BSTParaTreapMtEph external_body delegations

## Expected Results

| Agent | Target Holes | Est. Closed | Difficulty |
|-------|-------------|-------------|------------|
| 1 | 17 (Chap61+62) | 8-12 | Medium |
| 2 | 20 (Chap63+64) | 8-14 | Medium |
| 3 | 18 (Chap65+66+47) | 8-12 | Medium-Hard |
| 4 | 55 (Chap38+39) | 10-20 | Hard |

Total estimate: -34 to -58 holes → 81-105 remaining.

## R44+ Outlook

After R43, remaining holes will be:
- Structural floor (~53): thread-join, RWLOCK_GHOST, float, closure, example
- Chap38/39 overflow: whatever Agent 4 doesn't finish (~35-45)
- Chap43 remaining (6): AugOrderedTable, OrderedSet misc
- Chap59 JohnsonMtEph (4): parallel Johnson with ParaPair
- Scattered: Chap47 ParaHash, Chap41 StEph assumes

R44 would be a second pass on Chap38/39 + mop-up of any R43 overflow.
