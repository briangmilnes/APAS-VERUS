<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 57 Report

**Task**: Close 12 capacity `assume` statements in `src/Chap43/OrderedTableStPer.rs`.

## Summary

All 12 capacity assumes removed. `OrderedTableStPer.rs` is now clean (0 algorithmic holes).
Final verification: **4485 verified, 0 errors**.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 43 | OrderedTableStPer.rs | 12 | 0 | -12 |
| 2 | 43 | AugOrderedTableStPer.rs | 0 | 0 | 0 |
| 3 | 43 | OrderedTableMtPer.rs | 0 | 0 | 0 |

## Changes by Function

| # | Chap | File | Function | Change |
|---|:----:|---|---|---|
| 1 | 43 | OrderedTableStPer.rs | `map` | `assert` replaces `assume` |
| 2 | 43 | OrderedTableStPer.rs | `filter` | loop inv + `assert` |
| 3 | 43 | OrderedTableStPer.rs | `intersection` | loop inv + `assert` |
| 4 | 43 | OrderedTableStPer.rs | `difference` | loop inv + `assert` |
| 5 | 43 | OrderedTableStPer.rs | `restrict` | loop inv + `assert` |
| 6 | 43 | OrderedTableStPer.rs | `subtract` | loop inv + `assert` |
| 7 | 43 | OrderedTableStPer.rs | `split_key` | loop inv + `assert` |
| 8 | 43 | OrderedTableStPer.rs | `get_key_range` | loop inv + `assert` |
| 9 | 43 | OrderedTableStPer.rs | `split_rank_key` | loop inv + 2 asserts |
| 10 | 43 | OrderedTableStPer.rs | `tabulate` | requires + proof hint + `assert` |
| 11 | 43 | OrderedTableStPer.rs | `union` | requires + proof hint + loop inv + `assert` |
| 12 | 43 | AugOrderedTableStPer.rs | `tabulate` | propagated requires from #10 |
| 13 | 43 | AugOrderedTableStPer.rs | `union` | propagated requires from #11 |
| 14 | 43 | AugOrderedTableStPer.rs | `join_key` | propagated requires from StPer |
| 15 | 43 | OrderedTableStPer.rs | `join_key` | added `dom().len()` sum requires |
| 16 | 43 | OrderedTableMtPer.rs | `join_key` | propagated requires from StPer |

## Technique Summary

**Simple cases (map, filter, intersection, difference, restrict, subtract,
split\_key, get\_key\_range, split\_rank\_key)**: Added `result_vec@.len() <= i as nat`
to the loop invariant. After the loop this gives `result_vec@.len() <= n` where
`n == self.base_set.elements@.len()`. The broadcast group
`group_avltreeseqstper_len_bound` provides `n < usize::MAX`, so the assert discharges.

**`tabulate`**: The `result_vec` is keyed by `key_seq` from `ArraySetStEph::to_seq()`,
not an AVL tree — so `group_avltreeseqstper_len_bound` does not apply directly.
Added `keys@.len() < usize::MAX` to the `OrderedTableStPerTrait::tabulate` requires.
Used `key_seq@.unique_seq_to_set()` (from vstd) to prove
`key_seq.spec_len() == keys@.len()`, giving the loop invariant and enabling the assert.

**`union`**: The result grows from two sources, so `result_vec@.len() <=
self_len + other_len`. Added `self@.dom().len() + other@.dom().len() < usize::MAX` to
the `union` trait requires. Used `lemma_entries_to_map_len` from `TableStPer` to
bridge `dom().len()` to `base_set.elements@.len()` in the implementation.

**Cascading requires**: Every new `requires` on a StPer method needed to be
propagated to `AugOrderedTableStPer`, `OrderedTableMtPer`, and `OrderedTableStPer::join_key`
call sites to satisfy Verus's precondition checker.

## Remaining Holes in Chap43

| # | Chap | File | Hole Type | Count | Description |
|---|:----:|---|---|:---:|---|
| 1 | 43 | AugOrderedTableMtEph.rs | `external_body` | 1 | `lemma_mt_reducer_clone_total` |
| 2 | 43 | AugOrderedTableStPer.rs | `external_body` | 1 | `lemma_reducer_clone_total` |
| 3 | 43 | OrderedSetStEph.rs | `assume()` | 1 | `select` filter cardinality |
| 4 | 43 | OrderedSetStPer.rs | `assume()` | 1 | `select` filter cardinality |

The two `external_body` proof lemmas are for clone-total properties of generic
reducer functions; these require closure spec extensions not yet in vstd.
The two `select` assumes require sortedness facts not yet captured in the wf spec.

## Verification Count

- Before: 4485 verified, 1 error (pre-existing Chap47 issue)
- After: 4485 verified, 0 errors
