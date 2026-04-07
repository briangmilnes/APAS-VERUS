# R153 Agent 3 — OrderedTableStEph Migration to OrdKeyMap

## Summary

Migrated `OrderedTableStEph` from wrapping `ParamBST<Pair<K,V>>` directly to
wrapping `OrdKeyMap<K,V>`. This establishes the bridge layer from the design doc
(`docs/ordered-bst-refactor.md`).

## What changed

### Structural migration (OrderedTableStEph.rs)

1. **Struct field**: `pub tree: ParamBST<Pair<K,V>>` → `pub tree: OrdKeyMap<K,V>`
2. **Import**: Added `use crate::Chap38::OrdKeyMap::OrdKeyMap::{OrdKeyMap, OrdKeyMapTrait};`
3. **View**: `spec_pair_set_to_map(self.tree@)` → `self.tree@` (OrdKeyMap already views as Map)
4. **wf**: Expanded form using `self.tree.inner` (keeps local spec fns for proof compatibility)
5. **All BST accesses**: `self.tree.xxx` → `self.tree.inner.xxx` (147 occurrences)
6. **All constructors**: `OrderedTableStEph { tree: bst }` → `OrderedTableStEph { tree: OrdKeyMap { inner: bst } }` (13 occurrences)

### Delegation simplifications

| # | Method | Before (lines) | After (lines) | Delegates to |
|---|--------|----------------|---------------|--------------|
| 1 | `size` | 7 | 1 | `OrdKeyMap::size` |
| 2 | `is_empty` | 7 | 1 | `OrdKeyMap::is_empty` |
| 3 | `empty` | 5 | 1 | `OrdKeyMap::new` |
| 4 | `find` | 1 | 1 | `OrdKeyMap::find` |
| 5 | `insert` | 65 | 9 | `OrdKeyMap::find` + `OrdKeyMap::insert` |
| 6 | `delete` | 30 | 4 | `OrdKeyMap::find` + `OrdKeyMap::delete` |

### Methods NOT yet delegated (still use self.tree.inner)

These methods access the underlying ParamBST directly because OrdKeyMap
doesn't yet have equivalents:

- `next_key`, `previous_key`, `first_key`, `last_key`
- `rank_key`, `select_key`, `split_rank_key`
- `union`, `intersection`, `difference`
- `domain`, `collect`, `tabulate`, `map`, `filter`, `reduce`
- `restrict`, `subtract`, `join_key`, `get_key_range`
- `split_key` (OrdKeyMap::split lacks disjointness guarantee)
- All `_iter` variants of the above

### Downstream fixes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | AugOrderedTableStEph.rs | `self.base_table.tree@` → `self.base_table.tree.inner@` (5 occurrences) |
| 2 | 43 | OrderedTableMtEph.rs | `inner.tree.in_order()` → `inner.tree.inner.in_order()`, `.tree.size()` → `.tree.inner.size()` |

### rlimit adjustment

`bst_next_by_key` was marginally passing before; the additional module import
pushed it over. Added `#[verifier::rlimit(20)]` (from default ~10).

## Line count

| | Before | After | Delta |
|--|--------|-------|-------|
| OrderedTableStEph.rs | 5,569 | 5,466 | -103 |

The -103 is from delegation simplifications. The mechanical migration
(`.inner` everywhere) is line-neutral.

## Verification

- **Full validate**: 5,742 verified, 0 errors
- **RTT**: 3,690 passed, 0 skipped
- **PTT**: skipped per prompt rules

## What blocks further simplification

1. **OrdKeyMap::split lacks map-level disjointness**: The BST split guarantees
   set disjointness but OrdKeyMap doesn't expose it in Map terms. `split_key`
   can't delegate without this.

2. **OrdKeyMap lacks ordering operations**: next, prev, first, last, rank,
   select need BST traversal with TotalOrder reasoning. Agent 1/2 are building
   these.

3. **OrdKeyMap lacks bulk operations**: union, intersection, difference,
   domain, collect, tabulate, map, filter, reduce need BST-level iteration.

4. **Bridge lemmas still needed**: The 17 bridge lemmas remain because the
   non-delegated methods still use them. They can be removed once all methods
   delegate to OrdKeyMap.

## Next steps

- Agent 1/2 add ordering + bulk operations to OrdKeyMap
- Migrate remaining methods as OrdKeyMap gains them
- Once all methods delegate, remove bridge lemmas (~500 lines)
- Remove `bst_find_by_key` once `get_key_range_iter` uses OrdKeyMap
- Repeat for OrderedTableStPer and OrderedTableMtEph
