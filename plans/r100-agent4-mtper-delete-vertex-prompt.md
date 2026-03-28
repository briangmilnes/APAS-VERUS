# R100 Agent 4 — Prove MtPer delete_vertex assumes (if map strengthened), STEP 15

## Objective

AdjTableGraphMtPer::delete_vertex has 2 assumes:
1. `assume(updated.spec_adjtablegraphmtper_wf())` — graph wf after map
2. `assume(!updated.spec_adj().dom().contains(v@))` — v removed from domain

Agent2 R100 is strengthening OrderedTableMtPer::map ensures concurrently.
If map now ensures `result@.dom() =~= self@.dom()`, you can prove assume #2.

## Method

1. Read `src/Chap52/AdjTableGraphMtPer.rs` delete_vertex
2. Check if OrderedTableMtPer::map has been strengthened (agent2 may have
   pushed by now — `git pull` or check the current code)
3. If map ensures domain preservation: prove `!dom.contains(v@)` from
   `delete(v)` ensures `remove(v@)` + map preserves domain
4. For graph wf (#1): may need map to ensure value properties too — if not
   available, leave the assume

## Timing

Agent2 is working on map ensures in Chap43. You're working on the caller in
Chap52. No file conflicts. But your proof depends on agent2's ensures landing.

If map ensures haven't been strengthened when you start:
- Try with the current weak ensures — see if you can prove anything anyway
- If blocked, write up exactly what map ensures you need and report it

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — delete_vertex (lines 355-365)
- `src/Chap43/OrderedTableMtPer.rs` — map and delete signatures

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 15

## Report

Write `plans/agent4-r100-delete-vertex-report.md`.
