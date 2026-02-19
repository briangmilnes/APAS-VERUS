<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 47: Hash Tables — Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap47.txt`, `prompts/Chap47part2.txt`

## Phase 1: Inventory

58 function entries extracted by `veracity-review-module-fn-impls` across 9 source files. Most code lives outside `verus!` due to `HashTable` containing `Rc<dyn Fn>` and `Box<dyn Fn>`.

| # | File | Lines | V! Content | Outside verus! | Description |
|---|------|------:|------------|----------------|-------------|
| 1 | `ParaHashTableStEph.rs` | 155 | `LoadAndSize`, `EntryTrait` | `HashFunGen`, `HashFun`, `HashTable`, `ParaHashTableStEphTrait` | §1.1 Parametric design |
| 2 | `ChainedHashTable.rs` | 99 | `ChainEntry` (derive) | `ChainedHashTable` trait | §1.2 Separate chaining base |
| 3 | `FlatHashTable.rs` | 131 | `FlatEntry`, `EntryTrait for FlatEntry` | `FlatHashTable` trait | §2.1 Parametric flat base |
| 4 | `VecChainedHashTableStEph.rs` | 143 | `EntryTrait for Vec` | `ParaHashTableStEphTrait`, `ChainedHashTable` impls | §1.2 Vec chains |
| 5 | `LinkedListChainedHashTableStEph.rs` | 141 | (nothing) | All code | §1.2 LinkedList chains |
| 6 | `StructChainedHashTable.rs` | 193 | (nothing) | All code | §1.2 Custom linked list chains |
| 7 | `LinProbFlatHashTableStEph.rs` | 152 | Struct decl only | All impls | §2.2 Linear probing |
| 8 | `QuadProbFlatHashTableStEph.rs` | 156 | Struct decl only | All impls | §2.3 Quadratic probing |
| 9 | `DoubleHashFlatHashTableStEph.rs` | 193 | Struct decl, `second_hash` | All other impls | §2.4 Double hashing |

**Root cause for code outside verus!:** `HashTable` struct contains `Rc<dyn Fn>` and `Box<dyn Fn>` fields, which Verus cannot handle. This cascades through all traits and impls that reference `HashTable`, forcing ~85% of the codebase outside `verus!`.

## Phase 2: Prose Inventory

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 47.1 | Hash Table ADT: `createTable`, `insert`, `lookup`, `loadAndSize`, `resize`, (+ `delete` from §2) |
| 2 | Definition 47.2 | Load Factor: α = n/m |
| 3 | Definition 47.3 | Separate Chaining: inner table as a list of key-value pairs |
| 4 | Definition 47.5 | Probe Sequence: permutation of {0, 1, ..., m-1} |
| 5 | Definition 47.7 | Linear Probing: h_i(k) = (h(k) + i) mod m |
| 6 | Definition 47.8 | Quadratic Probing: h_i(k) = (h(k) + i²) mod m |
| 7 | Definition 47.9 | Secondary Clustering |
| 8 | Definition 47.10 | Double Hashing: h_i(k) = (h(k) + i·hh(k)) mod m |

### Algorithms / Data Structures

| # | Item | Description |
|---|------|-------------|
| 1 | §1.1 Parametric Design | Outer table = array of inner tables; hash function maps keys to inner table index |
| 2 | Data Structure 47.6 | Parametric Flat Hash Table: entry = Empty / Dead / Live(key, value); lookup, insert, delete via probe sequence |
| 3 | Lemma 47.1 | Quadratic probing: if m is prime and table ≥ half empty, first ⌈m/2⌉ probes are distinct |

### Exercises

| # | Exercise | Status |
|---|----------|--------|
| 1 | Exercise 47.1 | Implement nested tables using Table ADT (text file) — not directly implemented |
| 2 | Exercise 47.2 | When to reduce table size (text file) — not implemented |
| 3 | Exercise 47.3 | Implement resize with cost bound — implemented in all 6 concrete impls |
| 4 | Exercise 47.6 | Single higher-order function for lookup/insert/delete — not implemented |
| 5 | Exercise 47.7 | Complete parametric flat hash table implementation — implemented |

### Cost Specs

| # | Strategy | insert | lookup | delete | resize |
|---|----------|--------|--------|--------|--------|
| 1 | Separate Chaining | O(1+α) expected | O(1+α) expected | O(1+α) expected | O(n+m+m') |
| 2 | Flat (general) | O(1/(1−α)) expected | O(1/(1−α)) expected | O(1/(1−α)) expected | O(n+m+m') |
| 3 | Flat (successful lookup) | — | O((1/α)·ln(1/(1−α))) average | — | — |

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

| # | Module | Function | APAS Cost | Actual Cost | Match? | Notes |
|---|--------|----------|-----------|-------------|:------:|-------|
| 1 | ParaHashTableStEph | `createTable` | O(m) | O(m) | Yes | Creates m empty entries |
| 2 | All chained impls | `insert_chained` | O(1) head insert | O(chain_length) | **No** | Scans for duplicate key before inserting |
| 3 | All chained impls | `lookup_chained` | O(1+α) expected | O(chain_length) | Yes | Same analysis |
| 4 | All chained impls | `delete_chained` | O(1+α) expected | O(chain_length) | Yes | Same analysis |
| 5 | All impls | `resize` | O(n+m+m') | O(n+m+m') | Yes | Collects pairs, creates new table, reinserts |
| 6 | ParaHashTableStEph | `loadAndSize` | O(1) | O(1) | Yes | Field reads + division |
| 7 | All flat impls | `probe` | O(1) | O(1) | Yes | Hash + arithmetic |
| 8 | All flat impls | `find_slot` | O(1/(1−α)) exp | O(1/(1−α)) exp | Yes | Probes until empty/deleted |
| 9 | All flat impls | `insert` | O(1/(1−α)) exp | O(1/(1−α)) exp | Yes | find_slot + O(1) write |
| 10 | All flat impls | `lookup` | O(1/(1−α)) exp | O(1/(1−α)) exp | Yes | Probe until found/empty |
| 11 | All flat impls | `delete` | O(1/(1−α)) exp | O(1/(1−α)) exp | Yes | Tombstone marker |
| 12 | DoubleHash | `second_hash` | O(1) per APAS | O(sizeof(Key)) | **Differs** | FNV-1a iterates key bytes |

### Phase 3b: Implementation Fidelity

| # | Prose Item | Implementation | Fidelity | Notes |
|---|-----------|---------------|----------|-------|
| 1 | Definition 47.1 (Hash Table ADT) | `ParaHashTableStEphTrait` | ✅ Faithful | All ADT operations present; equality via Rust `PartialEq` rather than explicit param |
| 2 | §1.1 Parametric nested design | `ParaHashTableStEph.rs` | ✅ Faithful | Outer array + abstract inner `EntryTrait` |
| 3 | §1.2 Separate chaining | Vec, LinkedList, Struct chain impls | ✅ Faithful | Three chain implementations with different inner containers |
| 4 | Data Structure 47.6 (Flat entry type) | `FlatEntry::Empty/Occupied/Deleted` | ✅ Faithful | Names differ (Dead→Deleted, Live→Occupied) but semantics match |
| 5 | Data Structure 47.6 (Flat lookup) | All flat impls | ✅ Faithful | Probe sequence matches: Empty→None, Dead→next, Live+match→Some, Live+no match→next |
| 6 | Data Structure 47.6 (Flat insert) | All flat impls | ⚠️ Minor | Prose says no-op on existing key; code does upsert (updates value). More useful behavior. |
| 7 | Definition 47.7 (Linear probing) | `LinProbFlatHashTableStEph` | ✅ Faithful | `(h(k) + i) mod m` |
| 8 | Definition 47.8 (Quadratic probing) | `QuadProbFlatHashTableStEph` | ✅ Faithful | `(h(k) + i²) mod m`, max_attempts = `⌈m/2⌉` per Lemma 47.1 |
| 9 | Definition 47.10 (Double hashing) | `DoubleHashFlatHashTableStEph` | ✅ Faithful | `(h₁(k) + i·h₂(k)) mod m`, second hash ensures coprimality |
| 10 | Lemma 47.1 (Quadratic ⌈m/2⌉ guarantee) | `QuadProbFlatHashTableStEph` | ✅ Implemented | `max_attempts = table.current_size.div_ceil(2)` |
| 11 | Load factor α = n/m | `loadAndSize` | ✅ Faithful | Computes `num_elements / current_size` as f64 |
| 12 | Resize by doubling | All impls | ⚠️ Partial | `resize` exists but no automatic trigger; caller must decide when |

### Phase 3c: Spec Fidelity

Minimal verification exists. Only `EntryTrait` and a few type definitions are inside `verus!`, all with `external_body`. No `requires`/`ensures` specifications on any function.

## Phase 4: Parallelism Review

The "Para" prefix in `ParaHashTableStEph` refers to "parametric" (§1.1 "A Parametric Design"), not "parallel." No parallelism is expected for this chapter. No Mt modules exist.

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `ParaHashTableStEph` naming | Present | "Parametric", not "parallel" |
| 2 | Parallel insert/lookup | Not implemented | Textbook doesn't specify parallel hash table ops |
| 3 | Parallel resize | Not implemented | Could parallelize rehash, but not needed |

## Phase 5: Runtime Test Review

| # | Test File | Test Count | Coverage |
|---|-----------|:----------:|----------|
| 1 | `TestVecChainedHashTable.rs` | 9 | Entry CRUD, table insert/lookup/delete, resize, hash_index |
| 2 | `TestLinProbFlatHashTable.rs` | 26 | Entry CRUD, probe, find_slot, insert/lookup/delete, resize, load_and_size, edge cases |
| 3 | `TestDoubleHashFlatHashTable.rs` | 15 | Basic ops, second_hash properties (nonzero, odd), probe sequence, coprimality, resize |
| 4 | `TestParaHashTableStEph.rs` | 4 | createTable, loadAndSize (empty/with elements), metrics |
| 5 | `TestStructChainedHashTable.rs` | 11 | ChainList CRUD, table ops, default, resize, node clone |
| 6 | `TestLinkedListChainedHashTable.rs` | 13 | Entry CRUD, collision handling, resize, load_and_size, update |
| 7 | `TestQuadProbFlatHashTable.rs` | 12 | Basic ops, quadratic sequence verification, ⌈m/2⌉ limit, prime size guarantees |

**Total: 90 runtime tests across 7 files.**

Test quality is good. Notable coverage:
- Collision handling tested explicitly for LinkedList and double hashing
- Quadratic probing's ⌈m/2⌉ distinct-probe guarantee (Lemma 47.1) tested
- Double hashing coprimality tested
- Delete-through-tombstone probe chain integrity tested
- Resize preserves elements and excludes deleted entries

## Phase 6: PTT Review

No PTTs exist for Chapter 47. There are zero `proof fn` definitions and zero spec functions with `requires`/`ensures`. This is expected given that nearly all code is outside `verus!`.

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Exercise 47.1 (nested tables via Table ADT) | Not implemented | Text exercise |
| 2 | Exercise 47.2 (reduce table size) | Not implemented | Text exercise |
| 3 | Exercise 47.6 (single higher-order function) | Not implemented | Refactoring exercise |
| 4 | Auto-resize (doubling on load factor threshold) | Not implemented | `resize` exists but never triggered automatically |
| 5 | Primary clustering analysis | Not demonstrated | Prose discusses it; code does not measure or demonstrate |

### Code with No Prose Counterpart

| # | Item | Module | Purpose |
|---|------|--------|---------|
| 1 | `ChainEntry` struct | ChainedHashTable | Defined but unused — concrete impls use Vec/LinkedList/ChainList directly |
| 2 | `FlatHashTable::insert_with_probe`, `lookup_with_probe` | FlatHashTable | Default trait methods for flat tables |
| 3 | `metrics` accessor | ParaHashTableStEph | Scaffolding for performance metrics |
| 4 | `Debug` impls | All files | Trait impls for ergonomics |

### Critical Issues

| # | Issue | Severity | Notes |
|---|-------|----------|-------|
| 1 | Zero formal verification | High | Root cause: `dyn Fn` in `HashTable` prevents `verus!` placement |
| 2 | Chain insert scans for duplicates | Moderate | APAS says O(1) head insert; code scans for dup key making it O(chain_length) |
| 3 | Flat insert does upsert | Low | APAS insert is no-op on existing key; code updates value |
| 4 | `unsafe` in `second_hash` | Moderate | Raw pointer arithmetic over key bytes; could use `std::hash::Hash` instead |
| 5 | `loadAndSize` returns f64 ratio | Low | APAS returns (n, m) as integers; code computes α as f64 |
| 6 | No auto-resize | Moderate | Resize exists but is never triggered automatically |

## Phase 8: TOC and In/Out Review

### TOC Compliance

| # | File | Has TOC | Notes |
|---|------|:-------:|-------|
| 1 | `ParaHashTableStEph.rs` | Yes | Split between in/out clearly documented |
| 2 | `ChainedHashTable.rs` | Yes | |
| 3 | `FlatHashTable.rs` | Yes | Section 9 before 8 (order issue) |
| 4 | `VecChainedHashTableStEph.rs` | Yes | |
| 5 | `LinkedListChainedHashTableStEph.rs` | Yes | |
| 6 | `StructChainedHashTable.rs` | Yes | |
| 7 | `LinProbFlatHashTableStEph.rs` | Yes | |
| 8 | `QuadProbFlatHashTableStEph.rs` | Yes | |
| 9 | `DoubleHashFlatHashTableStEph.rs` | Yes | |

### In/Out Table

| # | File | Item | In V! | Out V! | Correct? | Reason |
|---|------|------|:-----:|:------:|:--------:|--------|
| 1 | ParaHashTableStEph | `LoadAndSize` | Yes | - | Correct | No dyn Fn |
| 2 | ParaHashTableStEph | `EntryTrait` | Yes | - | Correct | No dyn Fn |
| 3 | ParaHashTableStEph | `HashTable` struct | - | Yes | Forced | Contains `Rc<dyn Fn>`, `Box<dyn Fn>` |
| 4 | ParaHashTableStEph | `ParaHashTableStEphTrait` | - | Yes | Forced | Methods reference `HashTable` |
| 5 | FlatHashTable | `FlatEntry` enum | Yes | - | Correct | No dyn Fn |
| 6 | FlatHashTable | `EntryTrait for FlatEntry` | Yes (ext_body) | - | Correct | Entry-level ops, no dyn Fn |
| 7 | FlatHashTable | `FlatHashTable` trait | - | Yes | Forced | References `HashTable` |
| 8 | VecChainedHashTableStEph | `EntryTrait for Vec` | Yes (ext_body) | - | Correct | No dyn Fn |
| 9 | VecChainedHashTableStEph | `ParaHashTableStEphTrait` impl | - | Yes | Forced | References `HashTable` |
| 10 | DoubleHashFlatHashTableStEph | `second_hash` | Yes (ext_body) | - | Correct | Standalone fn |
| 11 | All others | All impls | - | Yes | Forced | `HashTable` cascade |
| 12 | All | Debug impls | - | Yes | Correct | Debug must be outside |

## Proof Holes Summary

| # | File | Hole Type | Location | Description |
|---|------|-----------|----------|-------------|
| 1 | `FlatHashTable.rs` | `external_body` ×4 | `FlatEntry::{new, insert, lookup, delete}` | Simple enum ops; potentially removable |
| 2 | `VecChainedHashTableStEph.rs` | `external_body` ×4 | `Vec::{new, insert, lookup, delete}` | Vec operations |
| 3 | `DoubleHashFlatHashTableStEph.rs` | `external_body` ×1 | `second_hash` | FNV-1a hash |
| 4 | `DoubleHashFlatHashTableStEph.rs` | `unsafe` ×1 | Inside `second_hash` | Raw pointer arithmetic |

**Total: 9 `external_body` + 1 `unsafe` = 10 holes.**

## Spec Strength Summary

| Classification | Count |
|---|:---:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 58 |

All 58 functions have **no spec**. The `external_body` functions inside `verus!` lack `ensures` clauses.

## Overall Assessment

### Strengths

1. **Complete prose coverage**: All strategies from the textbook are implemented — parametric nested (3 chain variants), linear probing, quadratic probing, and double hashing.
2. **Parametric design**: The trait hierarchy (`EntryTrait` → `ParaHashTableStEphTrait` → `FlatHashTable`/`ChainedHashTable`) faithfully captures the §1.1 parametric structure.
3. **Lemma 47.1 implemented**: Quadratic probing correctly limits to `⌈m/2⌉` attempts.
4. **Good test coverage**: 90 tests covering all implementations including collision handling, resize, and probe sequence properties.
5. **TOC compliance**: All 9 files have TOC headers with in/out annotations.
6. **Cost annotations**: Every function has APAS and Claude cost annotations in doc comments.

### Weaknesses

1. **Zero formal verification**: `dyn Fn` in `HashTable` forces ~85% of code outside `verus!`. The 10 `external_body` functions inside `verus!` have no specs.
2. **Chain insert deviates from prose**: All chain impls scan for duplicate keys (O(chain_length)) instead of O(1) head insert.
3. **`unsafe` in `second_hash`**: Raw pointer arithmetic for FNV-1a hashing.
4. **No auto-resize**: Resize logic exists but is never triggered automatically.
5. **Unused `ChainEntry` struct**: Defined but never used in any implementation.

### Review TODOs

| # | Priority | TODO | Notes |
|---|:--------:|------|-------|
| 1 | P1 | Investigate removing `external_body` from `FlatEntry` methods | Simple enum ops that Verus may verify |
| 2 | P2 | Add auto-resize logic | Double table when α > threshold |
| 3 | P2 | Consider redesigning `HashTable` to avoid `dyn Fn` | Use a `Hasher` trait bound instead, enabling struct inside `verus!` |
| 4 | P3 | Remove unused `ChainEntry` struct | Dead code |
| 5 | P3 | Replace `unsafe` in `second_hash` with `std::hash::Hash` | Safety improvement |
