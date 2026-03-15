# Agent 1 — Round 18 Report: Add Ghost(spec_fn) to filter

## Summary

Added `Ghost(spec_pred)` parameter to every `filter` function across Chap38, Chap41, and Chap42, following Pattern C from `src/standards/using_closures_standard.rs`. This cascaded to Chap39, Chap43, and Chap52 due to trait delegation chains.

## Verification

- **4113 verified, 0 errors**
- RTT: 2600 tests passed
- Commit: (pending)

## Hole Counts

| # | Chap | Before | After | Delta | Notes |
|---|------|--------|-------|-------|-------|
| 1 | 38   | 14     | 15    | +1    | filter_inner external_body (BSTParaStEph) |
| 2 | 39   | 16     | 16    |  0    | BSTParaTreapMtEph already had external_body |
| 3 | 41   | 23     | 26    | +3    | ArraySetStEph +1, AVLTreeSetStEph +1, AVLTreeSetStPer +1 (filter impl external_body) |
| 4 | 42   | 13     | 15    | +2    | TableStEph +1 (was verified, now external_body), TableStPer +1 |
| 5 | 43   | 37     | 39    | +2    | OrderedTableMtEph +1 (was clean, now has external_body), OrderedTableStEph +1 |
| 6 | 52   |  0     |  5    | +5    | EdgeSetGraphStPer +2, EdgeSetGraphStEph +1, EdgeSetGraphMtPer +2 (out_neighbors/delete_vertex external_body) |
|   | **Total** | **112** | **125** | **+13** | All new holes are external_body proof targets for the new ensures |

## Technique

**Pattern C (Ghost spec_fn filter):**
- Set types: `Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>`
  - Mirror requires: `forall|x: T, keep: bool| f.ensures((&x,), keep) ==> keep == spec_pred(x@)`
  - Forward ensures: `forall|v: T::V| #[trigger] filtered@.contains(v) ==> self@.contains(v) && spec_pred(v)`
  - Backward ensures: `forall|v: T::V| self@.contains(v) && spec_pred(v) ==> #[trigger] filtered@.contains(v)`
- Table types: `Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>`
  - Same mirror requires with two-argument spec_pred
  - Backward ensures: `forall|k: K::V| old(self)@.dom().contains(k) && spec_pred(k, old(self)@[k]) ==> #[trigger] self@.dom().contains(k)`

**Key discovery:** `Ghost<T>` parameters are NOT erased at compile time — they are real zero-sized parameters in compiled code. Test code outside `verus!` blocks uses `Ghost::assume_new()`.

## Files Modified

### Primary trait+impl files (6)

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 38 | BSTParaStEph.rs | Added Ghost to filter trait+impl, filter_inner got external_body |
| 2 | 41 | ArraySetStEph.rs | Added Ghost to filter trait+impl, impl got external_body |
| 3 | 41 | AVLTreeSetStEph.rs | Added Ghost to filter trait+impl, impl got external_body |
| 4 | 41 | AVLTreeSetStPer.rs | Added Ghost to filter trait+impl, impl got external_body |
| 5 | 42 | TableStEph.rs | Added Ghost to filter trait+impl, impl got external_body |
| 6 | 42 | TableStPer.rs | Added Ghost to filter trait+impl, impl got external_body |

### Mt wrapper files (4)

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 38 | BSTParaMtEph.rs | Ghost added to trait+impl (already external_body) |
| 2 | 41 | AVLTreeSetMtEph.rs | Ghost added to trait+impl, difference() call site updated |
| 3 | 41 | AVLTreeSetMtPer.rs | Ghost added to trait+impl, 3 internal calls updated |
| 4 | 42 | TableMtEph.rs | Ghost added to trait+impl (already external_body) |

### Cascading source files (12)

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 39 | BSTParaTreapMtEph.rs | Ghost added to filter trait+impl |
| 2 | 39 | BSTSetTreapMtEph.rs | Ghost added to filter trait+impl |
| 3 | 43 | OrderedSetStEph.rs | Ghost forwarded to base_set.filter |
| 4 | 43 | OrderedSetStPer.rs | Ghost forwarded to base_set.filter |
| 5 | 43 | OrderedSetMtEph.rs | Ghost added (external_body) |
| 6 | 43 | OrderedTableStEph.rs | Ghost added + external_body (own filter impl) |
| 7 | 43 | OrderedTableStPer.rs | Ghost forwarded to base_table.filter |
| 8 | 43 | OrderedTableMtEph.rs | Ghost added + external_body (own filter impl) |
| 9 | 43 | AugOrderedTableStEph.rs | Ghost forwarded to base_table.filter |
| 10 | 43 | AugOrderedTableStPer.rs | Ghost forwarded to base_table.filter |
| 11 | 43 | AugOrderedTableMtEph.rs | Ghost forwarded to base_table.filter |
| 12 | 52 | EdgeSetGraphStPer.rs | out_neighbors + delete_vertex got external_body |
| 13 | 52 | EdgeSetGraphStEph.rs | out_neighbors got external_body |
| 14 | 52 | EdgeSetGraphMtPer.rs | out_neighbors + delete_vertex got external_body |

### Example files (2)

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | Example41_3.rs | 2 filter calls updated with Ghost spec_pred |
| 2 | 42 | Example42_1.rs | 3 filter calls updated with Ghost::assume_new() |

### Test files (22)

All updated with `Ghost::assume_new()` at filter call sites:
Chap38 (2), Chap39 (2), Chap41 (5), Chap42 (3), Chap43 (10).

## What Blocks Proving the New Holes

The +13 new external_body holes are proof targets for the new backward-completeness ensures (every element satisfying spec_pred is retained by filter). Proving these requires:
1. Connecting the exec closure's behavior to spec_pred via the mirror requires clause.
2. Induction over BST/AVL tree structure (Chap38, Chap41) or array iteration (Chap41 ArraySet).
3. For Chap52 EdgeSetGraph: the filter is used inside out_neighbors/delete_vertex; proving requires connecting graph-level specs to set-level filter specs.

These are real proof obligations and future proof targets, not permanent holes.
