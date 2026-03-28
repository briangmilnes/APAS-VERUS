# R101 Agent 1 — Prove delete_vertex graph closure loop (StEph + StPer), STEP 20

## Objective

The last 2 actionable Chap52 holes: delete_vertex graph closure assumes in
AdjTableGraphStEph (line 521) and AdjTableGraphStPer (line 473).

Agent4 R100 proved these but the merge broke due to conflicts with agent1 R99.
Re-do the proof on the clean current base.

## What to prove

After the delete_vertex loop (iterate domain, remove v from each neighbor set),
prove the graph closure invariant: `forall|u, v| adj[u].contains(v) ==> dom.contains(v)`.

## Agent4 R100 technique (from its report)

Agent4 successfully proved this with:
- Loop invariants tracking domain preservation and value subset
- v-removal tracking through each iteration
- Post-loop `assert forall` for graph closure

The Verus ICE that blocked this previously is GONE (confirmed in R99).

## Read first

- `src/Chap52/AdjTableGraphStEph.rs` — delete_vertex (around line 506)
- `src/Chap52/AdjTableGraphStPer.rs` — delete_vertex (around line 446)
- `plans/agent4-r100-delete-vertex-report.md` — agent4's technique description

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- The ICE is gone. You CAN write `assert forall` over `Map<V::V, Set<V::V>>`.
- Use `lemma_spec_stored_value_view` (added in R99) for stored-value-wf.
- These are the last 2 provable Chap52 holes. Make them count.

## STEP 20

## Report

Write `plans/agent1-r101-delete-vertex-report.md`.
