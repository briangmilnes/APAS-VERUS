<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 47: Hash Tables — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 47.1 (Hash Tables) | ADT with createTable, insert, lookup, loadAndSize, resize on key-value pairs with hashable keys and an equality function |
| 2 | Definition 47.2 (Load Factor) | α = n/m where n = stored pairs, m = table size |
| 3 | Definition 47.3 (Separate Chaining) | Nested table using array of lists for collision resolution; insert at head is O(1), lookup/delete O(1+α) |
| 4 | Definition 47.5 (Probe Sequence) | Permutation of {0, 1, ..., m−1} for flat tables |
| 5 | Data Structure 47.6 (Parametric Flat Hash Tables) | Entry = Empty \| Dead \| Live(key, value); open addressing with parametric probe functions h₀(x), h₁(x), ..., h_{m−1}(x) |
| 6 | Definition 47.7 (Linear Probing) | hᵢ(k) = (h(k) + i) mod m |
| 7 | Definition 47.8 (Quadratic Probing) | hᵢ(k) = (h(k) + i²) mod m |
| 8 | Definition 47.9 (Secondary Clustering) | Two keys hashing to same location share probe sequence (affects linear and quadratic probing) |
| 9 | Definition 47.10 (Double Hashing) | hᵢ(k) = (h(k) + i·hh(k)) mod m; eliminates secondary clustering |

### Algorithms (Pseudocode)

| # | Item | Description |
|---|------|-------------|
| 1 | lookup (flat, DS 47.6) | Probe sequence: Empty→None, Dead→continue, Live(k',v')→match returns Some(v'), else continue |
| 2 | insert (flat, DS 47.6) | Probe sequence: Empty/Dead→place Live(k,v), Live(k',v') where k=k'→noop, else continue |
| 3 | delete (flat, DS 47.6) | Probe sequence: Empty→noop, Dead→continue, Live(k',v') where k=k'→mark Dead, else continue |

### Cost Specs

| # | Operation | Cost |
|---|-----------|------|
| 1 | Chained insert (head) | O(1) — "inserting the key-value pair at the head of the list" |
| 2 | Chained lookup/delete | O(1+α) expected — traverse chain |
| 3 | Flat insert / unsuccessful lookup | O(1/(1−α)) expected |
| 4 | Flat successful lookup | O((1/α)·ln(1/(1−α))) expected (averaged over all keys) |
| 5 | createTable | O(m) — allocate m entries |
| 6 | resize | O(n + m + m') — collect n pairs, deallocate m, allocate m', reinsert n |

### Theorems/Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Lemma 47.1 | If m is prime and table ≥ half empty, quadratic probing finds an empty cell; the first ⌈m/2⌉ probes are distinct |
| 2 | Load factor bound | Expected inner table size O(1+α); keep α constant by resizing when n exceeds cm |
| 3 | Double hashing | hh(k) must be coprime to m and non-zero; eliminates secondary clustering |

### Exercises

| # | Item | Status in Code |
|---|------|----------------|
| 1 | Exercise 47.1 — Describe nested table implementation using Table ADT | Implemented via trait hierarchy (ParaHashTableStEphTrait + ChainedHashTable + EntryTrait) |
| 2 | Exercise 47.2 — Discuss reducing hash table size | Not implemented (resize only doubles; no auto-shrink) |
| 3 | Exercise 47.3 — Implement resize operation and bound cost | Implemented in all 6 concrete types; cost bound not formally verified |
| 4 | Exercise 47.6 — Single higher-order function for flat table ops | Not implemented; separate insert/lookup/delete functions used |
| 5 | Exercise 47.7 — Complete parametric flat hash table (remaining ops) | Implemented: resize, loadAndSize, delete all present |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions carry APAS/Claude-Opus-4.6 cost annotation pairs in doc comments. No significant cost disagreements. Minor note: `second_hash` in DoubleHashFlatHashTableStEph iterates over key bytes (O(sizeof(Key))), but APAS treats hashing as O(1) — this is the standard assumption.

### 3b. Implementation Fidelity

| # | Prose Item | Code Implementation | Fidelity | Notes |
|---|-----------|---------------------|----------|-------|
| 1 | Def 47.1 createTable | `ParaHashTableStEphTrait::createTable` | Faithful | Hash function generator pattern matches APAS parametric design |
| 2 | Def 47.1 insert | Multiple implementations | Faithful | Each probing/chaining strategy implements correctly |
| 3 | Def 47.1 lookup | Multiple implementations | Faithful | Correctly stops at Empty, skips Dead for flat tables |
| 4 | Def 47.1 loadAndSize | `ParaHashTableStEphTrait::loadAndSize` | Partial | Returns load factor and size; chained implementations never update `num_elements` (see Deviation #4) |
| 5 | Def 47.1 resize | Multiple implementations | Faithful | Collects pairs, creates new table, reinserts |
| 6 | Def 47.3 Separate Chaining | Vec/LinkedList/StructChained | Faithful | Three chain representations exercise the parametric inner table design |
| 7 | DS 47.6 Entry type | `FlatEntry::Empty/Occupied/Deleted` | Faithful | Maps to Empty/Live/Dead from prose |
| 8 | DS 47.6 lookup (flat) | LinProb/QuadProb/DoubleHash lookup | Faithful | Matches pseudocode: probe sequence with Empty/Dead/Live handling |
| 9 | DS 47.6 insert (flat) | LinProb/QuadProb/DoubleHash insert | Faithful | Matches pseudocode; also handles update-on-duplicate (not in prose) |
| 10 | DS 47.6 delete (flat) | LinProb/QuadProb/DoubleHash delete | Faithful | Marks as Deleted (tombstone) per prose |
| 11 | Def 47.7 Linear Probing | `LinProbFlatHashTableStEph::probe` | Faithful | `(hash_val + attempt) % table.current_size` matches hᵢ(k) = (h(k) + i) mod m |
| 12 | Def 47.8 Quadratic Probing | `QuadProbFlatHashTableStEph::probe` | Faithful | `(hash_val + attempt*attempt) % table.current_size` matches hᵢ(k) = (h(k) + i²) mod m |
| 13 | Def 47.10 Double Hashing | `DoubleHashFlatHashTableStEph::probe` | Faithful | `(hash1 + attempt*step) % table.current_size` matches hᵢ(k) = (h(k) + i·hh(k)) mod m |
| 14 | Lemma 47.1 (⌈m/2⌉ bound) | QuadProb `max_attempts = div_ceil(2)` | Faithful | Lookup, delete, and find_slot all cap at ⌈m/2⌉ attempts per lemma |
| 15 | Double hashing coprimality | `second_hash` returns odd step for power-of-2 sizes, 1+hash%(m−1) for prime sizes | Faithful | Ensures hh(k) ≠ 0 and coprime to m |

### 3c. Deviations

| # | Deviation | Severity | Notes |
|---|-----------|----------|-------|
| 1 | `hash_index` placeholder in all 3 chained implementations always returns 0 | **High** | Vec, LinkedList, and Struct chained implementations have `let hash_val = 0; hash_val % table.current_size` — all keys hash to bucket 0. The `hash_fn` stored in `HashTable` is properly used by flat table implementations via `(table.hash_fn)(key)` but is never called by `hash_index`. All chained tables degenerate to a single chain. |
| 2 | Chained implementations never update `num_elements` | **High** | `insert_chained` (in `ChainedHashTable` trait) delegates to `table.table[index].insert(key, value)` but never increments `table.num_elements`. Similarly, `delete_chained` never decrements it. This means `loadAndSize` always reports load factor 0.0 for chained tables. Flat table implementations correctly update `num_elements`. |
| 3 | Chained insert scans for duplicate before inserting (O(1+α)) instead of O(1) head insert | **Low** | APAS says "inserting the key-value pair at the head of the list" requires O(1). All implementations scan for existing key first, providing update-on-duplicate semantics. This is a practical enhancement but changes the cost from O(1) to O(1+α). |
| 4 | No automatic resize on high load factor | **Medium** | APAS discusses resize triggers when α exceeds a threshold (doubling to keep α constant). Implementations provide `resize` but never auto-trigger it. |
| 5 | `FlatHashTable::lookup_with_probe` default method doesn't distinguish Empty from Deleted | **Low** | The default `lookup_with_probe` uses `FlatEntry::lookup` which returns None for both Empty and Deleted, so it always probes all m slots for missing keys (O(m) instead of O(1/(1−α))). Mitigated: all concrete flat implementations override `lookup` with correct Empty/Deleted handling. |
| 6 | LinkedList insert appends at tail (`push_back`) instead of head | **Low** | APAS says "inserting the key-value pair at the head of the list." `LinkedListChainedHashTableStEph::insert` uses `push_back` after scanning for duplicates. StructChained correctly inserts at head. Both are O(1+α) due to the duplicate scan, so the practical impact is nil. |

### 3d. Spec Fidelity

**No specs exist.** All functions are outside `verus!` blocks. Zero `requires`/`ensures` anywhere in the chapter. The entire chapter is unverified plain Rust.

The following prose properties have no formal spec:

| # | Property | Source |
|---|----------|--------|
| 1 | Lookup returns the correct value for a stored key | Def 47.1 |
| 2 | Insert preserves existing entries | Def 47.1 |
| 3 | Delete marks entries correctly (tombstone semantics for flat tables) | DS 47.6 |
| 4 | Resize preserves all key-value pairs (nothing less, nothing more) | Def 47.1 |
| 5 | Load factor computation α = n/m is correct | Def 47.2 |
| 6 | Probe sequences are permutations of {0, ..., m−1} | Def 47.5 |
| 7 | First ⌈m/2⌉ quadratic probes are distinct when m is prime | Lemma 47.1 |
| 8 | Double hash step is coprime to table size | Def 47.10 |

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 47.** All 9 source files are StEph (sequential ephemeral) or shared trait definitions.

APAS Section 1 notes that the outer table uses an array for O(1) access to inner tables, which naturally supports parallel resize (parallel map over inner tables). No parallel variants have been implemented.

## Phase 5: Runtime Test Review

### 5a. Test File Inventory

7 test files with 90 total tests cover all 9 source modules (some test files cover both trait and implementation).

| # | Test File | Tests | Covers |
|---|-----------|------:|--------|
| 1 | `TestParaHashTableStEph.rs` | 4 | createTable, loadAndSize, metrics (uses VecChained as concrete type) |
| 2 | `TestVecChainedHashTable.rs` | 9 | Vec EntryTrait CRUD, table insert/lookup/delete, resize, hash_index |
| 3 | `TestLinkedListChainedHashTable.rs` | 13 | LinkedList EntryTrait CRUD, table insert/lookup/delete, collision handling, resize, loadAndSize, update-existing |
| 4 | `TestStructChainedHashTable.rs` | 11 | ChainList CRUD, table insert/lookup/delete, resize, Default, Node Clone |
| 5 | `TestLinProbFlatHashTable.rs` | 26 | FlatEntry CRUD + Clone/Debug/PartialEq, probe, find_slot, insert/lookup/delete (including through-deleted, exhaustive, update, resize) |
| 6 | `TestQuadProbFlatHashTable.rs` | 12 | Quadratic probe sequence verification, Lemma 47.1 ⌈m/2⌉ bound, prime size guarantees, collision chains, resize |
| 7 | `TestDoubleHashFlatHashTable.rs` | 15 | second_hash properties (nonzero, odd for power-of-2), probe sequence, all-slot coverage for prime m, high load factor, resize |

### 5b. Coverage Analysis

| # | Source Module | Test File | Covered Operations | Status |
|---|--------------|-----------|-------------------|--------|
| 1 | ParaHashTableStEph.rs | TestParaHashTableStEph | createTable, loadAndSize, metrics | Covered (insert/lookup/delete tested via concrete impls) |
| 2 | ChainedHashTable.rs | (tested via Vec/LinkedList/Struct tests) | insert_chained, lookup_chained, delete_chained | Covered indirectly |
| 3 | FlatHashTable.rs | TestLinProbFlatHashTable | FlatEntry CRUD, insert_with_probe, lookup_with_probe | Covered |
| 4 | VecChainedHashTableStEph.rs | TestVecChainedHashTable | All 9 functions | Covered |
| 5 | LinkedListChainedHashTableStEph.rs | TestLinkedListChainedHashTable | All 9 functions | Covered |
| 6 | StructChainedHashTable.rs | TestStructChainedHashTable | All 10 functions | Covered |
| 7 | LinProbFlatHashTableStEph.rs | TestLinProbFlatHashTable | All 6 functions | **Well covered** (26 tests) |
| 8 | QuadProbFlatHashTableStEph.rs | TestQuadProbFlatHashTable | All 6 functions | **Well covered** (12 tests, includes Lemma 47.1) |
| 9 | DoubleHashFlatHashTableStEph.rs | TestDoubleHashFlatHashTable | All 7 functions | **Well covered** (15 tests, includes coprimality) |

### 5c. Test Quality Assessment

**Strengths:**

| # | Strength |
|---|----------|
| 1 | Flat table tests are thorough: insert, lookup, delete, update-existing, through-deleted probing, exhaustive probing, resize (empty, with elements, smaller), load/size tracking |
| 2 | Quadratic probing tests explicitly verify Lemma 47.1 (⌈m/2⌉ probe bound, prime size guarantees) |
| 3 | Double hashing tests verify second_hash properties (nonzero, odd for power-of-2, all-slot coverage for prime m) |
| 4 | Probe chain integrity tested: delete middle entry, verify later entries still findable |
| 5 | FlatEntry type tests (Clone, Debug, PartialEq) provide basic derive coverage |

**Weaknesses:**

| # | Weakness | Severity |
|---|----------|----------|
| 1 | `hash_index` placeholder bug not caught by tests. `test_vec_chained_hash_index` only asserts `index < 10` (always true since 0 < 10). No test verifies that different keys map to different buckets. | **High** |
| 2 | `num_elements` never-updated bug not caught for chained tables. `test_load_and_size` in TestLinkedListChainedHashTable only checks `result.size`, never `result.load`. | **High** |
| 3 | `test_collision_handling` (LinkedList) comments say keys distribute across 2 buckets, but all keys go to bucket 0 due to `hash_index` bug — test passes by accident. | **Medium** |
| 4 | No stress tests or randomized testing for any implementation. | **Low** |
| 5 | No test for resize shrinking in chained implementations (only tested for flat tables). | **Low** |

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed or present.** Chapter 47 has no `verus!` blocks, no iterators with ghost state, and no verified loops. No types implement `ForLoopGhostIterator` or `ForLoopGhostIteratorNew`.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Gap |
|---|-----------|-----|
| 1 | Exercise 47.2 (reducing hash table size) | Not implemented — no auto-shrink on low load factor |
| 2 | Exercise 47.6 (single higher-order function) | Not implemented — separate insert/lookup/delete methods |
| 3 | Lemma 47.1 formal proof | Applied correctly (⌈m/2⌉ bound) but not formally verified |
| 4 | Cost analysis (Section 2 expected-cost bounds) | Cost annotations in comments but no formal cost specs |
| 5 | Auto-resize on high load factor | APAS describes doubling when α exceeds threshold; not implemented |

### Code With No Prose Counterpart

| # | Item | Notes |
|---|------|-------|
| 1 | `Metrics` type parameter | Implementation scaffolding for instrumentation, not in prose |
| 2 | `second_hash` (FNV-1a) | APAS mentions hh(k) abstractly; implementation provides concrete FNV-1a hash |
| 3 | `insert_with_probe` / `lookup_with_probe` | Default trait method implementations in FlatHashTable; prose describes these inline |
| 4 | `ChainEntry` struct | Parametric container type wrapping inner table, not in prose |
| 5 | `LoadAndSize` struct | Return type for loadAndSize; prose returns a pair |
| 6 | `HashFunGen` / `HashFun` type aliases | Rust type machinery for hash function generator pattern |
| 7 | Update-on-duplicate semantics in insert | All implementations scan for existing key and update; prose assumes key not in table |

### Implementation Bugs

| # | Bug | Affected Files | Severity |
|---|-----|----------------|----------|
| 1 | `hash_index` always returns 0 | VecChainedHashTableStEph, LinkedListChainedHashTableStEph, StructChainedHashTable | **High** — all chained tables degenerate to single chain; should call `(table.hash_fn)(key)` |
| 2 | `num_elements` never updated in chained tables | All 3 chained implementations via `ChainedHashTable` trait | **High** — loadAndSize always reports 0 load for chained tables |
| 3 | LinkedList insert appends at tail not head | LinkedListChainedHashTableStEph | **Low** — uses `push_back` vs APAS "head of the list"; no correctness impact, minor efficiency difference |

## Phase 8: Table of Contents & In/Out Review

### TOC Headers

No TOC headers present in any file. This is expected since no files contain `verus!` blocks — the TOC standard applies to verusified files.

### In/Out Table

All code is outside `verus!`. No `verus!` blocks exist in Chapter 47.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | ParaHashTableStEph.rs | - | `✅ out` (LoadAndSize derive) | - | - | - | `✅ out` (LoadAndSize derive) | - | - | - |
| 2 | ChainedHashTable.rs | `✅ out` (ChainEntry derive) | `✅ out` (ChainEntry derive) | - | - | - | `✅ out` (ChainEntry derive) | - | - | - |
| 3 | FlatHashTable.rs | `✅ out` (FlatEntry derive) | `✅ out` (FlatEntry derive) | - | - | - | `✅ out` (FlatEntry derive) | - | - | - |
| 4 | VecChainedHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 5 | LinkedListChainedHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 6 | StructChainedHashTable.rs | `✅ out` (Node, ChainList derive) | `✅ out` (Node, ChainList derive) | `✅ out` (ChainList impl) | - | - | `✅ out` (Node, ChainList derive) | - | - | - |
| 7 | LinProbFlatHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 8 | QuadProbFlatHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 9 | DoubleHashFlatHashTableStEph.rs | - | - | - | - | - | - | - | - | - |

All derive impls are correctly outside `verus!` (since there are no `verus!` blocks). No `❌` items.

## Proof Holes

```
$ veracity-review-proof-holes -d src/Chap47/

✓ ChainedHashTable.rs
❌ DoubleHashFlatHashTableStEph.rs
  /home/milnes/projects/APAS-VERUS/src/Chap47/DoubleHashFlatHashTableStEph.rs:35: unsafe {}
  Holes: 1 total
    1 × unsafe {}
✓ FlatHashTable.rs
✓ LinProbFlatHashTableStEph.rs
✓ LinkedListChainedHashTableStEph.rs
✓ ParaHashTableStEph.rs
✓ QuadProbFlatHashTableStEph.rs
✓ StructChainedHashTable.rs
✓ VecChainedHashTableStEph.rs

Modules:
   8 clean (no holes)
   1 holed (contains holes)
   9 total

Holes Found: 1 total
   1 × unsafe {}
```

The single `unsafe` block in `second_hash` reads key bytes via raw pointer for the FNV-1a hash computation. This is a standard pattern for byte-level hashing in Rust and is inherently unsafe. The function ensures the read is bounded by `std::mem::size_of::<Key>()`. This is an acceptable hole for a hash function implementation.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | all |

All functions have no formal specification. The entire chapter is unverified plain Rust with no `verus!` blocks.

## Review TODOs

| # | Priority | Item | Notes |
|---|----------|------|-------|
| 1 | **P0** | Fix `hash_index` in all 3 chained implementations | Change `let hash_val = 0` to `let hash_val = (table.hash_fn)(key)` |
| 2 | **P0** | Fix `num_elements` tracking in chained tables | Increment in `insert_chained` (if new key), decrement in `delete_chained` (if found) |
| 3 | **P1** | Add tests that verify bucket distribution for chained tables | Catch the hash_index=0 and num_elements bugs |
| 4 | **P1** | Add test asserting load factor values for chained tables | Currently only `size` is checked, never `load` |
| 5 | **P2** | Fix LinkedList insert to use `push_front` instead of `push_back` | Match APAS "insert at head of the list" |
| 6 | **P2** | Consider auto-resize on high load factor | APAS discusses doubling when α exceeds threshold |
| 7 | **P3** | Add verus! blocks and specs (long-term verusification) | Currently 0% verified; this is a large effort |
| 8 | **P3** | Implement Exercise 47.6 (single higher-order probe function) | Interesting structural exercise |
| 9 | **P3** | Add Mt implementations | APAS outer-table array structure supports parallel resize |
