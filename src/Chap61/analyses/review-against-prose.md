# Review Against Prose: Chapter 61 — Edge Contraction

Date: 2026-03-15
Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

Source: `src/Chap61/analyses/veracity-review-module-fn-impls.md`

| # | Chap | File | Exec Fns | Spec Fns | Holes |
|---|------|------|----------|----------|-------|
| 1 | 61 | EdgeContractionStEph.rs | 2 | 1 | 0 |
| 2 | 61 | EdgeContractionMtEph.rs | 2 (+1 helper) | 1 | 0 |
| 3 | 61 | VertexMatchingStEph.rs | 2 | 1 | 0 |
| 4 | 61 | VertexMatchingMtEph.rs | 1 (+4 helpers) | 1 | 0 |

Total: 7 trait-level exec fns, 5 helper fns, 4 wf spec fns, 0 holes.

## Phase 2: Prose Inventory

Source: `prompts/Chap61.txt`

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 61 | Defn 61.1: Edge Partition | Definition |
| 2 | 61 | Defn 61.2: Vertex Matching | Definition |
| 3 | 61 | Alg 61.3: Greedy Vertex Matching | Algorithm |
| 4 | 61 | Alg 61.4: Parallel Vertex Matching | Algorithm |
| 5 | 61 | Alg 61.6: Parallel Edge Contraction | Algorithm |
| 6 | 61 | Defn 61.5: Star Graph | Definition |
| 7 | 61 | Ex 61.1-61.6 | Exercises (text proofs) |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions already have APAS + Claude-Opus-4.6 cost annotations.

| # | Chap | File | Function | APAS Cost | Claude Assessment |
|---|------|------|----------|-----------|-------------------|
| 1 | 61 | VertexMatchingStEph.rs | greedy_matching | W O(E), S O(E) | Agrees |
| 2 | 61 | VertexMatchingStEph.rs | parallel_matching_st | (no cost stated) | W O(E^2), S O(E^2) |
| 3 | 61 | VertexMatchingMtEph.rs | parallel_matching_mt | W O(E), S O(lg V) | W O(E^2), S O(E) |
| 4 | 61 | VertexMatchingMtEph.rs | flip_coins_parallel | W O(E), S O(1) | W O(E), S O(E) |
| 5 | 61 | VertexMatchingMtEph.rs | select_edges_parallel | W O(E), S O(lg V) | W O(E^2), S O(lg E + E) |
| 6 | 61 | VertexMatchingMtEph.rs | select_edges_recursive | N/A | W O(k*E), S O(lg k + E) |
| 7 | 61 | VertexMatchingMtEph.rs | should_select_edge | W O(deg), S O(deg) | W O(E), S O(E) |
| 8 | 61 | EdgeContractionStEph.rs | edge_contract | W O(V+E), S O(V+E) | Agrees |
| 9 | 61 | EdgeContractionStEph.rs | contract_round | W O(V+E), S O(V+E) | Agrees |
| 10 | 61 | EdgeContractionMtEph.rs | edge_contract_mt | W O(V+E), S O(lg V) | W O(V+E), S O(V+E) |
| 11 | 61 | EdgeContractionMtEph.rs | build_edges_parallel | N/A | W O(E), S O(lg E) |
| 12 | 61 | EdgeContractionMtEph.rs | contract_round_mt | W O(V+E), S O(lg V) | W O(E^2), S O(E) |

### 3b. Implementation Fidelity

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 61 | VertexMatchingStEph.rs | greedy_matching | Faithful to Alg 61.3 |
| 2 | 61 | VertexMatchingStEph.rs | parallel_matching_st | Sequential baseline of Alg 61.4; scans all edges for adjacency rather than using adjacency list |
| 3 | 61 | VertexMatchingMtEph.rs | parallel_matching_mt | Matches Alg 61.4 structure; Phase 1 (coin flip) sequential due to RNG |
| 4 | 61 | VertexMatchingMtEph.rs | should_select_edge | Scans all edges instead of using adjacency structure; O(E) vs O(degree) |
| 5 | 61 | EdgeContractionStEph.rs | edge_contract | Faithful to Alg 61.6 |
| 6 | 61 | EdgeContractionMtEph.rs | edge_contract_mt | Phases 1-2 sequential, Phase 3 parallel; matches Alg 61.6 intent |

### 3c. Spec Fidelity

All trait fns have `requires spec_wf(graph)` (graph well-formedness). No ensures clauses beyond return type. Specs are structural only (wf precondition, no postcondition about matching validity or graph structure preservation). The prose does not provide formal postconditions for these graph algorithms, so the weak specs are consistent with the textbook's informal treatment.

Spec strength: **weak** -- only wf preconditions, no functional ensures.

## Phase 4: Parallelism Review

| # | Chap | File | Function | Parallel? | Mechanism |
|---|------|------|----------|-----------|-----------|
| 1 | 61 | VertexMatchingMtEph.rs | parallel_matching_mt | Partial | Phase 1 sequential (RNG), Phase 2 ParaPair! |
| 2 | 61 | VertexMatchingMtEph.rs | select_edges_recursive | Yes | ParaPair! divide-and-conquer |
| 3 | 61 | EdgeContractionMtEph.rs | edge_contract_mt | Partial | Phases 1-2 sequential, Phase 3 ParaPair! |
| 4 | 61 | EdgeContractionMtEph.rs | build_edges_parallel | Yes | ParaPair! divide-and-conquer |

Parallelism assessment: Mt files use `ParaPair!` for genuine fork-join in edge-processing phases. However, coin flip (RNG) and vertex-to-block mapping phases are sequential. The `should_select_edge` function scans all edges (O(E)) rather than just incident edges, inflating work from O(E) to O(E^2). The prose's O(lg V) span is not achieved due to sequential RNG and non-adjacency-list representation.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Coverage |
|---|------|-----------|----------|
| 1 | 61 | TestEdgeContractionStEph.rs | edge_contract, contract_round |
| 2 | 61 | TestEdgeContractionMtEph.rs | edge_contract_mt, contract_round_mt |
| 3 | 61 | TestVertexMatchingStEph.rs | greedy_matching, parallel_matching_st |
| 4 | 61 | TestVertexMatchingMtEph.rs | parallel_matching_mt |

All 4 source files have corresponding RTTs. Coverage appears complete for trait-level functions.

## Phase 6: PTT Review

No PTTs exist for Chap61. None are needed (no complex requires patterns, no iterators).

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 61 | No functional ensures | Medium | No postcondition guarantees matching validity or graph structure |
| 2 | 61 | should_select_edge O(E) scan | Low | Uses edge-set representation; adjacency list would give O(degree) |
| 3 | 61 | Coin flip phase sequential | Low | Inherent to RNG; APAS also notes this |
| 4 | 61 | Exercises 61.1-61.6 not implemented | N/A | Text proof exercises, not algorithms |

## Phase 8: TOC Review

All files follow the standard TOC structure. EdgeContractionStEph.rs and VertexMatchingStEph.rs have sections 1, 2, 4, 8 inside verus!, with impl outside. EdgeContractionMtEph.rs and VertexMatchingMtEph.rs follow the same pattern. Trait definitions are inside verus!; implementations are outside (cfg(not(verus_keep_ghost))). This is the standard pattern for graph algorithm files where the algorithmic code uses HashMap and other non-Verus-compatible types.

## Summary

Chapter 61 implements all three APAS algorithms (Alg 61.3, 61.4, 61.6) in both St and Mt variants. All 4 modules are **clean** (0 holes). Cost annotations are present on all exec functions. The Mt variants use genuine ParaPair! parallelism for edge processing but have sequential phases for RNG and vertex mapping. Specs are structural (wf-only preconditions), matching the prose's informal treatment. RTT coverage is complete.
