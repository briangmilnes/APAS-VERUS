# Agent 2 — Round 48 Report

## Assignment

Close 5 holes across Chap59, Chap62, and Chap65 (graph algorithms).

## Results

| # | Chap | File | Hole | Before | After | Status |
|---|------|------|------|--------|-------|--------|
| 1 | 59 | JohnsonMtEphI64.rs | external_body on parallel_dijkstra_all | 1 | 0 | Closed |
| 2 | 65 | KruskalStEph.rs | external_body on sort_edges_by_weight | 1 | 1 | Spec strengthened |
| 3 | 65 | PrimStEph.rs | external_body on prim_mst | 1 | 1 | Deferred |
| 4 | 62 | StarContractionStEph.rs | external_body on star_contract | 1 | 1 | Deferred |
| 5 | 62 | StarContractionMtEph.rs | external_body on star_contract_mt | 1 | 1 | Deferred |

**Holes closed: 1** (parallel_dijkstra_all).
**Specs strengthened: 1** (sort_edges_by_weight now has sorted-by-weight ensures via FloatTotalOrder).

## Verification

- **Verus**: 4419 verified, 0 errors
- **RTT**: 2613 tests passed, 0 skipped
- **PTT**: Not run (no new PTT-worthy changes)

## Hole Counts After

| # | Chap | Holes | Warnings | Clean Files |
|---|------|-------|----------|-------------|
| 1 | 59 | 0 | 2 fn_missing_requires | 3 of 4 (75%) |
| 2 | 62 | 2 external_body | 0 | 2 of 4 (50%) |
| 3 | 65 | 2 external_body | 1 fn_missing_requires | 1 of 3 (33%) |

## Techniques Used

### parallel_dijkstra_all (Chap59, closed)

Removed `external_body` from the recursive parallel Dijkstra function in JohnsonMtEphI64.rs.

1. **Edge count propagation**: Added `graph@.A.len() * 2 + 2 <= usize::MAX` to `reweight_graph` requires and `reweighted@.A.len() <= graph@.A.len()` to its ensures. Proved via injective mapping lemma (`lemma_no_duplicates_injective`) linking edge sequence length to graph arc set cardinality.
2. **Named closures for ParaPair!**: Converted inline closures to named variables `f1`/`f2` with explicit `requires`/`ensures` referencing captured variables directly (`graph_left@`, `potentials_left.seq@.len()`).
3. **Precondition chain**: Propagated the edge-count bound from `johnson_apsp` through `reweight_graph` into `parallel_dijkstra_all` and its recursive calls.

### sort_edges_by_weight (Chap65, spec strengthened)

Added sorted-by-weight ensures clause using `FloatTotalOrder::le`:
```
forall|i: int, j: int| #![trigger edges@[i], edges@[j]]
    0 <= i <= j < edges@.len() ==>
    edges@[i].2.val.le(edges@[j].2.val)
```
The `external_body` remains because Rust's `sort_by` with a closure is not verifiable by Verus, but the spec now fully captures the sorting postcondition (length preservation + permutation + sorted order).

## Deferred Work

### prim_mst (Chap65)

Removing `external_body` from `prim_mst` requires:
- BinaryHeapPQ invariant maintenance (`spec_is_exec_heap`, `obeys_feq_clone`)
- Visited-set tracking through the main loop
- Inner neighbor-iteration loop with PQ insert invariants
- Priority queue multiset reasoning for decrease-key semantics

This is a substantial verification effort — the function body is ~80 lines with nested loops and complex PQ interactions.

### star_contract / star_contract_mt (Chap62)

Removing `external_body` requires a dependency chain:
1. Strengthen `sequential_star_partition` ensures (currently `ensures true`) to guarantee the partition has meaningful properties (e.g., at least one edge contracts)
2. Prove `build_quotient_graph` produces a well-formed graph with fewer edges than the input
3. Prove termination: each recursive step reduces edge count
4. Mt version mirrors the St version and needs the same chain

## fn_missing_requires Warnings (Not Actionable)

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 59 | JohnsonStEphI64.rs | adjust_distance | Pure arithmetic, no precondition |
| 2 | 59 | JohnsonStEphI64.rs | reweight_edge | Pure arithmetic, no precondition |
| 3 | 65 | PrimStEph.rs | pq_entry_new | Struct constructor, no precondition |

Per CLAUDE.md rules: cannot add `requires true` or `// veracity: no_requires`. These are genuinely precondition-free functions; left as-is for user review.
