<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 65 — Sequential MST Algorithms: Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory (tool-generated)

Generated via `veracity-review-module-fn-impls -d src/Chap65`.

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap65 | KruskalStEph | 3 | 0 | 0 | 3 | 0 | 3 | 0 | 0 | 3 |
| 2 | Chap65 | PrimStEph | 2 | 2 | 0 | 5 | 0 | 7 | 0 | 0 | 7 |
| 3 | Chap65 | UnionFindStEph | 6 | 7 | 0 | 0 | 0 | 7 | 0 | 0 | 7 |

**Key observation:** All 17 functions are outside `verus!` and have no `requires`/`ensures`. The only `verus!` block in the chapter is the `View` impl for `PQEntry` in `PrimStEph.rs`.

## Phase 2: Prose Inventory

Source: `prompts/Chap65.txt` (Chapter 65: Sequential MST Algorithms)

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Priority function p(v) | p(v) = min_{x in X} w(x,v) — minimum weight edge connecting v to visited set X |
| 2 | Union-Find ADT | Abstract data type with insert, union, find, equals operations |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 65.1 (Prim's) | Priority-first search MST; visits frontier vertex with least priority p(v), extends tree T = T union {(u,v)} |
| 2 | Algorithm 65.2 (Union-Find Kruskal) | Sort edges by weight, iterate with addEdge using Union-Find for cycle detection |

### Cost Specs

| # | Algorithm | Work | Span | Notes |
|---|-----------|------|------|-------|
| 1 | Prim's (binary heap) | O(m lg n) | O(m lg n) | Sequential; can be O(m + n lg n) with Fibonacci heaps |
| 2 | Kruskal's | O(m lg n) | O(m lg n) | Sort O(m lg n) + union-find O(m alpha(n)); sort dominates |
| 3 | Union-Find insert | Theta(1) | Theta(1) | |
| 4 | Union-Find find | O(alpha(n)) amortized | O(alpha(n)) amortized | With path compression |
| 5 | Union-Find union | O(alpha(n)) amortized | O(alpha(n)) amortized | With union by rank |
| 6 | Union-Find equals | O(alpha(n)) amortized | O(alpha(n)) amortized | Two find calls |

### Theorems/Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Correctness of Prim's | Follows from Lemma 64.3 (light-edge cut property); at each step the minimum-weight edge crossing cut (X, V\X) is in the MST |
| 2 | Correctness of Kruskal's | Maintains invariant that chosen edges are in MST; minimum-weight non-cycle edge is a light edge |

### Exercises

| # | Item | Description |
|---|------|-------------|
| 1 | Exercise 65.1 | Prove correctness of Prim's algorithm |
| 2 | Exercise 65.2 | Prove Kruskal's correctly finds MST of undirected graph with unique edge weights |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 17 exec functions already have paired APAS/Claude-Opus-4.6 cost comment lines. No additions needed.

#### Cost Disagreements

| # | Function | File | APAS Cost | Claude-Opus-4.6 Cost | Disagreement |
|---|----------|------|-----------|----------------------|--------------|
| 1 | `kruskal_mst` | KruskalStEph.rs | Work O(m lg n), Span O(m lg n) | Work O(m lg m), Span O(m lg m) | APAS says O(m lg n); implementation sorts edges so O(m lg m). Since m <= n^2, lg m <= 2 lg n, so asymptotically equivalent. The Claude annotation is technically more precise. |
| 2 | `mst_weight` (Kruskal) | KruskalStEph.rs | Work O(m), Span O(1) | Work O(\|MST\|), Span O(\|MST\|) | APAS Span O(1) is aspirational (parallel reduce). Implementation is sequential linear scan. |
| 3 | `mst_weight` (Prim) | PrimStEph.rs | Work O(m), Span O(1) | Work O(\|MST\|), Span O(\|MST\|) | Same as above. |

### 3b. Implementation Fidelity

#### `kruskal_mst` (Algorithm 65.2)

The implementation closely follows the prose pseudocode:
- Initializes Union-Find with all vertices via `iterate insert empty V` -> `for vertex in graph.vertices().iter() { uf.insert(...) }`
- Sorts edges by weight -> `edges_vec.sort_by(|e1, e2| w1.cmp(w2))`
- Iterates with addEdge -> `for edge in edges_vec.iter() { if !uf.equals(u, v) { mst_edges.insert(edge); uf.union(u, v); } }`

**Fidelity: High.** Direct translation of Algorithm 65.2.

#### `prim_mst` (Algorithm 65.1)

The implementation follows the priority-first search pattern:
- Uses `BinaryHeapPQ` for priority queue (matching prose's binary heap suggestion)
- Priority is the edge weight w(u,v), matching p(v) = min_{x in X} w(x,v)
- Maintains visited set X and extends tree T on each visit

**Deviations:**
1. Uses `get_neighbors` which scans all edges O(m) per call. With n vertices visited and O(m) work per visit, the inner loop is O(nm) in the worst case. The prose assumes adjacency list access in O(degree(v)) per vertex.
2. `get_edge_weight` also scans all edges O(m) per lookup.

**Fidelity: Medium.** Algorithmic structure matches, but data structure access patterns differ from APAS assumptions, inflating the actual cost to O(nm).

#### Union-Find (`UnionFindStEph`)

- Implements path compression in `find` (recursive with parent update). Matches prose.
- Implements union by rank in `union`. Matches prose.
- `equals` delegates to two `find` calls. Matches prose.
- `insert` uses HashMap for parent/rank storage. Matches ADT.

**Fidelity: High.** Standard textbook Union-Find with both optimizations.

### 3c. Spec Fidelity

No functions have `requires`/`ensures` clauses, so there is no spec to compare against prose. All 17 functions are unverified.

**Missing specs of note:**
- `kruskal_mst`: Should ensure result is a subset of input edges, result has n-1 edges (for connected graph), result forms a tree, result has minimum total weight.
- `prim_mst`: Same properties as Kruskal's.
- `find`: Should ensure result is a valid representative and path compression preserves set membership.
- `union`: Should ensure sets are merged correctly.
- `equals`: Should ensure result reflects same-set membership.

## Phase 4: Parallelism Review

**No `*Mt*` modules in Chapter 65.** All three modules are `*StEph*` (sequential ephemeral). The prose describes these as sequential algorithms (Work = Span for both Prim's and Kruskal's). No parallelism review needed.

## Phase 5: Runtime Test Review

**No runtime test files found.** There are no files matching `tests/*Chap65*`, `tests/*kruskal*`, `tests/*prim*`, or `tests/*union_find*`.

### 5a. Coverage Check

| # | Module | Exec Functions | Test File | Coverage |
|---|--------|---------------|-----------|----------|
| 1 | KruskalStEph | `kruskal_mst`, `mst_weight`, `verify_mst_size` | None | 0% |
| 2 | PrimStEph | `prim_mst`, `mst_weight`, `pq_entry_new`, `cmp`, `partial_cmp`, `get_neighbors`, `get_edge_weight` | None | 0% |
| 3 | UnionFindStEph | `new`, `insert`, `find`, `union`, `equals`, `num_sets`, `default` | None | 0% |

### 5b. Missing Tests (Proposed)

1. **UnionFindStEph**: Basic insert/find/union/equals operations, cycle of unions, path compression correctness, union by rank behavior, num_sets tracking.
2. **KruskalStEph**: Small graph MST correctness, verify MST size = n-1, total weight, comparison with known MST.
3. **PrimStEph**: Small graph MST correctness, different start vertices yield same MST weight, comparison with Kruskal result.
4. **Cross-algorithm**: Both Kruskal and Prim produce same MST weight on same graph.

## Phase 6: Proof-Time Test (PTT) Review

**No iterators and no verified loops** in Chapter 65. All code is outside `verus!`. No PTTs are needed or expected.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | KruskalStEph | None | N/A | Missing RTT |
| 2 | PrimStEph | None | N/A | Missing RTT |
| 3 | UnionFindStEph | None | N/A | Missing RTT |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Exercise 65.1 (Prove Prim's correctness) | Not implemented | Would require verified spec + proof |
| 2 | Exercise 65.2 (Prove Kruskal's correctness) | Not implemented | Would require verified spec + proof |
| 3 | Fibonacci heap variant of Prim's | Not implemented | O(m + n lg n) variant mentioned in prose; not expected |

### Code With No Prose Counterpart

| # | Function | File | Purpose |
|---|----------|------|---------|
| 1 | `mst_weight` | KruskalStEph.rs | Utility to compute total MST weight |
| 2 | `verify_mst_size` | KruskalStEph.rs | Utility to check MST has n-1 edges |
| 3 | `pq_entry_new` | PrimStEph.rs | Constructor for PQ entry struct |
| 4 | `cmp` / `partial_cmp` | PrimStEph.rs | Ord/PartialOrd impls for PQ entry |
| 5 | `get_neighbors` | PrimStEph.rs | Helper to extract neighbors from edge set |
| 6 | `get_edge_weight` | PrimStEph.rs | Helper to look up edge weight |
| 7 | `mst_weight` | PrimStEph.rs | Duplicate of Kruskal's mst_weight utility |
| 8 | `num_sets` | UnionFindStEph.rs | Count distinct sets; not in APAS ADT |
| 9 | `default` | UnionFindStEph.rs | Rust Default trait impl |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present | Section Headers | Notes |
|---|------|:-----------:|:---------------:|-------|
| 1 | KruskalStEph.rs | No | No | No `verus!` block at all |
| 2 | PrimStEph.rs | No | No | Minimal `verus!` block (View impl only) |
| 3 | UnionFindStEph.rs | No | No | No `verus!` block at all |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | KruskalStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | PrimStEph.rs | out | out | - | - | - | out | out | - | Ord/PartialOrd out |
| 3 | UnionFindStEph.rs | - | - | out | - | - | - | - | - | - |

**Issues:**
- PrimStEph.rs: `PQEntry` derives Clone, Eq, PartialEq, Debug outside `verus!`. Clone and PartialEq/Eq should be inside `verus!` with specs. Debug should stay outside.
- PrimStEph.rs: `Display` impl for `PQEntry` is outside `verus!` — correct placement.
- PrimStEph.rs: `Ord` and `PartialOrd` impls for `PQEntry` are outside `verus!` — these could benefit from specs inside `verus!`.
- UnionFindStEph.rs: `Default` impl is outside `verus!` — should be inside with `ensures` connecting to `new()`.

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap65/

Modules: 3 clean, 0 holed
Holes Found: 0 total
```

**All clean.** No `assume`, `admit`, or `external_body` in any Chap65 file. However, this is because the code is entirely unverified (outside `verus!`), not because the proofs are complete.

## Spec Strength Summary

| Classification | Count |
|----------------|-------|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 17 |

**All 17 functions have no spec.** The chapter is entirely unverified — no `requires`, no `ensures`, no functions inside `verus!` (except the trivial `View` impl).

## Overall Assessment

Chapter 65 is a **functionally correct but entirely unverified** implementation of two classical sequential MST algorithms (Prim's and Kruskal's) plus their supporting Union-Find data structure.

### Strengths

1. **Algorithmic fidelity**: Kruskal's algorithm closely follows Algorithm 65.2. Union-Find implements both path compression and union by rank.
2. **Zero proof holes**: No `assume`, `admit`, or `external_body` anywhere.
3. **Cost annotations**: All 17 exec functions have paired APAS/Claude-Opus-4.6 cost comments.

### Weaknesses

1. **No verification**: All code is outside `verus!`. No `requires`/`ensures` on any function. Spec strength is `none` across the board.
2. **No tests**: No RTT or PTT files exist. Zero test coverage.
3. **No TOC headers**: None of the three files follow the table-of-contents standard.
4. **Prim's cost inflation**: `get_neighbors` and `get_edge_weight` use O(m) linear scans instead of O(degree) adjacency list lookups, inflating Prim's actual cost to O(nm) vs the prose's O(m lg n).
5. **Duplicate code**: `mst_weight` is duplicated verbatim in both KruskalStEph.rs and PrimStEph.rs.
6. **Trait signature mismatch**: `PrimStEphTrait::prim_mst` takes `start: V` by value; the implementation `prim_mst` takes `start: &V` by reference. `KruskalStEphTrait::verify_mst_size` takes `graph` as first parameter; the implementation takes `n_vertices: N`.

### Priority Actions

1. Add runtime tests for all three modules.
2. Move code inside `verus!` and add specs to key functions (kruskal_mst, prim_mst, find, union, equals).
3. Fix trait/impl signature mismatches.
4. Refactor `get_neighbors`/`get_edge_weight` in PrimStEph to use proper adjacency structure or document the cost inflation.
5. Extract shared `mst_weight` to avoid duplication.
6. Add TOC headers to all files.
