# Agent 1 — Round 46 Report

## Summary

Closed 4 external_body holes across Chap61, Chap62, and Chap64.
Baseline: 69 holes, 4396 verified. Result: **65 holes, 4396 verified, 0 errors.**

## Holes Before/After

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 61 | EdgeContractionMtEph.rs | 1 | 0 | -1 | ParaPair named closures + union() |
| 2 | 62 | StarContractionMtEph.rs | 2 | 1 | -1 | ParaPair named closures + Arc |
| 3 | 62 | StarContractionStEph.rs | 1 | 1 | 0 | Blocked: spec_graphview_wf for quotient |
| 4 | 64 | TSPApproxStEph.rs | 2 | 0 | -2 | Fuel-based DFS + while loops |
| 5 | 65 | KruskalStEph.rs | 1 | 1 | 0 | Blocked: Vec::sort_by unverifiable |
| 6 | 65 | PrimStEph.rs | 1 | 1 | 0 | Blocked: complex PQ loop invariants |

## Chapters Closed

- **Chap61**: 1 → 0 holes. All 4 modules clean.
- **Chap64**: 2 → 0 actionable holes (3 fn_missing_ensures warnings remain).

## Verification Counts

- 4396 verified, 0 errors
- 2613 RTT passed, 0 failed
- 65 total holes (was 69)

## Techniques Used

### ParaPair Named Closures (Chap61, Chap62)
Replaced inline `ParaPair!(move || ..., move || ...)` with named closures
having explicit `requires`/`ensures`. Used `Arc` for shared data, `union()`
for merging SetStEph results. Pattern:
```rust
let f1 = move || -> (r: SetStEph<Edge<V>>)
    requires start <= mid, (mid as nat) <= (*edges1)@.len(), valid_key_type_Edge::<V>(),
    ensures r.spec_setsteph_wf(),
{ build_edges_parallel(edges1, map1, start, mid) };
let Pair(left, right) = ParaPair!(f1, f2);
left.union(&right)
```

### Fuel-Based DFS (Chap64)
Added `fuel: usize` parameter with `decreases fuel` to `euler_tour_dfs`.
Converted `for` loops to `while` loops over `.elements.to_vec()`.
Top-level `euler_tour` passes `tree_edges.elements.len()` as fuel.

### Direct Field Access (explored for Chap62)
Used `graph.E.elements.len()` to bypass `sizeE()` which requires
`spec_graphview_wf`. Not needed in final code since star_contract
reverted to external_body.

## Remaining Holes (Assigned Chapters)

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 62 | StarContractionMtEph.rs | star_contract_mt | Closure data-dependent requires |
| 2 | 62 | StarContractionStEph.rs | star_contract | spec_graphview_wf for quotient |
| 3 | 65 | KruskalStEph.rs | sort_edges_by_weight | Vec::sort_by unverifiable |
| 4 | 65 | PrimStEph.rs | prim_mst | BinaryHeapPQ loop invariants |

### Why star_contract Can't Be Closed
Both star_contract variants are higher-order recursive functions taking
`Fn` closures. The callers (ConnectivityMtEph, SpanTreeMtEph) pass
closures with non-trivial `requires` (e.g., `vertices.spec_setsteph_wf()`).
A simple `forall|v| base.requires((v,))` in the function's requires would
demand the closure accept ALL inputs, but these closures are selective.
Additionally, the StEph version needs `spec_graphview_wf(quotient_graph@)`
at the recursive call, which can't be proven since `build_quotient_graph`
has `ensures true`.

### Why sort_edges_by_weight Can't Be Closed
Uses `Vec::sort_by` with a closure comparator. Neither `Vec::sort_by`
nor closure-based comparators are verifiable in vstd/Verus. The function
already has meaningful ensures (length + permutation).

### Why prim_mst Can't Be Closed
The while loop calls `BinaryHeapPQ::delete_min` and `::insert` which
require `obeys_feq_clone::<PQEntry<V>>()`, `spec_is_exec_heap(pq.spec_seq())`,
and PQ length bounds. Maintaining these as loop invariants through a PQ
loop with `continue`, `break`, and nested neighbor iteration is very complex
for `ensures true`.
