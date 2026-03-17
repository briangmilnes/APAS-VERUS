# Veracity Proof Hole Detector: Issues and Fixes

## Issue 1: Section 4.1 vs Section 3 Hole Count Discrepancy (182 vs 171)

**Symptom**: Section 4.1 ("Worst src/* Directories") sums to 182 holes
across 9 holed chapters. Section 3 ("Summary of Holes") says 171. The
`chapter-cleanliness-status.sh` reads section 4.1 and reports 182.

**Root cause**: Section 4.1 per-directory counts include some
`fn_missing_*` warnings as "holes", while section 3 correctly excludes
them. Verified by examining Chap41:

- Per-file `Holes:` lines sum: 9 + 8 + 1 + 4 = 22
- Section 4.1 reports: 25
- Difference: 3 = exactly the number of `fn_missing_requires` errors
  in holed Chap41 files (parallel_filter, parallel_intersect, parallel_sort)

The global per-file `Holes:` lines sum to exactly 171, matching section 3.

**Evidence that fn_missing is NOT always counted**: Chap05 has 14
`fn_missing_*` errors but shows 0 in section 4.1 (because Chap05 has
no real holes). The inflation only appears in chapters that also have
real holes.

**Fix**: Section 4.1 per-directory aggregation should count only real
proof holes (assume, external_body, unsafe impl, assume_specification,
external) — the same set counted by section 3. Exclude fn_missing_*
variants: fn_missing_requires, fn_missing_wf_requires,
fn_missing_wf_ensures, fn_missing_requires_ensures.

**Verification**: After fix, section 4.1 sum should equal section 3
total (currently 171).

## Issue 2: UNSAFE_SEND_SYNC Not Detected

**Symptom**: 0 UNSAFE_SEND_SYNC in structural FP list, despite 2
`unsafe impl Send/Sync` holes in Chap41/AVLTreeSetMtEph.rs.

**Evidence**:
```
src/Chap41/AVLTreeSetMtEph.rs:622: error: unsafe impl - unsafe impl<T: StTInMtT + Ord + 'static> Send for AVLTreeSetMtEph<T> {}
src/Chap41/AVLTreeSetMtEph.rs:623: error: unsafe impl - unsafe impl<T: StTInMtT + Ord + 'static> Sync for AVLTreeSetMtEph<T> {}
```

The struct definition:
```rust
pub struct AVLTreeSetMtEph<T: StTInMtT + Ord> {
    inner: Arc<RwLock<AVLTreeSetStEphS<T>, AVLTreeSetMtEphInv>>,
    ghost_set_view: Ghost<Set<T::V>>,
}
```

The `Ghost<Set<T::V>>` field triggers the need for unsafe Send/Sync
because Ghost doesn't auto-derive Send/Sync, but Ghost is zero-sized
and erased at runtime — this is a structural FP.

**Fix**: Implement UNSAFE_SEND_SYNC detection per the spec in
`plans/veracity-false-positive-detection-prompt.md`:
1. When you see `unsafe impl Send` or `unsafe impl Sync`, find the
   target type definition
2. Check if the type has any `Ghost<...>` fields
3. If yes, classify as UNSAFE_SEND_SYNC structural FP

Also check `src/vstdplus/threads_plus.rs` (ThreadShareablePlus has
unsafe Send/Sync at lines 125-126) — but vstdplus is excluded from
analysis, so this won't appear.

## Issue 3: Add "Real Actionable Holes" to Section 3

**Current output**:
```
Holes Found: 171 total
   25 × assume() (14%)
   1 × assume_specification (0%)
   2 × unsafe impl (1%)
   141 × external_body (82%)
   2 × external (1%)

Structural False Positives: 165 detected
   143 × EQ_CLONE_ASSUME
   10 × STD_TRAIT_IMPL
   5 × RWLOCK_GHOST
   4 × THREAD_SPAWN
   3 × OPAQUE_EXTERNAL
```

**Problem**: The 143 EQ_CLONE_ASSUME are correctly excluded from "Holes
Found" (they're `info:` lines). But 22 FPs (STD_TRAIT_IMPL +
RWLOCK_GHOST + THREAD_SPAWN + OPAQUE_EXTERNAL) ARE included in the 171
count because they're `error:` lines. The user must mentally subtract.

**Fix**: Add a third line to section 3:
```
Holes Found: 171 total
   ...

Structural False Positives: 165 detected
   143 × EQ_CLONE_ASSUME (not in hole count — accept() pattern)
   10 × STD_TRAIT_IMPL (in hole count)
   5 × RWLOCK_GHOST (in hole count)
   4 × THREAD_SPAWN (in hole count)
   3 × OPAQUE_EXTERNAL (in hole count)
   0 × UNSAFE_SEND_SYNC (in hole count)    ← after Issue 2 fix

Real Actionable Holes: 149 (171 total - 22 structural FPs in count)
```

This makes it obvious: 149 is the number the proof team works against.

## Issue 4: Per-Chapter Analysis Logs Overwritten

**Symptom**: `src/Chap43/analyses/veracity-review-verus-proof-holes.log`
contains only OrderedTableStPer.rs (10 holes), not all 11 Chap43 files.

**Root cause**: `scripts/all-holes-by-chap.sh` runs veracity per-file
(or runs on the last file) and overwrites the log each time. Only the
final file's output survives.

**Fix**: Either:
- (a) Run veracity on the whole directory: `veracity-review-proof-holes src/Chap43/`
  instead of individual files, OR
- (b) Append rather than overwrite, OR
- (c) Use the global log as the source of truth and remove per-chapter
  hole logs entirely (the global log already has per-file detail)

Option (a) is simplest. The per-chapter log should reflect the chapter's
full state.

## Non-Issues (Confirmed Working Correctly)

**EQ_CLONE_ASSUME count (143)**: Not inflated. There are ~70 types
with PartialEq/Clone impls, each contributing 1-4 accept() calls
inside eq/clone bodies. These are correctly classified as `info:` lines
and excluded from "Holes Found: 171". The 25 real assume() holes are
separate. No fix needed.

## Summary of Fixes

| # | Issue | Severity | Fix |
|---|-------|----------|-----|
| 1 | Section 4.1 includes fn_missing in directory counts | High | Filter fn_missing from per-dir aggregation |
| 2 | UNSAFE_SEND_SYNC not detected | Medium | Implement detection per spec |
| 3 | No "Real Actionable" summary | Low | Add computed line to section 3 |
| 4 | Per-chapter logs overwritten | Low | Run veracity on whole directory |
