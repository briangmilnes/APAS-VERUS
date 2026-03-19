# R42 Agent 3: Graph Algorithms — Chap65 Prim + Chap61 EdgeContraction

## Baseline
- Main after R41 merge, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

Chapters 59-66 are graph algorithms with 50 holes total. All depend on Chap05 (clean).
The holes are mostly external_body on algorithm implementations. We start with the
smallest chapters to build momentum.

## Assignment

### File 1: `src/Chap65/PrimStEph.rs` — 3 holes

Prim's MST algorithm. Read the file first to understand the structure.

| # | Method | Type | Notes |
|---|--------|------|-------|
| 1 | pq_entry (or similar) | external_body | Priority queue entry construction |
| 2 | prim | external_body | Main Prim's algorithm |
| 3 | weight (or similar) | external_body | Edge weight computation |

Read the file — the holes may be on helper functions or the main algorithm. Prim's
iteratively grows an MST by always adding the minimum-weight edge crossing the cut.

**Strategy**: The algorithm likely uses a priority queue (BinaryHeapPQ from Chap45,
which is clean). The proof needs to show: (1) the PQ entries are valid edges,
(2) the selected edge crosses the cut, (3) the result is a spanning tree.

If the holes are on infrastructure (PQ entry construction, weight extraction), those
should be straightforward delegations.

### File 2: `src/Chap61/EdgeContractionStEph.rs` — 2 holes

Read the files in Chap61 to find which have holes.

| # | File | Holes |
|---|------|-------|
| 1 | EdgeContractionStEph.rs (or similar) | 2 |
| 2 | VertexMatchingStEph.rs (or similar) | 2 |

Edge contraction merges two vertices connected by an edge, combining their neighborhoods.
Vertex matching finds a maximal matching (set of non-adjacent edges).

**Strategy**: These are graph transformation operations. The proofs likely need to show
that the contracted graph preserves certain properties (vertex count decreases,
connectivity preserved). Read the ensures clauses to understand what needs proving.

### File 3: Additional Chap59 or Chap64 (if time)

If Chap65 and Chap61 go quickly, start on:
- `src/Chap64/TSPApproxStEph.rs` — 7 holes (TSP approximation)
- Or `src/Chap59/JohnsonMtEphI64.rs` — 5 ext_body (parallel Johnson)

### Priority

1. Chap65 PrimStEph (3 holes) — smallest, cleanest target
2. Chap61 EdgeContraction + VertexMatching (4 holes)
3. Bonus chapter if time

### Expected Results

Conservative: 3-4 holes closed.
Optimistic: 5-7 holes closed. Close Chap65.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent3-r42-report.md`.
