# Agent 1 Round 47 Report: Graph Chapter Mop-up

## Summary

Assigned Chap59 (2 holes), Chap62 (2 holes), Chap65 (2 holes) = 6 holes.
Eliminated 1 hole. Total: 6 → 5 holes.

## Results by Chapter

### Chap59 — Johnson's APSP (2 → 1 holes, -1)

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 59 | JohnsonStEphI64.rs | reweight_graph | assume | PROVED | Refactored to iterate labeled_arcs(); proved edge count via view injectivity + unique_seq_to_set |
| 2 | 59 | JohnsonMtEphI64.rs | parallel_dijkstra_all | external_body | external_body | Thread boundary; needs obeys_feq_clone for nested ArraySeqStEphS |

**Warnings (unchanged, 2):** `adjust_distance` and `reweight_edge` (fn_missing_requires). Both are total arithmetic functions that handle all i64/i128 inputs via overflow clamping. No real precondition exists.

**Proof technique for reweight_graph:** The original code iterated per-vertex out_neighbors, requiring a "sum of out-degrees = |A|" lemma. Refactored to iterate `labeled_arcs()` directly (same pattern as MtEph version). The edge count proof chain:
1. `arcs_seq.no_duplicates()` — from SetStEph::iter() postcondition
2. View function on LabEdge<usize, i128> is injective — structural (identity view)
3. `arcs_seq.map_values(view).no_duplicates()` — via vstd `lemma_no_duplicates_injective`
4. `arcs_seq.len() == graph@.A.len()` — via vstd `unique_seq_to_set`
5. `edges@.len() <= arcs_seq.len()` — from loop invariant (each insert increases len by at most 1)
6. Therefore `edges@.len() <= graph@.A.len()` — chain of (4) and (5)

### Chap62 — Star Contraction (2 → 2 holes, unchanged)

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 3 | 62 | StarContractionStEph.rs | star_contract | external_body | external_body | Requires strengthening 3 functions |
| 4 | 62 | StarContractionMtEph.rs | star_contract_mt | external_body | external_body | Same dependency chain as StEph |

**Blocking analysis:** Both functions are recursive with closure parameters. Removing external_body requires:
1. `sequential_star_partition` / `parallel_star_partition` ensures strengthened to include: centers ⊆ graph.V, all partition_map values are centers, all vertices covered
2. `build_quotient_graph` / `build_quotient_graph_parallel` ensures strengthened to include: `spec_graphview_wf(quotient@)` (edge endpoints are in centers)
3. Closure spec propagation: `forall|s| base.requires((s,))` and universal expand requires

Estimated effort: 50-100 lines of new invariant code across 4 files (StarPartition + StarContraction, St + Mt). Not a simple mop-up — this is cross-function ensures infrastructure work.

### Chap65 — MST: Kruskal + Prim (2 → 2 holes, unchanged)

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 5 | 65 | KruskalStEph.rs | sort_edges_by_weight | external_body | external_body | Comparison sort; already has strong spec (length + element preservation) |
| 6 | 65 | PrimStEph.rs | prim_mst | external_body | external_body | Full PQ loop algorithm; needs obeys_feq_clone for PQEntry |

**Warning (unchanged, 1):** `pq_entry_new` (fn_missing_requires). Trivial constructor with no precondition.

**Blocking analysis:**
- `sort_edges_by_weight`: Uses `Vec::sort_by` with closure. Comparison-based sorting is fundamentally not verifiable in Verus (closure comparison not expressible in spec). The existing ensures (`len preserved`, `elements from original`) is the strongest achievable spec. Acceptable external_body.
- `prim_mst`: Full Prim's algorithm with BinaryHeapPQ loop. Requires: (a) `obeys_feq_clone::<PQEntry<V>>()` for PQ operations, (b) loop invariant tracking visited set, MST edges, PQ contents, (c) termination proof. Substantial proof effort.

## Verification Status

- **Verified:** 4412 (unchanged — no new functions, just restructured reweight_graph)
- **RTT:** 2613 tests, 2613 passed
- **Errors:** 0

## Hole Summary

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Chap59 holes | 2 | 1 | -1 |
| Chap62 holes | 2 | 2 | 0 |
| Chap65 holes | 2 | 2 | 0 |
| **Total assigned** | **6** | **5** | **-1** |
| Warnings | 3 | 3 | 0 |

## What Blocks the Remaining 5 Holes

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 59 | JohnsonMtEphI64.rs | parallel_dijkstra_all | obeys_feq_clone for nested ArraySeqStEphS; thread boundary |
| 2 | 62 | StarContractionStEph.rs | star_contract | sequential_star_partition ensures too weak; need centers/partition_map properties |
| 3 | 62 | StarContractionMtEph.rs | star_contract_mt | parallel_star_partition ensures too weak; same as StEph |
| 4 | 65 | KruskalStEph.rs | sort_edges_by_weight | Comparison sort not verifiable (closure-based sort_by) |
| 5 | 65 | PrimStEph.rs | prim_mst | obeys_feq_clone for PQEntry; full PQ loop invariant needed |
