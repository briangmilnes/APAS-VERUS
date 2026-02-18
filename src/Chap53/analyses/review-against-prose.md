<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 53 — Graph Search: Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory (tool-generated)

Generated via `veracity-review-module-fn-impls -d src/Chap53`.

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap53 | GraphSearchMtPer | 4 | 1 | 0 | 4 | 0 | 5 | 0 | 0 | 5 |
| 2 | Chap53 | GraphSearchStEph | 4 | 1 | 0 | 4 | 0 | 5 | 0 | 0 | 5 |
| 3 | Chap53 | GraphSearchStPer | 4 | 1 | 0 | 4 | 0 | 5 | 0 | 0 | 5 |
| 4 | Chap53 | PQMinStEph | 4 | 2 | 0 | 4 | 0 | 6 | 0 | 0 | 6 |
| 5 | Chap53 | PQMinStPer | 4 | 2 | 0 | 4 | 0 | 6 | 0 | 0 | 6 |

**Key observation:** All 27 functions are outside `verus!` with no specifications. Zero functions are inside `verus!`. This chapter is entirely unverified.

## Phase 2: Prose Inventory

Source: `prompts/Chap53.txt` (Chapter 53 — Graph Search)

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 53.1 (Source) | A graph search starts at a source vertex s ∈ V or a set of source vertices. |
| 2 | Definition 53.2 (Visited Vertices) | The set X of already-visited vertices. |
| 3 | Definition 53.3 (Frontier / Discovered) | Frontier F = N⁺_G(X) \ X — unvisited out-neighbors of visited set. |
| 4 | Definition 53.5 (Reachability) | Vertex v is reachable from u if there is a path from u to v. |
| 5 | Definition 53.6 (Graph-Search Tree) | Rooted tree over visited vertices X ⊆ V with edges E' ⊆ E; each visited vertex (except source) has a parent in X. |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 53.4 (Graph Search, Single Source) | Generic graph search: `explore X F` loop choosing U ⊆ F, visiting U, updating X = X ∪ U, F = N⁺_G(X) \ X. |
| 2 | (Implicit) Multi-source variant | Exercise 53.3: extend to start from a set of sources. |
| 3 | (Implicit) BFS | SelectAll: select entire frontier each round. |
| 4 | (Implicit) DFS | SelectOne: select single most-recently-added vertex. |
| 5 | (Implicit) PFS | Priority-First Search: select by priority (Section 4). |

### Theorems / Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Theorem 53.1 | `graphSearch(G, s)` returns exactly the reachable vertices from s, in at most \|V\| rounds, for any selection of U. |

### Exercises / Problems

| # | Item | Description |
|---|------|-------------|
| 1 | Problem 53.2 | Graph Reachability: return all vertices reachable from v. |
| 2 | Exercise 53.3 | Multi-source graph search. |

### Cost Specs

The prose does **not** give explicit Work/Span bounds for Algorithm 53.4 or PFS in this chapter. It states:
- "at most |V| rounds" (Theorem 53.1)
- BFS is parallel (visits whole frontier per round)
- DFS is sequential (one vertex per round)
- PFS cost depends on the priority queue implementation (deferred to later chapters: Dijkstra, Prim)

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 53.1 (Web Crawling) | Conceptual example; no code implementation expected. |

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

Cost annotations have been added/updated in all 5 source files. Summary of findings:

| # | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|----------|-----------|---------------------|-----------|
| 1 | `graph_search` (St variants) | No explicit cost | Work Θ((|V|+|E|) log |V|), Span = Work | N/A — no APAS cost to compare |
| 2 | `graph_search_multi` (St variants) | No explicit cost | Work Θ((|V|+|E|) log |V|), Span = Work | N/A |
| 3 | `reachable` (St variants) | No explicit cost | Work Θ((|V|+|E|) log |V|), Span = Work | N/A |
| 4 | `graph_search` (MtPer) | No explicit cost | Work Θ((|V|+|E|) log |V|), Span = Work | N/A — neighbor loop is sequential |
| 5 | `pq_min` / `pq_min_multi` (St variants) | No explicit PFS cost | Work Θ(|V|² + |E| log |V|), Span = Work | N/A — `find_min_priority` uses `to_seq` O(|F|) per round |

**Key cost deviation in PQMin:** The `find_min_priority` helper calls `to_seq().nth(0)` which materializes the entire sorted sequence to extract the minimum — O(|F|) per call. Over |V| rounds, this contributes O(|V|²) to total work. A proper min-extraction on the AVL tree (O(log |V|)) would give the expected O((|V| + |E|) log |V|) for Dijkstra-style PFS.

### Phase 3b: Implementation Fidelity

| # | Prose Item | Code Function | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Algorithm 53.4 (Graph Search) | `graph_search_multi` / `explore` (all 3 variants) | **High** | Faithful implementation: recursive `explore` with visited set X, frontier F, selection strategy, union, difference. Line-by-line comments in StPer map to prose lines 4-13. |
| 2 | Theorem 53.1 (Reachability) | `reachable` (all 3 variants) | **High** | Calls `graph_search` with `SelectAll` and returns `visited`. Matches prose Problem 53.2. |
| 3 | Exercise 53.3 (Multi-source) | `graph_search_multi` | **High** | Extends single-source by accepting a set of sources as initial frontier. |
| 4 | Section 4 (PFS) | `pq_min` / `pq_min_multi` (2 variants) | **Medium** | Implements PFS concept but uses AVL tree set as implicit priority queue (sorted by `(priority, vertex)` pair). No dedicated PQ ADT. `find_min_priority` is O(|F|) instead of O(log |F|). |
| 5 | BFS strategy | `SelectAll` | **High** | Selects entire frontier — matches prose. |
| 6 | DFS strategy | `SelectOne` | **Partial** | Selects "first" element from sorted sequence (smallest by Ord), not most-recently-added as prose specifies. This is an arbitrary selection, not true DFS ordering. |

**Deviations:**
1. **SelectOne is not true DFS.** The prose says DFS selects "the single most recent vertex added to the frontier." The implementation selects `to_seq().nth(0)` — the smallest element by `Ord` ordering, which is arbitrary and does not correspond to recency. This makes `SelectOne` a minimum-first selection, not a depth-first one.
2. **PQMin uses AVL set as implicit PQ.** The frontier is stored as `AVLTreeSetStEph<Pair<Pair<P, V>, V>>` where the first element of the outer pair is `(priority, vertex)`, leveraging AVL sorted order for min-extraction. This is clever but the `to_seq().nth(0)` extraction is O(|F|) instead of O(log |F|).
3. **Graph-search tree not tracked.** Definition 53.6 describes a graph-search tree with parent pointers. The `SearchResult` struct has a `parent: Option<AVLTreeSetStPer<Pair<V, V>>>` field, but it is always set to `None`. The parent tracking infrastructure exists but is never populated.

### Phase 3c: Spec Fidelity

All functions are outside `verus!` with **no specifications whatsoever**. There are:
- No `requires` clauses
- No `ensures` clauses
- No `spec fn` definitions
- No `proof fn` definitions
- No `View` implementations

Spec fidelity: **N/A** — nothing to compare against prose properties.

**Prose properties that should be specified (if verusified):**
1. Theorem 53.1: `ensures result == reachable_set(graph, source)` — the returned set is exactly the reachable vertices.
2. Termination: the search terminates in at most |V| rounds.
3. Monotonicity: visited set grows each round (`old(visited).subset_of(visited)`).
4. Frontier correctness: `F == out_neighbors(X).difference(X)`.

## Phase 4: Parallelism Review

### Phase 4a: Mt Function Classification

Only one Mt module exists: `GraphSearchMtPer`.

| # | Function | Classification | Reason |
|---|----------|---------------|--------|
| 1 | `SelectAll::select` | Sequential | Returns clone of frontier — no spawning. |
| 2 | `SelectOne::select` | Sequential | Picks first element — no spawning. |
| 3 | `graph_search` | Sequential | Delegates to `graph_search_multi`. |
| 4 | `graph_search_multi` | Sequential | Sequential control flow; loop over selected vertices is `for i in 0..`. |
| 5 | `explore` | Sequential | Same sequential loop; individual set operations (union, difference) use MtPer types which have internal parallelism. |

### Phase 4b: Span Audit

| # | Function | Annotated Span | Actual Span | Match? | Notes |
|---|----------|---------------|-------------|--------|-------|
| 1 | `graph_search` (MtPer) | Θ((|V|+|E|) log |V|) | Θ((|V|+|E|) log |V|) | Yes | Sequential control flow; span = work. |
| 2 | `graph_search_multi` (MtPer) | Θ((|V|+|E|) log |V|) | Θ((|V|+|E|) log |V|) | Yes | Neighbor loop sequential. |
| 3 | `reachable` (MtPer) | Θ((|V|+|E|) log |V|) | Θ((|V|+|E|) log |V|) | Yes | Delegates. |

### Phase 4c: Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | `graph_search_multi` (MtPer) | Aspirational: Θ(d × log |V|) for BFS | Θ((|V|+|E|) log |V|) | **No** | Neighbor-gathering loop is sequential. AVLTreeSetMtPer set ops are internally parallel but the overall control flow serializes rounds. |

**Note:** The module header acknowledges this: "This is a SEQUENTIAL implementation using thread-safe types." True parallelism for BFS would require parallel `map` over selected vertices to gather neighbors, not a sequential `for` loop.

## Phase 5: Runtime Test Review

### Phase 5a: Coverage

**No runtime test files exist for Chapter 53.** No files matching `tests/*Chap53*`, `tests/*graph_search*`, or similar patterns were found.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | GraphSearchStEph | — | **Missing** |
| 2 | GraphSearchStPer | — | **Missing** |
| 3 | GraphSearchMtPer | — | **Missing** |
| 4 | PQMinStEph | — | **Missing** |
| 5 | PQMinStPer | — | **Missing** |

### Phase 5b: Test Quality

N/A — no tests exist.

### Phase 5c: Missing Tests (Proposed)

Priority order:
1. **GraphSearchStEph / StPer** — test `graph_search` and `reachable` on a small directed graph (e.g., 5 vertices). Verify reachable set matches expected. Test disconnected vertices are excluded.
2. **GraphSearchMtPer** — same tests using Mt types.
3. **PQMinStEph / StPer** — test `pq_min` on a weighted graph. Verify that vertices are visited in priority order.
4. **SelectOne** — test that it actually selects a vertex (not necessarily DFS order, given the deviation noted above).
5. **Edge cases** — empty graph, single vertex, self-loops, disconnected components.

## Phase 6: Proof-Time Test (PTT) Review

This chapter has **no iterators and no verified loops** — all code is outside `verus!`. No PTTs are needed. No PTT files were found.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Definition 53.6 (Graph-Search Tree) | **Partially implemented.** `SearchResult` has a `parent` field but it is always `None`. No code builds the tree. |
| 2 | Example 53.1 (Web Crawling) | **Not implemented.** Conceptual example — no code expected. |
| 3 | Connected Components (prose section after Problem 53.2) | **Not implemented.** Prose mentions using graph search to find connected components; not in code. |

### Code with No Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | `SelectionStrategy` trait | Verus-specific scaffolding. The prose describes selection informally ("choose U ⊆ F"); the code reifies this as a trait. Reasonable design choice. |
| 2 | `ClosurePriority` struct | Adapter to wrap closures as `PriorityFn`. Infrastructure helper. |
| 3 | `ClosurePriorityTrait` | Trait for constructing `ClosurePriority`. |
| 4 | `PQMinResult` struct | Result type for PFS; bundles visited set, priorities, and parent tree. |
| 5 | `SearchResult` struct | Result type for generic search; bundles visited set and parent tree. |
| 6 | `find_min_priority` (inner fn) | Helper to extract minimum from AVL-tree-based priority queue. |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present? | Notes |
|---|------|:----------:|-------|
| 1 | `GraphSearchStEph.rs` | **No** | No TOC comment block. |
| 2 | `GraphSearchStPer.rs` | **No** | No TOC comment block. |
| 3 | `GraphSearchMtPer.rs` | **No** | No TOC comment block. |
| 4 | `PQMinStEph.rs` | **No** | No TOC comment block. |
| 5 | `PQMinStPer.rs` | **No** | No TOC comment block. |

None of the files follow the table-of-contents standard. Since all code is outside `verus!`, the standard TOC sections (which assume `verus!` blocks) do not directly apply.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | `GraphSearchStEph.rs` | - | - | - | - | - | `✅ out` (derive on SearchResult) | - | - | - |
| 2 | `GraphSearchStPer.rs` | - | - | - | - | - | `✅ out` (derive on SearchResult) | - | - | - |
| 3 | `GraphSearchMtPer.rs` | - | - | - | - | - | `✅ out` (derive on SearchResult) | - | - | - |
| 4 | `PQMinStEph.rs` | - | - | - | - | - | `✅ out` (derive on PQMinResult) | - | - | - |
| 5 | `PQMinStPer.rs` | - | - | - | - | - | `✅ out` (derive on PQMinResult) | - | - | - |

Debug derives are correctly outside `verus!` (since there is no `verus!` block). Clone derives are on result structs — acceptable outside `verus!` for now. No `❌` items.

## Proof Holes Summary

```
✓ GraphSearchMtPer.rs
✓ GraphSearchStEph.rs
✓ GraphSearchStPer.rs
✓ PQMinStEph.rs
✓ PQMinStPer.rs

Modules: 5 clean, 0 holed
Holes Found: 0 total
```

**No proof holes** — but this is trivially true because no code is inside `verus!`. There are no proofs to have holes in.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | **27** |

All 27 functions have **no specifications**. The entire chapter is unverified Rust code with no `verus!` blocks, no `requires`/`ensures`, no `spec fn`, and no `proof fn`.

## Overall Assessment

Chapter 53 is a **functionally correct but entirely unverified** implementation of generic graph search, reachability, and priority-first search. The algorithmic structure faithfully follows the APAS prose (Algorithm 53.4, Theorem 53.1, Exercise 53.3, Problem 53.2).

### Strengths
1. Clean separation of concerns via `SelectionStrategy` trait — makes BFS/DFS/PFS pluggable.
2. Three variants (StEph, StPer, MtPer) following project conventions.
3. Multi-source search extends naturally from single-source.
4. No proof holes (trivially — no proofs exist).

### Issues (ranked by severity)
1. **No Verus verification at all.** 0/27 functions are inside `verus!`. No specs, no proofs, no ghost state.
2. **No runtime tests.** Zero test coverage for any module.
3. **SelectOne is not DFS.** It selects the minimum element by `Ord`, not the most-recently-added. This is an arbitrary-first search, not depth-first.
4. **PQMin find_min is O(|F|).** Using `to_seq().nth(0)` instead of proper O(log n) min-extraction adds an O(|V|²) term to PFS work.
5. **Parent tree never populated.** `SearchResult.parent` is always `None` despite Definition 53.6 defining the concept.
6. **MtPer is sequential.** The neighbor-gathering loop serializes what could be parallel work. The module header acknowledges this.
7. **No TOC headers.** None of the 5 files follow the table-of-contents standard.
8. **Missing PQMinMtPer / PQMinMtEph.** Only StEph and StPer variants exist for PQMin; no Mt variants.
9. **Missing GraphSearchMtEph / GraphSearchStEphPer crossover.** Only 3 of the 4 expected variants exist for graph search (no MtEph).
