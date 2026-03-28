# R97 Agent 1 — Switch AdjTableGraph to insert_wf, eliminate stored-value-wf assumes, STEP 20

## Objective

Agent2 R96 added `insert_wf` to TableStEph and TableStPer. Now switch
AdjTableGraphStEph and AdjTableGraphStPer from `insert` to `insert_wf`
and remove the stored-value-wf assumes.

## What to do

For each `self.adj.insert(key, value, |_old, new| new.clone())` call:

Replace with:
```rust
self.adj.insert_wf(key, value,
    |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
        ensures r@ == new@, r.spec_wf()
    { new.clone_wf() });
```

Then remove the `assume(... spec_stored_value(k).spec_avltreesetsteph_wf())` that
follows — `insert_wf` ensures all stored values are wf.

## Target functions

In AdjTableGraphStEph:
- `insert_vertex` — inserts empty set (no combine needed, but may use insert_wf anyway)
- `insert_edge` — inserts neighbor set with combine
- `delete_edge` — inserts modified neighbor set with combine

Same in AdjTableGraphStPer.

## Requires chain

`insert_wf` requires:
1. `old(self).spec_tablesteph_wf()` — from graph wf
2. `value.spec_wf()` — the new AVLTreeSetStEph must be wf
3. `forall|k| old(self)@.contains_key(k) ==> old(self).spec_stored_value(k).spec_wf()` — all existing stored values wf (from graph wf strengthening done in R92/R94)
4. `combine ensures wf` — the clone_wf closure ensures this

If the graph wf doesn't include stored-value-wf, you'll need to add it.
Check if `spec_adjtablegraphsteph_wf` already has this from R94 agent1's work.

## Read first

- `src/Chap42/TableStEph.rs` — `insert_wf` signature and requires
- `src/Chap42/TableStPer.rs` — same
- `src/Chap52/AdjTableGraphStEph.rs` — find all `self.adj.insert` calls
- `src/Chap52/AdjTableGraphStPer.rs` — same
- `src/vstdplus/clone_view.rs` — ClonePreservesWf, clone_wf

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify Table files (Chap42).
- Do NOT add new assumes.
- Each stored-value-wf assume you remove is a real hole eliminated.
- If insert_wf requires can't be satisfied (missing stored-value-wf in graph wf),
  add it to graph wf and re-prove. Agent1 R94 already did this for from_table.

## STEP 20

## Report

Write `plans/agent1-r97-insert-wf-report.md`.
