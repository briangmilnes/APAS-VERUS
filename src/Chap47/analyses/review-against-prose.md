# Chap47 Review Against Prose: Hash Tables

Generated: 2026-03-15

## Phase 1: Inventory

| # | Chap | File | Functions | Proof Fns | Holes |
|---|------|------|-----------|-----------|-------|
| 1 | 47 | ParaHashTableStEph.rs | 9 trait | 2 | 2 |
| 2 | 47 | ChainedHashTable.rs | 4 trait | 1 | 0 |
| 3 | 47 | FlatHashTable.rs | 4 trait | 0 | 0 |
| 4 | 47 | VecChainedHashTableStEph.rs | 8 IT | 1 | 4 |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 8 IT | 1 | 4 |
| 6 | 47 | StructChainedHashTable.rs | 11 IT | 1 | 4 |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 8 IT | 0 | 4 |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 8 IT | 0 | 4 |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 8 IT | 0 | 4 |

IT = in-trait functions. Total: 68 functions, 6 proof functions (all clean), 26 holes.

## Phase 2: Prose Inventory

APAS Chapter 47 defines:

**Definition 47.1 (Hash Table):**
- `createTable(m) : HashTable` -- create table with m buckets
- `insert(T, k, v)` -- insert key-value pair
- `lookup(T, k) : Option<v>` -- find value by key
- `loadAndSize(T) : (load, size)` -- load factor and table size
- `resize(T, m')` -- resize to m' buckets

**Nested Tables (Separate Chaining):**
- Outer array of inner tables, one per hash bucket.
- Insert: hash to bucket, insert into inner table.
- Lookup: hash to bucket, lookup in inner table.
- Delete: hash to bucket, delete from inner table.
- Cost: O(1 + alpha) expected per operation where alpha = n/m.

**Flat Tables (Open Addressing):**
- Single array with probe sequences for collision resolution.
- Entry states: Empty, Occupied(k,v), Deleted.
- Three probe strategies: linear, quadratic, double hashing.
- Lookup: probe until found or Empty.
- Insert: probe until Empty or Deleted slot.
- Delete: mark as Deleted (lazy deletion).
- Cost: O(1/(1 - alpha)) expected per operation.

**Resize:** Rehash all elements into new table when load factor exceeds threshold.

## Phase 3a: Cost Annotations

All nine files have proper two-line cost annotations (APAS + Claude-Opus-4.6).
No changes needed -- annotations were already in the correct format.

### Architecture

The implementation uses a trait hierarchy:

1. **ParaHashTableStEph** -- base trait defining HashTable struct, EntryTrait, and abstract
   insert/lookup/delete/resize methods.
2. **ChainedHashTable** -- extends base trait with chained-specific default implementations
   (insert_chained, lookup_chained, delete_chained) that delegate to EntryTrait operations.
3. **FlatHashTable** -- extends base trait with open-addressing defaults
   (insert_with_probe, lookup_with_probe) using a probe function.
4. **Six concrete implementations** -- three chaining variants (Vec, LinkedList, Struct)
   and three probing variants (linear, quadratic, double hash).

## Phase 3b: Implementation Fidelity

| # | Chap | File | Operation | APAS Algorithm | Implementation | Match? |
|---|------|------|-----------|----------------|----------------|--------|
| 1 | 47 | ParaHashTableStEph.rs | createTable | Allocate m buckets | Loop creating entries | Yes |
| 2 | 47 | ChainedHashTable.rs | insert | Hash + chain insert | hash_index + entry.insert | Yes |
| 3 | 47 | ChainedHashTable.rs | lookup | Hash + chain lookup | hash_index + entry.lookup | Yes |
| 4 | 47 | ChainedHashTable.rs | delete | Hash + chain delete | hash_index + entry.delete | Yes |
| 5 | 47 | FlatHashTable.rs | insert | Probe for slot | find_slot + entry.insert | Yes |
| 6 | 47 | FlatHashTable.rs | lookup | Probe sequence | Loop with probe(attempt) | Yes |
| 7 | 47 | FlatHashTable.rs | FlatEntry | Empty/Occupied/Deleted | FlatEntry enum | Yes |
| 8 | 47 | LinProbFlatHashTableStEph.rs | probe | (hash + attempt) % m | Same formula | Yes |
| 9 | 47 | QuadProbFlatHashTableStEph.rs | probe | (hash + attempt^2) % m | Same formula | Yes |
| 10 | 47 | DoubleHashFlatHashTableStEph.rs | probe | (h1 + attempt*h2) % m | Same formula | Yes |
| 11 | 47 | StructChainedHashTable.rs | chain ops | Linked list traversal | Recursive chain fns | Yes |

All implementations faithfully follow the APAS algorithms. The trait hierarchy cleanly
separates the abstract interface from concrete collision resolution strategies.

## Phase 3c: Spec Fidelity

| # | Chap | File | Function | APAS Spec | Verus Spec | Strength |
|---|------|------|----------|-----------|------------|----------|
| 1 | 47 | ParaHashTableStEph.rs | createTable | Empty table | map empty, proven | Strong |
| 2 | 47 | ParaHashTableStEph.rs | call_hash_fn | h(k) mod m | ensures index < size | Partial |
| 3 | 47 | ParaHashTableStEph.rs | compute_second_hash | h2(k) | ensures step >= 1 | Partial |
| 4 | 47 | ChainedHashTable.rs | insert_chained | Map insert | dom().contains(key) | Partial |
| 5 | 47 | ChainedHashTable.rs | lookup_chained | Map lookup | ensures true | Weak |
| 6 | 47 | ChainedHashTable.rs | delete_chained | Map delete | table len preserved | Weak |
| 7 | 47 | FlatHashTable.rs | insert_with_probe | Map insert | dom().contains(key) | Partial |
| 8 | 47 | FlatHashTable.rs | lookup_with_probe | Map lookup | ensures true | Weak |
| 9 | 47 | FlatHashTable.rs | EntryTrait::insert | Set entry | Occupied(k,v) | Strong |
| 10 | 47 | FlatHashTable.rs | EntryTrait::lookup | Get entry | partial (Empty/Deleted) | Partial |
| 11 | 47 | FlatHashTable.rs | EntryTrait::delete | Mark deleted | deleted => Deleted | Strong |
| 12 | 47 | All 6 impl files | insert | delegates | external_body | Hole |
| 13 | 47 | All 6 impl files | lookup | delegates | external_body | Hole |
| 14 | 47 | All 6 impl files | delete | delegates | external_body | Hole |
| 15 | 47 | All 6 impl files | resize | delegates | external_body | Hole |

**Spec Architecture:** The abstract spec uses `Map<Key, Value>` view type with
`spec_table_to_map` and `spec_entry_to_map` connecting entries to the logical map.
A `lemma_table_to_map_update_contains` is proven, enabling insert to establish
`table@.dom().contains(key)`. However, lookup and delete specs are weak (`ensures true`
or just length preservation).

**Root cause of weak specs:** The ghost hash function problem. `call_hash_fn` is
`external_body` because Verus cannot reason about opaque `Fn` closures. Without proving
that `hash(k)` is deterministic (same key always hashes to same bucket), lookup cannot
prove it finds the correct entry. This is a fundamental Verus limitation for closures.

## Phase 4: Parallelism Review

No Mt (multi-threaded) modules exist for Chapter 47. APAS does not present parallel hash
table implementations in this chapter.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Present |
|---|------|------|-----------|---------|
| 1 | 47 | ParaHashTableStEph.rs | TestParaHashTableStEph.rs | Yes |
| 2 | 47 | VecChainedHashTableStEph.rs | TestVecChainedHashTable.rs | Yes |
| 3 | 47 | LinkedListChainedHashTableStEph.rs | TestLinkedListChainedHashTable.rs | Yes |
| 4 | 47 | StructChainedHashTable.rs | TestStructChainedHashTable.rs | Yes |
| 5 | 47 | LinProbFlatHashTableStEph.rs | TestLinProbFlatHashTable.rs | Yes |
| 6 | 47 | QuadProbFlatHashTableStEph.rs | TestQuadProbFlatHashTable.rs | Yes |
| 7 | 47 | DoubleHashFlatHashTableStEph.rs | TestDoubleHashFlatHashTable.rs | Yes |

All seven testable files have dedicated test files. Tests cover:
- Entry creation, insert, update, delete
- Hash table operations (insert, lookup, delete)
- Resize behavior (empty, with elements, smaller table)
- Probe sequences (linear, quadratic, double hash)
- Load factor and size tracking
- Delete nonexistent keys
- Clone and PartialEq operations

RTT coverage is thorough. No gaps identified.

## Phase 6: PTT Review

No PTTs exist for Chapter 47. None are needed -- there are no iterators and no complicated
callability patterns.

## Phase 7: Gap Analysis

### Proof Holes (26 total)

| # | Chap | File | Hole Type | Count | Functions | Blocker |
|---|------|------|-----------|-------|-----------|---------|
| 1 | 47 | ParaHashTableStEph.rs | external_body | 2 | call_hash_fn, compute_second_hash | Opaque Fn closures |
| 2 | 47 | VecChainedHashTableStEph.rs | external_body | 4 | insert/lookup/delete/resize | Needs call_hash_fn spec |
| 3 | 47 | LinkedListChainedHashTableStEph.rs | external_body | 4 | insert/lookup/delete/resize | Same |
| 4 | 47 | StructChainedHashTable.rs | external_body | 4 | insert/lookup/delete/resize | Same |
| 5 | 47 | LinProbFlatHashTableStEph.rs | external_body | 4 | insert/lookup/delete/resize | Same |
| 6 | 47 | QuadProbFlatHashTableStEph.rs | external_body | 4 | insert/lookup/delete/resize | Same |
| 7 | 47 | DoubleHashFlatHashTableStEph.rs | external_body | 4 | insert/lookup/delete/resize | Same |

**Pattern:** All 24 concrete implementation holes are `external_body` on the four trait
methods (insert, lookup, delete, resize). These delegate to the verified default methods
in ChainedHashTable/FlatHashTable traits but are wrapped in `external_body`.

**Root blocker:** The 2 `external_body` functions in ParaHashTableStEph.rs (`call_hash_fn`
and `compute_second_hash`) cannot be verified because Verus cannot reason about the spec
of opaque `Fn(&Key, usize) -> usize` closures. This cascades: without a verified hash
function, the concrete implementations cannot prove their insert/lookup/delete correct.

### Verified Infrastructure (positive)

Despite the holes, the following infrastructure is fully verified:
- `createTable` -- proven to produce an empty map
- `lemma_table_to_map_update_contains` -- key proof lemma
- `ChainedHashTable::insert_chained` -- proven insert preserves table structure
- `FlatHashTable::insert_with_probe` -- proven insert with probe finds slot
- `FlatHashTable::lookup_with_probe` -- verified loop structure
- `FlatEntry` -- all four EntryTrait methods fully proven
- `StructChainedHashTable` chain functions -- chain_insert/lookup/delete verified
- All Clone/PartialEq/Debug/Display implementations

### Proof Targets (priority order)

1. **Ghost hash approach**: Define a `spec fn spec_hash(key: Key) -> usize` in the trait
   and require implementations to prove `call_hash_fn(h, key, m) == spec_hash(key) % m`.
   This would unlock reasoning about hash determinism.
2. **Chaining implementations**: Once hash spec exists, remove external_body from insert
   (already delegates to verified insert_chained). lookup/delete follow similarly.
3. **Flat table lookup correctness**: Requires proving that probe sequence visits the
   correct slot when key exists. Needs hash determinism.
4. **Resize**: Most complex -- needs to prove all elements are rehashed correctly.

### Warnings

| # | Chap | File | Warning | Line |
|---|------|------|---------|------|
| 1 | 47 | ParaHashTableStEph.rs | requires_true | 327 (metrics) |
| 2 | 47 | ParaHashTableStEph.rs | requires_true | 336 (loadAndSize) |
| 3 | 47 | StructChainedHashTable.rs | requires_true | 108 (chain_insert) |
| 4 | 47 | StructChainedHashTable.rs | requires_true | 141 (chain_lookup) |
| 5 | 47 | StructChainedHashTable.rs | requires_true | 163 (chain_delete) |

These are vacuous precondition warnings. The functions require no preconditions but
have meaningful postconditions. These are low-priority style issues, not proof holes.

## Phase 8: TOC Review

| # | Chap | File | TOC Present | Sections Correct | Issues |
|---|------|------|-------------|------------------|--------|
| 1 | 47 | ParaHashTableStEph.rs | Yes | Yes | None |
| 2 | 47 | ChainedHashTable.rs | Yes | Yes | None |
| 3 | 47 | FlatHashTable.rs | Yes | Yes | None |
| 4 | 47 | VecChainedHashTableStEph.rs | Yes | Yes | None |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | Yes | Yes | None |
| 6 | 47 | StructChainedHashTable.rs | Yes | Yes | None |
| 7 | 47 | LinProbFlatHashTableStEph.rs | Yes | Yes | None |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | Yes | Yes | None |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | Yes | Yes | None |

All TOCs are correct and follow the standard ordering.

## Summary

| Metric | Value |
|--------|-------|
| Files reviewed | 9 |
| Total functions | 68 |
| Clean proof functions | 6 (100%) |
| Proof holes | 26 (all external_body) |
| Clean modules | 2 (ChainedHashTable, FlatHashTable) |
| Holed modules | 7 |
| Cost annotation coverage | 100% (all functions annotated) |
| RTT coverage | 100% (all files have tests) |
| PTT coverage | N/A (none needed) |
| APAS ADT coverage | 100% (all 5 operations) |
| Implementation fidelity | Excellent -- all algorithms match APAS |
| Root blocker | Opaque Fn closure verification (call_hash_fn) |

### Architecture Assessment

The trait hierarchy (ParaHashTableStEph -> ChainedHashTable/FlatHashTable -> 6 impls) is
a strong design that separates concerns well. The abstract map-based spec layer
(spec_table_to_map, spec_entry_to_map) provides the right abstraction. The verified
default implementations in ChainedHashTable and FlatHashTable are clean and could be
leveraged once the hash function specification issue is resolved.

The 26 holes are structurally homogeneous (24 are the same pattern repeated across 6 files)
and share a single root cause. Resolving the `call_hash_fn` specification would likely
enable removing 24 of the 26 holes.
