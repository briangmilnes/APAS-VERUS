# R158 Agent 3 — Debug/Display [14] + Copyright [24] Report

## Summary

Fixed all [14] (Debug/Display outside verus!) and [24] (copyright line format) warnings
in `src/standards/`. Both warning types were concentrated exclusively in the standards
directory.

## Investigation

The task description mentioned ~181 [14] warnings and ~68 [24] warnings. The previous
session found 0 of each because the veracity command included `-e standards`, which
excluded the only directory containing these warnings. Running without `-e standards`
revealed the actual counts:

- **38 [14] warnings** (19 structs × 2 impls each)
- **28 [24] warnings** (28 standards files with wrong copyright prefix)

## Task B: Copyright [24] — 28 files fixed

All 28 standards files had `//  Copyright (C) 2025 ...` (plain comment, two spaces).
The correct format used by chapter files is `//! Copyright (C) 2025 ...` (doc comment).

Changed line 1 of all 28 files from `//  Copyright` to `//! Copyright`:

| # | File |
|---|------|
| 1 | arc_usage_standard.rs |
| 2 | capacity_bounds_standard.rs |
| 3 | constructor_feq_standard.rs |
| 4 | deep_view_standard.rs |
| 5 | finite_sets_standard.rs |
| 6 | helper_function_placement_standard.rs |
| 7 | hfscheduler_standard.rs |
| 8 | iterative_vs_recursive_standard.rs |
| 9 | iterator_ptt_standard.rs |
| 10 | iterators_standard.rs |
| 11 | mod_standard.rs |
| 12 | mt_type_bounds_standard.rs |
| 13 | multi_struct_standard.rs |
| 14 | mut_standard.rs |
| 15 | no_unsafe_standard.rs |
| 16 | partial_eq_eq_clone_standard.rs |
| 17 | rwlock_tsm_standard.rs |
| 18 | spec_naming_convention.rs |
| 19 | spec_wf_standard.rs |
| 20 | table_of_contents_standard.rs |
| 21 | toplevel_coarse_rwlocks_for_mt_modules.rs |
| 22 | total_order_standard.rs |
| 23 | tsm_standard.rs |
| 24 | using_closures_standard.rs |
| 25 | using_hashmap_standard.rs |
| 26 | using_rand_standard.rs |
| 27 | view_standard.rs |
| 28 | wrapping_iterators_standard.rs |

## Task A: Debug/Display [14] — 9 files, 19 structs fixed

Added `impl Debug` and `impl Display` outside `verus!` for all flagged structs.
No modifications inside `verus!`.

| # | File | Structs Added |
|---|------|---------------|
| 1 | arc_usage_standard.rs | Widget, WidgetInv, LockedWidget |
| 2 | capacity_bounds_standard.rs | BoundedStack |
| 3 | finite_sets_standard.rs | FiniteCollection |
| 4 | hfscheduler_standard.rs | BoundedCounterInv |
| 5 | iterative_vs_recursive_standard.rs | Numbers |
| 6 | partial_eq_eq_clone_standard.rs | Collection\<T\> |
| 7 | rwlock_tsm_standard.rs | CountDown, CountDownLockInterior, LockedCdStateMachine |
| 8 | spec_naming_convention.rs | BoundedCounter, BoundedCounterInv, LockedBoundedCounter |
| 9 | spec_wf_standard.rs | Container\<T\>, VecWrapper\<T\> |
| 10 | toplevel_coarse_rwlocks_for_mt_modules.rs | CountDown, CountDownInv, LockedCountDown |

## Verification Results

- **validate.sh**: 5763 verified, 0 errors
- **rtt.sh**: 3776 tests passed, 0 failed

## Warning Counts Before/After

| Warning | Before | After |
|---------|--------|-------|
| [14] Debug/Display | 38 | 0 |
| [24] Copyright | 28 | 0 |
