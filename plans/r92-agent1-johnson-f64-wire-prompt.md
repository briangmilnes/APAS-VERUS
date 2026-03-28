# R92 Agent 1 — Wire JohnsonF64 to DijkstraF64, STEP 10

## Objective

Remove 2 `external_body` from JohnsonStEphF64 and JohnsonMtEphF64. Both have
`johnson_apsp` marked external_body because DijkstraStEphF64 wasn't available
when agent4 built them. It's now merged.

## What to do

1. Read `src/Chap59/JohnsonStEphI64.rs` — the working I64 version's `johnson_apsp`
2. Read `src/Chap59/JohnsonStEphF64.rs` — your file, find `johnson_apsp`
3. Remove `#[verifier::external_body]` from `johnson_apsp` in JohnsonStEphF64
4. Wire it to call `DijkstraStEphF64::dijkstra()` (now available)
5. Same for JohnsonMtEphF64's `johnson_apsp`

The helper functions (add_dummy_source, reweight_graph, adjust_distance, etc.)
are already proved. Only the top-level `johnson_apsp` that calls Dijkstra is
external_body.

## Isolation

```bash
scripts/validate.sh isolate Chap59
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify DijkstraStEphF64.rs or BellmanFordStEphF64.rs.
- Do NOT add assume or accept.
- If DijkstraF64's ensures don't provide what Johnson needs (e.g., missing wf
  on result), use external_body and report the gap.

## STEP 10

## Report

Write `plans/agent1-r92-johnson-wire-report.md`.
