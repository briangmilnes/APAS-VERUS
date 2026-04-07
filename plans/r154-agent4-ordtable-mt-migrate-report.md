# R154 Agent 4 — OrderedTable Mt Migration to OrdKeyMap Report

## Summary

The Mt variants (OrderedTableMtEph, OrderedTableMtPer) wrap the St layer via RwLock
and do NOT directly hold ParamBST fields. The R153 migration of StEph/StPer to
OrdKeyMap was transparent to the Mt layer — the Mt files already work correctly.

## Analysis

### OrderedTableMtEph.rs

- Wraps `OrderedTableStEph<K,V>` via `RwLock`. No ParamBST in struct definition.
- Three `.tree.inner` references found:
  - **Line 791**: `inner.tree.inner.in_order()` — OrdKeyMap has no `in_order()`, must stay.
  - **Line 1048**: `self_read.borrow().tree.inner.size()` — simplified to `.tree.size()` using OrdKeyMap's `size()` method.
  - Added `OrdKeyMapTrait` import for the size() call.
- `ParamBSTTrait` import retained — still needed for `in_order()` on line 791.

### OrderedTableMtPer.rs

- Wraps `OrderedTableStPer<K,V>` via `RwLock`. No ParamBST in struct definition.
- Two `.tree.inner` references, both in proof blocks calling `lemma_pair_set_to_map_dom_finite`:
  - **Line 76**: `lemma_pair_set_to_map_dom_finite(inner.tree.inner@)` — needs `Set<(KV,VV)>` (ParamBST view), not `Map` (OrdKeyMap view). Must stay.
  - **Line 648**: Same pattern. Must stay.
- No changes needed.

### AugOrderedTableMtEph.rs

- No `.tree.inner` references. Already clean.

### AugOrderedTableStEph.rs / AugOrderedTableStPer.rs

- Multiple `.base_table.tree.inner@` references for `lemma_pair_set_to_map_dom_finite` and iterator ensures `.len()`.
- All need the `Set` view (ParamBST@), not the `Map` view (OrdKeyMap@). Must stay.
- Not in scope for this task (prompt says do not modify).

## Changes Made

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableMtEph.rs | Added `use OrdKeyMapTrait` import |
| 2 | 43 | OrderedTableMtEph.rs | `.tree.inner.size()` → `.tree.size()` in PartialEq impl |

## Why `.tree.inner` References Remain

OrdKeyMap does not expose:
1. `in_order()` — needed for iterator snapshot creation in MtEph
2. Raw `Set<(KV,VV)>` view — `OrdKeyMap@` returns `Map<KV,VV>`, but `lemma_pair_set_to_map_dom_finite` needs the underlying `Set`
3. Set `.len()` — used in iterator ensures to bound iterator length

These are legitimate internal accesses that would require expanding OrdKeyMap's API to eliminate.

## Validation

- `scripts/validate.sh isolate Chap43`: 2813 verified, 0 errors
- `scripts/validate.sh` (full): 5757 verified, 0 errors
- `scripts/rtt.sh`: 3717 passed, 0 skipped
