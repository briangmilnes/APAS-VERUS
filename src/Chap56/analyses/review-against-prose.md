# Review Against Prose: Chapter 56 -- Shortest Path Results

- Date: 2026-03-15
- Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)
- Prose source: `prompts/Chap56.txt` (APAS Chapter 56: Introduction to Shortest Paths)

## Phase 1: Inventory

From `veracity-review-module-fn-impls.md`:

| # | Chap | File | Fns | Tr | IT | V! | Holes | NoSpec |
|---|------|------|-----|----|----|----|----|--------|
| 1 | 56 | SSSPResultStEphI64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 2 | 56 | SSSPResultStEphF64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 3 | 56 | SSSPResultStPerI64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 4 | 56 | SSSPResultStPerF64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 5 | 56 | AllPairsResultStEphI64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 6 | 56 | AllPairsResultStEphF64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 7 | 56 | AllPairsResultStPerI64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 8 | 56 | AllPairsResultStPerF64.rs | 7 | 7 | 7 | 7 | 0 | 0 |
| 9 | 56 | PathWeightUtilsStEph.rs | 4 | 4 | 4 | 4 | 0 | 2 |
| 10 | 56 | PathWeightUtilsStPer.rs | 4 | 4 | 4 | 4 | 0 | 2 |
| 11 | 56 | Example56_1.rs | 3 | 3 | 3 | 3 | 0 | 3 |
| 12 | 56 | Example56_3.rs | 2 | 2 | 2 | 2 | 0 | 2 |

Total: 69 exec functions, 0 holes, 12 clean modules.

## Phase 2: Prose Inventory

Chapter 56 is an **introductory/definitional chapter**. It defines concepts but presents
no algorithms. Named items from the prose:

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 56 | Definition 56.1 (Path Weight) | definition |
| 2 | 56 | Definition 56.2 (Shortest Paths and Distance) | definition |
| 3 | 56 | Definition 56.3 (Sub-Path) | definition |
| 4 | 56 | Definition 56.4 (Sub-Paths Property) | definition |
| 5 | 56 | Example 56.1 (path weight) | example |
| 6 | 56 | Example 56.3 (negative weights) | example |
| 7 | 56 | Example 56.4 (sub-paths) | example |
| 8 | 56 | Example 56.5 (sub-paths property) | example |
| 9 | 56 | Problem 56.1 (Single-Pair SP) | problem |
| 10 | 56 | Problem 56.2 (SSSP) | problem |
| 11 | 56 | Problem 56.3 (All-Pairs SP) | problem |
| 12 | 56 | Problem 56.4 (SSSP+) | problem |

No algorithms or cost analyses in Chapter 56.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations added to all exec functions in:
- `SSSPResultStEphI64.rs`: new, get/set_distance, get/set_predecessor, is_reachable, extract_path
- `AllPairsResultStEphI64.rs`: new, get/set_distance, get/set_predecessor, is_reachable, extract_path

These are data structure scaffolding. The prose does not state costs for result structures.
PathWeightUtilsStEph.rs and PathWeightUtilsStPer.rs already had module-level cost comments.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose | Fidelity |
|---|------|------|----------|-------|----------|
| 1 | 56 | SSSPResultStEphI64.rs | new | Problem 56.2 | Faithful: initializes D with d(s)=0, d(v)=infinity |
| 2 | 56 | AllPairsResultStEphI64.rs | new | Problem 56.3 | Faithful: n x n matrices, diagonal=0, rest=infinity |
| 3 | 56 | PathWeightUtilsStEph.rs | path_weight_int | Def 56.1 | Faithful: sums edge weights along a path |
| 4 | 56 | PathWeightUtilsStEph.rs | validate_subpath_property_int | Def 56.4 | Faithful: checks sub-path optimality along path |

No deviations from prose. The result structures are reasonable Verus representations of
the distance and predecessor arrays described in the SSSP and APSP problem statements.

**UNREACHABLE sentinel**: The prose uses infinity for unreachable vertices. The code uses
`i64::MAX` for I64 variants and `WrappedF64` (positive infinity) for F64 variants. This is
an appropriate finite representation.

### 3c. Spec Fidelity

**I64 SSSP Result specs** (SSSPResultStEphI64, SSSPResultStPerI64):
- `new`: Strong. Ensures distances[source] = 0, distances[v] = UNREACHABLE for v != source, predecessors all NO_PREDECESSOR, correct lengths.
- `get_distance` / `set_distance`: Strong. Full pointwise ensures for reads and updates.
- `get_predecessor` / `set_predecessor`: Strong. Full pointwise ensures with NO_PREDECESSOR sentinel handling.
- `is_reachable`: Strong. Ensures result == (distance != UNREACHABLE).
- `extract_path`: Strong. Ensures path starts at source, ends at v, all indices in bounds.

**I64 AllPairs Result specs** (AllPairsResultStEphI64, AllPairsResultStPerI64):
- All 7 functions: Strong. 2D pointwise ensures for get/set, correct initialization, path extraction with start/end guarantees.

**F64 variants**: Specs are slightly weaker than I64 counterparts:
- F64 AllPairs `get_distance` / `set_distance` / `get_predecessor` / `set_predecessor`: ensure wf preservation and n preservation but lack pointwise distance/predecessor content ensures.
- F64 `extract_path`: ensures only `wf` precondition, no path content postconditions.
- F64 SSSP: slightly stronger than AllPairs F64 -- has pointwise distance content ensures but `extract_path` lacks path content postconditions.

**PathWeightUtils specs**: `path_weight_int` and `validate_subpath_property_int` have spec functions connected to ensures. Float variants (`path_weight_float`, `validate_subpath_property_float`) have no spec -- NoSpec.

**StPer vs StEph**: StPer variants use `self` (consuming) for set operations and return `updated: Self`. Specs are structurally identical to StEph in strength.

### F64 vs I64 Differences Summary

| # | Chap | Aspect | I64 | F64 |
|---|------|--------|-----|-----|
| 1 | 56 | Distance type | i64 | WrappedF64 |
| 2 | 56 | Unreachable sentinel | i64::MAX const | unreachable_dist() fn |
| 3 | 56 | is_reachable check | dist != UNREACHABLE | dist.is_finite() |
| 4 | 56 | AllPairs spec strength | Strong pointwise | Weak (wf only) |
| 5 | 56 | extract_path ensures | Path content | None (wf only) |
| 6 | 56 | PathWeightUtils float | N/A | NoSpec |

## Phase 4: Parallelism Review

No Mt modules in Chapter 56. All files are StEph or StPer. No parallelism expected (data
structures only).

## Phase 5: Runtime Test Review

12 test files covering all 12 modules:

| # | Chap | Test File | Covers |
|---|------|-----------|--------|
| 1 | 56 | TestSSSPResultStEphI64.rs | SSSPResultStEphI64 |
| 2 | 56 | TestSSSPResultStEphF64.rs | SSSPResultStEphF64 |
| 3 | 56 | TestSSSPResultStPerI64.rs | SSSPResultStPerI64 |
| 4 | 56 | TestSSSPResultStPerF64.rs | SSSPResultStPerF64 |
| 5 | 56 | TestAllPairsResultStEphI64.rs | AllPairsResultStEphI64 |
| 6 | 56 | TestAllPairsResultStEphF64.rs | AllPairsResultStEphF64 |
| 7 | 56 | TestAllPairsResultStPerI64.rs | AllPairsResultStPerI64 |
| 8 | 56 | TestAllPairsResultStPerF64.rs | AllPairsResultStPerF64 |
| 9 | 56 | TestPathWeightUtilsStEph.rs | PathWeightUtilsStEph |
| 10 | 56 | TestPathWeightUtilsStPer.rs | PathWeightUtilsStPer |
| 11 | 56 | TestExample56_1.rs | Example56_1 |
| 12 | 56 | TestExample56_3.rs | Example56_3 |

Full RTT coverage. Every module has a corresponding test file.

## Phase 6: PTT Review

No proof-time tests for Chapter 56. None needed -- no iterators and no complex
callability patterns.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 56 | F64 AllPairs specs are weak | Low | Only wf preservation; lacks pointwise content ensures. F64 arithmetic axiom gap blocks strengthening. |
| 2 | 56 | F64 extract_path has no content ensures | Low | Path start/end/bounds not specified for F64 variants. Could be strengthened without float arithmetic. |
| 3 | 56 | PathWeightUtils float fns have NoSpec | Low | path_weight_float and validate_subpath_property_float lack ensures. Float arithmetic spec gap. |
| 4 | 56 | No spec_distance connecting to prose delta_G | Medium | No spec function formalizing "delta_G(s,v) = shortest path weight from s to v". Result structures store distances but specs don't state they represent shortest paths. |

**Overall assessment**: Chapter 56 is **clean** (0 holes) with strong I64 specs and
adequate F64 specs. The chapter is definitional in the prose -- no algorithms, just
problem definitions and data structures. The implementation correctly provides SSSP and
AllPairs result containers for Chapters 57-59 to use.

## Phase 8: TOC Review

All files follow the standard TOC ordering (4. type definitions, 8. traits, 9. impls,
13. derive impls outside verus!). PathWeightUtils files additionally include section 2
(imports). No ordering violations detected.
