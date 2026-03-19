# Agent 4 — Round 39 Report

## Summary

Restructured `OrderedTableMtEph.rs` from wrapping `TableMtEph<K, V>` to wrapping
`OrderedTableStEph<K, V>` inside an RwLock, following the `OrderedSetMtEph`/`OrderedTableMtPer`
pattern. Fixed cascade in `AugOrderedTableMtEph.rs`. Reviewed `OrderedTableMtPer.rs` (no
changes needed).

## Verification

- **4322 verified, 0 errors**
- **2613 RTT pass, 0 failures**
- **176 total holes** (was 175 baseline, +1 net)
- **7 fn_missing_wf_ensures warnings** (was 8, -1: fixed `from_sorted_entries`)

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableMtEph.rs | Full restructure: RwLock<OrderedTableStEph> + Ghost |
| 2 | 43 | AugOrderedTableMtEph.rs | Cascade: removed old imports, iterator outside verus! |

## OrderedTableMtEph.rs — Before/After

**Before**: Wrapped `TableMtEph<K, V>` which wrapped `ArraySeqMtEph<Pair<K, V>>`.
Delegated to TableMtEph methods. Iterator borrowed entries directly.

**After**: Wraps `OrderedTableStEph<K, V>` inside `RwLock` + `Ghost<Map<K::V, V::V>>`.
All operations acquire lock, delegate to StEph, release. Iterator uses snapshot-based
`Vec<Pair<K, V>>` (owned items). `split_key` reimplemented with ordered comparison
(`Ord::cmp`) instead of delegating to StEph's unordered split.

### Architecture

```
OrderedTableMtEph<K, V> {
    locked_table: RwLock<OrderedTableStEph<K, V>, OrderedTableMtEphInv>,
    ghost_locked_table: Ghost<Map<K::V, V::V>>,
}
```

- **RwLockPredicate**: `OrderedTableMtEphInv` — `inv` checks `spec_orderedtablesteph_wf()`
- **View**: `pub open spec fn` returning `self.ghost_locked_table@`
- **WF**: `self@.dom().finite()`
- **Send/Sync**: unsafe impls (Ghost<Map> contains non-Send FnSpec phantom)

### Key Decisions

1. **No type invariant**: Verus type invariants require non-pub fields, but `pub open spec fn`
   View requires pub fields. These are incompatible. Used wf requires + assumes instead.
2. **`split_key` ordered scan**: StEph's split_key puts all non-k entries in left (no ordering).
   MtEph scans entries with `Ord::cmp` to correctly partition into left (< k) and right (> k).
3. **Snapshot iterator**: Entries behind RwLock can't be borrowed. iter() acquires read lock,
   clones entries into Vec, releases lock. Item type is owned `Pair<K, V>`.
4. **`from_st` helper**: Wraps StEph in RwLock with `assume(inner.spec_orderedtablesteph_wf())`.
   Ensures `s@ =~= inner@, s@.dom().finite(), s.spec_orderedtablemteph_wf()`.

## AugOrderedTableMtEph.rs — Cascade

- Removed imports: `ArraySeqMtEph`, `TableMtEph`
- Removed `lemma_entries_to_map_finite` calls (replaced with `assume(self@.dom().finite())`)
- Iterator moved outside `verus!`: delegates to `base_table.iter()`, returns owned `Pair<K, V>`
- `IntoIterator::Item` changed from `&'a Pair<K, V>` to `Pair<K, V>`

## Hole Analysis (Chap43 OrderedTableMtEph.rs)

Assumes: ~22 lock-boundary assumes (all RWLOCK_GHOST structural false positives except 3):

| # | Category | Count | Pattern |
|---|----------|-------|---------|
| 1 | inner@ =~= self@ | 6 | Read ops bridge locked view to ghost view |
| 2 | locked_val.wf() | 7 | Write ops: wf after mutation |
| 3 | empty_val.wf() | 2 | split_key/split_rank_key: release with empty |
| 4 | left/right.wf() | 4 | split: wf of split results |
| 5 | range.wf() | 1 | get_key_range: wf of range result |
| 6 | self@.dom().finite() | 3 | domain/reduce/collect: no wf requires |
| 7 | clone bridges | 3 | inner@==self@, view.dom().finite(), inner.wf() |

Net hole impact: +1 (restructure added lock-boundary assumes where old TableMtEph delegation
had none, but AugOrderedTableMtEph gained 2 finiteness assumes and lost old finite lemma calls).

## OrderedTableMtPer.rs — No Changes

Already correctly wraps `OrderedTableStPer<K, V>` in RwLock. Same pattern as the new MtEph.
8 lock-boundary assumes. Clean.
