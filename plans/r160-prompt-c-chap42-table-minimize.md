# R160 Prompt C — Minimize Chap42 Table Union Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap42/TableStEph.rs` and `src/Chap42/TableMtEph.rs`.

Report: `plans/r160-minimize-chap42-report.md`

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
- Key uniqueness proofs (these are the real invariants for Table).

## What to remove

- Tautologies, feq triggers, intermediate equalities.

## Functions to minimize

| # | File | Function | Lines | Asserts |
|---|------|----------|-------|---------|
| 1 | TableStEph.rs | union | 239 | 89 |
| 2 | TableMtEph.rs | union | 307 | 111 |

Same algorithm, St vs Mt. Minimize StEph first, apply pattern to MtEph.

## Validation

`scripts/validate.sh isolate Chap42` — 94 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
