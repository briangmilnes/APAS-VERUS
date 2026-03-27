# R88 Agent 2 — Fix AdjTableGraphMtPer + EdgeSetGraphMtPer, STEP 20

## Objective

Fix 2 files that reference the removed `.elements` field on AVLTreeSetMtPer.
Replace with trait method calls.

Files to fix:
1. `src/Chap52/AdjTableGraphMtPer.rs`
2. `src/Chap52/EdgeSetGraphMtPer.rs`

## lib.rs — uncomment your files

Uncomment BOTH files in lib.rs. They are currently commented out with `// BROKEN`.
Remove the comment prefix.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap52
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent2/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

`AVLTreeSetMtPer<V>` was refactored — the old `elements` field (AVLTreeSeqMtPer)
is now `tree` (ParamBST). The MtPer files still reference `.elements`.

Read these working files for the current API:
- `src/Chap41/AVLTreeSetMtPer.rs` — current MtPer trait and impl
- `src/Chap41/AVLTreeSetMtEph.rs` — current MtEph trait and impl
- `src/Chap52/AdjSeqGraphMtPer.rs` — working MtPer graph for reference
- `src/Chap52/EdgeSetGraphStPer.rs` — the StPer version (R82b agent4 proved it clean)

### AdjTableGraphMtPer

This file wraps AdjTableGraphStPer in RwLock. The StPer version was switched from
OrderedTable to Table by R82b agent3. Check if the MtPer version needs the same
Table switch, or if it wraps the StPer directly.

### EdgeSetGraphMtPer

This file wraps EdgeSetGraphStPer in RwLock. The StPer version is clean (0 holes).
The MtPer should follow the standard coarse-locking wrapper pattern.

## Important

- Do NOT modify files in Chap41 — only Chap52 MtPer files.
- Do NOT add `assume` or `accept`.
- Use `external_body` on functions that are too hard to prove within the step budget.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` for the locking pattern.

## STEP 20

## Report

Write `plans/agent2-round88-report.md`.
