# R75 Agent 1 — Prove Chap65 MST & Union-Find (11 holes)

## Objective

Prove or eliminate 11 holes across 3 files in Chap65 (MST algorithms and Union-Find).

## Files and holes

| # | Chap | File | Holes | Root causes |
|---|------|------|-------|-------------|
| 1 | 65 | UnionFindStEph.rs | 5 | 1 root (insert), 4 downstream |
| 2 | 65 | KruskalStEph.rs | 3 | 2 root (sort_edges_by_weight, mst_weight), 1 down |
| 3 | 65 | PrimStEph.rs | 3 | 2 root (cmp, mst_weight) + 4 accepted proof fn holes |

### UnionFindStEph.rs (5 holes — all external_body)

The single root cause is `insert()` (line ~219). All 4 others (`find`, `union`, `equals`,
`num_sets`) are downstream of `insert`. Fix `insert` first and the rest may cascade.

- `insert()` — external_body root cause
- `find()` — external_body downstream (blocked by insert)
- `union()` — external_body downstream (blocked by find)
- `equals()` — external_body downstream (blocked by find)
- `num_sets()` — external_body downstream (blocked by find)

**Strategy**: Read the current specs on the trait methods. The Union-Find data structure
tracks disjoint sets with path compression and union-by-rank. The `insert` function adds
a new singleton set. Read the function body inside the `external_body` to understand what
needs proving. The likely challenge is maintaining the representation invariant (parent
pointers, rank consistency, disjoint-set properties).

### KruskalStEph.rs (3 holes — all external_body)

- `sort_edges_by_weight()` — line ~64 — external_body root cause
- `kruskal_mst()` — line ~166 — external_body downstream (blocked by sort)
- `mst_weight()` — line ~286 — external_body root cause

**Strategy**: `sort_edges_by_weight` likely uses std sort which Verus can't verify. Consider
replacing with a verified sort (e.g., insertion sort from Chap03, or merge sort from Chap26)
or wrapping with appropriate specs. `mst_weight` likely sums edge weights — the challenge
is float arithmetic axioms. Check `src/vstdplus/float.rs` for available float axioms.

### PrimStEph.rs (3 actionable holes + 4 accepted proof fn holes)

- `cmp()` — line ~74 — external_body root cause (TotalOrder for float)
- `mst_weight()` — line ~335 — external_body root cause
- 4 accepted proof fn holes in TotalOrder (reflexive, transitive, antisymmetric, total) —
  these are axiomatized for float ordering. Leave the accepts in place unless you can prove
  them from `src/vstdplus/float.rs` axioms.

**Strategy**: The `cmp` function compares edge weights (floats). Check if `FloatTotalOrder`
from `src/vstdplus/float.rs` provides what's needed. `mst_weight` sums weights — same float
arithmetic challenge as Kruskal.

## Key resources

- `src/Chap65/UnionFindStEph.rs` — Union-Find implementation
- `src/Chap65/KruskalStEph.rs` — Kruskal's MST algorithm
- `src/Chap65/PrimStEph.rs` — Prim's MST algorithm
- `src/vstdplus/float.rs` — Float axioms and TotalOrder
- `src/Chap03/InsertionSortStEph.rs` — Verified sort (reference for replacing std sort)
- `src/Chap26/` — MergeSort implementations

## Approach

1. Start with UnionFindStEph — it has the most achievable root cause (1 root, 4 downstream).
2. Read the existing function bodies inside the `external_body` markers.
3. For each function, remove `external_body`, fix verification errors.
4. For float-related holes (Kruskal sort, mst_weight), check what axioms exist.
5. If float arithmetic proofs are blocked, document what's missing and move on.

## Validation

Run `scripts/validate.sh` after each file change. Run `scripts/rtt.sh` and `scripts/ptt.sh`
before committing. Push to `agent1/ready`.

## Report

Write `plans/agent1-round75-report.md` with holes before/after per file (table with Chap column).
