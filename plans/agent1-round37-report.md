# Agent 1 — Round 37 Report

## Summary

Proved 4 ordering operations in OrderedTableMtEph.rs by porting the TotalOrder
bridging proofs from OrderedTableStEph.rs. Made 2 TableMtEph lemmas public to
support the proofs.

## Verification

- **4290 verified, 0 errors** (was 4282 in R36)
- **2613 RTT passed**, 147 PTT passed
- **71 actionable holes** (was 75)

## Changes

### Tier 1: OrderedTableMtEph Ordering Ops (−4 holes)

| # | Chap | File | Function | Before | After | Technique |
|---|------|------|----------|--------|-------|-----------|
| 1 | 43 | OrderedTableMtEph.rs | first_key | external_body | verified | TotalOrder min scan + clone_plus |
| 2 | 43 | OrderedTableMtEph.rs | last_key | external_body | verified | TotalOrder max scan + clone_plus |
| 3 | 43 | OrderedTableMtEph.rs | previous_key | external_body | verified | TotalOrder predecessor scan |
| 4 | 43 | OrderedTableMtEph.rs | next_key | external_body | verified | TotalOrder successor scan |

**Technique**: Ported the identical proof structure from OrderedTableStEph.rs
(proved by Agent2 in R36). Each function iterates over `self.base_table.entries`
using `TotalOrder::cmp` to find min/max/predecessor/successor, with loop
invariants tracking the best candidate via `spec_index`. Post-loop proof bridges
from entry-level facts to map-domain-level ensures using
`lemma_entries_to_map_contains_key` and `lemma_entries_to_map_key_in_seq`.

**Supporting changes**:
- Added `pub` to `lemma_entries_to_map_contains_key` and
  `lemma_entries_to_map_key_in_seq` in `src/Chap42/TableMtEph.rs` (were private,
  needed by OrderedTableMtEph proofs).
- Added `clone_plus` import and changed feq imports to glob in
  `src/Chap43/OrderedTableMtEph.rs`.

### Tier 1 Remaining: rank_key, select_key (kept external_body)

Both `rank_key` and `select_key` are also external_body in OrderedTableStEph.rs —
no proved template to copy. The ensures use `Set::filter` with existential
quantifiers over TotalOrder, which requires linking a concrete loop count to
abstract set filter cardinality. This is a hard proof obligation shared across
StEph and MtEph.

### Tier 2: AVLTreeSetStEph Assumes (not attempted)

Two assumes at lines 1059, 1334:
```rust
assume(new_vec@.len() < usize::MAX);
```
Root cause: wf bounds tree size to `< usize::MAX`, but insert adds one element
needing `+ 1 < usize::MAX`. Fix requires adding
`requires old(self)@.len() + 1 < usize::MAX as nat` to the trait's `insert`
method, which cascades to 17 files across Chap41–55. Not attempted due to
cascade scope.

## Files Modified

| # | Chap | File | Nature of change |
|---|------|------|-----------------|
| 1 | 42 | TableMtEph.rs | Made 2 lemmas pub |
| 2 | 43 | OrderedTableMtEph.rs | Proved 4 ordering ops, added imports |

## Remaining Holes in OrderedTableMtEph.rs

| # | Chap | File | Function | Type | Blocker |
|---|------|------|----------|------|---------|
| 1 | 43 | OrderedTableMtEph.rs | rank_key | external_body | Set::filter cardinality proof (also blocked in StEph) |
| 2 | 43 | OrderedTableMtEph.rs | select_key | external_body | Set::filter cardinality proof (also blocked in StEph) |
