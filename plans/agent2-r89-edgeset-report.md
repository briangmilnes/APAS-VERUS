# R89 Agent 2 Report: Prove EdgeSetGraphMtPer holes (Chap52)

## Objective

Remove external_body from 6 functions in `src/Chap52/EdgeSetGraphMtPer.rs`.

## Result

**5 of 6 external_body removed.** 1 remains (`default`) — cannot add `requires`
to std `Default` trait impl.

## Holes Before/After

| # | Chap | File | Function | Before | After | Technique |
|---|------|------|----------|--------|-------|-----------|
| 1 | 52 | EdgeSetGraphMtPer.rs | out_neighbors | external_body | proven | iterate+insert via to_seq |
| 2 | 52 | EdgeSetGraphMtPer.rs | out_degree | external_body | proven | delegates to proven out_neighbors |
| 3 | 52 | EdgeSetGraphMtPer.rs | delete_vertex | external_body | proven | iterate+insert via to_seq |
| 4 | 52 | EdgeSetGraphMtPer.rs | insert_edge | external_body | proven | split chained inserts, clone_view |
| 5 | 52 | EdgeSetGraphMtPer.rs | delete_edge | external_body | proven | invariant proof, edges only shrink |
| 6 | 52 | EdgeSetGraphMtPer.rs | default | external_body | external_body | std Default has no requires |

## Technique Details

### ClonePreservesView bound

Added `V: ClonePreservesView` to struct/trait/impl bounds, matching the StPer
version. This enables `clone_view()` with `ensures result@ == self@`, needed for
insert_edge (split chained `.insert(u).insert(v)`) and out_neighbors/delete_vertex
(clone elements from seq iteration for re-insertion).

### iterate+insert replaces filter

`out_neighbors` and `delete_vertex` previously used `AVLTreeSetMtPer::filter` with
closures. Verus can't verify `Clone` on closures (filter requires `F: Pred + Clone`).
Replaced with sequential iterate through `to_seq()` + conditional `insert()` into
a new set. Loop invariants track:
- Forward: everything in result set came from edges matching the predicate
- Backward: all matching elements processed so far are in the result set
- Set equality via `to_set() =~= edges@` from `to_seq` ensures

### lemma_eq_spec_iff_view_eq

Added proof fn bridging `PartialEq::eq` to view equality via the cmp/ord chain
(same pattern as EdgeSetGraphStPer.rs). Needed for `==` comparisons in
out_neighbors and delete_vertex loops.

### Conjunction flakiness fix (ETSPStEph)

Full validation exposed a Z3 conjunction flakiness in the ETSPStEph n==3 base case
(all sub-assertions pass but the conjunction fails). Fixed with explicit incremental
conjunction building per the R28 workaround pattern.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 52 | EdgeSetGraphMtPer.rs | Removed 5 external_body, added ClonePreservesView bound, added lemma and proof bodies |
| 2 | 26 | ETSPStEph.rs | Conjunction flakiness fix in n==3 base case |

## Verification

| Check | Result |
|-------|--------|
| `validate.sh isolate Chap52` | 2759 verified, 0 errors |
| `validate.sh isolate Chap26` | 1065 verified, 0 errors |
| `validate.sh` (full) | 5301 verified, 0 errors |
| `holes.sh EdgeSetGraphMtPer.rs` | 1 hole (default external_body) |

## Steps Used

5 of 20 (2 for ETSPStEph, 3 for EdgeSetGraphMtPer).
