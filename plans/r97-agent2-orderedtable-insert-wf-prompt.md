# R97 Agent 2 — Add insert_wf to OrderedTableMtPer, STEP 20

## Objective

Agent2 R96 added `insert_wf` to TableStEph and TableStPer. Now add the same
to OrderedTableMtPer (Chap43) so AdjTableGraphMtPer can use it too.

## Pattern

OrderedTableMtPer wraps an inner persistent table with RwLock. The `insert_wf`
needs to:
1. Acquire lock
2. Call inner table's `insert_wf` (if available) or construct the value-wf chain
3. Release lock with strong ensures

Since OrderedTableMtPer already has `insert` with external_body and strong ensures
(from R95 agent1), you can add `insert_wf` as another external_body with EVEN
stronger ensures (includes stored-value-wf preservation).

Or if the inner table is a plain OrderedTableStPer (which doesn't have insert_wf
yet), you may need to add insert_wf there first, then wrap in MtPer.

## Read first

- `src/Chap43/OrderedTableMtPer.rs` — your file
- `src/Chap43/OrderedTableStPer.rs` — inner table (does it have insert_wf?)
- `src/Chap42/TableStEph.rs` — insert_wf pattern (from R96 agent2)
- `src/Chap42/TableStPer.rs` — insert_wf pattern

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT break existing callers.
- External_body with strong ensures is acceptable (same pattern as R95).
- The goal is to provide the API. MtPer callers switch in a follow-up round.

## STEP 20

## Report

Write `plans/agent2-r97-orderedtable-insert-wf-report.md`.
