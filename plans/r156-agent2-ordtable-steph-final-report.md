# R156 Agent 2 Report — OrderedTableStEph Final OrdKeyMap Delegation

## Summary

Delegated 5 OrderedTableStEph methods to OrdKeyMap, eliminating ~960 lines of
duplicate BST traversal code. Two methods (union, intersection) could not be
delegated due to a Verus closure-ensures identity gap.

## Line Count

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| OrderedTableStEph.rs lines | 3,977 | 3,017 | -960 |
| Verified functions (Chap43 isolate) | 2,814 | 2,812 | -2 |
| Verified functions (full) | 5,758 | 5,756 | -2 |
| RTT tests | 3,727 | 3,727 | 0 |

## Delegations Completed

| # | Chap | File | StEph Method | OrdKeyMap Method | Lines Saved |
|---|------|------|-------------|-----------------|-------------|
| 1 | 43 | OrderedTableStEph.rs | first_key_iter | first_key | ~55 |
| 2 | 43 | OrderedTableStEph.rs | last_key_iter | last_key | ~55 |
| 3 | 43 | OrderedTableStEph.rs | split_key_iter | split | ~75 |
| 4 | 43 | OrderedTableStEph.rs | get_key_range_iter | get_key_range | ~90 |
| 5 | 43 | OrderedTableStEph.rs | split_rank_key_iter | split_rank_key | ~125 |

## Delegations NOT Completed (Verus Limitation)

| # | Chap | File | StEph Method | OrdKeyMap Method | Reason |
|---|------|------|-------------|-----------------|--------|
| 1 | 43 | OrderedTableStEph.rs | union | union_with | Closure-ensures identity gap |
| 2 | 43 | OrderedTableStEph.rs | intersection | intersect_with | Closure-ensures identity gap |

**Verus closure-ensures identity gap**: The StEph trait declares `fn union(&mut self, ..., f: F)`
and the postcondition uses `f.ensures(...)` (i.e., `old(f).ensures(...)`). OrdKeyMap::union_with
takes `combine: &F` and its postcondition uses `combine.ensures(...)`. Verus treats
`f.ensures(...)` and `(&f).ensures(...)` as syntactically different in the SMT encoding,
even though they are semantically identical for `Fn` closures. Multiple approaches were
attempted (wrapper closures, helper functions, intermediate variables) but none resolved
the gap. The original iterative implementations are retained.

## Dead Code Deleted

| # | Chap | File | Function | Lines | Reason |
|---|------|------|----------|-------|--------|
| 1 | 43 | OrderedTableStEph.rs | bst_find_by_key | ~145 | Moved to OrdKeyMap::ordkeymap_find |
| 2 | 43 | OrderedTableStEph.rs | bst_split_by_key | ~450 | Moved to OrdKeyMap::ordkeymap_split |

## Proof Notes

- `first_key_iter` and `last_key_iter`: Direct delegation, no proof glue needed.
  OrdKeyMap::first_key/last_key specs match the StEph trait specs exactly.
- `split_key_iter`: Required one `lemma_pair_set_to_map_dom_finite` call to prove
  `old(self)@.dom().finite()` before `*self = Self::empty()`.
- `get_key_range_iter`: Direct delegation, OrdKeyMap::get_key_range specs match.
- `split_rank_key_iter`: Direct delegation to OrdKeyMap::split_rank_key.

## Verification

- `scripts/validate.sh`: 5756 verified, 0 errors
- `scripts/rtt.sh`: 3727 tests, 3727 passed
