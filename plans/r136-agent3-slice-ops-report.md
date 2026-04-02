# R136 Agent 3 — Add map, reduce, filter to ArraySeqMtEphSliceS

## Summary

Added three parallel D&C operations to `src/Chap19/ArraySeqMtEphSlice.rs`:
reduce, map, and filter. All three exploit O(1) slice split (`Arc::clone` +
window adjust) for D&C via `join()`. Updated the coarse lock experiment to
use the slice type instead of the Vec-backed type.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 19 | ArraySeqMtEphSlice.rs | Added `reduce`, `map`, `filter` to trait + impl |
| 2 | 19 | ArraySeqMtEphSlice.rs | Added bare impl: `reduce_dc`, `map_dc_vec`, `filter_dc_vec`, `lemma_monoid_fold_left` |
| 3 | 19 | ArraySeqMtEphSlice.rs | Added imports: join, MtReduceFn/MtMapFn/MtPred, spec_monoid |
| 4 | — | coarse_lock_parallel_tsm.rs | Switched from ArraySeqMtEphS to ArraySeqMtEphSliceS |
| 5 | — | Cargo.toml | Registered TestChap19ArraySeqMtEphSlice test binary |
| 6 | — | TestArraySeqMtEphSlice.rs | Added 17 new RTTs + fixed 15 existing (wrong type name) |

## Operation details

| Operation | Split | Span | Rejoin | Ensures |
|-----------|-------|------|--------|---------|
| reduce | O(1) slice | O(lg n) | O(1) scalar | `== spec_backing_seq().fold_left(id, spec_f)` |
| map | O(1) slice | O(n) | O(n) Vec concat | `mapped.len == self.spec_len()` + wf fields |
| filter | O(1) slice | O(n) | O(n) Vec concat | `filtered.spec_len() <= self.spec_len()` + wf |

### Key difference from Vec-backed D&C

The split is O(1): `self.slice(0, mid)` does `Arc::clone` + window adjust. The
Vec-backed `subseq_copy(0, mid)` copies O(mid) elements. Everything else (join
structure, closure specs, combine step) follows the same pattern.

## Proof techniques

- **reduce**: `lemma_fold_left_split` + `lemma_monoid_fold_left` (duplicated from
  Chap19/ArraySeqMtEph.rs for standalone compliance). Element-wise assertions to
  prove `left.spec_backing_seq() =~= parent.spec_backing_seq().subrange(0, mid)`.
- **map/filter**: Simple length ensures. Wf propagated via concrete field access
  (`mapped.start + mapped.len <= (*mapped.data)@.len()`) to avoid trait cycle.
- **Trait cycle avoidance**: `map` returns `ArraySeqMtEphSliceS<U>` (different type
  param), so calling trait spec fns on it creates a cycle. Used concrete field access
  instead.

## Experiment update

`coarse_lock_parallel_tsm.rs` now stores `ArraySeqMtEphSliceS<u64>` inside the lock
and calls `interior.seq.reduce(...)` and `interior.seq.map(...)` directly. The full
architecture: coarse lock + TSM + O(1) slice split + Mt type's own parallel operations.

## Validation

- Verification: 5488 (with experiment), 5481 (without) — 0 errors
- RTT: 3616 passed (3584 before + 32 new/fixed)
- PTT: 221 passed
- Zero assumes, zero accepts in new code
