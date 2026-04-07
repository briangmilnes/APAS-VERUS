# R155 Agent 1 Report: OrdKeyMap Combiner-Function Union/Intersect + new() wf + split disjointness

## Summary

Added `union_with` and `intersect_with` combiner-function variants to OrdKeyMap,
strengthened `new()` ensures with `dom().finite()`, and added disjointness ensures
to `split`.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 38 | OrdKeyMap.rs | Added `union_with<F>` to trait + impl (~250 lines) |
| 2 | 38 | OrdKeyMap.rs | Added `intersect_with<F>` to trait + impl (~200 lines) |
| 3 | 38 | OrdKeyMap.rs | Strengthened `new()` ensures: `empty@.dom().finite()` |
| 4 | 38 | OrdKeyMap.rs | Added `parts.0@.dom().disjoint(parts.2@.dom())` to `split` ensures |
| 5 | 38 | TestOrdKeyMap.rs | Added 10 RTTs for union_with/intersect_with |

## Task A: new() wf

Added `empty@.dom().finite()` to `new()`'s ensures. Full `spec_ordkeymap_wf()` was
not added because the axiom predicates (`obeys_cmp_spec`, `view_ord_consistent`,
`spec_pair_key_determines_order`) are not broadcast-provable for generic types.
Adding them as requires would cascade through OrderedTableStEph::empty() →
OrderedTableMtEph::empty() → all callers of empty(). The structural parts
(bst_wf, key_unique, view_gen) are proven internally but not exposed through
the trait because the trait's abstract spec cannot express them.

**Recommendation**: To fully ensure wf from `new()`, the trait needs axiom requires.
This should be coordinated across Chap38/Chap43 in a single round.

## Task B: union_with and intersect_with

Both follow the same 2-phase loop pattern as existing `union`/`intersect` but replace
fixed value selection with a combiner function call. Specs use `combine.ensures(...)`
for collision cases, matching OrderedTableStEph's convention.

- `union_with`: Phase 1 iterates self, calls combine when key exists in other.
  Phase 2 adds other-only keys. Ensures domain is union, self-only/other-only
  values preserved, both-key values via combine.ensures.
- `intersect_with`: Single phase iterates self, calls combine when key in other.
  Ensures domain is intersection, values via combine.ensures.

## Task C: split disjointness

Added `parts.0@.dom().disjoint(parts.2@.dom())` to split's ensures. Proof bridges
from BST's `left@.disjoint(right@)` at the set level to Map domain disjointness.

## Verification

- `validate isolate Chap38`: 1216 verified, 0 errors
- `validate` (full): 5757 verified, 0 errors
- `rtt`: 3727 passed (10 new), 0 failed
