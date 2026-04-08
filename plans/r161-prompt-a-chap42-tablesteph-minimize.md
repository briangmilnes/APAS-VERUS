# R161 Prompt A — Minimize Chap42 TableStEph Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap42/TableStEph.rs` — your file.

Report: `plans/r161-minimize-chap42-tablesteph-report.md`

## You are a senior proof engineer.

Remove unnecessary proof assertions. Strip all, read errors, add back minimum.

## Approach

For each function:
1. Read the ensures and loop invariants.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate: `scripts/validate.sh isolate Chap42` (94s per run).
4. Read which obligations fail. Add back MINIMUM.
5. Iterate. STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls, `choose` expressions, `assert forall` headers.
- Hash table index arithmetic proofs (modular arithmetic is real math).
- Key-value mapping proofs connecting entries to the Map view.
- Well-formedness maintenance through insert/delete.

## What to remove

- Tautologies, intermediate equalities, redundant case splits.
- `assert(obeys_feq_full_trigger::<T>())` — broadcast handles this.

## Functions to minimize

| # | Function | Proof Lines | Asserts | Priority |
|---|----------|-------------|---------|----------|
| 1 | union | 239 | ~70 | First — biggest |
| 2 | insert_wf | 191 | ~55 | |
| 3 | insert | 141 | ~45 | |
| 4 | delete_wf | 95 | ~30 | |
| 5 | intersection | 90 | ~30 | |
| 6 | difference | 81 | ~25 | |
| 7 | filter | 78 | ~25 | |
| 8 | delete | 67 | ~20 | |
| 9 | restrict | 66 | ~20 | |
| 10 | subtract | 66 | ~20 | |
| 11 | tabulate | 59 | ~18 | |

334 asserts total. Start with union. Apply patterns to the rest.

## Validation

`scripts/validate.sh isolate Chap42` — 94 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
