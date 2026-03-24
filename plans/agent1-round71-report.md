# R71 Agent 1 Report: Iterator Standard Review & Fixes — Chap05-06, Chap17, Chap18/ArraySeq

## Standards Referenced

- `src/standards/iterators_standard.rs` — 10-component standard for Vec-backed collections.
- `src/standards/wrapping_iterators_standard.rs` — Wrapping pattern for modules that delegate to an inner module's iterator.

## The 10 Required Components

| # | Component | Description |
|---|-----------|-------------|
| C1 | Iterator struct | Custom `FooIter<'a, T>` with inner field |
| C2 | View for iter | `type V = (int, Seq<T>)` |
| C3 | iter_invariant | `pub open spec fn iter_invariant(it: &FooIter) -> bool` |
| C4 | Iterator::next | Two-arm ensures (None/Some) |
| C5 | Ghost iter struct | `FooGhostIterator<'a, T>` with pos/elements/phantom |
| C6 | ForLoopGhostIteratorNew | `ghost_iter()` spec fn |
| C7 | ForLoopGhostIterator | 6 spec fns: exec_invariant, ghost_invariant, ghost_ensures, ghost_decrease, ghost_peek_next, ghost_advance |
| C8 | Ghost View | `type V = Seq<T>`, body = `elements.take(pos)` |
| C9 | iter() method | With ensures: `it@.0 == 0`, contents match, `iter_invariant(&it)` |
| C10 | IntoIterator for &Self | Delegates to iter() |

## Results Summary

All 7 issues (A–G) identified in the review have been fixed and verified.

| # | Chap | File | Before | After | Issues Fixed |
|---|------|------|--------|-------|--------------|
| 1 | 05 | SetStEph.rs | 8.5/10 | 10/10 | A (iter_invariant in ensures), C (IntoIterator for &Self) |
| 2 | 05 | SetMtEph.rs | 9.5/10 | 10/10 | A (iter_invariant in ensures) |
| 3 | 05 | RelationStEph.rs | 8.5/10 | 10/10 | A (iter_invariant in ensures), B (added iter_invariant spec fn) |
| 4 | 05 | MappingStEph.rs | 8.5/10 | 10/10 | A (iter_invariant in ensures), B (added iter_invariant spec fn) |
| 5 | 06 | DirGraphStEph.rs | 1.5/10 | 10/10 | E (full wrapping iterator), G (removed dead ghost structs) |
| 6 | 06 | DirGraphMtEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 7 | 06 | LabDirGraphStEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 8 | 06 | LabDirGraphMtEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 9 | 06 | UnDirGraphStEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 10 | 06 | UnDirGraphMtEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 11 | 06 | LabUnDirGraphStEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 12 | 06 | LabUnDirGraphMtEph.rs | 1/10 | 10/10 | E (full wrapping iterator) |
| 13 | 17 | MathSeq.rs | 9.5/10 | 10/10 | D (iter() ensures), F (GhostIter→GhostIterator rename) |
| 14 | 18 | ArraySeqStEph.rs | 10/10 | 10/10 | (reference, no changes) |

## Verification

- **Validate**: 4446 verified, 0 errors
- **RTT**: 2528 tests passed, 0 skipped
- **PTT**: 145 tests passed, 0 skipped

## Issue Details

### Issue A: iter() missing `iter_invariant(&it)` in ensures — FIXED (5 files)

Files: SetStEph, SetMtEph, RelationStEph, MappingStEph, MathSeq.
Added `iter_invariant(&it)` to iter() trait ensures in all five files.

### Issue B: Missing iter_invariant spec fn — FIXED (2 files)

Files: RelationStEph, MappingStEph.
Added `pub open spec fn iter_invariant` with body `0 <= it@.0 <= it@.1.len()`.

### Issue C: Missing IntoIterator for &Self — FIXED (1 file)

File: SetStEph.
Added `impl IntoIterator for &'a SetStEph<T>` delegating to `self.iter()`.

### Issue D: MathSeq iter() missing ensures entirely — FIXED (1 file)

File: MathSeq.
Added `ensures it@.0 == 0, it@.1 == self.data@, iter_invariant(&it)`.

### Issue E: Chap06 graph files lack wrapping iterators — FIXED (8 files)

All eight Chap06 graph files now have full 10-component wrapping iterators:
- Custom iterator struct wrapping `SetStEphIter<'a, V>`
- View, iter_invariant, Iterator::next with two-arm ensures
- Ghost iterator struct, ForLoopGhostIteratorNew, ForLoopGhostIterator (6 spec fns)
- Ghost View, IntoIterator for &Self using the new wrapper type

### Issue F: MathSeqGhostIter naming — FIXED (1 file)

File: MathSeq.
Renamed `MathSeqGhostIter` to `MathSeqGhostIterator` throughout.

### Issue G: Unused ghost view structs in DirGraphStEph — FIXED (1 file)

File: DirGraphStEph.
Removed dead `DirGraphVertexIterView` and `DirGraphArcIterView` structs.

## Techniques

- **Wrapping iterator template**: Applied the standard wrapping pattern from `wrapping_iterators_standard.rs` to all 8 graph files, customized for each type's bounds (StT+Hash for StEph, StTInMtT+Hash+'static for MtEph, HashOrd for LabUnDirGraphStEph).
- **TOC updates**: Added `// 10. iterators` section header to 5 graph files that had TOC comments but lacked the entry.
