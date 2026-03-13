# Round 6 Agent Work Plan — Hole Reduction

## Context

541 holes across 18 holed chapters (26 clean). 3764 verified, 0 errors.
Goal: reduce holes through real proof work, close near-clean chapters.

**Not assigned** (thread-boundary external_body, must stay):
Chap38 (25), Chap39 (38), Chap47 (39), Chap49 (4), Chap51 (6), Chap26 (4), Chap66 (3) = 119 holes skipped.

**Dependency chain:** Chap37 → Chap41 → Chap42 → Chap43. All have clean deps now.

## Agent 1: Graph + Set Mt Lock-Boundary (Chap06 + Chap05)

**60 holes, all assume. Target: -40.**

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | Chap06/DirGraphMtEph.rs | 20 | assume (view bridging) |
| 2 | Chap06/LabUnDirGraphMtEph.rs | 15 | assume (view bridging) |
| 3 | Chap06/UnDirGraphMtEph.rs | 10 | assume (view bridging) |
| 4 | Chap06/LabDirGraphMtEph.rs | 6 | assume (view bridging) |
| 5 | Chap05/SetMtEph.rs | 9 | assume (RwLock inv) |

All 60 holes follow the lock-boundary pattern: `assume(inner@ == self@)` after acquire_read, and `assume(result == self@.property)` from inner's ensures. Solve DirGraphMtEph first as template, replicate to the other 4 files.

**Clean chapter impact:** Chap06 could go clean (+1).

## Agent 2: BST MtEph + AVLTreeSeq (Chap37)

**62 holes: 31 assume, 30 ext_body, 1 trivial_wf. Target: -25.**

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | Chap37/BSTAVLMtEph.rs | 5 | assume (lock-boundary) |
| 2 | Chap37/BSTPlainMtEph.rs | 5 | assume |
| 3 | Chap37/BSTRBMtEph.rs | 5 | assume |
| 4 | Chap37/BSTBBAlphaMtEph.rs | 5 | assume |
| 5 | Chap37/BSTSplayMtEph.rs | 5 | assume |
| 6 | Chap37/BSTSplayStEph.rs | 2 | assume+trivial_wf |
| 7 | Chap37/AVLTreeSeq.rs | 3 | mixed |
| 8 | Chap37/AVLTreeSeqStEph.rs | 7 | 2 assume + 5 ext_body |
| 9 | Chap37/AVLTreeSeqStPer.rs | 13 | mixed |
| 10 | Chap37/AVLTreeSeqMtPer.rs | 12 | ext_body |

Priority: BST*MtEph lock-boundary (25 assumes, mechanical — solve BSTAVLMtEph, replicate to 4 siblings). Then AVLTreeSeq proves if time permits.

## Agent 3: Near-Clean + DP + Graph Search (Chap65, 52, 53, 50, 45)

**43 holes. Target: -15.**

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | Chap65/UnionFindStEph.rs | 1 | assume (eq bridge) |
| 2 | Chap52/EdgeSetGraphMtPer.rs | 1 | external_body |
| 3 | Chap53/GraphSearchStEph.rs | 7 | mixed |
| 4 | Chap53/PQMinStEph.rs | 5 | mixed |
| 5 | Chap53/GraphSearchMtPer.rs | 2 | assume |
| 6 | Chap50/MatrixChainMtEph.rs | 7 | 5 assume + 2 ext_body |
| 7 | Chap50/OptBinSearchTreeMtEph.rs | 4 | mixed |
| 8 | Chap50/MatrixChain+OptBST StPer/MtPer | 7 | mixed |
| 9 | Chap45/BinaryHeapPQ.rs | 9 | assume (multiset) |

Phase 1: Close Chap65 (1 hole) and Chap52 (1 hole) for +2 clean chapters.
Phase 2: Chap53 fn_missing_spec + algorithmic assumes.
Phase 3: Chap50 lock-boundary assumes.
Phase 4: Chap45 BinaryHeapPQ multiset proofs if time permits.

**Clean chapter impact:** Chap65 (+1), Chap52 (+1 possible).

## Agent 4: Ordered Sets + Tables (Chap43 + Chap42 + Chap41)

**161+84 holes total but focus on Mt wrappers. Target: -30.**

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | Chap43/OrderedSetMtEph.rs | 39 | assume (lock-boundary) |
| 2 | Chap43/OrderedTableMtEph.rs | 16 | ext_body (Mt delegation) |
| 3 | Chap43/OrderedTableMtPer.rs | 22 | ext_body |
| 4 | Chap42/TableMtEph.rs | 15 | 2 assume + 13 ext_body |
| 5 | Chap41/AVLTreeSetStEph.rs | 12 | assume (set semantics) |
| 6 | Chap41/AVLTreeSetStPer.rs | 10 | assume (set semantics) |
| 7 | Chap41/AVLTreeSetMtPer.rs | 5 | assume |
| 8 | Chap41/ArraySetStEph.rs | 9 | assume |

Priority: OrderedSetMtEph lock-boundary (mechanical, ~20 provable). Then Chap41 set-semantics assumes (insert/delete/union/difference) — these are real proof work. OrderedTable delegation last.

## Summary

| # | Agent | Chapters | Holes In | Target | Work Type |
|---|-------|----------|----------|--------|-----------|
| 1 | Agent 1 | 06, 05 | 60 | -40 | Lock-boundary assumes |
| 2 | Agent 2 | 37 | 62 | -25 | BST MtEph + AVLTreeSeq |
| 3 | Agent 3 | 65, 52, 53, 50, 45 | 43 | -15 | Near-clean + DP + PQ |
| 4 | Agent 4 | 43, 42, 41 | 128 | -30 | OrderedSet/Table Mt + set semantics |

**Projected:** -110 holes (541 → ~431). **+2 to +4 clean chapters** (28-30 total).

**No file overlap between agents. No merge dependencies. Merge in any order.**

## Verification

After each agent: `scripts/validate.sh` (0 errors), `scripts/rtt.sh`, `scripts/ptt.sh`.
After all merges: `scripts/all-holes-by-chap.sh` to regenerate baselines.
