# Agent 1 — Round 28 Report: Chap37 fn_missing_requires Sweep

## Summary

Fixed 31 of 53 `fn_missing_requires` warnings in Chap37.
Verification: **4114 verified, 0 errors** (unchanged from baseline).

## Per-file results

| # | Chap | File | Before | After | Fixed |
|---|------|------|--------|-------|-------|
| 1 | 37 | BSTRBMtEph.rs | 11 | 0 | 11 |
| 2 | 37 | BSTSplayMtEph.rs | 10 | 0 | 10 |
| 3 | 37 | BSTSplayStEph.rs | 5 | 5 | 0 |
| 4 | 37 | BSTSetAVLMtEph.rs | 3 | 3 | 0 |
| 5 | 37 | BSTSetBBAlphaMtEph.rs | 3 | 3 | 0 |
| 6 | 37 | BSTSetPlainMtEph.rs | 3 | 3 | 0 |
| 7 | 37 | BSTSetRBMtEph.rs | 2 | 2 | 0 |
| 8 | 37 | BSTSetSplayMtEph.rs | 3 | 3 | 0 |
| 9 | 37 | AVLTreeSeq.rs | 1 | 0 | 1 |
| 10 | 37 | AVLTreeSeqStEph.rs | 3 | 1 | 2 |
| 11 | 37 | AVLTreeSeqStPer.rs | 5 | 1 | 4 |
| 12 | 37 | AVLTreeSeqMtPer.rs | 4 | 1 | 3 |
| | | **Total** | **53** | **22** | **31** |

## Techniques used

**Group B — real requires added (19 functions):**
Functions with meaningful `ensures` received real `requires` clauses derived from their
body's preconditions:

- `link_spec_size(*link) <= usize::MAX as nat` — for BST MtEph helpers operating on links
  (new_node, is_red, size_link, update in BSTRBMtEph/BSTSplayMtEph)
- `values@.len() <= usize::MAX as nat` — for build_balanced taking slices
- `spec_cached_height(n) <= usize::MAX as nat` — for AVL height accessor functions
- `spec_cached_size(n) <= usize::MAX as nat` — for AVL size accessor functions
- `1 + link_spec_size(old(node).left) + link_spec_size(old(node).right) <= usize::MAX`
  — for `update` which recomputes cached size from children
- `spec_avltreeseqstper_wf(*cur)` — for dead-code inorder_collect in StPer
- `obeys_feq_full::<T>()` — for MtPer `rec` inside external_body

**Group A — requires + ensures true (12 functions):**
MtEph collect/parallel functions (in_order_collect, pre_order_collect, in_order_parallel,
pre_order_parallel, filter_parallel, reduce_parallel) received
`requires link_spec_size(*link) <= usize::MAX as nat, ensures true`.

**&mut old() fix:** All `update` functions needed `old(node).left` instead of `node.left`
in `requires` per Verus `&mut` semantics.

## Remaining 22 warnings — structural blockers

| # | Chap | File | Count | Blocker |
|---|------|------|-------|---------|
| 1 | 37 | BSTSplayStEph.rs | 5 | Any requires on helpers destabilizes splay SMT proof |
| 2 | 37 | BSTSet*MtEph.rs (5 files) | 14 | Wrapper fns called from ParaPair! closures; no natural requires on from_sorted_iter |
| 3 | 37 | AVLTreeSeqStEph.rs | 1 | push_left_iter called from Iterator::next (can't have requires) |
| 4 | 37 | AVLTreeSeqStPer.rs | 1 | push_left_iter_stper called from Iterator::next |
| 5 | 37 | AVLTreeSeqMtPer.rs | 1 | inorder_collect caller values_in_order has no requires; cascade needed |

**BSTSplayStEph detail:** Adding even trivially-true requires like `0nat <= usize::MAX`
to `new_node`, `size_link`, or `update` (called ~20 times inside `splay`) destabilizes
the SMT solver's proof of `spec_is_bst_link` postconditions in `splay`. Reverted all
changes after 3 attempts with different requires strengths.

**BSTSet*MtEph detail:** `values_vec`, `rebuild_from_vec`, `from_sorted_iter` are free
functions used across set algebra operations. `values_vec` is called inside `move ||`
closures in `ParaPair!`, making requires propagation complex. `from_sorted_iter` takes a
generic `IntoIterator` with no meaningful precondition.

**Iterator helper detail:** `push_left_iter` / `push_left_iter_stper` are called from
`Iterator::next()` which in Verus cannot have `requires` (trait method signature is fixed).

## Files modified (12)

All in `src/Chap37/`:
BSTRBMtEph.rs, BSTSplayMtEph.rs, BSTSplayStEph.rs (reverted — net zero change),
BSTSetAVLMtEph.rs, BSTSetBBAlphaMtEph.rs, BSTSetPlainMtEph.rs, BSTSetRBMtEph.rs,
BSTSetSplayMtEph.rs, AVLTreeSeq.rs, AVLTreeSeqStEph.rs, AVLTreeSeqStPer.rs,
AVLTreeSeqMtPer.rs
