# Review Against Prose: Chapter 55 -- Depth-First Search

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap55.txt`
- Reference: APAS Chapter 55

## Phase 1: Inventory

8 source files, 31 exec functions total, 4 proof functions.

| # | Chap | File | Tr | IT | ML | V! | Holes |
|---|------|------|:--:|:--:|:--:|:--:|:-----:|
| 1 | 55 | DFSStEph.rs | 1 | 1 | 1 | 2 | 0 |
| 2 | 55 | DFSStPer.rs | 1 | 1 | 1 | 2 | 0 |
| 3 | 55 | CycleDetectStEph.rs | 1 | 1 | 1 | 2 | 0 |
| 4 | 55 | CycleDetectStPer.rs | 1 | 1 | 1 | 2 | 0 |
| 5 | 55 | SCCStEph.rs | 1 | 1 | 4 | 5 | 0 |
| 6 | 55 | SCCStPer.rs | 1 | 1 | 5 | 6 | 0 |
| 7 | 55 | TopoSortStEph.rs | 1 | 1 | 7 | 8 | 0 |
| 8 | 55 | TopoSortStPer.rs | 1 | 1 | 3 | 4 | 0 |

## Phase 2: Prose Inventory

### Definitions
- Def 55.3: Tree and Non-Tree Edges (tree, back, forward, cross)
- Def 55.6: DFS Numbers (visit time, finish time)
- Def 55.9: Cycle-Detection Problem
- Def 55.11: DAG (Directed Acyclic Graph)
- Def 55.12: Topological Sort (total ordering consistent with edges)
- Def 55.14: Strongly Connected Graph
- Def 55.15: Strongly Connected Components
- Def 55.16: Component DAG
- Def 55.17: SCC Problem (find SCCs in topological order)

### Algorithms
- Alg 55.1: DFS with a Stack
- Alg 55.2: DFS Recursively (DFSReach)
- Alg 55.4: Generic DFS (with visit/finish/revisit callbacks)
- Alg 55.5: DFSAll (DFS over all vertices)
- Alg 55.7: DFS with Array Sequences (enumerable graphs)
- Alg 55.10: Directed Cycle Detection (ancestor set tracking)
- Alg 55.13: Topological Sort (finish order)
- Alg 55.18: Strongly Connected Components (decreasingFinish + transpose + DFSReach)

### Cost Specifications
- Cost Spec 55.8: DFS on adjacency tables: O((m+n) lg n); on array sequences: O(m+n)

### Theorems/Lemmas
- Thm 55.3: Back Edges Imply Cycles
- Lemma 55.1: DFS Numbers classify edge types
- Lemma 55.2: Bound on DFS calls (n+m calls to DFS, n to visit/finish, m to revisit)
- Lemma 55.4: DAG Finish Order (reachable vertices finish first)
- Lemma 55.5: First Visited in component has largest finish time
- Thm 55.6: SCC Correctness

### Exercises
- Ex 55.3: Prove DFS Numbers Lemma
- Ex 55.4: Exploration intervals
- Ex 55.5: Undirected cycle detection
- Ex 55.6: Topological sort is O(|V| + |E|) for enumerable graphs
- Ex 55.7: DFSReach in terms of generic DFS
- Ex 55.8: SCC work and span analysis

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 8 files have APAS cost annotations on trait functions. Claude annotations added to DFSStEph.rs, CycleDetectStEph.rs, SCCStEph.rs, TopoSortStEph.rs in this review.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Ref | Fidelity |
|---|------|------|----------|-----------|:--------:|
| 1 | 55 | DFSStEph.rs | dfs | Alg 55.2/55.7 | Match |
| 2 | 55 | DFSStEph.rs | dfs_recursive | Alg 55.7 DFS | Match |
| 3 | 55 | CycleDetectStEph.rs | has_cycle | Alg 55.10 | Match |
| 4 | 55 | CycleDetectStEph.rs | dfs_check_cycle | Alg 55.10 DFS | Match |
| 5 | 55 | TopoSortStEph.rs | topo_sort | Alg 55.13 | Match |
| 6 | 55 | TopoSortStEph.rs | dfs_finish_order | Alg 55.13 DFS | Match |
| 7 | 55 | TopoSortStEph.rs | dfs_finish_order_cycle_detect | Combined topo+cycle | Extension |
| 8 | 55 | TopoSortStEph.rs | topological_sort_opt | Optional topo sort | Extension |
| 9 | 55 | SCCStEph.rs | scc | Alg 55.18 | Match |
| 10 | 55 | SCCStEph.rs | compute_finish_order | decreasingFinish | Match |
| 11 | 55 | SCCStEph.rs | transpose_graph | transpose G | Match |
| 12 | 55 | SCCStEph.rs | dfs_reach | DFSReach on GT | Match |
| 13 | 55 | SCCStEph.rs | check_wf_adj_list_eph | N/A scaffolding | N/A |

**DFS (DFSStEph.rs)**: Implements Algorithm 55.7 (DFS with Array Sequences). Uses boolean array for visited tracking and recursive traversal. The termination measure is `spec_num_false(visited@)` -- the count of unvisited vertices -- which strictly decreases on each visit. This is a clean and correct implementation.

**Cycle Detection (CycleDetectStEph.rs)**: Implements Algorithm 55.10. Maintains both a `visited` array and an `ancestors` array (the current DFS path). On entering a vertex, marks it in both arrays; on finishing, unmarks it from ancestors. If a neighbor is already in ancestors, a back edge (cycle) is detected. This matches the prose exactly.

**Topological Sort (TopoSortStEph.rs)**: Implements Algorithm 55.13. The `dfs_finish_order` function appends vertices to `finish_order` when they finish (after all descendants are processed). The main `topo_sort` function reverses the finish order to get topological order (latest finish first). The `topological_sort_opt` function combines topological sort with cycle detection, returning `None` if the graph has a cycle.

**SCC (SCCStEph.rs)**: Implements Algorithm 55.18 faithfully:
1. Compute finish order via DFS (decreasingFinish)
2. Transpose the graph
3. Iterate through vertices in decreasing finish order, performing DFSReach on the transposed graph

The `check_wf_adj_list_eph` function is a runtime validation that the transposed graph has valid neighbor indices, which is needed because the transpose_graph function does not carry a proof of well-formedness.

### 3c. Spec Fidelity

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 55 | DFSStEph.rs | dfs | Strong | Full reachability spec |
| 2 | 55 | CycleDetectStEph.rs | has_cycle | Strong | ensures == !spec_is_dag |
| 3 | 55 | TopoSortStEph.rs | topo_sort | Strong | ensures spec_is_topo_order |
| 4 | 55 | TopoSortStEph.rs | topological_sort_opt | Strong | is_some <==> spec_is_dag |
| 5 | 55 | SCCStEph.rs | scc | Partial | Only ensures non-empty |

**DFS spec is strong**: The ensures clause states `forall|v| reachable@.contains(v as usize) <==> spec_reachable(graph, source, v)`, which is exactly the reachability theorem. This is the strongest spec in the graph chapter family.

**Cycle detection spec is strong**: `has_cycle == !spec_is_dag(graph)` directly encodes Theorem 55.3.

**Topological sort spec is strong**: `spec_is_dag(graph) ==> spec_is_topo_order(graph, order@)` with `spec_is_topo_order` requiring that all edges go forward in the ordering.

**SCC spec is partial**: `components@.len() >= 1 || graph@.len() == 0` only guarantees non-emptiness. The full SCC spec (`spec_is_scc`) is defined in spec fns but not connected to the ensures clause. Connecting it would require proving that the components partition all vertices, are each strongly connected, and are in topological order -- all of which match the prose (Theorem 55.6) but are not yet verified.

**Spec functions defined but not all connected to exec**:
- `spec_is_path`, `spec_reachable`, `spec_is_dag`, `spec_is_topo_order`, `spec_strongly_connected`, `spec_is_scc` -- all defined in TopoSortStEph.rs
- `spec_reachable` is used by DFSStEph.rs ensures (strong)
- `spec_is_dag` is used by CycleDetectStEph.rs and TopoSortStEph.rs ensures (strong)
- `spec_is_topo_order` is used by TopoSortStEph.rs ensures (strong)
- `spec_is_scc` is defined but NOT used in SCCStEph.rs ensures (gap)

## Phase 4: Parallelism Review

| # | Chap | File | Classification | Notes |
|---|------|------|:--------------:|-------|
| 1 | 55 | All 8 files | Sequential | DFS is inherently sequential |

The prose explicitly states (Section 4.1): "DFS is P-complete... unlikely to be highly parallel." All implementations are sequential, which is correct. There are no Mt variants because DFS, cycle detection, topological sort, and SCC are all inherently sequential algorithms.

**No parallelism gap**: The prose confirms DFS is sequential and the implementation matches.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Module Under Test |
|---|------|-----------|-------------------|
| 1 | 55 | TestDFSStEph.rs | DFSStEph |
| 2 | 55 | TestDFSStPer.rs | DFSStPer |
| 3 | 55 | TestCycleDetectStEph.rs | CycleDetectStEph |
| 4 | 55 | TestCycleDetectStPer.rs | CycleDetectStPer |
| 5 | 55 | TestSCCStEph.rs | SCCStEph |
| 6 | 55 | TestSCCStPer.rs | SCCStPer |
| 7 | 55 | TestTopoSortStEph.rs | TopoSortStEph |
| 8 | 55 | TestTopoSortStPer.rs | TopoSortStPer |

All 8 modules have RTT coverage. Complete coverage.

## Phase 6: PTT Review

No proof-time tests for Chap55. None required -- no iterators or complex callability patterns.

## Phase 7: Gap Analysis

### Prose items not implemented

| # | Chap | Prose Item | Status |
|---|------|-----------|--------|
| 1 | 55 | Alg 55.1: DFS with Stack | Not implemented (recursive version used) |
| 2 | 55 | Alg 55.4: Generic DFS (visit/finish/revisit callbacks) | Not implemented |
| 3 | 55 | Alg 55.5: DFSAll (iterate over all vertices) | Implemented inline in topo_sort/scc |
| 4 | 55 | Def 55.6: DFS Numbers (visit/finish times) | Not implemented as separate module |
| 5 | 55 | Lemma 55.1: DFS Numbers classify edges | Not implemented |
| 6 | 55 | Ex 55.5: Undirected cycle detection | Not implemented |
| 7 | 55 | SCC ensures: spec_is_scc postcondition | Defined but not connected |

**Key gap**: The generic DFS algorithm (Alg 55.4) with configurable visit/finish/revisit callbacks is not implemented. Instead, each application (reachability, cycle detection, topological sort, SCC) has its own specialized DFS. This is a reasonable engineering choice since the generic version with closures would be harder to verify in Verus, but it means code duplication across the four DFS-based algorithms.

**DFS Numbers**: The prose defines visit and finish times as DFS numbers (Def 55.6) and uses them to classify edge types (Lemma 55.1). These are not implemented as a standalone module but the finish-order concept is used directly by topological sort and SCC.

### Code with no prose counterpart

| # | Chap | File | Item | Notes |
|---|------|------|------|-------|
| 1 | 55 | TopoSortStEph.rs | spec_num_false | Termination measure |
| 2 | 55 | TopoSortStEph.rs | lemma_set_true_decreases_num_false | Termination proof |
| 3 | 55 | TopoSortStEph.rs | lemma_set_true_num_false_eq | Counting lemma |
| 4 | 55 | TopoSortStEph.rs | lemma_all_true_num_false_zero | Terminal case |
| 5 | 55 | TopoSortStEph.rs | lemma_all_false_num_false_eq_len | Initial case |
| 6 | 55 | TopoSortStEph.rs | topological_sort_opt | Combined topo+cycle |
| 7 | 55 | TopoSortStEph.rs | dfs_finish_order_cycle_detect | Combined DFS |
| 8 | 55 | SCCStEph.rs | check_wf_adj_list_eph | Runtime wf check |

The `spec_num_false` function and its four lemmas are Verus-specific infrastructure for proving DFS termination. Since Verus requires decreases clauses for recursive functions, counting unvisited vertices provides the termination measure. This is a clean pattern reused across DFS, cycle detection, topological sort, and SCC.

The `topological_sort_opt` function is an extension that combines topological sort with cycle detection, returning `Option<...>` -- this is not in the prose but is a practical improvement.

## Phase 8: TOC Review

All 8 files follow the standard TOC ordering. TopoSortStEph.rs has sections 4, 6, 7, 8, 9 (the most complex). DFSStEph.rs is minimal with sections 4, 8, 9. No violations.

**Warning**: SCCStEph.rs and SCCStPer.rs have `requires true` on `check_wf_adj_list_eph`/`check_wf_adj_list_per` which triggers a veracity `requires_true` warning. This is intentional (runtime validation function) but should use `// accept vacuous` or simply omit the requires clause.

## Proof Holes Summary

| # | Chap | File | Holes | Warnings |
|---|------|------|:-----:|:--------:|
| 1 | 55 | DFSStEph.rs | 0 | 0 |
| 2 | 55 | DFSStPer.rs | 0 | 0 |
| 3 | 55 | CycleDetectStEph.rs | 0 | 0 |
| 4 | 55 | CycleDetectStPer.rs | 0 | 0 |
| 5 | 55 | SCCStEph.rs | 0 | 1 (requires_true) |
| 6 | 55 | SCCStPer.rs | 0 | 1 (requires_true) |
| 7 | 55 | TopoSortStEph.rs | 0 | 0 |
| 8 | 55 | TopoSortStPer.rs | 0 | 0 |
| | | **Total** | **0** | **2** |

All 8 modules are clean (0 holes). 2 warnings for vacuous `requires true` in SCC check_wf functions.

## Overall Assessment

Chapter 55 is clean with 0 holes across all 8 modules implementing 4 distinct algorithms (DFS, Cycle Detection, Topological Sort, SCC). The implementation quality is high:

**Strongest specs**: DFS has a full reachability spec (`spec_reachable`), cycle detection has `spec_is_dag`, and topological sort has `spec_is_topo_order`. These are among the strongest algorithmic specs in the project.

**SCC gap**: The SCC ensures clause is weak (`components@.len() >= 1`), despite having the full `spec_is_scc` predicate defined. Connecting `scc` ensures to `spec_is_scc` would require proving the partition, strong connectivity, and topological order properties from Theorem 55.6.

**Design choice**: Using specialized DFS variants rather than the generic Alg 55.4 with callbacks avoids closure verification complexity. Each variant has its own DFS recursive function with application-specific state (ancestors for cycle detection, finish_order for topo sort, component set for SCC).

**Termination proof**: The `spec_num_false(visited@)` measure is elegant -- it counts unvisited vertices and decreases by exactly 1 on each visit, providing a clean termination argument shared across all four algorithms.

**Missing prose items**: DFS Numbers (Def 55.6), Generic DFS with callbacks (Alg 55.4), and stack-based DFS (Alg 55.1) are not implemented. These are less critical than the four algorithms that are implemented.
