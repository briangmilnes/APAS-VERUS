# R161 Prompt D — Minimize Chap65 UnionFindStEph Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER run RTTs.** Skip `scripts/rtt.sh` entirely.
5. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap65/UnionFindStEph.rs` — your file.

Report: `plans/r161-minimize-chap65-unionfind-report.md`

## You are a senior proof engineer.

Remove unnecessary proof assertions. Strip all, read errors, add back minimum.

## Approach

For each function:
1. Read the ensures and loop invariants.
2. Comment out ALL proof blocks and standalone asserts.
3. Validate: `scripts/validate.sh isolate Chap65` (30s per run).
4. Read which obligations fail. Add back MINIMUM.
5. Iterate. STEP 15 per function — move on if stuck.

## What to keep

- ALL lemma calls, `choose` expressions, `assert forall` headers.
- Parent array invariants (well-formedness, acyclicity, rank bounds).
- Union-by-rank ordering proofs.
- Path compression correctness proofs.

## What to remove

- Tautologies, intermediate equalities, redundant case splits.
- `assert(obeys_feq_full_trigger::<T>())` — broadcast handles this.

## Functions to minimize

| # | Function | Proof Lines | Kind | Priority |
|---|----------|-------------|------|----------|
| 1 | lemma_insert_preserves_wf | 98 | proof | First |
| 2 | lemma_union_wf_ordering | 89 | proof | |
| 3 | lemma_union_wf_parent | 76 | proof | |
| 4 | lemma_union_merge_wf | 73 | proof | |
| 5 | lemma_rank_lt_elements | 65 | proof | |
| 6 | lemma_establish_union_pre | 58 | proof | |
| 7 | union_merge_exec | 56 | exec | |

106 asserts total. These are mostly proof lemmas (6 of 7), not exec
functions. Union-Find wf is real math — expect 30-50% reduction.
Start with lemma_insert_preserves_wf. Apply patterns to the rest.

## Validation

`scripts/validate.sh isolate Chap65` — estimated 30 seconds per run.
Full `scripts/validate.sh` before pushing.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
