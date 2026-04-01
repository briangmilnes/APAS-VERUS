# R132 Agent 3 Report: Parallel D&C RTTs

## Summary

Added 57 new RTTs across 3 test files exercising the parallel D&C map, reduce, and
filter paths at empty, singleton, small, odd, medium, and large sizes.

Fixed algorithmic bug: `reduce_par` stack-overflowed on empty sequences (missing
`len == 0` base case in 4 functions across 3 source files).

## Bug Fix

`reduce_par` (and `reduce_inner` in MtPer) had `a.seq@.len() > 0` in requires and
no `len == 0` base case. When called on an empty sequence, D&C recursion split into
two empty halves indefinitely, causing stack overflow. Fixed by:

1. Removing `a.seq@.len() > 0` precondition
2. Adding `len == 0` base case returning `id` with proof (`s =~= Seq::empty()`,
   `reveal_with_fuel(Seq::fold_left, 1)`)

Matches the existing pattern in `reduce_dc` (Chap18/ArraySeqMtEph.rs:1276).

| # | Chap | File | Function |
|---|------|------|----------|
| 1 | 18 | ArraySeqMtEph.rs | `reduce_par` |
| 2 | 18 | ArraySeqMtPer.rs | `reduce_par` |
| 3 | 18 | ArraySeqMtPer.rs | `reduce_inner` |
| 4 | 19 | ArraySeqMtEph.rs | `reduce_par` |

## Tests Added

| # | Chap | File | Tests Before | Tests After | New |
|---|------|------|-------------|------------|-----|
| 1 | 18 | TestArraySeqMtEph.rs | 10 | 27 | 17 |
| 2 | 18 | TestArraySeqMtPer.rs | 10 | 27 | 17 |
| 3 | 19 | TestArraySeqMtEph.rs | 20 | 43 | 23 |

Total new tests: 57

## Test Matrix Per File

Each file received these tests for map_par, reduce_par, filter_par:

| Operation | empty | singleton | small(5) | odd(77) | medium(100) | large(10K) | all_pass | none_pass |
|-----------|-------|-----------|----------|---------|-------------|------------|----------|-----------|
| map_par | x | x | x | x | (18 existing) | x | - | - |
| reduce_par | x | x | x | x | (18 existing) | x | - | - |
| filter_par | x | x keep/drop | x | - | (18 existing) | x | x | x |

Note: Chap19 got additional medium-size tests for map/reduce/filter since it had none.

## Validation

- Verus: Chap18 995 verified 0 errors, Chap19 827 verified 0 errors
- RTT: 3583 passed, 0 failed, 0 skipped
