<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 53 — Graph Search: Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

| # | File | exec fns | external_body | spec fns | proof fns | View | verus! |
|---|------|:--------:|:-------------:|:--------:|:---------:|:----:|:------:|
| 1 | PQMinStEph.rs | 4 | 4 | 0 | 0 | No | Yes |
| 2 | PQMinStPer.rs | 4 | 4 | 0 | 0 | No | Yes |
| 3 | GraphSearchStEph.rs | 5 | 5 | 0 | 0 | No | Yes |
| 4 | GraphSearchStPer.rs | 5 | 5 | 0 | 0 | No | Yes |
| 5 | GraphSearchMtPer.rs | 5 | 5 | 0 | 0 | No | Yes |
| | **Total** | **23** | **23** | **0** | **0** | | |

All 5 files are inside `verus!{}` blocks with type definitions, traits, and impls. All 23 exec functions have `#[verifier::external_body]`. No `requires`/`ensures` on any function. No spec fns or proof fns.

**Gating:** `#[cfg(all(not(any(feature = "experiments_only", feature = "dev_only")), not(verus_keep_ghost)))]` — skipped during Verus verification due to dependencies on Chap37 (AVLTreeSeq) and Chap41 (AVLTreeSet). `GraphSearchMtPer` additionally gated behind `feature = "all_chapters"`.

## Phase 2: Prose Inventory

Source: `prompts/Chap53.txt` (Chapter 53 — Graph Search)

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 53.1 (Source) | Graph search starts at source vertex s ∈ V or a set of source vertices. |
| 2 | Definition 53.2 (Visited Vertices) | Set X of already-visited vertices. |
| 3 | Definition 53.3 (Frontier / Discovered) | Frontier F = N⁺_G(X) \ X — unvisited out-neighbors of visited set. |
| 4 | Definition 53.5 (Reachability) | Vertex v reachable from u if path exists from u to v. |
| 5 | Definition 53.6 (Graph-Search Tree) | Rooted tree over visited vertices X ⊆ V with edges E' ⊆ E. |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 53.4 (Graph Search, Single Source) | Generic: `explore X F` choosing U ⊆ F, visiting U, X = X ∪ U, F = N⁺_G(X) \ X. |
| 2 | (Implicit) Multi-source | Exercise 53.3: extend to start from a set of sources. |
| 3 | (Implicit) BFS | SelectAll: select entire frontier each round. |
| 4 | (Implicit) DFS | SelectOne: select single most-recently-added vertex. |
| 5 | (Implicit) PFS | Priority-First Search: select by priority (Section 4). |

### Theorems / Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Theorem 53.1 | `graphSearch(G, s)` returns exactly reachable vertices from s, in ≤ \|V\| rounds. |

### Exercises / Problems

| # | Item | Description |
|---|------|-------------|
| 1 | Problem 53.2 | Graph Reachability: return all vertices reachable from v. |
| 2 | Exercise 53.3 | Multi-source graph search. |

### Cost Specs

The prose gives **no explicit Work/Span bounds** for Algorithm 53.4 or PFS. It states:
- "at most |V| rounds" (Theorem 53.1)
- BFS is parallel (visits whole frontier per round)
- DFS is sequential (one vertex per round)
- PFS cost depends on priority queue implementation (deferred to Dijkstra, Prim)

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 23 exec functions have APAS/Claude-Opus-4.6 cost annotation pairs.

| # | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|----------|-----------|---------------------|-----------|
| 1 | `graph_search` (St variants) | No explicit cost | Work Θ((n+m) log n), Span = Work | N/A |
| 2 | `graph_search_multi` (St variants) | No explicit cost | Work Θ((n+m) log n), Span = Work | N/A |
| 3 | `reachable` (St variants) | No explicit cost | Work Θ((n+m) log n), Span = Work | N/A |
| 4 | `graph_search` (MtPer) | No explicit cost | Work Θ((n+m) log n), Span = Work | N/A — neighbor loop sequential |
| 5 | `pq_min` / `pq_min_multi` | No explicit PFS cost | Work Θ(n² + m log n), Span = Work | N/A — `find_min_priority` uses `to_seq` O(|F|) per round |
| 6 | `SelectAll::select` | N/A — scaffolding | Work Θ(n), Span Θ(n) — clone | N/A |
| 7 | `SelectOne::select` | N/A — scaffolding | Work Θ(n), Span Θ(n) — to_seq | N/A |
| 8 | `ClosurePriority::new` | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |
| 9 | `ClosurePriority::priority` | N/A — scaffolding | Work Θ(1), Span Θ(1) | N/A |

**Key cost deviation in PQMin:** `find_min_priority` calls `to_seq().nth(0)` which materializes the entire sorted sequence to extract the minimum — O(|F|) per call. Over |V| rounds this contributes O(|V|²). A proper min-extraction on the AVL tree (O(log |V|)) would give the expected O((n + m) log n) for Dijkstra-style PFS.

### 3b. Implementation Fidelity

| # | Prose Item | Code Function | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Algorithm 53.4 | `graph_search_multi` / `explore` (3 variants) | **High** | Recursive `explore` with visited X, frontier F, selection strategy, union, difference. |
| 2 | Theorem 53.1 (Reachability) | `reachable` (3 variants) | **High** | Calls `graph_search` with `SelectAll`, returns visited. Matches Problem 53.2. |
| 3 | Exercise 53.3 (Multi-source) | `graph_search_multi` | **High** | Accepts set of sources as initial frontier. |
| 4 | Section 4 (PFS) | `pq_min` / `pq_min_multi` (2 variants) | **Medium** | Uses AVL set as implicit PQ. `find_min_priority` is O(|F|) not O(log |F|). |
| 5 | BFS strategy | `SelectAll` | **High** | Selects entire frontier. |
| 6 | DFS strategy | `SelectOne` | **Partial** | Selects smallest by Ord, not most-recently-added as prose requires. |

**Deviations:**
1. **SelectOne is not true DFS.** Prose says "most recent vertex added to frontier." Code selects `to_seq().nth(0)` — minimum by Ord, not by recency.
2. **PQMin uses AVL set as implicit PQ.** Frontier is `AVLTreeSet<Pair<Pair<P, V>, V>>` leveraging sorted order. Clever but min-extraction is O(|F|) not O(log |F|).
3. **Graph-search tree not tracked.** `SearchResult.parent` is always `None` despite Def 53.6.

### 3c. Spec Fidelity

No `requires`/`ensures` on any function. No spec fns. Spec fidelity: **N/A**.

Properties that should be specified when verusified:
1. Theorem 53.1: `ensures result == reachable_set(graph, source)`
2. Termination: at most |V| rounds
3. Monotonicity: visited set grows each round
4. Frontier correctness: `F == out_neighbors(X).difference(X)`

## Phase 4: Parallelism Review

One Mt module: `GraphSearchMtPer`.

| # | Function | Parallel? | Notes |
|---|----------|:---------:|-------|
| 1 | `SelectAll::select` | No | Returns clone of frontier. |
| 2 | `SelectOne::select` | No | Picks first element. |
| 3 | `graph_search` | No | Delegates to `graph_search_multi`. |
| 4 | `graph_search_multi` / `explore` | No | Sequential `for` loop over selected vertices. Individual AVLTreeSetMtPer ops (union, difference) use internal parallelism. |
| 5 | `reachable` | No | Delegates. |

Module header acknowledges: "This is a SEQUENTIAL implementation using thread-safe types." True BFS parallelism would require parallel `map` over selected vertices.

## Phase 5: Runtime Test Review

**No runtime test files exist** for Chapter 53.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | GraphSearchStEph | — | **Missing** |
| 2 | GraphSearchStPer | — | **Missing** |
| 3 | GraphSearchMtPer | — | **Missing** |
| 4 | PQMinStEph | — | **Missing** |
| 5 | PQMinStPer | — | **Missing** |

Proposed priority: (1) GraphSearchStEph/StPer on small directed graph, (2) PQMinStEph/StPer on weighted graph, (3) Edge cases (empty, single vertex, disconnected).

## Phase 6: Proof-Time Test (PTT) Review

No verified loops or iterators. No PTTs needed until verusified.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Definition 53.6 (Graph-Search Tree) | **Partial** — `parent` field exists but always `None`. |
| 2 | Example 53.1 (Web Crawling) | Not implemented — conceptual. |
| 3 | Connected Components | Not implemented — prose mentions but not required. |

### Code With No Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | `SelectionStrategy` trait | Reifies prose's informal "choose U ⊆ F" as a trait. |
| 2 | `ClosurePriority` struct + trait | Adapter for closure-based priority functions. |
| 3 | `PQMinResult` / `SearchResult` structs | Result types bundling visited set, priorities, parent tree. |
| 4 | `find_min_priority` (inner fn) | Helper for AVL-based priority extraction. |

## Phase 8: TOC and In/Out Table

### TOC Presence

| # | File | TOC | Section Headers |
|---|------|:---:|:---------------:|
| 1 | PQMinStEph.rs | Partial | Yes (4, 8, 9, 13) |
| 2 | PQMinStPer.rs | Partial | Yes (4, 8, 9, 13) |
| 3 | GraphSearchStEph.rs | Partial | Yes (4, 8, 9, 13) |
| 4 | GraphSearchStPer.rs | Partial | Yes (4, 8, 9, 13) |
| 5 | GraphSearchMtPer.rs | Partial | Yes (4, 8, 9, 13) |

Section headers are present but no formal TOC comment block at top of file.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | PQMinStEph.rs | `✅ in` (derive on PQMinResult) | - | - | - | - | `✅ out` | - | - | - |
| 2 | PQMinStPer.rs | `✅ in` (derive on PQMinResult) | - | - | - | - | `✅ out` | - | - | - |
| 3 | GraphSearchStEph.rs | `✅ in` (derive on SearchResult) | - | - | - | - | `✅ out` | - | - | - |
| 4 | GraphSearchStPer.rs | `✅ in` (derive on SearchResult) | - | - | - | - | `✅ out` | - | - | - |
| 5 | GraphSearchMtPer.rs | `✅ in` (derive on SearchResult) | - | - | - | - | `✅ out` | - | - | - |

All Clone derives are inside `verus!` (correct). Debug impls are outside `verus!` (correct — Verus limitation). No `❌` items.

## Proof Holes Summary

```
veracity-review-proof-holes output (2026-02-18):

Modules: 0 clean, 5 holed
Holes Found: 23 total (all external_body)
Proof Functions: 0 total

PQMinStEph.rs:       4 × external_body
PQMinStPer.rs:       4 × external_body
GraphSearchStEph.rs: 5 × external_body
GraphSearchStPer.rs: 5 × external_body
GraphSearchMtPer.rs: 5 × external_body
```

## Action Items

| # | Action | Priority |
|---|--------|----------|
| 1 | Remove `external_body` from exec fns and add `requires`/`ensures` | High |
| 2 | Add runtime tests for all 5 modules | High |
| 3 | Fix SelectOne to use recency-based selection (true DFS) | Medium |
| 4 | Fix PQMin `find_min_priority` to use O(log n) extraction | Medium |
| 5 | Populate parent tree in SearchResult | Medium |
| 6 | Add formal TOC comment blocks | Low |
| 7 | Add spec fns for reachability and frontier correctness | Low |
