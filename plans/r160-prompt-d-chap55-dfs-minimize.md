# R160 Prompt D — Minimize Chap55 DFS Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read files in `src/Chap55/`.

Report: `plans/r160-minimize-chap55-report.md`

## You are a senior proof engineer.

Remove unnecessary proof assertions. Strip all, read errors, add back minimum.

## Approach

For each function:
1. Read the ensures and loop invariants.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate: `scripts/validate.sh isolate Chap55` (89s per run).
4. Read which obligations fail. Add back MINIMUM.
5. Iterate. STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls, `choose` expressions, `assert forall` headers.
- Graph reachability proofs (these are real invariants).
- Visited-set monotonicity proofs.
- Cycle detection logic.

## What to remove

- Tautologies, intermediate equalities, redundant case splits.

## Functions to minimize

| # | File | Function | Lines | Asserts |
|---|------|----------|-------|---------|
| 1 | CycleDetectStEph.rs | dfs_check_cycle | 359 | 151 |
| 2 | CycleDetectStPer.rs | dfs_check_cycle | 237 | 113 |
| 3 | TopoSortStEph.rs | dfs_finish_order | 236 | 123 |
| 4 | TopoSortStPer.rs | dfs_finish_order | 204 | ~100 |

Real graph proof — expect 30-50% reduction, not 85%. Do StEph variants
first, apply patterns to StPer.

## Validation

`scripts/validate.sh isolate Chap55` — 89 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
