# PROVE BIG OR GO HOME — Proof Hole Reduction Plan

Baseline: 568 holes (228 assume, 324 external_body, 4 external, 12 trivial wf).
3737 verified, 0 errors. 22 clean chapters, 24 holed.

## Progress

| Round | Holes | assume | ext_body | triv wf | Clean Ch | Verified |
|-------|-------|--------|----------|---------|----------|----------|
| 0 | 637 | 186 | 430 | 17 | 18 | 3632 |
| 1 | 552 | 101 | 430 | 17 | 20 | 3632 |
| 2 | 482 | 72 | 394 | 12 | 25 | 3658 |
| 3 | 456 | 55 | 385 | 12 | 25 | 3670 |
| 4* | 568 | 228 | 324 | 12 | 22 | 3737 |
| Target | <200 | <30 | <300 | 0 | 30+ | 3800+ |

Round 1: Converted lock-boundary + eq/clone assumes to accept().
Round 2: Agents proved holes; Chap03/06/21/23/28/56 went clean.
Round 3: Agent3 proved Chap65 UnionFind (0 holes). Agent4 reduced Chap43/47.
  Agent3 removed 9 external_body in Chap50 (57→48). Chap44/65 now clean.
Round 4: Agents added spec_wf + fn_missing_spec across many chapters (+67 verified).
  BUT agents also mass-converted ~300 accept→assume WITHOUT approval, inflating
  assume count. Human reverted all unauthorized accepts. Hole count went UP because
  previously-accepted holes are now back as assumes. Real progress: +67 verified,
  -61 external_body, many new specs. Net hole movement is misleading.

## Strategy

**Section 4.2 drives everything.** Only files whose dependencies are already clean can
be worked on. Fixing a file may unblock downstream files.

**CRITICAL: Do not convert assume to accept without explicit user approval.**
Round 4 taught us this the hard way. The strategy categories below are updated:

1. **Leave assumes alone** — existing assumes stay as assumes. The user decides
   when to promote them. Agents do NOT touch assume/accept conversions.
2. **Write real specs** — trivial `spec_wf { true }` predicates, missing requires/ensures.
   Moderate effort, unblocks downstream consumers.
3. **Prove** — close assumes by writing real proofs (loop invariants, termination,
   correctness). Replace the assume with actual proof steps. Hard work, highest value.
4. **Close trivial_wf** — add `// accept hole` comment where `{ true }` is genuinely
   correct (Vec-backed types with no structural invariant).

## Remaining Holed Chapters (by holes)

| # | Chap | Holes | Files | Status |
|---|------|-------|-------|--------|
| 1 | 43 | 132 | 11 | Blocked (internal deps) |
| 2 | 41 | 98 | 7 | Blocked (internal deps) |
| 3 | 37 | 66 | 19 | Blocked (internal deps) |
| 4 | 06 | 51 | 20 | Clean deps — actionable |
| 5 | 47 | 39 | 9 | Blocked (internal deps) |
| 6 | 39 | 38 | 4 | Blocked (internal deps) |
| 7 | 38 | 33 | 2 | Blocked (internal deps) |
| 8 | 45 | 26 | 7 | Blocked (internal deps) |
| 9 | 53 | 23 | 5 | Blocked (internal deps) |
| 10 | 50 | 22 | 8 | Clean deps — actionable |
| 11 | 42 | 18 | 4 | Blocked (internal deps) |
| 12 | 05 | 9 | 5 | Clean deps — actionable |
| 13 | 51 | 8 | 8 | Blocked (internal deps) |
| 14 | 18 | 6 | 7 | Clean deps — actionable |
| 15 | 49 | 4 | 8 | Blocked (internal deps) |
| 16 | 26 | 4 | 8 | Clean deps — actionable |
| 17 | 66 | 3 | 2 | Clean deps — actionable |
| 18 | 19 | 3 | 4 | Clean deps — actionable |
| 19 | 55 | 2 | 8 | Clean deps — actionable |
| 20 | 52 | 1 | 14 | Clean deps — actionable |
| 21 | 65 | 1 | 3 | Clean deps — actionable |
| 22 | 12 | 1 | 3 | Clean deps — actionable |

## Section 4.2 Next Target Files (clean deps only, by holes)

| # | File | Holes |
|---|------|-------|
| 1 | Chap06/DirGraphMtEph.rs | 20 |
| 2 | Chap06/LabUnDirGraphMtEph.rs | 15 |
| 3 | Chap06/UnDirGraphMtEph.rs | 10 |
| 4 | Chap05/SetMtEph.rs | 9 |
| 5 | Chap50/MatrixChainMtEph.rs | 8 |
| 6 | Chap45/BinaryHeapPQ.rs | 8 |
| 7 | Chap50/OptBinSearchTreeMtEph.rs | 6 |
| 8 | Chap06/LabDirGraphMtEph.rs | 6 |
| 9 | Chap66/BoruvkaStEph.rs | 3 |
| 10 | Chap50/MatrixChainMtPer.rs | 3 |
| 11 | Chap50/OptBinSearchTreeMtPer.rs | 3 |
| 12 | Chap26/ETSPMtEph.rs | 2 |
| 13 | Chap26/ETSPStEph.rs | 2 |
| 14 | Chap47/ParaHashTableStEph.rs | 1 |
| 15 | Chap18/LinkedListStPer.rs | 1 |
| 16 | Chap18/LinkedListStEph.rs | 1 |
| 17 | Chap18/ArraySeqMtEph.rs | 1 |
| 18 | Chap50/OptBinSearchTreeStPer.rs | 1 |
| 19 | Chap19/ArraySeqMtEph.rs | 1 |
| 20 | Chap19/ArraySeqStPer.rs | 1 |
| 21 | Chap12/Exercise12_5.rs | 1 |
| 22 | Chap65/UnionFindStEph.rs | 1 |
| 23 | Chap18/ArraySeqStPer.rs | 1 |
| 24 | Chap50/OptBinSearchTreeStEph.rs | 1 |
| 25 | Chap18/ArraySeqStEph.rs | 1 |
| 26 | Chap19/ArraySeqStEph.rs | 1 |
| 27 | Chap18/ArraySeqMtPer.rs | 1 |

## 4-Agent Work Split — Round 5

Split by chapter clusters. Focus on provable holes and trivial_wf cleanup.
**No assume→accept conversions. Period.**

### Agent 1: Graphs + Foundation Near-Clean (Chap05/06/18/19)
69 holes. Chap18/19 trivial_wf to 0. Chap06 lock-boundary proves. Chap05 assess.

### Agent 2: BST + Collections (Chap37/39/42)
122 holes. Chap42 first. Chap37 trivial_wf. Chap39 assess.

### Agent 3: Tables + Priority Queues (Chap43/45/47)
197 holes. Chap45 BinaryHeapPQ first. Chap47 ParaHashTable. Chap43 assess.

### Agent 4: DP + Remaining Near-Clean (Chap12/26/38/50/53/65/66)
87 holes. Near-clean chapters first (12/65/66/26). Chap50 readers. Chap53/38 assess.
