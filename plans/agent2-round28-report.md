# Agent 2 Round 28 Report

## Summary

All 4 tasks completed. Chap40 is now fully clean: 0 holes across all 3 BST files.
Verification: 4118 verified, 0 errors. RTT: 2613 passed.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Change |
|---|------|------|-------------|------------|--------|
| 1 | 40 | BSTKeyValueStEph.rs | 1 (assume) | 0 | -1 |
| 2 | 40 | BSTReducedStEph.rs | 1 (external_body) | 0 | -1 |
| 3 | 40 | BSTSizeStEph.rs | 1 (external_body) | 0 | -1 |

**Total: 3 holes removed, 0 remaining in Chap40.**

## Tasks Completed

### Task 1: Fix fn_missing_requires (6 warnings)
Added real `requires` clauses (well-formedness predicates) to:
- `clone_link` and `compare_kv_links` in BSTKeyValueStEph.rs
- `clone_link` and `compare_reduced_links` in BSTReducedStEph.rs
- `compare_links` and `clone_link` in BSTSizeStEph.rs

### Task 2: Prove delete in BSTReducedStEph.rs
Removed `external_body` from `delete` (line 690). Implemented rotation-based `delete_link`
following the proven BSTKeyValueStEph pattern. Added 3 free spec functions
(`spec_root_key_link`, `spec_has_left_child_link`, `spec_has_right_child_link`) and root key
postconditions to `rotate_left`/`rotate_right` to expose key relationships after rotation.

### Task 3: Prove delete in BSTSizeStEph.rs
Same pattern as BSTReducedStEph but adapted for `Set<T>`. Removed `external_body` from
`delete` (line 575). Added matching spec functions and rotation postconditions.

### Task 4: Prove assume in BSTKeyValueStEph.rs
Removed `assume(spec_ordered_link(link))` at line 1276 (rotate_left branch of `delete_link`).
This was a Z3 conjunction flakiness issue: all 4 sub-assertions of `spec_ordered_link`
verified individually but Z3 could not form the conjunction. The fix required:

1. Pre-take reveal: `reveal_with_fuel(spec_ordered_link, 2)` + `assert(spec_ordered_link(link))`
   while link still points to `Some(rotated)`, establishing ordering facts before `take()`.
2. Post-delete: incremental conjunction building with ghost variables `c1`-`c4`, then an
   explicit equivalence assertion `assert(spec_ordered_link(link) == (c1 && c2 && c3 && c4))`
   which guided Z3 to connect the conjunction to the spec function definition.

## Techniques Used

- Rotation-based delete: take/modify/put-back pattern with ghost content captures
- Free spec functions for rotation postconditions (trait method specs are uninterpreted)
- `reveal_with_fuel` timing: before `take()` to establish ordering on the live Link
- Ghost variable incremental conjunction building for Z3 conjunction flakiness
- Explicit equivalence assertion to bridge ghost conjunction to spec function

## Verification Counts

- Before: 4111 verified (R26 baseline)
- After: 4118 verified, 0 errors
- RTT: 2613 passed
- Chap40 modules: 3 clean (100%), 0 holed
