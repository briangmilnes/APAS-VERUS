# Agent 4 Work Plan — Small Chapters + Tables

Baseline from hole analysis logs (2026-03-11):

| # | Chap | Holes | Errors | Hole Types | Clean/Total |
|---|------|-------|--------|------------|-------------|
| 1 | 03 | 0 | 1 fn_missing_requires | — | 0/1 |
| 2 | 06 | 0 | 2 fn_missing_ensures | — | 19/20 |
| 3 | 12 | 1 | 0 | 1 trivial_wf | 2/3 |
| 4 | 21 | 0 | 4 fn_missing_requires | — | 9/12 |
| 5 | 23 | 2 | 1 fn_missing_requires | 2 trivial_wf | 0/2 |
| 6 | 26 | 4 | 0 | 4 external_body | 6/8 |
| 7 | 43 | 142 | 2 fn_missing_requires | 18 assume, 124 ext_body | 1/11 |
| 8 | 45 | 23 | 4 fn_missing_spec | 16 ext_body, 4 ext, 3 trivial_wf | 0/7 |
| 9 | 47 | 39 | 6 fn_missing_spec | 39 external_body | 0/9 |
| | **Total** | **211** | **20** | | **37/73** |

All modules depend upon only clean modules — everything is actionable now.

---

## Phase 1: Fix fn_missing_requires/ensures (20 errors, 0 holes)

These are not holes but prevent files from being classified as clean.
Each needs reading the function, understanding what the precondition should be, and adding it.

| # | Chap | File | Function | Error |
|---|------|------|----------|-------|
| 1 | 03 | InsertionSortStEph.rs | insertion_sort | fn_missing_requires |
| 2 | 06 | DirGraphStEph.rs | iter_vertices | fn_missing_ensures |
| 3 | 06 | DirGraphStEph.rs | iter_arcs | fn_missing_ensures |
| 4 | 21 | Algorithm21_5.rs | primes_bf | fn_missing_requires |
| 5 | 21 | Exercise21_7.rs | is_even | fn_missing_requires |
| 6 | 21 | Exercise21_7.rs | is_vowel | fn_missing_requires |
| 7 | 21 | Exercise21_8.rs | is_prime | fn_missing_requires |
| 8 | 23 | BalBinTreeStEph.rs | clone_tree | fn_missing_requires |
| 9 | 43 | AugOrderedTableStPer.rs | calculate_reduction | fn_missing_requires |
| 10 | 43 | OrderedTableStPer.rs | from_sorted_entries | fn_missing_requires |
| 11 | 45 | BinaryHeapPQ.rs | parent | fn_missing_requires |
| 12 | 45 | BinaryHeapPQ.rs | is_heap | fn_missing_ensures |
| 13 | 45 | LeftistHeapPQ.rs | total_order_le | fn_missing_requires |
| 14 | 45 | HeapsortExample.rs | is_sorted | fn_missing_req+ens |
| 15 | 47 | ParaHashTableStEph.rs | createTable | fn_missing_requires |
| 16 | 47 | ParaHashTableStEph.rs | metrics | fn_missing_requires |
| 17 | 47 | ChainedHashTable.rs | lookup_chained | fn_missing_ensures |
| 18 | 47 | StructChainedHashTable.rs | chain_insert | fn_missing_requires |
| 19 | 47 | StructChainedHashTable.rs | chain_lookup | fn_missing_requires |
| 20 | 47 | StructChainedHashTable.rs | chain_delete | fn_missing_requires |

**Impact**: Fixes 20 errors. Cleans Chap03 (1 file), Chap21 (3 files). Partially unblocks
Chap06 (1 file), Chap23 (1 file needs wf too), Chap45 (needs wf/ext_body too), Chap47
(needs ext_body too).

**Effort**: Low. Read each function, determine appropriate requires/ensures from the existing
ensures/body, add the missing spec clause. Validate after each chapter batch.

---

## Phase 2: Write real spec_wf predicates (6 trivial_wf holes)

Replace `{ true }` with real invariants.

| # | Chap | File | Spec fn | What the invariant should capture |
|---|------|------|---------|-----------------------------------|
| 1 | 12 | Exercise12_5.rs | spec_concurrentstackmt_wf | Lock-free stack structural |
| 2 | 23 | BalBinTreeStEph.rs | spec_balbintreesteph_wf | Height balance, valid tree |
| 3 | 23 | PrimTreeSeqStPer.rs | spec_primtreeseqstper_wf | Tree-based seq invariants |
| 4 | 45 | BinaryHeapPQ.rs | spec_binaryheappq_wf | Heap ordering + capacity |
| 5 | 45 | SortedListPQ.rs | spec_sortedlistpq_wf | Sorted order of elements |
| 6 | 45 | UnsortedListPQ.rs | spec_unsortedlistpq_wf | Non-negative length |

**Impact**: -6 holes. Cleans Chap12 (becomes 0 holes). Partially cleans Chap23, Chap45.

**Effort**: Moderate. Must read each data structure, understand what invariants are maintained,
write a spec that is strong enough to be useful but not so strong it breaks existing ensures.
Run validate after each to confirm no regressions.

---

## Phase 3: Prove assumes (18 assumes in Chap43)

The Chap43 assumes fall into two categories:

### 3a. OrderedSetStPer spec_avltreesetstper_wf assumes (12 holes)

All 12 are `assume(self.base_set.spec_avltreesetstper_wf())` or
`assume(other.base_set.spec_avltreesetstper_wf())`. These establish that the underlying
AVLTreeSetStPer is well-formed before calling its methods.

**Strategy**: Add `spec_orderedsetstper_wf` requires to these functions (they already have
ensures). The wf predicate should require `self.base_set.spec_avltreesetstper_wf()`.
Then the assumes become provable from the precondition.

### 3b. OrderedTableStEph assumes (2 holes)

`assume(self.base_table.spec_tablesteph_wf())` in size() and find(). Same pattern as 3a.

### 3c. OrderedTableStPer assume (1 hole)

`assume(entries.spec_avltreeseqstper_wf())` in from_sorted_entries. Add requires.

### 3d. AugOrderedTableStPer assumes (2 holes)

`assume(reducer.requires(...))` in calculate_reduction and join_key. These assume the
reducer function's precondition is satisfied. Need to add requires on the enclosing function
that establishes this, or prove it from loop invariants.

### 3e. OrderedSetStEph assume (1 hole)

`assume(result@ =~= eph_seq@)` in to_seq. Proving View equivalence between ephemeral and
persistent sequences. May need a lemma about Vec→Seq view preservation.

**Impact**: -18 holes. Significantly cleans Chap43 St files.

**Effort**: 3a/3b/3c are moderate — add wf requires, validate. 3d needs more thought about
reducer preconditions. 3e needs proof reasoning about view equivalence.

---

## Phase 4: External_body reduction — ETSP (4 holes, Chap26)

| # | File | Function | Assessment |
|---|------|----------|------------|
| 1 | ETSPStEph.rs | sort_and_split | Sorts points, splits in half. Uses ord. |
| 2 | ETSPStEph.rs | find_best_swap | Iterates edges, finds swap. Loop proof. |
| 3 | ETSPMtEph.rs | sort_and_split | Same as St, parallel context |
| 4 | ETSPMtEph.rs | find_best_swap | Same as St, parallel context |

**Effort**: Moderate-hard. Need to read the algorithms, understand ETSP, write loop invariants.
The Mt versions may share logic with St.

---

## Phase 5: External_body reduction — Chap45 (16 ext_body + 4 external)

BalancedTreePQ has 16 external_body + 2 external. It wraps AVLTreeSetStPer operations.
Many functions (insert, delete_min, meld, etc.) are delegations to the underlying tree.

**Strategy**: Read BalancedTreePQ, determine which functions can be proved by delegation
(similar to Phase 3a pattern: add wf requires, call base type method, prove ensures from
base type's ensures). The external impls (Default, ExtTrait) need assessment.

Example45_2 (1 external) and HeapsortExample (1 external) are comparison/demo files.

**Impact**: Up to -20 holes.

**Effort**: Moderate. Most are delegation wrappers.

---

## Phase 6: External_body reduction — Chap47 Hash Tables (39 holes)

All 39 holes are external_body across 8 hash table implementation files. These are real
algorithmic functions: insert, lookup, delete, resize, probe, find_slot.

| # | File | Holes | Assessment |
|---|------|-------|------------|
| 1 | ParaHashTableStEph.rs | 1 | loadAndSize — simple field access |
| 2 | ChainedHashTable.rs | 2 | insert/delete with chaining |
| 3 | FlatHashTable.rs | 2 | insert/lookup with probing |
| 4 | LinkedListChainedHT.rs | 5 | LL-based chaining ops |
| 5 | VecChainedHT.rs | 5 | Vec-based chaining ops |
| 6 | StructChainedHT.rs | 5 | Struct-based chaining ops |
| 7 | LinProbFlatHT.rs | 6 | Linear probing ops |
| 8 | QuadProbFlatHT.rs | 6 | Quadratic probing ops |
| 9 | DoubleHashFlatHT.rs | 7 | Double hash probing ops |

**Strategy**: Start with ParaHashTableStEph (1 hole, simple). Then ChainedHashTable (2 holes).
These establish patterns for the remaining files which share the same HashTable<K,V,E,M,H>
generic structure.

**Effort**: Hard. Hash table proofs need loop invariants for probing sequences, capacity
invariants, and collision resolution correctness. The APAS textbook analysis guides the specs.

---

## Phase 7: External_body reduction — Chap43 (124 ext_body)

The largest chunk. Breaks down as:

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | OrderedSetMtEph.rs | 23 | Mt wrapper — coarse RwLock migration |
| 2 | OrderedTableMtPer.rs | 21 | Mt wrapper — coarse RwLock migration |
| 3 | OrderedTableMtEph.rs | 17 | Mt wrapper — coarse RwLock migration |
| 4 | OrderedTableStEph.rs | 16 | St — delegate to base type |
| 5 | OrderedSetStEph.rs | 14 | St — delegate to base type + iterators |
| 6 | OrderedSetStPer.rs | 13 | St — delegate to base type |
| 7 | OrderedTableStPer.rs | 10 | St — delegate to base type |
| 8 | AugOrderedTableStEph.rs | 5 | St — delegate to base type |
| 9 | AugOrderedTableMtEph.rs | 5 | Mt wrapper |

**Strategy**:
- **St files (58 ext_body)**: These wrap AVLTree operations. If the base AVLTree methods have
  strong enough specs, we can prove the delegation. Read the base type specs first.
- **Mt files (66 ext_body)**: Need coarse RwLock migration per `plans/coarse-rwlock-migration-plan.md`.
  This is the big structural change — converting from external_body Mt wrappers to the
  standard Layer 1/Layer 2 RwLock pattern.

**Effort**: Very high. Chap43 is the largest chapter and the Mt migration is the most
complex structural work.

---

## Execution Order

| Step | Phase | Files | Holes/Errors | Cumulative Clean |
|------|-------|-------|--------------|------------------|
| 1 | 1 (fn_missing) | Chap03,21 | -0 holes, -5 errors | +4 files clean |
| 2 | 1 (fn_missing) | Chap06,23,45,47,43 | -0 holes, -15 errors | (partial) |
| 3 | 2 (trivial_wf) | Chap12,23,45 | -6 holes | +1 file clean |
| 4 | 3a-c (assumes) | Chap43 St wf | -15 holes | (partial) |
| 5 | 3d-e (assumes) | Chap43 St reducer+view | -3 holes | (partial) |
| 6 | 4 (ETSP) | Chap26 | -4 holes | +2 files clean |
| 7 | 5 (PQ ext_body) | Chap45 | -20 holes | (partial) |
| 8 | 6 (hash tables) | Chap47 | -39 holes | (partial) |
| 9 | 7 St (ordered) | Chap43 St | -58 holes | (partial) |
| 10 | 7 Mt (ordered) | Chap43 Mt | -66 holes | (partial) |

**Target**: Reduce from 211 holes + 20 errors to <50 holes + 0 errors.

---

## Rules

- Use `accept()` only per standards: lock-boundary, eq/clone. Never elsewhere.
- Run `scripts/validate.sh` after each batch of changes. Show full output.
- Run `scripts/holes.sh src/ChapNN/` before and after to verify counts.
- Fix trigger warnings as they occur.
- Read files before modifying.
- When in doubt about an assume, leave it and flag for review.
