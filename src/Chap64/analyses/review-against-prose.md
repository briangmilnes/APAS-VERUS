# Review Against Prose: Chapter 64 — Minimum Spanning Trees (Introduction, Spanning Tree, TSP Approx)

Date: 2026-03-15
Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

Source: `src/Chap64/analyses/veracity-review-module-fn-impls.md`

| # | Chap | File | Exec Fns | Spec Fns | Holes |
|---|------|------|----------|----------|-------|
| 1 | 64 | SpanTreeStEph.rs | 2 | 1 | 0 |
| 2 | 64 | SpanTreeMtEph.rs | 2 (+1 Arc helper) | 1 | 0 |
| 3 | 64 | TSPApproxStEph.rs | 4 (+3 helpers) | 1 | 0 |

Total: 8 trait-level exec fns, 4 helper fns, 3 wf spec fns, 0 holes.

## Phase 2: Prose Inventory

Source: `prompts/Chap64.txt`

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 64 | Defn 64.1: Spanning Tree | Definition |
| 2 | 64 | Lemma 64.1: Spanning Trees Edge Replacement | Lemma |
| 3 | 64 | Defn 64.2: MST | Definition |
| 4 | 64 | Lemma 64.2: MST Edge Replacement | Lemma |
| 5 | 64 | Defn 64.3: Graph Cut | Definition |
| 6 | 64 | Lemma 64.3: Light-Edge Property | Lemma |
| 7 | 64 | Defn 64.4: Metric TSP | Definition |
| 8 | 64 | Ex 64.1: Spanning tree edge count | Exercise (text) |
| 9 | 64 | Ex 64.2: Spanning tree via star contraction | Exercise (algorithm) |
| 10 | 64 | Ex 64.3: Unique MST | Exercise (text) |
| 11 | 64 | Ex 64.4: Heaviest cycle edge not in MST | Exercise (text) |
| 12 | 64 | Section 4: TSP 2-approximation via MST | Algorithm |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions already have APAS + Claude-Opus-4.6 cost annotations.

| # | Chap | File | Function | APAS Cost | Claude Assessment |
|---|------|------|----------|-----------|-------------------|
| 1 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 2 | 64 | SpanTreeStEph.rs | verify_spanning_tree | N/A (scaffolding) | W O(V + E_tree), S O(V + E_tree) |
| 3 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O((n+m) lg n) |
| 4 | 64 | SpanTreeMtEph.rs | verify_spanning_tree | N/A (scaffolding) | W O(V + E_tree), S O(E_tree) |
| 5 | 64 | TSPApproxStEph.rs | euler_tour | W O(n), S O(n) | Agrees |
| 6 | 64 | TSPApproxStEph.rs | euler_tour_dfs | N/A (helper) | W O(n * m_tree), S O(n * m_tree) |
| 7 | 64 | TSPApproxStEph.rs | shortcut_tour | W O(n), S O(n) | Agrees |
| 8 | 64 | TSPApproxStEph.rs | tour_weight | W O(n), S O(n) | Agrees |
| 9 | 64 | TSPApproxStEph.rs | approx_metric_tsp | W O(n+m), S O(n+m) | Agrees |
| 10 | 64 | TSPApproxStEph.rs | get_neighbors | N/A (helper) | W O(m), S O(m) |
| 11 | 64 | TSPApproxStEph.rs | get_edge_weight | N/A (helper) | W O(m), S O(m) |

### 3b. Implementation Fidelity

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction | Faithful to Ex 64.2; uses star_contract with base/expand |
| 2 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt | Parallel version; expand closure uses 2-way join() split |
| 3 | 64 | TSPApproxStEph.rs | euler_tour | DFS-based Euler tour per prose Section 4 |
| 4 | 64 | TSPApproxStEph.rs | shortcut_tour | Faithful to prose shortcut via visited set |
| 5 | 64 | TSPApproxStEph.rs | approx_metric_tsp | Faithful to prose: MST -> Euler tour -> shortcut |

### 3c. Spec Fidelity

SpanTree trait fns: `requires spec_wf(graph)` only. No postcondition on spanning tree validity (e.g., |E_tree| = |V|-1, tree connects all vertices, edges subset of graph edges). The verify_spanning_tree function is a runtime checker, not a spec-level guarantee.

TSPApprox trait fns: `requires spec_wf(graph)` only. No postcondition on tour validity (Hamiltonian cycle) or weight bound (at most 2*MST weight). The 2-approximation guarantee from the prose (W(MST) <= W(TSP) <= 2*W(MST)) is not encoded as a spec.

Spec strength: **weak** -- wf preconditions only, no functional postconditions.

## Phase 4: Parallelism Review

| # | Chap | File | Function | Parallel? | Mechanism |
|---|------|------|----------|-----------|-----------|
| 1 | 64 | SpanTreeMtEph.rs | spanning_tree_star_contraction_mt | Partial | Expand closure uses join() for partition edges and quotient edges |
| 2 | 64 | SpanTreeMtEph.rs | verify_spanning_tree | Partial | 2-way split with join() |

Parallelism assessment: The Mt spanning tree uses `join()` from HFScheduler for two phases of the expand closure: (1) adding partition edges from left/right halves, and (2) mapping quotient edges back from left/right halves. However, within each half, the inner loops scanning original_edges for each quotient edge are sequential O(E), so span is dominated by sequential work. The 2-way split provides some parallelism but does not achieve polylog span. `verify_spanning_tree` also uses a 2-way join() split. The RwLock on spanning_edges enables concurrent writes but each write acquires/releases the lock, creating contention.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Coverage |
|---|------|-----------|----------|
| 1 | 64 | TestSpanTreeStEph.rs | spanning_tree_star_contraction, verify_spanning_tree |
| 2 | 64 | TestSpanTreeMtEph.rs | spanning_tree_star_contraction_mt, verify_spanning_tree |
| 3 | 64 | TestTSPApproxStEph.rs | euler_tour, shortcut_tour, tour_weight, approx_metric_tsp |

All 3 source files have corresponding RTTs. Coverage is complete.

## Phase 6: PTT Review

No PTTs exist for Chap64. None are needed.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 64 | No spanning tree postcondition | Medium | No ensures on edge count, connectivity, or subset property |
| 2 | 64 | No TSP approximation bound spec | Medium | 2*MST weight bound not encoded |
| 3 | 64 | SpanTreeMtEph span dominated by sequential loops | Medium | Inner O(E) scans per quotient edge not parallelized |
| 4 | 64 | euler_tour_dfs scans tree_edges linearly | Low | O(m_tree) per vertex call instead of adjacency lookup |
| 5 | 64 | get_neighbors/get_edge_weight use flat edge scan | Low | O(m) per call; adjacency list would give O(degree) |
| 6 | 64 | No Mt variant for TSPApprox | Low | Only StEph exists; DFS-based Euler tour is inherently sequential |

## Phase 8: TOC Review

SpanTreeStEph.rs has `pub type T<V>` outside verus! (line 21), which is acceptable for type aliases. SpanTreeMtEph.rs has RwLockPredicate and Arc helper inside verus! (section 4/8), which is correct per the standard. TSPApproxStEph.rs follows the pattern with trait inside verus! and implementations outside. All files have clean TOC structure.

## Summary

Chapter 64 implements Exercise 64.2 (spanning tree via star contraction) in both St and Mt variants, plus the TSP 2-approximation algorithm from Section 4 (St only). All 3 modules are **clean** (0 holes). The SpanTreeMtEph variant uses `join()` from HFScheduler for genuine parallelism in the expand closure, though inner loops remain sequential. TSPApproxStEph implements the full pipeline: Euler tour, shortcut, weight computation. No Mt variant for TSP (DFS is sequential). Cost annotations are present on all exec functions. Specs are structural (wf-only), with no postconditions on spanning tree validity or TSP approximation bounds. RTT coverage is complete.
