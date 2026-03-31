# Agent 4 — R119 Report: Chap53 GraphSearch spec strengthening

## Summary

Strengthened `GraphSearchMtPer.rs` specs to match `GraphSearchStPer.rs`.
All 18 `veracity-compare-par-mut` warnings resolved. Zero new holes, zero
new assumes.

## Changes

### `src/Chap53/GraphSearchMtPer.rs`

Added `Ghost(vertex_universe)` parameter to all 3 trait functions, all 3 free
functions, all 3 trait impl delegations, and the internal `graph_search_explore`
helper. Added 5 missing requires clauses per function:

| # | Requires clause added | Functions |
|---|----------------------|-----------|
| 1 | `graph.ensures ==> r.spec_avltreesetmtper_wf()` | all 3 |
| 2 | `vertex_universe.finite()` | all 3 |
| 3 | `vertex_universe.len() + vertex_universe.len() < usize::MAX as nat` | all 3 |
| 4 | `vertex_universe.contains(source@)` / `sources@.subset_of(vertex_universe)` | graph_search, reachable / graph_search_multi |
| 5 | `graph.ensures ==> r@.subset_of(vertex_universe)` | all 3 |

Added vertex_universe invariants and subset proofs to `graph_search_explore`
loop (matching the StPer pattern). Added `graph_search` singleton subset proof.

### `tests/Chap53/TestGraphSearchMtPer.rs`

Updated all 7 RTT call sites to pass `Ghost::assume_new()` for the new
ghost parameter. Added `use vstd::prelude::*;` import.

## Warning counts

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 53 | GraphSearchMtPer.rs | 18 | 0 |

## Verification

- `validate.sh isolate Chap53`: 2112 verified, 0 Chap53 errors.
  Pre-existing rlimit in Chap37 `insert_at_link` (same before/after).
- `rtt.sh`: 3529 passed, 0 failed.

## Proof hole status

The existing assume at line 179 (`assume(neighbors.spec_avltreesetmtper_wf())`)
was NOT touched per instructions (agent2's target).
