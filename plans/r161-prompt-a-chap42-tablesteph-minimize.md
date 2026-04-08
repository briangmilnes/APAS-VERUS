# R161 Prompt A — Minimize Chap42 TableStEph Proofs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER run RTTs.** Skip `scripts/rtt.sh` entirely.
5. **NEVER delete `target/` or any subdirectory.**

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
- ALL `lemma_entries_to_map_*` calls — Z3 cannot chain through `spec_entries_to_map`.
- Key-value mapping proofs connecting entries to the Map view.
- Well-formedness maintenance through insert/delete.
- The `spec_index → kept@` bridge assert after `from_vec` — load-bearing.

## Prior art from R160 (union was already minimized)

R160 minimized `union` in this file. Key findings that apply to ALL remaining functions:
- `spec_index` asserts MUST be placed **unconditionally before** if/else branches.
  Z3 processes branches independently and cannot hoist facts from siblings.
- `ArraySeqStEphS::lemma_view_index` bridges `spec_index` to the view — keep these.
- Choose-equality restatements are usually redundant (Z3 derives from choose condition).
- Length tautologies (`len == i + 1`, `len < usize::MAX`) are usually removable.
- Duplicate bounds assertions established earlier in the block are removable.

## What to remove

- Tautologies, intermediate equalities, redundant case splits.
- `assert(obeys_feq_full_trigger::<T>())` — broadcast handles this.

## Functions to minimize

| # | Function | Proof Lines | Asserts | Priority |
|---|----------|-------------|---------|----------|
| 1 | insert_wf | 191 | ~55 | First — biggest remaining |
| 2 | insert | 141 | ~45 | |
| 3 | delete_wf | 95 | ~30 | |
| 4 | intersection | 90 | ~30 | |
| 5 | difference | 81 | ~25 | |
| 6 | filter | 78 | ~25 | |
| 7 | delete | 67 | ~20 | |
| 8 | restrict | 66 | ~20 | |
| 9 | subtract | 66 | ~20 | |
| 10 | tabulate | 59 | ~18 | |

~288 asserts remaining (union already minimized in R160). Start with insert_wf.

## Validation

`scripts/validate.sh isolate Chap42` — 94 seconds per run.
Full `scripts/validate.sh` before pushing.

## Report

Per function: assert count before/after, line count before/after,
isolate validation time before/after.

## When done

RCP.
