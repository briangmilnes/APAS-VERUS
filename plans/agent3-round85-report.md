# Agent 3 — Round 85 Report

## Objective

Remove `external_body` from `topo_sort` in TopoSortStEph.rs and TopoSortStPer.rs.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 55 | TopoSortStEph.rs | 2 | 1 | -1 |
| 2 | 55 | TopoSortStPer.rs | 2 | 1 | -1 |
| **Total** | | | **4** | **2** | **-2** |

## Verification

- Chap55 isolate: **2143 verified, 0 errors** (up from 2128 baseline)
- Zero trigger warnings

## What Was Proved

Removed `external_body` from `topo_sort` in both files. The proof establishes
`spec_is_topo_order(graph, order@)` under the `spec_is_dag(graph)` assumption,
which requires four sub-properties:

1. **Length**: `order@.len() == graph@.len()` — from conservation law (finish_order.len() + num_false(visited) == n) plus all-visited.
2. **No duplicates**: `order@.no_duplicates()` — from conditional no-duplicates invariant on dfs_finish_order, maintained through reverse.
3. **Valid indices**: `order@[k] < graph@.len()` — from existing finish_order bounds, carried through reverse.
4. **Edge ordering**: for edge order[i]→order[j], i < j — the hard part.

## Proof Architecture

### Strengthened dfs_finish_order ensures (conditional, backward-compatible)

Added to dfs_finish_order without changing requires (SCC callers unaffected):

- **Prefix preservation**: old finish_order is a prefix of new
- **New elements unvisited**: elements pushed during this call were unvisited at call start
- **Conditional no_duplicates**: if old state had no_dup + elements visited, new state has no_dup
- **Conditional elements visited**: same condition → all elements in finish_order are visited
- **Vertex is last**: when vertex was unvisited, it's the last element pushed
- **Early return unchanged**: if vertex was already visited, finish_order length unchanged
- **Conditional neighbors explored**: all pushed vertices have their neighbors visited
- **Conditional edge ordered** (needs DAG): for any edge fo[a]→fo[b], b < a
- **Reachability**: all newly pushed elements are reachable from vertex

### New spec functions

- `spec_vertex_neighbors_visited` / `_per` — all neighbors of a vertex are visited
- `spec_neighbors_explored` / `_per` — all finish_order elements have neighbors visited
- `spec_edge_ordered` / `_per` — edge ordering among finish_order elements

### New proof lemmas (4 per file variant)

- `lemma_edge_implies_reachable` — single edge → reachable
- `lemma_self_reachable` — vertex reaches itself
- `lemma_reachable_via_edge` — edge u→v ∧ reachable(v, w) → reachable(u, w)
- `lemma_reachable_edge_contradicts_dag` — reachable(u, v) ∧ edge v→u → ¬DAG

### Edge ordering proof at push point (4-case analysis)

When pushing vertex at position n (last):
1. **old→old**: both in pre-push range → loop invariant
2. **vertex→old**: vertex at pos n, target at pos b < n → b < n = a ✓
3. **old→vertex**: impossible — old elements' neighbors are all visited, vertex was unvisited (for old(fo) elements); new elements are reachable from vertex, edge to vertex → cycle contradicts DAG
4. **vertex→vertex**: self-loop contradicts DAG

### topo_sort proof structure

Main loop maintains: no_dup, elements visited, neighbors explored, conditional edge ordered.
Reversed loop carries reversal relationship `reversed[m] == fo[n-1-m]`.
Edge ordering for reversed: `fo[a]→fo[b] ∧ b < a` translates to `order[i]→order[j] ∧ i < j` via index reversal.

## Remaining Holes

| # | Chap | File | Function | Hole Type |
|---|------|------|----------|-----------|
| 1 | 55 | TopoSortStEph.rs | topological_sort_opt | external_body |
| 2 | 55 | TopoSortStPer.rs | topological_sort_opt | external_body |

### What blocks topological_sort_opt

`topological_sort_opt` has a harder ensures: `is_some <==> is_dag`. This requires proving:
- **Cycle detection correctness**: `dfs_finish_order_cycle_detect` returns false iff a cycle exists. This requires tracking the recursion stack and proving that a back edge implies a cycle.
- **Completeness**: if the graph is a DAG, the function returns Some. The current structural proof covers this direction.
- **Soundness**: if the function returns Some, the graph is a DAG. Needs the cycle detection to correctly reject all cyclic graphs.

The edge ordering proof infrastructure built here (reachability lemmas, neighbors-explored, edge-ordered) is directly reusable for this.

## Techniques Used

- Conditional ensures (implications in postconditions) to add properties without changing requires
- Ghost variable snapshots (fo_pre, vis_pre) for chaining through recursive calls
- Proof decomposition: 4 small reachability lemmas (each < 30 lines) instead of monolithic proof
- 4-case analysis for edge ordering at push point
- Index reversal mapping for finish_order → topo_order translation

## Steps Used: 10 of 20
