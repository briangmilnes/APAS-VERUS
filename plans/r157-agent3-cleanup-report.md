# R157 Agent 3 — StEph + StPer Dead Code Cleanup Report

## Summary

Deleted 293 lines of unreachable bridge lemmas and spec fns from
`src/Chap43/OrderedTableStEph.rs` and `src/Chap43/OrderedTableStPer.rs`.

No `#[cfg(never)]` blocks existed in either file.

## Deleted Items

### StEph (-279 lines)

| # | Item | Kind | Reason |
|---|------|------|--------|
| 1 | `spec_ord_agrees_total_order` | spec fn | Zero call sites in file |
| 2 | `lemma_view_gen_subset` | proof fn | Zero call sites in file |
| 3 | `lemma_view_gen_union` | proof fn | Zero call sites in file |
| 4 | `lemma_cmp_equal_congruent` | proof fn | Zero call sites in file |
| 5 | `lemma_key_unique_remove` | proof fn | Zero call sites in file |
| 6 | `lemma_key_unique_disjoint_union` | proof fn | Zero call sites in file |
| 7 | `lemma_set_to_map_remove_pair` | proof fn | Zero call sites in file |
| 8 | `lemma_set_to_map_union_root` | proof fn | Zero call sites in file |
| 9 | `lemma_cmp_antisymmetry` | proof fn | Zero call sites in file |

### StPer (-14 lines)

| # | Item | Kind | Reason |
|---|------|------|--------|
| 1 | `lemma_view_gen_subset` | proof fn | Zero call sites in file |

## Retained Items (confirmed active callers)

StEph: `lemma_view_gen_insert`, `lemma_pair_set_to_map_dom_finite`,
`lemma_pair_set_to_map_len`, `lemma_key_unique_insert`,
`lemma_sorted_keys_pairwise_distinct`, `lemma_key_unique_subset`,
`lemma_key_unique_empty`, `lemma_set_to_map_insert`, `lemma_set_to_map_empty`.

StPer: All remaining lemmas have confirmed call sites.

## Line Count Change

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 3017 | 2738 | -279 |
| 2 | 43 | OrderedTableStPer.rs | 2295 | 2281 | -14 |
| — | — | Total | 5312 | 5019 | -293 |

## Validation

- `scripts/validate.sh isolate Chap43`: 2811 verified, 0 errors
- `scripts/rtt.sh`: 3752 passed, 0 skipped

## Commit

`5ac4724c7` on `agent3/ready`
