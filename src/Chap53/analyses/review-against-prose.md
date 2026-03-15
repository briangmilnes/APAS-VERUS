# Review Against Prose: Chapter 53 -- Graph Search

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap53.txt`
- Reference: APAS Chapter 53

## Phase 1: Inventory

5 source files, 23 exec functions total, 0 proof functions.

| # | Chap | File | Tr | IT | ML | V! | Holes |
|---|------|------|:--:|:--:|:--:|:--:|:-----:|
| 1 | 53 | GraphSearchStEph.rs | 4 | 4 | 4 | 5 | 0 |
| 2 | 53 | GraphSearchStPer.rs | 4 | 4 | 4 | 5 | 0 |
| 3 | 53 | GraphSearchMtPer.rs | 4 | 4 | 4 | 5 | 0 |
| 4 | 53 | PQMinStEph.rs | 2 | 2 | 4 | 4 | 0 |
| 5 | 53 | PQMinStPer.rs | 2 | 2 | 4 | 4 | 0 |

## Phase 2: Prose Inventory

### Definitions
- Def 53.1: Source vertex in graph search
- Def 53.2: Visited vertices (set X)
- Def 53.3: Frontier and Discovered Vertices (F = N+(X) \ X)
- Def 53.5: Reachability
- Def 53.6: Graph-Search Tree

### Algorithms
- Alg 53.4: Generic Graph Search (Single Source) -- iterative explore with visited X and frontier F
- Exercise 53.3: Multi-source graph search

### Theorems/Properties
- Thm 53.1: Graph search solves reachability (at most |V| rounds)
- Problem 53.2: The Graph Reachability Problem

### Priority-First Search (PFS)
- Section 4: PFS as specialization of graph search
- Three selection strategies: all frontier, single most recent, highest priority

### Cost Specifications
- No explicit cost spec in prose; cost depends on data structure choices

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions in GraphSearchStEph.rs, GraphSearchStPer.rs, GraphSearchMtPer.rs, PQMinStEph.rs, PQMinStPer.rs already have APAS/Claude two-line cost annotations.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Ref | Fidelity |
|---|------|------|----------|-----------|:--------:|
| 1 | 53 | GraphSearchStEph.rs | graph_search | Alg 53.4 | Match |
| 2 | 53 | GraphSearchStEph.rs | graph_search_multi | Ex 53.3 | Match |
| 3 | 53 | GraphSearchStEph.rs | reachable | Problem 53.2 | Match |
| 4 | 53 | GraphSearchStEph.rs | graph_search_explore | Alg 53.4 explore | Match |
| 5 | 53 | GraphSearchStEph.rs | SelectAll | BFS strategy | Match |
| 6 | 53 | GraphSearchStEph.rs | SelectOne | DFS strategy | Match |
| 7 | 53 | PQMinStEph.rs | pq_min | Section 53.4 PFS | Match |
| 8 | 53 | PQMinStEph.rs | pq_min_multi | Multi-source PFS | Match |
| 9 | 53 | PQMinStEph.rs | pq_explore | PFS explore loop | Match |
| 10 | 53 | PQMinStEph.rs | pq_find_min_priority | Min extraction | Match |

**Implementation matches APAS Algorithm 53.4 closely.** The explore function maintains visited set X and frontier F, selects a subset U via the strategy, visits U, updates X = X union U, and computes new frontier F = N+(X) \ X. The implementation uses AVLTreeSet for both X and F, with union/difference operations.

**PFS implementation**: Uses a frontier of `Pair<Pair<P, V>, V>` entries sorted by (priority, vertex) to achieve min-priority selection via the AVL tree's natural ordering. This matches the PFS concept from Section 53.4.

**SelectAll vs SelectOne**: SelectAll returns the entire frontier (BFS behavior). SelectOne extracts the first element of the frontier sequence (DFS-like behavior).

### 3c. Spec Fidelity

| # | Chap | File | Function | Spec Assessment |
|---|------|------|----------|:---------------:|
| 1 | 53 | GraphSearchStEph.rs | graph_search | Partial |
| 2 | 53 | GraphSearchStEph.rs | graph_search_multi | Partial |
| 3 | 53 | GraphSearchStEph.rs | reachable | Partial |
| 4 | 53 | PQMinStEph.rs | pq_min | Partial |
| 5 | 53 | PQMinStEph.rs | pq_min_multi | Partial |

**Spec gap**: The ensures clauses guarantee `visited@.contains(source@)` (source is in returned set) and `sources@.subset_of(search.visited@)` (all sources reached). However, they do not prove the full reachability theorem (Thm 53.1): that the returned set equals *all* reachable vertices. This would require a spec function `spec_reachable(graph, source, v)` and an ensures `forall|v| visited@.contains(v@) <==> spec_reachable(...)`. The graph is passed as a closure `Fn(&V) -> Set<V>`, making such a spec difficult without a fixed graph representation.

## Phase 4: Parallelism Review

| # | Chap | File | Classification | Notes |
|---|------|------|:--------------:|-------|
| 1 | 53 | GraphSearchMtPer.rs | Sequential | Uses MtPer types but explore loop is sequential |

The MtPer variant uses AVLTreeSetMtPer which internally supports parallel operations (union, intersection, filter), but the overall graph_search_explore loop iterates over frontier vertices sequentially. This is consistent with the generic algorithm, which does not specify parallelism at the search level. BFS parallelism (visiting entire frontier at once) is handled in Chap54's dedicated BFS modules.

**No parallelism gap**: The prose notes that graph search can visit multiple vertices in parallel when using BFS strategy, but that parallelism is realized in Chap54, not here.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Module Under Test |
|---|------|-----------|-------------------|
| 1 | 53 | TestGraphSearchStEph.rs | GraphSearchStEph |
| 2 | 53 | TestGraphSearchStPer.rs | GraphSearchStPer |
| 3 | 53 | TestGraphSearchMtPer.rs | GraphSearchMtPer |
| 4 | 53 | TestPQMinStEph.rs | PQMinStEph |
| 5 | 53 | TestPQMinStPer.rs | PQMinStPer |

All 5 modules have RTT coverage. Complete coverage.

## Phase 6: PTT Review

No proof-time tests for Chap53. None required -- no iterators or complex callability patterns.

## Phase 7: Gap Analysis

### Prose items not implemented

| # | Chap | Prose Item | Status |
|---|------|-----------|--------|
| 1 | 53 | Graph-Search Tree (Def 53.6) | Partially implemented |
| 2 | 53 | Search tree parent tracking | SearchResult has parent field but populated as None |
| 3 | 53 | Example 53.1: Web Crawling | N/A (example, not algorithm) |
| 4 | 53 | Connected Components via search | Not implemented (later chapters) |
| 5 | 53 | Beam search (top-k PFS) | Not implemented |

**Note**: The `SearchResult` struct has a `parent: Option<AVLTreeSetStEph<Pair<V, V>>>` field that could hold the search tree edges, but it is always set to `None` in the current implementation. This means the graph-search tree (Def 53.6) is defined but not constructed.

### Code with no prose counterpart

| # | Chap | File | Item | Notes |
|---|------|------|------|-------|
| 1 | 53 | PQMinStEph.rs | PQMinResult struct | Combines visited, priorities, parent |
| 2 | 53 | PQMinStEph.rs | pq_find_min_priority | Helper to extract min from AVL set |

These are implementation details needed for the PFS algorithm.

## Phase 8: TOC Review

All 5 files follow the standard TOC ordering. GraphSearchStEph.rs has sections 4, 8, 9, 11, 13. PQMinStEph.rs has sections 4, 6, 8, 9, 11, 13. No violations.

## Proof Holes Summary

| # | Chap | File | Holes |
|---|------|------|:-----:|
| 1 | 53 | GraphSearchStEph.rs | 0 |
| 2 | 53 | GraphSearchStPer.rs | 0 |
| 3 | 53 | GraphSearchMtPer.rs | 0 |
| 4 | 53 | PQMinStEph.rs | 0 |
| 5 | 53 | PQMinStPer.rs | 0 |
| | | **Total** | **0** |

All 5 modules are clean. No proof holes.

## Overall Assessment

Chapter 53 is fully clean with 0 holes across all 5 modules. The implementation faithfully follows APAS Algorithm 53.4 (generic graph search) and Section 53.4 (priority-first search). The main spec weakness is that the ensures clauses prove source inclusion but not full reachability (Thm 53.1). This is because the graph is abstracted as a closure, making it difficult to state reachability specs without a concrete graph representation. The parent-tracking feature (graph-search tree, Def 53.6) is structurally present but not populated. Weighted graph extensions (for Dijkstra/Prim) are not present here but would naturally extend PQMinStEph.
