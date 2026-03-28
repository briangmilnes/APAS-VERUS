# R94 Agent 1 — Re-apply 10 Chap52 assume proofs (lost in merge), STEP 20

## Objective

In R92, you proved 10 assumes in Chap52 AdjTableGraph files. Those proofs were
lost in a merge conflict when agents 2/3/4's structural changes (capacity→requires,
iteration functions, delete_vertex) took priority. Re-apply your proofs on the
current codebase.

## Your R92 report documented these 10 proves

| # | File | Function | What was proved |
|---|------|----------|----------------|
| 1 | AdjTableGraphMtPer.rs | empty() | wf from type-level predicates + vacuous closure |
| 2 | AdjTableGraphMtPer.rs | insert_vertex() | capacity moved to requires |
| 3 | AdjTableGraphStPer.rs | out_neighbors() | Some branch from Table::find ensures |
| 4 | AdjTableGraphStPer.rs | out_neighbors() | None branch from find + empty ensures |
| 5 | AdjTableGraphStEph.rs | from_table() | wf from strengthened requires |
| 6 | AdjTableGraphStPer.rs | from_table() | wf from strengthened requires |
| 7 | AdjTableGraphStEph.rs | insert_edge() | dom.contains(u@) from Table::insert ensures |
| 8 | AdjTableGraphStEph.rs | insert_edge() | dom.contains(v@) from second insert/find_ref |
| 9 | AdjTableGraphStPer.rs | insert_edge() | dom.contains(u@) same chain |
| 10 | AdjTableGraphStPer.rs | insert_edge() | dom.contains(v@) same chain |

## What changed since R92

- Agent2 (R93) moved 5 capacity assumes to requires
- Agent3 (R92) proved num_edges + vertices (removed external_body, added proof bodies)
- Agent4 (R92) proved delete_vertex (removed external_body, added proof bodies)

The files are structurally different now. Re-read each function and re-apply
the same proof technique. The assumes should still be there — they just weren't
removed because your branch conflicted.

## Strategy

Read your R92 report (plans/agent1-r92-chap52-fifty-report.md) for the techniques.
Then for each of the 10 proves, find the assume in the current code and replace it
with the proof.

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify num_edges, vertices, or delete_vertex — those are already proved.
- Do NOT add new assumes.
- The files have changed — re-read before editing.

## STEP 20

## Report

Write `plans/agent1-r94-reapply-report.md`.
