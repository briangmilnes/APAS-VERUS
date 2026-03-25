# R75 Agent 2 Report

## Summary

Eliminated 6 of 12 targeted holes across 3 files. All changes verify clean (4751 verified,
0 errors) and all 2619 RTT pass.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 64 | SpanTreeStEph.rs | 2 | 2 | 0 |
| 2 | 64 | TSPApproxStEph.rs | 4 | 2 | -2 |
| 3 | 66 | BoruvkaStEph.rs | 6 | 2 | -4 |
| **Total** | | | **12** | **6** | **-6** |

## Changes Made

### TSPApproxStEph.rs (Chap64) — 2 holes eliminated

- **`get_neighbors`**: Removed `external_body`. Body is `graph.ng(v)` which already has
  matching ensures. Added `valid_key_type_LabEdge::<V, WrappedF64>()` to requires.
- **`get_edge_weight`**: Removed `external_body`. Body delegates to `get_edge_label`.
  Added `valid_key_type_LabEdge::<V, WrappedF64>()` to requires.
- Propagated `valid_key_type_LabEdge::<V, WrappedF64>()` to callers: `euler_tour`,
  `euler_tour_dfs`, `tour_weight`, `approx_metric_tsp` (requires + trait declarations).

### BoruvkaStEph.rs (Chap66) — 4 holes eliminated

- **PartialEq for `LabeledEdge<V>`**: Changed from `#[verifier::external]` (no spec) to
  `#[verifier::external_body]` on `fn eq` with `ensures equal == (self@ == other@)`.
  Added `PartialEqSpecImpl` bridge. Changed bounds to `V: StT + Ord + Copy`. Veracity
  now classifies as `structural_false_positive STD_TRAIT_IMPL` (info, not a hole).
- **`bridge_star_partition`**: Removed `external_body`. Three-phase loop (coin flips,
  partition selection, remaining vertices) verifies with iterator invariants.
- **`boruvka_mst_with_seed`**: Removed `external_body`. Simple delegation to `boruvka_mst`.
- **`mst_weight`**: Removed `external_body`. Added `mst_labels.spec_setsteph_wf()` to
  loop invariant to satisfy `mem()` precondition.

### SpanTreeStEph.rs (Chap64) — unchanged

No changes. Both holes blocked by upstream API limitations:
- `spanning_tree_star_contraction`: `star_contract` requires closures with universally
  quantified requires (always-true), but expand closure needs parameter-dependent requires.
- `verify_spanning_tree`: `graph.edges()` doesn't ensure `spec_setsteph_wf()` on return.

## Remaining Holes (6)

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction | star_contract closure interface |
| 2 | 64 | SpanTreeStEph.rs | verify_spanning_tree | edges() missing wf ensures |
| 3 | 64 | TSPApproxStEph.rs | euler_tour_dfs | DFS with mutable visited set |
| 4 | 64 | TSPApproxStEph.rs | euler_tour | Blocked by euler_tour_dfs |
| 5 | 66 | BoruvkaStEph.rs | vertex_bridges | Iterator→set membership→finiteness chain |
| 6 | 66 | BoruvkaStEph.rs | boruvka_mst | Recursive, needs decreases (termination proof) |

## Verification

- `scripts/validate.sh`: 4751 verified, 0 errors (83s)
- `scripts/rtt.sh`: 2619 passed, 0 skipped (13s)
