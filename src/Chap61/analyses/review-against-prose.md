<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 61: Edge Contraction — Review Against Prose

- **Date**: 2026-02-19
- **Reviewer**: Claude-Opus-4.6
- **Source files**: 4 (`EdgeContractionMtEph.rs`, `EdgeContractionStEph.rs`, `VertexMatchingMtEph.rs`, `VertexMatchingStEph.rs`)
- **Runtime tests**: 0
- **Proof-time tests**: 0
- **Proof holes**: 0
- **Verus coverage**: Partial — trait definitions inside `verus!`, implementations `#[cfg(not(verus_keep_ghost))]`
- **lib.rs gate**: No longer behind `#[cfg(not(verus_keep_ghost))]` — Verus sees the trait declarations

## Phase 1: Inventory (Tool-Generated)

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec | SpecStr |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|:-------:|
| 1 | Chap61 | EdgeContractionMtEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 | none |
| 2 | Chap61 | EdgeContractionStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 | none |
| 3 | Chap61 | VertexMatchingMtEph | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 | none |
| 4 | Chap61 | VertexMatchingStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 | none |

12 exec functions total. 7 trait method declarations inside `verus!` (type-checked by Verus), 5 implementation functions outside `verus!` with `#[cfg(not(verus_keep_ghost))]`. All 12 have no functional spec (no `requires`/`ensures`).

**Fully inside verus!**: EdgeContractionStEph (V!=2, -V!=0), VertexMatchingStEph (V!=2, -V!=0) — all trait method declarations are inside `verus!` with no remaining outside functions.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Prose Section |
|---|------|---------------|
| 1 | Edge Partition (Definition 61.1) | §1 |
| 2 | Vertex Matching (Definition 61.2) | §1 |
| 3 | Star Graph (Definition 61.5) | §1.1.2 |

### Algorithms

| # | Algorithm | Prose Section | Implemented? |
|---|-----------|---------------|:------------:|
| 1 | Algorithm 61.3: Greedy Vertex Matching | §1 | Yes — `greedy_matching` (St) |
| 2 | Algorithm 61.4: Parallel Vertex Matching | §1 | Yes — `parallel_matching_mt` (Mt), `parallel_matching_st` (St baseline) |
| 3 | Algorithm 61.6: Parallel Edge Contraction | §2 | Yes — `edge_contract_mt` (Mt), `edge_contract` (St) |

### Cost Specs from Prose

| # | Algorithm | Prose Cost |
|---|-----------|-----------|
| 1 | Greedy Vertex Matching | Work O(\|E\|), Span O(\|E\|) — sequential |
| 2 | Parallel Vertex Matching | Work O(\|E\|), Span O(lg \|V\|) — each edge checks only incident edges in parallel |
| 3 | Edge Contraction (cycle graph) | Work O(n + m), Span O(lg² n) — over O(lg n) rounds |
| 4 | Edge Partition probability | P(edge selected in cycle) = 1/8, expected m/8 edges per round |

### Exercises/Problems

| # | Item | Type | Implemented? |
|---|------|------|:------------:|
| 1 | Exercise 61.1 | Text proof (isolated vertex example) | No — text only |
| 2 | Exercise 61.2 | Text proof (greedy not maximum) | No — text only |
| 3 | Exercise 61.3 | Text proof (greedy within factor 2) | No — text only |
| 4 | Exercise 61.4 | Text proof (greedy is sequential) | No — text only |
| 5 | Exercise 61.5 | Text proof (parallel matching produces valid matching) | No — text only |
| 6 | Exercise 61.6 | Algorithm improvement (increase expected edges) | No |

### Theorems/Properties

| # | Property | Prose Section |
|---|----------|---------------|
| 1 | Parallel matching produces a valid vertex matching (no vertex matched twice) | §1 |
| 2 | Greedy matching is within factor 2 of optimal | Exercise 61.3 |
| 3 | P(edge selected in cycle) = 1/8 | §1.1.1 |
| 4 | Expected m/8 edges selected per round in cycle | §1.1.1 |
| 5 | O(lg n) rounds suffice for cycle contraction w.h.p. | §2 |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Function | File | APAS Cost | Claude-Opus-4.6 Cost | Agree? | Notes |
|---|----------|------|-----------|----------------------|:------:|-------|
| 1 | `edge_contract_mt` | EdgeContractionMtEph | W O(\|V\|+\|E\|), S O(lg \|V\|) | W Θ(\|V\|+\|E\|), S Θ(\|V\|+\|E\|) | **No** | Phases 1-2 are sequential loops over V and E; only Phase 3 is parallel |
| 2 | `contract_round_mt` | EdgeContractionMtEph | W O(\|V\|+\|E\|), S O(lg \|V\|) | W Θ(\|E\|²), S Θ(\|E\|) | **No** | Dominated by parallel_matching_mt cost |
| 3 | `build_edges_parallel` | EdgeContractionMtEph | N/A — scaffolding | W Θ(\|E\|), S Θ(lg \|E\|) | — | Genuine parallel divide-and-conquer |
| 4 | `edge_contract` | EdgeContractionStEph | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | W Θ(\|V\|+\|E\|), S Θ(\|V\|+\|E\|) | **Yes** | |
| 5 | `contract_round` | EdgeContractionStEph | W O(\|V\|+\|E\|), S O(\|V\|+\|E\|) | W Θ(\|V\|+\|E\|), S Θ(\|V\|+\|E\|) | **Yes** | |
| 6 | `parallel_matching_mt` | VertexMatchingMtEph | W O(\|E\|), S O(lg \|V\|) | W Θ(\|E\|²), S Θ(\|E\|) | **No** | Coin flip phase is sequential (RNG); edge selection scans all \|E\| edges per candidate |
| 7 | `flip_coins_parallel` | VertexMatchingMtEph | W Θ(\|E\|), S Θ(1) | W Θ(\|E\|), S Θ(\|E\|) | **No** | Name says "parallel" but RNG is inherently sequential |
| 8 | `select_edges_parallel` | VertexMatchingMtEph | W O(\|E\|), S O(lg \|V\|) | W Θ(\|E\|²), S Θ(lg \|E\| + \|E\|) | **No** | should_select_edge scans all edges |
| 9 | `select_edges_recursive` | VertexMatchingMtEph | N/A — scaffolding | W Θ(k × \|E\|), S Θ(lg k + \|E\|) | — | ParaPair is genuine parallelism but base case is Θ(\|E\|) |
| 10 | `should_select_edge` | VertexMatchingMtEph | W O(deg(u)+deg(v)), S O(deg(u)+deg(v)) | W Θ(\|E\|), S Θ(\|E\|) | **No** | Iterates all graph edges, not just incident edges |
| 11 | `greedy_matching` | VertexMatchingStEph | W Θ(\|E\|), S Θ(\|E\|) | W Θ(\|E\|), S Θ(\|E\|) | **Yes** | |
| 12 | `parallel_matching_st` | VertexMatchingStEph | (no cost stated) | W Θ(\|E\|²), S Θ(\|E\|²) | — | Sequential baseline of parallel algorithm |

**Cost disagreements**: 6 out of 12 functions. Root cause: `should_select_edge` iterates over all edges rather than using an adjacency list to check only incident edges. This inflates the work from O(degree) to O(\|E\|) for each edge, making the overall algorithms O(\|E\|²) instead of O(\|E\|).

### 3b. Implementation Fidelity

| # | Function | Prose Algorithm | Fidelity | Notes |
|---|----------|----------------|----------|-------|
| 1 | `greedy_matching` | Algorithm 61.3 | **Faithful** | Iterates edges, checks endpoint membership. Correct greedy strategy. |
| 2 | `parallel_matching_mt` | Algorithm 61.4 | **Partial deviation** | Coin-flip phase is sequential (RNG is inherently sequential). Edge selection is parallel via divide-and-conquer. The adjacency check (`should_select_edge`) scans all edges instead of using adjacency lists, changing the asymptotic cost. |
| 3 | `parallel_matching_st` | Algorithm 61.4 (seq baseline) | **Faithful to intent** | Sequential simulation of Algorithm 61.4. Same adjacency-scanning issue. |
| 4 | `edge_contract_mt` | Algorithm 61.6 | **Partial deviation** | Phases 1-2 (vertex mapping, vertex set construction) are sequential. Only Phase 3 (edge routing) uses ParaPair parallelism. APAS expects all phases parallel. |
| 5 | `edge_contract` | Algorithm 61.6 (seq) | **Faithful** | Correctly builds vertex-to-block mapping and quotient graph. |
| 6 | `contract_round_mt` | Composed | **Faithful** | Correctly composes parallel_matching_mt + edge_contract_mt. |
| 7 | `contract_round` | Composed | **Faithful** | Correctly composes greedy_matching + edge_contract. |

**Key deviation**: `should_select_edge` iterates `graph.edges().iter()` (all edges) to check adjacency, rather than using an adjacency list or neighbor iterator. The prose assumes O(degree) adjacency checks via an adjacency-list representation. The `UnDirGraphMtEph` type exposes `edges()` (all edges) and `incident()` but no per-vertex neighbor iterator, which forces the full scan.

### 3c. Spec Fidelity

All 12 functions have **no functional specification** (no `requires`/`ensures`). Trait definitions are now inside `verus!` blocks, so Verus type-checks the trait method signatures. However, the trait methods declare no `requires` or `ensures` clauses — the signatures are purely structural (parameter types and return types) with no functional contracts.

None of the prose's correctness properties (valid matching, factor-2 approximation, contraction ratio) are formally specified. The implementation functions remain outside `verus!` behind `#[cfg(not(verus_keep_ghost))]`, so their bodies are not verified.

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

| # | Function | Classification | Evidence |
|---|----------|:-------------:|---------|
| 1 | `parallel_matching_mt` | **Mixed** | Phase 1 (coin flip) is sequential; Phase 2 delegates to parallel recursive helper |
| 2 | `flip_coins_parallel` | **Sequential** | Sequential for loop over RNG — name is misleading |
| 3 | `select_edges_parallel` | **Delegating** | Builds data structures sequentially, then delegates to `select_edges_recursive` |
| 4 | `select_edges_recursive` | **Parallel** | Uses `ParaPair!` for divide-and-conquer |
| 5 | `should_select_edge` | **Sequential** | Sequential loop over all edges |
| 6 | `edge_contract_mt` | **Mixed** | Phases 1-2 sequential; Phase 3 delegates to `build_edges_parallel` |
| 7 | `build_edges_parallel` | **Parallel** | Uses `ParaPair!` for divide-and-conquer |
| 8 | `contract_round_mt` | **Delegating** | Calls parallel_matching_mt then edge_contract_mt |

### 4b. Span Audit

| # | Function | Annotated Span | Actual Span | Gap? |
|---|----------|---------------|-------------|:----:|
| 1 | `parallel_matching_mt` | O(lg \|V\|) | Θ(\|E\|) | **Yes** — coin flip is Θ(\|E\|), should_select_edge is Θ(\|E\|) |
| 2 | `flip_coins_parallel` | Θ(1) | Θ(\|E\|) | **Yes** — sequential RNG |
| 3 | `select_edges_parallel` | O(lg \|V\|) | Θ(lg \|E\| + \|E\|) = Θ(\|E\|) | **Yes** — base case is Θ(\|E\|) |
| 4 | `select_edges_recursive` | Θ(lg k + \|E\|) | Θ(lg k + \|E\|) | No — correctly stated |
| 5 | `should_select_edge` | Θ(\|E\|) | Θ(\|E\|) | No |
| 6 | `edge_contract_mt` | O(lg \|V\|) | Θ(\|V\| + \|E\|) | **Yes** — Phases 1-2 are sequential |
| 7 | `build_edges_parallel` | Θ(lg \|E\|) | Θ(lg \|E\|) | No — genuine parallelism |
| 8 | `contract_round_mt` | O(lg \|V\|) | Θ(\|E\|) | **Yes** — dominated by parallel_matching_mt |

### 4c. Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|:---------:|-------|
| 1 | `parallel_matching_mt` | O(lg \|V\|) | Θ(\|E\|) | Partial | Coin flip sequential; edge selection parallel but base case Θ(\|E\|) |
| 2 | `flip_coins_parallel` | Θ(1) | Θ(\|E\|) | **No** | RNG is inherently sequential |
| 3 | `select_edges_parallel` | O(lg \|V\|) | Θ(\|E\|) | Yes | Parallel divide-and-conquer, but span bottlenecked by should_select_edge |
| 4 | `select_edges_recursive` | Θ(lg k + \|E\|) | Θ(lg k + \|E\|) | Yes | Correct — ParaPair divide-and-conquer |
| 5 | `should_select_edge` | O(degree) | Θ(\|E\|) | No | Scans all edges, not just neighbors |
| 6 | `edge_contract_mt` | O(lg \|V\|) | Θ(\|V\| + \|E\|) | Partial | Only Phase 3 is parallel |
| 7 | `build_edges_parallel` | Θ(lg \|E\|) | Θ(lg \|E\|) | **Yes** | Correct — ParaPair divide-and-conquer |
| 8 | `contract_round_mt` | O(lg \|V\|) | Θ(\|E\|) | Partial | Composed of partially parallel pieces |

## Phase 5: Runtime Test Review

### 5a. Coverage Check

**No runtime tests exist for Chapter 61.** Zero test files found in `tests/`.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | VertexMatchingStEph | — | **Missing** |
| 2 | VertexMatchingMtEph | — | **Missing** |
| 3 | EdgeContractionStEph | — | **Missing** |
| 4 | EdgeContractionMtEph | — | **Missing** |

### 5b. Test Quality

N/A — no tests exist.

### 5c. Missing Tests (Proposed)

| # | Priority | Test Description |
|---|:--------:|-----------------|
| 1 | High | `greedy_matching` on triangle graph (K3) — should return 1 edge |
| 2 | High | `greedy_matching` on path graph (a-b-c-d) — should return 2 non-adjacent edges |
| 3 | High | `parallel_matching_mt` on cycle graph — verify result is a valid matching |
| 4 | High | `parallel_matching_st` on same graph — compare with mt version (same seed) |
| 5 | High | `edge_contract` on triangle with 1-edge matching — verify contracted graph |
| 6 | High | `edge_contract_mt` on same input — compare with sequential version |
| 7 | Medium | `contract_round` on cycle graph — verify vertex count decreases |
| 8 | Medium | `contract_round_mt` on cycle graph — verify vertex count decreases |
| 9 | Low | Matching validity check: no two edges in result share an endpoint |

## Phase 6: Proof-Time Test (PTT) Review

Chapter 61 has **no verified loops, no iterators, and no `requires`/`ensures` clauses**. Trait definitions are inside `verus!` blocks and are type-checked, but since they declare no functional specs, there is nothing to test at proof time. **No PTTs are needed** until functional specs are added to the trait methods.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | VertexMatchingStEph | — | — | Missing RTT (no PTT needed) |
| 2 | VertexMatchingMtEph | — | — | Missing RTT (no PTT needed) |
| 3 | EdgeContractionStEph | — | — | Missing RTT (no PTT needed) |
| 4 | EdgeContractionMtEph | — | — | Missing RTT (no PTT needed) |

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Type | Notes |
|---|-----------|------|-------|
| 1 | Definition 61.1 (Edge Partition) | Type definition | No explicit type; partitions are implicit in the contraction output |
| 2 | Definition 61.2 (Vertex Matching) | Type definition | Represented as `SetStEph<Edge<V>>` — no dedicated type |
| 3 | Definition 61.5 (Star Graph) | Type definition | No star graph type or constructor |
| 4 | Exercise 61.1 | Text proof | Not applicable for code |
| 5 | Exercise 61.2 | Text proof | Not applicable for code |
| 6 | Exercise 61.3 | Text proof (factor-2 bound) | Could be a proof fn but is not |
| 7 | Exercise 61.5 | Text proof (matching validity) | Could be a proof fn but is not |
| 8 | Exercise 61.6 | Algorithm improvement | Not implemented |
| 9 | §1.1.1 Probability analysis | Property | P(edge selected) = 1/8 — not formalized |
| 10 | §2 O(lg n) rounds w.h.p. | Theorem | Not formalized |

### Code with No Prose Counterpart

| # | Function | File | Notes |
|---|----------|------|-------|
| 1 | `flip_coins_parallel` | VertexMatchingMtEph | Helper — coin flipping phase of Algorithm 61.4 |
| 2 | `select_edges_parallel` | VertexMatchingMtEph | Helper — edge selection phase of Algorithm 61.4 |
| 3 | `select_edges_recursive` | VertexMatchingMtEph | Scaffolding — parallel recursion for edge selection |
| 4 | `should_select_edge` | VertexMatchingMtEph | Helper — per-edge selection predicate |
| 5 | `build_edges_parallel` | EdgeContractionMtEph | Scaffolding — parallel edge routing |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present? | Notes |
|---|------|:------------:|-------|
| 1 | EdgeContractionMtEph.rs | **No** | Trait inside `verus!`, impls outside — no TOC yet |
| 2 | EdgeContractionStEph.rs | **No** | Trait inside `verus!`, impls outside — no TOC yet |
| 3 | VertexMatchingMtEph.rs | **No** | Trait inside `verus!`, impls outside — no TOC yet |
| 4 | VertexMatchingStEph.rs | **No** | Trait inside `verus!`, impls outside — no TOC yet |

All four files now have `verus!` blocks containing trait definitions. The TOC standard applies but is not yet implemented. TOCs should be added when more code moves inside `verus!`.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | EdgeContractionMtEph.rs | - | - | - | - | - | - | - | - | trait defs ✅ in |
| 2 | EdgeContractionStEph.rs | - | - | - | - | - | - | - | - | trait defs ✅ in |
| 3 | VertexMatchingMtEph.rs | - | - | - | - | - | - | - | - | trait defs ✅ in |
| 4 | VertexMatchingStEph.rs | - | - | - | - | - | - | - | - | trait defs ✅ in |

No derive impls in any file. Trait definitions are correctly inside `verus!` blocks. Implementation functions are outside `verus!` behind `#[cfg(not(verus_keep_ghost))]` — this is the expected pattern for partially verusified modules where the impl bodies use features Verus cannot yet handle (RNG, ParaPair!, HashMap, etc.).

## Proof Holes Summary

**Last verified:** 2026-02-18 (`veracity-review-proof-holes`)

```
✓ EdgeContractionMtEph.rs
✓ EdgeContractionStEph.rs
✓ VertexMatchingMtEph.rs
✓ VertexMatchingStEph.rs

Modules: 4 clean, 0 holed
Holes Found: 0
```

No proof holes. The trait definitions inside `verus!` contain no `assume`, `admit`, or `external_body`. The cfg-gated implementations are invisible to Verus. VertexMatchingStEph.rs was updated 2026-02-18; hole count remains 0.

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | **12** |

All 12 functions have **no functional specification**. The trait method declarations inside `verus!` have no `requires`/`ensures` — they are type-checked but not functionally specified.

## Overall Assessment

### Status: Partially Verusified — Trait Definitions Inside verus!

Chapter 61 has been partially verusified: trait definitions have been moved inside `verus!` blocks and the chapter is no longer gated behind `#[cfg(not(verus_keep_ghost))]` in `lib.rs`. Verus now type-checks the trait method signatures. Implementation functions remain outside `verus!` behind `#[cfg(not(verus_keep_ghost))]`.

The three main algorithms from the prose are implemented:
- Algorithm 61.3 (Greedy Vertex Matching) — faithfully implemented
- Algorithm 61.4 (Parallel Vertex Matching) — implemented with significant cost deviations
- Algorithm 61.6 (Parallel Edge Contraction) — implemented with partial parallelism

### Critical Issues

| # | Severity | Issue |
|---|:--------:|-------|
| 1 | **High** | All 12 functions have no spec (no `requires`/`ensures`) — trait signatures are type-checked but not functionally specified |
| 2 | **High** | Implementation bodies remain outside `verus!` — no formal verification of algorithm logic |
| 3 | **High** | No runtime tests at all — no informal validation either |
| 4 | **High** | `should_select_edge` scans all edges O(\|E\|) instead of O(degree) — inflates Mt algorithm from O(\|E\|) work to O(\|E\|²) |
| 5 | **Medium** | `flip_coins_parallel` is actually sequential — RNG is inherently sequential, name is misleading |
| 6 | **Medium** | `edge_contract_mt` Phases 1-2 are sequential — only Phase 3 (edge routing) uses parallelism |
| 7 | **Low** | No TOC headers in any file |
| 8 | **Low** | Return type mismatch: trait `edge_contract` returns `UnDirGraphStEph<SetStEph<V>>` but impl returns `UnDirGraphStEph<V>` |

### What Verusification Achieved

1. **Trait definitions inside `verus!`** — Verus type-checks method signatures, ensuring type consistency.
2. **Chapter visible to Verus** — no longer gated behind `#[cfg(not(verus_keep_ghost))]` in `lib.rs`.
3. **Foundation for specs** — trait methods can now receive `requires`/`ensures` clauses incrementally.

### What Remains

1. **Add functional specs** — at minimum, `greedy_matching` and `edge_contract` should have `ensures` stating the result is a valid matching / valid contracted graph.
2. **Move implementation bodies inside `verus!`** where feasible (sequential functions that don't use RNG or ParaPair!).
3. **Fix `should_select_edge`** to use per-vertex adjacency information instead of scanning all edges.
4. **Add runtime tests** — at least 6 tests covering basic graph shapes (triangle, path, cycle, star).
5. **Rename `flip_coins_parallel`** to `flip_coins` since it is actually sequential.
6. **Parallelize Phases 1-2 of `edge_contract_mt`** — both vertex mapping and vertex set construction can be done in parallel with divide-and-conquer.
7. **Fix return type mismatch** in `EdgeContractionStEphTrait::edge_contract`.
