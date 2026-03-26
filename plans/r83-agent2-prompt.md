# R83 Agent 2 — Prove CycleDetect + TopoSort (4 files, 6 holes), STEP 20

## Objective

Remove `external_body` from:
1. `CycleDetectStEph.rs:214` — `has_cycle`
2. `CycleDetectStPer.rs:170` — `has_cycle`
3. `TopoSortStEph.rs:478` — `topological_sort_opt`
4. `TopoSortStEph.rs:545` — `topo_sort`
5. `TopoSortStPer.rs:323` — `topological_sort_opt`
6. `TopoSortStPer.rs:396` — `topo_sort`

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap55
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB and will OOM.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh` — orchestrator runs those after merge.
Push to `agent2/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Context

### CycleDetect

`has_cycle` calls `dfs_check_cycle` (already proved) which returns true if a back
edge is found during DFS. The ensures: `result == !spec_is_dag(graph)`.

- `spec_is_dag(graph)` = no directed cycle exists (no path from v back to v)
- Back edge during DFS ↔ cycle in directed graph

Soundness (back edge → cycle): if DFS finds edge u→v where v is on the current
DFS stack (gray), then the stack path from v to u plus edge u→v forms a cycle.

Completeness (cycle → back edge): if a cycle exists, DFS must encounter a back
edge. By contradiction: if all edges in the cycle are tree/forward/cross edges,
the cycle vertices would have inconsistent discovery/finish times.

### TopoSort

`topological_sort_opt` calls `dfs_finish_order_cycle_detect` (already proved) to
get finish order, checks for cycles, returns reverse finish order if DAG.

`topo_sort` is the trait impl that wraps `topological_sort_opt`.

The ensures: `spec_is_dag(graph) ==> spec_is_topo_order(graph, order@)`.

- `spec_is_topo_order` = no duplicates, all vertices present, and for every edge
  u→v, u appears before v in the ordering.
- Reverse DFS finish order is a valid topological order for DAGs. Key property:
  for edge u→v in a DAG, finish[u] > finish[v] (u finishes after v).

### Available lemmas

- `lemma_set_true_decreases_num_false` — proved
- `lemma_set_true_num_false_eq` — proved
- `lemma_all_true_num_false_zero` — proved
- `dfs_finish_order` — proved (structural)
- `dfs_finish_order_cycle_detect` — proved (structural)
- `dfs_check_cycle` — proved (structural)

### Priority

Start with CycleDetect (simpler — 2 functions). Then TopoSort. `topo_sort` is a
thin wrapper around `topological_sort_opt`, so proving opt proves both.

If correctness proofs are too hard within 20 steps, strengthen the structural
ensures on the already-proved helpers to carry more information (e.g., back-edge
detection implies cycle existence) and leave the final correctness proof for a
future round.

## STEP 20

## Report

Write `plans/agent2-round83-report.md`.
