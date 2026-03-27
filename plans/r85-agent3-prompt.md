# R85 Agent 3 — Prove TopoSort (4 holes) via proof decomposition, STEP 20

## Objective

Remove `external_body` from:
1. `TopoSortStEph.rs` — `topological_sort_opt`, `topo_sort`
2. `TopoSortStPer.rs` — `topological_sort_opt`, `topo_sort`

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap55
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent3/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## PLAN FIRST, THEN EDIT

The previous attempt (R83 agent2) found Z3 flakiness — postconditions verified in
some runs but not others. The fix is proof decomposition into small deterministic
lemmas that verify reliably, not hoping Z3 gets lucky.

### What needs to be proved

`topological_sort_opt` ensures:
- `spec_is_dag(graph) <==> result.is_some()`
- `result.is_some() ==> spec_is_topo_order(graph, order@)`

`topo_sort` is a thin wrapper that calls `topological_sort_opt`.

`spec_is_topo_order` requires:
1. `order.no_duplicates()` — each vertex appears exactly once
2. `order[k] < graph@.len()` — all entries are valid vertex indices
3. For every edge u→v: u appears before v in the ordering

### Available infrastructure (from R83 agent2)

- `dfs_finish_order` and `dfs_finish_order_cycle_detect` — proved (structural)
- Named closures for tabulate initialization — proved
- All-visited loop invariants with ghost monotonicity — in place (inside external_body)
- `lemma_set_true_*` family — proved
- `spec_is_path`, `spec_has_edge`, `spec_is_dag` — defined with fixed triggers (R83 agent1)

### Proof strategy: ghost finish-time ordering

The key property of DFS: for any edge u→v in a DAG, u finishes AFTER v
(finish_time[u] > finish_time[v]). Reverse finish order is therefore a valid
topological order.

**Step 1: Track finish times.** Add ghost parameters to `dfs_finish_order`:
- `Ghost(finish_time): Ghost<Map<int, nat>>` — maps vertex to its finish time
- `Ghost(next_time): Ghost<nat>` — monotonically increasing counter
- Ensures: after DFS from vertex, all reachable vertices have finish times,
  and for every tree edge u→v, finish_time[u] > finish_time[v].

This is the same pattern agent2 used successfully for CycleDetect (ghost ord +
next_time). Read `CycleDetectStEph.rs` lines 260-595 for the working example.

**Step 2: Prove no_duplicates.** Each vertex gets visited exactly once (visited
array). Each visited vertex gets pushed to finish_order exactly once (at DFS
completion). So finish_order has no duplicates.

**Step 3: Prove edge ordering.** For edge u→v in DAG: DFS from u explores v
before completing u. So finish_time[v] < finish_time[u]. In reverse finish
order, u comes before v.

**Step 4: Decompose into small lemmas.** Each step above should be a separate
proof fn with its own rlimit. Do NOT try to prove everything in one function.
Small lemmas that each verify deterministically avoid Z3 flakiness.

### Z3 flakiness mitigation

- Keep each proof fn under rlimit(30)
- Use `assert by` blocks to isolate quantifier contexts
- Use `reveal` only on the specific sub-predicate needed
- Avoid more than 3-4 quantifiers per function body
- If a lemma flakes, split it further

## Important

- Do NOT modify DFS, CycleDetect, or SCC files.
- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- If the full proof exceeds 20 steps, prove what you can and report what remains.

## STEP 20

## Report

Write `plans/agent3-round85-report.md`.
