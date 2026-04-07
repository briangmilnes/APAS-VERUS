# R156 Agent 1 Report — OrdKeyMap filter/map/collect/Clone

## Summary

Added 5 new operations to `OrdKeyMap` in `src/Chap38/OrdKeyMap.rs`:

| # | Chap | File | Operation | Lines | Status |
|---|------|------|-----------|-------|--------|
| 1 | 38 | OrdKeyMap.rs | `collect` | ~30 | Verified |
| 2 | 38 | OrdKeyMap.rs | `filter` | ~50 | Verified |
| 3 | 38 | OrdKeyMap.rs | `map_values` | ~90 | Verified |
| 4 | 38 | OrdKeyMap.rs | `reduce` | ~20 | Verified |
| 5 | 38 | OrdKeyMap.rs | `Clone` | ~6 | Verified |

## Implementation Details

- **collect**: Delegates to `self.inner.in_order()`, clones each element into a `Vec<Pair<K,V>>`.
- **filter**: Uses `ParamBST::filter` with an adapter closure that bridges `Fn(&K, &V) -> bool` to `Fn(&Pair<K,V>) -> bool`. Ghost `spec_pred` companion for spec reasoning. Proof pattern copied from OrderedTableStEph.
- **map_values**: Iterates via `in_order()`, applies `f` to each `(k, v)` pair, inserts `Pair(k_clone, new_val)` into fresh `ParamBST`. Key freshness proof via pairwise-distinct sorted keys. Pattern from OrderedTableStEph::map.
- **reduce**: Iterates via `in_order()`, folds with `f(&acc, &pair.1)`. Minimal ensures (`self@.dom().finite()`).
- **Clone**: Delegates to `self.inner.clone()` which already ensures `cloned@ == self@`.

## Verification

- Isolate: 1228 verified, 0 errors
- Full: 5766 verified, 0 errors
- RTTs: 3727 passed, 0 skipped

## Techniques

- Adapter closure pattern for filter (pair predicate wrapping key-value predicate)
- `lemma_sorted_keys_pairwise_distinct` for key freshness in map_values loop
- `lemma_view_gen_subset`/`lemma_view_gen_insert` for maintaining `spec_set_pair_view_generated`
- `lemma_key_unique_subset`/`lemma_key_unique_insert` for key uniqueness preservation
