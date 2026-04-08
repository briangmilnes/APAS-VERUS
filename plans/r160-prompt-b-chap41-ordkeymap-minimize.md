# R160 Prompt B — Minimize Chap41 OrdKeyMap Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap41/OrdKeyMap.rs` — your file.

Report: `plans/r160-minimize-chap41-report.md`

## You are a senior proof engineer.

Your job is to remove unnecessary proof assertions. Do NOT test them one
by one. Think about what each proof NEEDS.

## Approach

For each function:
1. Read the ensures and loop invariants.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate: `scripts/validate.sh isolate Chap41` (51s per run).
4. Read which obligations fail. Add back MINIMUM to fix each.
5. Iterate. STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls, `choose` expressions, `assert forall` headers.
- ALL TotalOrder transitivity chains (these are the real ordering proofs).
- ALL BST structural reasoning (left/right subtree exclusion).

## What to remove

- Tautologies, feq trigger assertions the broadcast handles.
- Intermediate equalities Z3 derives from the operation above.
- Redundant case splits inside `by` blocks.
- The minimizer already confirmed 79% of union_with asserts are removable.

## Functions to minimize

| # | Function | Lines | Asserts |
|---|----------|-------|---------|
| 1 | ordkeymap_split | 354 | 230 |
| 2 | ordkeymap_prev | 279 | 224 |
| 3 | ordkeymap_next | 282 | 214 |
| 4 | ordkeymap_rank | 257 | 207 |
| 5 | ordkeymap_select | 303 | 168 |
| 6 | union_with | 258 | 164 |
| 7 | union | 230 | 164 |

Start with union_with (we know 79% is removable). next/prev are mirrors —
do next, apply symmetrically to prev.

## Validation

`scripts/validate.sh isolate Chap41` — 51 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
