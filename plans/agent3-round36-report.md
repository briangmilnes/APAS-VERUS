# Agent 3 Round 36 Report

## Summary

Proved 12 Mt ordering delegations (6 in OrderedTableMtPer, 6 in OrderedSetMtEph)
by removing `external_body` and adding `assume(inner@ =~= self@)` RwLock ghost bridges.
Fixed 46 missing trigger annotations across 6 Chap43 files, eliminating all Chap43
trigger warnings from validation output.

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 43 | OrderedTableMtPer.rs | 8 (6 ext + 2 asm) | 8 (8 asm) | -6 ext | 0 real actionable; all 8 are RWLOCK_GHOST FPs |
| 2 | 43 | OrderedSetMtEph.rs | 15 (8 ext + 7 asm) | 15 (2 ext + 13 asm) | -6 ext | 2 real actionable (filter, to_seq) |
| 3 | 43 | OrderedSetStEph.rs | 11 | 11 | 0 | Trigger fixes only |
| 4 | 43 | OrderedSetStPer.rs | 9 | 9 | 0 | Trigger fixes only |
| 5 | 43 | OrderedTableMtEph.rs | — | — | 0 | Trigger fixes only |
| 6 | 43 | AugOrderedTableMtEph.rs | — | — | 0 | Trigger fixes only |

**Net: -12 external_body (converted to assume-bridged verified bodies)**

The total hole count didn't change because each removed `external_body` was replaced by
an `assume(inner@ =~= self@)` (the standard RwLock ghost bridge). But the quality
improvement is significant: 12 function bodies are now verified by Verus instead of opaque.

## Techniques Used

1. **RwLock read-delegation with ghost bridge**: `acquire_read` -> `borrow` -> call StPer/StEph
   method -> `assume(inner@ =~= self@)` -> `release_read`. The assume bridges the locked
   inner table's view to the Mt wrapper's ghost view.

2. **Explicit trigger annotations**: Added `#[trigger] TotalOrder::le(v, t)` for first/last
   ensures, `#![trigger t@]` for previous/next/rank/select ensures, and
   `#[trigger] old(self)@.contains(x)` for split/split_rank ensures. Consistent pattern
   across all 6 Chap43 files.

3. **wf ensures propagation**: Added `spec_orderedsetmteph_wf()` to `from_st` ensures
   in OrderedSetMtEph.rs.

## Functions Proved (12 total)

### OrderedTableMtPer.rs (6)
- `first_key` (line 402): read-lock delegation to StPer
- `last_key` (line 413): read-lock delegation to StPer
- `previous_key` (line 424): read-lock delegation to StPer
- `next_key` (line 435): read-lock delegation to StPer
- `rank_key` (line 482): read-lock delegation to StPer
- `select_key` (line 493): read-lock delegation to StPer

### OrderedSetMtEph.rs (6)
- `first` (line 378): read-lock delegation to StEph
- `last` (line 389): read-lock delegation to StEph
- `previous` (line 400): read-lock delegation to StEph
- `next` (line 411): read-lock delegation to StEph
- `rank` (line 459): read-lock delegation to StEph
- `select` (line 470): read-lock delegation to StEph

## Trigger Fixes (46 annotations across 6 files)

- OrderedTableMtPer.rs: 6 (trait ensures)
- OrderedSetMtEph.rs: 6 (trait ensures)
- OrderedSetStEph.rs: 16 (trait + impl ensures)
- OrderedSetStPer.rs: 14 (trait + impl ensures)
- OrderedTableMtEph.rs: 6 (trait ensures)
- AugOrderedTableMtEph.rs: 6 (trait ensures)

Result: 0 Chap43 trigger warnings in validate output (was ~39).

## Remaining Holes

### OrderedSetMtEph.rs (2 real actionable)
- `filter` (external_body): needs acquire_write pattern with ghost spec_pred bridging
- `to_seq` (external_body): needs AVLTreeSeqStPerS -> ArraySeqStPerS conversion loop with invariant

### OrderedSetStEph.rs / OrderedSetStPer.rs
All ordering methods remain external_body in the base St implementations. These need
algorithmic proofs (tree sortedness -> scanning correctness), not Mt delegation.

## Verification

- 4220 verified, 0 errors
- 0 Chap43 trigger warnings
