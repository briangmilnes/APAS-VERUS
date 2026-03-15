# Review Against Prose: Chapter 63 — Graph Connectivity

Date: 2026-03-15
Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

Source: `src/Chap63/analyses/veracity-review-module-fn-impls.md`

| # | Chap | File | Exec Fns | Spec Fns | Holes |
|---|------|------|----------|----------|-------|
| 1 | 63 | ConnectivityStEph.rs | 4 (+1 helper) | 1 | 0 |
| 2 | 63 | ConnectivityMtEph.rs | 4 (+3 helpers) | 1 | 0 |

Total: 8 trait-level exec fns, 4 helper fns, 2 wf spec fns, 0 holes.

## Phase 2: Prose Inventory

Source: `prompts/Chap63.txt`

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 63 | Defn 63.1: Graph Connectivity Problem | Definition |
| 2 | 63 | Alg 63.2: countComponents | Algorithm |
| 3 | 63 | Alg 63.3: connectedComponents | Algorithm |
| 4 | 63 | Ex 63.1: countComponents via starContract | Exercise |
| 5 | 63 | Ex 63.2: connectedComponents via starContract | Exercise |
| 6 | 63 | Ex 63.3: Work/span of count components | Exercise (text) |
| 7 | 63 | Ex 63.4: Work/span of connected components | Exercise (text) |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions already have APAS + Claude-Opus-4.6 cost annotations.

| # | Chap | File | Function | APAS Cost | Claude Assessment |
|---|------|------|----------|-----------|-------------------|
| 1 | 63 | ConnectivityStEph.rs | count_components | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 2 | 63 | ConnectivityStEph.rs | connected_components | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 3 | 63 | ConnectivityStEph.rs | count_components_hof | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 4 | 63 | ConnectivityStEph.rs | connected_components_hof | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 5 | 63 | ConnectivityStEph.rs | build_quotient_edges | N/A | W O(m), S O(m) |
| 6 | 63 | ConnectivityMtEph.rs | count_components_mt | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O(m) |
| 7 | 63 | ConnectivityMtEph.rs | connected_components_mt | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O(n lg n) |
| 8 | 63 | ConnectivityMtEph.rs | count_components_hof | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O(m) |
| 9 | 63 | ConnectivityMtEph.rs | connected_components_hof | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O(n lg n) |
| 10 | 63 | ConnectivityMtEph.rs | build_quotient_edges_parallel | N/A | W O(m), S O(m) |
| 11 | 63 | ConnectivityMtEph.rs | route_edges_parallel | N/A | W O(k), S O(k) |
| 12 | 63 | ConnectivityMtEph.rs | compose_maps_parallel | N/A | W O(P), S O(P) |

### 3b. Implementation Fidelity

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 63 | ConnectivityStEph.rs | count_components | Faithful to Alg 63.2. Recursive star contraction with base=|V| |
| 2 | 63 | ConnectivityStEph.rs | connected_components | Faithful to Alg 63.3. Line 10 composition C compose P |
| 3 | 63 | ConnectivityStEph.rs | count_components_hof | Faithful to Ex 63.1. Uses star_contract with base/expand |
| 4 | 63 | ConnectivityStEph.rs | connected_components_hof | Faithful to Ex 63.2. Uses star_contract with base/expand |
| 5 | 63 | ConnectivityMtEph.rs | count_components_mt | Parallel version; uses parallel_star_partition |
| 6 | 63 | ConnectivityMtEph.rs | connected_components_mt | Parallel version; compose_maps_parallel is sequential |
| 7 | 63 | ConnectivityMtEph.rs | count_components_hof | Uses star_contract_mt |
| 8 | 63 | ConnectivityMtEph.rs | connected_components_hof | Uses star_contract_mt |

### 3c. Spec Fidelity

All trait fns have `requires spec_wf(graph)` (graph well-formedness). No functional ensures (no postcondition guaranteeing correct component count or valid component mapping). The prose states correctness informally via induction on contraction rounds; these are text-proof properties not encoded as specs.

Spec strength: **weak** -- wf preconditions only, no postconditions on result correctness.

## Phase 4: Parallelism Review

| # | Chap | File | Function | Parallel? | Mechanism |
|---|------|------|----------|-----------|-----------|
| 1 | 63 | ConnectivityMtEph.rs | count_components_mt | Partial | Delegates to parallel_star_partition (which is sequential) + route_edges_parallel (ParaPair!) |
| 2 | 63 | ConnectivityMtEph.rs | connected_components_mt | Partial | Same as above + compose_maps_parallel (sequential) |
| 3 | 63 | ConnectivityMtEph.rs | route_edges_parallel | Yes | ParaPair! divide-and-conquer |
| 4 | 63 | ConnectivityMtEph.rs | build_quotient_edges_parallel | Yes | Delegates to route_edges_parallel |
| 5 | 63 | ConnectivityMtEph.rs | compose_maps_parallel | No | Sequential HashMap iteration despite name |
| 6 | 63 | ConnectivityMtEph.rs | count_components_hof | Partial | Delegates to star_contract_mt |
| 7 | 63 | ConnectivityMtEph.rs | connected_components_hof | Partial | Delegates to star_contract_mt |

Parallelism assessment: The Mt file uses ParaPair! for edge routing (quotient graph construction). However, `compose_maps_parallel` is sequential despite its name (line 224-229: sequential HashMap loop). The dependency on `parallel_star_partition` (from Chap62) being sequential is inherited. The prose's O(lg^2 n) span is not achieved.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Coverage |
|---|------|-----------|----------|
| 1 | 63 | TestConnectivityStEph.rs | count_components, connected_components, hof variants |
| 2 | 63 | TestConnectivityMtEph.rs | count_components_mt, connected_components_mt, hof variants |

Both source files have corresponding RTTs. Coverage is complete for trait-level functions.

## Phase 6: PTT Review

No PTTs exist for Chap63. None are needed.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 63 | No functional ensures | Medium | No correctness postcondition (count, component mapping) |
| 2 | 63 | compose_maps_parallel sequential | Medium | Named "parallel" but implemented sequentially |
| 3 | 63 | Inherits Chap62 partition sequentiality | Medium | parallel_star_partition is sequential; cascades here |
| 4 | 63 | route_edges_parallel merge sequential | Low | Set union after ParaPair! is O(k) |

## Phase 8: TOC Review

Both files follow the standard TOC structure. Trait definitions inside verus!; implementations outside (cfg(not(verus_keep_ghost))). Clean separation of sections.

## Summary

Chapter 63 implements both APAS algorithms (Alg 63.2 countComponents, Alg 63.3 connectedComponents) plus both exercises (Ex 63.1, 63.2 using starContract higher-order function), in both St and Mt variants. All 2 modules are **clean** (0 holes). The Mt variant achieves genuine parallelism in edge routing via ParaPair! but inherits the sequential partition bottleneck from Chap62 and has a sequential `compose_maps_parallel`. Cost annotations are present on all exec functions. Specs are structural (wf-only). RTT coverage is complete.
