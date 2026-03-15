# Agent 3 — Round 20 Report

## Mission
Prove hash table operations across 8 Chap47 files (33 holes: 26 external_body + 7 fn_missing_spec).

## Results Summary

| # | Metric | Before | After | Delta |
|---|--------|--------|-------|-------|
| 1 | external_body holes | 26 | 26 | 0 |
| 2 | fn_missing_spec | 7 | 0 | -7 |
| 3 | Clean modules | 1 | 2 | +1 |
| 4 | Verified functions | 3940 | 3942 | +2 |
| 5 | Exec fns with complete spec | 83 (74%) | 90 (78%) | +7 |
| 6 | Proof functions | 4 | 6 | +2 |
| 7 | Validation errors | 0 | 0 | 0 |
| 8 | RTT | 2600 pass | 2600 pass | 0 |
| 9 | PTT | 147 pass | 147 pass | 0 |

## Per-File Changes

| # | Chap | File | Holes Before | Holes After | Changes |
|---|------|------|-------------|-------------|---------|
| 1 | 47 | ParaHashTableStEph.rs | 5 (2 ext + 3 miss) | 2 (2 ext) | Added requires to createTable/metrics/loadAndSize; added ensures table@ == Map::empty() to createTable; added 2 proof lemmas; strengthened EntryTrait insert/delete ensures |
| 2 | 47 | ChainedHashTable.rs | 0 | 0 | Strengthened insert_chained ensures with table@.dom().contains(key) |
| 3 | 47 | FlatHashTable.rs | 1 (miss) | 0 | Added ensures to lookup_with_probe; strengthened insert_with_probe with table@.dom().contains(key) |
| 4 | 47 | VecChainedHashTableStEph.rs | 4 (ext) | 4 (ext) | No change |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 4 (ext) | 4 (ext) | No change |
| 6 | 47 | StructChainedHashTable.rs | 7 (4 ext + 3 miss) | 4 (ext) | Added requires to chain_insert/chain_lookup/chain_delete |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 4 (ext) | 4 (ext) | No change |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 4 (ext) | 4 (ext) | No change |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 4 (ext) | 4 (ext) | No change |

## Spec Strengthening Achieved

1. **createTable** — `ensures table@ == Map::empty()`: newly created tables have an empty abstract map.
2. **EntryTrait::insert** — `ensures self.spec_entry_to_map().dom().contains(key)`: after entry insert, the entry's map domain contains the key. Proven for all 4 implementations (Vec, LinkedListStEphS, ChainList, FlatEntry).
3. **EntryTrait::delete** — `ensures !deleted ==> self.spec_entry_to_map() == old(self).spec_entry_to_map()`: if delete finds nothing, the entry's map is unchanged. Proven for all 4 implementations.
4. **insert_with_probe** — `ensures table@.dom().contains(key)`: after flat-table insert, the table's abstract map domain contains the key.
5. **insert_chained** — `ensures table@.dom().contains(key)`: after chained insert, the table's abstract map domain contains the key.

## Proof Lemmas Added

1. **lemma_table_to_map_push_empty** — A sequence of empty-map entries produces Map::empty() under spec_table_to_map. Used in createTable proof.
2. **lemma_table_to_map_update_contains** — If a new entry's map contains key k, then spec_table_to_map after updating that slot also contains k. Used in insert_with_probe and insert_chained proofs.

## Why 26 external_body Holes Remain

All 26 external_body holes are on hash table operations (insert/lookup/delete/resize) across 5 strategy files + StructChainedHashTable, plus 2 hash function wrappers.

**Root cause: opaque hash function.** The hash function type `H: Fn(&Key, usize) -> usize` has no spec. `call_hash_fn` is `external_body` with only `ensures index < table_size`. Without a spec connecting key identity to bucket assignment, we cannot prove:
- `table@ == old(table)@.insert(key, value)` (insert correctness)
- `found == Some(table@[*key])` when key is in the table (lookup correctness)
- `table@ == old(table)@.remove(*key)` (delete correctness)
- `resized@ == table@` (resize correctness)

**Path forward:** Add a ghost spec function to HashTable:
```rust
pub struct HashTable<Key, Value, Entry, Metrics, H> {
    ...
    pub ghost spec_hash: FnSpec(Key, nat) -> nat,
}
```
Then `call_hash_fn` could ensure `index == table.spec_hash(*key, table_size as nat)`, enabling invariant-based proofs. This requires API changes cascading through all Chap47 files and test files.

**Secondary blocker: PartialEq bridge for generic Key.** EntryTrait::lookup ensures `!dom.contains(*key) ==> found is None` could not be proven because generic `Key: PartialEq` lacks the spec bridge connecting exec `==` to spec `==`. This blocks entry-level lookup correctness.

**Tertiary blocker: clone_entry spec.** The clone-and-set-back pattern in chained operations requires `clone_entry` to preserve `spec_entry_to_map()`. Without `ensures cloned.spec_entry_to_map() == self.spec_entry_to_map()` on clone_entry (blocked by generic Clone bridge), delete-preserves-map proofs are blocked.

## Techniques Used

- Proof lemma decomposition (spec_table_to_map over sequence updates)
- Extensional equality (`=~=`) for Map/Set domain reasoning
- Ghost variable capture for pre-mutation state
- Broadcast lemma usage (vstd Map::union_prefer_right domain)

## Commit

Hash: (see git log after push)
Branch: agent3/ready
