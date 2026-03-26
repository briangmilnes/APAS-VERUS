# R83 Agent 1 — Prove DFS (StEph + StPer), STEP 20

## Objective

Remove `external_body` from `dfs` in `DFSStEph.rs:214` and `DFSStPer.rs:179`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap55
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB and will OOM.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh` — orchestrator runs those after merge.
Push to `agent1/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Context

`dfs_recursive` is already proved in both files (R82b agent1 work). The `dfs` trait
impl wraps it: creates a visited array, calls `dfs_recursive(graph, visited, reachable, start)`,
returns the reachable set.

The ensures is the hard part — proving that the returned set equals `spec_reachable(graph, start)`:
- Everything in the result is reachable from start (soundness)
- Everything reachable from start is in the result (completeness)

### What's available

- `dfs_recursive` is proved with ensures:
  - visited entries set to true only for vertices reachable from the call
  - reachable set grows only with reachable vertices
  - spec_num_false(visited) decreases (termination)
- `spec_reachable(graph, u, v)` — exists a path from u to v
- `spec_is_path(graph, path)` — valid directed path
- `spec_has_edge(graph, u, v)` — directed edge exists
- `lemma_set_true_decreases_num_false` — proved
- `lemma_set_true_num_false_eq` — proved

### Approach

The key insight: `dfs_recursive` marks visited[v] = true for exactly the vertices
reachable from `vertex`. After `dfs(graph, start)` calls `dfs_recursive(graph, visited,
reachable, start)`:
- **Soundness**: every insert into reachable happens inside dfs_recursive when visiting
  a vertex — the vertex is reachable because we followed edges from start.
- **Completeness**: if v is reachable from start, there's a path start→...→v. Each edge
  in the path is explored by dfs_recursive (because the predecessor is visited, so its
  neighbors are iterated). So v gets visited.

You likely need an inductive lemma on path length: if there's a path of length k from
start to v, and dfs_recursive visits all vertices reachable via paths of length < k,
then it also visits v.

Read the files fully before starting. The StPer variant is structurally identical.

## STEP 20

## Report

Write `plans/agent1-round83-report.md`.
