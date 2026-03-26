# Agent 1 Round 82b Report — Chap55 external_body Removal

## Summary

Removed 13 `external_body` annotations from Chap55 graph algorithm files, proving
the structural properties of DFS traversal, cycle detection, topological sort helpers,
and SCC helpers. Verification count increased from 4685 to 4733 (+48 verified).

## Results

| # | Chap | File | Function | Status |
|---|------|------|----------|--------|
| 1 | 55 | DFSStEph.rs | `dfs_recursive` | **Proved** |
| 2 | 55 | DFSStPer.rs | `dfs_recursive` | **Proved** |
| 3 | 55 | CycleDetectStEph.rs | `dfs_check_cycle` | **Proved** |
| 4 | 55 | CycleDetectStPer.rs | `dfs_check_cycle` | **Proved** |
| 5 | 55 | TopoSortStEph.rs | `dfs_finish_order` | **Proved** |
| 6 | 55 | TopoSortStEph.rs | `dfs_finish_order_cycle_detect` | **Proved** |
| 7 | 55 | TopoSortStPer.rs | `dfs_finish_order` | **Proved** |
| 8 | 55 | TopoSortStPer.rs | `dfs_finish_order_cycle_detect` | **Proved** |
| 9 | 55 | SCCStEph.rs | `check_wf_adj_list_eph` | **Proved** |
| 10 | 55 | SCCStEph.rs | `dfs_reach` | **Proved** |
| 11 | 55 | SCCStPer.rs | `dfs_finish_order` | **Proved** |
| 12 | 55 | SCCStPer.rs | `check_wf_adj_list_per` | **Proved** |
| 13 | 55 | SCCStPer.rs | `dfs_reach` | **Proved** |

## Remaining external_body (14)

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 55 | DFSStEph.rs | `dfs` (trait impl) | Correctness: reachability <==> spec_reachable |
| 2 | 55 | DFSStPer.rs | `dfs` (trait impl) | Correctness: reachability <==> spec_reachable_per |
| 3 | 55 | CycleDetectStEph.rs | `has_cycle` (trait impl) | Correctness: has_cycle == !spec_is_dag |
| 4 | 55 | CycleDetectStPer.rs | `has_cycle` (trait impl) | Correctness: has_cycle == !spec_is_dag_per |
| 5 | 55 | TopoSortStEph.rs | `topological_sort_opt` | Correctness: DAG <==> topo_order, spec_is_topo_order |
| 6 | 55 | TopoSortStEph.rs | `topo_sort` (trait impl) | Correctness: spec_is_topo_order |
| 7 | 55 | TopoSortStPer.rs | `topological_sort_opt` | Correctness: DAG <==> topo_order, spec_is_topo_order_per |
| 8 | 55 | TopoSortStPer.rs | `topo_sort` (trait impl) | Correctness: spec_is_topo_order_per |
| 9 | 55 | SCCStEph.rs | `compute_finish_order` | View bridge for tabulate + visited tracking |
| 10 | 55 | SCCStEph.rs | `transpose_graph` | from_vec view bridge for result_vecs |
| 11 | 55 | SCCStEph.rs | `scc` (trait impl) | Depends on compute_finish_order + transpose |
| 12 | 55 | SCCStPer.rs | `compute_finish_order` | View bridge for Vec<bool> + finish tracking |
| 13 | 55 | SCCStPer.rs | `transpose_graph` | from_vec view bridge for result_vecs |
| 14 | 55 | SCCStPer.rs | `scc` (trait impl) | Depends on compute_finish_order + transpose |

## Techniques Used

1. **View-to-spec_index bridge lemmas**: `lemma_bool_view_eq_spec_index`,
   `lemma_usize_view_eq_spec_index`, `lemma_graph_view_bridge` — connect
   `ArraySeqStEphS@[j]` to `spec_index(j)` for Z3.

2. **Extensional equality for visited update**: After `visited.set(vertex, true)`,
   assert `visited@ =~= old(visited)@.update(vertex, true)` to connect the lemma
   `lemma_set_true_num_false_eq`.

3. **Ensures using `graph@.len()`**: Changed ensures from `old(visited)@.len()` to
   `graph@.len()` on free functions to prevent Z3 from losing function-entry facts
   deep in loop bodies after recursive calls.

4. **Neighbors view bridge in loop invariant**: Must persist `*neighbors == graph.spec_index(vertex)`
   and `neighbors@ =~= graph@[vertex]` through the loop invariant, since Z3 loses
   these after recursive calls that modify `&mut` parameters.

5. **Combined bounds**: `reachable@.len() + spec_num_false(visited@) <= graph@.len()`
   (DFS) and `component@.len() + spec_num_false(visited@) <= init_comp_len + spec_num_false(old(visited)@)`
   (SCC dfs_reach) to bound set sizes for insert preconditions.

## What Blocks the Remaining Functions

The 8 trait-impl and top-level functions require **correctness proofs** — not just structural
properties. These involve:
- **Reachability**: DFS visits exactly the reachable vertices (graph theory induction)
- **DAG detection**: No back edge <==> acyclic graph
- **Topological ordering**: Reverse finish order respects edge direction
- **SCC decomposition**: Kosaraju's algorithm correctness

The `compute_finish_order` and `transpose_graph` functions are structurally provable but
require additional work on view bridges for `tabulate` and `from_vec` in non-verus contexts.

## Verification

- Validate: 4733 verified, 0 errors
- RTT: Pre-existing 10 compile errors (Chap55 spec imports in non-verus mode — not caused by this round)
- PTT: 157 passed, 0 skipped
