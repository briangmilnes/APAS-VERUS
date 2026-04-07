# R153 Agent 4 Report: OrderedTableStPer OrdKeyMap Migration

## Summary

Migrated `OrderedTableStPer` from wrapping `ParamBST<Pair<K,V>>` directly to
wrapping `OrdKeyMap<K,V>`. This is the StPer (persistent) variant of the
ordered table.

## Changes Made

### OrderedTableStPer.rs (src/Chap43/OrderedTableStPer.rs)

| # | Chap | Change | Detail |
|---|------|--------|--------|
| 1 | 43 | Struct field | `pub tree: ParamBST<Pair<K,V>>` → `pub tree: OrdKeyMap<K,V>` |
| 2 | 43 | View impl | `spec_pair_set_to_map(self.tree@)` → `self.tree@` (OrdKeyMap already views as Map) |
| 3 | 43 | wf predicate | 10-line conjunction → `self.tree.spec_ordkeymap_wf()` |
| 4 | 43 | size() | 8 lines → 3 lines (delegates to `self.tree.size()`) |
| 5 | 43 | All BST access | `self.tree.method()` → `self.tree.inner.method()` (~80 sites) |
| 6 | 43 | Struct constructors | `OrderedTableStPer { tree: bst }` → `OrderedTableStPer { tree: OrdKeyMap { inner: bst } }` (~17 sites) |
| 7 | 43 | rlimit annotation | Added `#[verifier::rlimit(20)]` on `bst_next_by_key` (borderline with new imports) |

### External Files

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | AugOrderedTableStPer.rs | `self.base_table.tree@` → `self.base_table.tree.inner@` (4 sites) |
| 2 | 43 | OrderedTableMtPer.rs | `inner.tree@`/`range.tree@` → `.tree.inner@` (2 sites) |

## Line Counts

| Metric | Before | After |
|--------|--------|-------|
| OrderedTableStPer.rs | 4,339 | 4,326 |
| Net change | — | -13 |

## Why Modest Reduction

The prompt expected 1,500-2,000 lines. The actual reduction is modest because:

1. **OrdKeyMap lacks most operations.** OrdKeyMap only provides: new, size,
   is_empty, find, insert, delete, split. OrderedTableStPer needs 30+ methods
   including union, intersection, difference, filter, map, tabulate, restrict,
   subtract, next_key, previous_key, first_key, last_key, rank_key, select_key,
   split_key, split_rank_key, get_key_range, collect, domain, join_key, plus
   all `_iter` variants.

2. **Methods without OrdKeyMap support use `self.tree.inner`.** These methods
   still need all the same BST access, bridge lemmas, and proof machinery — just
   accessed via `.inner` instead of directly.

3. **OrdKeyMap API doesn't expose wf in constructors.** `OrdKeyMap::new()` only
   ensures `empty@ == Map::empty()`, not `spec_ordkeymap_wf()`. So empty/singleton
   can't delegate and must construct the ParamBST directly, proving wf manually.

4. **Bridge lemmas still needed.** The 15 proof fns and 5 spec fns in Section 6-7
   are still called by the ~25 methods that access `self.tree.inner` directly.

## What DID Improve

1. **View is direct**: `self.tree@` gives `Map<K::V, V::V>` without conversion.
2. **wf is one line**: `self.tree.spec_ordkeymap_wf()` replaces 10-line conjunction.
3. **size delegates**: `self.tree.size()` replaces manual BST size + bridge proof.
4. **Foundation for future**: When OrdKeyMap gets union/next/prev/rank/select,
   those methods can trivially delegate, each saving 50-300 lines of proof.

## Verification

- **Full validate**: 5,742 verified, 0 errors
- **RTT**: 3,690 passed, 0 failed
- **No new holes, assumes, accepts, or external_body added**
- **No ensures weakened**
