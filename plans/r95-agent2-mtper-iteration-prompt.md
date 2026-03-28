# R95 Agent 2 — Prove MtPer iteration external_body (num_edges, delete_vertex), STEP 20

## Objective

Remove the last external_body functions in AdjTableGraphMtPer:
- `num_edges` — iterate domain, sum neighbor set sizes
- `delete_vertex` — delete key, iterate domain, remove v from each neighbor set

Agent3 (R92) proved num_edges and vertices for StEph/StPer. Port the pattern to MtPer.
Agent4 (R92) proved delete_vertex for StEph/StPer. Port that too.

## Challenge: MtPer uses OrderedTableMtPer, not TableStEph

The StEph/StPer proofs use `Table::find_ref`, `Table::entries`, and full
insert/delete ensures. OrderedTableMtPer has weaker ensures (agent1 R95 is
strengthening them concurrently). Work with what's available — if MtPer
ensures are too weak, use assumes with `// blocked by OrderedTableMtPer weak
ensures` comments.

## Read first

- `src/Chap52/AdjTableGraphMtPer.rs` — your file
- `src/Chap52/AdjTableGraphStEph.rs` — proved num_edges/delete_vertex (your template)
- `src/Chap52/AdjTableGraphStPer.rs` — proved versions (alternate template)

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify StEph or StPer files.
- Do NOT modify OrderedTableMtPer (agent1 is on that).
- Assumes are OK for MtPer-weak-API gaps — document them.
- Removing external_body with assumes is progress over external_body alone.

## STEP 20

## Report

Write `plans/agent2-r95-mtper-iteration-report.md`.
