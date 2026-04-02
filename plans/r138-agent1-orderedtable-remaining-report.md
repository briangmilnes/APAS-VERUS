# R138 Agent 1 — Fix remaining OrderedTable in_order bugs

## Summary

Replaced O(n) in_order traversals with O(lg n) BST descent algorithms for 7 functions
in OrderedTableStEph.rs. Two functions (`next_key_iter`, `previous_key_iter`) fully verify.
Five functions have proof gaps in their helper functions that need bridge proof polishing.

## Infrastructure Added (5 BST helper functions)

All in `src/Chap43/OrderedTableStEph.rs`, placed between `bst_find_by_key` and the impl block:

| # | Chap | File | Helper | Status | Description |
|---|------|------|--------|--------|-------------|
| 1 | 43 | OrderedTableStEph.rs | `bst_next_by_key` | VERIFIED | O(lg n) BST successor by key descent |
| 2 | 43 | OrderedTableStEph.rs | `bst_prev_by_key` | VERIFIED | O(lg n) BST predecessor by key descent |
| 3 | 43 | OrderedTableStEph.rs | `bst_split_by_key` | 2 proof gaps | O(lg n) key-only BST split via expose+join_mid |
| 4 | 43 | OrderedTableStEph.rs | `bst_rank_by_key` | 3 proof gaps | O(lg n) rank via size-based BST descent |
| 5 | 43 | OrderedTableStEph.rs | `bst_select_by_rank` | 2 proof gaps | O(lg n) select via size-based BST descent |

## Functions Fixed

| # | Chap | File | Function | Old | New | Status |
|---|------|------|----------|-----|-----|--------|
| 1 | 43 | OrderedTableStEph.rs | `next_key_iter` | O(n) in_order scan | O(lg n) `bst_next_by_key` | VERIFIED |
| 2 | 43 | OrderedTableStEph.rs | `previous_key_iter` | O(n) in_order scan | O(lg n) `bst_prev_by_key` | VERIFIED |
| 3 | 43 | OrderedTableStEph.rs | `split_key_iter` | O(n lg n) in_order + inserts | O(lg n) `bst_split_by_key` | Proof gap in helper |
| 4 | 43 | OrderedTableStEph.rs | `get_key_range_iter` | O(n lg n) in_order + inserts | O(lg n) two `bst_split_by_key` | Proof gap in helper |
| 5 | 43 | OrderedTableStEph.rs | `rank_key_iter` | O(n) in_order count | O(lg n) `bst_rank_by_key` | Proof gap in helper |
| 6 | 43 | OrderedTableStEph.rs | `select_key` | O(n²) in_order + rank each | O(lg n) `bst_select_by_rank` | Proof gap in helper |
| 7 | 43 | OrderedTableStEph.rs | `split_rank_key_iter` | O(n lg n) in_order + inserts | O(n) in_order + split_by_key | Proof gap (uses in_order for select) |

## StPer Status

StPer changes were not made in this round. The StPer file needs the same 7 function
replacements, plus first_key_iter and last_key_iter (which were fixed in StEph by agent 4
in R137 but not in StPer).

## Validation

- Chap43 isolate: 2654 verified, 5 errors (was 2648 verified, 0 errors)
- New verifications: +6 (bst_next_by_key, bst_prev_by_key, next_key_iter, previous_key_iter, and supporting lemmas)
- New errors: 5 (all in new helper functions, not regressions)

## Proof Gap Details

### `bst_split_by_key` (2 errors)
- **Found-value postcondition**: When k is found by recursive descent into left/right subtree,
  the proof doesn't connect the recursive found value to the tree-level map value.
  Fix: add `lemma_pair_in_set_map_contains` chain from subtree found to tree-level map.
- **Size bound postcondition**: left.len() + right.len() <= tree.len() not established
  after join_mid reconstruction.

### `bst_rank_by_key` (3 errors)
- **Equal arm**: `tree_dom.filter(rank_pred) =~= left_dom` — the left_dom ⊆ filter
  direction needs the `reveal_param_bst_backings` witness chain but the backing Pair
  witnesses aren't flowing. Needs explicit quantifier instantiation.
- **Greater arm**: Same disjoint-union length proof issue.

### `bst_select_by_rank` (2 errors)
- Similar to rank — the `tree_dom.filter(rank_pred_sel) =~= left_dom.filter(...)` 
  equivalence proof in the i < left_size and i > left_size arms.

## Techniques Used

- **BST descent with expose()**: Recursive tree walk comparing keys, same pattern as
  `bst_find_by_key` from R135.
- **TotalOrderBridge**: Used `cmp_spec_less_implies_le` and `cmp_spec_greater_implies_le`
  to bridge BST cmp_spec ordering to TotalOrder::le postconditions.
- **reveal_param_bst_backings**: Called at exec level (not inside proof blocks — Verus
  doesn't allow exec calls in proof mode) to expose BST type invariant for quantifier
  instantiation.
- **None postcondition**: Added ensures for the None return case to enable callers to
  use the "no successor/predecessor exists" fact in their proofs.

## What's Needed to Complete

1. Fix 5 remaining proof gaps in `bst_split_by_key`, `bst_rank_by_key`, `bst_select_by_rank`
2. Apply the same changes to OrderedTableStPer.rs (mirror of StEph)
3. Fix first_key_iter and last_key_iter in StPer (still O(n), agent 4 only fixed StEph)
