# R84 Agent 2 — Prove CycleDetect has_cycle completeness (2 holes), STEP 20

## Objective

Remove `external_body` from `has_cycle` in `CycleDetectStEph.rs:214` and
`CycleDetectStPer.rs:170`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap55
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh`.
Push to `agent2/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## PLAN FIRST, THEN EDIT

Read the files fully before making any changes. Understand what's already proved
and what remains.

## What's already proved (R83 agent2 work)

`dfs_check_cycle` (the recursive helper) now has two proved ensures:

1. **Soundness**: `has_cycle ==> !spec_is_dag(graph)` — if DFS finds a back edge,
   a cycle provably exists. Uses ghost `Seq<int>` DFS path parameter + cycle
   witness construction via `lemma_cycle_not_dag`.

2. **Ancestors restoration**: `!has_cycle ==> ancestors@ =~= old(ancestors)@` — if
   no cycle found, ancestors array is fully restored.

## What remains: completeness

`has_cycle` ensures: `result == !spec_is_dag(graph)` (biconditional).

The soundness direction (result true → !DAG) follows from dfs_check_cycle's ensures.
The missing direction: **result false → DAG** (if DFS finds no back edge, the graph
has no cycles).

### Approach: prove the contrapositive

Prove: `!spec_is_dag(graph) ==> dfs finds a back edge`.

If the graph has a cycle, there exist vertices v0→v1→...→vk→v0. DFS from any
start vertex will eventually visit one of these cycle vertices. When DFS reaches
a cycle vertex vi and recurses along cycle edges, it will reach vj that is already
an ancestor (on the DFS stack), triggering a back-edge detection.

### What you need

1. **Complete DFS coverage**: `has_cycle` calls `dfs_check_cycle` from every
   unvisited vertex. After the loop, all vertices are visited. You need to prove
   this — the loop visits vertex 0, then 1, etc., and dfs_check_cycle marks all
   reachable vertices as visited.

2. **Cycle vertex must be visited**: since all vertices get visited, any cycle
   vertex gets visited.

3. **Back edge on cycle**: when DFS first enters a cycle vertex, it will explore
   the cycle edges. Since the cycle forms a loop, one of these explorations will
   find an ancestor vertex still on the stack.

   This is the hard part. You may need a ghost invariant tracking: "for every
   cycle in the graph, if any cycle vertex is visited with this vertex as ancestor,
   a back edge will be found."

### Alternative: structural completeness

A potentially simpler approach: prove that `dfs_check_cycle` with the ancestors
restoration ensure gives you: "if no back edge found from vertex v, then there is
no cycle reachable from v through unvisited vertices."

Then `has_cycle`'s loop (which starts DFS from every unvisited vertex) gives:
"if no back edge found from any vertex, no cycle exists anywhere."

### Available infrastructure

- `spec_is_dag(graph)` — no directed cycle exists
- `spec_is_path(graph, path)` — valid directed path
- `spec_has_edge(graph, u, v)` — directed edge
- `spec_reachable(graph, u, v)` — path exists from u to v
- `lemma_cycle_not_dag(graph, path)` — proved: cycle path → !DAG
- Ghost DFS path parameter on `dfs_check_cycle` — proved working
- `lemma_reachable_step`, `lemma_reachable_self` — from DFS proof (agent1 R83)
- `lemma_neighbor_closed_path` — from DFS proof (agent1 R83)
- `spec_in_path` helper — from CycleDetect ghost path work

Read `DFSStEph.rs` for agent1's gray-set and neighbor-closure technique. The
completeness proof here has a similar flavor.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- Do NOT modify DFS, TopoSort, or SCC files — only CycleDetect.
- If the full correctness proof exceeds 20 steps, prove as much as possible and
  report what lemma/invariant is missing.

## STEP 20

## Report

Write `plans/agent2-round84-report.md`.
