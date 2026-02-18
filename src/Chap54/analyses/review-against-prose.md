<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 54 — Breadth-First Search: Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory Summary

| # | File | Functions | external_body | verus! | Notes |
|---|------|:---------:|:-------------:|:------:|-------|
| 1 | BFSStEph.rs | 1 | 1 | Yes | Sequential ephemeral BFS |
| 2 | BFSStPer.rs | 1 | 1 | Yes | Sequential persistent BFS |
| 3 | BFSMtEph.rs | 1 | 1 | Yes | Parallel ephemeral BFS (sequential impl) |
| 4 | BFSMtPer.rs | 2 | 2 | Yes | Parallel persistent BFS with thread::spawn |

**Totals:** 4 files, 5 exec functions, 5 `#[verifier::external_body]` holes, 0 verified functions.

All files are inside `verus! {}` blocks. All exec functions are `#[verifier::external_body]`. No specs (`requires`/`ensures`) on any function. Chap54 is NOT gated with `not(verus_keep_ghost)` — it compiles under Verus but verifies trivially.

## Phase 2: Prose Inventory

Source: `prompts/Chap54.txt`.

### Definitions

| # | Name | Description |
|---|------|-------------|
| 1 | Definition 54.1 — Distance of a Vertex | δ_G(s,v) = shortest distance from s to v (edge count) |
| 2 | Definition 54.2 — Breadth First Search | Graph search visiting vertices in increasing distance order |

### Algorithms

| # | Name | Description | Implemented? |
|---|------|-------------|:------------:|
| 1 | Algorithm 54.3 — Sequential BFS: Reachability | Queue-based BFS; returns reachable set X and max distance | Yes (BFSStEph, BFSStPer) |
| 2 | Algorithm 54.4 — Parallel BFS: Reachability | Layer-by-layer BFS; returns X and max distance | Partially (BFSMtEph, BFSMtPer) |
| 3 | Algorithm 54.5 — Unweighted Shortest Paths (BFSDistance) | Parallel BFS returning table mapping each reachable vertex to its distance | No |
| 4 | Algorithm 54.6 — BFS Tree with Sequences | Parallel BFS computing shortest-path tree via inject; returns parent sequence | No |

### Cost Specs

| # | Algorithm | Work | Span | Notes |
|---|-----------|------|------|-------|
| 1 | Sequential BFS (54.3, queue-based) | O(\|V\| + \|E\|) | O(\|V\| + \|E\|) | Sequential — Span == Work |
| 2 | Parallel BFS (54.4, set-based) | O(m·lg n) | O(d·lg² n) | Tree-based sets; d = max distance |
| 3 | Parallel BFS (54.6, sequence-based) | O(n + m) | O(d·lg n) | Ephemeral array sequences |

### Theorems/Properties

| # | Name | Description | Proved? |
|---|------|-------------|:-------:|
| 1 | Lemma 54.1 — Parallel BFS and Distances | At round i: X = {v : δ(s,v) < i}, F = {v : δ(s,v) = i} | No |
| 2 | Theorem 54.2 — BFS Tree Gives Shortest Paths | Path from v to s in BFS tree reversed is a shortest path | No |

### Exercises

| # | Name | Description | Text proof? | Implemented? |
|---|------|-------------|:-----------:|:------------:|
| 1 | Exercise 54.1 | Prove frontier update is consistent with generic graph search | Yes | No |
| 2 | Exercise 54.2 | Prove sequential BFS correctness (visits in distance order) | Yes | No |
| 3 | Exercise 54.3 | Prove queue-based sequential BFS is correct | Yes | No |
| 4 | Exercise 54.4 | Identify which frontiers contribute neighbors in N_G(F_i) | Yes | No |

All exercises are text proofs; none require code implementations.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|------|----------|-----------|----------------------|:---------:|
| 1 | BFSStEph.rs | `bfs` | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | ✅ Agrees |
| 2 | BFSStPer.rs | `bfs` | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | ✅ Agrees |
| 3 | BFSMtEph.rs | `bfs` | W O(\|V\|+\|E\|), S O(d·lg n) | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | ❌ Disagrees |
| 4 | BFSMtPer.rs | `bfs` | W O(\|V\|+\|E\|), S O(d·lg n) | W O(\|V\|+\|E\|), S O(d·lg \|F_i\|) | ⚠️ Partial |
| 5 | BFSMtPer.rs | `process_layer_parallel` | N/A (scaffolding) | W O(\|F_i\|), S O(lg \|F_i\|) | N/A |

**Disagreement details:**
- **BFSMtEph.rs `bfs`**: Despite being in an Mt module, the implementation processes each layer vertex sequentially in a `for` loop. No `thread::spawn` or parallel primitive is used. Span equals Work.
- **BFSMtPer.rs `bfs`**: Uses `process_layer_parallel` with `thread::spawn` fork-join for neighbor collection, achieving parallelism within layers. However, distance updates after each layer are applied sequentially via a `for` loop over `distance_updates`.

### 3b. Implementation Fidelity

| # | Module | APAS Algorithm | Fidelity | Notes |
|---|--------|----------------|:--------:|-------|
| 1 | BFSStEph | Alg 54.3 (Sequential BFS) | ✅ Faithful | Queue-based BFS with distance array. Uses `VecDeque`. Matches prose: pop front, mark visited, push unvisited neighbors. |
| 2 | BFSStPer | Alg 54.3 (Sequential BFS) | ✅ Faithful | Same algorithm as StEph but uses persistent `ArraySeqStPerS::update` instead of mutable `set`. |
| 3 | BFSMtEph | Alg 54.4 (Parallel BFS) | ❌ Sequential | Layer-by-layer structure matches prose, but within each layer, vertices are processed sequentially. No parallelism despite Mt name. |
| 4 | BFSMtPer | Alg 54.4 (Parallel BFS) | ⚠️ Partial | Layer-by-layer with genuine fork-join in `process_layer_parallel`. But deduplication and distance updates are sequential. |

**Deviation: Return type.** The prose returns `(X, max_distance)`. The implementations return a distance array (UNREACHABLE for unvisited vertices). The reachable set and max distance can be derived from this, so this is a representation choice.

**Deviation: Missing algorithms.** Algorithms 54.5 (BFSDistance) and 54.6 (BFS Tree with Sequences) are not implemented.

### 3c. Spec Fidelity

All functions have `#[verifier::external_body]` and **no specs** (`requires`/`ensures`). No preconditions are expressed (e.g., `source < graph.length()` is checked at runtime but not specified). No postconditions are expressed.

**Spec fidelity: N/A — no specs exist to compare against prose.**

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | Module | Function | Classification | Evidence |
|---|--------|----------|:--------------:|----------|
| 1 | BFSMtEph | `bfs` | Sequential | No `thread::spawn`, no `join`. Sequential `for` loop within layers. |
| 2 | BFSMtPer | `bfs` | Parallel (partial) | Calls `process_layer_parallel` which uses `thread::spawn`. Distance update loop is sequential. |
| 3 | BFSMtPer | `process_layer_parallel` | Parallel | Fork-join via `thread::spawn` + `join`. Splits layer in half recursively. |

### 4b. Span Audit

| # | Module | Function | APAS Span | Actual Span | Match? | Notes |
|---|--------|----------|-----------|-------------|:------:|-------|
| 1 | BFSMtEph | `bfs` | O(d·lg n) | O(\|V\|+\|E\|) | ❌ | Span == Work. No parallelism. |
| 2 | BFSMtPer | `bfs` | O(d·lg n) | O(d·lg \|F_i\|) | ⚠️ | Neighbor collection parallel; distance updates sequential per layer. |
| 3 | BFSMtPer | `process_layer_parallel` | N/A | O(lg \|F_i\|) | N/A | Scaffolding function. |

## Phase 5: RTT Review

### 5a. Coverage Check

| # | Source Module | Test File | Tests | Status |
|---|-------------|----------|:-----:|--------|
| 1 | BFSStEph | `tests/Chap54/TestBFSStEph.rs` | 7 | ✅ Present |
| 2 | BFSStPer | `tests/Chap54/TestBFSStPer.rs` | 7 | ✅ Present |
| 3 | BFSMtEph | `tests/Chap54/TestBFSMtEph.rs` | 7 | ✅ Present |
| 4 | BFSMtPer | `tests/Chap54/TestBFSMtPer.rs` | 7 | ✅ Present |

### 5b. Test Quality

All 4 test files cover:
- Empty graph
- Single vertex
- Line graph (chain)
- DAG (Example 54.2 from prose)
- Unreachable vertices (disconnected graph)
- Cycle
- Invalid source (out of range)

Good coverage of edge cases. Tests verify exact distance values. The Example 54.2 graph from the prose is used as a test case.

### 5c. Missing Test Cases

| # | Missing Case | Priority | Notes |
|---|-------------|:--------:|-------|
| 1 | Example 54.1 graph (named vertices) | Low | DAG test covers similar structure |
| 2 | Complete graph (d=1) | Low | Would verify all distances are 1 |
| 3 | Large graph stress test | Low | Would catch performance regressions |

## Phase 6: PTT Review

**No PTTs needed.** All exec functions are `#[verifier::external_body]` — there are no verified loops, no iterators with ghost state, and no Verus-verified patterns to exercise. PTTs become relevant only when `external_body` wrappers are replaced with verified implementations.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Type | Notes |
|---|-----------|------|-------|
| 1 | Algorithm 54.5 — BFSDistance | Algorithm | Returns table mapping vertices to distances |
| 2 | Algorithm 54.6 — BFS Tree with Sequences | Algorithm | Computes shortest-path tree via inject |
| 3 | Lemma 54.1 — Parallel BFS and Distances | Theorem | X_i and F_i invariants |
| 4 | Theorem 54.2 — BFS Tree Gives Shortest Paths | Theorem | Shortest path property |
| 5 | Exercise 54.1–54.4 | Text proofs | Not formalized |

### Code with No Prose Counterpart

| # | Module | Function | Purpose |
|---|--------|----------|---------|
| 1 | BFSMtPer | `process_layer_parallel` | Verus-specific scaffolding: fork-join helper for parallel layer processing |

## Phase 8: TOC / In-Out Table

### TOC Presence

| # | File | TOC Present? | Sections |
|---|------|:------------:|----------|
| 1 | BFSStEph.rs | ✅ Yes | 4, 8, 9 |
| 2 | BFSStPer.rs | ✅ Yes | 4, 8, 9 |
| 3 | BFSMtEph.rs | ✅ Yes | 4, 8, 9 |
| 4 | BFSMtPer.rs | ✅ Yes | 4, 8, 9 |

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BFSStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | BFSStPer.rs | - | - | - | - | - | - | - | - | - |
| 3 | BFSMtEph.rs | - | - | - | - | - | - | - | - | - |
| 4 | BFSMtPer.rs | - | - | - | - | - | - | - | - | - |

No derive impls in any file. The BFS modules define only type aliases, traits, and algorithmic functions, not data types.

## Action Items

| # | Priority | Action | Notes |
|---|:--------:|--------|-------|
| 1 | High | Remove `external_body` from BFSStEph::bfs and add `requires`/`ensures` | Simplest starting point for verification |
| 2 | High | Remove `external_body` from BFSStPer::bfs | Same algorithm, persistent |
| 3 | Medium | Make BFSMtEph genuinely parallel or rename/document as sequential-layered | Currently sequential despite Mt name |
| 4 | Medium | Add `requires`/`ensures` to BFSMtPer::`bfs` and `process_layer_parallel` | Define correctness specs |
| 5 | Low | Implement Algorithm 54.6 (BFS Tree with Sequences) | Most algorithmically interesting variant |
| 6 | Low | Implement Algorithm 54.5 (BFSDistance) | Table-based distance computation |

## Proof Holes Summary

| # | File | Function | Hole Type | Justification |
|---|------|----------|-----------|---------------|
| 1 | BFSStEph.rs | `bfs` | `external_body` | Batch-verusified; awaiting verification |
| 2 | BFSStPer.rs | `bfs` | `external_body` | Batch-verusified; awaiting verification |
| 3 | BFSMtEph.rs | `bfs` | `external_body` | Batch-verusified; awaiting verification |
| 4 | BFSMtPer.rs | `bfs` | `external_body` | Batch-verusified; awaiting verification |
| 5 | BFSMtPer.rs | `process_layer_parallel` | `external_body` | Threading boundary; will remain external_body |

**Modules:** 4 files, 5 `external_body` holes, 0 verified functions.
