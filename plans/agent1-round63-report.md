# Agent 1 — Round 63 Report

## Baseline
- 4504 verified, 0 errors, 6 holes, 2610 RTT, 147 PTT.

## After
- 4505 verified, 0 errors, 5 holes, 2610 RTT, 147 PTT.

## Summary

Closed the `call_hash_fn` `external_body` hole in Chap47/ParaHashTableStEph.rs.
Previously classified as structural ("Verus cannot reason about opaque Fn closures"),
but closable via `f.requires`/`f.ensures` quantifiers from the closures standard.

Key insight: quantify `spec_hash_fn_valid` over ALL values of type H (not a specific
instance) so the property survives `Clone::clone` (which has `ensures true` for
generic types). This makes the hash-function bridge a type-level property.

## Technique: Type-Level Closure Spec Bridge

```rust
pub open spec fn spec_hash_fn_valid<Key, H: Fn(&Key, usize) -> usize>(
    spec_hash: spec_fn(Key) -> nat,
) -> bool {
    (forall|h: H, k: &Key, ts: usize| ts > 0 ==> #[trigger] h.requires((k, ts)))
    && (forall|h: H, k: &Key, ts: usize, idx: usize|
        ts > 0 && #[trigger] h.ensures((k, ts), idx)
            ==> idx < ts && idx as nat == spec_hash(*k) % (ts as nat))
}
```

Quantifying `forall|h: H, ...|` instead of using a specific `hash_fn` instance means:
- The property is about the TYPE H, not any particular value.
- Clone operations don't invalidate it (cloned value is still type H).
- All call sites that have `spec_hash_fn_valid::<Key, H>(spec_hash@)` in scope can
  instantiate `h` with any local `&H` reference.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | Added `spec_hash_fn_valid` spec fn |
| 2 | 47 | ParaHashTableStEph.rs | Updated `spec_hashtable_wf` to include `spec_hash_fn_valid` |
| 3 | 47 | ParaHashTableStEph.rs | Removed `external_body` from `call_hash_fn`, added closure proof body |
| 4 | 47 | ParaHashTableStEph.rs | Added `spec_hash_fn_valid` to `createTable` requires |
| 5 | 47 | FlatHashTable.rs | Added `spec_hash_fn_valid` to `probe` and `find_slot` requires |
| 6 | 47 | LinProbFlatHashTableStEph.rs | `spec_impl_wf` override + 3 loop invariants |
| 7 | 47 | QuadProbFlatHashTableStEph.rs | `spec_impl_wf` override + 3 loop invariants |
| 8 | 47 | DoubleHashFlatHashTableStEph.rs | `spec_impl_wf` override + 3 loop invariants |

No changes needed for chained hash tables (ChainedHashTable, VecChainedHashTableStEph,
LinkedListChainedHashTableStEph, StructChainedHashTable) — they pick up
`spec_hash_fn_valid` automatically through `spec_hashtable_wf`.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 47 | ParaHashTableStEph.rs | 1 (external_body) | 0 | -1 |
| 2 | 47 | StructChainedHashTable.rs | 1 (assume) | 1 (assume) | 0 |
| 3 | 47 | All other files | 0 | 0 | 0 |

Global: 6 holes -> 5 holes (-1).

## Remaining Holes

| # | Chap | File | Hole | What Blocks It |
|---|------|------|------|----------------|
| 1 | 43 | OrderedSetMtEph.rs | 1 assume | Spec gap (sortedness not in wf) |
| 2 | 45 | BinaryHeapPQStEph.rs | 1 assume | Heap property infrastructure |
| 3 | 47 | StructChainedHashTable.rs | 1 assume | EntryTrait::lookup has no requires for feq |
| 4 | 53 | LCSStEph.rs | 1 assume | DP correctness proof |
| 5 | 53 | EditDistanceStEph.rs | 1 assume | DP correctness proof |

## Verification

```
verification results:: 4505 verified, 0 errors
RTT: 2610 passed, 0 skipped
PTT: 147 passed, 0 skipped
Holes: 5 (down from 6)
```
