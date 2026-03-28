# R99 Agent 2 — Fix MtPer capacity + strengthen map ensures, STEP 15

## Objective

Fix 3 MtPer holes:
1. `insert_edge` capacity assume (needs +3 not +2 in requires)
2-3. `delete_vertex` graph wf + dom exclusion (needs stronger map ensures)

## Hole 1: insert_edge capacity

Line 463: `assume(new_adj@.dom().len() + 1 < usize::MAX)`. The function
requires `dom.len() + 2 < usize::MAX` but after two conditional inserts
the domain may have grown by 2. Change requires to `dom.len() + 3 < usize::MAX`
to cover the third insert (neighbor set update). Or prove the domain only grows
by at most 2 by tracking sizes through the conditional branches.

## Holes 2-3: delete_vertex — SKIP if ICE blocks

Lines 362-363. These need quantifiers over map values which may trigger the
Verus ICE. Agent1 R99 is testing whether the ICE still exists. If agent1
reports it's gone, attempt these. If agent1 reports it persists, skip them
and focus only on hole 1 (capacity).

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — find the 3 assumes
- `src/Chap43/OrderedTableMtPer.rs` — map() signature and ensures

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 15

## Report

Write `plans/agent2-r99-capacity-map-report.md`.
