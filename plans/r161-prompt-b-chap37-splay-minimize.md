# R161 Prompt B — Minimize Chap37 BSTSplayStEph Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap37/BSTSplayStEph.rs` — your file.

Report: `plans/r161-minimize-chap37-splay-report.md`

## You are a senior proof engineer.

Remove unnecessary proof assertions. Strip all, read errors, add back minimum.

## Approach

For each function:
1. Read the ensures and loop invariants.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate: `scripts/validate.sh isolate Chap37` (30s per run).
4. Read which obligations fail. Add back MINIMUM.
5. Iterate. STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls, `choose` expressions, `assert forall` headers.
- BST ordering invariants through rotations — this is real proof.
- Size/height maintenance through zig, zig-zig, zig-zag cases.
- Splay amortized analysis invariants if present.

## What to remove

- Tautologies, intermediate equalities, redundant case splits.
- `assert(obeys_feq_full_trigger::<T>())` — broadcast handles this.

## Functions to minimize

| # | Function | Proof Lines | Asserts | Priority |
|---|----------|-------------|---------|----------|
| 1 | splay | 812 | ~180 | First — THE monster |
| 2 | bst_insert | 101 | ~35 | |

250 asserts total. splay is the #1 biggest function in the entire codebase.
Expect heavy reduction — 812 proof lines for a splay operation suggests
massive AI slop. Real splay proof is zig/zig-zig/zig-zag cases with BST
ordering through rotations. STEP 20 for splay given its size.

## Validation

`scripts/validate.sh isolate Chap37` — estimated 30 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
