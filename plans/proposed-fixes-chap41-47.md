<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Proposed Fixes: Chap41–Chap47 (Agent3)

**Scope:** Chap41, Chap42, Chap43, Chap44, Chap45, Chap47  
**Focus:** Bogus views, unaccepted external_body, unaccepted assumes  
**Priority:** Verusification — Prove Big Or Go Home

## Executed (2025-02-21)

1. **Chap41 AVLTreeSetStEph, StPer, MtPer** — Replaced bogus `spec_set_view` (Set::empty()) with real definition `self.elements@.to_set()`. Uses `closed spec fn` so body is visible within module. Verification passes.
2. **Chap47 ChainedHashTable** — Replaced `external_body` on ChainEntry::clone with body + `assume(result == *self)` (generic Container:Clone cannot be proven). Verification passes.

**Deferred:** Chap43 OrderedSetMtEph/OrderedTableMtPer (view behind RwLock), Chap41 assumes→proofs (larger effort), ParaHashTableStEph HashTable (contains dyn Fn).

---

## Severity-Ordered Fix Table

| # | Sev | Chapter | File | Issue | Description |
|---|-----|---------|------|-------|-------------|
| 1 | crit | Chap41 | AVLTreeSetMtEph.rs | Bogus spec_set_view | spec_set_view external_body returns Set::empty() |
| 2 | crit | Chap41 | AVLTreeSetMtPer.rs | Bogus spec_set_view | spec_set_view external_body returns Set::empty() |
| 3 | crit | Chap41 | AVLTreeSetStEph.rs | Bogus spec_set_view | spec_set_view external_body returns Set::empty() |
| 4 | crit | Chap41 | AVLTreeSetStPer.rs | Bogus spec_set_view | spec_set_view external_body returns Set::empty() |
| 5 | crit | Chap41 | ArraySetEnumMtEph.rs | Bogus view | view() external_body returns Set::empty() |
| 6 | crit | Chap43 | OrderedSetMtEph.rs | Bogus view | view() external_body returns Set::empty() |
| 7 | crit | Chap43 | OrderedTableMtPer.rs | Bogus view | view() external_body returns Map::empty() |
| 8 | high | Chap41 | AVLTreeSetStEph.rs | 33 unaccepted assumes | size, to_seq, empty, singleton, from_seq, filter, intersection, difference, union, find, delete, insert |
| 9 | high | Chap41 | AVLTreeSetStPer.rs | 35 unaccepted assumes | Same ops as StEph |
| 10 | high | Chap41 | AVLTreeSetMtPer.rs | 10 unaccepted assumes | size, to_seq, empty, singleton, find |
| 11 | high | Chap41 | ArraySetStEph.rs | 21 unaccepted assumes | find, filter, intersection, difference, union, delete, insert |
| 12 | high | Chap41 | AVLTreeSetMtEph.rs | 14 external_body | All ops external; RwLock dummy inv |
| 13 | high | Chap41 | ArraySetEnumMtEph.rs | 15 external_body | All ops external |
| 14 | high | Chap42 | TableMtEph.rs | 15 external_body | All ops external |
| 15 | high | Chap42 | TableStEph.rs | 14 external_body | All ops external |
| 16 | high | Chap43 | OrderedSetMtEph.rs | 22 external_body | All ops external (excl view) |
| 17 | high | Chap43 | OrderedTableMtPer.rs | 20 external_body | All ops external (excl view) |
| 18 | high | Chap43 | OrderedSetStEph.rs | 12 external_body | to_seq, from_seq, first, last, prev, next, split, get_range, rank, select, split_rank |
| 19 | high | Chap43 | OrderedSetStPer.rs | 10 external_body | first, last, prev, next, split, get_range, rank, select, split_rank |
| 20 | high | Chap43 | OrderedTableStEph.rs | 16 external_body | singleton, delete, map, filter, reduce, collect, first_key, last_key, prev_key, next_key, split_key, get_key_range, rank_key, select_key, split_rank_key |
| 21 | high | Chap43 | OrderedTableStPer.rs | 27 external_body | All ops external |
| 22 | high | Chap43 | AugOrderedTable* | 14 external_body | calculate_reduction, recalculate_reduction, join_key, reduce_range_parallel, find, lookup, clone |
| 23 | med | Chap41 | AVLTreeSetMtEph.rs | Dummy RwLockPredicate | inv returns true; underspecified |
| 24 | med | Chap41 | AVLTreeSetMtEph.rs | RwLock external_body | new_set_mt_lock requires external_body |
| 25 | med | Chap47 | ChainedHashTable.rs | external_body clone | ChainEntry::clone external_body |
| 26 | med | Chap47 | ParaHashTableStEph.rs | Struct outside verus! | HashTable struct should be inside verus! |
| 27 | low | Chap41 | Example41_3.rs | 4 external_body | Example functions |
| 28 | low | Chap42 | TableStPer.rs | assume_eq_clone | Verus workaround; acceptable |
| 29 | low | Chap45 | BinaryHeapPQ, SortedListPQ, UnsortedListPQ | assume_eq_clone | Verus workaround; acceptable |

---

## Summary by Severity

| Severity | Count |
|----------|-------|
| critical | 7 |
| high | 16 |
| medium | 4 |
| low | 3 |

---

## Heavy Lifts (Prove Big Or Go Home)

### Chap41 AVLTreeSet* (~45 assumes + bogus spec_set_view)

1. **Prove spec_set_view** — Define `spec_set_view` as `elements@.to_set()` (or equivalent) for AVLTreeSeq-backed types. Requires AVLTreeSeq view/spec lemmas connecting seq to set.
2. **Replace assumes with proofs** — Each assume is a proof obligation. Key lemmas: `elements.spec_well_formed() ==> self@ == elements@.to_set()`, `result@ == self@.intersect(other@)` for intersection, etc.
3. **AVLTreeSetMtEph** — Either verusify RwLock usage (proper inv) or keep external_body at spawn boundary only; prove spec_set_view and delegate to AVLTreeSetStEph.

### Chap43 OrderedSetMtEph / OrderedTableMtPer (bogus view)

1. **OrderedSetMtEph** — View must reflect ParamTreap contents. Tree stores `Pair<K,V>`; set view = domain of tree. Prove `view() == tree@.to_set()` (or tree's key set).
2. **OrderedTableMtPer** — View must reflect ParamTreap as Map. Prove `view() == tree@.to_map()` (key→value from Pair sequence).

### Chap47 ChainedHashTable

1. **ChainEntry::clone** — Prove `result == *self` from `chain.clone()`; may need Container View or spec.
2. **ParaHashTableStEph** — Move HashTable struct inside verus!; HashFunGen uses `dyn Fn` — may require external_type or abstraction.

---

## Acceptable (Informational)

- `assume(false); diverge()` in thread join — valid idiom
- `assume_eq_clone_workaround` — Verus limitation on generic Clone
- `accept()` in PartialEq — documented pattern
