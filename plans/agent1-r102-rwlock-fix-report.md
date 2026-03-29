# Agent 1 R102 Report: OrderedTableMtPer RwLock Predicate Fix

## Summary

Fixed the OrderedTableMtPerInv RwLock predicate to carry `expected_view` ghost field,
establishing the link between the locked inner table and the ghost abstract view.
This unlocked removal of 6 external_body annotations and 10 assume statements.

## What Changed

**Root cause:** `OrderedTableMtPerInv` was a unit struct. Its `inv` only checked
`v.spec_orderedtablestper_wf()` but never linked `v@` to the ghost view. Every
function that needed `inner@ == self@` had to assume it.

**Fix:** Added `pub ghost expected_view: Map<K::V, V::V>` to the predicate struct.
The `inv` now checks `v@ == self.expected_view`. The type_invariant links
`locked_table.pred().expected_view == ghost_locked_table@`. At construction
(`from_st_table`), the expected_view is set to `inner@`.

**Proof chain:** After `acquire_read()`:
- Predicate inv gives `inner@ == pred.expected_view`
- Type invariant gives `pred.expected_view == ghost_locked_table@`
- View definition gives `ghost_locked_table@ == self@`
- Therefore `inner@ == self@` (assertion, not assumption)

## Holes Before/After — Chap43 OrderedTableMtPer.rs

| # | Chap | Function | Before | After |
|---|------|----------|--------|-------|
| 1 | 43 | from_st_table | assume(wf) | proved (lemma_pair_set_to_map_dom_finite) |
| 2 | 43 | size | assume(count==dom.len) | proved (predicate chain) |
| 3 | 43 | find | external_body | proved (predicate chain) |
| 4 | 43 | insert | external_body | proved (predicate chain + from_st_table) |
| 5 | 43 | insert_wf | external_body | proved (predicate chain + from_st_table) |
| 6 | 43 | delete | external_body | proved (predicate chain + from_st_table) |
| 7 | 43 | delete_wf | external_body | proved (predicate chain + from_st_table) |
| 8 | 43 | map | external_body | 1 RWLOCK_GHOST assume (domain eq, structural) |
| 9 | 43 | first_key | assume(inner@==self@) | proved (predicate chain) |
| 10 | 43 | last_key | assume(inner@==self@) | proved (predicate chain) |
| 11 | 43 | previous_key | assume(inner@==self@) | proved (predicate chain) |
| 12 | 43 | next_key | assume(inner@==self@) | proved (predicate chain) |
| 13 | 43 | join_key | 2 assumes (dom.len) | proved (predicate chain) |
| 14 | 43 | rank_key | assume(inner@==self@) | proved (predicate chain) |
| 15 | 43 | select_key | assume(inner@==self@) | proved (predicate chain) |
| 16 | 43 | clone | 3 assumes | 2 assumes (clone workaround, non-actionable) |

**Net: 6 external_body removed, 10 assumes removed. 0 actionable holes remain.**

## Chap52 AdjTableGraphMtPer — Not Proved

The 3 Chap52 holes remain:

| # | Chap | Line | Hole | Blocker |
|---|------|------|------|---------|
| 1 | 52 | 228 | assume(partial_sum <= total_edges) | Needs inductive spec_sum_adj_sizes decomposition lemma |
| 2 | 52 | 240 | assume(count == spec_num_edges) | Same inductive lemma |
| 3 | 52 | 365 | assume(delete_vertex wf) | Needs MtPer map value-level ensures (collect doesn't expose key-value correspondence) |

**Why not proved:** These are algorithmic proofs, not lock-boundary issues. Hole 3
requires strengthening MtPer map's ensures to propagate value information, which
requires strengthening StPer collect's ensures first (it only guarantees wf + length,
not entry-to-table correspondence). Holes 1-2 require a ghost inductive proof
relating the loop sum to the recursive spec_sum_adj_sizes definition.

## Global Metrics

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Actionable holes | 14 | 8 | -6 |
| Clean chapters | 42 | 43 | +1 |
| Holed chapters | 4 | 3 | -1 |
| Verified count | -- | 5396 | -- |
| RTT passed | -- | 3083 | -- |
| PTT passed | -- | 157 | -- |

Chap43 moved from holed to clean.

## Techniques Used

- Ghost field in RwLock predicate struct (expected_view pattern)
- Type invariant linking predicate ghost to struct ghost
- `use_type_invariant(self)` before acquire_read for proof chain
- `from_st_table` helper with `requires inner.spec_orderedtablestper_wf()`
- `obeys_view_eq_trigger::<K>()` to satisfy StPer preconditions
