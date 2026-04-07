# R153 Agent 2 Report: OrdKeyMap next/prev/rank/select

## Summary

Added 4 ordering operations to OrdKeyMap (Chap38): `next_key`, `prev_key`,
`rank_key`, `select_key`. All fully verified with 0 holes. All proofs adapted
from the reference implementations in OrderedTableStEph (Chap43).

## What was added

| # | Chap | File | Function | Lines | Source |
|---|------|------|----------|-------|--------|
| 1 | 38 | OrdKeyMap.rs | `ordkeymap_next` | ~280 | adapted from `bst_next_by_key` (Chap43) |
| 2 | 38 | OrdKeyMap.rs | `ordkeymap_prev` | ~280 | adapted from `bst_prev_by_key` (Chap43) |
| 3 | 38 | OrdKeyMap.rs | `ordkeymap_rank` | ~210 | adapted from `bst_rank_by_key` (Chap43) |
| 4 | 38 | OrdKeyMap.rs | `ordkeymap_select` | ~260 | adapted from `bst_select_by_rank` (Chap43) |

Trait methods added to `OrdKeyMapTrait`: `next_key`, `prev_key`, `rank_key`, `select_key`.
Each delegates to the corresponding free function.

## File changes

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 38 | OrdKeyMap.rs | 1467 | 2830 | +1363 |
| 2 | - | Cargo.toml | - | +4 | +4 (test entry) |
| 3 | 38 | tests/Chap38/TestOrdKeyMap.rs | 0 | 230 | +230 (new) |

## Verification

- Isolate Chap38: 1204 verified, 0 errors (25s)
- Full validation: 5750 verified, 0 errors (137s)
- RTT: 3717 passed (was 3690, +27 new OrdKeyMap tests)

## Technique

All four proofs were adapted from the working, verified proofs in
`src/Chap43/OrderedTableStEph.rs`. The adaptation was mechanical:
- Changed `bst_next_by_key` → `ordkeymap_next` (same pattern for all four)
- Used OrdKeyMap's bridge lemmas (same names, already present from R152)
- Added `K: TotalOrder` bound on trait methods via `where` clause
- Returns `Option<K>` matching OrderedTableStEph's API pattern

All proofs verified on the first attempt with no modifications needed.

## API design notes

Return types match OrderedTableStEph's pattern:
- `next_key`, `prev_key`, `select_key` → `Option<K>` (key only)
- `rank_key` → `usize`

Callers who need the value can use `find()` after getting the key. This avoids
cloning the value in the BST descent and keeps the ensures clauses clean.

## New RTT coverage

27 new tests covering:
- Basic operations (new, insert, find, delete, split, overwrite)
- next_key: basic, last element, before all, between keys, after all, empty
- prev_key: basic, first element, after all, between keys, before all, empty
- rank_key: basic, missing keys, empty
- select_key: basic, out of range, empty
- Round-trip: rank(select(i)) == i
- Forward/backward walk via next/prev
