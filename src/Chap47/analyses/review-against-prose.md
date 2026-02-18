<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 47: Hash Tables — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6
**Project:** APAS-VERUS-agent2
**Prose source:** `prompts/Chap47.txt`

## Phase 1: Source File Inventory

| # | File | Prose Section | Inside verus! | Outside verus! | Reason Outside |
|---|------|---------------|:---:|:---:|----------------|
| 1 | `ParaHashTableStEph.rs` | §1.1 Parametric Design | `LoadAndSize`, `EntryTrait` | `HashFunGen`, `HashFun`, `HashTable`, `ParaHashTableStEphTrait` | `Rc<dyn Fn>`, `Box<dyn Fn>` fields in HashTable |
| 2 | `ChainedHashTable.rs` | §1.2 Separate Chaining | `ChainEntry` (derive only) | `ChainedHashTable` trait | References `HashTable` (dyn Fn) |
| 3 | `FlatHashTable.rs` | §2.1 Parametric Flat | `FlatEntry`, `EntryTrait for FlatEntry` | `FlatHashTable` trait | References `HashTable` (dyn Fn) |
| 4 | `VecChainedHashTableStEph.rs` | §1.2 (Vec chains) | `EntryTrait for Vec` | `ParaHashTableStEphTrait`, `ChainedHashTable` impls | References `HashTable` (dyn Fn) |
| 5 | `LinkedListChainedHashTableStEph.rs` | §1.2 (LinkedList chains) | (nothing) | All code | `LinkedList` not supported in Verus |
| 6 | `StructChainedHashTable.rs` | §1.2 (custom linked list) | (nothing) | All code | Recursive `Node<K,V>` with `Box` |
| 7 | `LinProbFlatHashTableStEph.rs` | §2 Linear Probing | Struct decl only | All impls | References `HashTable` (dyn Fn) |
| 8 | `QuadProbFlatHashTableStEph.rs` | §2 Quadratic Probing | Struct decl only | All impls | References `HashTable` (dyn Fn) |
| 9 | `DoubleHashFlatHashTableStEph.rs` | §2 Double Hashing | Struct decl, `second_hash` | All other impls | References `HashTable` (dyn Fn) |

**Summary:** 9 source files, 7 test files, 0 PTT files. The dominant reason code lives outside `verus!` is the `HashTable` struct containing `Rc<dyn Fn>` and `Box<dyn Fn>` fields, which Verus cannot handle. This cascades through all traits and impls that reference `HashTable`.

## Phase 2: Prose Alignment

### Definition 47.1 — Hash Table ADT

| # | ADT Operation | Prose Description | Implemented | File(s) |
|---|---------------|-------------------|:-----------:|---------|
| 1 | `createTable(eqFn, hashFnGen, initSize)` | Creates empty table with equality fn, hash fn generator, initial size | Yes | `ParaHashTableStEph.rs` — but equality fn is not an explicit parameter (uses Rust's `PartialEq` trait) |
| 2 | `insert(table, key, value)` | Inserts key-value pair | Yes | All 6 concrete implementations |
| 3 | `lookup(table, key)` | Returns value or indicates not found | Yes | All 6 concrete implementations |
| 4 | `loadAndSize(table)` | Returns (n, m) | Yes | `ParaHashTableStEph.rs` — returns `LoadAndSize { load: f64, size: N }` (computes α directly rather than returning raw n,m) |
| 5 | `resize(table, newSize)` | Rehashes into new table | Yes | All 6 concrete implementations |
| 6 | `delete(table, key)` | Removes key from table | Yes | All 6 concrete implementations (not in Definition 47.1 but in Data Structure 47.6) |

### Section 1.1 — Parametric Nested Design

| # | Prose Concept | Code Alignment | Notes |
|---|---------------|:-:|-------|
| 1 | Outer table = array, inner table = abstract | Aligned | `HashTable.table: Vec<Entry>` is the array; `Entry` is the abstract inner table via `EntryTrait` |
| 2 | Hash function maps keys to `{0..m-1}` | Aligned | `HashFunGen<K> = Rc<dyn Fn(N) -> Box<dyn Fn(&K) -> N>>` generates size-specific hash fns |
| 3 | Load factor α = n/m | Aligned | `loadAndSize` computes `num_elements as f64 / current_size as f64` |
| 4 | Resize by doubling to keep α bounded | Partial | `resize` exists but caller must decide when/how to trigger it; no automatic doubling |

### Section 1.2 — Separate Chaining

| # | Prose Statement | Code Alignment | Notes |
|---|-----------------|:-:|-------|
| 1 | Insert at head of list in O(1) | **GAP** | All 3 chain implementations scan for duplicate keys first, making insert O(chain_length) not O(1). Textbook assumes no dup check. |
| 2 | Lookup = linear scan of chain | Aligned | All implementations do linear scan |
| 3 | Delete = find + remove | Aligned | All implementations find then remove |
| 4 | Expected cost O(1+α) for all ops | Aligned (with caveat) | True in expectation if α is constant and hash is uniform; code does match this expected analysis |

### Section 2 — Flat Tables / Open Addressing

#### Data Structure 47.6 — Entry type

| # | Prose | Code | Notes |
|---|-------|------|-------|
| 1 | `Empty` | `FlatEntry::Empty` | Aligned |
| 2 | `Dead` | `FlatEntry::Deleted` | Name differs (`Dead` vs `Deleted`) but semantics match |
| 3 | `Live(key, value)` | `FlatEntry::Occupied(key, value)` | Name differs but semantics match |

#### Lookup (Data Structure 47.6, lines 1-10)

| # | Prose Step | Code | Notes |
|---|-----------|------|-------|
| 1 | `T[h_i(k)]` = Empty → None | Aligned | All 3 flat impls check `FlatEntry::Empty => return None` |
| 2 | `T[h_i(k)]` = Dead → recurse `i+1` | Aligned | `FlatEntry::Deleted => attempt += 1` |
| 3 | `T[h_i(k)]` = Live(k', v') and k==k' → Some v | Aligned | `FlatEntry::Occupied(k, v) if k == key => Some(v.clone())` |
| 4 | `T[h_i(k)]` = Live(k', v') and k≠k' → recurse `i+1` | Aligned | Falls through to `attempt += 1` |

#### Insert (Data Structure 47.6, lines 1-10)

| # | Prose Step | Code | Notes |
|---|-----------|------|-------|
| 1 | Empty → update(T, h_i(k), Live(k,v)) | Aligned | `FlatEntry::Empty => Occupied(key, value)` |
| 2 | Dead → update(T, h_i(k), Live(k,v)) | Aligned | `FlatEntry::Deleted => Occupied(key, value)` |
| 3 | Live(k', v') and k==k' → () (no-op) | **Differs** | Code updates the value: `Occupied(key, value)`. Prose says do nothing. Code behavior is more useful (upsert). |
| 4 | Live(k', v') and k≠k' → recurse `i+1` | Aligned | Falls through to next probe |

#### Delete (Data Structure 47.6, lines 1-10)

| # | Prose Step | Code | Notes |
|---|-----------|------|-------|
| 1 | Empty → () | Aligned | `FlatEntry::Empty => return false` |
| 2 | Dead → recurse `i+1` | Aligned | `FlatEntry::Deleted => attempt += 1` |
| 3 | Live(k', v') and k==k' → update(T, k, Dead) | Aligned | `table.table[slot] = FlatEntry::Deleted` |
| 4 | Live(k', v') and k≠k' → recurse `i+1` | Aligned | Falls through to `attempt += 1` |

#### Probe Sequences

| # | Strategy | Prose | Code | Formula |
|---|----------|-------|:---:|---------|
| 1 | Linear | Implicit in §2 | `LinProbFlatHashTableStEph` | `(h(k) + i) mod m` |
| 2 | Quadratic | Lemma 47.1: first ⌈m/2⌉ distinct | `QuadProbFlatHashTableStEph` | `(h(k) + i²) mod m`, max_attempts = `⌈m/2⌉` |
| 3 | Double | Exercise 47.4 area | `DoubleHashFlatHashTableStEph` | `(h₁(k) + i·h₂(k)) mod m` |

## Phase 3: Cost Analysis

### Chaining Operations

| # | Operation | APAS Expected | Code Actual | Match? | Notes |
|---|-----------|---------------|-------------|:------:|-------|
| 1 | `createTable` | O(m) | O(m) | Yes | Creates m empty entries |
| 2 | Chain insert (head) | O(1) | O(chain_length) | **No** | Code scans for dup key before inserting |
| 3 | Chain lookup | O(1+α) expected | O(chain_length) | Yes | Same analysis; chain_length is O(1+α) expected |
| 4 | Chain delete | O(1+α) expected | O(chain_length) | Yes | Same analysis |
| 5 | `resize` | O(n+m+m') | O(n+m+m') | Yes | Collects n pairs from m buckets, creates m' new buckets, reinserts |
| 6 | `loadAndSize` | O(1) | O(1) | Yes | Field reads + division |

### Flat Table Operations

| # | Operation | APAS Expected | Code Actual | Match? | Notes |
|---|-----------|---------------|-------------|:------:|-------|
| 1 | `probe` (each strategy) | O(1) | O(1) | Yes | Hash + arithmetic |
| 2 | `find_slot` | O(1/(1−α)) expected | O(1/(1−α)) expected | Yes | Probes until empty/deleted |
| 3 | Insert (unsuccessful) | O(1/(1−α)) expected | O(1/(1−α)) expected | Yes | |
| 4 | Lookup (unsuccessful) | O(1/(1−α)) expected | O(1/(1−α)) expected | Yes | |
| 5 | Lookup (successful) | O((1/α)·ln(1/(1−α))) average | O(1/(1−α)) worst probe | Coarser | Code uses same probe loop; APAS has tighter average-case bound |
| 6 | Delete | O(1/(1−α)) expected | O(1/(1−α)) expected | Yes | Tombstone (Deleted) marker preserves probe chains |
| 7 | `resize` | O(n+m+m') | O(n+m+m') | Yes | |
| 8 | `second_hash` (double) | O(1) per APAS | O(sizeof(Key)) | **Differs** | FNV-1a iterates over key bytes; O(1) only if key size is bounded |

## Phase 4: Parallelism

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | `ParaHashTableStEph` naming | Present | Name suggests parallel, but implementation is fully sequential |
| 2 | Parallel insert/lookup | Not implemented | Textbook doesn't specify parallel hash table ops in this chapter |
| 3 | Parallel resize | Not implemented | Could parallelize rehash with fork-join, but not done |

The "Para" prefix in `ParaHashTableStEph` refers to "parametric" (§1.1 "A Parametric Design"), not "parallel." This is consistent with the textbook section title. No parallelism is expected for this chapter.

## Phase 5: Runtime Tests (RTT)

| # | Test File | Tests | Coverage |
|---|-----------|:-----:|----------|
| 1 | `TestVecChainedHashTable.rs` | 8 | Entry CRUD, table insert/lookup/delete, resize, hash_index |
| 2 | `TestLinProbFlatHashTable.rs` | 20 | Entry CRUD, probe, find_slot, insert/lookup/delete, resize, load_and_size, edge cases |
| 3 | `TestDoubleHashFlatHashTable.rs` | 12 | Basic ops, second_hash properties (nonzero, odd), probe sequence, coprimality, resize |
| 4 | `TestParaHashTableStEph.rs` | 4 | createTable, loadAndSize (empty/with elements), metrics |
| 5 | `TestStructChainedHashTable.rs` | 12 | ChainList CRUD, table ops, default, resize, node clone |
| 6 | `TestLinkedListChainedHashTable.rs` | 11 | Entry CRUD, collision handling, resize, load_and_size, update |
| 7 | `TestQuadProbFlatHashTable.rs` | 14 | Basic ops, quadratic sequence verification, ⌈m/2⌉ limit, prime size guarantees |

**Total: 81 runtime tests across 7 files.**

Test quality is good. Notable coverage:
- Collision handling tested explicitly for LinkedList and double hashing
- Quadratic probing's ⌈m/2⌉ distinct-probe guarantee (Lemma 47.1) tested
- Double hashing coprimality tested (all slots visited with prime table size)
- Delete-through-tombstone probe chain integrity tested for all flat table variants
- Resize preserves elements and excludes deleted entries

**Missing test:** No test file for the `FlatHashTable` trait's default methods (`insert_with_probe`, `lookup_with_probe`) in isolation for quad/double (only tested for linear probing in `TestLinProbFlatHashTable`).

## Phase 6: Proof-Time Tests (PTT)

No PTTs exist for Chapter 47. There are zero `proof fn` definitions across all 9 source files, and zero spec functions with `requires`/`ensures`. This is expected given that nearly all code is outside `verus!` blocks due to the `dyn Fn` limitation.

## Phase 7: Gaps and Issues

### Critical

| # | Issue | Severity | File(s) | Description |
|---|-------|----------|---------|-------------|
| 1 | `hash_index` always returns 0 | **Critical** | `VecChainedHashTableStEph.rs`, `LinkedListChainedHashTableStEph.rs`, `StructChainedHashTable.rs` | All three chained implementations have `let hash_val = 0; hash_val % table.current_size` — a placeholder that sends every key to bucket 0. Tests pass because the chain handles all elements, but this defeats the purpose of hashing. Should call `(table.hash_fn)(key)`. |
| 2 | Zero formal verification | **High** | All files | No `requires`, `ensures`, `spec fn`, or `proof fn` anywhere. The entire chapter is unverified exec code. The root cause is `HashTable` containing `dyn Fn` types. |

### Moderate

| # | Issue | Severity | File(s) | Description |
|---|-------|----------|---------|-------------|
| 3 | Chain insert scans for duplicates | Moderate | Vec, LinkedList, Struct chain impls | APAS says insert-at-head is O(1); code scans for duplicate key making it O(chain_length). This is arguably better behavior (upsert) but doesn't match the prose algorithm. |
| 4 | Flat insert does upsert instead of no-op | Low | All flat impls | APAS insert says "if keyEqual(k, k') then ()" (no-op on existing key). Code updates the value. More useful but differs from prose. |
| 5 | `unsafe` in `second_hash` | Moderate | `DoubleHashFlatHashTableStEph.rs` | Raw pointer arithmetic over key bytes for FNV-1a hash. Correct but unsafe. Could use `std::hash::Hash` instead. |
| 6 | `num_elements` not maintained by chained impls | Moderate | Vec, LinkedList, Struct chain impls | The `ParaHashTableStEphTrait` insert/delete for chained variants delegate to `insert_chained`/`delete_chained` which do not update `table.num_elements`. Only flat table impls maintain this counter. This causes `loadAndSize` to report incorrect load factors for chained tables. |
| 7 | `loadAndSize` returns `f64` instead of `(n, m)` | Low | `ParaHashTableStEph.rs` | APAS returns (load, size) as integers. Code computes the ratio as `f64`. Acceptable but differs from the ADT definition. |

### Design

| # | Issue | File(s) | Description |
|---|-------|---------|-------------|
| 8 | No auto-resize | All impls | Resize exists but is never triggered automatically. Caller must manually check load factor and call resize. APAS discusses "doubling every time the load factor exceeds the desired bound" but this is not implemented. |
| 9 | `ChainEntry` struct unused | `ChainedHashTable.rs` | `ChainEntry<Key, Value, Container>` is defined but never used — the concrete impls use `Vec<(K,V)>`, `LinkedList<(K,V)>`, and `ChainList<K,V>` directly as `Entry` types. |

## Phase 8: TOC and In/Out Table

### TOC Compliance

| # | File | Has TOC | Sections Present | Notes |
|---|------|:-------:|------------------|-------|
| 1 | `ParaHashTableStEph.rs` | Yes | 1,2,4,8,13 | Split between in/out clearly documented |
| 2 | `ChainedHashTable.rs` | Yes | 1,2,4,8,13 | |
| 3 | `FlatHashTable.rs` | Yes | 1,2,4,9,8,13 | Section 9 before 8 (order issue) |
| 4 | `VecChainedHashTableStEph.rs` | Yes | 1,2,4,9 | |
| 5 | `LinkedListChainedHashTableStEph.rs` | Yes | 1,2,4,9 | |
| 6 | `StructChainedHashTable.rs` | Yes | 1,2,4,9,11,13 | |
| 7 | `LinProbFlatHashTableStEph.rs` | Yes | 1,2,4,9 | |
| 8 | `QuadProbFlatHashTableStEph.rs` | Yes | 1,2,4,9 | |
| 9 | `DoubleHashFlatHashTableStEph.rs` | Yes | 1,2,4,9 | |

### In/Out Table

| # | File | Item | Inside verus! | Outside verus! | Correct? | Reason |
|---|------|------|:---:|:---:|:---:|--------|
| 1 | `ParaHashTableStEph` | `LoadAndSize` struct | Yes | - | Correct | No dyn Fn |
| 2 | `ParaHashTableStEph` | `EntryTrait` | Yes | - | Correct | No dyn Fn |
| 3 | `ParaHashTableStEph` | `HashFunGen`, `HashFun` types | - | Yes | Forced | `Rc<dyn Fn>`, `Box<dyn Fn>` |
| 4 | `ParaHashTableStEph` | `HashTable` struct | - | Yes | Forced | Contains `HashFunGen`, `HashFun` fields |
| 5 | `ParaHashTableStEph` | `ParaHashTableStEphTrait` | - | Yes | Forced | Methods reference `HashTable` |
| 6 | `ParaHashTableStEph` | `Debug for LoadAndSize` | - | Yes | Correct | Debug must be outside |
| 7 | `ChainedHashTable` | `ChainEntry` struct | - | Yes (derive) | Acceptable | Outside with `#[derive]` |
| 8 | `ChainedHashTable` | `ChainedHashTable` trait | - | Yes | Forced | References `HashTable` |
| 9 | `ChainedHashTable` | `Debug for ChainEntry` | - | Yes | Correct | Debug must be outside |
| 10 | `FlatHashTable` | `FlatEntry` enum | Yes | - | Correct | No dyn Fn |
| 11 | `FlatHashTable` | `EntryTrait for FlatEntry` | Yes (ext_body) | - | Correct | entry-level ops, no dyn Fn |
| 12 | `FlatHashTable` | `FlatHashTable` trait | - | Yes | Forced | References `HashTable` |
| 13 | `FlatHashTable` | `Debug for FlatEntry` | - | Yes | Correct | Debug must be outside |
| 14 | `VecChainedHashTableStEph` | `EntryTrait for Vec` | Yes (ext_body) | - | Correct | No dyn Fn |
| 15 | `VecChainedHashTableStEph` | `ParaHashTableStEphTrait` impl | - | Yes | Forced | References `HashTable` |
| 16 | `VecChainedHashTableStEph` | `ChainedHashTable` impl | - | Yes | Forced | References `HashTable` |
| 17 | `LinkedListChainedHashTableStEph` | `EntryTrait for LinkedList` | - | Yes | Forced | `LinkedList` not in Verus |
| 18 | `LinkedListChainedHashTableStEph` | All other impls | - | Yes | Forced | `LinkedList` + `HashTable` |
| 19 | `StructChainedHashTable` | `Node`, `ChainList` structs | - | Yes | Forced | Recursive `Box<Node>` |
| 20 | `StructChainedHashTable` | All impls | - | Yes | Forced | Recursive types + `HashTable` |
| 21 | `StructChainedHashTable` | `Debug for Node/ChainList` | - | Yes | Correct | Debug must be outside |
| 22 | `LinProbFlatHashTableStEph` | Struct decl | Yes | - | Correct | |
| 23 | `LinProbFlatHashTableStEph` | All impls | - | Yes | Forced | References `HashTable` |
| 24 | `QuadProbFlatHashTableStEph` | Struct decl | Yes | - | Correct | |
| 25 | `QuadProbFlatHashTableStEph` | All impls | - | Yes | Forced | References `HashTable` |
| 26 | `DoubleHashFlatHashTableStEph` | Struct decl | Yes | - | Correct | |
| 27 | `DoubleHashFlatHashTableStEph` | `second_hash` | Yes (ext_body) | - | Correct | Standalone fn, no dyn Fn |
| 28 | `DoubleHashFlatHashTableStEph` | All other impls | - | Yes | Forced | References `HashTable` |

**Root cause analysis:** The single design decision to store `Rc<dyn Fn>` and `Box<dyn Fn>` in `HashTable` forces the entire `ParaHashTableStEphTrait` and all its implementors outside `verus!`. This cascades to ~85% of the codebase being unverifiable. The only items that remain inside `verus!` are:
- Type definitions that don't reference `HashTable` (`LoadAndSize`, `FlatEntry`, struct decls)
- `EntryTrait` (methods don't take `HashTable`)
- `EntryTrait` impls for `Vec` and `FlatEntry` (external_body but inside verus!)
- `second_hash` (standalone, no dyn Fn)

## Proof Holes

| # | File | Hole Type | Location | Description |
|---|------|-----------|----------|-------------|
| 1 | `FlatHashTable.rs` | `external_body` | `FlatEntry::new` | Trivial — could potentially verify |
| 2 | `FlatHashTable.rs` | `external_body` | `FlatEntry::insert` | Trivial — enum assignment |
| 3 | `FlatHashTable.rs` | `external_body` | `FlatEntry::lookup` | Match + key comparison |
| 4 | `FlatHashTable.rs` | `external_body` | `FlatEntry::delete` | Match + enum assignment |
| 5 | `VecChainedHashTableStEph.rs` | `external_body` | `Vec::new` | Trivial |
| 6 | `VecChainedHashTableStEph.rs` | `external_body` | `Vec::insert` | Linear scan + push |
| 7 | `VecChainedHashTableStEph.rs` | `external_body` | `Vec::lookup` | Linear scan |
| 8 | `VecChainedHashTableStEph.rs` | `external_body` | `Vec::delete` | Linear scan + remove |
| 9 | `DoubleHashFlatHashTableStEph.rs` | `external_body` | `second_hash` | FNV-1a hash over key bytes |
| 10 | `DoubleHashFlatHashTableStEph.rs` | `unsafe` | Inside `second_hash` | Raw pointer arithmetic over key bytes |

**Total: 9 `external_body` + 1 `unsafe`.**

Holes 1-4 (`FlatEntry` methods) are potentially removable — they are simple enum operations that Verus should be able to verify if given proper `ensures` clauses. Holes 5-8 (`Vec` methods) require Verus's `Vec` specs which may be sufficient. Holes 9-10 are inherently unverifiable (raw pointer arithmetic).

## Action Items

| # | Priority | Description | Files |
|---|----------|-------------|-------|
| 1 | **P0** | Fix `hash_index` placeholder — call `(table.hash_fn)(key)` instead of returning 0 | `VecChainedHashTableStEph.rs`, `LinkedListChainedHashTableStEph.rs`, `StructChainedHashTable.rs` |
| 2 | **P1** | Fix `num_elements` tracking in chained insert/delete (either in `insert_chained`/`delete_chained` or in each concrete impl) | `ChainedHashTable.rs` or all chained impls |
| 3 | **P1** | Investigate removing `external_body` from `FlatEntry` methods (simple enum ops that Verus may verify) | `FlatHashTable.rs` |
| 4 | **P2** | Add auto-resize logic (e.g., double table when α > threshold) | All impls or a wrapper layer |
| 5 | **P2** | Consider redesigning `HashTable` to avoid `dyn Fn` — e.g., use a `Hasher` trait bound instead, enabling the struct to live inside `verus!` | `ParaHashTableStEph.rs` and all dependents |
| 6 | **P3** | Remove unused `ChainEntry` struct | `ChainedHashTable.rs` |
| 7 | **P3** | Replace `unsafe` pointer arithmetic in `second_hash` with `std::hash::Hash` | `DoubleHashFlatHashTableStEph.rs` |
| 8 | **P3** | Add PTTs once spec functions exist (blocked on P5) | New `rust_verify_test/` files |
