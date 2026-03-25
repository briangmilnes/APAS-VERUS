# R75 Agent 2 — Prove Chap64 graph algorithms + Chap66 BoruvkaStEph (12 holes)

## Objective

Prove or eliminate 12 holes across 3 files: Chap64 SpanTree and TSPApprox, plus Chap66
BoruvkaStEph (sequential Boruvka).

## Files and holes

| # | Chap | File | Holes | Root causes |
|---|------|------|-------|-------------|
| 1 | 64 | SpanTreeStEph.rs | 2 | 2 root |
| 2 | 64 | TSPApproxStEph.rs | 4 | 2 root, 2 downstream |
| 3 | 66 | BoruvkaStEph.rs | 6 | 3 root, 2 downstream + 1 external |

### SpanTreeStEph.rs (2 holes — both external_body root causes)

- `spanning_tree_star_contraction()` — line ~53 — external_body root cause
- `verify_spanning_tree()` — line ~149 — external_body root cause

**Strategy**: These are graph algorithms operating on adjacency structures. Read the function
bodies to understand the algorithm. The spanning tree star contraction uses random coin flips
and vertex partitioning. Check if the specs are achievable given the current graph library
support in Chap52/Chap06.

### TSPApproxStEph.rs (4 holes — all external_body)

- `euler_tour_dfs()` — line ~111 — external_body root cause
- `euler_tour()` — line ~89 — external_body downstream (blocked by euler_tour_dfs)
- `get_neighbors()` — line ~294 — external_body root cause
- `get_edge_weight()` — line ~304 — external_body downstream (blocked by get_neighbors)

**Strategy**: `euler_tour_dfs` is a DFS traversal building an Euler tour. `get_neighbors`
and `get_edge_weight` are graph utility functions. Read the function bodies. DFS traversal
proofs typically need a visited-set invariant. Check how other graph traversal functions
in Chap52/Chap06 are proved.

### BoruvkaStEph.rs (6 holes — 5 external_body + 1 external)

- `vertex_bridges()` — line ~200 — external_body root cause
- `bridge_star_partition()` — line ~267 — external_body root cause
- `boruvka_mst()` — line ~369 — external_body downstream (blocked by vertex_bridges)
- `boruvka_mst_with_seed()` — line ~467 — external_body downstream (blocked by boruvka_mst)
- `mst_weight()` — line ~480 — external_body root cause
- `PartialEq for LabeledEdge` — line ~64 — external root cause

**Strategy**: Boruvka's MST algorithm uses star contraction. `vertex_bridges` finds minimum
weight edges per vertex. `bridge_star_partition` partitions based on coin flips and bridges.
The PartialEq external hole may be solvable using the standard eq/clone workaround pattern
from `src/standards/partial_eq_eq_clone_standard.rs`. `mst_weight` has the same float
arithmetic challenge as other MST files.

## Key resources

- `src/Chap64/SpanTreeStEph.rs`, `src/Chap64/TSPApproxStEph.rs`
- `src/Chap66/BoruvkaStEph.rs`
- `src/Chap06/` — Graph implementations (DirGraph, LabDirGraph)
- `src/Chap52/` — Adjacency representations
- `src/vstdplus/float.rs` — Float axioms
- `src/standards/partial_eq_eq_clone_standard.rs` — PartialEq workaround pattern
- `src/standards/using_rand_standard.rs` — Random number patterns (for coin flips)

## Approach

1. Start with BoruvkaStEph PartialEq — likely quick fix with standard pattern.
2. Read SpanTreeStEph function bodies — understand the star contraction algorithm.
3. Read TSPApproxStEph function bodies — understand the Euler tour DFS.
4. For each function, remove `external_body`, fix verification errors iteratively.
5. For float-related holes (mst_weight), check available axioms and attempt.

## Validation

Run `scripts/validate.sh` after each file change. Run `scripts/rtt.sh` and `scripts/ptt.sh`
before committing. Push to `agent2/ready`.

## Report

Write `plans/agent2-round75-report.md` with holes before/after per file (table with Chap column).
