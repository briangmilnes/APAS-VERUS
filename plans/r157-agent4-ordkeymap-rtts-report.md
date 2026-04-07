# R157 Agent 4 Report — OrdKeyMap RTTs for R156 Additions

## Summary

Added 24 runtime tests to `tests/Chap41/TestOrdKeyMap.rs` covering the five
operations added by R156 Agent 1: `collect`, `filter`, `map_values`, `reduce`, and
`Clone`. All 3776 RTTs pass (up from 3752).

## Scope Note

R157 Agent 1 additions (`domain`, `tabulate`, `restrict`, `subtract`) were not
present in `src/Chap41/OrdKeyMap.rs` at the time this agent ran. Tests for those
operations were not written.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | TestOrdKeyMap.rs | +24 tests, +`use vstd::prelude::Ghost` import |

## New Tests Added

| # | Test Name | Covers |
|---|-----------|--------|
| 1 | test_ordkeymap_collect_empty | collect on empty map |
| 2 | test_ordkeymap_collect_singleton | collect single entry |
| 3 | test_ordkeymap_collect_sorted_order | collect returns ascending key order |
| 4 | test_ordkeymap_collect_values_correct | collect Pair values correct |
| 5 | test_ordkeymap_filter_keep_some | filter retains matching subset |
| 6 | test_ordkeymap_filter_keep_none | filter with always-false predicate |
| 7 | test_ordkeymap_filter_keep_all | filter with always-true predicate |
| 8 | test_ordkeymap_filter_by_value | filter on value threshold |
| 9 | test_ordkeymap_filter_empty_map | filter on empty map |
| 10 | test_ordkeymap_map_values_double | map_values doubles values |
| 11 | test_ordkeymap_map_values_keys_unchanged | map_values preserves key set |
| 12 | test_ordkeymap_map_values_key_dependent | map_values uses key in transform |
| 13 | test_ordkeymap_map_values_empty | map_values on empty map |
| 14 | test_ordkeymap_reduce_sum | reduce sums all values |
| 15 | test_ordkeymap_reduce_empty | reduce on empty map returns identity |
| 16 | test_ordkeymap_reduce_max | reduce computes max |
| 17 | test_ordkeymap_reduce_singleton | reduce on single entry |
| 18 | test_ordkeymap_clone_equals_original | clone matches original |
| 19 | test_ordkeymap_clone_modify_clone_no_effect_on_original | clone is independent |
| 20 | test_ordkeymap_clone_empty | clone of empty map |
| 21 | test_ordkeymap_collect_after_filter | collect on filtered result |
| 22 | test_ordkeymap_clone_filter_map_values_independent | clone + filter + map_values chain |
| 23 | test_ordkeymap_reduce_after_map_values | map_values then reduce |
| 24 | test_ordkeymap_collect_roundtrip | collect entries rebuild identical map |

## RTT Results

```
Summary [16.340s] 3776 tests run: 3776 passed, 0 skipped
```

## Implementation Notes

- `filter` takes `Ghost::assume_new()` as the spec_pred argument in RTT context (ghost
  types are erased at runtime; `Ghost::assume_new()` is the standard RTT idiom).
- `map_values` and `reduce` take plain closures; no ghost parameters needed.
- `Clone` is a standard Rust trait impl — `m.clone()` works directly.
