# R83 Agent 3 — Prove SCC (StEph + StPer, 6 holes), STEP 20

## Objective

Remove `external_body` from:
1. `SCCStEph.rs:86` — `compute_finish_order`
2. `SCCStEph.rs:160` — `transpose_graph`
3. `SCCStEph.rs:435` — `scc` (blocked by #1, #2)
4. `SCCStPer.rs:172` — `compute_finish_order`
5. `SCCStPer.rs:248` — `transpose_graph`
6. `SCCStPer.rs:498` — `scc` (blocked by #4, #5)

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap55
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB and will OOM.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh` — orchestrator runs those after merge.
Push to `agent3/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Context

Kosaraju's SCC algorithm:
1. `compute_finish_order` — run DFS on all vertices, record finish order
2. `transpose_graph` — reverse all edges
3. `scc` — run DFS on transposed graph in reverse finish order; each DFS tree is an SCC

### compute_finish_order (structural, not correctness)

Iterates over all vertices, calls `dfs_finish_order` (already proved) on unvisited ones.
The body is straightforward — loop over vertices 0..n, skip visited, call dfs_finish_order.

The R82b report says the blocker is "view bridge for tabulate + visited tracking."
This means Z3 needs help connecting the Vec<bool> visited array to the spec-level
predicates. Use the view bridge lemmas from R82b agent1's work:
- `lemma_bool_view_eq_spec_index`
- `lemma_usize_view_eq_spec_index`

### transpose_graph (structural)

Creates a new adjacency list with all edges reversed. For each vertex u, iterate its
neighbors; for each neighbor v, add u to v's adjacency list in the transposed graph.

The R82b report says the blocker is "from_vec view bridge for result_vecs." This means
proving that the Vec<Vec<N>> result, when wrapped in ArraySeqStEph, has the correct view.

### scc (correctness — Kosaraju's)

Calls compute_finish_order, transpose_graph, then runs DFS on the transposed graph.
The ensures is `spec_is_scc(graph, components)` — components are strongly connected,
partition vertices, are disjoint, and respect topological order.

This is the hardest proof in Chap55. Kosaraju's correctness relies on:
- Finish order property: if C1 and C2 are SCCs with edge C1→C2, then max_finish(C1) > max_finish(C2)
- DFS on transposed graph in reverse finish order discovers complete SCCs

### Priority

1. `transpose_graph` (both variants) — structural, should be straightforward
2. `compute_finish_order` (both variants) — structural, view bridge work
3. `scc` (both variants) — correctness, hardest

If `scc` correctness is too hard within 20 steps, prove #1 and #2 and leave `scc`
with `external_body`. Report what the correctness proof would need.

## STEP 20

## Report

Write `plans/agent3-round83-report.md`.
