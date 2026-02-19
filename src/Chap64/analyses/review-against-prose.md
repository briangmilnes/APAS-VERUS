<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 64: Minimum Spanning Trees — Review Against Prose

**Date:** 2026-02-13 (updated 2026-02-18: verusification — traits inside verus!, impls cfg-gated)
**Last mechanical audit:** 2026-02-19 — proof holes log regenerated; 0 holes unchanged.
**Reviewer:** Claude-Opus-4.6
**Source:** `prompts/Chap64.txt` (APAS Part XVII, Chapter 64)

## Phase 1: Inventory (tool-generated)

Run: `veracity-review-module-fn-impls -d src/Chap64`

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 2 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 3 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 7 | 0 | 7 | 0 | 0 | 7 |

**Verusification status (2026-02-18):**
- **SpanTreeStEph:** Fully structured inside `verus!`. Trait definition (2 fns) is inside `verus!{}`. Both impl functions are cfg-gated behind `#[cfg(not(verus_keep_ghost))]`. V!=2, -V!=0.
- **SpanTreeMtEph:** Same structure. Trait definition (2 fns) is inside `verus!{}`. Both impl functions are cfg-gated. V!=2, -V!=0.
- **TSPApproxStEph:** The entire trait definition and all impl functions are behind `#[cfg(not(verus_keep_ghost))]` because the trait methods reference `HashMap` (a runtime-only type that Verus cannot parse in trait signatures). Only `use vstd::prelude::*;` is ungated. V!=0, -V!=7.
- All 11 functions still have no `requires`/`ensures` (spec strength: none).
- Zero proof holes (trivially — no proof obligations exist).
- `lib.rs` gate: Chap64 remains behind `#[cfg(feature = "all_chapters")]` (no change).

## Phase 2: Prose Inventory

### Definitions

| # | Name | Prose Reference | Implemented? |
|---|------|----------------|:------------:|
| 1 | Spanning Tree (Def 64.1) | A tree T = (V, E') with E' ⊆ E for connected undirected G | Implicit in `SpanTreeStEph` / `SpanTreeMtEph` |
| 2 | Minimum Spanning Tree (Def 64.2) | Min-weight spanning tree for weighted graph | Not implemented (Chap65 topic) |
| 3 | Graph Cut (Def 64.3) | Partition (U, V\U) with cut edges | Not implemented |
| 4 | Metric TSP (Def 64.4) | Complete undirected graph with non-negative weights satisfying triangle inequality | `TSPApproxStEph` |

### Algorithms

| # | Name | Prose Reference | Implemented? | Module |
|---|------|----------------|:------------:|--------|
| 1 | Spanning Tree via Star Contraction | Exercise 64.2 | Yes | `SpanTreeStEph`, `SpanTreeMtEph` |
| 2 | TSP 2-Approximation via MST | Section 4 | Yes | `TSPApproxStEph` |

### Cost Specs

| # | Algorithm | APAS Work | APAS Span | Notes |
|---|-----------|-----------|-----------|-------|
| 1 | Spanning Tree (star contraction) | O((n+m) lg n) | O(lg² n) parallel | Sequential span = Work |
| 2 | Euler Tour | O(n) | O(n) | DFS-based, inherently sequential |
| 3 | Shortcut Tour | O(n) | O(n) | Linear scan with HashSet |
| 4 | Tour Weight | O(n) | O(n) | Linear sum |
| 5 | TSP 2-Approx | O(n+m) | O(n+m) | Composition of euler_tour + shortcut + weight |

### Theorems/Properties

| # | Name | Prose Reference | Proved? |
|---|------|----------------|:-------:|
| 1 | Spanning Trees Edge Replacement (Lemma 64.1) | Swapping edge preserves spanning tree | No |
| 2 | MST Edge Replacement (Lemma 64.2) | Non-MST edge is heavier than path edges | No |
| 3 | Light-Edge Property (Lemma 64.3) | Min cut edge is in MST | No |
| 4 | Unique MST (Exercise 64.3) | Unique edge weights → unique MST | No |
| 5 | Heaviest cycle edge not in MST (Exercise 64.4) | Max-weight edge on any cycle is not in MST | No |
| 6 | TSP 2-approximation bound | W(MST) ≤ W(TSP) ≤ 2·W(MST) | Not formally proved |

### Exercises/Problems

| # | Exercise | Description | Implemented? |
|---|----------|-------------|:------------:|
| 1 | Exercise 64.1 | Spanning tree has |V|-1 edges | Checked in `verify_spanning_tree` |
| 2 | Exercise 64.2 | Spanning tree via star contraction | Yes |
| 3 | Exercise 64.3 | Unique MST for unique edge weights | No |
| 4 | Exercise 64.4 | Heaviest cycle edge not in MST | No |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions now have paired APAS / Claude-Opus-4.6 cost annotations. Summary of disagreements:

| # | Function | APAS | Claude-Opus-4.6 | Agrees? | Reason |
|---|----------|------|-----------------|:-------:|--------|
| 1 | `spanning_tree_star_contraction` (St) | O((n+m) lg n) / O((n+m) lg n) | O((n+m) lg n) / O((n+m) lg n) | Yes | — |
| 2 | `spanning_tree_star_contraction_mt` (Mt) | O((n+m) lg n) / O(lg² n) | O((n+m) lg n) / O((n+m) lg n) | **No** | Expand closure inner loop scanning original_edges is sequential O(E). 2-way thread::spawn splits do not achieve polylog span. |
| 3 | `verify_spanning_tree` (St) | N/A scaffolding | O(V + E_tree) / O(V + E_tree) | — | No prose counterpart |
| 4 | `verify_spanning_tree` (Mt) | N/A scaffolding | O(V + E_tree) / O(E_tree) | — | 2-way split halves work but span is O(E_tree/2), not O(lg V) |
| 5 | `euler_tour` | O(n) / O(n) | O(n) / O(n) | Yes | — |
| 6 | `euler_tour_dfs` | N/A helper | O(n * m_tree) / O(n * m_tree) | — | Linear scan of tree_edges per vertex per neighbor |
| 7 | `shortcut_tour` | O(n) / O(n) | O(n) / O(n) | Yes | — |
| 8 | `tour_weight` | O(n) / O(n) | O(n) / O(n) | Yes | — |
| 9 | `get_neighbors` (TSP) | N/A helper | O(m) / O(m) | — | Linear scan over all edges |
| 10 | `get_edge_weight` (TSP) | N/A helper | O(m) / O(m) | — | Linear scan over all edges |
| 11 | `approx_metric_tsp` | O(n+m) / O(n+m) | O(n+m) / O(n+m) | Yes | — |

### 3b. Implementation Fidelity

| # | Function | Fidelity | Notes |
|---|----------|:--------:|-------|
| 1 | `spanning_tree_star_contraction` (St) | Faithful | Uses `star_contract` from Chap62 with base/expand closures, matching the prose. |
| 2 | `spanning_tree_star_contraction_mt` (Mt) | Partial | Uses `star_contract_mt` from Chap62 but parallelism within expand closure is limited to 2-way splits (not recursive fork-join). Inner loop scanning original_edges is sequential. |
| 3 | `euler_tour` | Faithful | DFS-based Euler tour as described in Section 4. Visits each edge twice. |
| 4 | `shortcut_tour` | Faithful | Filters duplicate visits using HashSet, appends start vertex to close the cycle. |
| 5 | `tour_weight` | Faithful | Sums consecutive edge weights. |
| 6 | `approx_metric_tsp` | Faithful | Composition: euler_tour → shortcut_tour → tour_weight, matching the prose 3-step approach. |

**Deviation notes:**
- `euler_tour_dfs` scans all edges via `get_neighbors` (O(m) per call) rather than using an adjacency list. For a tree with n-1 edges this is O(n) per vertex × O(n) calls = O(n²) in the worst case, though the prose assumes O(n) total via adjacency list access.
- `get_neighbors` and `get_edge_weight` both do linear scans over all labeled edges O(m), where an adjacency representation would give O(degree).

### 3c. Spec Fidelity

**2026-02-18 update:** Trait definitions for `SpanTreeStEphTrait` and `SpanTreeMtEphTrait` are now inside `verus!{}` blocks. However, the trait method signatures still have **no `requires`/`ensures` clauses** — they are bare signatures with doc comments only. All impl functions remain outside `verus!` behind `#[cfg(not(verus_keep_ghost))]`.

**TSPApproxStEph special case:** The entire `TSPApproxStEphTrait` trait definition and all impl functions are behind `#[cfg(not(verus_keep_ghost))]` because the trait methods reference `HashMap` (a runtime type that Verus cannot parse in trait method signatures). Only `use vstd::prelude::*;` is ungated. This module has zero `verus!` content.

Spec fidelity cannot be assessed — the prose's correctness claims (spanning tree validity, 2-approximation guarantee) are entirely untested at the specification level.

The prose states:
- A spanning tree has |V|-1 edges (Exercise 64.1) — checked at runtime in `verify_spanning_tree` but not formally specified.
- The shortcut tour visits each vertex exactly once — not specified.
- W(TSP_approx) ≤ 2·W(MST) — not specified or proved.

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | Function | Classification | Evidence |
|---|----------|:--------------:|---------|
| 1 | `spanning_tree_star_contraction_mt` | Parallel | Uses `star_contract_mt` (Chap62) which internally uses parallel star partition. Expand closure uses `thread::spawn` for 2-way splits. |
| 2 | `verify_spanning_tree` (Mt) | Parallel | 2-way `thread::spawn` split of edge vector for parallel verification. |

### 4b. Span Audit

| # | Function | APAS Span | Claude-Opus-4.6 Span | Match? | Notes |
|---|----------|-----------|---------------------|:------:|-------|
| 1 | `spanning_tree_star_contraction_mt` | O(lg² n) | O((n+m) lg n) | **No** | Expand closure inner loop (scanning original_edges for each quotient edge) is sequential O(E). The 2-way partition-vec splits do not recursively subdivide to polylog depth. APAS span is aspirational. |
| 2 | `verify_spanning_tree` (Mt) | O(lg V) | O(E_tree) | **No** | Single 2-way split gives span O(E_tree/2), not O(lg V). Would need recursive parallel reduce for polylog span. |

### 4c. Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|:---------:|-------|
| 1 | `spanning_tree_star_contraction_mt` | O(lg² n) | O((n+m) lg n) | Yes (shallow) | 2-way thread::spawn, not recursive fork-join. Expand closure has sequential inner loop. |
| 2 | `verify_spanning_tree` (Mt) | O(lg V) | O(E_tree) | Yes (shallow) | Single 2-way split. Needs recursive parallel reduce. |

## Phase 5: Runtime Test Review

### 5a. Coverage Check

**No runtime tests exist for Chapter 64.** No files matching `tests/*Chap64*`, `tests/*span_tree*`, or `tests/*tsp*` were found.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | `SpanTreeStEph` | — | **Missing** |
| 2 | `SpanTreeMtEph` | — | **Missing** |
| 3 | `TSPApproxStEph` | — | **Missing** |

### 5b. Test Quality

No tests to assess.

### 5c. Missing Tests (Proposed)

| # | Priority | Test | Rationale |
|---|:--------:|------|-----------|
| 1 | High | `test_spanning_tree_star_contraction` — build a small known graph, compute spanning tree, verify |V|-1 edges, all edges from original graph, connectivity | No formal specs; runtime test is the only correctness evidence |
| 2 | High | `test_spanning_tree_mt` — same as above for Mt variant | Parallelism correctness needs runtime validation |
| 3 | High | `test_euler_tour` — build a tree, compute Euler tour, verify each edge traversed exactly twice | Core TSP subroutine |
| 4 | High | `test_shortcut_tour` — verify each vertex appears exactly once (except start/end) | Correctness of Hamiltonian cycle conversion |
| 5 | Medium | `test_approx_metric_tsp` — build a complete metric graph, verify tour weight ≤ 2x MST weight | 2-approximation guarantee |
| 6 | Medium | `test_verify_spanning_tree` — test with valid and invalid spanning trees | Scaffolding correctness |

## Phase 6: Proof-Time Test (PTT) Review

Chapter 64 has **no verified proof obligations** — the `verus!` blocks contain only trait definitions with no `requires`/`ensures`. **No PTTs are needed.**

### 6a. Unified Test Inventory Table

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | `SpanTreeStEph` | — | — | Missing RTT (no PTT needed) |
| 2 | `SpanTreeMtEph` | — | — | Missing RTT (no PTT needed) |
| 3 | `TSPApproxStEph` | — | — | Missing RTT (no PTT needed) |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Description | Notes |
|---|-----------|-------------|-------|
| 1 | Definition 64.1 (Spanning Tree) | Formal definition of spanning tree | Implicit in code but no formal type/spec |
| 2 | Definition 64.2 (MST) | Minimum spanning tree problem | Implemented in Chap65 (Kruskal, Prim) |
| 3 | Definition 64.3 (Graph Cut) | Cut (U, V\U) and cut edges | Not implemented |
| 4 | Lemma 64.1 (Edge Replacement) | Swapping edges preserves spanning tree | No proof |
| 5 | Lemma 64.2 (MST Edge Replacement) | Non-MST edge heavier than path edges | No proof |
| 6 | Lemma 64.3 (Light-Edge Property) | Min cut edge is in MST | No proof |
| 7 | Exercise 64.3 | Unique MST for unique edge weights | Not implemented |
| 8 | Exercise 64.4 | Heaviest cycle edge not in MST | Not implemented |
| 9 | Sequential spanning tree via DFS/BFS | Prose mentions DFS-tree and BFS-tree as spanning trees | Not implemented (star contraction used instead) |

### Code With No Prose Counterpart

| # | Function | Module | Notes |
|---|----------|--------|-------|
| 1 | `verify_spanning_tree` | SpanTreeStEph, SpanTreeMtEph | Scaffolding — runtime validation of spanning tree properties |
| 2 | `get_neighbors` | TSPApproxStEph | Internal helper for adjacency traversal |
| 3 | `get_edge_weight` | TSPApproxStEph | Internal helper for edge weight lookup |
| 4 | `euler_tour_dfs` | TSPApproxStEph | Internal helper — recursive DFS for Euler tour |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present? | Section Headers? | Notes |
|---|------|:------------:|:----------------:|-------|
| 1 | `SpanTreeStEph.rs` | No | No | Has `verus!` block with trait; TOC standard applies minimally |
| 2 | `SpanTreeMtEph.rs` | No | No | Has `verus!` block with trait; TOC standard applies minimally |
| 3 | `TSPApproxStEph.rs` | No | No | No `verus!` block (all code cfg-gated); TOC standard not applicable |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | `SpanTreeStEph.rs` | - | - | - | - | - | - | - | - | Trait in verus!, 2 impls cfg-gated |
| 2 | `SpanTreeMtEph.rs` | - | - | - | - | - | - | - | - | Trait in verus!, 2 impls cfg-gated |
| 3 | `TSPApproxStEph.rs` | - | - | - | - | - | - | - | - | Entire trait + 7 impls cfg-gated (HashMap in trait sigs) |

No derive impls in any Chap64 file. No action items.

## Proof Holes Summary

**Last verified:** 2026-02-18 (`veracity-review-proof-holes`)

```
✓ SpanTreeMtEph.rs
✓ SpanTreeStEph.rs
✓ TSPApproxStEph.rs

Modules: 3 clean, 0 holed
Holes Found: 0
```

**0 proof holes** — trivially clean because no proof obligations exist. The `verus!` blocks contain only trait definitions. SpanTreeMtEph.rs, SpanTreeStEph.rs, and TSPApproxStEph.rs were all updated 2026-02-18; hole count remains 0.

## Spec Strength Summary

| Classification | Count |
|:--------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 11 |

**All 11 functions have no formal specification** (no `requires`/`ensures`). The SpanTree trait signatures inside `verus!` are bare (no contracts). The TSPApprox trait is entirely outside `verus!`.

## Overall Assessment

Chapter 64 implements the two main algorithms from the prose:

1. **Spanning tree via star contraction** (Exercise 64.2) — both sequential and parallel variants, delegating to Chap62's `star_contract` / `star_contract_mt`.
2. **TSP 2-approximation via MST** (Section 4) — Euler tour, shortcut, and weight computation.

**Verusification status (2026-02-18):** Partially verusified. SpanTree modules have trait definitions inside `verus!{}` blocks (V!=2, -V!=0 for both St and Mt). TSPApproxStEph remains entirely outside `verus!` because its trait methods reference `HashMap` (V!=0, -V!=7). All impl functions across all three modules are cfg-gated behind `#[cfg(not(verus_keep_ghost))]`.

**Strengths:**
- Both prose algorithms are implemented and structurally faithful.
- Cost annotations are complete and paired (APAS + Claude-Opus-4.6).
- No proof holes.
- SpanTree traits are now inside `verus!`, establishing the foundation for future spec work.

**Weaknesses:**

| # | Severity | Issue |
|---|:--------:|-------|
| 1 | High | **No formal specs.** Trait signatures inside `verus!` have no `requires`/`ensures`. Zero contracts. |
| 2 | High | **No runtime tests.** No RTT files exist for any module. |
| 3 | High | **TSPApprox entirely outside verus!.** The `HashMap` dependency in trait signatures prevents verusification without type refactoring. |
| 4 | Medium | **All impl code outside verus!.** The 11 exec functions are behind `#[cfg(not(verus_keep_ghost))]` — they compile and run but are not verified. |
| 5 | Medium | **Parallelism is shallow.** The Mt spanning tree uses 2-way `thread::spawn` splits rather than recursive fork-join, so span equals work rather than achieving polylog. |
| 6 | Medium | **Helper functions use linear scans** (`get_neighbors`, `get_edge_weight`) where adjacency-list lookups would be more efficient, causing `euler_tour_dfs` to be O(n^2) rather than the prose's O(n). |
| 7 | Low | **Five lemmas/exercises from the prose are unimplemented** (Lemmas 64.1-64.3, Exercises 64.3-64.4). |
| 8 | Low | **No TOC headers** in any file. |

**Priority action items:**
1. Add runtime tests (RTTs) for all three modules — highest priority since there is zero test coverage.
2. Add `requires`/`ensures` specs to SpanTree trait methods inside `verus!`.
3. Consider refactoring TSPApprox trait to avoid `HashMap` in signatures, enabling verusification.
4. Improve Mt parallelism in `SpanTreeMtEph` — replace 2-way splits with recursive parallel reduce.
5. Replace linear-scan helpers with adjacency-list based lookups to match prose cost bounds.
