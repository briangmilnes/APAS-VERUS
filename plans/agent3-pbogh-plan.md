# Agent 3 PBOGH Work Plan — DP + Graph Algorithms

Chapters: Chap65, Chap66, Chap50, Chap53.
Baseline: 86 holes + 11 fn_missing_requires = 97 issues across 18 files.

## Results

| # | Phase | Holes Removed | Method | Notes |
|---|-------|---------------|--------|-------|
| 1 | Phase 1 | -5 | accept pattern | Clone/PartialEq standards |
| 2 | Phase 2 | -2 | remove assume | Verus auto-proved wf maintenance |
| 3 | Phase 3 | -8 | proof engineering | 4 new wf properties + rank-based termination |
| 4 | Phase 5b | -1 | verify body | SelectOne::select in GraphSearchStEph |
| | **Total** | **-16** | | **86 → 71 holes (was -15, then -1 more)** |

### Key Accomplishments

**Chap65 UnionFindStEph: 10 → 0 holes (fully clean).**

The flagship proof engineering achievement. All 10 assumes removed:
- 2 wf assumes in insert/num_sets: Verus auto-proved.
- 2 wf + domain assumes in compression pass: Verus auto-proved.
- 2 termination assumes: Replaced step counters with rank-based decreasing measures.
- 1 root correctness assume: Proved via new loop invariant `self.roots[current@] == old(self).roots[v@]`.
- 2 union wf + ensures assumes: Proved via Map::new construction.
- 1 PartialEq assume: Converted to accept (standard pattern).

Four new well-formedness properties added to `spec_unionfindsteph_wf`:
1. Self-parenting nodes are roots.
2. Following a parent pointer preserves the root component.
3. Non-root nodes have strictly smaller rank than their parent.
4. Every element's rank is at most its root's rank.

**Chap50 OBST clone: 4 holes removed via accept pattern.**

- KeyProb::clone: `accept(cloned == *self)` (structural, no View).
- OBSTStEphS/StPerS::clone: `accept(cloned@ == self@)` (view-based).

**Chap53 GraphSearchStEph SelectOne::select: 1 hole removed.**

Removed external_body, added seq-length proof-by-contradiction, clone-view accept,
and explicit subset_of proof. StPer/MtPer variants blocked by data structure spec gaps
(AVLTreeSetStPer requires wf for size(), AVLTreeSeqStPer requires wf for nth(), but
to_seq() doesn't ensure wf on the returned seq).

## Current State

| # | Chap | Files | Holes | Change | Notes |
|---|------|-------|-------|--------|-------|
| 1 | 65 | 3 (0 holed) | 0 | -10 | **Fully clean** |
| 2 | 66 | 2 (1 holed) | 3 | 0 | Blocked: RNG |
| 3 | 50 | 8 (6 holed) | 57 | -4 | Clone accepts; rest blocked |
| 4 | 53 | 5 (5 holed) | 11 | -1 | SelectOne verified; rest blocked |

Total: 71 holes (was 86). 11 fn_missing_requires unchanged (total functions, no real requires).

## Remaining Work — Blocked Analysis

### Chap66 (3 holes): RNG-dependent
BoruvkaStEph uses `rand::StdRng`. Cannot verify through randomization.

### Chap50 (57 holes): Verus limitations + concurrency
- St files (7 holes): Iterator chains (.map().fold()), recursive DP with closures,
  field mutation through Vec indexing. All fundamental Verus limitations.
- Mt files (50 holes): Arc<RwLock> pattern for concurrent memo tables. Trait methods
  lack requires/ensures. Would need full refactor to coarse locking standard.

### Chap53 (11 holes): Recursion proofs + spec gaps
- graph_search_explore (3 files): Recursive, ensures `frontier ⊆ visited_all` requires
  reachability argument — not a simple loop invariant.
- SelectOne::select (StPer, MtPer): Blocked by data structure wf spec gaps.
- PQMin (2 files): All 6 functions external_body, recursive, no postconditions on helpers.
- fn_missing_requires (11): Total functions taking closures. No real precondition needed.

## Validation

- Verus: 3637 verified, 0 errors
- RTT: 2600 tests passed
- PTT: 147 tests passed
