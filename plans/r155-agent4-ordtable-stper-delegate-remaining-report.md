# R155 Agent 4 — OrderedTableStPer Delegation Report

## Summary

Delegated all delegable methods in `src/Chap43/OrderedTableStPer.rs` through
`OrdKeyMap` instead of bypassing to `self.tree.inner` (the raw `ParamBST`).
Deleted three dead functions. Net: **-884 lines** (4326 → 3442).

## Changes Made

| # | Chap | File | Change | Lines Removed |
|---|------|------|--------|---------------|
| 1 | 43 | OrderedTableStPer.rs | `spec_orderedtablestper_find_pre` → `self.tree.spec_ordkeymap_wf()` | 8 |
| 2 | 43 | OrderedTableStPer.rs | `find` → `self.tree.find(k)` | 0 |
| 3 | 43 | OrderedTableStPer.rs | `previous_key_iter` → `self.tree.prev_key(k)` | 0 |
| 4 | 43 | OrderedTableStPer.rs | `next_key_iter` → `self.tree.next_key(k)` | 0 |
| 5 | 43 | OrderedTableStPer.rs | Deleted `lemma_cmp_antisymmetry` (dead, no callers) | 11 |
| 6 | 43 | OrderedTableStPer.rs | Deleted `bst_next_by_key` (now dead after #4) | ~355 |
| 7 | 43 | OrderedTableStPer.rs | Deleted `bst_prev_by_key` (now dead after #3) | ~335 |
| 8 | 43 | OrderedTableStPer.rs | `rank_key_iter` → `self.tree.rank_key(k)` | ~120 |
| 9 | 43 | OrderedTableStPer.rs | `select_key` → `self.tree.select_key(i)` | ~55 |

**Git diff**: 11 insertions, 895 deletions.

## Cannot Delegate (with reasons)

| # | Chap | File | Function | Why Not |
|---|------|------|----------|---------|
| 1 | 43 | OrderedTableStPer.rs | `insert` | Persistent op clones `self.tree.inner`; OrdKeyMap has no Clone |
| 2 | 43 | OrderedTableStPer.rs | `delete` / `delete_value_preserved` | Same reason as insert |
| 3 | 43 | OrderedTableStPer.rs | `domain`, `tabulate`, `map`, `filter`, `collect` | All use `in_order()`, not in OrdKeyMap trait |
| 4 | 43 | OrderedTableStPer.rs | `split_key_iter` | Uses `in_order()` to build two trees |
| 5 | 43 | OrderedTableStPer.rs | `get_key_range_iter` | Uses `in_order()` |
| 6 | 43 | OrderedTableStPer.rs | `split_rank_key_iter` | Uses `in_order()` |
| 7 | 43 | OrderedTableStPer.rs | `restrict`, `subtract` | Use `in_order()` |
| 8 | 43 | OrderedTableStPer.rs | `first_key_iter`, `last_key_iter` | Use `min_key()`, `max_key()`, not in OrdKeyMap trait |
| 9 | 43 | OrderedTableStPer.rs | `iter` | Uses `in_order()` directly |
| 10 | 43 | OrderedTableStPer.rs | `singleton` | Uses `ParamBST::singleton()`, no OrdKeyMap equivalent |
| 11 | 43 | OrderedTableStPer.rs | `empty` | `OrdKeyMap::new()` doesn't ensure `spec_ordkeymap_wf()`, proof burden unchanged |

## Verification Results

- **Isolate (Chap43)**: 2810 verified, 0 errors (run twice: after initial delegations, after rank/select)
- **Full validate**: 5754 verified, 0 errors
- **RTT**: 3717 passed, 0 skipped

## Notes

- `bst_find_by_key` is still live — used by `insert` and `delete` for the existing-key lookup.
- `self.tree.inner` has 109 remaining references, all in functions that cannot be delegated.
- All alg-analysis comments updated to reflect O(lg n) where delegation to OrdKeyMap achieves
  it (rank_key and select_key dropped from O(n) loop to O(lg n) BST descent).
