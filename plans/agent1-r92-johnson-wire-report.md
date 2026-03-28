# Agent 1 R92 Report: Wire JohnsonF64 to DijkstraF64

## Objective

Remove 2 `external_body` from JohnsonStEphF64 and JohnsonMtEphF64 by wiring
them to call the now-available DijkstraStEphF64.

## Results

| # | Chap | File | external_body removed | Status |
|---|------|------|-----------------------|--------|
| 1 | 59 | JohnsonStEphF64.rs | `johnson_apsp` | Verified |
| 2 | 59 | JohnsonMtEphF64.rs | `johnson_apsp` | Verified |

## Changes

### JohnsonStEphF64.rs (Chap59)
- Removed `#[verifier::external_body]` from `johnson_apsp`.
- Added import for `DijkstraStEphF64::dijkstra` and `NO_PREDECESSOR`.
- Wrote full Phase 1/2/3 body following the I64 pattern:
  - Phase 1: Bellman-Ford on augmented graph, extract potentials.
  - Phase 2: Reweight graph.
  - Phase 3: Sequential Dijkstra loop from each vertex, adjust distances.
- Added edge count ensures (`reweighted@.A.len() <= graph@.A.len()`) to
  `reweight_graph` with proof block (injective mapping via `map_values`).

### JohnsonMtEphF64.rs (Chap59)
- Removed `#[verifier::external_body]` from `johnson_apsp`.
- Added import for `DijkstraStEphF64::dijkstra`.
- Wrote full Phase 1/2/3 body following the MtEphI64 pattern:
  - Phase 1: Bellman-Ford on augmented graph.
  - Phase 2: Reweight graph.
  - Phase 3: Parallel Dijkstra via `parallel_dijkstra_all` (ParaPair! divide-and-conquer).
- Added `parallel_dijkstra_all` function with recursive binary split, ParaPair!
  fork-join, and ArraySeqStEphS::append for combining results.
- Added edge count ensures to `reweight_graph` with proof block.

### WeightedDirGraphStEphF64.rs (Chap06)
- Added `g@.A =~= edges@` to `from_weighed_edges` ensures (matching I128 variant).
- Added loop invariant tracking edge_set/edges correspondence.
- Added proof block establishing `edge_set@ =~= edges@` (same pattern as I128).

## Verification

- Full validate: **5368 verified, 0 errors**
- RTT: **3083 passed**
- PTT: **157 passed**
- Chap59 holes: **0**

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 59 | JohnsonStEphF64.rs | 1 | 0 | -1 |
| 2 | 59 | JohnsonMtEphF64.rs | 1 | 0 | -1 |

## Techniques

- Direct field access (`*bf_result.distances.nth(i)`) to avoid SSSPResultStEphF64's
  `get_distance` wf precondition that BellmanFord's ensures don't provide.
- Injective map_values proof for edge count bounds through `from_weighed_edges`.
- Strengthened WeightedDirGraphStEphF64::from_weighed_edges ensures to match I128.
