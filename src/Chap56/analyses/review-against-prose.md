<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 56 — Shortest Paths: Review Against Prose

- **Date:** 2026-02-13
- **Reviewer:** Claude-Opus-4.6
- **Prose source:** `prompts/Chap56.txt`
- **Source files:** 12 files in `src/Chap56/`

## Phase 1: Inventory (Tool-Generated)

Tool: `veracity-review-module-fn-impls -d src/Chap56`

- **81 functions** across 12 files
- **0 inside `verus!`** — entire chapter is plain Rust, no verification
- **0 specs** (no `requires`/`ensures`)
- **0 proof holes**
- **0 proof functions**

See `analyses/veracity-review-module-fn-impls.md` for the full function-by-function detail.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 56.1 — Path Weight | Weight of a path = sum of edge weights along the path |
| 2 | Definition 56.2 — Shortest Path and Distance | Shortest path = path with minimal weight; δ_G(u,v) = weight of shortest path |
| 3 | Definition 56.3 — Sub-Path | A sub-path is a path contained within another path |
| 4 | Definition 56.4 — Sub-Paths Property | Any sub-path of a shortest path is itself a shortest path |

### Algorithms

None. Chapter 56 is introductory — it defines concepts and problems. Algorithms are in Chapters 57 (Dijkstra), 58 (Bellman-Ford), and 59 (Johnson).

### Cost Specs

None explicitly stated. The chapter defines problems (SSSP, SSSP+, All-Pairs) but defers algorithmic cost analysis to later chapters.

### Problems

| # | Item | Description |
|---|------|-------------|
| 1 | Problem 56.1 — Single-Pair Shortest Paths | Find shortest path from s to t |
| 2 | Problem 56.2 — SSSP | Find shortest paths from s to all vertices |
| 3 | Problem 56.3 — All-Pairs Shortest Paths | Find shortest paths between all pairs |
| 4 | Problem 56.4 — SSSP+ | SSSP restricted to non-negative weights |

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 56.1 | Path weight computation: path ⟨s,a,b,e⟩ = 6, ⟨s,a,b,s⟩ = 10 |
| 2 | Example 56.3 | Negative weight cycle: cycle ⟨s,a,b,s⟩ = −4, shortest path to e = −∞ |
| 3 | Example 56.4 | Sub-paths property illustration |
| 4 | Example 56.5 | Pittsburgh-to-San-Francisco sub-paths property illustration |

### Theorems/Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Sub-Paths Property (Def 56.4) | Any sub-path of a shortest path is itself a shortest path |

### Exercises/Problems

None. Chapter 56 contains no exercises.

## Phase 3: Algorithmic Analysis

### 3a. Cost Disagreements

| # | File | Function | Issue |
|---|------|----------|-------|
| 1 | PathWeightUtilsStPer.rs | `validate_subpath_property_int` | Module header states O(k²) but implementation is O(k) — only checks consecutive edges, not all subpath pairs |
| 2 | PathWeightUtilsStPer.rs | `validate_subpath_property_float` | Same as above |
| 3 | PathWeightUtilsStEph.rs | `validate_subpath_property_int` | Same as above |
| 4 | PathWeightUtilsStEph.rs | `validate_subpath_property_float` | Same as above |

Note: The O(k) implementation IS correct and sufficient — checking that dist[v] = dist[u] + w(u,v) for each consecutive pair implies all sub-paths are optimal by induction. The module header cost annotation is simply an overestimate, not a correctness issue.

### 3b. Implementation Fidelity

Chapter 56 defines concepts, not algorithms. The code implements **data structures** for storing shortest path results (SSSPResult, AllPairsResult) and **utility functions** for computing path weights and validating the sub-paths property. These are scaffolding for Chapters 57-59, not implementations of prose algorithms.

| # | Code Module | Prose Counterpart | Fidelity |
|---|-------------|-------------------|----------|
| 1 | `PathWeightUtils{StEph,StPer}` | Def 56.1 (Path Weight) | Faithful — sums edge weights along a vertex path |
| 2 | `PathWeightUtils{StEph,StPer}::validate_subpath_property_*` | Def 56.4 (Sub-Paths Property) | Faithful — validates the relaxation condition on consecutive edges |
| 3 | `Example56_1` | Example 56.1 | Faithful — demonstrates path weight with positive/negative weights |
| 4 | `Example56_3` | Example 56.3 | Faithful — demonstrates negative weight cycles |
| 5 | `SSSPResult{StEph,StPer}{Int,Float}` | Problem 56.2 (SSSP) | Scaffolding — data structure for SSSP results, not an algorithm |
| 6 | `AllPairsResult{StEph,StPer}{Int,Float}` | Problem 56.3 (All-Pairs) | Scaffolding — data structure for all-pairs results, not an algorithm |

**Deviations:**
- None significant. The code is infrastructure, and it correctly models the prose's conceptual framework.

### 3c. Spec Fidelity

**No specs exist.** All 81 functions are outside `verus!{}` with no `requires`/`ensures`. The entire chapter is unverified plain Rust.

Key properties that SHOULD be specified when verusified:
- `path_weight`: result equals sum of edge weights (Def 56.1)
- `extract_path`: returned path starts at source and ends at target
- `set_distance` / `set_predecessor`: returned structure differs from input only at the updated index
- `is_reachable`: equivalent to `distance != UNREACHABLE/INFINITY`
- `validate_subpath_property`: returns true iff the relaxation condition holds on all consecutive edges

## Phase 4: Parallelism Review

**No Mt modules exist.** All 12 files are St (sequential). Chapter 56 is purely definitional — no algorithms to parallelize.

Not applicable.

## Phase 5: Runtime Test Review

### 5a. Coverage

**No runtime test files exist** for Chapter 56. Zero coverage.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | AllPairsResultStEphFloat | — | Missing |
| 2 | AllPairsResultStEphInt | — | Missing |
| 3 | AllPairsResultStPerFloat | — | Missing |
| 4 | AllPairsResultStPerInt | — | Missing |
| 5 | Example56_1 | — | Missing |
| 6 | Example56_3 | — | Missing |
| 7 | PathWeightUtilsStEph | — | Missing |
| 8 | PathWeightUtilsStPer | — | Missing |
| 9 | SSSPResultStEphFloat | — | Missing |
| 10 | SSSPResultStEphInt | — | Missing |
| 11 | SSSPResultStPerFloat | — | Missing |
| 12 | SSSPResultStPerInt | — | Missing |

### 5b. Test Quality

N/A — no tests exist.

### 5c. Missing Tests (Priority)

| # | Module | Priority | Rationale |
|---|--------|----------|-----------|
| 1 | SSSPResultStEphInt | High | Used by Chap57-58 Dijkstra/Bellman-Ford; exercises new/set/get/extract_path |
| 2 | AllPairsResultStEphInt | High | Used by Chap59 Johnson's; exercises matrix operations |
| 3 | PathWeightUtilsStEph | High | Core utility; test path_weight with known graphs, validate_subpath on computed shortest paths |
| 4 | Example56_1 | Medium | Run examples and verify expected output values (path weight = 6, 10, 5) |
| 5 | Example56_3 | Medium | Run negative cycle examples and verify expected values |
| 6 | Float variants | Lower | Same logic as Int; test that OrderedFloat comparison works correctly |
| 7 | StPer variants | Lower | Same logic as StEph but with persistent data structures |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs exist** for Chapter 56. Since all code is outside `verus!{}`, there are no iterators, no verified loops, and no ghost state to test. **No PTTs are needed** until the chapter is verusified.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | AllPairsResultStEphFloat | — | — | Missing both |
| 2 | AllPairsResultStEphInt | — | — | Missing both |
| 3 | AllPairsResultStPerFloat | — | — | Missing both |
| 4 | AllPairsResultStPerInt | — | — | Missing both |
| 5 | Example56_1 | — | — | Missing both |
| 6 | Example56_3 | — | — | Missing both |
| 7 | PathWeightUtilsStEph | — | — | Missing both |
| 8 | PathWeightUtilsStPer | — | — | Missing both |
| 9 | SSSPResultStEphFloat | — | — | Missing both |
| 10 | SSSPResultStEphInt | — | — | Missing both |
| 11 | SSSPResultStPerFloat | — | — | Missing both |
| 12 | SSSPResultStPerInt | — | — | Missing both |

### 6b-6d. Iterator/Loop Coverage

N/A — no verified iterators or loops.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Definition 56.2 — formal spec of δ_G(u,v) | No spec fn defining shortest-path distance |
| 2 | Example 56.4 — sub-path enumeration | Not implemented (conceptual) |
| 3 | Example 56.5 — Pittsburgh-to-SF | Not implemented (conceptual) |
| 4 | Problem 56.1 — Single-Pair | No dedicated single-pair result type (extract_path partially covers this) |

### Code With No Prose Counterpart

| # | Code Item | Purpose |
|---|----------|---------|
| 1 | `set_distance` / `set_predecessor` | Mutable scaffolding for algorithms in Chap57-59 |
| 2 | `get_predecessor` | Path reconstruction helper |
| 3 | `extract_path` | Path reconstruction from predecessor array — implied by SSSP but not explicitly defined in prose |
| 4 | Float variants of all types | Prose uses real numbers; code splits into i64 and OrderedFloat<f64> |
| 5 | Eph/Per variants | Prose doesn't distinguish persistence; code provides both styles |

## Phase 8: Table of Contents Review

### TOC Status

| # | File | TOC Present | Section Headers | In/Out Correct |
|---|------|:-----------:|:---------------:|:--------------:|
| 1 | AllPairsResultStEphFloat.rs | No | No | N/A (all outside verus!) |
| 2 | AllPairsResultStEphInt.rs | No | No | N/A |
| 3 | AllPairsResultStPerFloat.rs | No | No | N/A |
| 4 | AllPairsResultStPerInt.rs | No | No | N/A |
| 5 | Example56_1.rs | No | No | N/A |
| 6 | Example56_3.rs | No | No | N/A |
| 7 | PathWeightUtilsStEph.rs | No | No | N/A |
| 8 | PathWeightUtilsStPer.rs | No | No | N/A |
| 9 | SSSPResultStEphFloat.rs | No | No | N/A |
| 10 | SSSPResultStEphInt.rs | No | No | N/A |
| 11 | SSSPResultStPerFloat.rs | No | No | N/A |
| 12 | SSSPResultStPerInt.rs | No | No | N/A |

Since no files use `verus!{}`, the TOC standard does not strictly apply (it governs section ordering inside/outside `verus!`). TOC headers should be added when the chapter is verusified.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | AllPairsResultStEphFloat | - | - | - | - | - | - | - | - | — |
| 2 | AllPairsResultStEphInt | - | - | - | - | - | - | - | - | — |
| 3 | AllPairsResultStPerFloat | - | - | - | - | - | - | - | - | — |
| 4 | AllPairsResultStPerInt | - | - | - | - | - | - | - | - | — |
| 5 | Example56_1 | - | - | - | - | - | - | - | - | — |
| 6 | Example56_3 | - | - | - | - | - | - | - | - | — |
| 7 | PathWeightUtilsStEph | - | - | - | - | - | - | - | - | — |
| 8 | PathWeightUtilsStPer | - | - | - | - | - | - | - | - | — |
| 9 | SSSPResultStEphFloat | - | - | - | - | - | - | - | - | — |
| 10 | SSSPResultStEphInt | - | - | - | - | - | - | - | - | — |
| 11 | SSSPResultStPerFloat | - | - | - | - | - | - | - | - | — |
| 12 | SSSPResultStPerInt | - | - | - | - | - | - | - | - | — |

No derive trait impls in any file. No `❌` items.

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap56/

12 clean (no holes)
0 holed
12 total

🎉 No proof holes found! All proofs are complete.
```

(Trivially clean — no code is inside `verus!{}` so there are no proofs to have holes in.)

## Spec Strength Summary

| Classification | Count |
|----------------|-------|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 81 |

All 81 functions have **no spec** — the entire chapter is plain Rust outside `verus!{}`.

## Overall Assessment

Chapter 56 is an **introductory/definitional chapter** that establishes the concepts of path weight, shortest paths, distance, and the sub-paths property. It defines four problem variants (single-pair, SSSP, SSSP+, all-pairs) but provides no algorithms — those are deferred to Chapters 57-59.

**Implementation quality:** The code faithfully implements the prose concepts as data structures and utility functions. The type structure (SSSP vs AllPairs, Eph vs Per, Int vs Float) provides a clean interface for the algorithm chapters to consume.

**Key findings:**

| # | Finding | Severity |
|---|---------|----------|
| 1 | No `verus!{}` blocks — entire chapter unverified | Major |
| 2 | No `requires`/`ensures` on any function | Major |
| 3 | No runtime tests | Moderate |
| 4 | Module headers overstate `validate_subpath_property` cost as O(k²); actual impl is O(k) | Minor |
| 5 | No TOC headers (expected given no verus!) | Minor |
| 6 | AllPairsResultStEph `set_distance`/`set_predecessor` clone entire rows — O(n) per update where O(1) is possible with interior mutability | Design note |

**Recommended actions:**

| # | Action | Priority |
|---|--------|----------|
| 1 | Verusify all modules (wrap in `verus!{}`, add `requires`/`ensures`) | High |
| 2 | Add runtime tests for SSSPResult, AllPairsResult, PathWeightUtils | High |
| 3 | Fix module header cost annotations for `validate_subpath_property` (O(k²) → O(k)) | Low |
| 4 | Add TOC headers when verusifying | Low |
| 5 | Consider adding a `spec fn spec_distance` for δ_G(u,v) when verusifying | Medium |
