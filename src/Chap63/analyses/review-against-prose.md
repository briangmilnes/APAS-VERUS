<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 63: Graph Connectivity — Review Against Prose

**Date:** 2026-02-19
**Reviewer:** Claude-Opus-4.6
**Source:** `prompts/Chap63.txt`, `src/Chap63/ConnectivityStEph.rs`, `src/Chap63/ConnectivityMtEph.rs`

## Phase 1: Inventory (Tool-Generated)

Generated via `veracity-review-module-fn-impls -d src/Chap63`.

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap63 | ConnectivityMtEph | 4 | 0 | 0 | 7 | 4 | 3 | 0 | 0 | 7 |
| 2 | Chap63 | ConnectivityStEph | 4 | 0 | 0 | 5 | 4 | 1 | 0 | 0 | 5 |

**Verusification status (2026-02-18):**
- Both modules now have `verus!{}` blocks containing their trait definitions (4 trait fns each).
- ConnectivityStEph: 4 trait fns inside `verus!`, 1 helper (`build_quotient_edges`) outside behind `#[cfg(not(verus_keep_ghost))]`.
- ConnectivityMtEph: 4 trait fns inside `verus!`, 3 helpers (`build_quotient_edges_parallel`, `route_edges_parallel`, `compose_maps_parallel`) outside behind `#[cfg(not(verus_keep_ghost))]`.
- All 4 public impl functions per module are cfg-gated behind `#[cfg(not(verus_keep_ghost))]`.
- The trait method signatures have no `requires`/`ensures` — they are unspecified placeholders.
- `lib.rs` gate: Chap63 is now behind `#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]` (standard chapter gate, no longer excluded from Verus verification).

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 63.1 | The Graph Connectivity Problem — find all connected components |
| 2 | Assumption | Edge-set representation with both arc directions per undirected edge |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 63.2 | `countComponents(G=(V,E))` — recursive star contraction to count connected components |
| 2 | Algorithm 63.3 | `connectedComponents(G=(V,E))` — computes connected components returning (representatives, vertex-to-component mapping) |

### Cost Specs

The prose does not state costs directly for Algorithms 63.2 or 63.3. Instead, Exercises 63.3 and 63.4 ask the reader to derive them. From the star contraction analysis in Chapter 62:

| # | Algorithm | Expected Work | Expected Span (parallel) | Expected Span (sequential) |
|---|-----------|--------------|--------------------------|---------------------------|
| 1 | Algorithm 63.2 | O((n+m) lg n) | O(lg^2 n) | O((n+m) lg n) |
| 2 | Algorithm 63.3 | O((n+m) lg n) | O(lg^2 n) | O((n+m) lg n) |

### Exercises

| # | Exercise | Description | Implemented? |
|---|----------|-------------|:------------:|
| 1 | Exercise 63.1 | Express countComponents via starContract HOF (base + expand) | Yes — `count_components_hof` |
| 2 | Exercise 63.2 | Express connectedComponents via starContract HOF (base + expand) | Yes — `connected_components_hof` |
| 3 | Exercise 63.3 | Work and span of countComponents | N/A (text proof) — cost annotations present |
| 4 | Exercise 63.4 | Work and span of connectedComponents | N/A (text proof) — cost annotations present |

### Theorems/Properties

No explicit theorems stated. The correctness argument is informal:
- Base case: no edges implies each vertex is its own component.
- Inductive case: star partition produces a quotient graph with fewer vertices; recursion computes components of quotient; map composition recovers original vertex mapping.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All exec functions now have APAS/Claude-Opus-4.6 cost comment pairs.

**Cost disagreements found:**

| # | Function | File | APAS Span | Claude-Opus-4.6 Span | Reason |
|---|----------|------|-----------|----------------------|--------|
| 1 | `count_components_mt` | ConnectivityMtEph.rs | O(lg^2 n) | O(m) | `route_edges_parallel` uses sequential set union after `ParaPair!` — merge at top level is O(m/2) |
| 2 | `connected_components_mt` | ConnectivityMtEph.rs | O(lg^2 n) | O(n lg n) | `compose_maps_parallel` is entirely sequential O(n) per round times O(lg n) rounds |
| 3 | `build_quotient_edges_parallel` | ConnectivityMtEph.rs | N/A | O(m) | Delegates to `route_edges_parallel` with sequential merge |
| 4 | `route_edges_parallel` | ConnectivityMtEph.rs | N/A | O(k) not O(lg k) | Sequential `for` loop merges two halves after `ParaPair!` |
| 5 | `compose_maps_parallel` | ConnectivityMtEph.rs | N/A | O(|P|) span = work | Named "parallel" but implementation is sequential |
| 6 | `count_components_hof` (Mt) | ConnectivityMtEph.rs | O(lg^2 n) | O(m) | Inherits merge bottleneck from star_contract_mt |
| 7 | `connected_components_hof` (Mt) | ConnectivityMtEph.rs | O(lg^2 n) | O(n lg n) | Inherits compose bottleneck from star_contract_mt |

No cost disagreements in `ConnectivityStEph.rs` — all sequential implementations agree with APAS.

### Phase 3b: Implementation Fidelity

**Algorithm 63.2 (count_components):**
- St: Faithful to prose. Base case checks `|E| = 0`, returns `|V|`. Inductive case calls `sequential_star_partition`, builds quotient via `build_quotient_edges`, recurses.
- Mt: Structurally faithful but with parallelism gaps (see Phase 4).

**Algorithm 63.3 (connected_components):**
- St: Faithful to prose. Base case returns `(V, {v->v})`. Inductive case: partition, quotient, recurse, compose `C . P` on Line 10.
- Mt: Structurally faithful. `compose_maps_parallel` implements the map composition but sequentially.

**Trait return type mismatch:** The traits declare `connected_components` and `connected_components_hof` as returning `SetStEph<SetStEph<V>>` (set of sets), but the free function implementations return `(SetStEph<V>, HashMap<V, V>)` (representatives + mapping). The implementation return type matches the prose (Algorithm 63.3 returns `(V'', {u->C[v]})`). The trait is never `impl`'d — it serves as documentation only. The trait signatures should be updated to match the implementations.

**Trait parameter mismatch (Mt):** All Mt free functions take an extra `seed: u64` parameter not present in the trait declarations.

**Exercises 63.1 and 63.2:** Both correctly implemented using `star_contract` / `star_contract_mt` HOF with appropriate `base` and `expand` closures. The `base` and `expand` functions match the prose description.

### Phase 3c: Spec Fidelity

**2026-02-18 update:** Trait definitions for both `ConnectivityStEphTrait` and `ConnectivityMtEphTrait` are now inside `verus!{}` blocks. However, the trait method signatures still have **no `requires`/`ensures` clauses** — they are bare signatures with doc comments only. All impl functions remain outside `verus!` behind `#[cfg(not(verus_keep_ghost))]`.

There is nothing to compare against the prose at the specification level. The code remains functionally unverified — the `verus!` blocks establish the trait structure but carry no formal contracts.

## Phase 4: Parallelism Review

### Phase 4a: Classification of Mt Functions

| # | Function | Classification | Mechanism |
|---|----------|---------------|-----------|
| 1 | `count_components_mt` | Parallel (partial) | Delegates to `parallel_star_partition` (parallel) + `build_quotient_edges_parallel` (partially parallel) |
| 2 | `connected_components_mt` | Parallel (partial) | Same as above + `compose_maps_parallel` (sequential) |
| 3 | `build_quotient_edges_parallel` | Parallel (partial) | Wraps `route_edges_parallel` which uses `ParaPair!` but sequential merge |
| 4 | `route_edges_parallel` | Parallel (partial) | `ParaPair!` for recursion but sequential set union for merge |
| 5 | `compose_maps_parallel` | Sequential | `for` loop over HashMap, no threading despite name |
| 6 | `count_components_hof` | Delegating | Delegates to `star_contract_mt` |
| 7 | `connected_components_hof` | Delegating | Delegates to `star_contract_mt` |

### Phase 4b: Span Audit

- `route_edges_parallel`: Claims O(lg k) span. The `ParaPair!` divides work, but the merge (`for edge in pair.1.iter() { result.insert(...) }`) is sequential O(|pair.1|). The span recurrence is T(k) = T(k/2) + O(k/2), which solves to T(k) = O(k). **Aspirational span; actual span is O(k).**
- `compose_maps_parallel`: Claims Span O(lg |P|) in original comment. Implementation is a sequential `for` loop. **Aspirational; actual span is O(|P|).**

### Phase 4c: Parallelism Gap Table

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|:---------:|-------|
| 1 | `count_components_mt` | O(lg^2 n) | O(m) | Partial | Edge routing merge is sequential |
| 2 | `connected_components_mt` | O(lg^2 n) | O(n lg n) | Partial | Map composition is sequential |
| 3 | `build_quotient_edges_parallel` | — | O(m) | Partial | Sequential merge bottleneck |
| 4 | `route_edges_parallel` | — | O(k) | Partial | ParaPair! but sequential set union |
| 5 | `compose_maps_parallel` | — | O(|P|) | No | Entirely sequential loop |
| 6 | `count_components_hof` | O(lg^2 n) | O(m) | Partial | Inherits from star_contract_mt |
| 7 | `connected_components_hof` | O(lg^2 n) | O(n lg n) | Partial | Inherits from star_contract_mt |

**Root causes of parallelism gaps:**
1. `route_edges_parallel`: The set union after `ParaPair!` iterates one set and inserts into the other. A parallel merge (e.g., using a concurrent set or a parallel union algorithm) would recover O(lg m) span.
2. `compose_maps_parallel`: Named "parallel" but has a TODO comment: "For now, compose sequentially since tuples don't implement Display." A parallel map operation would recover O(lg n) span.

## Phase 5: Runtime Test Review

### Phase 5a: Coverage

**No runtime test files exist for Chapter 63.** No files matching `tests/*Chap63*` or `tests/*chap63*` were found.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | ConnectivityStEph | — | Missing RTT |
| 2 | ConnectivityMtEph | — | Missing RTT |

### Phase 5b-5c: Missing Tests

All 12 exec functions lack runtime test coverage. Recommended tests:

| # | Priority | Test | Rationale |
|---|:--------:|-------|-----------|
| 1 | High | `count_components` on single-component graph (Example 63.1) | Validates base algorithm |
| 2 | High | `count_components` on multi-component graph | Tests non-trivial component counting |
| 3 | High | `connected_components` on Example 63.1 graph | Validates mapping correctness |
| 4 | High | `connected_components` on disconnected graph | Multiple components with correct representatives |
| 5 | Medium | `count_components` on empty graph (no vertices) | Edge case |
| 6 | Medium | `count_components` on isolated vertices (no edges) | Base case — each vertex is own component |
| 7 | Medium | HOF variants produce same results as direct variants | Equivalence check |
| 8 | Medium | Mt variants produce same results as St variants | Cross-variant equivalence |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 63 has no iterators, no verified loops, and no types with `iter()`, `IntoIterator`, `GhostIterator`, or `ForLoopGhostIterator`. The `verus!` blocks contain only trait definitions with no proof obligations.

No PTT files were found matching `rust_verify_test/tests/*Chap63*`.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Example 63.1 (edge-set graph) | Not implemented as a named test/example |
| 2 | Example 63.2 (countComponents execution trace) | Not implemented as a test |
| 3 | Example 63.3 (connectedComponents on single-component graph) | Not implemented as a test |
| 4 | Example 63.4 (connectedComponents detailed trace) | Not implemented as a test |
| 5 | Exercise 63.3 (work/span derivation for counting) | Text proof — cost annotations present |
| 6 | Exercise 63.4 (work/span derivation for components) | Text proof — cost annotations present |

### Code With No Prose Counterpart

| # | Function | File | Purpose |
|---|----------|------|---------|
| 1 | `build_quotient_edges` | ConnectivityStEph.rs | Helper — implements Line 7 of Algorithm 63.2/63.3 |
| 2 | `build_quotient_edges_parallel` | ConnectivityMtEph.rs | Parallel version of above |
| 3 | `route_edges_parallel` | ConnectivityMtEph.rs | Divide-and-conquer helper for parallel edge routing |
| 4 | `compose_maps_parallel` | ConnectivityMtEph.rs | Map composition helper for Line 10 of Algorithm 63.3 |

These are expected helper functions — the prose describes the operations inline (Lines 7 and 10) but does not name them.

## Phase 8: Table of Contents Review

### TOC Presence

Both files now have `verus!{}` blocks containing trait definitions but lack `// Table of Contents` headers. The TOC standard applies minimally since only trait definitions are inside `verus!`.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | ConnectivityStEph.rs | - | - | - | - | - | - | - | - | Trait in verus!, 1 helper + 4 impls cfg-gated |
| 2 | ConnectivityMtEph.rs | - | - | - | - | - | - | - | - | Trait in verus!, 3 helpers + 4 impls cfg-gated |

No derive impls exist. The `verus!` blocks contain only trait definitions (no exec functions, no proof functions). All exec code is outside `verus!` behind `#[cfg(not(verus_keep_ghost))]`.

## Proof Holes Summary

```
veracity-review-proof-holes output (2026-02-18):

Modules: 2 clean, 0 holed, 2 total
Proof Functions: 0 total
Holes Found: 0 total
```

This is vacuously true — the `verus!` blocks contain only trait definitions with no proof obligations.

## Spec Strength Summary

| Classification | Count |
|---------------|:-----:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 12 |

All 12 functions have spec strength **none** — no `requires`/`ensures` exist anywhere in the chapter. The trait signatures inside `verus!` are bare (no contracts).

## Overall Assessment

Chapter 63 implements all named algorithms and exercises from the prose:
- Algorithm 63.2 (`countComponents`) — both St and Mt variants
- Algorithm 63.3 (`connectedComponents`) — both St and Mt variants
- Exercise 63.1 (`countComponents` via starContract HOF) — both St and Mt variants
- Exercise 63.2 (`connectedComponents` via starContract HOF) — both St and Mt variants

**Verusification status (2026-02-18):** Partially verusified. Trait definitions are inside `verus!{}` blocks, establishing the module structure for future verification. All implementation functions are cfg-gated behind `#[cfg(not(verus_keep_ghost))]`. Chap63 is now included in standard Verus verification runs (no longer behind `not(verus_keep_ghost)` gate in `lib.rs`).

**Key Issues:**

| # | Severity | Issue |
|---|:--------:|-------|
| 1 | High | **No formal specs.** Trait signatures inside `verus!` have no `requires`/`ensures`. Zero contracts. |
| 2 | High | **No runtime tests.** No RTT files exist for either module. |
| 3 | Medium | **All impl code outside verus!.** The 12 exec functions are behind `#[cfg(not(verus_keep_ghost))]` — they compile and run but are not verified. |
| 4 | Medium | **Mt parallelism gaps.** `route_edges_parallel` has O(k) span (not O(lg k)) due to sequential set merge. `compose_maps_parallel` is entirely sequential. |
| 5 | Medium | **Trait/implementation mismatch.** Trait declares `connected_components` returning `SetStEph<SetStEph<V>>` but implementation returns `(SetStEph<V>, HashMap<V, V>)`. Mt trait methods lack the `seed` parameter. |
| 6 | Low | **Trait cost annotations incorrect.** Traits claim O(|V| + |E|) work but the recursive algorithms are O((n+m) lg n). |
| 7 | Low | **No TOC headers** in either file. |
