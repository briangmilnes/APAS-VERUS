<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 56 — Shortest Paths (Introduction): Review Against Prose

**Date:** 2026-02-18 (updated from 2026-02-17)
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

| # | File | exec fns | external_body | spec fns | proof fns | View | verus! | Trait Wired |
|---|------|:--------:|:-------------:|:--------:|:---------:|:----:|:------:|:-----------:|
| 1 | SSSPResultStEphI64.rs | 7 | 4 | 0 | 0 | Yes | Yes | Yes |
| 2 | SSSPResultStPerI64.rs | 7 | 3 | 0 | 0 | Yes | Yes | Yes |
| 3 | SSSPResultStEphFloat.rs | 7 | 5 | 0 | 1 | Yes | Yes | No (agent2 rewrite) |
| 4 | SSSPResultStPerFloat.rs | 7 | 7 | 0 | 0 | Yes | Yes | Yes |
| 5 | AllPairsResultStEphI64.rs | 7 | 2 | 0 | 0 | Yes | Yes | Yes |
| 6 | AllPairsResultStPerI64.rs | 7 | 4 | 0 | 0 | Yes | Yes | Yes |
| 7 | AllPairsResultStEphFloat.rs | 7 | 7 | 0 | 0 | Yes | Yes | Yes |
| 8 | AllPairsResultStPerFloat.rs | 7 | 7 | 0 | 0 | Yes | Yes | Yes |
| 9 | PathWeightUtilsStEph.rs | 4 | 4 | 0 | 0 | No | Yes | N/A (free fns) |
| 10 | PathWeightUtilsStPer.rs | 4 | 4 | 0 | 0 | No | Yes | N/A (free fns) |
| 11 | Example56_1.rs | 3 | 3 | 0 | 0 | No | Yes | N/A |
| 12 | Example56_3.rs | 2 | 2 | 0 | 0 | No | Yes | N/A |
| | **Total** | **69** | **52** | **0** | **1** | | | |

**Changes since 2026-02-17:**
- Files renamed: `*Int.rs` → `*I64.rs` (agent2 merge).
- All 8 result files now use `impl Trait for Type` (trait wiring complete). No bare_impl errors remain.
- `SSSPResultStEphFloat.rs` rewritten by agent2: replaces `OrderedF64` with custom `F64Dist` type, adds axioms (`axiom_unreachable_not_finite`), verifies `new()` with loop invariant. 1 clean proof function. Trait wiring not yet applied to this file.
- I64 result files: `new()`, `get_distance`, `get_predecessor`, `is_reachable` now verified (no external_body) — `set_distance`, `set_predecessor`, `extract_path` remain external_body.
- Total external_body reduced from 69 to 52.

**Gating:** Chap56 module itself: `#[cfg(not(feature = "experiments_only"))]`. The 4 core modules (SSSPResultStEphI64, SSSPResultStPerI64, AllPairsResultStEphI64, AllPairsResultStPerI64) compile normally. The remaining 8 modules (Float variants, PathWeightUtils, Examples) are additionally gated behind `#[cfg(feature = "all_chapters")]`.

## Phase 2: Prose Inventory

Source: `prompts/Chap56.txt` (Chapter 56 — Introduction to Shortest Paths)

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 56.1 — Path Weight | Weight of path = sum of edge weights along the path. |
| 2 | Definition 56.2 — Shortest Path and Distance | Shortest path = minimal-weight path; δ_G(u,v) = weight of shortest path. |
| 3 | Definition 56.3 — Sub-Path | A sub-path of a path is itself a path contained within the path. |
| 4 | Definition 56.4 — Sub-Paths Property | Any sub-path of a shortest path is itself a shortest path. |

### Problems

| # | Item | Description |
|---|------|-------------|
| 1 | Problem 56.1 — Single-Pair Shortest Paths | Find shortest path from s to t. |
| 2 | Problem 56.2 — SSSP | Find shortest paths from s to all vertices. |
| 3 | Problem 56.3 — All-Pairs Shortest Paths | Find shortest paths between all pairs. |
| 4 | Problem 56.4 — SSSP+ | SSSP restricted to non-negative weights. |

### Examples

| # | Item | Description |
|---|------|-------------|
| 1 | Example 56.1 | Path weight: ⟨s,a,b,e⟩ = 6, ⟨s,a,b,s⟩ = 10. |
| 2 | Example 56.3 | Negative weight cycle: cycle ⟨s,a,b,s⟩ = −4, distance to e = −∞. |
| 3 | Example 56.4 | Sub-path enumeration (conceptual). |
| 4 | Example 56.5 | Pittsburgh-to-SF sub-paths property (conceptual). |

### Cost Specs

None. Chapter 56 is definitional — algorithms and costs are in Chapters 57-59.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 69 exec functions have APAS/Claude-Opus-4.6 cost annotation pairs.

| # | Function Group | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|---------------|-----------|---------------------|-----------|
| 1 | `SSSP*.new` | Work Θ(n), Span Θ(n) | Work Θ(n), Span Θ(n) | Agree |
| 2 | `SSSP*.get_distance` | Work Θ(1), Span Θ(1) | Work Θ(1), Span Θ(1) | Agree |
| 3 | `SSSP*.set_distance` (Eph) | (no cost stated) | Work Θ(1), Span Θ(1) | N/A |
| 4 | `SSSP*.set_distance` (Per) | (no cost stated) | Work Θ(n), Span Θ(n) | N/A — persistent copy |
| 5 | `SSSP*.get_predecessor` | (no cost stated) | Work Θ(1), Span Θ(1) | N/A |
| 6 | `SSSP*.set_predecessor` (Eph) | (no cost stated) | Work Θ(1), Span Θ(1) | N/A |
| 7 | `SSSP*.set_predecessor` (Per) | (no cost stated) | Work Θ(n), Span Θ(n) | N/A — persistent copy |
| 8 | `SSSP*.is_reachable` | Work Θ(1), Span Θ(1) | Work Θ(1), Span Θ(1) | Agree |
| 9 | `SSSP*.extract_path` | (no cost stated) | Work Θ(k), Span Θ(k) | N/A |
| 10 | `AllPairs*.new` | Work Θ(n²), Span Θ(n²) | Work Θ(n²), Span Θ(n²) | Agree |
| 11 | `AllPairs*.get_distance` | Work Θ(1), Span Θ(1) | Work Θ(1), Span Θ(1) | Agree |
| 12 | `AllPairs*.set_distance` (Eph) | (no cost stated) | Work Θ(n), Span Θ(n) | N/A — clones row |
| 13 | `AllPairs*.set_distance` (Per) | (no cost stated) | Work Θ(n), Span Θ(n) | N/A — persistent update |
| 14 | `AllPairs*.is_reachable` | Work Θ(1), Span Θ(1) | Work Θ(1), Span Θ(1) | Agree |
| 15 | `AllPairs*.extract_path` | (no cost stated) | Work Θ(k), Span Θ(k) | N/A |
| 16 | `path_weight_int` / `_float` | Work Θ(k), Span Θ(k) | Work Θ(k), Span Θ(k) | Agree |
| 17 | `validate_subpath_property_*` | (no cost stated) | Work Θ(k), Span Θ(k) | N/A |
| 18 | Example fns | N/A — demonstration | Work Θ(1), Span Θ(1) | N/A — constant-sized |

**Cost disagreements:**
- Module headers for `PathWeightUtils*.rs` state `validate_subpath_property` as O(k²), but the implementation is O(k) — it checks k-1 consecutive edges, not all pairs. The O(k) annotation on the functions themselves is correct; the module-level header overstates.

### 3b. Implementation Fidelity

Chapter 56 defines concepts, not algorithms. The code implements **data structures** for shortest path results and **utility functions** for path weights.

| # | Code Module | Prose Counterpart | Fidelity |
|---|-------------|-------------------|----------|
| 1 | `PathWeightUtils*.path_weight_*` | Def 56.1 (Path Weight) | **High** — sums edge weights along vertex path. |
| 2 | `PathWeightUtils*.validate_subpath_property_*` | Def 56.4 (Sub-Paths Property) | **High** — validates relaxation on consecutive edges. |
| 3 | `Example56_1` | Example 56.1 | **High** — demonstrates path weight computation. |
| 4 | `Example56_3` | Example 56.3 | **High** — demonstrates negative weight cycles. |
| 5 | `SSSPResult*` | Problem 56.2 (SSSP) | **Scaffolding** — data structure for Chap57-58. |
| 6 | `AllPairsResult*` | Problem 56.3 (All-Pairs) | **Scaffolding** — data structure for Chap59. |

No significant deviations. The code is infrastructure for Chapters 57-59.

### 3c. Spec Fidelity

No `requires`/`ensures` on any function. Spec fidelity: **N/A**.

Key specs that should exist when verusified:
- `path_weight`: result equals sum of edge weights (Def 56.1)
- `extract_path`: returned path starts at source and ends at target
- `set_distance`/`set_predecessor`: returned structure differs only at updated index
- `is_reachable`: equivalent to `distance != UNREACHABLE/INFINITY`

## Phase 4: Parallelism Review

**No Mt modules exist.** All 12 files are St (sequential). Chapter 56 is definitional — no algorithms to parallelize. Not applicable.

## Phase 5: Runtime Test Review

**No runtime test files exist** for Chapter 56.

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | SSSPResultStEphInt | — | Missing |
| 2 | SSSPResultStPerInt | — | Missing |
| 3 | SSSPResultStEphFloat | — | Missing |
| 4 | SSSPResultStPerFloat | — | Missing |
| 5 | AllPairsResultStEphInt | — | Missing |
| 6 | AllPairsResultStPerInt | — | Missing |
| 7 | AllPairsResultStEphFloat | — | Missing |
| 8 | AllPairsResultStPerFloat | — | Missing |
| 9 | PathWeightUtilsStEph | — | Missing |
| 10 | PathWeightUtilsStPer | — | Missing |
| 11 | Example56_1 | — | Missing |
| 12 | Example56_3 | — | Missing |

Proposed priority: (1) SSSPResultStEphInt (used by Chap57-58), (2) AllPairsResultStEphInt (used by Chap59), (3) PathWeightUtilsStEph (core utility), (4) Examples.

## Phase 6: Proof-Time Test (PTT) Review

No verified loops or iterators. No PTTs needed until verusified.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Definition 56.2 — formal spec of δ_G(u,v) | No spec fn defining shortest-path distance. |
| 2 | Example 56.4 — sub-path enumeration | Not implemented (conceptual). |
| 3 | Example 56.5 — Pittsburgh-to-SF | Not implemented (conceptual). |
| 4 | Problem 56.1 — Single-Pair | No dedicated single-pair type (extract_path partially covers). |

### Code With No Prose Counterpart

| # | Code Item | Purpose |
|---|----------|---------|
| 1 | `set_distance` / `set_predecessor` | Mutable scaffolding for algorithms in Chap57-59. |
| 2 | `get_predecessor` | Path reconstruction helper. |
| 3 | `extract_path` | Reconstruct path from predecessor array — implied by SSSP but not explicitly in prose. |
| 4 | Float variants | Prose uses real numbers; code splits into i64 and OrderedFloat<f64>. |
| 5 | Eph/Per variants | Prose doesn't distinguish persistence; code provides both. |

## Phase 8: TOC and In/Out Table

### TOC Presence

| # | File | TOC | Section Headers |
|---|------|:---:|:---------------:|
| 1 | SSSPResultStEphInt.rs | Yes | Yes (4, 5, 8, 9) |
| 2 | SSSPResultStPerInt.rs | Yes | Yes (4, 5, 8, 9) |
| 3 | SSSPResultStEphFloat.rs | Yes | Yes (4, 5, 8, 9) |
| 4 | SSSPResultStPerFloat.rs | Yes | Yes (4, 5, 8, 9) |
| 5 | AllPairsResultStEphInt.rs | Yes | Yes (4, 5, 8, 9) |
| 6 | AllPairsResultStPerInt.rs | Yes | Yes (4, 5, 8, 9) |
| 7 | AllPairsResultStEphFloat.rs | Yes | Yes (4, 5, 8, 9) |
| 8 | AllPairsResultStPerFloat.rs | Yes | Yes (4, 5, 8, 9) |
| 9 | PathWeightUtilsStEph.rs | Yes | Yes (4, 8, 9) |
| 10 | PathWeightUtilsStPer.rs | Yes | Yes (4, 8, 9) |
| 11 | Example56_1.rs | Yes | Yes (8, 9) |
| 12 | Example56_3.rs | Yes | Yes (8, 9) |

All files have TOC comment blocks and section headers.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | SSSPResultStEphInt | - | - | - | - | - | - | - | - | - |
| 2 | SSSPResultStPerInt | - | - | - | - | - | - | - | - | - |
| 3 | SSSPResultStEphFloat | - | - | - | - | - | - | - | - | - |
| 4 | SSSPResultStPerFloat | - | - | - | - | - | - | - | - | - |
| 5 | AllPairsResultStEphInt | - | - | - | - | - | - | - | - | - |
| 6 | AllPairsResultStPerInt | - | - | - | - | - | - | - | - | - |
| 7 | AllPairsResultStEphFloat | - | - | - | - | - | - | - | - | - |
| 8 | AllPairsResultStPerFloat | - | - | - | - | - | - | - | - | - |
| 9 | PathWeightUtilsStEph | - | - | - | - | - | - | - | - | - |
| 10 | PathWeightUtilsStPer | - | - | - | - | - | - | - | - | - |
| 11 | Example56_1 | - | - | - | - | - | - | - | - | - |
| 12 | Example56_3 | - | - | - | - | - | - | - | - | - |

No derive trait impls in any file. No `❌` items.

## Proof Holes Summary

```
Modules: 0 clean, 12 holed
Proof Functions: 1 clean, 0 holed
Holes Found: 52 total (all external_body)

SSSPResultStEphI64.rs:         4 × external_body  (was 7)
SSSPResultStPerI64.rs:         3 × external_body  (was 7)
SSSPResultStEphFloat.rs:       5 × external_body  (was 7; F64Dist axioms + verified new)
SSSPResultStPerFloat.rs:       7 × external_body
AllPairsResultStEphI64.rs:     2 × external_body  (was 7)
AllPairsResultStPerI64.rs:     4 × external_body  (was 7)
AllPairsResultStEphFloat.rs:   7 × external_body
AllPairsResultStPerFloat.rs:   7 × external_body
PathWeightUtilsStEph.rs:       4 × external_body
PathWeightUtilsStPer.rs:       4 × external_body
Example56_1.rs:                3 × external_body
Example56_3.rs:                2 × external_body
```

## Action Items

| # | Action | Priority |
|---|--------|----------|
| 1 | Wire trait for SSSPResultStEphFloat (agent2 rewrote file; trait wiring not yet applied) | High |
| 2 | Remove remaining `external_body` from I64 result files (set_distance, set_predecessor, extract_path) | High |
| 3 | Add runtime tests (SSSPResultStEphI64 first, then AllPairsResultStEphI64) | High |
| 4 | Add `spec fn spec_distance` for δ_G(u,v) | Medium |
| 5 | Remove `external_body` from Float result files and verify | Medium |
| 6 | Remove `external_body` from PathWeightUtils* and verify | Medium |
| 7 | Fix module header cost for `validate_subpath_property` (O(k²) → O(k)) | Low |
| 8 | Remove `external_body` from Example fns (demonstration code) | Low |
