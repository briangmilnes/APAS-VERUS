# R74 Agent 5 — Prove Chap64/65/66 graph algorithm holes (35 holes)

## Objective

Prove or eliminate 35 holes across 7 files in the graph algorithm chapters. These are
independent of Chap37/43 — clean dependencies, can be proved in isolation.

### Chap64 — Spanning Trees & TSP (6 holes)

**SpanTreeStEph.rs** (2 holes):
- `spanning_tree_star_contraction` (line ~53) — external_body, root cause
- `verify_spanning_tree` (line ~149) — external_body, root cause

**TSPApproxStEph.rs** (4 holes):
- `get_neighbors` (line ~294) — external_body, root cause
- `get_edge_weight` (line ~304) — external_body, root cause
- `euler_tour_dfs` (line ~111) — external_body, blocked by get_neighbors
- `euler_tour` (line ~89) — external_body, blocked by euler_tour_dfs

### Chap65 — Union-Find, Kruskal, Prim (11 holes)

**UnionFindStEph.rs** (5 holes):
- `insert` (line ~219) — external_body, root cause (cascades to all others)
- `find` (line ~231) — blocked by insert
- `union` (line ~256) — blocked by find
- `equals` (line ~277) — blocked by find
- `num_sets` (line ~285) — blocked by find

**KruskalStEph.rs** (3 holes):
- `sort_edges_by_weight` (line ~64) — external_body, root cause
- `mst_weight` (line ~286) — external_body, root cause
- `kruskal_mst` (line ~166) — blocked by sort_edges_by_weight

**PrimStEph.rs** (3 holes):
- `total` proof fn (line ~72) — assume on TotalOrder::le for PrimEntry
- `cmp` (line ~74) — external_body
- `mst_weight` (line ~335) — external_body

### Chap66 — Boruvka MST (18 holes)

**BoruvkaStEph.rs** (6 holes):
- `PartialEq for LabeledEdge<V>` (line ~64) — external impl
- `vertex_bridges` (line ~200) — external_body, root cause
- `bridge_star_partition` (line ~267) — external_body, root cause
- `boruvka_mst` (line ~369) — blocked by bridge_star_partition
- `boruvka_mst_with_seed` (line ~467) — blocked by boruvka_mst
- `mst_weight` (line ~480) — external_body, root cause

**BoruvkaMtEph.rs** (12 holes):
- `PartialEq for LabeledEdge<V>` (line ~49) — external impl
- 8 root-cause external_body functions (hash_coin_flips_mt, compute_remaining_mt,
  collect_mst_labels_mt, build_partition_map_mt, vertex_bridges_mt, filter_tail_to_head_mt,
  reroute_edges_mt, mst_weight)
- 3 downstream external_body functions

## Strategy

**Priority order** (maximize impact):
1. **UnionFindStEph.rs** — prove `insert` first (root cause for 4 downstream). Focus on
   the data structure invariant: parent array + rank array maintain union-find properties.
2. **TSPApproxStEph.rs** — `get_neighbors` and `get_edge_weight` are graph utility
   functions, likely straightforward.
3. **SpanTreeStEph.rs** — 2 algorithmic functions.
4. **KruskalStEph.rs** — `sort_edges_by_weight` may need sorting proof; `mst_weight` is
   a reduction.
5. **BoruvkaStEph.rs** — start with `vertex_bridges` and `mst_weight`.
6. **BoruvkaMtEph.rs** — mirror StEph proofs with RwLock wrappers.
7. **PrimStEph.rs** — float TotalOrder proof (may need axioms from vstdplus/float.rs).

## Assigned files

| # | Chap | File | Holes |
|---|------|------|-------|
| 1 | 64 | SpanTreeStEph.rs | 2 external_body |
| 2 | 64 | TSPApproxStEph.rs | 4 external_body |
| 3 | 65 | UnionFindStEph.rs | 5 external_body |
| 4 | 65 | KruskalStEph.rs | 3 external_body |
| 5 | 65 | PrimStEph.rs | 1 assume + 2 external_body |
| 6 | 66 | BoruvkaStEph.rs | 5 external_body + 1 external |
| 7 | 66 | BoruvkaMtEph.rs | 11 external_body + 1 external |

## Validation

```bash
scripts/validate.sh    # must pass: 4735+ verified, 0 errors
scripts/rtt.sh         # must pass: 2619+ tests
```

Fix all warnings in your assigned files before committing.

## Required reading (before writing any code)

1. `CLAUDE.md` — project rules.
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — RwLock Mt wrapper pattern
   (for BoruvkaMtEph).
3. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone assume pattern (ONLY in
   eq/clone bodies). Relevant for `LabeledEdge<V>` PartialEq impls.
4. For each Mt file, read the St counterpart FIRST:
   - `src/Chap66/BoruvkaStEph.rs` before `BoruvkaMtEph.rs`.

## Rules

- Do NOT weaken ensures to make proofs easier.
- Do NOT add `accept()` or convert `assume` to `accept`.
- Do NOT sequentialize MtEph parallel code.
- Commit to your branch, push to `origin/agent5/ready`.
- Write report to `plans/agent5-round74-report.md`.
