# R160 Prompt A — Minimize Chap39 BSTParaTreapMtEph Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap39/BSTParaTreapMtEph.rs` — your file.

Report: `plans/r160-minimize-chap39-report.md`

## You are a senior proof engineer.

Your job is to remove unnecessary proof assertions. Do NOT test them one
by one. Think about what each proof NEEDS.

## Approach

For each function:
1. Read the ensures and loop invariants — these are the proof obligations.
2. Comment out ALL proof blocks and standalone asserts in the function body.
3. Validate: `scripts/validate.sh isolate Chap39` (17s per run).
4. Read which obligations fail. Add back MINIMUM assertions to fix each.
5. Iterate (usually 3-5 rounds). STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls — Z3 can't discover library lemmas.
- ALL `choose` expressions — existential witnesses Z3 can't guess.
- ALL `assert forall ... by { }` headers — these ARE the proof obligations.
  Simplify the `by` bodies but keep the headers.
- ALL real math: heap property, BST ordering, size tracking through rotations,
  priority comparisons.

## What to remove

- `assert(x == x)` tautologies.
- `assert(obeys_feq_full_trigger::<T>())` — broadcast handles this.
- Intermediate equalities restating what the previous line computed.
- Redundant case splits inside `by` blocks where Z3 handles the case analysis.
- `assert(false)` in dead code paths Z3 already knows are unreachable.

## Functions to minimize

| # | Function | Lines | Asserts | Priority |
|---|----------|-------|---------|----------|
| 1 | join_pair_inner | 208 | 262 | First — worst ratio |
| 2 | filter_inner | 155 | ~100 | |
| 3 | union_inner | 153 | ~100 | |
| 4 | difference_inner | 143 | ~95 | |
| 5 | intersect_inner | 142 | ~95 | |
| 6 | split_inner | 132 | ~90 | |

Start with join_pair_inner. Apply the patterns you learn to the others.

## Validation

`scripts/validate.sh isolate Chap39` — 17 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

For each function: assert count before/after, line count before/after,
isolate validation time before/after, which asserts were removed and why,
which are irreducible and why.

## When done

RCP.
