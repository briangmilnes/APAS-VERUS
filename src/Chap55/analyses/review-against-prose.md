<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 55: Depth-First Search — Review Against Prose

- **Date**: 2026-02-13
- **Reviewer**: Claude-Opus-4.6
- **Chapter**: 55 — Depth-First Search
- **Files reviewed**: 8
- **Verification status**: Not verusified (plain Rust, no `verus!` blocks)

## Phase 1: Inventory (Tool-Generated)

Generated via `veracity-review-module-fn-impls -d src/Chap55`.

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap55 | CycleDetectStEph | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 2 | Chap55 | CycleDetectStPer | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 3 | Chap55 | DFSStEph | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 4 | Chap55 | DFSStPer | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 5 | Chap55 | SCCStEph | 1 | 0 | 0 | 5 | 0 | 5 | 0 | 0 | 5 |
| 6 | Chap55 | SCCStPer | 1 | 0 | 0 | 5 | 0 | 5 | 0 | 0 | 5 |
| 7 | Chap55 | TopoSortStEph | 1 | 0 | 0 | 4 | 0 | 4 | 0 | 0 | 4 |
| 8 | Chap55 | TopoSortStPer | 1 | 0 | 0 | 4 | 0 | 4 | 0 | 0 | 4 |

**Key observation**: All 26 functions are outside `verus!`, all have NoSpec. Zero functions have requires/ensures.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Def 55.3 | Tree and Non-Tree Edges in DFS (tree, back, forward, cross edges) |
| 2 | Def 55.6 | DFS Numbers (visit time, finish time) |
| 3 | Def 55.9 | Cycle-Detection Problem |
| 4 | Def 55.11 | Directed Acyclic Graph (DAG) |
| 5 | Def 55.12 | Topological Sort of a DAG |
| 6 | Def 55.14 | Strongly Connected Graph |
| 7 | Def 55.15 | Strongly Connected Components |
| 8 | Def 55.16 | Component DAG |
| 9 | Def 55.17 | SCC Problem |

### Algorithms

| # | Item | Description | Implemented? |
|---|------|-------------|:------------:|
| 1 | Alg 55.1 | DFS with a Stack | No |
| 2 | Alg 55.2 | DFS, Recursively (DFSReach) | Yes — `DFSStEph::dfs`, `DFSStPer::dfs` |
| 3 | Alg 55.4 | Generic DFS (with visit/finish/revisit callbacks) | No — not directly; specialized versions exist |
| 4 | Alg 55.5 | Generic DFSAll | No — not directly; specialized in TopoSort and SCC |
| 5 | Alg 55.7 | DFS with Array Sequences | Partially — `DFSStEph` uses array for visited but AVL set for result |
| 6 | Alg 55.10 | Directed Cycle Detection | Yes — `CycleDetectStEph::has_cycle`, `CycleDetectStPer::has_cycle` |
| 7 | Alg 55.13 | Topological Sort | Yes — `TopoSortStEph::topo_sort`, `TopoSortStPer::topo_sort` |
| 8 | Alg 55.18 | Strongly Connected Components | Yes — `SCCStEph::scc`, `SCCStPer::scc` |

### Cost Specifications

| # | Item | Description |
|---|------|-------------|
| 1 | Cost 55.8 | DFS: O((m+n) log n) for tree-based sets / adj tables; O(m+n) for array sequences |

### Theorems/Lemmas

| # | Item | Description | Proved? |
|---|------|-------------|:-------:|
| 1 | Lemma 55.1 | DFS Numbers classify edges (cross/forward/back) | No |
| 2 | Lemma 55.2 | Bound on DFS calls: n+m calls to DFS, n to visit/finish, m to revisit | No |
| 3 | Thm 55.3 | Back edges iff cycles in directed graphs | No |
| 4 | Lemma 55.4 | DAG Finish Order: if u reachable from v, u finishes before v | No |
| 5 | Lemma 55.5 | First Visited: first vertex visited in component has latest finish time | No |
| 6 | Thm 55.6 | SCC Correctness | No |

### Exercises

| # | Item | Description | Implemented? |
|---|------|-------------|:------------:|
| 1 | Ex 55.3 | Prove the DFS Numbers Lemma | No |
| 2 | Ex 55.4 | Restate DFS Numbers Lemma in terms of exploration intervals | No |
| 3 | Ex 55.5 | Design cycle finding for undirected graphs | No |
| 4 | Ex 55.6 | Prove topo sort Work/Span O(|V| + |E|) for enumerable graphs | No |
| 5 | Ex 55.7 | Give DFSReach for SCC in terms of generic DFS | No |
| 6 | Ex 55.8 | Work/Span of SCC algorithm | No |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations have been added to all 26 exec functions. Key findings:

| # | Function | APAS Cost | Actual Cost | Deviation Reason |
|---|----------|-----------|-------------|------------------|
| 1 | `DFSStEph::dfs` | O(|V| + |E|) | O((|V| + |E|) log |V|) | AVLTreeSetStEph insert is O(log n), not O(1) |
| 2 | `DFSStPer::dfs` | O(|V| + |E|) | O((|V| + |E|) log |V|) | AVLTreeSetStPer find/insert O(log n) |
| 3 | `CycleDetectStEph::has_cycle` | O(|V| + |E|) | O((|V| + |E|) log |V|) | Ancestors set uses AVL O(log n) ops |
| 4 | `CycleDetectStPer::has_cycle` | O(|V| + |E|) | O((|V| + |E|) log |V|) | AVL ops O(log n); ancestors clone O(|V|) worst case |
| 5 | `TopoSortStEph::topo_sort` | O(|V| + |E|) | O(|V|^2 + |E|) | `Vec::insert(0, ..)` is O(|V|) per finish |
| 6 | `TopoSortStPer::topo_sort` | O(|V| + |E|) | O(|V|^2 + (|V|+|E|) log |V|) | Vec::insert(0, ..) + AVL ops |
| 7 | `SCCStEph::scc` | O(|V| + |E|) | O(|V|^2 + (|V|+|E|) log |V|) | Vec::insert(0, ..) + AVL ops + component rebuild |
| 8 | `SCCStPer::scc` | O(|V| + |E|) | O(|V|^2 + (|V|+|E|) log |V|) | Vec::insert(0, ..) + AVL ops + union |

**Root cause of cost disagreements:**

1. **`Vec::insert(0, vertex)`**: Every finish operation prepends to a Vec, which is O(n). APAS Algorithm 55.13 uses sequence prepend `<v> @ Sigma` which is O(1) for a linked-list-backed sequence. The code uses `Vec` which has O(n) prepend. This inflates topo sort and SCC from O(|V| + |E|) to O(|V|^2 + |E|).

2. **AVL tree sets for visited/ancestors/result**: APAS Algorithm 55.7 uses array sequences (boolean arrays) for visited tracking, giving O(1) per operation. The `StEph` variants correctly use `ArraySeqStEphS<B>` for visited but then collect results into `AVLTreeSetStEph` which adds O(log n) per insert. The `StPer` variants use `AVLTreeSetStPer` for everything, giving O(log n) per operation.

3. **Component accumulation in SCC**: `SCCStEph::scc` rebuilds the entire components sequence each time a new component is found (copies all previous components into a new Vec), which is O(|V|) per component.

### 3b. Implementation Fidelity

| # | Function | Prose Algorithm | Fidelity | Notes |
|---|----------|-----------------|:--------:|-------|
| 1 | `DFSStEph::dfs` | Alg 55.2 + 55.7 | Partial | Uses array for visited (55.7) but AVL set for result (not in prose); result set is extra — prose just returns visited |
| 2 | `DFSStPer::dfs` | Alg 55.2 | Partial | Uses AVL set instead of functional set; returns visited set directly (matches prose) |
| 3 | `CycleDetectStEph::has_cycle` | Alg 55.10 | Good | Matches prose: maintains ancestors set, checks on revisit. Uses DFSAll loop over all vertices |
| 4 | `CycleDetectStPer::has_cycle` | Alg 55.10 | Good | Same structure as StEph but persistent. `ancestors.clone()` on each recursive call is correct for persistence |
| 5 | `TopoSortStEph::topo_sort` | Alg 55.13 | Good | Matches prose: DFSAll with finish prepending vertex to result. Separate `topological_sort_opt` adds cycle detection (beyond prose) |
| 6 | `TopoSortStPer::topo_sort` | Alg 55.13 | Good | Same as StEph but persistent |
| 7 | `SCCStEph::scc` | Alg 55.18 | Good | Matches prose: compute finish order, transpose, DFSReach on transposed graph |
| 8 | `SCCStPer::scc` | Alg 55.18 | Good | Same structure as StEph but persistent |

**Notable deviations:**

1. **DFS returns different type**: The prose `DFSReach` returns the visited set X. `DFSStEph::dfs` maintains both a boolean array (`visited`) for O(1) checking and an `AVLTreeSetStEph` for the result. This is redundant — the boolean array alone would suffice with a final conversion, or the AVL set alone could serve both purposes.

2. **No generic DFS**: The prose defines a generic DFS (Alg 55.4) with `visit`/`finish`/`revisit` callbacks, then specializes it. The code implements each specialization directly without a shared generic framework. This is pragmatic but means the cycle-detection, topo-sort, and SCC modules each have their own DFS recursive function.

3. **Stack-based DFS (Alg 55.1)**: Not implemented at all. Only the recursive variant exists.

4. **`topological_sort_opt`**: This function is not in the prose. It combines topological sort with cycle detection. A useful extension but not from APAS.

### 3c. Spec Fidelity

No Verus specifications exist in any Chap55 file. All functions are plain Rust with zero `requires`/`ensures`. There is nothing to compare against the prose's stated properties.

**Expected specs (if verusified):**

| # | Function | Expected ensures |
|---|----------|-----------------|
| 1 | `dfs` | Returns exactly the set of vertices reachable from source in G |
| 2 | `has_cycle` | Returns true iff there exists a cycle in the directed graph |
| 3 | `topo_sort` | Returns a permutation of vertices where for every edge (u,v), u appears before v |
| 4 | `scc` | Returns a partition of V into maximal strongly connected components in topological order |

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chap55.** This is correct per the prose: "Unlike BFS, DFS is inherently sequential, because it only visits one vertex at a time" and "DFS is P-complete" (Remark in Section 4.1).

No parallelism audit table is needed.

## Phase 5: Runtime Test Review

**No test files exist for Chap55.** The glob `tests/*Chap55*` returned zero results.

### 5a. Coverage Check

| # | Module | Test File | Status |
|---|--------|-----------|--------|
| 1 | `DFSStEph` | (none) | Missing |
| 2 | `DFSStPer` | (none) | Missing |
| 3 | `CycleDetectStEph` | (none) | Missing |
| 4 | `CycleDetectStPer` | (none) | Missing |
| 5 | `TopoSortStEph` | (none) | Missing |
| 6 | `TopoSortStPer` | (none) | Missing |
| 7 | `SCCStEph` | (none) | Missing |
| 8 | `SCCStPer` | (none) | Missing |

### 5b. Test Quality

N/A — no tests exist.

### 5c. Missing Tests (Priority)

All 8 modules need runtime tests. Since there are no Verus specs, runtime tests are the **only** evidence of correctness. Priority:

| # | Module | Priority | Reason |
|---|--------|:--------:|--------|
| 1 | `DFSStEph` | High | Foundation for cycle detect, topo sort, SCC |
| 2 | `DFSStPer` | High | Same |
| 3 | `CycleDetectStEph` | High | Core algorithm, no formal verification |
| 4 | `CycleDetectStPer` | Medium | Same algorithm, persistent variant |
| 5 | `TopoSortStEph` | High | Important application, includes `topological_sort_opt` |
| 6 | `TopoSortStPer` | Medium | Same algorithm, persistent variant |
| 7 | `SCCStEph` | High | Most complex algorithm in chapter, combines multiple sub-algorithms |
| 8 | `SCCStPer` | Medium | Same algorithm, persistent variant |

**Recommended test cases:**

- DFS: empty graph, single vertex, linear chain, complete graph, disconnected graph, Example 55.1 from prose
- Cycle detection: DAG (no cycle), simple cycle, self-loop, complex graph with cycle, Example 55.5
- Topo sort: linear chain, diamond DAG, wide DAG, Example 55.6 (dressing dependency graph)
- SCC: single component, all singletons, Example 55.10 from prose, verify topological ordering of components

## Phase 6: Proof-Time Test (PTT) Review

No PTTs exist for Chap55. No iterators or verified loops exist (no `verus!` blocks at all). **No PTTs are needed** until the modules are verusified.

### 6a. Unified Test Inventory Table

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | `DFSStEph` | (none) | (none) | Missing RTT |
| 2 | `DFSStPer` | (none) | (none) | Missing RTT |
| 3 | `CycleDetectStEph` | (none) | (none) | Missing RTT |
| 4 | `CycleDetectStPer` | (none) | (none) | Missing RTT |
| 5 | `TopoSortStEph` | (none) | (none) | Missing RTT |
| 6 | `TopoSortStPer` | (none) | (none) | Missing RTT |
| 7 | `SCCStEph` | (none) | (none) | Missing RTT |
| 8 | `SCCStPer` | (none) | (none) | Missing RTT |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Item | Description |
|---|------|-------------|
| 1 | Alg 55.1 | DFS with a Stack — not implemented |
| 2 | Alg 55.4 | Generic DFS (callback-based visit/finish/revisit) — not implemented as generic; specialized versions exist |
| 3 | Alg 55.5 | Generic DFSAll — not implemented as generic |
| 4 | Def 55.6 / DFS Numbers | DFS number generation (visit time / finish time tables) — not implemented as a standalone module |
| 5 | Lemma 55.1 | DFS Numbers edge classification — not proved |
| 6 | Lemma 55.2 | Bound on DFS calls — not proved |
| 7 | Thm 55.3 | Back edges iff cycles — not proved |
| 8 | Lemma 55.4 | DAG Finish Order — not proved |
| 9 | Lemma 55.5 | First Visited — not proved |
| 10 | Thm 55.6 | SCC Correctness — not proved |
| 11 | Ex 55.3-55.8 | All 6 exercises — none implemented |

### Code With No Prose Counterpart

| # | Function | Notes |
|---|----------|-------|
| 1 | `topological_sort_opt` (both Eph/Per) | Combines topo sort with cycle detection; not in prose but useful extension |
| 2 | `dfs_finish_order_cycle_detect` (both Eph/Per) | Helper for `topological_sort_opt`; not in prose |

## Phase 8: Table of Contents Review

**No file has a TOC block.** None of the 8 files have `// Table of Contents` or numbered section headers. All files are simple module structures without `verus!` blocks, so the TOC standard from `table-of-contents-standard.mdc` does not strictly apply (sections 1-11 are inside `verus!`). However, if these modules are verusified in the future, TOCs should be added.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | `CycleDetectStEph.rs` | - | - | - | - | - | - | - | - | - |
| 2 | `CycleDetectStPer.rs` | - | - | - | - | - | - | - | - | - |
| 3 | `DFSStEph.rs` | - | - | - | - | - | - | - | - | - |
| 4 | `DFSStPer.rs` | - | - | - | - | - | - | - | - | - |
| 5 | `SCCStEph.rs` | - | - | - | - | - | - | - | - | - |
| 6 | `SCCStPer.rs` | - | - | - | - | - | - | - | - | - |
| 7 | `TopoSortStEph.rs` | - | - | - | - | - | - | - | - | - |
| 8 | `TopoSortStPer.rs` | - | - | - | - | - | - | - | - | - |

No derive impls exist. These are algorithm modules (free functions), not data type modules.

## Proof Holes Summary

```
✓ CycleDetectStEph.rs
✓ CycleDetectStPer.rs
✓ DFSStEph.rs
✓ DFSStPer.rs
✓ SCCStEph.rs
✓ SCCStPer.rs
✓ TopoSortStEph.rs
✓ TopoSortStPer.rs

Modules: 8 clean, 0 holed
Proof Functions: 0 total
Holes Found: 0
```

No proof holes — but this is trivially true since there are no `verus!` blocks and no proof functions at all.

## Spec Strength Summary

| Classification | Count |
|----------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 26 |

All 26 functions have **no spec** (no `requires`/`ensures`). The entire chapter is unverified plain Rust.

## Overall Assessment

Chapter 55 implements the four core DFS applications from the prose (reachability, cycle detection, topological sort, SCC) in both ephemeral and persistent variants. The implementations are algorithmically correct in structure and follow the prose algorithms reasonably closely.

**Strengths:**
1. All four major algorithms are implemented (DFS, cycle detection, topo sort, SCC)
2. Both StEph and StPer variants exist for each
3. `topological_sort_opt` adds a useful cycle-detection extension beyond the prose
4. No proof holes (trivially, since no Verus code exists)
5. Code is clean, readable, and well-structured

**Weaknesses:**
1. **Zero Verus verification**: No `verus!` blocks, no specs, no proofs. This is the largest gap.
2. **Zero runtime tests**: No test files exist at all. There is no evidence of correctness beyond code review.
3. **Cost deviations**: `Vec::insert(0, ..)` inflates topo sort and SCC from O(|V| + |E|) to O(|V|^2 + |E|). This could be fixed by using `push` + reverse, or a linked-list-backed sequence.
4. **No generic DFS framework**: Alg 55.4 (generic DFS with callbacks) is not implemented. Each algorithm has its own copy of the DFS recursive function.
5. **Missing Alg 55.1**: Stack-based DFS is not implemented.
6. **No DFS numbers module**: Def 55.6 (visit/finish times) is not implemented as a standalone algorithm.
7. **No theorems/lemmas proved**: All 6 stated lemmas/theorems are unproved.
8. **No exercises implemented**: All 6 exercises are missing.

**Recommended next steps (priority order):**
1. Add runtime tests for all 8 modules
2. Fix `Vec::insert(0, ..)` cost issue in topo sort and SCC
3. Implement generic DFS framework (Alg 55.4/55.5) to reduce code duplication
4. Begin verusification starting with `DFSStEph` (simplest, foundation for others)
5. Add DFS numbers module
6. Implement stack-based DFS (Alg 55.1)
