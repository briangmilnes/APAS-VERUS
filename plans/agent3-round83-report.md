# Agent 3 Round 83 Report — SCC Proof (6 holes removed)

## Objective
Remove all 6 `external_body` annotations from SCCStEph.rs and SCCStPer.rs (Chapter 55).

## Result: 6/6 holes closed

| # | Chap | File | Function | Holes Before | Holes After |
|---|------|------|----------|--------------|-------------|
| 1 | 55 | SCCStEph.rs | `compute_finish_order` | 1 | 0 |
| 2 | 55 | SCCStEph.rs | `transpose_graph` | 1 | 0 |
| 3 | 55 | SCCStEph.rs | `scc` | 1 | 0 |
| 4 | 55 | SCCStPer.rs | `compute_finish_order` | 1 | 0 |
| 5 | 55 | SCCStPer.rs | `transpose_graph` | 1 | 0 |
| 6 | 55 | SCCStPer.rs | `scc` | 1 | 0 |

## Verification
- `scripts/validate.sh isolate Chap55`: **2104 verified, 0 errors, 0 trigger warnings**
- Both SCCStEph.rs and SCCStPer.rs are now fully clean per veracity

## Techniques Used

### compute_finish_order (both variants)
- **Named closure with explicit ensures** for tabulate: `|_x: usize| -> (r: B) ensures !r { false }` — anonymous closures don't propagate ensures through tabulate's `f.ensures` bridge
- **Ghost pre-snapshot** (`let ghost pre_vis = visited@`) for monotonicity proofs through DFS calls: after `dfs_finish_order`, visited@[j] for j < start is maintained via `pre_vis[j]` + dfs monotonicity ensures
- **Capacity bound** `graph@.len() < usize::MAX` added to requires — needed for `AVLTreeSeqStEphS::from_vec` which requires `values@.len() < usize::MAX`; propagated to trait `scc` requires

### transpose_graph (both variants)
- **Ghost clone capture** for from_vec view bridge: `let ghost cv_view = cloned_vec@` before `from_vec(cloned_vec)` preserves the connection between the consumed Vec's view and `adj_vecs@[m]@`
- **View bridge chain**: `new_arr@[j] == new_arr.spec_index(j) == cv_view[j] == adj_vecs@[m]@[j]` with `lemma_usize_view_eq_spec_index` providing `@` ↔ `spec_index`
- **Missing invariant** in StPer: `n == graph@.len()` was absent from the result_vecs loop, preventing Z3 from connecting `n` to `graph@.len()` inside the loop body

### scc (both variants)
- **Restructured** to handle first vertex before the main loop: process `finish_order[0]` unconditionally, push the component, then loop from i=1. This guarantees `components_vec@.len() >= 1` when `graph@.len() > 0`
- **Added `visited@[vertex as int]`** to `dfs_reach` ensures — needed to track that the vertex is visited after the call; maintained through recursive neighbor loop via ghost pre-snapshot + monotonicity
- **obeys_feq_full_trigger** assertion for `AVLTreeSetStEph<N>` before `AVLTreeSeqStEphS::from_vec` — triggers the broadcast axiom

## Steps Used: 11 (of 20 budget)
