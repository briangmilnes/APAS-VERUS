# R158 Agent 2 — Final StPer Delegation to OrdKeyMap

## Summary

Delegated 8 OrderedTableStPer methods to OrdKeyMap, eliminating 617 lines of
manual BST-level proof code. All methods preserve their existing trait ensures.

## Methods Delegated

| # | Chap | File | Method | Lines Before | Lines After | Technique |
|---|------|------|--------|--------------|-------------|-----------|
| 1 | 43 | OrderedTableStPer.rs | `empty` | 11 | 7 | OrdKeyMap::new() |
| 2 | 43 | OrderedTableStPer.rs | `singleton` | 17 | 10 | OrdKeyMap::new() + insert |
| 3 | 43 | OrderedTableStPer.rs | `insert` | 66 | 6 | clone inner + OrdKeyMap::insert |
| 4 | 43 | OrderedTableStPer.rs | `delete` | 30 | 6 | clone inner + OrdKeyMap::delete |
| 5 | 43 | OrderedTableStPer.rs | `domain` | 55 | 3 | OrdKeyMap::domain() |
| 6 | 43 | OrderedTableStPer.rs | `tabulate` | 140 | 7 | OrdKeyMap::tabulate() |
| 7 | 43 | OrderedTableStPer.rs | `filter` | 51 | 5 | OrdKeyMap::filter() |
| 8 | 43 | OrderedTableStPer.rs | `collect` | 30 | 5 | OrdKeyMap::collect() + from_vec |

## Dead Code Removed

| # | Chap | File | Function | Lines |
|---|------|------|----------|-------|
| 1 | 43 | OrderedTableStPer.rs | `bst_find_by_key` | 143 |
| 2 | 43 | OrderedTableStPer.rs | `lemma_cmp_equal_congruent` | 13 |
| 3 | 43 | OrderedTableStPer.rs | `lemma_key_unique_remove` | 7 |
| 4 | 43 | OrderedTableStPer.rs | `lemma_key_unique_subset` | 10 |
| 5 | 43 | OrderedTableStPer.rs | `lemma_set_to_map_insert` | 48 |
| 6 | 43 | OrderedTableStPer.rs | `lemma_set_to_map_remove_pair` | 37 |
| 7 | 43 | OrderedTableStPer.rs | `lemma_set_to_map_empty` | 5 |
| 8 | 43 | OrderedTableStPer.rs | `spec_rank_pred` | 4 |

Removed imports: `ArraySeqStEphTrait`, `Greater`, `Less`.

## Methods NOT Delegated (and why)

| # | Method | Reason |
|---|--------|--------|
| 1 | `map` | OrdKeyMap::map_values has weaker ensures (no value tracking) |
| 2 | `restrict` | OrdKeyMap::restrict requires `keys.spec_arraysetsteph_wf()` not in StPer trait |
| 3 | `subtract` | Same as restrict |
| 4 | `from_sorted_entries` | No OrdKeyMap equivalent |

## File Size

- Before: 2281 lines
- After: 1664 lines
- Delta: **-617 lines** (29 inserted, 646 deleted)

## Validation

- Isolate Chap43: 2811 verified, 0 errors, 0 trigger warnings
- Full validate: 5755 verified, 0 errors
- RTT: 3776 passed, 0 skipped
- No PTTs run (per instructions)
