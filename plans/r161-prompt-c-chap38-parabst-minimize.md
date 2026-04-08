# R161 Prompt C — Minimize Chap38 BSTParaStEph Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap38/BSTParaStEph.rs` — your file.

Report: `plans/r161-minimize-chap38-parabst-report.md`

## You are a senior proof engineer.

Remove unnecessary proof assertions. Strip all, read errors, add back minimum.

## Approach

For each function:
1. Read the ensures and loop invariants.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate: `scripts/validate.sh isolate Chap38` (20s per run).
4. Read which obligations fail. Add back MINIMUM.
5. Iterate. STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls, `choose` expressions, `assert forall` headers.
- Set algebra proofs (union/intersect/difference correctness).
- BST ordering maintenance through split/join.
- Size tracking through recursive operations.

## What to remove

- Tautologies, intermediate equalities, redundant case splits.
- `assert(obeys_feq_full_trigger::<T>())` — broadcast handles this.

## Functions to minimize

| # | Function | Proof Lines | Asserts | Priority |
|---|----------|-------------|---------|----------|
| 1 | difference | 144 | ~50 | First |
| 2 | intersect | 137 | ~45 | |
| 3 | split | 119 | ~40 | |
| 4 | union | 91 | ~30 | |
| 5 | collect_in_order | 87 | ~30 | |
| 6 | filter_inner | 53 | ~18 | |

293 asserts total. These are the core parallel BST set operations.
Real set algebra — expect 30-50% reduction, not 85%.
Start with difference. Apply patterns to the rest.

## Validation

`scripts/validate.sh isolate Chap38` — estimated 20 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
