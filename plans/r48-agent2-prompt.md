# R48 Agent 2: Graph Chapters (Chap59 + Chap62 + Chap65) — 5 holes

## Assignment

Close remaining holes in graph chapters. Agent 1 proved reweight_graph in R47
but bounced off the rest. Push harder — especially on sort_edges_by_weight
which should NOT need external_body.

## Baseline

38 holes total. 4419 verified. Your chapters: Chap59 (1) + Chap62 (2) + Chap65 (2) = 5 holes.

## REQUIRED READING

1. `src/standards/arc_usage_standard.rs`
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
3. `src/standards/using_closures_standard.rs`
4. `src/standards/partial_eq_eq_clone_standard.rs`

## Current Holes

Run `scripts/holes.sh` on each chapter to verify.

### Chap59 — 1 hole

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 1 | 59 | JohnsonMtEphI64.rs | 88 | parallel_dijkstra_all | external_body | Parallel Dijkstra over all vertices |

This runs Dijkstra from every vertex in parallel. Read the function body,
understand what it does, and see if you can remove external_body. The sequential
version works. The parallel version needs fork-join or tabulate over vertices.

Also fix the 2 fn_missing_requires warnings in JohnsonStEphI64.rs (adjust_distance
line 73, reweight_edge line 89). Add real preconditions, not `requires true`.

### Chap62 — 2 holes

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 2 | 62 | StarContractionStEph.rs | 72 | star_contract | external_body | Recursive contraction |
| 3 | 62 | StarContractionMtEph.rs | 72 | star_contract_mt | external_body | Parallel recursive contraction |

Both are recursive higher-order functions with `base` and `expand` closures.
Agent 1 analyzed the blockers: sequential/parallel_star_partition ensures are
too weak. Strengthen them:
- Centers subset of graph.V
- All partition_map values are centers
- All vertices covered
- Quotient graph is well-formed

Then the recursive body should verify. Read the St version first.

### Chap65 — 2 holes

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 4 | 65 | KruskalStEph.rs | 58 | sort_edges_by_weight | external_body | Edge sorting for Kruskal |
| 5 | 65 | PrimStEph.rs | 95 | prim_mst | external_body | Full Prim's algorithm |

**sort_edges_by_weight**: This should NOT be external_body. It calls Vec::sort_by
with a comparator. Options:
- Write the sort body using a verified sort (e.g., insertion sort on edges).
- If Vec::sort_by must stay, keep external_body but ensure the ensures are
  STRONG: output is sorted by weight, output is a permutation of input (multiset
  preserved), output length equals input length. The current ensures may already
  be strong — check them.
- If the ensures are already strong, this is an acceptable external_body and
  you should focus elsewhere.

**prim_mst**: Full Prim's algorithm with BinaryHeapPQ. Read the body. This may
need a loop invariant tracking: visited set grows monotonically, MST edges
connect visited to unvisited, PQ contains frontier edges. Substantial proof
but not impossible.

Fix the fn_missing_requires warning on pq_entry_new (line 72).

## What NOT to do
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT use Arc<RwLock> as struct field — read arc_usage_standard.rs.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap59/` + `src/Chap62/` + `src/Chap65/`.
Write your report to `plans/agent2-round48-report.md`.
