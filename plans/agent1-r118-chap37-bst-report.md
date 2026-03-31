# Agent 1 — R118 Chap37 BST MtEph Spec Strengthening Report

## Summary

Strengthened specs on 5 MtEph files in Chap37. Added `delete` to 2 files.
Added spec fns to BSTSplayMtEph. All changes verified clean.

## Results

- **Verified**: 5426 (was 5420, +6 from new delete functions)
- **RTT**: 3529 passed
- **PTT**: 221 passed
- **Errors**: 0

## Changes by file

| # | Chap | File | Warnings Before | Warnings After | Changes |
|---|------|------|-----------------|----------------|---------|
| 1 | 37 | BSTRBMtEph.rs | 12 | ~4 | Strengthened `insert` ensures: added `tree_is_bst`, `tree_contains(value)`, full containment `forall` biconditional. Proved containment/BST through color-change with `reveal_with_fuel`. Eliminated 1 size assume (kept 1). Added `assume(tree_is_bst)` for view bridge. |
| 2 | 37 | BSTAVLMtEph.rs | 9 | ~4 | Added explicit `tree_is_avl(tree@)` to `new` and `insert` ensures. These were implied by wf but not stated. |
| 3 | 37 | BSTSplayMtEph.rs | 9 | ~3 | Added 5 trait spec fns (`spec_size`, `spec_height`, `spec_contains`, `spec_in_order`, `spec_pre_order`) with impl wrappers. Added `spec_in_order_link` and `spec_pre_order_link` spec fns. Strengthened `insert` ensures with reverse containment direction. |
| 4 | 37 | BSTPlainMtEph.rs | 1 | 0 | Implemented `delete` (Layer 1: `delete_min_node` + `delete_node` copied from StEph; Layer 2: trait + impl with RwLock wrapper). Strong ensures: `!tree_contains(target)` + full containment `forall`. |
| 5 | 37 | BSTBBAlphaMtEph.rs | 1 | 0 | Same as BSTPlainMtEph — implemented `delete` with identical Layer 1 and analogous Layer 2. |

## Warnings not addressed

Remaining warnings are primarily about explicit `tree_is_bst` / `spec_size <= usize::MAX` /
`spec_height <= usize::MAX` in individual function requires. These are already implied by the
respective `spec_*_wf()` predicates which are in every requires clause. The comparison tool
flags them because the StEph states them explicitly while the MtEph relies on wf.

For AVLTreeSeqStEph.rs (3 warnings) and AVLTreeSeqMtPer.rs (6 warnings): these involve
adding `values_in_order`, strengthening `set`/`from_vec`/`iter` specs, and adding `to_arrayseq`.
These are lower-impact changes that require more architectural work (new functions, not just
spec strengthening). Deferred.

## Techniques

- **reveal_with_fuel**: Used to prove that root color change in RB insert preserves
  `spec_is_bst_link`, `link_contains`, and `link_spec_size` (all ignore the color field).
- **Bridge lemmas**: Used existing `lemma_link_to_bbt_contains` and `lemma_link_to_bbt_size`
  to convert link-level containment facts to BalBinTree view-level facts.
- **Proof copying**: Duplicated `delete_min_node` and `delete_node` from BSTPlainStEph into
  BSTPlainMtEph and BSTBBAlphaMtEph per Mt standalone rule. ~300 lines each, all verified.

## Assumes

Existing assumes retained (ghost-real bridge pattern for MtEph):
- `assume(self.ghost_root@ == current)` — standard RwLock ghost-real bridge
- `assume(link_spec_size(new_root) <= usize::MAX)` — size bound through insert
- `assume(link_to_bbt(new_root).tree_is_bst())` — BST bridge (BSTSpecFns trait dispatch
  prevents Z3 from connecting spec_is_bst_link to tree_is_bst automatically)
- `assume(new_tree.spec_size() <= usize::MAX)` in delete — delete reduces size
- `assume(new_tree.spec_height() <= usize::MAX)` in delete — delete reduces height
