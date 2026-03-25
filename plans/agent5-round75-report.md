# Agent 5 — Round 75 Report

## Summary

Eliminated 12 proof holes across 5 files by removing `external_body` from iterator
functions and rewriting BSTSetAVLMtEph algorithmic bodies to use explicit while loops
instead of std iterator chains and BTreeSet.

## Verification

- validate: 4771 verified, 0 errors
- RTT: 2619 tests passed
- PTT: 157 tests passed

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | AVLTreeSeqStEph.rs | 2 | 0 | -2 |
| 2 | 37 | AVLTreeSeqStPer.rs | 2 | 0 | -2 |
| 3 | 37 | BSTSetAVLMtEph.rs | 17 | 13 | -4 |
| 4 | 43 | OrderedTableMtEph.rs | 2 | 0 | -2 |
| 5 | 43 | AugOrderedTableMtEph.rs | 2 | 0 | -2 |
| **Total** | | | **25** | **13** | **-12** |

## Techniques Used

1. **Iterator external_body removal** (AVLTreeSeqStEph/StPer): Previous session strengthened
   `push_left_iter` ensures from `ensures true` to preserving Ghost fields, enabling
   `iter()` and `into_iter()` bodies to verify. Added `requires self.spec_wf()` to
   `into_iter()` since it delegates to `iter()`.

2. **Mt iterator external_body removal** (OrderedTableMtEph): Replaced `for` loop with
   explicit `while` loop with invariants. Added RWLOCK_GHOST `assume(inner.spec_orderedtablesteph_wf())`
   after acquiring read lock. Discovered `ArraySeqStPerS::spec_arrayseqstper_wf()` is
   always `true` (Vec-backed), eliminating wf assumes on `in_order()` return values.

3. **Mt iterator delegation** (AugOrderedTableMtEph): `iter()` just delegates to
   `self.base_table.iter()`. `into_iter()` requires `spec_augorderedtablemteph_wf()` which
   includes `base_table.spec_orderedtablemteph_wf()`.

4. **BSTSetAVLMtEph rewrite**: Removed BTreeSet dependency entirely. Replaced std iterator
   chains with explicit `while` loops over `in_order()` + `nth()`. Replaced
   `from_sorted_iter` (generic IntoIterator) with `build_from_vec` (Vec<T>). Eliminated
   ParaPair from `join_pair` and `join_m` using sequential insert loops.

## Remaining Holes in BSTSetAVLMtEph (13)

| # | Chap | Type | Function | Notes |
|---|------|------|----------|-------|
| 1-6 | 37 | assume | values_vec, delete, split, join_pair, join_m, iter_in_order | `assume(obeys_feq_clone::<T>())` — structural req for `in_order()` |
| 7-8 | 37 | assume | filter, reduce | Same assume inside external_body |
| 9-10 | 37 | external_body | filter, reduce | FnMut closure `requires` not provable in Verus |
| 11-13 | 37 | external_body | union, intersection, difference | Recursive ParaPair! closures — deferred |

## Notes

- The 8 `assume(obeys_feq_clone::<T>())` are structural: `BSTAVLMtEph::in_order()` requires
  this predicate (the Verus eq/clone workaround). These are analogous to RWLOCK_GHOST assumes.
- `filter` and `reduce` have verified loop bodies inside `external_body` — the only unverified
  part is the FnMut closure `requires` obligation. These could be proven in a future Verus
  version with better FnMut support.
- `union`, `intersection`, `difference` need recursive ParaPair! closures and are deferred.
- This work pioneers the pattern for BSTSetRBMtEph (16 holes) and BSTSetSplayMtEph (13 holes).
