# R157 Agent 3 — Final StEph + StPer Cleanup: Delete Dead Code. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap43/OrderedTableStEph.rs` and `src/Chap43/OrderedTableStPer.rs`.

Report file: `plans/r157-agent3-cleanup-report.md`

## Problem

After rounds of delegation, both OrderedTable files have:
- Bridge lemmas that may no longer be called
- Spec fns that were only used by deleted bst_*_by_key functions
- `#[cfg(never)]` bypassed code that should be deleted
- Dead imports

## What to do

### For each file (StEph and StPer):

1. Find all `#[cfg(never)]` blocks. Delete them entirely (they're bypassed
   dead code from previous rounds, not active proof work).

2. For each bridge lemma in section 7 (`lemma_pair_set_to_map_*`,
   `lemma_key_unique_*`, `lemma_view_gen_*`, `lemma_cmp_*`,
   `lemma_sorted_keys_*`, `lemma_set_to_map_*`):
   - Search for callers in the same file
   - If zero callers, delete the lemma (it now lives in OrdKeyMap)
   - If still called, leave it

3. For each bridge spec fn (`spec_pair_set_to_map`, `spec_key_unique_pairs_set`,
   `spec_set_pair_view_generated`, `spec_rank_pred`, `spec_ord_agrees_total_order`):
   - Search for callers
   - If zero callers, delete

4. Clean up dead imports (`use` statements for removed functions)

5. Validate after each batch of deletions:
   `scripts/validate.sh isolate Chap43`

## Expected reduction

Potentially ~500-800 lines across both files from dead lemmas and spec fns.

## Validation

`scripts/validate.sh isolate Chap43` after each batch.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.
- Delete only genuinely dead code (zero callers in file).

## When done

RCP. Report what was deleted and line counts.
