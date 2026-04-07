# R151 Agent 2 Report: Mt Type Bounds (StTInMtT)

## Summary

Fixed veracity [23b] violations: replaced raw partial bounds (`Eq + Clone + Send + Sync + 'static`,
`MtKey + 'static`, `MtVal + 'static`) with the correct APAS-VERUS trait aliases throughout 13
target files across 6 chapters.

## Results

| Round | Verified | RTT | PTT |
|-------|----------|-----|-----|
| R151 Before | 5702 | 3690 | — |
| R151 After  | 5702 | 3690 | — |

(PTT not run per task rules.)

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 18 | ArraySeqMtEph.rs | `Clone + Eq + Send + Sync + 'static` → `StTInMtT` on free fn + ninject_par where |
| 2 | 18 | ArraySeqMtEphSlice.rs | Trait/impl/free fns: `Eq + Clone` → `StTInMtT`; map U param; IntoIterator; +import |
| 3 | 19 | ArraySeqMtEphSlice.rs | Same as Chap18/ArraySeqMtEphSlice.rs |
| 4 | 27 | ReduceContractMtEph.rs | `StTInMtT + Clone + 'static` → `StTInMtT` (removed redundant bounds) |
| 5 | 35 | OrderStatSelectMtEph.rs | Trait/impl/spec fn/free fns: `TotalOrder + Eq + Clone` → `StTInMtT + TotalOrder` |
| 6 | 35 | OrderStatSelectMtPer.rs | Same as OrderStatSelectMtEph.rs |
| 7 | 36 | QuickSortMtEphSlice.rs | Trait/impl/spec/proof/free fn: `TotalOrder + Eq + Clone` → `StTInMtT + TotalOrder` |
| 8 | 39 | BSTParaTreapMtEph.rs | `MtKey + ClonePreservesView + 'static` → `MtKey + ClonePreservesView` (replace_all) |
| 9 | 39 | BSTSetTreapMtEph.rs | `MtKey + ClonePreservesView + 'static` → `MtKey + ClonePreservesView` (replace_all) |
| 10 | 49 | MinEditDistMtEph.rs | Removed redundant `where T: Send + Sync + 'static` from method and free fn |
| 11 | 49 | MinEditDistMtPer.rs | Same as MinEditDistMtEph.rs |
| 12 | 49 | SubsetSumMtEph.rs | Removed `Send + Sync + 'static` from free fn; preserved `Into<i32> + Copy` in trait + impl |
| 13 | 49 | SubsetSumMtPer.rs | Same as SubsetSumMtEph.rs |

## Key Decisions

- **StTInMtT already includes `Send + Sync + 'static`**: All raw `Eq + Clone + Send + Sync + 'static`
  bounds became `StTInMtT`. No loss of capability — StTInMtT is strictly stronger.
- **MtKey already includes `'static`**: `MtKey + 'static` is redundant. Simplified to `MtKey`.
- **MtVal already includes `'static`**: `MtVal + 'static` is redundant. Simplified to `MtVal`.
- **SubsetSum `Into<i32> + Copy` preserved**: These are real algorithmic constraints (converting
  multiset elements to i32 for DP, copying elements through join arms). They must appear as
  `where` clauses on the trait method, not just on internal free functions.
- **Cascade fixes**: Changing trait T-bound from `Eq + Clone` to `StTInMtT` propagated to
  IntoIterator impls, spec functions, proof lemmas, and map/U type parameters in all affected
  files.
- **Pre-existing error noted**: `QuickSortMtEph.rs` (Chap36) has 1 pre-existing verification
  error (loop invariant, line 441) from Agent 1's TotalOrder work. Not our file, not our change.

## Validation

- Chap18 isolate: 1076 verified, 0 errors
- Chap19 isolate: 896 verified, 0 errors
- Chap27 isolate: 933 verified, 0 errors
- Chap35 isolate: 1306 verified, 0 errors
- Chap36 isolate: 938 verified, 0 errors (1 pre-existing error in QuickSortMtEph.rs, not our file)
- Chap39 isolate: 1295 verified, 0 errors
- Chap49 isolate: 1364 verified, 0 errors
- **Full validate**: 5702 verified, 0 errors
- **RTT**: 3690 passed, 0 failed
