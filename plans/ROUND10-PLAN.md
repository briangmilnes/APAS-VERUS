# Round 10 Plan — 4 Agents, 311 holes → target ≤ 265

## Context

Round 9 merged: 3976 verified, 311 holes, 36 clean chapters, 10 holed.
Agent 3 reverted 77 unauthorized accepts (inflated hole count back to true baseline).
Real proofs: ~47 across all agents. Chap52 closed. 3 Chap53 files closed.

## Current State (post Round 9 merge)

| # | Chapter | Real Holes | Files | Deps | Notes |
|---|---------|-----------|-------|------|-------|
| 1 | Chap37 | 6 | 19 | internal | Critical bottleneck — unblocks 41, 45 |
| 2 | Chap38 | 36 | 2 | clean deps | BSTParaStEph, BSTParaMtEph |
| 3 | Chap39 | 26 | 4 | internal | BSTTreapMtEph(8), BSTParaTreapMtEph(15), BSTSetTreapMtEph(3) |
| 4 | Chap41 | 64 | 7 | blocked Ch37 | Largest actionable chapter |
| 5 | Chap42 | 18 | 4 | blocked Ch41 | Untouched |
| 6 | Chap43 | 143 | 11 | blocked Ch37,41,42 | Largest overall — deferred |
| 7 | Chap45 | 4 | 7 | blocked Ch37 | Near close |
| 8 | Chap47 | 11 | 9 | internal | All Verus limitations (closures, wrapping) |
| 9 | Chap50 | 0 | 8 | clean deps | Only fn_missing_ensures — trivially closable |
| 10 | Chap53 | 1 | 5 | blocked Ch41 | GraphSearchMtPer only |

## Dependency Chain

```
Chap37 → Chap41 → Chap42 → Chap43
Chap37 → Chap45
Chap41 → Chap53
```

## Agent Assignments

### Agent 1: Chap37 (finish) + Chap45 (close) + Chap50 (close)

**Mission**: Close the bottleneck. Chap37 has only 6 real holes. Close Chap50 (trivial).
Prove Chap45 (4 holes — no more "structural" excuses).

**Files**:
- Chap37: AVLTreeSeq.rs (1 ext_body), AVLTreeSeqStPer.rs (2 assume),
  AVLTreeSeqMtPer.rs (1 assume + 2 ext_body), BSTSplayStEph.rs (0 real holes, 1 trivial_wf)
  Plus fn_missing_requires cleanup across BSTMtEph files
- Chap45: BalancedTreePQ.rs (1 ext_body), BinaryHeapPQ.rs (1 assume),
  Example45_2.rs (1 external), HeapsortExample.rs (1 fn_missing_ensures)
- Chap50: OptBinSearchTreeMtEph.rs (fn_missing_ensures), OptBinSearchTreeMtPer.rs (fn_missing_ensures)

**Specific guidance**:
- Chap37 remaining: feq/clone bridges (AVLTreeSeq next(), StPer set_rec/build_balanced
  assumes, MtPer thread boundaries). Read Agent 1 Round 9 report for what blocks each.
- Chap45 BinaryHeapPQ: The `sorted` assume needs heap property invariant. This is a real
  proof — not "structurally blocked". The heap property IS the spec.
- Chap45 BalancedTreePQ: `contains` is an AVL tree search. The sorted invariant can be
  captured by strengthening spec_wf.
- Chap50: Just add ensures to obst_rec in MtEph/MtPer. Copy from StEph/StPer versions.

**Target**: Chap37 ≤ 3, Chap45 ≤ 2, Chap50 closed.

### Agent 2: Chap47 (finish) + Chap42 (start)

**Mission**: Continue hash table proofs. Start Chap42 (Table variants).

**Files**:
- Chap47: All 9 files. 11 remaining external_body (all Verus limitations):
  - ParaHashTableStEph.rs: 5 (compute_load_factor, call_hash_fn, probes)
  - Flat hash tables: 5 (probe functions using Fn closures)
  - ChainedHashTable.rs: 1 (insert_chained — no IndexMut)
- Chap42: TableStEph.rs, TableStPer.rs, TableMtEph.rs, Example42_1.rs (18 holes total)

**Specific guidance**:
- Chap47 remaining 11: All involve Fn closure calls or wrapping arithmetic that Verus
  can't verify. Try workarounds: inline the closure body, use match instead of closure call,
  use checked arithmetic instead of wrapping. If truly blocked, document precisely.
- Chap42: Depends on Chap41::ArraySetStEph (3 holes). Start proving what doesn't depend
  on ArraySetStEph. The Table types use hash tables internally — check if they share
  patterns with Chap47.

**Target**: Chap47 ≤ 8, Chap42 ≤ 12.

### Agent 3: Chap38 (36 holes) + Chap39 (26 holes)

**Mission**: Parallel BST and Treap proofs. Both have clean or internal deps — no excuses.

**Files**:
- Chap38: BSTParaStEph.rs, BSTParaMtEph.rs (36 holes across 2 files)
- Chap39: BSTTreapMtEph.rs (8), BSTParaTreapMtEph.rs (15), BSTSetTreapMtEph.rs (3),
  BSTTreapStEph.rs (0 — already clean)

**Specific guidance**:
- BSTParaStEph.rs: Agent 3 identified in Round 9 that "ghost contents removed" broke
  the link↔set bridge. If restoring `pub ghost contents: Set<T::V>` to the RwLock
  predicate is the fix, DO IT. Don't just catalog the problem — fix the root cause.
- BSTParaMtEph.rs: Apply same ghost contents fix if applicable.
- BSTSetTreapMtEph.rs: Only 3 remaining. Agent 3 identified: singleton/insert/delete
  need old(self)@ through &self interior mutability. Try external_body spec strengthening
  on ParamTreap::view to return something other than Set::empty().
- BSTParaTreapMtEph.rs: 15 holes. Read the StEph counterpart for patterns.
- Read `src/standards/using_closures_standard.rs` for closure requires propagation.

**Target**: Chap38 ≤ 26, Chap39 ≤ 18.

### Agent 4: Chap41 (64 holes) + Chap53 (close)

**Mission**: Chap41 is the critical path for unblocking Chap42, 43, 53. Close Chap53.

**Files**:
- Chap41: AVLTreeSetStEph.rs (16), AVLTreeSetStPer.rs (10), AVLTreeSetMtEph.rs (19),
  AVLTreeSetMtPer.rs (12), ArraySetStEph.rs (3), ArraySetEnumMtEph.rs (1),
  Example41_3.rs (3)
- Chap53: GraphSearchMtPer.rs (1 hole)

**Specific guidance**:
- ArraySetStEph.rs (3 holes): Fix these FIRST. They unblock Chap42.
- AVLTreeSetStEph/StPer: Agent 3 proved some in Round 9 (clone, insert already-present).
  Continue: feq cascade (add requires obeys_feq_full to trait fns, update all callers),
  sorted sequence invariant (add to spec_wf for size/find proofs).
- AVLTreeSetMtEph/MtPer: Agent 4 proved 6 in Round 9. Continue with wf-implies-inv
  pattern. The Arc<RwLock> opaque state blocks further for MtEph.
- Chap53 GraphSearchMtPer: 1 external_body. Blocked by MtPer's lack of wf spec.
  If AVLTreeSetMtPer gets wf spec this round, this becomes provable.

**CRITICAL**: NO accept(). NO assume-to-accept conversions. Leave assumes and explain blockers.

**Target**: Chap41 ≤ 50, Chap53 closed (0).

## File Partition (no overlaps)

| Agent | Chapters | Key Files |
|-------|----------|-----------|
| 1 | Chap37, Chap45, Chap50 | AVLTreeSeq*, BSTSplayStEph, BST*MtEph (fn_missing only), BalancedTreePQ, BinaryHeapPQ, OptBinSearchTree* |
| 2 | Chap47, Chap42 | All Chap47, TableStEph, TableStPer, TableMtEph, Example42_1 |
| 3 | Chap38, Chap39 | BSTParaStEph, BSTParaMtEph, BSTTreapMtEph, BSTParaTreapMtEph, BSTSetTreapMtEph |
| 4 | Chap41, Chap53 | AVLTreeSet*, ArraySet*, Example41_3, GraphSearchMtPer |

## Targets

| Agent | Holes | Target | Close? |
|-------|-------|--------|--------|
| 1 | 10 | ≤ 5 | Chap50 |
| 2 | 29 | ≤ 20 | — |
| 3 | 62 | ≤ 44 | — |
| 4 | 65 | ≤ 50 | Chap53 |

**Combined target**: 311 → ≤ 265 (-46). Close Chap50, Chap53. Near-close Chap37, Chap45.

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/ChapNN/ # track hole reduction
```

Push to `agentN/ready`. Write `plans/agentN-round10-report.md`.
