# Agent 4 — R89 Chap61 Report

## Objective

Fix 12 verification errors across 3 files in Chap61 (EdgeContraction + VertexMatching).

## Result

All 12 errors fixed. 0 Chap61 holes. 4/4 modules clean.

## Errors Before / After

| # | Chap | File | Errors Before | Errors After |
|---|------|------|:---:|:---:|
| 1 | 61 | EdgeContractionStEph.rs | 7 | 0 |
| 2 | 61 | VertexMatchingStEph.rs | 2 | 0 |
| 3 | 61 | VertexMatchingMtEph.rs | 3 | 0 |

## Verification

- `scripts/validate.sh isolate Chap61`: 1247 verified, 3 errors (all in `experiments/f32_ieee_total_order.rs`, pre-existing)
- `scripts/holes.sh src/Chap61/`: 0 actionable holes, 4 clean modules
- RTT build has pre-existing failures (float specs, CycleDetect, closure borrow) — none from Chap61 changes

## Root Cause

Two issues across all 12 errors:

1. **Missing `spec_setsteph_wf()` preconditions.** `SetStEph::iter()` and `SetStEph::to_seq()` require `self.spec_setsteph_wf()`, but the graph functions didn't propagate this. The graph fields (`graph.E`, `graph.V`) and the `matching` parameter all needed explicit wf in `requires`.

2. **Verus for-loop iterator isolation.** Chained calls like `graph.edges().iter()` create temporaries that Verus can't track across loop iterations. Fixed by: (a) storing the collection reference and iterator in named variables, (b) using the `for x in iter: it` named-iterator pattern, (c) adding `#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]`.

## Changes

### EdgeContractionStEph.rs (7 errors fixed)
- Added `matching.spec_setsteph_wf()`, `graph.E.spec_setsteph_wf()`, `graph.V.spec_setsteph_wf()` to `edge_contract` requires
- Added `graph.E.spec_setsteph_wf()`, `graph.V.spec_setsteph_wf()` to `contract_round` requires
- Converted all 4 for-loops to named-iterator pattern with `loop_isolation(false)`
- Added appropriate wf invariants to each loop

### VertexMatchingStEph.rs (2 errors fixed)
- Added `graph.E.spec_setsteph_wf()` to `greedy_matching` requires
- Added `matching.spec_setsteph_wf()` to `greedy_matching` ensures (needed by `contract_round`)
- Added `graph.E.spec_setsteph_wf()` to `parallel_matching_st` requires
- Converted `greedy_matching`'s for-loop to named-iterator pattern with `loop_isolation(false)`

### VertexMatchingMtEph.rs (3 errors fixed)
- Added `graph.E.spec_setsteph_wf()` to `parallel_matching_mt` and `should_select_edge` requires
- Converted `should_select_edge`'s for-loop to named-iterator pattern with `loop_isolation(false)`

### EdgeContractionMtEph.rs (minimal change)
- Added `graph.E.spec_setsteph_wf()` to `contract_round_mt` requires (propagation from changed `parallel_matching_mt`)

## Techniques

- **Named iterator pattern**: `let it = collection.iter(); for x in iter: it` — avoids Verus losing track of temporary references
- **`loop_isolation(false)`**: Allows pre-loop context (requires) to flow through for-loops without restating in invariants
- **Precondition propagation**: Added `spec_setsteph_wf()` on graph fields to function requires, threaded through callers
