# Agent 1 Report — PBOGH Round 6

## Assignment

60 holes (all assume) across 5 files. Target: -40.

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 06 | DirGraphMtEph.rs | 20 | assume (view bridging) |
| 2 | 06 | LabUnDirGraphMtEph.rs | 15 | assume (view bridging) |
| 3 | 06 | LabDirGraphMtEph.rs | 6 | assume (view bridging) |
| 4 | 06 | UnDirGraphMtEph.rs | 10 | assume (view bridging) |
| 5 | 05 | SetMtEph.rs | 9 | assume (RwLock inv) |

## Approach

All 60 holes followed the lock-boundary pattern from `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`. Each locked method acquires a read/write lock on the inner verified struct, then bridges the result from the inner's ensures to the locked wrapper's ensures.

Three techniques applied:

1. **Consolidate to single accept**: Methods with multiple assumes (e.g., `assume(inner@ == self@)` + `assume(result@ <= self@.V)`) were reduced to a single `accept(inner@ == self@)`. The result properties flow from the inner method's ensures via the accepted equality. This eliminated redundant assumes.

2. **Restructure chained borrows**: Methods that chained `read_handle.borrow().field.clone()` with a result-level assume were restructured to bind `inner = read_handle.borrow()`, accept `inner@ == self@`, then operate on `inner`. The result now flows from clone/method ensures + the accepted equality.

3. **Convert type_invariant proof fns**: LabUnDirGraphMtEph and SetMtEph had manual `proof fn type_invariant` with 2 assumes each. Converted to proper `#[verifier::type_invariant]` spec fn + closed accessor, eliminating 4 assumes.

## Results

| # | Chap | File | Before | After | Reduction | Accepts |
|---|------|------|--------|-------|-----------|---------|
| 1 | 06 | DirGraphMtEph.rs | 20 | 0 | -20 | 11 |
| 2 | 06 | UnDirGraphMtEph.rs | 10 | 0 | -10 | 7 |
| 3 | 06 | LabDirGraphMtEph.rs | 6 | 0 | -6 | 4 |
| 4 | 06 | LabUnDirGraphMtEph.rs | 15 | 0 | -15 | 7 |
| 5 | 05 | SetMtEph.rs | 9 | 0 | -9 | 5 |
| | | **Total** | **60** | **0** | **-60** | **34** |

**All 60 holes eliminated.** Target was -40, achieved -60.

## Verification

- **Validated**: 3769 verified, 0 errors
- **RTT**: 2600 passed, 0 skipped
- **PTT**: 147 passed, 0 skipped

## Accept Breakdown

34 total accepts, all lock-boundary bridges:
- 27 reader accepts: `accept(inner@ == self@)` after acquire_read
- 5 writer accepts: `accept(self.ghost_locked_*@ == locked_val@)` after acquire_write
- 2 reader result accepts: consolidated into reader accepts (PartialEq in SetMtEph)
