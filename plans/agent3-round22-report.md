# Agent 3 — Round 22 Report

## Mission

Strengthen EntryTrait specs and prove hash table insert/delete operations in Chap47.
Target: remove 14 of 26 external_body holes by proving insert + delete across all
strategy files.

## Results Summary

| # | Metric | Before | After | Delta |
|---|--------|--------|-------|-------|
| 1 | external_body holes | 26 | 26 | 0 |
| 2 | Verified functions | 3957 | 3960 | +3 |
| 3 | Proof lemmas | 6 | 9 | +3 |
| 4 | Spec ensures strengthened | 0 | 8 | +8 |
| 5 | RTT | 2600 pass | 2600 pass | 0 |
| 6 | PTT | 147 pass | 147 pass | 0 |
| 7 | Validation errors | 0 | 0 | 0 |

## Why the Full Mission Is Not Achievable

The mission's premise — that insert/delete can be proved without a ghost hash function —
is incorrect. Two independent blockers prevent closing any of the 26 external_body holes:

### Blocker 1: Table-Level Insert/Delete Needs Hash Function Invariant

`spec_table_to_map` uses `union_prefer_right` from left to right:
```
spec_table_to_map([e0, e1, ..., en]) = e0.map ∪_R e1.map ∪_R ... ∪_R en.map
```

When updating entry at index `i` from `old_map` to `old_map.insert(key, value)`:
- Entries at index `j > i` can OVERRIDE key via `union_prefer_right`.
- So `spec_table_to_map(updated)` equals `old_table@.insert(key, value)` ONLY IF
  key does not appear in any entry at position `j != i`.
- This is the hash function invariant: each key maps to exactly one bucket.
- Without a ghost `spec_hash: FnSpec(Key, nat) -> nat` field on HashTable, this
  invariant cannot be expressed or maintained.

### Blocker 2: PartialEq Spec Bridge for Generic Key

Entry-level insert/delete implementations use exec `==` (e.g., `self[i].0 == key`)
to find/remove entries. Connecting exec equality to spec equality requires the
`PartialEqSpecImpl` bridge, which does not exist for generic `Key: PartialEq`.

This blocks:
- `chain_insert` proving `spec_chain_to_map(out)[key] == value` in the recursive case.
- Vec/LinkedList proving `spec_seq_pairs_to_map(self@) == old.map.insert(key, value)`.
- All entry-level `== old.map.remove(key)` proofs.

### Blocker 3: FlatEntry Semantic Mismatch

`EntryTrait::insert` with ensures `== old.map.insert(key, value)` is FALSE for FlatEntry
when the old slot had a different key. FlatEntry::insert replaces the entire slot:
`Occupied(k2, v2)` → `Occupied(key, value)` loses k2. But
`{k2→v2}.insert(key, value) = {k2→v2, key→value}` preserves k2.

A uniform trait-level strengthening is impossible because flat entries and chained entries
have fundamentally different per-slot semantics.

## What Was Accomplished

### New Proof Lemmas (ParaHashTableStEph.rs)

| # | Chap | Lemma | Purpose |
|---|------|-------|---------|
| 1 | 47 | `lemma_table_to_map_not_contains` | If key absent from all entries, absent from table map |
| 2 | 47 | `lemma_table_to_map_update_insert` | Single-entry update → table map insert (with hash invariant precondition) |
| 3 | 47 | `lemma_table_to_map_update_remove` | Single-entry update → table map remove (with hash invariant precondition) |

These are the core proof lemmas needed when the ghost hash function infrastructure
is added. The hard proof work (induction over `spec_table_to_map` with
`union_prefer_right` reasoning, extensional Map equality) is done.

### Strengthened Ensures

| # | Chap | File | Function | New Ensures |
|---|------|------|----------|-------------|
| 1 | 47 | ChainedHashTable.rs | `insert_chained` | `current_size` preserved, `num_elements <= old + 1` |
| 2 | 47 | ChainedHashTable.rs | `delete_chained` | `current_size` preserved |
| 3 | 47 | VecChainedHashTableStEph.rs | `Vec::insert` | `self@.last() == (key, value)`, `map[key] == value` |
| 4 | 47 | VecChainedHashTableStEph.rs | `Vec::delete` | `deleted ==> len + 1 == old.len` |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | `LinkedList::insert` | `seq@.last() == (key, value)`, `map[key] == value` |
| 6 | 47 | LinkedListChainedHashTableStEph.rs | `LinkedList::delete` | `deleted ==> len + 1 == old.len` |
| 7 | 47 | FlatHashTable.rs | `FlatEntry::insert` | `map[key] == value` |
| 8 | 47 | FlatHashTable.rs | `FlatEntry::delete` | `deleted ==> map == Map::empty()` |

## Path Forward: Ghost Hash Function Infrastructure

To close the 24 strategy-file external_body holes (the 2 hash-wrapper holes are genuine
FFI boundaries), the following infrastructure is needed:

1. **Add `ghost spec_hash: FnSpec(Key, nat) -> nat` to HashTable struct.**
2. **Strengthen `spec_hashtable_wf`** to include the hash bucket invariant:
   `forall |k, j| j != spec_hash(k) % size ==> !table[j].map.dom.contains(k)`.
3. **Strengthen `call_hash_fn`** ensures: `index == spec_hash(*key, size) % size`.
4. **Update `createTable`** to accept and propagate `spec_hash`.
5. **Update ALL strategy files** and test files to pass `spec_hash` through constructors.
6. **Add `Key: PartialEqSpecImpl`** bound (or assume bridge in each entry impl) for
   entry-level map equality proofs.
7. **Use `lemma_table_to_map_update_insert`/`remove`** in strategy file implementations.

Steps 1-5 are a large cascading change (~200 lines across 10+ files). Step 6 requires
either a new trait bound on Key or per-impl assumes (with user approval). Step 7 uses
the lemmas already proven this round.

Estimated scope: 1 full round dedicated to the ghost hash infrastructure.

## Per-File Changes

| # | Chap | File | Holes Before | Holes After | Changes |
|---|------|------|-------------|-------------|---------|
| 1 | 47 | ParaHashTableStEph.rs | 2 | 2 | Added 3 proof lemmas |
| 2 | 47 | ChainedHashTable.rs | 0 | 0 | Strengthened insert/delete_chained ensures |
| 3 | 47 | FlatHashTable.rs | 0 | 0 | Strengthened FlatEntry insert/delete ensures |
| 4 | 47 | VecChainedHashTableStEph.rs | 4 | 4 | Strengthened Vec insert/delete ensures |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 4 | 4 | Strengthened LinkedList insert/delete ensures |
| 6 | 47 | StructChainedHashTable.rs | 4 | 4 | No change (ChainList blocked by PartialEq bridge) |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 | No change |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 | No change |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 | No change |

## Techniques Used

- Inductive proof over `spec_table_to_map` with `union_prefer_right` decomposition.
- Extensional Map equality (`=~=`) for proving Map algebra identities.
- `assert forall ... implies ... by { assert(...); }` pattern for trigger propagation
  through `Seq::drop_last`/`Seq::update`.
- Sub-lemma composition (not_contains feeds into update_insert/update_remove).

## Commit

Branch: agent3/ready
