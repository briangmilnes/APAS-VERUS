# Chap47 Spec Audit — Hash Tables (ADT 47.1)

Audited 2026-03-15 against APAS Chapter 47, Definition 47.1.

## Structural Finding: No View Impl

`HashTable<Key, Value, Entry, Metrics, H>` has **no `View` impl**. There is no ghost
field or spec function providing a logical `Map<Key, Value>` view of the table's contents.
All existing specs are structural (table length, current_size, num_elements) without
reference to what keys/values are stored.

**Consequence**: The ADT 47.1 functional specs (insert stores key, lookup retrieves stored
value, delete removes key, resize preserves all content) CANNOT be expressed until a View
impl is established.

## Proposed View Design

```rust
// Ghost field on HashTable:
pub ghost abstract_map: Map<Key::V, Value::V>,

impl View for HashTable<...> {
    type V = Map<Key::V, Value::V>;
    open spec fn view(&self) -> Map<Key::V, Value::V> { self.abstract_map }
}
```

Each insert/delete/resize impl would update the ghost field to maintain the invariant
that `self@` equals the logical content of the physical table.

## ParaHashTableStEph.rs (base trait)

| # | Function | Current ensures | ADT 47.1 correct spec | Status |
|---|----------|----------------|----------------------|--------|
| 1 | createTable | size, elements=0, wf | `self@ == Map::empty()` | STRUCTURAL |
| 2 | insert | table.len, current_size, num_elements <= old+1 | `self@.contains_key(k@) && self@[k@] == v@` | STRUCTURAL |
| 3 | lookup | no ensures | `Some(v) ==> self@[k@] == v@, None ==> !self@.contains_key(k@)` | STRUCTURAL |
| 4 | delete | table.len, current_size | `!self@.contains_key(k@)`, other keys preserved | STRUCTURAL |
| 5 | metrics | correct | Correct | OK |
| 6 | loadAndSize | correct | Correct | OK |
| 7 | resize | current_size, table.len | `self@ == old(self)@` (content preserved) | STRUCTURAL |

Infrastructure holes (cannot be removed without restructuring):
- `call_hash_fn`: external_body with tight ensures (`index < table_size`). Correct.
- `compute_second_hash`: external_body (std::hash). Correct.

## FlatHashTable.rs (flat hash table trait)

| # | Function | Current ensures | After | Status |
|---|----------|----------------|-------|--------|
| 1 | probe | `slot < current_size` | No change | OK |
| 2 | find_slot | `slot < current_size` | No change | OK |
| 3 | insert_with_probe | none | Added `table.len, current_size preserved` | IMPROVED |
| 4 | lookup_with_probe | none | No change (nothing meaningful without View) | STRUCTURAL |

## LinProbFlatHashTableStEph.rs

CLEAN (0 holes). All structural specs correct. Probe function verified.

| # | Function | Ensures | Status |
|---|----------|---------|--------|
| 1 | insert | table.len, current_size, num_elements <= old+1 | OK (structural) |
| 2 | lookup | none | STRUCTURAL |
| 3 | delete | table.len, current_size | OK (structural) |
| 4 | resize | current_size, table.len | OK (structural) |
| 5 | probe | `slot < current_size` | OK |
| 6 | find_slot | `slot < current_size` | OK |

## QuadProbFlatHashTableStEph.rs

CLEAN (0 holes). Same pattern as LinProb.

## DoubleHashFlatHashTableStEph.rs

`second_hash` missing ensures (veracity warning). Otherwise same pattern.

| # | Function | Ensures | Status |
|---|----------|---------|--------|
| 1 | second_hash | none | MISSING ensures |
| 2-6 | (same as LinProb) | structural | OK |

## ChainedHashTable.rs (chained hash table trait)

| # | Function | Current ensures | Status |
|---|----------|----------------|--------|
| 1 | hash_index | `index < current_size` | OK |
| 2 | insert_chained | table.len preserved | STRUCTURAL |
| 3 | lookup_chained | `ensures true` | WEAK (vacuous) |
| 4 | delete_chained | table.len preserved | STRUCTURAL |

## LinkedListChainedHashTableStEph.rs

CLEAN (0 holes). Uses `LinkedListStEphS<(Key, Value)>` for chains.

## VecChainedHashTableStEph.rs

CLEAN (0 holes). Uses `Vec<(Key, Value)>` for chains.

## StructChainedHashTable.rs

Most advanced spec infrastructure. Has `spec_chain_to_map` per chain and
`ChainListTrait::spec_to_map`. Chain-level insert/delete have Map-level ensures.
Best candidate for whole-table View extension.

| # | Function | Current ensures | Status |
|---|----------|----------------|--------|
| 1 | chain_insert | `spec_chain_to_map(out).dom().contains(key)` | PARTIAL (chain-level) |
| 2 | chain_lookup | `chain is None ==> found is None` | PARTIAL |
| 3 | chain_delete | `!deleted ==> map preserved` | PARTIAL (chain-level) |

## Summary

| File | Holes | Spec quality | Notes |
|------|-------|-------------|-------|
| ParaHashTableStEph.rs | 2 (external_body) | Structural only | Needs View for ADT specs |
| FlatHashTable.rs | 0 | Improved (+ensures) | insert_with_probe now has ensures |
| LinProbFlatHashTableStEph.rs | 0 | Structural | OK |
| QuadProbFlatHashTableStEph.rs | 0 | Structural | OK |
| DoubleHashFlatHashTableStEph.rs | 0 | Structural | second_hash missing ensures |
| ChainedHashTable.rs | 0 | Structural | lookup_chained `ensures true` |
| LinkedListChainedHashTableStEph.rs | 0 | Structural | OK |
| VecChainedHashTableStEph.rs | 0 | Structural | OK |
| StructChainedHashTable.rs | 0 | Partial chain specs | Best foundation for View |

## Recommended Next Steps

1. **Define `View` on `HashTable`** with ghost `Map<Key::V, Value::V>` field.
2. **StructChainedHashTable first**: already has `spec_chain_to_map`. Extend to
   whole-table View by summing per-bucket maps.
3. **Flat hash tables**: Define View as the set of Live entries. More complex due
   to probe sequence semantics.
4. **After View**: Strengthen insert/lookup/delete/resize ensures per ADT 47.1.
