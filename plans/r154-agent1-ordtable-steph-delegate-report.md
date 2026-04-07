# R154 Agent 1 Report: Delegate OrderedTableStEph Methods to OrdKeyMap

## Summary

Delegated 6 methods from OrderedTableStEph to OrdKeyMap, simplified the wf predicate,
and removed 4 dead free functions. Reduced OrderedTableStEph.rs from 5,466 to 3,967
lines (−1,499 lines, 27% reduction).

## Changes

### wf simplification

`spec_orderedtablesteph_wf` reduced from 12 conjuncts to:
```rust
open spec fn spec_orderedtablesteph_wf(&self) -> bool {
    self.tree.spec_ordkeymap_wf()
}
```

### Methods delegated

| # | Chap | File | Method | OrdKeyMap method | Lines before | Lines after | Bridge proof |
|---|------|------|--------|-----------------|--------------|-------------|-------------|
| 1 | 43 | OrderedTableStEph.rs | previous_key_iter | prev_key | ~8 (call to bst_prev_by_key) | 1 | none |
| 2 | 43 | OrderedTableStEph.rs | next_key_iter | next_key | ~8 (call to bst_next_by_key) | 1 | none |
| 3 | 43 | OrderedTableStEph.rs | rank_key_iter | rank_key | ~5 (call to bst_rank_by_key) | 1 | none |
| 4 | 43 | OrderedTableStEph.rs | select_key | select_key | ~5 (call to bst_select_by_rank) | 1 | none |
| 5 | 43 | OrderedTableStEph.rs | difference | difference | ~130 | 3 | 1 line (lemma_pair_set_to_map_dom_finite) |

### Methods NOT delegated (incompatible signatures)

| # | Chap | File | Method | Reason |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableStEph.rs | intersection | Takes `F: Fn(&V,&V)->V` combine; OrdKeyMap::intersect keeps self values only |
| 2 | 43 | OrderedTableStEph.rs | union | Takes `F: Fn(&V,&V)->V` combine; OrdKeyMap::union is right-biased |
| 3 | 43 | OrderedTableStEph.rs | split_key_iter | OrdKeyMap::split ensures lack Map-level disjointness |
| 4 | 43 | OrderedTableStEph.rs | get_key_range_iter | Uses bst_split_by_key + bst_find_by_key at BST level |
| 5 | 43 | OrderedTableStEph.rs | split_rank_key_iter | Uses bst_split_by_key + bst_find_by_key at BST level |

### Dead code removed

| # | Function | Lines removed | Reason |
|---|----------|--------------|--------|
| 1 | bst_next_by_key | 356 | Replaced by OrdKeyMap::next_key |
| 2 | bst_prev_by_key | 336 | Replaced by OrdKeyMap::prev_key |
| 3 | bst_rank_by_key | 314 | Replaced by OrdKeyMap::rank_key |
| 4 | bst_select_by_rank | 353 | Replaced by OrdKeyMap::select_key |
| | **Total** | **1,359** | |

Two BYPASSED comments mark the deletion points.

### Dead spec fns retained (not used but harmless)

- `spec_rank_pred` (3 lines) — was only used in deleted functions
- `spec_ord_agrees_total_order` (4 lines) — was only used in deleted functions

## Verification

- Full validation: 5,752 verified, 0 errors
- RTTs: 3,717 passed, 0 skipped
- Isolate Chap43: 2,808 verified, 0 errors (down from 2,813 — 4 fewer obligations from deleted functions)

## Potential future work

1. Add `dom().finite()` to OrdKeyMap::difference ensures to eliminate the bridge proof
2. Add Map-level disjointness to OrdKeyMap::split ensures to enable split_key delegation
3. Add `obeys_view_eq` to OrdKeyMap wf (or derive it) to enable bst_find_by_key replacement
4. Add OrdKeyMap methods for intersection/union with combine functions
