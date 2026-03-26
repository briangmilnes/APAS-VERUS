# Agent 3 Round 82 Report — Chap52+53 AVLTreeSet API Refactor

## Summary

Refactored Chap52 and Chap53 files to work with the current AVLTreeSet API.
Added `PartialOrd`/`Ord` impls to `AVLTreeSetStEph` and `AVLTreeSetStPer` (needed
as `OrderedTable` value types). Fixed `GraphSearchMtPer` to use `to_seq()` instead
of removed `.elements` field.

## Files Modified

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | AVLTreeSetStEph.rs | Added `PartialOrd` + `Ord` impls (outside verus!, lexicographic) |
| 2 | 41 | AVLTreeSetStPer.rs | Added `PartialOrd` + `Ord` impls (outside verus!, lexicographic) |
| 3 | 52 | EdgeSetGraphStEph.rs | Fields made `pub`, added `reject_recursive_types`, `Sized` on trait, Ord laws in wf, capacity bounds, `view_ord_consistent` import, seq wf invariants |
| 4 | 52 | EdgeSetGraphStPer.rs | Same as StEph variant, added `spec_vertices`/`spec_edges` to trait |
| 5 | 52 | AdjTableGraphStEph.rs | Added `when` clause to `spec_sum_adj_sizes` termination |
| 6 | 53 | GraphSearchMtPer.rs | Replaced `frontier.elements` with `frontier.to_seq()`, added `spec_avltreeseqmtper_wf()` to loop invariant, added `frontier.spec_avltreesetmtper_wf()` to `SelectionStrategy::select` requires |
| 7 | — | lib.rs | Uncommented EdgeSetGraphStEph, EdgeSetGraphStPer, GraphSearchMtPer; updated BROKEN comments on AdjTableGraph files |

## Files Uncommented (3)

| # | Chap | File | Status |
|---|------|------|--------|
| 1 | 52 | EdgeSetGraphStEph.rs | Compiles, 4 verification holes |
| 2 | 52 | EdgeSetGraphStPer.rs | Compiles, 4 verification holes |
| 3 | 53 | GraphSearchMtPer.rs | Clean (0 errors) |

## Files Still Commented Out (4)

| # | Chap | File | Why |
|---|------|------|-----|
| 1 | 52 | AdjTableGraphStEph.rs | 44 verif errors; Z3 diverges when table wf is exposed |
| 2 | 52 | AdjTableGraphStPer.rs | 44 verif errors; same Z3 divergence |
| 3 | 52 | AdjTableGraphMtPer.rs | Depends on AVLTreeSetMtPer chain (separate refactor) |
| 4 | 52 | EdgeSetGraphMtPer.rs | Depends on AVLTreeSetMtPer chain (separate refactor) |

## Verification Holes (8 total, all in EdgeSetGraph)

| # | Chap | File | Line | Type | What blocks it |
|---|------|------|------|------|----------------|
| 1 | 52 | EdgeSetGraphStEph.rs | 187 | precondition | filter closure/ghost spec_pred consistency |
| 2 | 52 | EdgeSetGraphStEph.rs | 210,212 | loop invariant | depends on filter proof |
| 3 | 52 | EdgeSetGraphStEph.rs | 224 | precondition | insert capacity in insert_edge nested calls |
| 4 | 52 | EdgeSetGraphStEph.rs | 114,119,127,132 | postcondition | wf preservation through mutations |
| 5 | 52 | EdgeSetGraphStPer.rs | 153 | precondition | filter closure/ghost spec_pred consistency |
| 6 | 52 | EdgeSetGraphStPer.rs | 178,190 | loop invariant | depends on filter proof |
| 7 | 52 | EdgeSetGraphStPer.rs | 271,272 | assertion | delete_vertex filter proof (a != v → new_vertices.contains(a)) |
| 8 | 52 | EdgeSetGraphStPer.rs | 222,262,286 | postcondition | wf preservation through mutations |

All holes are pre-existing proof gaps (not regressions from the API refactor).
The filter-closure consistency issue is the root: Verus needs proof that
`(edge.0 == u_clone) ↔ (edge_view.0 == u@)` which requires PartialEq/View consistency.

## Verification Counts

- **Before**: 4680 verified, 0 errors
- **After**: 4714 verified, 8 errors (+34 verified from new files)
- **PTT**: 157 passed, 0 failed
- **RTT**: Pre-existing compilation errors (not from this round)

## Techniques Used

- Lexicographic `Ord` implementation (matching AVLTreeSetMtPer pattern)
- Wf predicate augmentation with Ord consistency laws
- `to_seq()` API migration from removed `.elements` field
- Capacity bounds propagation for insert operations
- `reject_recursive_types` annotation for recursive type parameter
