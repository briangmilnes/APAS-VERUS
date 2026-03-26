# R82b Agent 1 — Prove Chap55 external_body functions, STEP 20

## Objective

Remove `external_body` from functions in Chap55 and prove them. There are 27
external_body functions across 8 files. Prioritize by chapter order and difficulty.

## Isolation

Use isolated validation during development:
```bash
scripts/validate.sh isolate Chap55
```
This includes Chap55 + transitive deps (Chap02, 18, 19, 23, 37, 38, 41).
Before pushing, run a full `scripts/validate.sh` to confirm.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## Priority order

Start with the simplest files (fewest external_body, most mechanical):

### Tier 1 — DFS (4 external_body, 2 files)
- `DFSStEph.rs`: `dfs_recursive`, `dfs` (2)
- `DFSStPer.rs`: `dfs_recursive`, `dfs` (2)

DFS is a single recursive traversal. The loop/recursion marks vertices visited
and explores neighbors. The proof needs: wf propagation through recursive calls,
visited array tracking, and the ensures (all reachable vertices visited).

### Tier 2 — CycleDetect (4 external_body, 2 files)
- `CycleDetectStEph.rs`: `dfs_check_cycle`, `has_cycle` (2)
- `CycleDetectStPer.rs`: `dfs_check_cycle`, `has_cycle` (2)

Similar to DFS but tracks discovery/finish state for back-edge detection.

### Tier 3 — TopoSort (8 external_body, 2 files)
- `TopoSortStEph.rs`: `dfs_finish_order`, `dfs_finish_order_cycle_detect`,
  `topological_sort_opt`, `topo_sort` (4)
- `TopoSortStPer.rs`: same (4)

Builds on DFS finish order. The 4 proof lemmas in TopoSortStEph already verify.

### Tier 4 — SCC (11 external_body, 2 files)
- `SCCStEph.rs`: `compute_finish_order`, `transpose_graph`,
  `check_wf_adj_list_eph`, `dfs_reach`, `scc` (5)
- `SCCStPer.rs`: same + `dfs_finish_order` (6)

Most complex — Kosaraju's algorithm. Save for last.

## What blocks proving

From the R82 report:
- **wf propagation**: Functions calling AVLTreeSet insert/size need wf in invariants.
- **Seq spec_len**: Loop invariants need `neighbors.spec_len()` connections for `nth(i)`.
- **Graph wf instantiation**: Z3 needs `vertex < graph@.len()` to instantiate the wf forall.
- **Correctness proofs**: Reachability, cycle detection, topological ordering, SCC
  decomposition are non-trivial graph theory proofs. If a correctness postcondition
  is too hard, leave the function with `external_body` and report what you tried.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- If a function is too hard to prove in the step budget, leave it `external_body`
  and move to the next function. Report what blocks it.
- The 4 proof lemmas in TopoSortStEph (lemma_set_true_*) are already verified — use them.

## STEP 20

## Validation

Before pushing: run full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.
Push to `agent1/ready`.

## Report

Write `plans/agent1-round82b-report.md` with functions proved, external_body removed,
and what remains.
