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
