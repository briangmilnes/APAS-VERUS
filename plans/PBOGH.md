# PROVE BIG OR GO HOME — Proof Hole Reduction Plan

Baseline: 482 holes (72 assume, 394 external_body, 4 external, 12 trivial wf).
3658 verified, 0 errors. 25 clean chapters, 12 holed.

## Progress

Round 1: Converted 85 lock-boundary assumes and 37 eq/clone assumes to `accept()`.
New standards: `partial_eq_eq_clone_standard.rs`, updated `toplevel_coarse_rwlocks_for_mt_modules.rs`.

Round 2 (4-agent merge): Agents proved holes across their assigned chapters.
Newly clean: Chap03, Chap06, Chap21, Chap23, Chap28, Chap56.
Verified: 3632 → 3658 (+26). Holes: 552 → 482 (-70). Clean chapters: 20 → 25 (+5).

## Strategy

**Section 4.2 drives everything.** Only files whose dependencies are already clean can
be worked on. Fixing a file may unblock downstream files.

Three work categories, in priority order:

1. **Accept what must be accepted** — Verus limitations (lock boundary, eq/clone, external
   container ops). Convert assume→accept. Zero proof effort, high hole reduction.
2. **Write real specs** — trivial `spec_wf { true }` predicates, missing requires/ensures.
   Moderate effort, unblocks downstream consumers.
3. **Prove** — algorithmic assumes (loop invariants, termination, correctness). Hard work,
   highest value.

## Remaining Holed Chapters (by holes)

| # | Chap | Holes | Files | Status |
|---|------|-------|-------|--------|
| 1 | 43 | 127 | 11 | Blocked (internal deps) |
| 2 | 37 | 120 | 19 | Blocked (internal deps) |
| 3 | 41 | 85 | 7 | Blocked (internal deps) |
| 4 | 50 | 57 | 8 | Clean deps — actionable |
| 5 | 39 | 43 | 4 | Blocked (internal deps) |
| 6 | 47 | 40 | 9 | Blocked (internal deps) |
| 7 | 38 | 33 | 2 | Blocked (internal deps) |
| 8 | 53 | 22 | 5 | Blocked (internal deps) |
| 9 | 45 | 18 | 7 | Blocked (internal deps) |
| 10 | 42 | 17 | 4 | Blocked (internal deps) |
| 11 | 49 | 8 | 8 | Blocked (internal deps) |
| 12 | 51 | 8 | 8 | Blocked (internal deps) |

Near-clean (1-7 holes, clean deps — actionable now):

| # | Chap | Holes | Files | Status |
|---|------|-------|-------|--------|
| 1 | 18 | 7 | 7 | Clean deps — actionable |
| 2 | 40 | 6 | 3 | Clean deps — actionable |
| 3 | 55 | 4 | 8 | Clean deps — actionable |
| 4 | 26 | 4 | 8 | Clean deps — actionable |
| 5 | 19 | 3 | 4 | Clean deps — actionable |
| 6 | 66 | 3 | 2 | Clean deps — actionable |
| 7 | 52 | 1 | 14 | Clean deps — actionable |
| 8 | 12 | 1 | 3 | Clean deps — actionable |
| 9 | 57 | 1 | 3 | Clean deps — actionable |

## Section 4.2 Next Target Files (clean deps only, by holes)

| # | File | Holes |
|---|------|-------|
| 1 | Chap50/MatrixChainMtEph.rs | 15 |
| 2 | Chap50/OptBinSearchTreeMtEph.rs | 15 |
| 3 | Chap50/OptBinSearchTreeMtPer.rs | 12 |
| 4 | Chap50/MatrixChainMtPer.rs | 8 |
| 5 | Chap50/OptBinSearchTreeStEph.rs | 4 |
| 6 | Chap50/OptBinSearchTreeStPer.rs | 3 |
| 7 | Chap66/BoruvkaStEph.rs | 3 |
| 8 | Chap40/BSTSizeStEph.rs | 2 |
| 9 | Chap26/ETSPMtEph.rs | 2 |
| 10 | Chap26/ETSPStEph.rs | 2 |
| 11 | Chap47/ParaHashTableStEph.rs | 2 |
| 12 | Chap57/DijkstraStEphI64.rs | 1 |
| 13 | Chap18/LinkedListStPer.rs | 1 |
| 14 | Chap19/ArraySeqMtEph.rs | 1 |
| 15 | Chap18/LinkedListStEph.rs | 1 |
| 16 | Chap18/ArraySeq.rs | 1 |
| 17 | Chap19/ArraySeqStEph.rs | 1 |
| 18 | Chap18/ArraySeqStPer.rs | 1 |
| 19 | Chap12/Exercise12_5.rs | 1 |
| 20 | Chap18/ArraySeqStEph.rs | 1 |
| 21 | Chap18/ArraySeqMtEph.rs | 1 |
| 22 | Chap19/ArraySeqStPer.rs | 1 |
| 23 | Chap18/ArraySeqMtPer.rs | 1 |

## 4-Agent Work Split

Split by dependency chains. Agents work bottom-up within their assignment.
Focus on near-clean chapters first (quick wins to get more chapters to 0).

### Agent 1: Foundation Sequences (Chap18/19) + Near-Clean (Chap12/52/55/57)

Chap18 (7 holes) and Chap19 (3 holes) are nearly clean — each file has 1 hole.
Chap12 (1), Chap52 (1), Chap55 (4), Chap57 (1) are also near-clean with clean deps.
Total: 17 holes across 39 files. Getting these to 0 adds 6 more clean chapters.

Priority: Chap18 → Chap19 → Chap12 → Chap57 → Chap52 → Chap55.

### Agent 2: BST + Collections (Chap37/39/40/41/42)

Continue coarse RwLock migration for remaining Mt files. Prove BSTSizeStEph (2 holes).
Chap40 (6 holes) has clean deps — start there. Chap37/39/41/42 have internal deps.
Total: 271 holes but most are external_body that become accepts via migration.

Priority: Chap40 → Chap37 → Chap42 → Chap39 → Chap41.

### Agent 3: DP + Graph Algorithms (Chap26/50/53/66)

Chap50 (57 holes) is the big target — all in section 4.2 with clean deps.
Chap26 (4 holes) and Chap66 (3 holes) also have clean deps.
Chap53 (22 holes) has internal deps.
Total: 86 holes across 25 files.

Priority: Chap26 → Chap66 → Chap50 (St first, then Mt) → Chap53.

### Agent 4: Tables + Priority Queues (Chap38/43/45/47/49/51)

Chap45 (18 holes) — BinaryHeapPQ has trivial spec_wf. Write real invariants.
Chap47 (40 holes), Chap43 (127 holes) — large Mt chapters needing coarse RwLock migration.
Chap38 (33 holes) — per-node locking, genuinely hard.
Chap49 (8 holes), Chap51 (8 holes) — internal deps.
Total: 108 holes across 39 files.

Priority: Chap45 → Chap47 → Chap42 → Chap43 → Chap49 → Chap51 → Chap38.

## Scoreboard

| Metric | Round 0 | Round 1 | Round 2 | Target |
|--------|---------|---------|---------|--------|
| Total holes | 637 | 552 | 482 | <200 |
| assume() | 186 | 101 | 72 | <30 |
| external_body | 430 | 430 | 394 | <300 |
| trivial wf | 17 | 17 | 12 | 0 |
| Clean chapters | 18 | 20 | 25 | 30+ |
| Verified fns | 3632 | 3632 | 3658 | 3700+ |
