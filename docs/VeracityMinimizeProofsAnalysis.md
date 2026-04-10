# Veracity Minimize-Proofs Analysis: R170

Date: 2026-04-10

## Overview

R170 ran `veracity-minimize-proofs` across all APAS-VERUS chapters using 6 parallel
agents (agents 2-7) in isolated worktrees. The tool tests each assert and proof block
individually: comment it out, verify, and mark it UNNEEDED (removable) or NEEDED
(required). Thresholds were set to `--max-incremental 0.00 --max-memory-increase 0.00`
— strict improvement only, no regressions allowed.

Two waves were run:
- **Wave 1:** Agents 2-7 each took a batch of chapters (Chap03-55).
- **Wave 2:** Agents 2,3,4 took remaining chapters (Chap56-66) after agent 7's load
  was rebalanced.

## Cost: minimize run times

Wall-clock time per chapter (Wave 2, 6 agents on a contended 32 GB box):

| # | Agent | Chap | Asserts | Time | Notes |
|---|-------|------|---------|------|-------|
| 1 | 2 | 59 | 20 | 24m | |
| 2 | 2 | 62 | — | 18m | |
| 3 | 2 | 66 | — | 6m | |
| 4 | 3 | 57 | 2 | 9m | died on Chap58 |
| 5 | 4 | 61 | — | 26s | small chapter |
| 6 | 4 | 63 | — | 3m | |
| 7 | 4 | 64 | — | 1m | |
| 8 | 5 | 42 | 160+ | 8h 1m | largest prior-round chapter |
| 9 | 5 | 43 | 432 | running | at 64% after ~11h |
| 10 | 6 | 44 | — | 19m | |
| 11 | 6 | 45 | — | 2h 34m | |
| 12 | 6 | 47 | — | 1h 21m | |
| 13 | 6 | 49 | — | 5m | |
| 14 | 6 | 50 | — | 16m | |
| 15 | 6 | 51 | — | 12m | |
| 16 | 6 | 52 | 681 | running | at 72% after ~11h |
| 17 | 7 | 53 | — | 23m | |
| 18 | 7 | 54 | — | 36m | |
| 19 | 7 | 55 | 818 | 8h 59m | largest chapter, 818 asserts |
| 20 | 7 | 56 | — | 3m | |
| 21 | 7 | 57 | — | 5m | died on Chap58 |

Total agent-hours (Wave 2): ~55h across 6 agents, ~11h wall clock.
Wave 1 timing was lost with the logs, but covered 22 chapters with similar profiles.

Each iteration costs one `validate isolate ChapNN` (~30-60s per chapter). A chapter
with 400 asserts at 40s/iteration = ~4.5h. Lock contention from 6 concurrent agents
adds 20-50% overhead.

### Per-chapter detail: CPU, memory, LOC, NEEDED/UNNEEDED

Isolate-mode verification stats from Wave 2 logs. CPU is z3 children time, RSS is
peak z3 RSS. LOC is total lines (comments not counted). NEEDED/UNNEEDED are assert
and proof block counts after testing.

| # | Agent | Chap | CPU Before | CPU After | CPU Δ | RSS Before | RSS After | RSS Δ | LOC Before | LOC After | LOC Δ | Needed | Unneeded | % Removed |
|---|-------|------|-----------|-----------|-------|-----------|-----------|-------|-----------|-----------|-------|--------|----------|-----------|
| 1 | 2 | 59 | 432s | 470s | +9% | 984 MB | 970 MB | -1% | 1479 | 1499 | +20 | 20 | 8 | 29% |
| 2 | 2 | 62 | 337s | 754s | +124% | 516 MB | 1162 MB | +125% | 2245 | 2341 | +96 | 96 | 2 | 2% |
| 3 | 2 | 66 | 727s | 179s | **-75%** | 429 MB | 389 MB | -9% | 1303 | 1323 | +20 | 20 | 0 | 0% |
| 4 | 3 | 57 | 418s | 377s | -10% | 949 MB | 659 MB | **-31%** | 448 | 454 | +6 | 6 | 4 | 40% |
| 5 | 4 | 61 | 421s | 329s | -22% | 955 MB | 431 MB | **-55%** | 537 | 537 | 0 | 0 | 0 | — |
| 6 | 4 | 63 | 176s | 312s | +77% | 422 MB | 445 MB | +5% | 284 | 284 | 0 | 0 | 0 | — |
| 7 | 4 | 64 | 429s | 387s | -10% | 1170 MB | 440 MB | **-62%** | 565 | 565 | 0 | 0 | 0 | — |
| 8 | 5 | 42 | 249s | 344s | +38% | 694 MB | 654 MB | -6% | 6837 | 7666 | +829 | 836 | 160 | 16% |
| 9 | 6 | 44 | 249s | 152s | **-39%** | 714 MB | 599 MB | -16% | 430 | 454 | +24 | 25 | 11 | 31% |
| 10 | 6 | 45 | 144s | 149s | +3% | 500 MB | 584 MB | +17% | 4724 | 5073 | +349 | 360 | 16 | 4% |
| 11 | 6 | 47 | 165s | 234s | +42% | 677 MB | 714 MB | +5% | 4662 | 5014 | +352 | 360 | 0 | 0% |
| 12 | 6 | 49 | 131s | 162s | +24% | 496 MB | 589 MB | +19% | 1297 | 1306 | +9 | 18 | 0 | 0% |
| 13 | 6 | 50 | 46s | 28s | **-39%** | 351 MB | 0 MB | **-100%** | 2132 | 2194 | +62 | 68 | 15 | 18% |
| 14 | 6 | 51 | 148s | 104s | **-30%** | 454 MB | 496 MB | +9% | 2653 | 2708 | +55 | 59 | 1 | 2% |
| 15 | 7 | 53 | 259s | 136s | **-47%** | 624 MB | 600 MB | -4% | 1647 | 1714 | +67 | 67 | 3 | 4% |
| 16 | 7 | 54 | 136s | 136s | 0% | 650 MB | 609 MB | -6% | 2429 | 2545 | +116 | 118 | 14 | 11% |
| 17 | 7 | 55 | 208s | 203s | -2% | 704 MB | 406 MB | **-42%** | 4740 | 5419 | +679 | 681 | 271 | 28% |
| 18 | 7 | 56 | 212s | 81s | **-62%** | 431 MB | 253 MB | **-41%** | 2282 | 2296 | +14 | 14 | 0 | 0% |
| 19 | 7 | 57 | 429s | 288s | **-33%** | 726 MB | 648 MB | -11% | 458 | 467 | +9 | 9 | 1 | 10% |

**Wave 2 totals:** 2757 asserts/proof blocks tested, 506 removed (18%).
LOC increased by +2698 lines from Veracity comment markers on NEEDED items.

Note: LOC *increases* because NEEDED markers add comment lines. UNNEEDED items are
commented out (code stays as a comment, not deleted). The net LOC change is from
markers, not from adding new code.

### Chapter status summary

| Status | Count | Chapters |
|--------|-------|----------|
| Done and merged clean | 29 | 03,06,11,17,18,19,21,23,26,27,28,35,38,39,40,41,42,44,49,50,51,54,55,56,61,62,63,64,66 |
| Done, merged with ensures fixes | 2 | 50 (OBSTMtPer, MatrixChainStPer ensures restored) |
| Done, partially merged (files reverted) | 5 | 36 (full), 37 (BSTRBMtEph), 53 (PQMinStPer), 57 (Dijkstra F64/U64), 59 (Johnson 4 files) |
| Still running | 2 | 43 (agent5, 64%), 52 (agent6, 72%) |
| Failed (stale Chap19) | 1 | 58 |
| Never assigned | 3 | 30, 60, 65 |
| No asserts to test | 4 | 45 (merged markers only), 47 (merged markers only), 61, 63 |

Some chapters show *increases* in CPU or memory after minimize. This happens when
removing asserts that were Z3 speed hints — the 0.00 threshold prevents removing
asserts that increase CPU or memory for *that specific assert's test*, but
interactions between remaining asserts can shift Z3's search strategy for the
full chapter. The net effect across the full crate is still a 19% CPU reduction
and 25% RSS reduction.

## Codebase size (post-minimize)

As of commit `9950c339a` (2026-04-10):

```
    Spec    Proof     Exec     Rust   Total
  31,172   40,314   66,874   11,801  150,161 (code lines, comments excluded)
                                     200,482 (total lines including comments)
```

270 source files across 45 chapters.

### Per-chapter LOC

| Chap | Files | Spec | Proof | Exec | Rust |
|------|-------|------|-------|------|------|
| 02 | 2 | 40 | 35 | 96 | 76 |
| 03 | 1 | 13 | 24 | 49 | 7 |
| 05 | 5 | 713 | 356 | 1,221 | 200 |
| 06 | 21 | 2,052 | 1,823 | 4,940 | 756 |
| 11 | 5 | 39 | 53 | 323 | 37 |
| 12 | 3 | 13 | 0 | 122 | 46 |
| 17 | 1 | 153 | 61 | 236 | 82 |
| 18 | 9 | 2,007 | 1,808 | 4,762 | 418 |
| 19 | 5 | 1,150 | 900 | 2,403 | 214 |
| 21 | 12 | 159 | 426 | 595 | 50 |
| 23 | 2 | 466 | 150 | 573 | 152 |
| 26 | 8 | 518 | 884 | 1,313 | 129 |
| 27 | 5 | 117 | 265 | 438 | 17 |
| 28 | 11 | 395 | 733 | 863 | 66 |
| 30 | 1 | 6 | 0 | 36 | 25 |
| 35 | 4 | 196 | 834 | 653 | 12 |
| 36 | 3 | 217 | 832 | 1,139 | 9 |
| 37 | 20 | 3,403 | 4,302 | 7,069 | 973 |
| 38 | 3 | 617 | 1,061 | 1,119 | 140 |
| 39 | 5 | 1,436 | 1,011 | 2,109 | 294 |
| 40 | 3 | 1,094 | 853 | 1,695 | 223 |
| 41 | 8 | 1,526 | 968 | 1,752 | 379 |
| 42 | 5 | 686 | 416 | 2,563 | 265 |
| 43 | 12 | 2,674 | 1,553 | 3,442 | 836 |
| 44 | 2 | 115 | 110 | 229 | 225 |
| 45 | 7 | 887 | 1,912 | 2,274 | 562 |
| 47 | 9 | 539 | 2,257 | 2,218 | 220 |
| 49 | 8 | 360 | 18 | 928 | 549 |
| 50 | 8 | 616 | 99 | 1,479 | 612 |
| 51 | 9 | 759 | 291 | 1,658 | 257 |
| 52 | 15 | 1,407 | 2,271 | 3,184 | 351 |
| 53 | 5 | 505 | 234 | 977 | 190 |
| 54 | 5 | 560 | 605 | 1,380 | 123 |
| 55 | 9 | 897 | 2,309 | 2,213 | 179 |
| 56 | 12 | 653 | 0 | 1,643 | 193 |
| 57 | 3 | 82 | 53 | 323 | 75 |
| 58 | 2 | 68 | 0 | 372 | 57 |
| 59 | 4 | 263 | 91 | 1,155 | 62 |
| 61 | 4 | 62 | 0 | 475 | 87 |
| 62 | 4 | 349 | 570 | 1,422 | 95 |
| 63 | 2 | 56 | 0 | 228 | 50 |
| 64 | 3 | 83 | 0 | 482 | 76 |
| 65 | 3 | 764 | 868 | 651 | 98 |
| 66 | 2 | 171 | 2 | 1,150 | 117 |

### Infrastructure modules

| Module | Files | Spec | Proof | Exec | Rust |
|--------|-------|------|-------|------|------|
| vstdplus | 26 | 1,087 | 1,506 | 655 | 963 |
| standards | 29 | 1,113 | 42 | 956 | 693 |
| Types.rs | 1 | 72 | 20 | 16 | 72 |
| lib.rs | 1 | 0 | 0 | 0 | 466 |

### Composition

- **40,314 proof lines** — 27% of code. This is the verification effort.
- **31,172 spec lines** — 21% of code. Function contracts and ghost definitions.
- **66,874 exec lines** — 44% of code. The actual algorithms.
- **11,801 rust lines** — 8% of code. Outside verus! (Debug, Display, macros, tests).

### Veracity marker overhead

The minimize run added 10,529 `// Veracity:` comment lines to the codebase:
- 9,678 `// Veracity: NEEDED` — marks asserts/proof blocks that Z3 requires.
- 915 `// Veracity: UNNEEDED` — commented-out asserts/proof blocks (corpses left in place).

These 10,529 lines are ~5.3% of total lines. They are included in the 200,482 total
line count but excluded from the spec/proof/exec/rust breakdown (which counts only
code lines). The UNNEEDED lines are dead code preserved as comments per project policy
("leave the corpse"). The NEEDED markers are informational — they tell future agents
and humans which asserts are load-bearing.

## Results

### Verification improvement

| Metric | Pre-minimize | Post-minimize | Change |
|--------|-------------|---------------|--------|
| Verified | 5598 | 5598 | unchanged |
| z3 CPU | 392s | 318s | **-19%** |
| z3 RSS | 961 MB | 719 MB | **-25%** |
| Wall clock | 169s | 135s | **-20%** |
| RTT | 3776 | 3776 | unchanged |
| PTT | 221 | 221 | unchanged |

Pre-minimize commit: `a940468df`. Post-minimize commit: `c22c64b44`.

### Chapters merged (38 total)

**Wave 1 (prior round, source markers, logs lost in rebase):**

| # | Agent | Chap | Markers | Unneeded |
|---|-------|------|---------|----------|
| 1 | 2 | 03 | 3 | 0 |
| 2 | 2 | 06 | 739 | 21 |
| 3 | 2 | 11 | 38 | 3 |
| 4 | 2 | 17 | 32 | 1 |
| 5 | 2 | 21 | 135 | 0 |
| 6 | 2 | 23 | 72 | 0 |
| 7 | 2 | 26 | 224 | 28 |
| 8 | 2 | 27 | 105 | 4 |
| 9 | 2 | 28 | 138 | 0 |
| 10 | 2 | 35 | 304 | 0 |
| 11 | 3 | 37 | 876 | 24 |
| 12 | 3 | 38 | 397 | 16 |
| 13 | 3 | 39 | 783 | 37 |
| 14 | 3 | 40 | 197 | 0 |
| 15 | 3 | 41 | 762 | 0 |
| 16 | 5 | 18 | 648 | 18 |
| 17 | 5 | 19 | 222 | 24 |
| 18 | 7 | 43 | 100 | 0 |
| 19 | 7 | 45 | 366 | 0 |
| 20 | 7 | 47 | 346 | 0 |
| 21 | 7 | 51 | 60 | 0 |

**Wave 2 (current round, logs on disk):**

| # | Agent | Chap |
|---|-------|------|
| 22 | 2 | 59 |
| 23 | 2 | 62 |
| 24 | 2 | 66 |
| 25 | 3 | 57 |
| 26 | 4 | 61 |
| 27 | 4 | 63 |
| 28 | 4 | 64 |
| 29 | 5 | 42 |
| 30 | 6 | 44 |
| 31 | 6 | 45 |
| 32 | 6 | 47 |
| 33 | 6 | 49 |
| 34 | 6 | 50 |
| 35 | 6 | 51 |
| 36 | 7 | 53 |
| 37 | 7 | 54 |
| 38 | 7 | 55 |
| 39 | 7 | 56 |

### Completed but partially merged (files reverted due to veracity bugs)

| # | Chap | Reverted files | Bug symptom |
|---|------|----------------|-------------|
| 1 | 36 | full chapter | conjunction flakiness |
| 2 | 37 | BSTRBMtEph.rs | marker shift — RTT: test_rb_balancing |
| 3 | 53 | PQMinStPer.rs | marker shift — 6 RTT failures |
| 4 | 57 | DijkstraStEphF64.rs, DijkstraStEphU64.rs | marker shift — 12 RTT failures |
| 5 | 59 | JohnsonMt/StEphF64.rs, JohnsonMt/StEphI64.rs | marker shift — 12 RTT failures |

These chapters had other files successfully merged; only the listed files were reverted.
Exception: Chap36 was fully reverted.

### Still running at time of merge

| # | Agent | Chap | Progress |
|---|-------|------|----------|
| 1 | 5 | 43 | [275/432] — 64% |
| 2 | 6 | 52 | [491/681] — 72% |

### Not run

| # | Chap | Reason |
|---|------|--------|
| 1 | 30 | Agent 2 assigned but produced 0 markers — chapter may have no asserts |
| 2 | 58 | Agents 3 and 7 both failed on stale Chap19 dep |
| 3 | 60 | Never assigned |
| 4 | 65 | Never assigned (UnionFindStEph/KruskalStEph commented out in lib.rs) |

### Agent status at end of round

| # | Agent | Completed | Died on | Exit |
|---|-------|-----------|---------|------|
| 1 | 2 | 03,06,11,17,21,23,26,27,28,35,36,59,62,66 | — | clean |
| 2 | 3 | 37,38,39,40,41,57 | Chap58 (stale Chap19) | dead |
| 3 | 4 | 61,63,64 | — | clean |
| 4 | 5 | 18,19,42 | Chap43 still running | alive |
| 5 | 6 | 44,45,47,49,50,51 | Chap52 still running | alive |
| 6 | 7 | 43,45,47,51,53,54,55,56,57 | Chap58 (stale Chap19) | dead |

---

## Problems

### Root cause: veracity proof block detector is not AST-aware

All four problems stem from the same root cause: the proof block detector in
`veracity-minimize-proofs` operates on lines, not on the Rust/Verus AST. It
pattern-matches indented lines and treats them as proof blocks.

Full bug report: `plans/veracity-bug-minimize-proofs-not-ast-aware.md`

#### Symptom 1: Ensures clauses marked as UNNEEDED proof block

Ensures clause lines look like indented expressions. The detector comments them out.
The function body verifies without the ensures (weaker postcondition), but callers
break.

Files fixed during merge:

| # | File | Ensures removed | Detected by |
|---|------|-----------------|-------------|
| 1 | Chap18/ArraySeqMtPer.rs:1849 | `it@.1 == self.seq@` | PTT failure |
| 2 | Chap50/OptBinSearchTreeMtPer.rs:420 | `ensures equal == (self@ == other@)` | Manual audit |
| 3 | Chap50/MatrixChainStPer.rs:422 | `ensures equal == (self@ == other@)` | Manual audit |

Additional instances exist in agent worktrees (Chap38 BSTParaStEph/MtEph,
Chap19 ArraySeqMtEphSlice) that were not merged.

#### Symptom 2: NEEDED marker insertion displaces exec code

Inserting `// Veracity: NEEDED proof block` markers at proof block boundaries
can displace `let ghost` bindings and exec statements. Verus still verifies (ghost
is erased), but runtime behavior changes silently.

Files reverted: 8 files across 4 chapters, 30 RTT failures total.

#### Symptom 3: #[cfg(verus_keep_ghost)] treated as proof block

The cfg gate on `PartialEqSpecImpl` impls is structural. Commenting it out causes
cargo compile errors (`cannot find trait PartialEqSpecImpl`).

Files fixed: Chap18/ArraySeqStPer.rs, Chap37/AVLTreeSeq.rs.

#### Symptom 4: Removed asserts cause conjunction flakiness

Some UNNEEDED asserts are Z3 speed hints that stabilize later conjunction proofs.
The tool tests each assert in isolation and correctly finds the function verifies
without it. But removing the assert destabilizes a conjunction proof elsewhere in
the same function.

Chap36/QuickSortStEph.rs fully reverted.

### Stale Chap19 dependency killed two agents

Agents 3 and 7 both died when they reached Chap58, which depends on Chap19
(ArraySeqStEph). Their worktrees had a pre-minimize version of Chap19 where
`len()` and `nth()` postconditions fail against the current dependency graph.
The agents were rebased before the Chap19 minimize changes landed on main.

Fix: rebase agents before assigning chapters with cross-chapter dependencies.

### Wave 1 logs lost in rebase

The Wave 1 minimize-proofs logs were untracked files in agent worktrees. When agents
were rebased with `git reset --hard origin/main`, the logs were destroyed. Source
changes survived because they were recovered from `git reflog`.

Fix: commit logs immediately after each minimize run completes, before any rebase.
Wave 2 logs were snapshot-committed to prevent recurrence.

---

## Merge process

Each merge followed this sequence:

1. Copy `src/ChapNN/*.rs` from agent worktree to main
2. Check for known bugs:
   - `grep 'UNNEEDED.*cfg.verus_keep_ghost'` — cfg gate removal
   - `grep 'UNNEEDED proof block'` + check for ensures/requires patterns
   - Verify prior fixes not overwritten (ETSPMtEph rlimit, ArraySeqMtPer ensures)
3. `scripts/validate.sh` — must be 0 errors
4. `scripts/rtt.sh` — must be 0 failures
5. `scripts/ptt.sh` — must be 0 failures
6. If RTT/PTT fails, revert the specific corrupted files and re-validate
7. Commit and push

Two rounds of merge were needed:
- Wave 1 (prior round): 22 chapters attempted, 20 merged, Chap36 fully reverted
- Wave 2 (current round): 18 chapters attempted, all merged with 7 file reverts

---

## Recommendations

1. **Fix the proof block detector** to be AST-aware before the next minimize run.
   See `plans/veracity-bug-minimize-proofs-not-ast-aware.md`.

2. **Always run RTT after merging minimize results.** Verus verification alone does
   not catch marker-shift corruption (ghost code is erased before exec).

3. **Commit logs immediately** after minimize runs complete. Do not rely on untracked
   files surviving rebases.

4. **Re-run reverted files** after the detector is fixed. The 8 reverted files and
   Chap36 represent ~1500 asserts that could still benefit from minimization.

5. **Rebase agents before assigning cross-chapter work.** Chap58 depends on Chap19;
   agents with stale Chap19 will fail.

6. **Remaining chapters** to minimize: Chap30, 43 (running), 52 (running), 58, 60, 65.
