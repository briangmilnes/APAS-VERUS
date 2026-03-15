# Agent 1 Round 15 Report

## Summary

- **Holes before**: 149
- **Holes after**: 143
- **Delta**: -6
- **Verified**: 4090
- **Errors**: 0
- **Clean modules**: 218 / 257

## Per-File Changes

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 42 | TableMtEph.rs | 11 | 6 | -5 | Ported sequential proofs from TableStEph; wrote `lemma_entries_to_map_subseq_value` for value preservation without `spec_keys_no_dups` |
| 2 | 41 | AVLTreeSetMtPer.rs | 11 | 10 | -1 | Replaced `assume(obeys_feq_full::<T>())` with broadcast trigger `obeys_feq_full_trigger::<T>()` |

## Functions Proved (TableMtEph.rs)

| # | Function | Holes Removed | Notes |
|---|----------|---------------|-------|
| 1 | `domain` | 1 external_body | Sequential loop building ArraySetStEph |
| 2 | `delete` | 1 external_body | Filter by key != target, PartialEq comparison |
| 3 | `difference` | 1 external_body | Nested scan, check other's entries for matching keys |
| 4 | `restrict` | 1 external_body | Filter by `keys.find(&pair.0)` |
| 5 | `subtract` | 1 external_body | Filter by `!keys.find(&pair.0)` |

## Key Technical Contributions

### `lemma_entries_to_map_subseq_value` (new proof fn)

The core challenge was that the MtEph trait lacks `spec_tablemteph_wf()` (i.e., `spec_keys_no_dups`) in the requires for delete/difference/restrict/subtract, unlike StEph. This means value preservation can't use `lemma_entries_to_map_get` (which needs no_dups).

Wrote a general inductive lemma proving: if `filtered` is a strictly-increasing-sources subsequence of `entries` that includes ALL entries with key `k`, then `spec_entries_to_map(filtered)[k] == spec_entries_to_map(entries)[k]`. Proof by induction on `entries.len()` with three cases: last entry has key k (both maps agree on last.1), last entry kept but different key (recurse on prefix/filtered.drop_last), last not kept (recurse on prefix/same filtered).

## Remaining Holes

### TableMtEph.rs (6 external_body)

All 6 remaining functions take closure parameters (tabulate, map, filter, intersection, union, insert). The trait signatures lack `forall|...| f.requires((...))` in their requires clauses. Without this, calling the closure in verified code is impossible. Adding f.requires would cascade to Chap43 callers (Agent 2 territory). These are structurally blocked until the trait signatures are updated.

### AVLTreeSetMtPer.rs (10 holes: 2 assume + 8 external_body)

- `assume(r == self@.len())` in `size`: Needs `seq.len() == seq.to_set().len()` which requires distinct elements proof from AVL sortedness.
- `assume(!self@.contains(x@))` in `find`: Binary search not-found postcondition. Needs sorted loop invariant + connection between exec `<` and spec ordering.
- 8 external_body: from_seq, filter, intersection, difference, union, delete, insert, Ord::cmp — parallel implementations at thread boundaries.

## Commit

To be committed on `agent1/ready`.
