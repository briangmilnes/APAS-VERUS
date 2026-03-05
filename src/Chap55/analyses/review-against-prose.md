<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 55 — Depth-First Search: Review Against Prose

**Date:** 2026-03-05 (updated from 2026-02-19)
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory Summary

| # | File | Exec fns | Spec fns | Proof fns | external_body | verus! | Notes |
|---|------|:--------:|:--------:|:---------:|:-------------:|:------:|-------|
| 1 | DFSStEph.rs | 2 | 0 | 0 | 0 | Yes | struct+trait+impl pattern |
| 2 | DFSStPer.rs | 2 | 0 | 0 | 0 | Yes | struct+trait+impl pattern |
| 3 | TopoSortStEph.rs | 4 | 9 | 1 | 0 | Yes | Graph spec definitions host |
| 4 | TopoSortStPer.rs | 4 | 8 | 0 | 0 | Yes | Per graph spec definitions |
| 5 | CycleDetectStEph.rs | 2 | 0 | 0 | 0 | Yes | struct+trait+impl pattern |
| 6 | CycleDetectStPer.rs | 2 | 1 | 0 | 0 | Yes | Local spec_wf_adj_list_per |
| 7 | SCCStEph.rs | 5 | 0 | 0 | 0 | Yes | struct+trait+impl pattern |
| 8 | SCCStPer.rs | 5 | 0 | 0 | 0 | Yes | struct+trait+impl pattern |

**Totals:** 8 files, 26 exec functions, 18 spec functions, 1 proof function, 0 `#[verifier::external_body]` holes.

All files are inside `verus! {}` blocks. All 8 modules use trait-impl pattern with unit structs. All trait methods have `requires`/`ensures`. Graph spec definitions (reachability, DAG, topo order, SCC) defined in TopoSortStEph/StPer as shared infrastructure.

**Cfg gating:** All 8 modules are commented out in `lib.rs` because they depend on unverified chapters: Chap37 (AVLTreeSeq) and Chap41 (AVLTreeSet). Cannot be compiled under Verus until dependencies are verified.

## Phase 2: Prose Inventory

Source: `prompts/Chap55.txt`.

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
| 2 | Alg 55.2 | DFS, Recursively (DFSReach) | Yes — DFSStEph::dfs, DFSStPer::dfs |
| 3 | Alg 55.4 | Generic DFS (visit/finish/revisit callbacks) | No directly; specialized versions exist |
| 4 | Alg 55.5 | Generic DFSAll | No directly; specialized in TopoSort and SCC |
| 5 | Alg 55.7 | DFS with Array Sequences | Partially — DFSStEph uses array for visited but AVL set for result |
| 6 | Alg 55.10 | Directed Cycle Detection | Yes — CycleDetectStEph::has_cycle, CycleDetectStPer::has_cycle |
| 7 | Alg 55.13 | Topological Sort | Yes — TopoSortStEph::topo_sort, TopoSortStPer::topo_sort |
| 8 | Alg 55.18 | Strongly Connected Components | Yes — SCCStEph::scc, SCCStPer::scc |

### Cost Specifications

| # | Item | Description |
|---|------|-------------|
| 1 | Cost 55.8 | DFS: O((m+n) log n) for tree-based sets/adj tables; O(m+n) for array sequences |

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

| # | Item | Description | Text proof? | Implemented? |
|---|------|-------------|:-----------:|:------------:|
| 1 | Ex 55.3 | Prove the DFS Numbers Lemma | Yes | No |
| 2 | Ex 55.4 | Restate DFS Numbers Lemma in terms of exploration intervals | Yes | No |
| 3 | Ex 55.5 | Design cycle finding for undirected graphs | No | No |
| 4 | Ex 55.6 | Prove topo sort Work/Span O(\|V\| + \|E\|) for enumerable graphs | Yes | No |
| 5 | Ex 55.7 | Give DFSReach for SCC in terms of generic DFS | No | No |
| 6 | Ex 55.8 | Work/Span of SCC algorithm | No | No |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

26 exec functions with APAS cost annotations. Cost disagreements below reflect data structure choices.

#### DFS

| # | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|------|----------|-----------|----------------------|:---------:|
| 1 | DFSStEph.rs | `dfs` | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | W O((\|V\|+\|E\|) log \|V\|), S same | ❌ AVL insert O(log n) |
| 2 | DFSStEph.rs | `dfs_recursive` | (internal helper) | W O(\|V\|+\|E\|) amort., S same | N/A |
| 3 | DFSStPer.rs | `dfs` | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | W O((\|V\|+\|E\|) log \|V\|), S same | ❌ AVL find/insert O(log n) |
| 4 | DFSStPer.rs | `dfs_recursive` | (internal helper) | W O(log \|V\|) per call | N/A |

#### Topological Sort

| # | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|------|----------|-----------|----------------------|:---------:|
| 5 | TopoSortStEph.rs | `topological_sort_opt` | W O(\|V\|+\|E\|), S same | W O(\|V\|+\|E\|), S same | ✅ push+reverse |
| 6 | TopoSortStEph.rs | `topo_sort` | W O(\|V\|+\|E\|), S same | W O(\|V\|+\|E\|), S same | ✅ push+reverse |
| 7 | TopoSortStEph.rs | `dfs_finish_order_cycle_detect` | (internal helper) | W O(\|V\|) per finish | N/A |
| 8 | TopoSortStEph.rs | `dfs_finish_order` | (internal helper) | W O(\|V\|) per finish | N/A |
| 9 | TopoSortStPer.rs | `topological_sort_opt` | W O(\|V\|+\|E\|), S same | W O((\|V\|+\|E\|) log \|V\|), S same | ⚠️ AVL O(log n) |
| 10 | TopoSortStPer.rs | `topo_sort` | W O(\|V\|+\|E\|), S same | W O((\|V\|+\|E\|) log \|V\|), S same | ⚠️ AVL O(log n) |
| 11 | TopoSortStPer.rs | `dfs_finish_order_cycle_detect` | (internal helper) | W O(\|V\|+log \|V\|) per call | N/A |
| 12 | TopoSortStPer.rs | `dfs_finish_order` | (internal helper) | W O(\|V\|+log \|V\|) per call | N/A |

#### Cycle Detection

| # | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|------|----------|-----------|----------------------|:---------:|
| 13 | CycleDetectStEph.rs | `has_cycle` | W O(\|V\|+\|E\|), S same | W O((\|V\|+\|E\|) log \|V\|), S same | ❌ AVL ops O(log n) |
| 14 | CycleDetectStEph.rs | `dfs_check_cycle` | (internal helper) | W O(log \|V\|) per call | N/A |
| 15 | CycleDetectStPer.rs | `has_cycle` | W O(\|V\|+\|E\|), S same | W O((\|V\|+\|E\|) log \|V\|), S same | ❌ AVL ops O(log n) |
| 16 | CycleDetectStPer.rs | `dfs_check_cycle` | (internal helper) | W O(log \|V\|) per call + clone | N/A |

#### Strongly Connected Components

| # | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|------|----------|-----------|----------------------|:---------:|
| 17 | SCCStEph.rs | `scc` | W O(\|V\|+\|E\|), S same | W O((\|V\|+\|E\|) log \|V\|), S same | ⚠️ AVL + rebuild |
| 18 | SCCStEph.rs | `compute_finish_order` | (internal helper) | W O(\|V\|+\|E\|), S same | push+reverse |
| 19 | SCCStEph.rs | `dfs_finish_order` | (internal helper) | W O(\|V\|) per finish | N/A |
| 20 | SCCStEph.rs | `transpose_graph` | (internal helper) | W O(\|V\|+\|E\|), S same | N/A |
| 21 | SCCStEph.rs | `dfs_reach` | (internal helper) | W O(deg(v)+log \|V\|) per call | N/A |
| 22 | SCCStPer.rs | `scc` | W O(\|V\|+\|E\|), S same | W O((\|V\|+\|E\|) log \|V\|), S same | ⚠️ AVL + union |
| 23 | SCCStPer.rs | `compute_finish_order` | (internal helper) | W O((\|V\|+\|E\|) log \|V\|), S same | push+reverse |
| 24 | SCCStPer.rs | `dfs_finish_order` | (internal helper) | W O(\|V\|+log \|V\|) per call | N/A |
| 25 | SCCStPer.rs | `transpose_graph` | (internal helper) | W O(\|V\|+\|E\|), S same | N/A |
| 26 | SCCStPer.rs | `dfs_reach` | (internal helper) | W O(deg(v) log \|V\|+\|comp\| log \|comp\|) per call | N/A |

**Root cause of cost disagreements:**

1. ~~**`Vec::insert(0, vertex)`**~~ **FIXED**: Now uses `push` + `reverse` pattern. O(1) amortized per insert.

2. **AVL tree sets for visited/ancestors/result**: APAS Algorithm 55.7 uses array sequences (boolean arrays) for visited tracking, giving O(1) per operation. The StEph variants correctly use `ArraySeqStEphS<B>` for visited but collect results into `AVLTreeSetStEph` which adds O(log n) per insert. The StPer variants use `AVLTreeSetStPer` for everything, giving O(log n) per operation.

3. **Component accumulation in SCC**: SCCStEph::scc rebuilds the entire components sequence each time a new component is found (copies all previous components into a new Vec), which is O(|V|) per component.

### 3b. Implementation Fidelity

| # | Function | Prose Algorithm | Fidelity | Notes |
|---|----------|-----------------|:--------:|-------|
| 1 | DFSStEph::dfs | Alg 55.2 + 55.7 | ⚠️ Partial | Array for visited (55.7) but AVL set for result (not in prose) |
| 2 | DFSStPer::dfs | Alg 55.2 | ⚠️ Partial | AVL set instead of functional set; returns visited set directly |
| 3 | CycleDetectStEph::has_cycle | Alg 55.10 | ✅ Good | Maintains ancestors set, checks on revisit. Uses DFSAll loop. |
| 4 | CycleDetectStPer::has_cycle | Alg 55.10 | ✅ Good | Same as StEph, persistent. `ancestors.clone()` is correct. |
| 5 | TopoSortStEph::topo_sort | Alg 55.13 | ✅ Good | DFSAll with finish prepending vertex to result. |
| 6 | TopoSortStPer::topo_sort | Alg 55.13 | ✅ Good | Same as StEph, persistent. |
| 7 | SCCStEph::scc | Alg 55.18 | ✅ Good | Compute finish order, transpose, DFSReach on transposed graph. |
| 8 | SCCStPer::scc | Alg 55.18 | ✅ Good | Same structure as StEph, persistent. |

**Notable deviations:**

1. **DFS returns different type**: Prose `DFSReach` returns visited set X. DFSStEph maintains both a boolean array and an AVLTreeSetStEph — redundant. The boolean array alone suffices with a final conversion.

2. **No generic DFS**: The prose defines generic DFS (Alg 55.4) with visit/finish/revisit callbacks, then specializes. The code implements each specialization directly — pragmatic but means each module has its own DFS recursive function.

3. **Stack-based DFS (Alg 55.1)**: Not implemented. Only the recursive variant exists.

4. **`topological_sort_opt`**: Combines topological sort with cycle detection. Not in the prose but a useful extension.

### 3c. Spec Fidelity

All trait methods now have `requires`/`ensures`. Graph spec definitions (Definitions 55.3, 55.11, 55.12, 55.14, 55.17) are formalized as spec fns in TopoSortStEph/StPer.

| # | Function | Spec Status | ensures |
|---|----------|:-----------:|---------|
| 1 | `dfs` | ✅ Strong | `reachable@.contains(v) <==> spec_reachable(graph, source, v)` |
| 2 | `has_cycle` | ✅ Strong | `has_cycle == !spec_is_dag(graph)` |
| 3 | `topo_sort` | ✅ Strong | `spec_is_dag(graph) ==> spec_is_topo_order(graph, order@)` |
| 4 | `topological_sort_opt` | ✅ Strong | `is_some <==> spec_is_dag`, `is_some ==> spec_is_topo_order` |
| 5 | `scc` | ⚠️ Partial | `components@.len() >= 1 || graph@.len() == 0` (full spec_is_scc deferred) |

**Graph spec definitions added:**

| # | Spec fn | APAS Reference | Type |
|---|---------|---------------|------|
| 1 | `spec_has_edge` | Edge predicate | `(graph, u, v) -> bool` |
| 2 | `spec_is_path` | Path as vertex sequence | `(graph, path) -> bool` |
| 3 | `spec_reachable` | Def 55.3 reachability | `(graph, u, v) -> bool` |
| 4 | `spec_is_dag` | Def 55.11 DAG | `(graph) -> bool` |
| 5 | `spec_is_topo_order` | Def 55.12 topological ordering | `(graph, order) -> bool` |
| 6 | `spec_strongly_connected` | Def 55.14 strongly connected | `(graph, vertices) -> bool` |
| 7 | `spec_is_scc` | Def 55.17 SCC decomposition | `(graph, components) -> bool` |

All specs are aspirational — cannot be verified until Chap37/41 dependencies are resolved and modules uncommented in lib.rs.

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chap55.** This is correct per the prose: "Unlike BFS, DFS is inherently sequential, because it only visits one vertex at a time" and "DFS is P-complete" (Remark in Section 4.1). No parallelism audit needed.

## Phase 5: RTT Review

### 5a. Coverage Check

| # | Source Module | Test File | Tests | Status |
|---|-------------|----------|:-----:|--------|
| 1 | DFSStEph | `tests/Chap55/TestDFSStEph.rs` | 6 | ✅ Present |
| 2 | DFSStPer | `tests/Chap55/TestDFSStPer.rs` | 7 | ✅ Present |
| 3 | TopoSortStEph | `tests/Chap55/TestTopoSortStEph.rs` | 11 | ✅ Present |
| 4 | TopoSortStPer | `tests/Chap55/TestTopoSortStPer.rs` | 11 | ✅ Present |
| 5 | CycleDetectStEph | `tests/Chap55/TestCycleDetectStEph.rs` | 7 | ✅ Present |
| 6 | CycleDetectStPer | `tests/Chap55/TestCycleDetectStPer.rs` | 7 | ✅ Present |
| 7 | SCCStEph | `tests/Chap55/TestSCCStEph.rs` | 7 | ✅ Present |
| 8 | SCCStPer | `tests/Chap55/TestSCCStPer.rs` | 7 | ✅ Present |

**Total: 63 tests across 8 files.**

### 5b. Test Quality

**DFS tests** cover: empty graph, single vertex, line graph, DAG, cycle, disconnected graph. DFSStPer also tests invalid source.

**CycleDetect tests** cover: empty graph, single node, linear chain (no cycle), simple cycle, self-loop, DAG (no cycle), DAG with back edge (cycle).

**TopoSort tests** cover: single node, linear DAG, branching DAG, cycle (returns None), self-loop (returns None), disconnected components, empty graph. Both `topological_sort_opt` and `topo_sort` are tested.

**SCC tests** cover: single node, two nodes no edges, simple cycle, two separate SCCs, linear DAG (each vertex is its own SCC), self-loop, complex graph with multiple SCCs.

Good coverage. Tests exercise the prose examples and edge cases.

### 5c. Missing Test Cases

| # | Missing Case | Priority | Notes |
|---|-------------|:--------:|-------|
| 1 | Prose Example 55.1 (named vertices s,a,b,c,d,e,f) | Low | Similar graphs are tested |
| 2 | Prose Example 55.10 (SCC worked example) | Medium | Would verify exact SCC output against prose |
| 3 | Topo sort output validation | Medium | Tests check length but not edge-order invariant |
| 4 | SCC topological order validation | Medium | Tests check count but not ordering property |

## Phase 6: PTT Review

**No PTTs needed.** Chap55 modules are commented out in lib.rs (depend on unverified Chap37/Chap41). PTTs become relevant when dependencies are verified and modules are uncommented.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Type | Notes |
|---|-----------|------|-------|
| 1 | Alg 55.1 — DFS with a Stack | Algorithm | Only recursive DFS exists |
| 2 | Alg 55.4 — Generic DFS (callbacks) | Algorithm | Specialized versions exist instead |
| 3 | Alg 55.5 — Generic DFSAll | Algorithm | Specialized versions exist instead |
| 4 | Def 55.6 — DFS Numbers | Module | Visit/finish time tables not implemented standalone |
| 5 | Lemma 55.1 — DFS Numbers edge classification | Theorem | Not proved |
| 6 | Lemma 55.2 — Bound on DFS calls | Theorem | Not proved |
| 7 | Thm 55.3 — Back edges iff cycles | Theorem | Not proved |
| 8 | Lemma 55.4 — DAG Finish Order | Theorem | Not proved |
| 9 | Lemma 55.5 — First Visited | Theorem | Not proved |
| 10 | Thm 55.6 — SCC Correctness | Theorem | Not proved |
| 11 | Ex 55.3–55.8 | Exercises | None implemented |

### Code with No Prose Counterpart

| # | Module | Function | Notes |
|---|--------|----------|-------|
| 1 | TopoSortStEph/StPer | `topological_sort_opt` | Combines topo sort with cycle detection; useful extension |
| 2 | TopoSortStEph/StPer | `dfs_finish_order_cycle_detect` | Helper for `topological_sort_opt` |

## Phase 8: TOC / In-Out Table

### TOC Presence

| # | File | TOC Present? | Notes |
|---|------|:------------:|-------|
| 1 | DFSStEph.rs | ✅ Yes | Sections 1, 2, 4, 8, 9 |
| 2 | DFSStPer.rs | ✅ Yes | Sections 1, 2, 4, 8, 9 |
| 3 | TopoSortStEph.rs | ✅ Yes | Sections 1, 2, 4, 6, 7, 8, 9 |
| 4 | TopoSortStPer.rs | ✅ Yes | Sections 1, 2, 4, 6, 8, 9 |
| 5 | CycleDetectStEph.rs | ✅ Yes | Sections 1, 2, 4, 8, 9 |
| 6 | CycleDetectStPer.rs | ✅ Yes | Sections 1, 2, 6, 8, 9 |
| 7 | SCCStEph.rs | ✅ Yes | Sections 1, 2, 4, 8, 9 |
| 8 | SCCStPer.rs | ✅ Yes | Sections 1, 2, 4, 8, 9 |

All 8 Chap55 files have TOC headers.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | DFSStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | DFSStPer.rs | - | - | - | - | - | - | - | - | - |
| 3 | TopoSortStEph.rs | - | - | - | - | - | - | - | - | - |
| 4 | TopoSortStPer.rs | - | - | - | - | - | - | - | - | - |
| 5 | CycleDetectStEph.rs | - | - | - | - | - | - | - | - | - |
| 6 | CycleDetectStPer.rs | - | - | - | - | - | - | - | - | - |
| 7 | SCCStEph.rs | - | - | - | - | - | - | - | - | - |
| 8 | SCCStPer.rs | - | - | - | - | - | - | - | - | - |

No derive impls. These are algorithm modules (free functions), not data type modules.

## Action Items

| # | Priority | Action | Status |
|---|:--------:|--------|:------:|
| 1 | ~~High~~ | ~~Add TOC comment blocks to all 8 Chap55 files~~ | ✅ Done |
| 2 | ~~High~~ | ~~Fix `Vec::insert(0, ..)` → `push` + `reverse`~~ | ✅ Already done |
| 3 | High | Uncomment in lib.rs when Chap37/Chap41 verified | Blocked |
| 4 | ~~Medium~~ | ~~Add trait-impl pattern (unit structs) to all 8 files~~ | ✅ Done |
| 5 | ~~Medium~~ | ~~Add `requires`/`ensures` to all trait methods~~ | ✅ Done |
| 6 | ~~Medium~~ | ~~Add graph spec definitions (reachability, DAG, etc.)~~ | ✅ Done |
| 7 | Medium | Strengthen SCC ensures to full `spec_is_scc` | Open |
| 8 | Medium | Replace AVLTreeSet with array-based visited in DFSStPer/CycleDetectStPer | Open |
| 9 | Medium | Fix SCCStEph component accumulation (rebuilds Vec every iteration) | Open |
| 10 | Low | Implement generic DFS framework (Alg 55.4/55.5) | Open |
| 11 | Low | Validate topo sort output order and SCC topological order in tests | Open |

## Proof Holes Summary

**Last verified:** 2026-03-05 (`veracity-review-proof-holes`)

**Modules:** 8 clean (no holes), 0 holed. 0 `external_body` holes, 1 verified proof function (`lemma_set_true_decreases_num_false`).

No proof holes exist. All exec functions have specs (`requires`/`ensures`) but cannot be verified until Chap37/41 dependencies are resolved and modules are uncommented in lib.rs.
