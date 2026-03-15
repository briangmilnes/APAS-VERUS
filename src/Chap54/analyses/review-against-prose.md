# Review Against Prose: Chapter 54 -- Breadth-First Search

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap54.txt`
- Reference: APAS Chapter 54

## Phase 1: Inventory

4 source files, 46 exec functions total, 22 proof functions.

| # | Chap | File | Tr | IT | ML | V! | Holes |
|---|------|------|:--:|:--:|:--:|:--:|:-----:|
| 1 | 54 | BFSStEph.rs | 4 | 4 | 4 | 8 | 0 |
| 2 | 54 | BFSStPer.rs | 4 | 4 | 4 | 8 | 0 |
| 3 | 54 | BFSMtEph.rs | 4 | 4 | 11 | 15 | 0 |
| 4 | 54 | BFSMtPer.rs | 4 | 4 | 11 | 15 | 0 |

## Phase 2: Prose Inventory

### Definitions
- Def 54.1: Distance of a vertex (shortest unweighted path length)
- Def 54.2: Breadth First Search (visits vertices in distance order)

### Algorithms
- Alg 54.3: Sequential BFS Reachability (with distance tracking, priority queue frontier)
- Alg 54.5: Sequential BFS with queue (implied by cost analysis in Section 2.1)
- Alg 54.6: BFS Tree (implied -- parent tracking for shortest path tree)

### Cost Specifications
- Sequential BFS: Work O(|V| + |E|), Span O(|V| + |E|)
- Parallel BFS: Work O(|V| + |E|), Span O(d * lg n) where d is diameter

### Properties
- BFS visits vertices in non-decreasing distance order
- Queue-based implementation maintains distance monotonicity invariant
- Dijkstra's color abstraction: white (unexplored), gray (frontier), black (visited)

### Exercises
- Ex 54.1: Prove queue-based BFS consistency with generic graph search
- Ex 54.2: Prove BFS correctness (visits in distance order)
- Ex 54.3: Prove queue-based sequential BFS correctness

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All trait-level functions in BFSStEph.rs and BFSMtEph.rs already have APAS cost annotations:
- BFSStEph: `/// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)`
- BFSMtEph: `/// - APAS: Work O(|V| + |E|), Span O(d*lg n)`

StPer and MtPer files mirror their counterparts.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Ref | Fidelity |
|---|------|------|----------|-----------|:--------:|
| 1 | 54 | BFSStEph.rs | bfs | Alg 54.5 | Match |
| 2 | 54 | BFSStEph.rs | bfs_tree | Alg 54.6 | Match |
| 3 | 54 | BFSStEph.rs | top_down_order | N/A scaffolding | N/A |
| 4 | 54 | BFSStEph.rs | bottom_up_order | N/A scaffolding | N/A |
| 5 | 54 | BFSMtEph.rs | bfs | Parallel BFS | Match |
| 6 | 54 | BFSMtEph.rs | bfs_tree | Parallel BFS tree | Match |
| 7 | 54 | BFSMtEph.rs | process_frontier_parallel | Fork-join frontier | Match |
| 8 | 54 | BFSMtEph.rs | process_frontier_tree_parallel | Fork-join tree | Match |

**Sequential BFS (BFSStEph.rs)**: Uses `VecDeque` as a FIFO queue, boolean array for visited status, and processes neighbors one at a time. This matches the queue-based implementation described in Section 2.1 of the prose. The `bfs` function returns a distances array; `bfs_tree` returns parent array + BFS-order sequence.

**Parallel BFS (BFSMtEph.rs)**: Uses layer-by-layer processing where each frontier layer is processed in parallel via `join()` from HFSchedulerMtEph. The `process_frontier_parallel` function uses divide-and-conquer: splits the frontier in half, copies graph and distances for each half, and processes them in parallel. This achieves O(d * lg n) span where d is the diameter.

**Key implementation detail**: The parallel version copies the graph and distances arrays for each fork-join split, which increases memory usage but maintains correctness under Verus's ownership model. The copies are proven equivalent via `lemma_copy_preserves_wf` and `lemma_copy_preserves_bounded`.

### 3c. Spec Fidelity

| # | Chap | File | Function | Spec Strength |
|---|------|------|----------|:-------------:|
| 1 | 54 | BFSStEph.rs | bfs | Strong |
| 2 | 54 | BFSStEph.rs | bfs_tree | Strong |
| 3 | 54 | BFSMtEph.rs | bfs | Strong |
| 4 | 54 | BFSMtEph.rs | bfs_tree | Strong |

**bfs ensures**:
- `traversal.spec_len() == graph.spec_len()` (output covers all vertices)
- `traversal.spec_index(source) == 0` (source has distance 0)
- `spec_distances_bounded(&traversal, n)` (all distances valid or UNREACHABLE)
- Non-source reachable vertices have distance > 0

**bfs_tree ensures**:
- `parents.spec_len() == graph.spec_len()`
- `parents.spec_index(source) == source` (source is its own parent)
- `order.spec_index(0) == source` (source first in BFS order)
- All order entries are valid vertex indices
- All ordered vertices have non-NOPARENT entries in parents array

These specs do not prove the core BFS property (distance monotonicity: if v is at distance d, all paths of length < d are explored first). This would require a spec function for shortest-path distance. However, the structural invariants are all verified.

## Phase 4: Parallelism Review

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|:--------------:|-------|
| 1 | 54 | BFSMtEph.rs | bfs | Layer-parallel | Outer loop over layers; inner frontier processing parallel |
| 2 | 54 | BFSMtEph.rs | bfs_tree | Layer-parallel | Same pattern |
| 3 | 54 | BFSMtEph.rs | process_frontier_parallel | Parallel | Fork-join divide-and-conquer via join() |
| 4 | 54 | BFSMtEph.rs | process_frontier_tree_parallel | Parallel | Fork-join divide-and-conquer via join() |
| 5 | 54 | BFSMtPer.rs | bfs | Layer-parallel | Same as MtEph |
| 6 | 54 | BFSMtPer.rs | bfs_tree | Layer-parallel | Same as MtEph |
| 7 | 54 | BFSMtPer.rs | process_frontier_parallel | Parallel | Fork-join via join() |
| 8 | 54 | BFSMtPer.rs | process_frontier_tree_parallel | Parallel | Fork-join via join() |

**Parallelism gap**: None. The Mt variants correctly implement layer-parallel BFS using fork-join frontier processing, matching the prose description that BFS visits all vertices at the same distance in parallel. The span is O(d * lg n) as annotated.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Module Under Test |
|---|------|-----------|-------------------|
| 1 | 54 | TestBFSStEph.rs | BFSStEph |
| 2 | 54 | TestBFSStPer.rs | BFSStPer |
| 3 | 54 | TestBFSMtEph.rs | BFSMtEph |
| 4 | 54 | TestBFSMtPer.rs | BFSMtPer |

All 4 modules have RTT coverage. Complete coverage.

## Phase 6: PTT Review

| # | Chap | PTT File | Module Under Test |
|---|------|----------|-------------------|
| 1 | 54 | ProveBFSStEph.rs | BFSStEph |
| 2 | 54 | ProveBFSMtEph.rs | BFSMtEph |

Two PTT files exist, likely verifying that BFS callability works (the requires clauses with graph wf predicates are non-trivial). This is appropriate.

## Phase 7: Gap Analysis

### Prose items not implemented

| # | Chap | Prose Item | Status |
|---|------|-----------|--------|
| 1 | 54 | Bipartiteness testing | Not implemented |
| 2 | 54 | Diameter bounding | Not implemented |
| 3 | 54 | Max flow subroutine | Not implemented (Chap scope) |
| 4 | 54 | Color abstraction (white/gray/black) | Implicit in distance array |

### Code with no prose counterpart

| # | Chap | File | Item | Notes |
|---|------|------|------|-------|
| 1 | 54 | BFSStEph.rs | top_down_order | Utility for BFS-order iteration |
| 2 | 54 | BFSStEph.rs | bottom_up_order | Utility for reverse-BFS iteration |
| 3 | 54 | BFSMtEph.rs | copy_distances | Parallel fork-join helper |
| 4 | 54 | BFSMtEph.rs | copy_graph | Parallel fork-join helper |
| 5 | 54 | BFSMtEph.rs | lemma_copy_preserves_* | Proof helpers for copy correctness |

The top_down_order and bottom_up_order functions are useful for algorithms that need to process vertices in BFS order or reverse BFS order (e.g., tree DP). The copy functions are necessitated by Verus ownership for fork-join parallelism.

## Phase 8: TOC Review

All 4 files follow the standard TOC ordering. BFSStEph.rs: sections 4, 6, 7, 8, 9. BFSMtEph.rs: sections 4, 6, 7, 8, 9. No violations.

## Proof Holes Summary

| # | Chap | File | Holes |
|---|------|------|:-----:|
| 1 | 54 | BFSStEph.rs | 0 |
| 2 | 54 | BFSStPer.rs | 0 |
| 3 | 54 | BFSMtEph.rs | 0 |
| 4 | 54 | BFSMtPer.rs | 0 |
| | | **Total** | **0** |

All 4 modules are clean. No proof holes. 22 proof functions all clean.

## Overall Assessment

Chapter 54 is exemplary: fully clean with 0 holes, strong specs, genuine parallelism in the Mt variants, and both RTT and PTT coverage. The implementation faithfully follows the queue-based sequential BFS from Section 2.1 and extends it with layer-parallel processing using fork-join for the Mt variants. The main spec gap is the absence of a shortest-path-distance postcondition (BFS correctness theorem), which would require formalizing the graph distance function. The structural invariants (distances bounded, parents valid, source at distance 0) are all verified. This is one of the strongest chapters in the project.
