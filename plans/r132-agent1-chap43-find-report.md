# R132 Agent 1 — Strengthen OrderedTableMtEph::find ensures

## Summary

Strengthened `find` and `lookup` ensures in `src/Chap43/OrderedTableMtEph.rs` to include
value correspondence (`v@ == self@[k@]`), matching the StEph version.

## Changes

| # | Chap | File | Method | Change |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableMtEph.rs | `find` (trait, L130) | Added `&& v@ == self@[k@]` to Some arm |
| 2 | 43 | OrderedTableMtEph.rs | `lookup` (trait, L141) | Added `&& v@ == self@[k@]` to Some arm |

## Why it works

The impl body at line 413-421:
1. `inner.find(k)` — StEph find ensures `v@ == inner@[k@]`
2. `assume(inner@ =~= self@)` — existing RWLOCK_GHOST bridge (extensional map equality)
3. From `inner@ =~= self@`, Verus derives `inner@[k@] == self@[k@]`
4. Therefore `v@ == self@[k@]` — no new assumes needed.

The `lookup` impl delegates to `find`, so it inherits the stronger ensures automatically.

## Validation

- `scripts/validate.sh isolate Chap43`: 2622 verified, 0 errors
- `scripts/rtt.sh`: 3529 passed, 0 skipped
- No new proof holes introduced
- No new assumes or external_body added

## Holes (unchanged)

Chap43 OrderedTableMtEph.rs holes are all pre-existing RWLOCK_GHOST structural false
positives — standard lock-boundary assumes. No change in hole count.
