<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 47: Hash Tables — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory (Tool-Generated)

58 functions extracted across 9 source files. All functions are outside `verus!` blocks. No functions have `requires`/`ensures`. No functions are inside `verus!` macro.

| # | Module | Functions | V! | -V! | NoSpec | SpecStr |
|---|--------|----------:|:--:|:---:|-------:|:-------:|
| 1 | ChainedHashTable | 4 | 0 | 4 | 4 | all none |
| 2 | DoubleHashFlatHashTableStEph | 7 | 0 | 7 | 7 | all none |
| 3 | FlatHashTable | 8 | 0 | 8 | 8 | all none |
| 4 | LinProbFlatHashTableStEph | 6 | 0 | 6 | 6 | all none |
| 5 | LinkedListChainedHashTableStEph | 6 | 0 | 6 | 6 | all none |
| 6 | ParaHashTableStEph | 8 | 0 | 8 | 8 | all none |
| 7 | QuadProbFlatHashTableStEph | 6 | 0 | 6 | 6 | all none |
| 8 | StructChainedHashTable | 7 | 0 | 7 | 7 | all none |
| 9 | VecChainedHashTableStEph | 6 | 0 | 6 | 6 | all none |

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 47.1 (Hash Tables) | ADT with createTable, insert, lookup, loadAndSize, resize on key-value pairs |
| 2 | Definition 47.2 (Load Factor) | α = n/m where n = stored pairs, m = table size |
| 3 | Definition 47.3 (Separate Chaining) | Nested table using array of lists for collision resolution |
| 4 | Definition 47.5 (Probe Sequence) | Permutation of {0, 1, ..., m−1} for flat tables |
| 5 | Data Structure 47.6 (Parametric Flat Hash Tables) | Entry = Empty | Dead | Live(key, value); open addressing with parametric probe functions |
| 6 | Definition 47.7 (Linear Probing) | h_i(k) = (h(k) + i) mod m |
| 7 | Definition 47.8 (Quadratic Probing) | h_i(k) = (h(k) + i²) mod m |
| 8 | Definition 47.9 (Secondary Clustering) | Two keys hashing to same location share probe sequence |
| 9 | Definition 47.10 (Double Hashing) | h_i(k) = (h(k) + i·hh(k)) mod m |

### Algorithms (Pseudocode)

| # | Item | Description |
|---|------|-------------|
| 1 | lookup (flat) | Probe sequence: Empty→None, Dead→continue, Live(k',v')→match or continue |
| 2 | insert (flat) | Probe sequence: Empty/Dead→place, Live(k',v')→match(noop) or continue |
| 3 | delete (flat) | Probe sequence: Empty→noop, Dead→continue, Live(k',v')→match(mark Dead) or continue |

### Cost Specs

| # | Operation | Cost |
|---|-----------|------|
| 1 | Chained insert | O(1) (insert at head of chain) |
| 2 | Chained lookup/delete | O(1+α) expected (traverse chain) |
| 3 | Flat insert (unsuccessful lookup) | O(1/(1−α)) expected |
| 4 | Flat successful lookup | O((1/α)·ln(1/(1−α))) expected |
| 5 | createTable | O(m) |
| 6 | resize | O(n + m + m') |

### Theorems/Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Lemma 47.1 | If m is prime and table ≥ half empty, quadratic probing finds empty cell; first ⌈m/2⌉ probes are distinct |

### Exercises

| # | Item | Status |
|---|------|--------|
| 1 | Exercise 47.1 | Describe nested table implementation using Table ADT (text) |
| 2 | Exercise 47.2 | Discuss reducing hash table size (text) |
| 3 | Exercise 47.3 | Implement resize operation and bound cost |
| 4 | Exercise 47.6 | Show parametric flat hash table can use single higher-order function |
| 5 | Exercise 47.7 | Complete parametric flat hash table implementation (remaining operations) |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All 58 exec functions now have APAS/Claude-Opus-4.6 cost annotation pairs.

**Cost disagreements found:** None significant. All implementations agree with APAS costs. Minor note: `second_hash` in DoubleHashFlatHashTableStEph iterates over key bytes (O(sizeof(Key))), but APAS treats hashing as O(1) — this is the standard assumption.

### 3b. Implementation Fidelity

| # | Prose Item | Code Implementation | Fidelity | Notes |
|---|-----------|---------------------|----------|-------|
| 1 | Def 47.1 createTable | `ParaHashTableStEphTrait::createTable` | Faithful | Hash function generator pattern matches APAS parametric design |
| 2 | Def 47.1 insert | Multiple implementations | Faithful | Each probing strategy implements correctly |
| 3 | Def 47.1 lookup | Multiple implementations | Faithful | Correctly stops at Empty, skips Dead for flat tables |
| 4 | Def 47.1 loadAndSize | `ParaHashTableStEphTrait::loadAndSize` | Faithful | Returns load factor and size |
| 5 | Def 47.1 resize | Multiple implementations | Faithful | Collects pairs, creates new table, reinserts |
| 6 | Def 47.3 Separate Chaining | Vec/LinkedList/StructChained | Faithful | Three chain representations provided |
| 7 | DS 47.6 Entry type | `FlatEntry::Empty/Occupied/Deleted` | Faithful | Maps to Empty/Live/Dead from prose |
| 8 | DS 47.6 lookup (flat) | LinProb/QuadProb/DoubleHash lookup | Faithful | Matches pseudocode: probe sequence with Empty/Dead/Live handling |
| 9 | DS 47.6 insert (flat) | LinProb/QuadProb/DoubleHash insert | Faithful | Matches pseudocode |
| 10 | DS 47.6 delete (flat) | LinProb/QuadProb/DoubleHash delete | Faithful | Marks as Deleted (tombstone) per prose |
| 11 | Def 47.7 Linear Probing | `LinProbFlatHashTableStEph::probe` | Faithful | h_i(k) = (h(k) + i) mod m |
| 12 | Def 47.8 Quadratic Probing | `QuadProbFlatHashTableStEph::probe` | Faithful | h_i(k) = (h(k) + i²) mod m |
| 13 | Def 47.10 Double Hashing | `DoubleHashFlatHashTableStEph::probe` | Faithful | h_i(k) = (h(k) + i·hh(k)) mod m |
| 14 | Lemma 47.1 (⌈m/2⌉ bound) | QuadProb max_attempts | Faithful | `div_ceil(2)` limits probe count per lemma |

**Deviations:**

| # | Deviation | Severity | Notes |
|---|-----------|----------|-------|
| 1 | `hash_index` in Vec/LinkedList/Struct chained implementations is a placeholder (always returns 0) | High | All keys hash to bucket 0, making the table degenerate to a single chain |
| 2 | Chained insert does duplicate-key scan (O(n)) instead of O(1) head insert | Low | APAS says "inserting the key-value pair at the head of the list" requires O(1); implementations scan for existing key first, which is O(1+α) but provides update-on-duplicate semantics |
| 3 | No automatic resize on high load factor | Medium | APAS discusses resize triggers when α exceeds a threshold; implementations provide resize but don't auto-trigger |

### 3c. Spec Fidelity

**No specs exist.** All 58 functions have spec strength "none" — no `requires`/`ensures` anywhere in the chapter. The entire chapter is unverified plain Rust with no `verus!` blocks.

The following prose properties have no formal spec:
- Lookup returns the correct value for a stored key
- Insert preserves existing entries
- Delete marks entries correctly (tombstone semantics for flat tables)
- Resize preserves all key-value pairs
- Load factor computation is correct
- Probe sequences are permutations of {0, ..., m−1}
- Lemma 47.1 (quadratic probing distinctness guarantee)

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 47.** All 9 files are `StEph` (sequential ephemeral) or shared trait definitions. No parallelism review is needed.

## Phase 5: Runtime Test Review

**No runtime tests exist.** No files matching `tests/Chap47*` or `tests/**/Chap47*` were found.

### 5a. Coverage Check

| # | Source Module | Test File | Status |
|---|-------------|-----------|--------|
| 1 | ParaHashTableStEph.rs | — | Missing RTT |
| 2 | ChainedHashTable.rs | — | Missing RTT |
| 3 | FlatHashTable.rs | — | Missing RTT |
| 4 | VecChainedHashTableStEph.rs | — | Missing RTT |
| 5 | LinkedListChainedHashTableStEph.rs | — | Missing RTT |
| 6 | StructChainedHashTable.rs | — | Missing RTT |
| 7 | LinProbFlatHashTableStEph.rs | — | Missing RTT |
| 8 | QuadProbFlatHashTableStEph.rs | — | Missing RTT |
| 9 | DoubleHashFlatHashTableStEph.rs | — | Missing RTT |

### 5c. Missing Tests — Priority

1. **All concrete implementations** (VecChained, LinkedListChained, StructChained, LinProb, QuadProb, DoubleHash) — these are the only evidence of correctness since there are no formal specs
2. **hash_index placeholder** — critical bug: always returns 0, would be caught by any test with multiple keys

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs needed.** Chapter 47 has no `verus!` blocks, no iterators, and no verified loops. No types implement `iter()`, `IntoIterator`, `GhostIterator`, or `ForLoopGhostIterator`.

### 6a. Unified Test Inventory

| # | Source Module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | ParaHashTableStEph | — | — | Missing both |
| 2 | ChainedHashTable | — | — | Missing both |
| 3 | FlatHashTable | — | — | Missing both |
| 4 | VecChainedHashTableStEph | — | — | Missing both |
| 5 | LinkedListChainedHashTableStEph | — | — | Missing both |
| 6 | StructChainedHashTable | — | — | Missing both |
| 7 | LinProbFlatHashTableStEph | — | — | Missing both |
| 8 | QuadProbFlatHashTableStEph | — | — | Missing both |
| 9 | DoubleHashFlatHashTableStEph | — | — | Missing both |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Gap |
|---|-----------|-----|
| 1 | Delete operation (chained) | Not in `ParaHashTableStEphTrait` trait — but present in `ChainedHashTable` trait and all concrete impls |
| 2 | Exercise 47.3 (resize implementation and cost bound) | Resize is implemented but cost bound is not formally verified |
| 3 | Exercise 47.6 (single higher-order function) | Not implemented — the flat table uses separate insert/lookup/delete |
| 4 | Lemma 47.1 (quadratic probing proof) | Correctly applied (⌈m/2⌉ bound) but not formally proved |

### Code With No Prose Counterpart

| # | Item | Notes |
|---|------|-------|
| 1 | `Metrics` type parameter | Implementation scaffolding for instrumentation, not in prose |
| 2 | `second_hash` (FNV-1a) | APAS mentions hh(k) abstractly; implementation provides a concrete FNV-1a hash |
| 3 | `insert_with_probe` / `lookup_with_probe` | Default trait implementations in FlatHashTable; prose describes these inline |
| 4 | `ChainEntry` struct | Parametric container type not in prose |
| 5 | `LoadAndSize` struct | Return type for loadAndSize; prose returns pair |
| 6 | `HashFunGen` / `HashFun` types | Type aliases for hash function machinery |

## Phase 8: Table of Contents Review

**No TOC headers present in any file.** None of the 9 files follow the table-of-contents standard. This is expected since no files contain `verus!` blocks — the TOC standard applies to verusified files.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | ParaHashTableStEph.rs | - | ✅ out (LoadAndSize derive) | - | - | - | ✅ out (LoadAndSize derive) | - | - | - |
| 2 | ChainedHashTable.rs | ✅ out (ChainEntry derive) | ✅ out (ChainEntry derive) | - | - | - | ✅ out (ChainEntry derive) | - | - | - |
| 3 | FlatHashTable.rs | ✅ out (FlatEntry derive) | ✅ out (FlatEntry derive) | - | - | - | ✅ out (FlatEntry derive) | - | - | - |
| 4 | VecChainedHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 5 | LinkedListChainedHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 6 | StructChainedHashTable.rs | ✅ out (Node, ChainList derive) | ✅ out (Node, ChainList derive) | ✅ out (ChainList impl) | - | - | ✅ out (Node, ChainList derive) | - | - | - |
| 7 | LinProbFlatHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 8 | QuadProbFlatHashTableStEph.rs | - | - | - | - | - | - | - | - | - |
| 9 | DoubleHashFlatHashTableStEph.rs | - | - | - | - | - | - | - | - | - |

All derive impls are outside `verus!` which is correct for non-verusified code. No `❌` items.

## Proof Holes Summary

```
Modules:
   8 clean (no holes)
   1 holed (contains holes)
   9 total

Holes Found: 1 total
   1 × unsafe {} in DoubleHashFlatHashTableStEph.rs:35 (FNV-1a byte iteration)
```

The single `unsafe` block in `second_hash` reads key bytes via raw pointer for the FNV-1a hash computation. This is inherently unsafe in Rust but is a standard pattern for byte-level hashing.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 58 |

**All 58 functions have no formal specification.** The entire chapter is unverified plain Rust with no `verus!` blocks.

## Overall Assessment

Chapter 47 implements the APAS hash table designs faithfully in terms of algorithmic structure:

1. **Parametric design** is well-captured via Rust traits (`ParaHashTableStEphTrait`, `ChainedHashTable`, `FlatHashTable`, `EntryTrait`).
2. **Three chaining variants** (Vec, LinkedList, custom struct) exercise the parametric inner table design.
3. **Three flat table variants** (linear probing, quadratic probing, double hashing) match APAS Definitions 47.7, 47.8, 47.10.
4. **Lemma 47.1** is correctly applied in quadratic probing (⌈m/2⌉ probe limit).

**Critical gaps:**

1. **No verification at all** — 0 of 58 functions are inside `verus!`. No specs, no proofs.
2. **No runtime tests** — 0 test files for 9 source modules. The `hash_index` placeholder bug (always returns 0) would be immediately caught by tests.
3. **Placeholder hash_index** — Three chained implementations have broken `hash_index` that always returns 0, making all keys collide to bucket 0. The `hash_fn` in the `HashTable` struct is properly used by flat table implementations but ignored by chained implementations.
4. **No Mt implementations** — APAS discusses parallelism (Section 1 mentions array-based outer table for O(1) access); no parallel variants exist.
