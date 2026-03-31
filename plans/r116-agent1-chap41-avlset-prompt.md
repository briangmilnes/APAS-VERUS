# R116 Agent 1 — Strengthen AVLTreeSet MtPer/MtEph specs (Chap41). AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 31 warnings on Chap41 (AVLTreeSet).
Two files are affected: `AVLTreeSetMtPer.rs` and `AVLTreeSetMtEph.rs`.
The bulk is a single repeated pattern: missing `view_ord_consistent::<T>()`
and `vstd::laws_cmp::obeys_cmp_spec::<T>()` requires clauses.

## Warnings by category

### Missing `view_ord_consistent` + `obeys_cmp_spec` requires (18 warnings)

Both MtPer and MtEph are missing these two requires on multiple functions.
StEph/StPer have them. These are mechanical additions — add the two clauses
to each function's requires, then verify callers still satisfy them.

**MtEph** (12 warnings, 6 functions):
- `filter` (line 148): missing both
- `intersection` (line 167): missing both
- `difference` (line 174): missing both
- `union` (line 181): missing both
- `delete` (line 199): missing both
- `insert` (line 206): missing both

**MtPer** (6 warnings, 3 functions + 2 bare):
- `filter` (line 133): missing both
- `intersection` (line 151): StPer has requires, MtPer has none
- `difference` (line 155): StPer has requires, MtPer has none
- `union` (line 159): StPer has requires, MtPer has none
- `delete` (line 171): StPer has requires, MtPer has none
- `insert` (line 175): StPer has requires, MtPer has none

For the MtPer functions that have NO requires at all (intersection, difference,
union, delete, insert), check what StPer requires and add the full set, not
just the two cmp clauses.

### Missing `_iter` functions (14 fns across 2 warnings)

MtPer missing 7: `find_iter`, `insert_iter`, `delete_iter`, `filter_iter`,
`intersection_iter`, `union_iter`, `difference_iter`.

MtEph missing the same 7.

These are iterator-returning variants of the main operations. Check if the
underlying iterator infrastructure exists in the Mt variants. If not, these
are larger tasks — document what's needed but don't implement unless the
iterator types already exist.

### StEph vs StPer (3 missing fns, 1 warning)

StEph missing `spec_elements_sorted_per`, `spec_values_seq_per`,
`insert_sorted_per` from StPer's `TotalOrderTrait`. These are Per-specific
sorted-sequence specs that StEph doesn't need (Eph uses `&mut`, not sorted
persistent sequences). Likely a false positive or intentional omission.

### Ensures differences (2 warnings)

- MtEph `to_seq` (line 119): MtPer has 4 ensures, MtEph has 3.
- MtEph `from_seq` (line 138): MtPer requires `seq@.len() <= usize::MAX - 2`,
  MtEph doesn't.

Check if these are needed.

## Work order

1. Read `src/Chap41/AVLTreeSetMtEph.rs` and `src/Chap41/AVLTreeSetMtPer.rs`.
2. Read `src/Chap41/AVLTreeSetStEph.rs` and `src/Chap41/AVLTreeSetStPer.rs`
   for the reference requires.
3. Add `view_ord_consistent::<T>()` and `obeys_cmp_spec::<T>()` to all 12
   functions (6 MtEph + 6 MtPer).
4. Check MtPer's bare functions for full requires from StPer.
5. Check `to_seq` ensures gap and `from_seq` requires gap.
6. Document the `_iter` functions — are they implementable?
7. Validate: `scripts/validate.sh isolate Chap41`.
8. RTT: `scripts/rtt.sh Chap41`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept.
- Adding requires may break callers — check RTTs and fix call sites.
- No subagents.

## STEP 20

## Report

Write `plans/agent1-r116-chap41-avlset-report.md`. Include before/after
warning count.
