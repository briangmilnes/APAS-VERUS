# R86 Agent 2 — Prove topological_sort_opt (2 holes), STEP 20

## Objective

Remove `external_body` from `topological_sort_opt` in `TopoSortStEph.rs` and
`TopoSortStPer.rs`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap55
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent2/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Context

`topo_sort` is already proved (R85 agent3). It wraps `topological_sort_opt`.

`topological_sort_opt` ensures:
```rust
spec_is_dag(graph) <==> result.is_some()
result.is_some() ==> spec_is_topo_order(graph, order@)
```

The topo_order direction (`is_some() ==> spec_is_topo_order`) is proved via the
ghost finish-time infrastructure from R85. What remains is the biconditional:
**`result.is_some() <==> spec_is_dag(graph)`**.

### What's already proved

- `dfs_finish_order` with conditional edge ordering ensures (R85)
- `dfs_finish_order_cycle_detect` — structural proof
- `topo_sort` — proved, delegates to `topological_sort_opt`
- 4 reachability lemmas (R85 agent3)
- `spec_is_topo_order` proof from reverse finish order (R85)

### What remains

The biconditional has two directions:

**DAG → Some (completeness):** If the graph is a DAG, `dfs_finish_order_cycle_detect`
returns true for every vertex (no back edges found), so `topological_sort_opt`
returns Some with the reversed finish order. This direction should follow from
`has_cycle` completeness — CycleDetect is fully proved (R84 agent2). The same
`spec_acyclic_ord` / ghost finish-time scheme works here.

**Some → DAG (soundness):** If `topological_sort_opt` returns Some, it means
`dfs_finish_order_cycle_detect` returned true for all vertices (no cycles found).
This means `has_cycle` would return false, which (by CycleDetect's proved ensures)
means `spec_is_dag(graph)`.

### Approach

The key insight: `topological_sort_opt` calls `dfs_finish_order_cycle_detect`
in a loop, same as `has_cycle`. The cycle detection logic is identical. You can
reuse the ghost finish-time ordering pattern from CycleDetect:

1. Thread `Ghost(ord)` and `Ghost(next_time)` through the DFS calls
2. Maintain `spec_acyclic_ord` / `spec_is_valid_ord` as loop invariant
3. After the loop: all vertices finished + acyclic ord → `spec_is_dag`
4. Conversely: if cycle detected (returns None), use `lemma_cycle_not_dag`

Read `CycleDetectStEph.rs` for the working ghost ord pattern. The infrastructure
is directly portable — `topological_sort_opt` has the same DFS loop structure.

## Important

- Do NOT modify DFS, CycleDetect, or SCC files.
- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.

## STEP 20

## Report

Write `plans/agent2-round86-report.md`.
