<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 56 — Shortest Paths (Introduction): Review Against Prose

**Date:** 2026-02-18 (updated 2026-02-18)
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory

| # | File | exec fns | external_body | spec fns | proof fns | View | verus! | Trait Wired |
|---|------|:--------:|:-------------:|:--------:|:---------:|:----:|:------:|:-----------:|
| 1 | SSSPResultStEphI64.rs | 7 | 0 | 0 | 0 | Yes | Yes | Yes |
| 2 | SSSPResultStPerI64.rs | 7 | 0 | 0 | 0 | Yes | Yes | Yes |
| 3 | SSSPResultStEphFloat.rs | 7 | 5 | 2 | 1 | Yes | Yes | Yes |
| 4 | SSSPResultStPerFloat.rs | 7 | 7 | 0 | 0 | Yes | Yes | Yes |
| 5 | AllPairsResultStEphI64.rs | 7 | 0 | 0 | 0 | Yes | Yes | Yes |
| 6 | AllPairsResultStPerI64.rs | 7 | 0 | 0 | 0 | Yes | Yes | Yes |
| 7 | AllPairsResultStEphFloat.rs | 7 | 7 | 0 | 0 | Yes | Yes | Yes |
| 8 | AllPairsResultStPerFloat.rs | 7 | 7 | 0 | 0 | Yes | Yes | Yes |
| 9 | PathWeightUtilsStEph.rs | 4 | 2 | 0 | 0 | No | Yes | N/A (free fns) |
| 10 | PathWeightUtilsStPer.rs | 4 | 2 | 0 | 0 | No | Yes | N/A (free fns) |
| 11 | Example56_1.rs | 3 | 3 | 0 | 0 | No | Yes | N/A |
| 12 | Example56_3.rs | 2 | 2 | 0 | 0 | No | Yes | N/A |
| | **Total** | **69** | **35** | **2** | **1** | | | |

**Changes since last review:**
- **All I64 files now fully verified (0 external_body):**
  - SSSPResultStEphI64: `new` rewritten with while loops (bypasses `obeys_feq_clone`), `set_distance`/`set_predecessor` use direct `Vec::set`, `extract_path` uses step counter + manual reverse.
  - SSSPResultStPerI64: `set_distance`/`set_predecessor` use direct `Vec::set` (bypasses `ArraySeqStPerS::update` and its `obeys_feq_clone` requirement), `extract_path` same pattern as Eph.
  - AllPairsResultStEphI64: `new` uses nested while loops, `extract_path` with step counter + bounds checks + manual reverse.
  - AllPairsResultStPerI64: `new` uses nested while loops, `set_distance`/`set_predecessor` use `Vec::clone` + `Vec::set`, `extract_path` same pattern.
- **PathWeightUtils int functions verified:** `path_weight_int` and `validate_subpath_property_int` in both StEph and StPer now verified. `for` loops → `while` with invariants, `saturating_add` → `checked_add` (spec'd for i64 in vstd). Float variants remain `external_body` (OrderedFloat arithmetic not spec'd).
- **SSSPResultStEphFloat.rs:** 5 external_body (down from 6). F64Dist trait functions remain `external_body` (f64 operations not verifiable).
- **Total external_body:** 35 (down from 53, reduced by 18).

**Gating:** Chap56: `#[cfg(not(any(feature = "experiments_only", feature = "dev_only")))]`. SSSPResultStEphI64, SSSPResultStPerI64, AllPairsResultStEphI64, AllPairsResultStPerI64, SSSPResultStEphFloat are **not** behind `all_chapters`. PathWeightUtilsStEph, PathWeightUtilsStPer, SSSPResultStPerFloat, AllPairsResultStEphFloat, AllPairsResultStPerFloat, Example56_1, Example56_3 are behind `#[cfg(feature = "all_chapters")]`.

**Verus verification:** 1811 verified, 0 errors (full build). All I64 files and PathWeightUtils int functions fully verified.

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

I64 result files now have `ensures` on key functions (new, set_distance, set_predecessor). SSSPResultStEphFloat has spec fns (`spec_is_finite`, `UNREACHABLE_SPEC`) and verified `new`. PathWeightUtils int functions verified but lack postconditions. Spec fidelity: **partial** (improving).

Key specs that should exist when verusified:
- `path_weight`: result equals sum of edge weights (Def 56.1)
- `extract_path`: returned path starts at source and ends at target
- `set_distance`/`set_predecessor`: returned structure differs only at updated index
- `is_reachable`: equivalent to `distance != UNREACHABLE/INFINITY`

## Phase 4: Parallelism Review

**No Mt modules exist.** All 12 files are St (sequential). Chapter 56 is definitional — no algorithms to parallelize. Not applicable.

## Phase 5: Runtime Test Review

**All 12 source modules now have runtime tests.**

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | SSSPResultStEphI64 | TestSSSPResultStEphI64.rs | Present |
| 2 | SSSPResultStPerI64 | TestSSSPResultStPerI64.rs | Present |
| 3 | SSSPResultStEphFloat | TestSSSPResultStEphFloat.rs | Present |
| 4 | SSSPResultStPerFloat | TestSSSPResultStPerFloat.rs | Present |
| 5 | AllPairsResultStEphI64 | TestAllPairsResultStEphI64.rs | Present |
| 6 | AllPairsResultStPerI64 | TestAllPairsResultStPerI64.rs | Present |
| 7 | AllPairsResultStEphFloat | TestAllPairsResultStEphFloat.rs | Present |
| 8 | AllPairsResultStPerFloat | TestAllPairsResultStPerFloat.rs | Present |
| 9 | PathWeightUtilsStEph | TestPathWeightUtilsStEph.rs | Present |
| 10 | PathWeightUtilsStPer | TestPathWeightUtilsStPer.rs | Present |
| 11 | Example56_1 | TestExample56_1.rs | Present |
| 12 | Example56_3 | TestExample56_3.rs | Present |

## Phase 6: Proof-Time Test (PTT) Review

Multiple verified while loops now exist: `new` functions in all 4 I64 result files (with nested loops for AllPairs), `extract_path` in all 4 I64 result files (step counter + manual reverse), and `path_weight_int`/`validate_subpath_property_int` in both PathWeightUtils files. SSSPResultStEphFloat also has a verified `new`. No PTTs yet; consider adding when specs are strengthened.

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
| 4 | Float variants | Prose uses real numbers; code splits into i64 and OrderedFloat<f64>/F64Dist. |
| 5 | Eph/Per variants | Prose doesn't distinguish persistence; code provides both. |

## Phase 8: TOC and In/Out Table

### TOC Presence

| # | File | TOC | Section Headers |
|---|------|:---:|:---------------:|
| 1 | SSSPResultStEphI64.rs | Yes | Yes (4, 5, 8, 9) |
| 2 | SSSPResultStPerI64.rs | Yes | Yes (4, 5, 8, 9) |
| 3 | SSSPResultStEphFloat.rs | Yes | Yes (4, 5, 6, 7, 9, 13) |
| 4 | SSSPResultStPerFloat.rs | Yes | Yes (4, 5, 8, 9) |
| 5 | AllPairsResultStEphI64.rs | Yes | Yes (4, 5, 8, 9) |
| 6 | AllPairsResultStPerI64.rs | Yes | Yes (4, 5, 8, 9) |
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
| 1 | SSSPResultStEphI64 | - | - | - | - | - | - | - | - | - |
| 2 | SSSPResultStPerI64 | - | - | - | - | - | - | - | - | - |
| 3 | SSSPResultStEphFloat | ✅ in (F64Dist) | ✅ out (F64Dist) | - | - | - | ✅ out (F64Dist) | - | - | - |
| 4 | SSSPResultStPerFloat | - | - | - | - | - | - | - | - | - |
| 5 | AllPairsResultStEphI64 | - | - | - | - | - | - | - | - | - |
| 6 | AllPairsResultStPerI64 | - | - | - | - | - | - | - | - | - |
| 7 | AllPairsResultStEphFloat | - | - | - | - | - | - | - | - | - |
| 8 | AllPairsResultStPerFloat | - | - | - | - | - | - | - | - | - |
| 9 | PathWeightUtilsStEph | - | - | - | - | - | - | - | - | - |
| 10 | PathWeightUtilsStPer | - | - | - | - | - | - | - | - | - |
| 11 | Example56_1 | - | - | - | - | - | - | - | - | - |
| 12 | Example56_3 | - | - | - | - | - | - | - | - | - |

SSSPResultStEphFloat has F64Dist with Clone (in verus!), PartialEq and Debug (outside verus!). No `❌` items.

## Proof Holes Summary

```
Modules: 4 clean, 8 holed
Proof Functions: 1 clean, 0 holed
Holes Found: 35 total (all external_body)

AllPairsResultStEphFloat.rs:     7 × external_body
AllPairsResultStEphI64.rs:       ✓ clean
AllPairsResultStPerFloat.rs:     7 × external_body
AllPairsResultStPerI64.rs:       ✓ clean
Example56_1.rs:                  3 × external_body
Example56_3.rs:                  2 × external_body
PathWeightUtilsStEph.rs:         2 × external_body (float variants only)
PathWeightUtilsStPer.rs:         2 × external_body (float variants only)
SSSPResultStEphFloat.rs:         5 × external_body, 1 clean proof fn
SSSPResultStEphI64.rs:           ✓ clean
SSSPResultStPerFloat.rs:         7 × external_body
SSSPResultStPerI64.rs:           ✓ clean
```

## Action Items

| # | Action | Priority |
|---|--------|----------|
| 1 | Wire trait for SSSPResultStEphFloat (apply trait-impl pattern) | High |
| 2 | Remove `external_body` from Float result files (requires OrderedFloat spec support) | Medium |
| 3 | Add `spec fn spec_distance` for δ_G(u,v) | Medium |
| 4 | Strengthen postconditions on verified functions (e.g., `extract_path` → ensures path starts at source) | Medium |
| 5 | Fix module header cost for `validate_subpath_property` (O(k²) → O(k)) | Low |
| 6 | Remove `external_body` from Example fns (demonstration code) | Low |
