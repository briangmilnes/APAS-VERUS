# Agent 3 — Round 23 Report

## Mission

Add ghost hash function infrastructure to Chap47 hash tables. Thread a
`Ghost<spec_fn(Key) -> nat>` field through the `HashTable` struct, `call_hash_fn`,
all probing functions, `createTable`, and all 6 strategy files + 7 test files.
Strengthen `spec_hashtable_wf` with a hash bucket invariant.

## Summary

R23 was a **cascading API change** across 16 files (9 src + 7 tests). Every
`HashTable` now carries a ghost spec-level hash function enabling future proof
work. The `spec_hashtable_wf` predicate now includes a hash bucket invariant:
keys can only appear in their correct bucket. The `call_hash_fn` wrapper now
has a strong ensures connecting the exec hash to the ghost hash. This is
infrastructure — no holes were closed, but the specs are now strong enough
for future rounds to prove insert/delete/lookup without external_body.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | Added `spec_hash: Ghost<spec_fn(Key) -> nat>` field to HashTable, `#[verifier::reject_recursive_types(Key)]`, strengthened `spec_hashtable_wf` with `current_size > 0` and hash bucket invariant, added `spec_hash` param to `call_hash_fn`/`linear_probe`/`quadratic_probe`/`double_hash_probe`/`createTable`, strengthened trait ensures for insert/delete/resize |
| 2 | 47 | ChainedHashTable.rs | Updated `hash_index`/`insert_chained`/`lookup_chained`/`delete_chained` to use `spec_hashtable_wf` |
| 3 | 47 | VecChainedHashTableStEph.rs | Threaded `table.spec_hash` through all `call_hash_fn` calls and `resize` struct literal |
| 4 | 47 | LinkedListChainedHashTableStEph.rs | Same threading as Vec variant |
| 5 | 47 | StructChainedHashTable.rs | Same threading as Vec variant |
| 6 | 47 | LinProbFlatHashTableStEph.rs | Threaded `table.spec_hash` through all `linear_probe`/`call_hash_fn` calls and `resize` struct literal |
| 7 | 47 | QuadProbFlatHashTableStEph.rs | Threaded `table.spec_hash` through all `quadratic_probe`/`call_hash_fn` calls and `resize` struct literal |
| 8 | 47 | DoubleHashFlatHashTableStEph.rs | Threaded `table.spec_hash` through all `double_hash_probe`/`call_hash_fn` calls and `resize` struct literal |
| 9-15 | 47 | Test*.rs (7 files) | Added `use vstd::prelude::Ghost;` import, `Ghost::assume_new()` third arg to all `createTable` calls, `spec_hash: Ghost::assume_new()` to struct literals |

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 |
| 2 | 47 | ChainedHashTable.rs | 0 | 0 | 0 |
| 3 | 47 | FlatHashTable.rs | 0 | 0 | 0 |
| 4 | 47 | VecChainedHashTableStEph.rs | 4 | 4 | 0 |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 4 | 4 | 0 |
| 6 | 47 | StructChainedHashTable.rs | 4 | 4 | 0 |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 | 0 |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 | 0 |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 | 0 |
| | | **Total** | **26** | **26** | **0** |

## Why No Holes Closed

The 24 strategy-file holes are all `external_body` on `insert`/`lookup`/`delete`/`resize`
implementations. Removing `external_body` requires proving the body satisfies the ensures.
The fundamental blocker is the **PartialEq spec bridge**: in generic code `if key1 == key2`,
Verus cannot connect exec equality (`PartialEq::eq`) to spec equality without
`Key: PartialEqSpecImpl` — which isn't available in these generic trait impls. Without
this bridge, we cannot prove that a linear scan for a matching key actually finds the
spec-level entry. This is a Verus type system limitation for generic hash table code.

The infrastructure built in R23 (ghost hash function, strengthened wf, hash bucket
invariant) is a necessary precondition for future proof work once the PartialEq bridge
is resolved.

## Technical Decisions

1. **`Ghost<spec_fn(Key) -> nat>` over `pub ghost`**: The `pub ghost` approach
   (`pub ghost spec_hash: spec_fn(Key) -> nat`) fails because ghost fields cannot be
   set in exec struct literals. The `Ghost<T>` wrapper makes it an exec-level ZST field.
2. **`spec_fn` over `FnSpec`**: `FnSpec` is deprecated in current Verus; `spec_fn` is
   the replacement.
3. **`#[verifier::reject_recursive_types(Key)]`**: Required because `spec_fn(Key) -> nat`
   uses `Key` in a non-positive position.

## Verification

- `scripts/validate.sh`: 3976 verified, 0 errors
- `scripts/rtt.sh`: 2600 tests passed

## Remaining Holes (26)

All 26 holes are `external_body` on `ParaHashTableStEphTrait` implementations:
- `call_hash_fn` and `compute_second_hash` in ParaHashTableStEph.rs (2)
- `insert`, `lookup`, `delete`, `resize` in each of 6 strategy files (24)

## What Blocks Closing Them

1. **PartialEq spec bridge** — generic `Key: PartialEq` exec `==` not connected to spec `==`
2. **`call_hash_fn` external_body** — inherently external (wraps opaque `Fn` closure)
3. **`compute_second_hash` external_body** — uses `std::hash::Hash` (not verifiable)
