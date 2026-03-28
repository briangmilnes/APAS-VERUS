# R96 Agent 3 — Prove MtPer external_body functions, STEP 20

## Objective

AdjTableGraphMtPer has remaining external_body functions. With the strengthened
OrderedTableMtPer ensures (from R95 agent1), some may now be provable.

Check which external_body functions remain in AdjTableGraphMtPer and prove
them using the new OrderedTableMtPer find/insert/delete ensures.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — find all `#[verifier::external_body]`
- `src/Chap43/OrderedTableMtPer.rs` — the strengthened ensures
- `src/Chap52/AdjTableGraphStEph.rs` — proved StEph versions for patterns

## What to expect

OrderedTableMtPer now has:
- find: `Some(v) => contains_key(k@) && [k@] == v@, None => !contains_key(k@)`
- insert: `dom() =~= self@.dom().insert(k@), wf()`
- delete: `updated@ == self@.remove(k@), wf()`

This should unblock at minimum:
- `has_edge` — find + check set membership
- `out_neighbors` — find + return
- `out_degree` — out_neighbors + size

Functions that mutate (insert_edge, delete_edge, insert_vertex, delete_vertex)
may still need assumes for graph-closure wf (ICE blocked).

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify OrderedTableMtPer.rs or any file outside Chap52.
- Do NOT add assume or accept unless ICE-blocked (then document).
- Each external_body removed is progress, even if assumes replace it.

## STEP 20

## Report

Write `plans/agent3-r96-mtper-extbody-report.md`.
