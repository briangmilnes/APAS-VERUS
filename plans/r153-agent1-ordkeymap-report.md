# R153 Agent 1 Report: OrdKeyMap union/intersect/difference

## Summary

Added `union`, `intersect`, and `difference` to OrdKeyMap (Chap38). All three
operations are fully verified with 0 holes, 0 assumes, 0 external_body.

## What was added

| # | Chap | File | Operation | Lines | Complexity |
|---|------|------|-----------|-------|------------|
| 1 | 38 | OrdKeyMap.rs | `union` | ~210 | O(n * m) |
| 2 | 38 | OrdKeyMap.rs | `intersect` | ~130 | O(n * m) |
| 3 | 38 | OrdKeyMap.rs | `difference` | ~120 | O(n * m) |

## Specifications

- **union**: `combined@.dom() =~= self@.dom().union(other@.dom())`. Self-only keys
  preserve self's value. Other-present keys get other's value (other wins on collision).
- **intersect**: `common@.dom() =~= self@.dom().intersect(other@.dom())`. Values
  preserved from self.
- **difference**: `remaining@.dom() =~= self@.dom().difference(other@.dom())`. Values
  preserved from self.

All three guarantee `combined.spec_ordkeymap_wf()` in ensures.

## Implementation approach

Followed OrderedTableStEph's iterative pattern: sort via `in_order()`, iterate entries,
conditionally insert into a new BST. Union uses a two-phase approach (Phase 1: iterate
self with other-lookup; Phase 2: iterate other, add self-absent keys).

Key simplification vs OrderedTableStEph: no closure parameter for value combining.
Value tracking uses map-level equality (`p.1 == other_map[p.0]`) instead of
set-level containment (`other_tree.contains(p)`), avoiding a problematic ghost-variable
chain through `other.inner@`.

## Verification

- Isolate: 1203 verified, 0 errors
- Full: 5749 verified, 0 errors
- RTT: 3690 passed, 0 skipped

## Techniques

- `obeys_view_eq_trigger::<K>()` assertion before loops to trigger broadcast axiom
  for `obeys_view_eq::<K>()` (not included in `spec_ordkeymap_wf`, needed for `find`).
- Map-level value tracking invariant avoids need to reason about `other.inner@` directly.
- Freshness proof via pairwise-distinct keys from `lemma_sorted_keys_pairwise_distinct`.
